#[derive(Debug, Default)]
pub struct CreateQueue {
	pub queue_name: String,
	pub attribute: Vec<Attribute>,
}

impl XmlParser for CreateQueue {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = CreateQueue::default();
		try!(start_element("CreateQueue", stack));
		obj.queue_name = try!(string_field("QueueName", stack));
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("CreateQueue", stack));
		Ok(obj)
	}
}

impl SQSRequest<CreateQueueResponse> for CreateQueue {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "CreateQueue");
		params.put("QueueName", &self.queue_name.to_string());
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct CreateQueueResponse {
	pub create_queue_result: CreateQueueResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for CreateQueueResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = CreateQueueResponse::default();
		try!(start_element("CreateQueueResponse", stack));
		obj.create_queue_result = try!(CreateQueueResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("CreateQueueResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct GetQueueUrl {
	pub queue_name: String,
	pub queue_owner_awsaccount_id: Option<String>,
}

impl XmlParser for GetQueueUrl {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueUrl::default();
		try!(start_element("GetQueueUrl", stack));
		obj.queue_name = try!(string_field("QueueName", stack));
		if try!(peek_at_name(stack)) == "QueueOwnerAWSAccountId" { obj.queue_owner_awsaccount_id = Some(try!(string_field("QueueOwnerAWSAccountId", stack))) }
		try!(end_element("GetQueueUrl", stack));
		Ok(obj)
	}
}

impl SQSRequest<GetQueueUrlResponse> for GetQueueUrl {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "GetQueueUrl");
		params.put("QueueName", &self.queue_name.to_string());
		params.optional_put("QueueOwnerAWSAccountId", &self.queue_owner_awsaccount_id);
		params
	}
}

#[derive(Debug, Default)]
pub struct GetQueueUrlResponse {
	pub get_queue_url_result: GetQueueUrlResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for GetQueueUrlResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueUrlResponse::default();
		try!(start_element("GetQueueUrlResponse", stack));
		obj.get_queue_url_result = try!(GetQueueUrlResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("GetQueueUrlResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ListQueues {
	pub queue_name_prefix: Option<String>,
	pub attribute: Vec<Attribute>,
}

impl XmlParser for ListQueues {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListQueues::default();
		try!(start_element("ListQueues", stack));
		if try!(peek_at_name(stack)) == "QueueNamePrefix" { obj.queue_name_prefix = Some(try!(string_field("QueueNamePrefix", stack))) }
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("ListQueues", stack));
		Ok(obj)
	}
}

impl SQSRequest<ListQueuesResponse> for ListQueues {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "ListQueues");
		params.optional_put("QueueNamePrefix", &self.queue_name_prefix);
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct ListQueuesResponse {
	pub list_queues_result: ListQueuesResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for ListQueuesResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListQueuesResponse::default();
		try!(start_element("ListQueuesResponse", stack));
		obj.list_queues_result = try!(ListQueuesResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("ListQueuesResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibility {
	pub receipt_handle: String,
	pub visibility_timeout: i32,
	pub attribute: Vec<Attribute>,
}

impl XmlParser for ChangeMessageVisibility {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibility::default();
		try!(start_element("ChangeMessageVisibility", stack));
		obj.receipt_handle = try!(string_field("ReceiptHandle", stack));
		obj.visibility_timeout = i32::from_str(try!(string_field("VisibilityTimeout", stack)).as_ref()).unwrap();
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("ChangeMessageVisibility", stack));
		Ok(obj)
	}
}

impl SQSRequest<ChangeMessageVisibilityResponse> for ChangeMessageVisibility {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "ChangeMessageVisibility");
		params.put("ReceiptHandle", &self.receipt_handle.to_string());
		params.put("VisibilityTimeout", &self.visibility_timeout.to_string());
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for ChangeMessageVisibilityResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityResponse::default();
		try!(start_element("ChangeMessageVisibilityResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("ChangeMessageVisibilityResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessage {
	pub receipt_handle: String,
	pub attribute: Vec<Attribute>,
}

impl XmlParser for DeleteMessage {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessage::default();
		try!(start_element("DeleteMessage", stack));
		obj.receipt_handle = try!(string_field("ReceiptHandle", stack));
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("DeleteMessage", stack));
		Ok(obj)
	}
}

impl SQSRequest<DeleteMessageResponse> for DeleteMessage {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "DeleteMessage");
		params.put("ReceiptHandle", &self.receipt_handle.to_string());
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for DeleteMessageResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageResponse::default();
		try!(start_element("DeleteMessageResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("DeleteMessageResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteQueue {
	pub attribute: Vec<Attribute>,
}

impl XmlParser for DeleteQueue {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteQueue::default();
		try!(start_element("DeleteQueue", stack));
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("DeleteQueue", stack));
		Ok(obj)
	}
}

impl SQSRequest<DeleteQueueResponse> for DeleteQueue {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "DeleteQueue");
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct DeleteQueueResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for DeleteQueueResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteQueueResponse::default();
		try!(start_element("DeleteQueueResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("DeleteQueueResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct GetQueueAttributes {
	pub attribute_name: Vec<String>,
	pub unused: Option<String>,
}

impl XmlParser for GetQueueAttributes {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueAttributes::default();
		try!(start_element("GetQueueAttributes", stack));
		while try!(peek_at_name(stack)) == "AttributeName" { obj.attribute_name.push(try!(string_field("AttributeName", stack)));}
		if try!(peek_at_name(stack)) == "Unused" { obj.unused = Some(try!(string_field("Unused", stack))) }
		try!(end_element("GetQueueAttributes", stack));
		Ok(obj)
	}
}

impl SQSRequest<GetQueueAttributesResponse> for GetQueueAttributes {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "GetQueueAttributes");
		let mut index = 1; for ref elem in &self.attribute_name { params.put(&format!("AttributeName.{}", index), &elem); index += 1; }
		params.optional_put("Unused", &self.unused);
		params
	}
}

#[derive(Debug, Default)]
pub struct GetQueueAttributesResponse {
	pub get_queue_attributes_result: GetQueueAttributesResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for GetQueueAttributesResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueAttributesResponse::default();
		try!(start_element("GetQueueAttributesResponse", stack));
		obj.get_queue_attributes_result = try!(GetQueueAttributesResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("GetQueueAttributesResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ReceiveMessage {
	pub max_number_of_messages: Option<i32>,
	pub visibility_timeout: Option<i32>,
	pub wait_time_seconds: Option<i32>,
	pub attribute_name: Vec<String>,
	pub message_attribute_name: Vec<String>,
	pub unused: Option<String>,
}

impl XmlParser for ReceiveMessage {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ReceiveMessage::default();
		try!(start_element("ReceiveMessage", stack));
		if try!(peek_at_name(stack)) == "MaxNumberOfMessages" { obj.max_number_of_messages = Some(i32::from_str(try!(string_field("MaxNumberOfMessages", stack)).as_ref()).unwrap()) }
		if try!(peek_at_name(stack)) == "VisibilityTimeout" { obj.visibility_timeout = Some(i32::from_str(try!(string_field("VisibilityTimeout", stack)).as_ref()).unwrap()) }
		if try!(peek_at_name(stack)) == "WaitTimeSeconds" { obj.wait_time_seconds = Some(i32::from_str(try!(string_field("WaitTimeSeconds", stack)).as_ref()).unwrap()) }
		while try!(peek_at_name(stack)) == "AttributeName" { obj.attribute_name.push(try!(string_field("AttributeName", stack)));}
		while try!(peek_at_name(stack)) == "MessageAttributeName" { obj.message_attribute_name.push(try!(string_field("MessageAttributeName", stack)));}
		if try!(peek_at_name(stack)) == "Unused" { obj.unused = Some(try!(string_field("Unused", stack))) }
		try!(end_element("ReceiveMessage", stack));
		Ok(obj)
	}
}

impl SQSRequest<ReceiveMessageResponse> for ReceiveMessage {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "ReceiveMessage");
		params.optional_put("MaxNumberOfMessages", &self.max_number_of_messages);
		params.optional_put("VisibilityTimeout", &self.visibility_timeout);
		params.optional_put("WaitTimeSeconds", &self.wait_time_seconds);
		let mut index = 1; for ref elem in &self.attribute_name { params.put(&format!("AttributeName.{}", index), &elem); index += 1; }
		let mut index = 1; for ref elem in &self.message_attribute_name { params.put(&format!("MessageAttributeName.{}", index), &elem); index += 1; }
		params.optional_put("Unused", &self.unused);
		params
	}
}

#[derive(Debug, Default)]
pub struct ReceiveMessageResponse {
	pub receive_message_result: ReceiveMessageResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for ReceiveMessageResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ReceiveMessageResponse::default();
		try!(start_element("ReceiveMessageResponse", stack));
		obj.receive_message_result = try!(ReceiveMessageResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("ReceiveMessageResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessage {
	pub message_body: String,
	pub delay_seconds: Option<i32>,
	pub attribute: Vec<Attribute>,
	pub message_attribute: Vec<MessageAttribute>,
}

impl XmlParser for SendMessage {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessage::default();
		try!(start_element("SendMessage", stack));
		obj.message_body = try!(string_field("MessageBody", stack));
		if try!(peek_at_name(stack)) == "DelaySeconds" { obj.delay_seconds = Some(i32::from_str(try!(string_field("DelaySeconds", stack)).as_ref()).unwrap()) }
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		while try!(peek_at_name(stack)) == "MessageAttribute" { obj.message_attribute.push(try!(MessageAttribute::parse_xml(stack)));}
		try!(end_element("SendMessage", stack));
		Ok(obj)
	}
}

impl SQSRequest<SendMessageResponse> for SendMessage {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "SendMessage");
		params.put("MessageBody", &self.message_body.to_string());
		params.optional_put("DelaySeconds", &self.delay_seconds);
		serialize_attribute(&mut params, &self.attribute);
		serialize_message_attribute(&mut params, &self.message_attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct SendMessageResponse {
	pub send_message_result: SendMessageResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for SendMessageResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageResponse::default();
		try!(start_element("SendMessageResponse", stack));
		obj.send_message_result = try!(SendMessageResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("SendMessageResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SetQueueAttributes {
	pub attribute: Vec<Attribute>,
}

impl XmlParser for SetQueueAttributes {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SetQueueAttributes::default();
		try!(start_element("SetQueueAttributes", stack));
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("SetQueueAttributes", stack));
		Ok(obj)
	}
}

impl SQSRequest<SetQueueAttributesResponse> for SetQueueAttributes {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "SetQueueAttributes");
		serialize_attribute(&mut params, &self.attribute);
		params
	}
}

#[derive(Debug, Default)]
pub struct SetQueueAttributesResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for SetQueueAttributesResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SetQueueAttributesResponse::default();
		try!(start_element("SetQueueAttributesResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("SetQueueAttributesResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct BatchResultErrorEntry {
	pub id: String,
	pub code: String,
	pub message: String,
	pub sender_fault: String,
}

impl XmlParser for BatchResultErrorEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = BatchResultErrorEntry::default();
		try!(start_element("BatchResultErrorEntry", stack));
		obj.id = try!(string_field("Id", stack));
		obj.code = try!(string_field("Code", stack));
		obj.message = try!(string_field("Message", stack));
		obj.sender_fault = try!(string_field("SenderFault", stack));
		try!(end_element("BatchResultErrorEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchRequestEntry {
	pub id: String,
	pub receipt_handle: String,
	pub visibility_timeout: Option<i32>,
}

impl XmlParser for ChangeMessageVisibilityBatchRequestEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityBatchRequestEntry::default();
		try!(start_element("ChangeMessageVisibilityBatchRequestEntry", stack));
		obj.id = try!(string_field("Id", stack));
		obj.receipt_handle = try!(string_field("ReceiptHandle", stack));
		if try!(peek_at_name(stack)) == "VisibilityTimeout" { obj.visibility_timeout = Some(i32::from_str(try!(string_field("VisibilityTimeout", stack)).as_ref()).unwrap()) }
		try!(end_element("ChangeMessageVisibilityBatchRequestEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatch {
	pub change_message_visibility_batch_request_entry: Vec<ChangeMessageVisibilityBatchRequestEntry>,
}

impl XmlParser for ChangeMessageVisibilityBatch {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityBatch::default();
		try!(start_element("ChangeMessageVisibilityBatch", stack));
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchRequestEntry" { obj.change_message_visibility_batch_request_entry.push(try!(ChangeMessageVisibilityBatchRequestEntry::parse_xml(stack)));}
		try!(end_element("ChangeMessageVisibilityBatch", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResultEntry {
	pub id: String,
}

impl XmlParser for ChangeMessageVisibilityBatchResultEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityBatchResultEntry::default();
		try!(start_element("ChangeMessageVisibilityBatchResultEntry", stack));
		obj.id = try!(string_field("Id", stack));
		try!(end_element("ChangeMessageVisibilityBatchResultEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResult {
	pub change_message_visibility_batch_result_entry: Vec<ChangeMessageVisibilityBatchResultEntry>,
	pub batch_result_error_entry: Vec<BatchResultErrorEntry>,
}

impl XmlParser for ChangeMessageVisibilityBatchResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityBatchResult::default();
		try!(start_element("ChangeMessageVisibilityBatchResult", stack));
		while try!(peek_at_name(stack)) == "ChangeMessageVisibilityBatchResultEntry" { obj.change_message_visibility_batch_result_entry.push(try!(ChangeMessageVisibilityBatchResultEntry::parse_xml(stack)));}
		while try!(peek_at_name(stack)) == "BatchResultErrorEntry" { obj.batch_result_error_entry.push(try!(BatchResultErrorEntry::parse_xml(stack)));}
		try!(end_element("ChangeMessageVisibilityBatchResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ChangeMessageVisibilityBatchResponse {
	pub change_message_visibility_batch_result: ChangeMessageVisibilityBatchResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for ChangeMessageVisibilityBatchResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ChangeMessageVisibilityBatchResponse::default();
		try!(start_element("ChangeMessageVisibilityBatchResponse", stack));
		obj.change_message_visibility_batch_result = try!(ChangeMessageVisibilityBatchResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("ChangeMessageVisibilityBatchResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchRequestEntry {
	pub id: String,
	pub receipt_handle: String,
}

impl XmlParser for DeleteMessageBatchRequestEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageBatchRequestEntry::default();
		try!(start_element("DeleteMessageBatchRequestEntry", stack));
		obj.id = try!(string_field("Id", stack));
		obj.receipt_handle = try!(string_field("ReceiptHandle", stack));
		try!(end_element("DeleteMessageBatchRequestEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatch {
	pub delete_message_batch_request_entry: Vec<DeleteMessageBatchRequestEntry>,
}

impl XmlParser for DeleteMessageBatch {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageBatch::default();
		try!(start_element("DeleteMessageBatch", stack));
		while try!(peek_at_name(stack)) == "DeleteMessageBatchRequestEntry" { obj.delete_message_batch_request_entry.push(try!(DeleteMessageBatchRequestEntry::parse_xml(stack)));}
		try!(end_element("DeleteMessageBatch", stack));
		Ok(obj)
	}
}

impl SQSRequest<DeleteMessageBatchResponse> for DeleteMessageBatch {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "DeleteMessageBatch");
		serialize_delete_message_batch_request_entry(&mut params, &self.delete_message_batch_request_entry);
		params
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchResultEntry {
	pub id: String,
}

impl XmlParser for DeleteMessageBatchResultEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageBatchResultEntry::default();
		try!(start_element("DeleteMessageBatchResultEntry", stack));
		obj.id = try!(string_field("Id", stack));
		try!(end_element("DeleteMessageBatchResultEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchResult {
	pub delete_message_batch_result_entry: Vec<DeleteMessageBatchResultEntry>,
	pub batch_result_error_entry: Vec<BatchResultErrorEntry>,
}

impl XmlParser for DeleteMessageBatchResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageBatchResult::default();
		try!(start_element("DeleteMessageBatchResult", stack));
		while try!(peek_at_name(stack)) == "DeleteMessageBatchResultEntry" { obj.delete_message_batch_result_entry.push(try!(DeleteMessageBatchResultEntry::parse_xml(stack)));}
		while try!(peek_at_name(stack)) == "BatchResultErrorEntry" { obj.batch_result_error_entry.push(try!(BatchResultErrorEntry::parse_xml(stack)));}
		try!(end_element("DeleteMessageBatchResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct DeleteMessageBatchResponse {
	pub delete_message_batch_result: DeleteMessageBatchResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for DeleteMessageBatchResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = DeleteMessageBatchResponse::default();
		try!(start_element("DeleteMessageBatchResponse", stack));
		obj.delete_message_batch_result = try!(DeleteMessageBatchResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("DeleteMessageBatchResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchRequestEntry {
	pub id: String,
	pub message_body: String,
	pub delay_seconds: Option<i32>,
	pub message_attribute: Vec<MessageAttribute>,
}

impl XmlParser for SendMessageBatchRequestEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageBatchRequestEntry::default();
		try!(start_element("SendMessageBatchRequestEntry", stack));
		obj.id = try!(string_field("Id", stack));
		obj.message_body = try!(string_field("MessageBody", stack));
		if try!(peek_at_name(stack)) == "DelaySeconds" { obj.delay_seconds = Some(i32::from_str(try!(string_field("DelaySeconds", stack)).as_ref()).unwrap()) }
		while try!(peek_at_name(stack)) == "MessageAttribute" { obj.message_attribute.push(try!(MessageAttribute::parse_xml(stack)));}
		try!(end_element("SendMessageBatchRequestEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatch {
	pub send_message_batch_request_entry: Vec<SendMessageBatchRequestEntry>,
}

impl XmlParser for SendMessageBatch {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageBatch::default();
		try!(start_element("SendMessageBatch", stack));
		while try!(peek_at_name(stack)) == "SendMessageBatchRequestEntry" { obj.send_message_batch_request_entry.push(try!(SendMessageBatchRequestEntry::parse_xml(stack)));}
		try!(end_element("SendMessageBatch", stack));
		Ok(obj)
	}
}

impl SQSRequest<SendMessageBatchResponse> for SendMessageBatch {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "SendMessageBatch");
		serialize_send_message_batch_request_entry(&mut params, &self.send_message_batch_request_entry);
		params
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchResultEntry {
	pub id: String,
	pub message_id: String,
	pub md5_of_message_body: String,
	pub md5_of_message_attributes: String,
}

impl XmlParser for SendMessageBatchResultEntry {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageBatchResultEntry::default();
		try!(start_element("SendMessageBatchResultEntry", stack));
		obj.id = try!(string_field("Id", stack));
		obj.message_id = try!(string_field("MessageId", stack));
		obj.md5_of_message_body = try!(string_field("MD5OfMessageBody", stack));
		obj.md5_of_message_attributes = try!(string_field("MD5OfMessageAttributes", stack));
		try!(end_element("SendMessageBatchResultEntry", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchResult {
	pub send_message_batch_result_entry: Vec<SendMessageBatchResultEntry>,
	pub batch_result_error_entry: Vec<BatchResultErrorEntry>,
}

impl XmlParser for SendMessageBatchResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageBatchResult::default();
		try!(start_element("SendMessageBatchResult", stack));
		while try!(peek_at_name(stack)) == "SendMessageBatchResultEntry" { obj.send_message_batch_result_entry.push(try!(SendMessageBatchResultEntry::parse_xml(stack)));}
		while try!(peek_at_name(stack)) == "BatchResultErrorEntry" { obj.batch_result_error_entry.push(try!(BatchResultErrorEntry::parse_xml(stack)));}
		try!(end_element("SendMessageBatchResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessageBatchResponse {
	pub send_message_batch_result: SendMessageBatchResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for SendMessageBatchResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageBatchResponse::default();
		try!(start_element("SendMessageBatchResponse", stack));
		obj.send_message_batch_result = try!(SendMessageBatchResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("SendMessageBatchResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct AddPermission {
	pub label: String,
	pub awsaccount_id: Vec<String>,
	pub action_name: Vec<String>,
}

impl XmlParser for AddPermission {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = AddPermission::default();
		try!(start_element("AddPermission", stack));
		obj.label = try!(string_field("Label", stack));
		while try!(peek_at_name(stack)) == "AWSAccountId" { obj.awsaccount_id.push(try!(string_field("AWSAccountId", stack)));}
		while try!(peek_at_name(stack)) == "ActionName" { obj.action_name.push(try!(string_field("ActionName", stack)));}
		try!(end_element("AddPermission", stack));
		Ok(obj)
	}
}

impl SQSRequest<AddPermissionResponse> for AddPermission {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "AddPermission");
		params.put("Label", &self.label.to_string());
		let mut index = 1; for ref elem in &self.awsaccount_id { params.put(&format!("AWSAccountId.{}", index), &elem); index += 1; }
		let mut index = 1; for ref elem in &self.action_name { params.put(&format!("ActionName.{}", index), &elem); index += 1; }
		params
	}
}

#[derive(Debug, Default)]
pub struct AddPermissionResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for AddPermissionResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = AddPermissionResponse::default();
		try!(start_element("AddPermissionResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("AddPermissionResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct RemovePermission {
	pub label: String,
}

impl XmlParser for RemovePermission {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = RemovePermission::default();
		try!(start_element("RemovePermission", stack));
		obj.label = try!(string_field("Label", stack));
		try!(end_element("RemovePermission", stack));
		Ok(obj)
	}
}

impl SQSRequest<RemovePermissionResponse> for RemovePermission {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "RemovePermission");
		params.put("Label", &self.label.to_string());
		params
	}
}

#[derive(Debug, Default)]
pub struct RemovePermissionResponse {
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for RemovePermissionResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = RemovePermissionResponse::default();
		try!(start_element("RemovePermissionResponse", stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("RemovePermissionResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueues;

impl XmlParser for ListDeadLetterSourceQueues {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListDeadLetterSourceQueues::default();
		try!(start_element("ListDeadLetterSourceQueues", stack));
		try!(end_element("ListDeadLetterSourceQueues", stack));
		Ok(obj)
	}
}

impl SQSRequest<ListDeadLetterSourceQueuesResponse> for ListDeadLetterSourceQueues {
	fn to_params(&self) -> Params {
		let mut params = Params::new();
		params.put("Action", "ListDeadLetterSourceQueues");
		params
	}
}

#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueuesResponse {
	pub list_dead_letter_source_queues_result: ListDeadLetterSourceQueuesResult,
	pub response_metadata: ResponseMetadata,
}

impl XmlParser for ListDeadLetterSourceQueuesResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListDeadLetterSourceQueuesResponse::default();
		try!(start_element("ListDeadLetterSourceQueuesResponse", stack));
		obj.list_dead_letter_source_queues_result = try!(ListDeadLetterSourceQueuesResult::parse_xml(stack));
		obj.response_metadata = try!(ResponseMetadata::parse_xml(stack));
		try!(end_element("ListDeadLetterSourceQueuesResponse", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct CreateQueueResult {
	pub queue_url: String,
}

impl XmlParser for CreateQueueResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = CreateQueueResult::default();
		try!(start_element("CreateQueueResult", stack));
		obj.queue_url = try!(string_field("QueueUrl", stack));
		try!(end_element("CreateQueueResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct GetQueueUrlResult {
	pub queue_url: String,
}

impl XmlParser for GetQueueUrlResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueUrlResult::default();
		try!(start_element("GetQueueUrlResult", stack));
		obj.queue_url = try!(string_field("QueueUrl", stack));
		try!(end_element("GetQueueUrlResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ListQueuesResult {
	pub queue_url: Vec<String>,
}

impl XmlParser for ListQueuesResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListQueuesResult::default();
		try!(start_element("ListQueuesResult", stack));
		while try!(peek_at_name(stack)) == "QueueUrl" { obj.queue_url.push(try!(string_field("QueueUrl", stack)));}
		try!(end_element("ListQueuesResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ListDeadLetterSourceQueuesResult {
	pub queue_url: Vec<String>,
}

impl XmlParser for ListDeadLetterSourceQueuesResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ListDeadLetterSourceQueuesResult::default();
		try!(start_element("ListDeadLetterSourceQueuesResult", stack));
		while try!(peek_at_name(stack)) == "QueueUrl" { obj.queue_url.push(try!(string_field("QueueUrl", stack)));}
		try!(end_element("ListDeadLetterSourceQueuesResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct Attribute {
	pub name: String,
	pub value: String,
}

impl XmlParser for Attribute {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = Attribute::default();
		try!(start_element("Attribute", stack));
		obj.name = try!(string_field("Name", stack));
		obj.value = try!(string_field("Value", stack));
		try!(end_element("Attribute", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct MessageAttribute {
	pub name: String,
	pub value: MessageAttributeValue,
}

impl XmlParser for MessageAttribute {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = MessageAttribute::default();
		try!(start_element("MessageAttribute", stack));
		obj.name = try!(string_field("Name", stack));
		obj.value = try!(MessageAttributeValue::parse_xml(stack));
		try!(end_element("MessageAttribute", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct GetQueueAttributesResult {
	pub attribute: Vec<Attribute>,
}

impl XmlParser for GetQueueAttributesResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = GetQueueAttributesResult::default();
		try!(start_element("GetQueueAttributesResult", stack));
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		try!(end_element("GetQueueAttributesResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct Message {
	pub message_id: String,
	pub receipt_handle: String,
	pub md5_of_body: String,
	pub md5_of_message_attributes: String,
	pub body: String,
	pub attribute: Vec<Attribute>,
	pub message_attribute: Vec<MessageAttribute>,
}

impl XmlParser for Message {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = Message::default();
		try!(start_element("Message", stack));
		obj.body = try!(string_field("Body", stack));
		
		let mut current_tag = try!(peek_at_name(stack));
		while msg_body_element(&current_tag) {		
			if current_tag == "ReceiptHandle" { obj.receipt_handle = try!(string_field("ReceiptHandle", stack)); }
			if current_tag == "MD5OfBody" { obj.md5_of_body = try!(string_field("MD5OfBody", stack)); }
			if current_tag == "MessageId" { obj.message_id = try!(string_field("MessageId", stack)); }
			if current_tag == "MD5OfMessageAttributes" { obj.md5_of_message_attributes = try!(string_field("MD5OfMessageAttributes", stack)); }
			current_tag = try!(peek_at_name(stack));
		}
		while try!(peek_at_name(stack)) == "Attribute" { obj.attribute.push(try!(Attribute::parse_xml(stack)));}
		while try!(peek_at_name(stack)) == "MessageAttribute" { obj.message_attribute.push(try!(MessageAttribute::parse_xml(stack)));}
		try!(end_element("Message", stack));
		Ok(obj)
	}
}

fn msg_body_element(element: &str) -> bool {
	["Body", "ReceiptHandle", "MD5OfBody", "MessageId", "MD5OfMessageAttributes"].contains(&element)
}

#[derive(Debug, Default)]
pub struct ReceiveMessageResult {
	pub message: Vec<Message>,
}

impl XmlParser for ReceiveMessageResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ReceiveMessageResult::default();
		try!(start_element("ReceiveMessageResult", stack));
		while try!(peek_at_name(stack)) == "Message" { obj.message.push(try!(Message::parse_xml(stack)));}
		try!(end_element("ReceiveMessageResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct SendMessageResult {
	pub message_id: String,
	pub md5_of_message_body: String,
	pub md5_of_message_attributes: String,
}

impl XmlParser for SendMessageResult {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = SendMessageResult::default();
		try!(start_element("SendMessageResult", stack));
		obj.message_id = try!(string_field("MessageId", stack));
		obj.md5_of_message_body = try!(string_field("MD5OfMessageBody", stack));
		if try!(peek_at_name(stack)) == "MD5OfMessageAttributes" { obj.md5_of_message_attributes = try!(string_field("MD5OfMessageAttributes", stack)); }
		try!(end_element("SendMessageResult", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct ResponseMetadata {
	pub request_id: String,
}

impl XmlParser for ResponseMetadata {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ResponseMetadata::default();
		try!(start_element("ResponseMetadata", stack));
		obj.request_id = try!(string_field("RequestId", stack));
		try!(end_element("ResponseMetadata", stack));
		Ok(obj)
	}
}

#[derive(Debug, Default)]
pub struct MessageAttributeValue {
	pub data_type: String,
	pub string_value: String,
	pub binary_value: String,
}

impl XmlParser for MessageAttributeValue {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = MessageAttributeValue::default();
		try!(start_element("MessageAttributeValue", stack));
		obj.data_type = try!(string_field("DataType", stack));
		try!(end_element("MessageAttributeValue", stack));
		Ok(obj)
	}
}

