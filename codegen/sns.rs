use std::collections::HashMap;
use std::str;
/// Response for GetSubscriptionAttributes action.
#[derive(Debug, Default)]
pub struct GetSubscriptionAttributesResponse {
	/// A map of the subscription's attributes. Attributes in this map include the
	/// following:
	///   * `SubscriptionArn` \-- the subscription's ARN
	///   * `TopicArn` \-- the topic ARN that the subscription is associated with
	///   * `Owner` \-- the AWS account ID of the subscription's owner
	///   * `ConfirmationWasAuthenticated` \-- true if the subscription confirmation request was authenticated
	///   * `DeliveryPolicy` \-- the JSON serialization of the subscription's delivery policy
	///   * `EffectiveDeliveryPolicy` \-- the JSON serialization of the effective delivery policy that takes into account the topic delivery policy and account system defaults
	pub attributes: SubscriptionAttributesMap,
}

/// Parse GetSubscriptionAttributesResponse from XML
struct GetSubscriptionAttributesResponseParser;
impl GetSubscriptionAttributesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSubscriptionAttributesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSubscriptionAttributesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(SubscriptionAttributesMapParser::parse_xml("Attributes", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSubscriptionAttributesResponse contents to a SignedRequest
struct GetSubscriptionAttributesResponseWriter;
impl GetSubscriptionAttributesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSubscriptionAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SubscriptionAttributesMapWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
pub type Binary = Vec<u8>;
/// Parse Binary from XML
struct BinaryParser;
impl BinaryParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Binary, XmlParseError> {
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
/// The user-specified message attribute value. For string data types, the value
/// attribute has the same restrictions on the content as the message body. For
/// more information, see
/// [Publish](http://docs.aws.amazon.com/sns/latest/api/API_Publish.html).
/// Name, type, and value must not be empty or null. In addition, the message body
/// should not be empty or null. All parts of the message attribute, including
/// name, type, and value, are included in the message size restriction, which is
/// currently 256 KB (262,144 bytes). For more information, see [Using Amazon SNS
/// Message Attributes](http://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttribu
/// tes.html).
#[derive(Debug, Default)]
pub struct MessageAttributeValue {
	/// Amazon SNS supports the following logical data types: String, Number, and
	/// Binary. For more information, see [Message Attribute Data Types](http://docs.a
	/// ws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.Dat
	/// aTypes).
	pub data_type: String,
	/// Strings are Unicode with UTF8 binary encoding. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	pub string_value: Option<String>,
	/// Binary type attributes can store any binary data, for example, compressed
	/// data, encrypted data, or images.
	pub binary_value: Option<Binary>,
}

/// Parse MessageAttributeValue from XML
struct MessageAttributeValueParser;
impl MessageAttributeValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = MessageAttributeValue::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "DataType" {
				obj.data_type = try!(StringParser::parse_xml("DataType", stack));
				continue;
			}
			if current_name == "StringValue" {
				obj.string_value = Some(try!(StringParser::parse_xml("StringValue", stack)));
				continue;
			}
			if current_name == "BinaryValue" {
				obj.binary_value = Some(try!(BinaryParser::parse_xml("BinaryValue", stack)));
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
		StringWriter::write_params(params, &(prefix.to_string() + "DataType"), &obj.data_type);
		if let Some(ref obj) = obj.string_value {
			StringWriter::write_params(params, &(prefix.to_string() + "StringValue"), obj);
		}
		if let Some(ref obj) = obj.binary_value {
			BinaryWriter::write_params(params, &(prefix.to_string() + "BinaryValue"), obj);
		}
	}
}
/// Input for Unsubscribe action.
#[derive(Debug, Default)]
pub struct UnsubscribeInput {
	/// The ARN of the subscription to be deleted.
	pub subscription_arn: subscriptionARN,
}

/// Parse UnsubscribeInput from XML
struct UnsubscribeInputParser;
impl UnsubscribeInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UnsubscribeInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = UnsubscribeInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write UnsubscribeInput contents to a SignedRequest
struct UnsubscribeInputWriter;
impl UnsubscribeInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &UnsubscribeInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type TopicAttributesMap = HashMap<attributeName,attributeValue>;
/// Parse TopicAttributesMap from XML
struct TopicAttributesMapParser;
impl TopicAttributesMapParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicAttributesMap, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(attributeNameParser::parse_xml("attributeName", stack));
			let value = try!(attributeValueParser::parse_xml("attributeValue", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
/// Write TopicAttributesMap contents to a SignedRequest
struct TopicAttributesMapWriter;
impl TopicAttributesMapWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TopicAttributesMap) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			attributeNameWriter::write_params(params, &format!("{}.{}", prefix, "attributeName"), &key);
			attributeValueWriter::write_params(params, &format!("{}.{}", prefix, "attributeValue"), &value);
			index += 1;
		}
	}
}
pub type DelegatesList = Vec<delegate>;
/// Parse DelegatesList from XML
struct DelegatesListParser;
impl DelegatesListParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DelegatesList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "delegate" {
			obj.push(try!(delegateParser::parse_xml("delegate", stack)));
		}
		Ok(obj)
	}
}
/// Write DelegatesList contents to a SignedRequest
struct DelegatesListWriter;
impl DelegatesListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DelegatesList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			delegateWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Input for ListEndpointsByPlatformApplication action.
#[derive(Debug, Default)]
pub struct ListEndpointsByPlatformApplicationInput {
	/// NextToken string is used when calling ListEndpointsByPlatformApplication
	/// action to retrieve additional records that are available after the first page
	/// results.
	pub next_token: Option<String>,
	/// PlatformApplicationArn for ListEndpointsByPlatformApplicationInput action.
	pub platform_application_arn: String,
}

/// Parse ListEndpointsByPlatformApplicationInput from XML
struct ListEndpointsByPlatformApplicationInputParser;
impl ListEndpointsByPlatformApplicationInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEndpointsByPlatformApplicationInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListEndpointsByPlatformApplicationInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = Some(try!(StringParser::parse_xml("NextToken", stack)));
				continue;
			}
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListEndpointsByPlatformApplicationInput contents to a SignedRequest
struct ListEndpointsByPlatformApplicationInputWriter;
impl ListEndpointsByPlatformApplicationInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListEndpointsByPlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.next_token {
			StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type SubscriptionAttributesMap = HashMap<attributeName,attributeValue>;
/// Parse SubscriptionAttributesMap from XML
struct SubscriptionAttributesMapParser;
impl SubscriptionAttributesMapParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionAttributesMap, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(attributeNameParser::parse_xml("attributeName", stack));
			let value = try!(attributeValueParser::parse_xml("attributeValue", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
/// Write SubscriptionAttributesMap contents to a SignedRequest
struct SubscriptionAttributesMapWriter;
impl SubscriptionAttributesMapWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SubscriptionAttributesMap) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			attributeNameWriter::write_params(params, &format!("{}.{}", prefix, "attributeName"), &key);
			attributeValueWriter::write_params(params, &format!("{}.{}", prefix, "attributeValue"), &value);
			index += 1;
		}
	}
}
/// Indicates that the customer already owns the maximum allowed number of topics.
#[derive(Debug, Default)]
pub struct TopicLimitExceededException {
	pub message: string,
}

/// Parse TopicLimitExceededException from XML
struct TopicLimitExceededExceptionParser;
impl TopicLimitExceededExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicLimitExceededException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = TopicLimitExceededException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write TopicLimitExceededException contents to a SignedRequest
struct TopicLimitExceededExceptionWriter;
impl TopicLimitExceededExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TopicLimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type SubscriptionsList = Vec<Subscription>;
/// Parse SubscriptionsList from XML
struct SubscriptionsListParser;
impl SubscriptionsListParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Subscription" {
			obj.push(try!(SubscriptionParser::parse_xml("Subscription", stack)));
		}
		Ok(obj)
	}
}
/// Write SubscriptionsList contents to a SignedRequest
struct SubscriptionsListWriter;
impl SubscriptionsListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SubscriptionsList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SubscriptionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Parse String from XML
struct StringParser;
impl StringParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<String, XmlParseError> {
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
/// Response from GetEndpointAttributes of the EndpointArn.
#[derive(Debug, Default)]
pub struct GetEndpointAttributesResponse {
	/// Attributes include the following:
	///   * `CustomUserData` \-- arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.
	///   * `Enabled` \-- flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.
	///   * `Token` \-- device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.
	pub attributes: MapStringToString,
}

/// Parse GetEndpointAttributesResponse from XML
struct GetEndpointAttributesResponseParser;
impl GetEndpointAttributesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetEndpointAttributesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetEndpointAttributesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetEndpointAttributesResponse contents to a SignedRequest
struct GetEndpointAttributesResponseWriter;
impl GetEndpointAttributesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetEndpointAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for SetPlatformApplicationAttributes action.
#[derive(Debug, Default)]
pub struct SetPlatformApplicationAttributesInput {
	/// A map of the platform application attributes. Attributes in this map include
	/// the following:
	///   * `PlatformCredential` \-- The credential received from the notification service. For APNS/APNS_SANDBOX, PlatformCredential is "private key". For GCM, PlatformCredential is "API key". For ADM, PlatformCredential is "client secret".
	///   * `PlatformPrincipal` \-- The principal received from the notification service. For APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For GCM, PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client id".
	///   * `EventEndpointCreated` \-- Topic ARN to which EndpointCreated event notifications should be sent.
	///   * `EventEndpointDeleted` \-- Topic ARN to which EndpointDeleted event notifications should be sent.
	///   * `EventEndpointUpdated` \-- Topic ARN to which EndpointUpdate event notifications should be sent.
	///   * `EventDeliveryFailure` \-- Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application's endpoints.
	pub attributes: MapStringToString,
	/// PlatformApplicationArn for SetPlatformApplicationAttributes action.
	pub platform_application_arn: String,
}

/// Parse SetPlatformApplicationAttributesInput from XML
struct SetPlatformApplicationAttributesInputParser;
impl SetPlatformApplicationAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetPlatformApplicationAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetPlatformApplicationAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetPlatformApplicationAttributesInput contents to a SignedRequest
struct SetPlatformApplicationAttributesInputWriter;
impl SetPlatformApplicationAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetPlatformApplicationAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
/// Response for ListSubscriptions action
#[derive(Debug, Default)]
pub struct ListSubscriptionsResponse {
	/// Token to pass along to the next `ListSubscriptions` request. This element is
	/// returned if there are more subscriptions to retrieve.
	pub next_token: nextToken,
	/// A list of subscriptions.
	pub subscriptions: SubscriptionsList,
}

/// Parse ListSubscriptionsResponse from XML
struct ListSubscriptionsResponseParser;
impl ListSubscriptionsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSubscriptionsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(nextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "Subscription" {
				obj.subscriptions = try!(SubscriptionsListParser::parse_xml("Subscription", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSubscriptionsResponse contents to a SignedRequest
struct ListSubscriptionsResponseWriter;
impl ListSubscriptionsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		SubscriptionsListWriter::write_params(params, &(prefix.to_string() + "Subscription"), &obj.subscriptions);
	}
}
/// Response for ListTopics action.
#[derive(Debug, Default)]
pub struct ListTopicsResponse {
	/// A list of topic ARNs.
	pub topics: TopicsList,
	/// Token to pass along to the next `ListTopics` request. This element is returned
	/// if there are additional topics to retrieve.
	pub next_token: nextToken,
}

/// Parse ListTopicsResponse from XML
struct ListTopicsResponseParser;
impl ListTopicsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListTopicsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListTopicsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Topic" {
				obj.topics = try!(TopicsListParser::parse_xml("Topic", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(nextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListTopicsResponse contents to a SignedRequest
struct ListTopicsResponseWriter;
impl ListTopicsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListTopicsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TopicsListWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topics);
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
pub type topicName = String;
/// Parse topicName from XML
struct topicNameParser;
impl topicNameParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<topicName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write topicName contents to a SignedRequest
struct topicNameWriter;
impl topicNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &topicName) {
		params.put(name, obj);
	}
}
pub type ListOfPlatformApplications = Vec<PlatformApplication>;
/// Parse ListOfPlatformApplications from XML
struct ListOfPlatformApplicationsParser;
impl ListOfPlatformApplicationsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOfPlatformApplications, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PlatformApplication" {
			obj.push(try!(PlatformApplicationParser::parse_xml("PlatformApplication", stack)));
		}
		Ok(obj)
	}
}
/// Write ListOfPlatformApplications contents to a SignedRequest
struct ListOfPlatformApplicationsWriter;
impl ListOfPlatformApplicationsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListOfPlatformApplications) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			PlatformApplicationWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Indicates that the customer already owns the maximum allowed number of
/// subscriptions.
#[derive(Debug, Default)]
pub struct SubscriptionLimitExceededException {
	pub message: string,
}

/// Parse SubscriptionLimitExceededException from XML
struct SubscriptionLimitExceededExceptionParser;
impl SubscriptionLimitExceededExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionLimitExceededException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SubscriptionLimitExceededException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SubscriptionLimitExceededException contents to a SignedRequest
struct SubscriptionLimitExceededExceptionWriter;
impl SubscriptionLimitExceededExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SubscriptionLimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type string = String;
/// Parse string from XML
struct stringParser;
impl stringParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<string, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write string contents to a SignedRequest
struct stringWriter;
impl stringWriter {
	fn write_params(params: &mut Params, name: &str, obj: &string) {
		params.put(name, obj);
	}
}
/// Response for GetTopicAttributes action.
#[derive(Debug, Default)]
pub struct GetTopicAttributesResponse {
	/// A map of the topic's attributes. Attributes in this map include the following:
	///   * `TopicArn` \-- the topic's ARN
	///   * `Owner` \-- the AWS account ID of the topic's owner
	///   * `Policy` \-- the JSON serialization of the topic's access control policy
	///   * `DisplayName` \-- the human-readable name used in the "From" field for notifications to email and email-json endpoints
	///   * `SubscriptionsPending` \-- the number of subscriptions pending confirmation on this topic
	///   * `SubscriptionsConfirmed` \-- the number of confirmed subscriptions on this topic
	///   * `SubscriptionsDeleted` \-- the number of deleted subscriptions on this topic
	///   * `DeliveryPolicy` \-- the JSON serialization of the topic's delivery policy
	///   * `EffectiveDeliveryPolicy` \-- the JSON serialization of the effective delivery policy that takes into account system defaults
	pub attributes: TopicAttributesMap,
}

/// Parse GetTopicAttributesResponse from XML
struct GetTopicAttributesResponseParser;
impl GetTopicAttributesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTopicAttributesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTopicAttributesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(TopicAttributesMapParser::parse_xml("Attributes", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTopicAttributesResponse contents to a SignedRequest
struct GetTopicAttributesResponseWriter;
impl GetTopicAttributesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTopicAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TopicAttributesMapWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Response from CreatePlatformApplication action.
#[derive(Debug, Default)]
pub struct CreatePlatformApplicationResponse {
	/// PlatformApplicationArn is returned.
	pub platform_application_arn: String,
}

/// Parse CreatePlatformApplicationResponse from XML
struct CreatePlatformApplicationResponseParser;
impl CreatePlatformApplicationResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformApplicationResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePlatformApplicationResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePlatformApplicationResponse contents to a SignedRequest
struct CreatePlatformApplicationResponseWriter;
impl CreatePlatformApplicationResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformApplicationResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type topicARN = String;
/// Parse topicARN from XML
struct topicARNParser;
impl topicARNParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<topicARN, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write topicARN contents to a SignedRequest
struct topicARNWriter;
impl topicARNWriter {
	fn write_params(params: &mut Params, name: &str, obj: &topicARN) {
		params.put(name, obj);
	}
}
/// Indicates that the user has been denied access to the requested resource.
#[derive(Debug, Default)]
pub struct AuthorizationErrorException {
	pub message: string,
}

/// Parse AuthorizationErrorException from XML
struct AuthorizationErrorExceptionParser;
impl AuthorizationErrorExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AuthorizationErrorException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AuthorizationErrorException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AuthorizationErrorException contents to a SignedRequest
struct AuthorizationErrorExceptionWriter;
impl AuthorizationErrorExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AuthorizationErrorException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for RemovePermission action.
#[derive(Debug, Default)]
pub struct RemovePermissionInput {
	/// The ARN of the topic whose access control policy you wish to modify.
	pub topic_arn: topicARN,
	/// The unique label of the statement you want to remove.
	pub label: label,
}

/// Parse RemovePermissionInput from XML
struct RemovePermissionInputParser;
impl RemovePermissionInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemovePermissionInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = RemovePermissionInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			if current_name == "Label" {
				obj.label = try!(labelParser::parse_xml("Label", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write RemovePermissionInput contents to a SignedRequest
struct RemovePermissionInputWriter;
impl RemovePermissionInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &RemovePermissionInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
		labelWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
/// Response from CreateTopic action.
#[derive(Debug, Default)]
pub struct CreateTopicResponse {
	/// The Amazon Resource Name (ARN) assigned to the created topic.
	pub topic_arn: topicARN,
}

/// Parse CreateTopicResponse from XML
struct CreateTopicResponseParser;
impl CreateTopicResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateTopicResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateTopicResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateTopicResponse contents to a SignedRequest
struct CreateTopicResponseWriter;
impl CreateTopicResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateTopicResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub type ListOfEndpoints = Vec<Endpoint>;
/// Parse ListOfEndpoints from XML
struct ListOfEndpointsParser;
impl ListOfEndpointsParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOfEndpoints, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Endpoint" {
			obj.push(try!(EndpointParser::parse_xml("Endpoint", stack)));
		}
		Ok(obj)
	}
}
/// Write ListOfEndpoints contents to a SignedRequest
struct ListOfEndpointsWriter;
impl ListOfEndpointsWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListOfEndpoints) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			EndpointWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Platform application object.
#[derive(Debug, Default)]
pub struct PlatformApplication {
	/// Attributes for platform application object.
	pub attributes: MapStringToString,
	/// PlatformApplicationArn for platform application object.
	pub platform_application_arn: String,
}

/// Parse PlatformApplication from XML
struct PlatformApplicationParser;
impl PlatformApplicationParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PlatformApplication, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PlatformApplication::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PlatformApplication contents to a SignedRequest
struct PlatformApplicationWriter;
impl PlatformApplicationWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PlatformApplication) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type protocol = String;
/// Parse protocol from XML
struct protocolParser;
impl protocolParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<protocol, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write protocol contents to a SignedRequest
struct protocolWriter;
impl protocolWriter {
	fn write_params(params: &mut Params, name: &str, obj: &protocol) {
		params.put(name, obj);
	}
}
pub type label = String;
/// Parse label from XML
struct labelParser;
impl labelParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<label, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write label contents to a SignedRequest
struct labelWriter;
impl labelWriter {
	fn write_params(params: &mut Params, name: &str, obj: &label) {
		params.put(name, obj);
	}
}
/// Indicates that a request parameter does not comply with the associated
/// constraints.
#[derive(Debug, Default)]
pub struct InvalidParameterException {
	pub message: string,
}

/// Parse InvalidParameterException from XML
struct InvalidParameterExceptionParser;
impl InvalidParameterExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidParameterException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidParameterException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidParameterException contents to a SignedRequest
struct InvalidParameterExceptionWriter;
impl InvalidParameterExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidParameterException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for CreatePlatformApplication action.
#[derive(Debug, Default)]
pub struct CreatePlatformApplicationInput {
	/// The following platforms are supported: ADM (Amazon Device Messaging), APNS
	/// (Apple Push Notification Service), APNS_SANDBOX, and GCM (Google Cloud
	/// Messaging).
	pub platform: String,
	/// Application names must be made up of only uppercase and lowercase ASCII
	/// letters, numbers, underscores, hyphens, and periods, and must be between 1 and
	/// 256 characters long.
	pub name: String,
	/// For a list of attributes, see [SetPlatformApplicationAttributes](http://docs.a
	/// ws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html)
	pub attributes: MapStringToString,
}

/// Parse CreatePlatformApplicationInput from XML
struct CreatePlatformApplicationInputParser;
impl CreatePlatformApplicationInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformApplicationInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePlatformApplicationInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Platform" {
				obj.platform = try!(StringParser::parse_xml("Platform", stack));
				continue;
			}
			if current_name == "Name" {
				obj.name = try!(StringParser::parse_xml("Name", stack));
				continue;
			}
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePlatformApplicationInput contents to a SignedRequest
struct CreatePlatformApplicationInputWriter;
impl CreatePlatformApplicationInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Platform"), &obj.platform);
		StringWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for CreateTopic action.
#[derive(Debug, Default)]
pub struct CreateTopicInput {
	/// The name of the topic you want to create.
	/// Constraints: Topic names must be made up of only uppercase and lowercase ASCII
	/// letters, numbers, underscores, and hyphens, and must be between 1 and 256
	/// characters long.
	pub name: topicName,
}

/// Parse CreateTopicInput from XML
struct CreateTopicInputParser;
impl CreateTopicInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateTopicInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateTopicInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Name" {
				obj.name = try!(topicNameParser::parse_xml("Name", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateTopicInput contents to a SignedRequest
struct CreateTopicInputWriter;
impl CreateTopicInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateTopicInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
	}
}
pub type action = String;
/// Parse action from XML
struct actionParser;
impl actionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<action, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write action contents to a SignedRequest
struct actionWriter;
impl actionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &action) {
		params.put(name, obj);
	}
}
/// Input for DeleteEndpoint action.
#[derive(Debug, Default)]
pub struct DeleteEndpointInput {
	/// EndpointArn of endpoint to delete.
	pub endpoint_arn: String,
}

/// Parse DeleteEndpointInput from XML
struct DeleteEndpointInputParser;
impl DeleteEndpointInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteEndpointInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteEndpointInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "EndpointArn" {
				obj.endpoint_arn = try!(StringParser::parse_xml("EndpointArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteEndpointInput contents to a SignedRequest
struct DeleteEndpointInputWriter;
impl DeleteEndpointInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteEndpointInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for GetSubscriptionAttributes.
#[derive(Debug, Default)]
pub struct GetSubscriptionAttributesInput {
	/// The ARN of the subscription whose properties you want to get.
	pub subscription_arn: subscriptionARN,
}

/// Parse GetSubscriptionAttributesInput from XML
struct GetSubscriptionAttributesInputParser;
impl GetSubscriptionAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSubscriptionAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetSubscriptionAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetSubscriptionAttributesInput contents to a SignedRequest
struct GetSubscriptionAttributesInputWriter;
impl GetSubscriptionAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetSubscriptionAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type messageStructure = String;
/// Parse messageStructure from XML
struct messageStructureParser;
impl messageStructureParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<messageStructure, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write messageStructure contents to a SignedRequest
struct messageStructureWriter;
impl messageStructureWriter {
	fn write_params(params: &mut Params, name: &str, obj: &messageStructure) {
		params.put(name, obj);
	}
}
pub type ActionsList = Vec<action>;
/// Parse ActionsList from XML
struct ActionsListParser;
impl ActionsListParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActionsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "action" {
			obj.push(try!(actionParser::parse_xml("action", stack)));
		}
		Ok(obj)
	}
}
/// Write ActionsList contents to a SignedRequest
struct ActionsListWriter;
impl ActionsListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ActionsList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			actionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type subject = String;
/// Parse subject from XML
struct subjectParser;
impl subjectParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<subject, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write subject contents to a SignedRequest
struct subjectWriter;
impl subjectWriter {
	fn write_params(params: &mut Params, name: &str, obj: &subject) {
		params.put(name, obj);
	}
}
/// Response from CreateEndpoint action.
#[derive(Debug, Default)]
pub struct CreateEndpointResponse {
	/// EndpointArn returned from CreateEndpoint action.
	pub endpoint_arn: String,
}

/// Parse CreateEndpointResponse from XML
struct CreateEndpointResponseParser;
impl CreateEndpointResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateEndpointResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreateEndpointResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "EndpointArn" {
				obj.endpoint_arn = try!(StringParser::parse_xml("EndpointArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreateEndpointResponse contents to a SignedRequest
struct CreateEndpointResponseWriter;
impl CreateEndpointResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreateEndpointResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Indicates that the requested resource does not exist.
#[derive(Debug, Default)]
pub struct NotFoundException {
	pub message: string,
}

/// Parse NotFoundException from XML
struct NotFoundExceptionParser;
impl NotFoundExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotFoundException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = NotFoundException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write NotFoundException contents to a SignedRequest
struct NotFoundExceptionWriter;
impl NotFoundExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &NotFoundException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for SetEndpointAttributes action.
#[derive(Debug, Default)]
pub struct SetEndpointAttributesInput {
	/// A map of the endpoint attributes. Attributes in this map include the
	/// following:
	///   * `CustomUserData` \-- arbitrary user data to associate with the endpoint. Amazon SNS does not use this data. The data must be in UTF-8 format and less than 2KB.
	///   * `Enabled` \-- flag that enables/disables delivery to the endpoint. Amazon SNS will set this to false when a notification service indicates to Amazon SNS that the endpoint is invalid. Users can set it back to true, typically after updating Token.
	///   * `Token` \-- device token, also referred to as a registration id, for an app and mobile device. This is returned from the notification service when an app and mobile device are registered with the notification service.
	pub attributes: MapStringToString,
	/// EndpointArn used for SetEndpointAttributes action.
	pub endpoint_arn: String,
}

/// Parse SetEndpointAttributesInput from XML
struct SetEndpointAttributesInputParser;
impl SetEndpointAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetEndpointAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetEndpointAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			if current_name == "EndpointArn" {
				obj.endpoint_arn = try!(StringParser::parse_xml("EndpointArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetEndpointAttributesInput contents to a SignedRequest
struct SetEndpointAttributesInputWriter;
impl SetEndpointAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetEndpointAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for GetPlatformApplicationAttributes action.
#[derive(Debug, Default)]
pub struct GetPlatformApplicationAttributesInput {
	/// PlatformApplicationArn for GetPlatformApplicationAttributesInput.
	pub platform_application_arn: String,
}

/// Parse GetPlatformApplicationAttributesInput from XML
struct GetPlatformApplicationAttributesInputParser;
impl GetPlatformApplicationAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPlatformApplicationAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPlatformApplicationAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPlatformApplicationAttributesInput contents to a SignedRequest
struct GetPlatformApplicationAttributesInputWriter;
impl GetPlatformApplicationAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPlatformApplicationAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type nextToken = String;
/// Parse nextToken from XML
struct nextTokenParser;
impl nextTokenParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<nextToken, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write nextToken contents to a SignedRequest
struct nextTokenWriter;
impl nextTokenWriter {
	fn write_params(params: &mut Params, name: &str, obj: &nextToken) {
		params.put(name, obj);
	}
}
/// Input for CreatePlatformEndpoint action.
#[derive(Debug, Default)]
pub struct CreatePlatformEndpointInput {
	/// For a list of attributes, see [SetEndpointAttributes](http://docs.aws.amazon.c
	/// om/sns/latest/api/API_SetEndpointAttributes.html).
	pub attributes: Option<MapStringToString>,
	/// Unique identifier created by the notification service for an app on a device.
	/// The specific name for Token will vary, depending on which notification service
	/// is being used. For example, when using APNS as the notification service, you
	/// need the device token. Alternatively, when using GCM or ADM, the device token
	/// equivalent is called the registration ID.
	pub token: String,
	/// PlatformApplicationArn returned from CreatePlatformApplication is used to
	/// create a an endpoint.
	pub platform_application_arn: String,
	/// Arbitrary user data to associate with the endpoint. Amazon SNS does not use
	/// this data. The data must be in UTF-8 format and less than 2KB.
	pub custom_user_data: Option<String>,
}

/// Parse CreatePlatformEndpointInput from XML
struct CreatePlatformEndpointInputParser;
impl CreatePlatformEndpointInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformEndpointInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = CreatePlatformEndpointInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = Some(try!(MapStringToStringParser::parse_xml("Attributes", stack)));
				continue;
			}
			if current_name == "Token" {
				obj.token = try!(StringParser::parse_xml("Token", stack));
				continue;
			}
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			if current_name == "CustomUserData" {
				obj.custom_user_data = Some(try!(StringParser::parse_xml("CustomUserData", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write CreatePlatformEndpointInput contents to a SignedRequest
struct CreatePlatformEndpointInputWriter;
impl CreatePlatformEndpointInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformEndpointInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.attributes {
			MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "Token"), &obj.token);
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
		if let Some(ref obj) = obj.custom_user_data {
			StringWriter::write_params(params, &(prefix.to_string() + "CustomUserData"), obj);
		}
	}
}
/// Endpoint for mobile app and device.
#[derive(Debug, Default)]
pub struct Endpoint {
	/// Attributes for endpoint.
	pub attributes: MapStringToString,
	/// EndpointArn for mobile app and device.
	pub endpoint_arn: String,
}

/// Parse Endpoint from XML
struct EndpointParser;
impl EndpointParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Endpoint, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Endpoint::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			if current_name == "EndpointArn" {
				obj.endpoint_arn = try!(StringParser::parse_xml("EndpointArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Endpoint contents to a SignedRequest
struct EndpointWriter;
impl EndpointWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Endpoint) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
pub type attributeValue = String;
/// Parse attributeValue from XML
struct attributeValueParser;
impl attributeValueParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attributeValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write attributeValue contents to a SignedRequest
struct attributeValueWriter;
impl attributeValueWriter {
	fn write_params(params: &mut Params, name: &str, obj: &attributeValue) {
		params.put(name, obj);
	}
}
/// Input for ListPlatformApplications action.
#[derive(Debug, Default)]
pub struct ListPlatformApplicationsInput {
	/// NextToken string is used when calling ListPlatformApplications action to
	/// retrieve additional records that are available after the first page results.
	pub next_token: String,
}

/// Parse ListPlatformApplicationsInput from XML
struct ListPlatformApplicationsInputParser;
impl ListPlatformApplicationsInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPlatformApplicationsInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPlatformApplicationsInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(StringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPlatformApplicationsInput contents to a SignedRequest
struct ListPlatformApplicationsInputWriter;
impl ListPlatformApplicationsInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPlatformApplicationsInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Response for Publish action.
#[derive(Debug, Default)]
pub struct PublishResponse {
	/// Unique identifier assigned to the published message.
	/// Length Constraint: Maximum 100 characters
	pub message_id: messageId,
}

/// Parse PublishResponse from XML
struct PublishResponseParser;
impl PublishResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PublishResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PublishResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MessageId" {
				obj.message_id = try!(messageIdParser::parse_xml("MessageId", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PublishResponse contents to a SignedRequest
struct PublishResponseWriter;
impl PublishResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PublishResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		messageIdWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
	}
}
pub type attributeName = String;
/// Parse attributeName from XML
struct attributeNameParser;
impl attributeNameParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write attributeName contents to a SignedRequest
struct attributeNameWriter;
impl attributeNameWriter {
	fn write_params(params: &mut Params, name: &str, obj: &attributeName) {
		params.put(name, obj);
	}
}
/// Response for ConfirmSubscriptions action.
#[derive(Debug, Default)]
pub struct ConfirmSubscriptionResponse {
	/// The ARN of the created subscription.
	pub subscription_arn: subscriptionARN,
}

/// Parse ConfirmSubscriptionResponse from XML
struct ConfirmSubscriptionResponseParser;
impl ConfirmSubscriptionResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ConfirmSubscriptionResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ConfirmSubscriptionResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ConfirmSubscriptionResponse contents to a SignedRequest
struct ConfirmSubscriptionResponseWriter;
impl ConfirmSubscriptionResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ConfirmSubscriptionResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
#[derive(Debug, Default)]
pub struct DeleteTopicInput {
	/// The ARN of the topic you want to delete.
	pub topic_arn: topicARN,
}

/// Parse DeleteTopicInput from XML
struct DeleteTopicInputParser;
impl DeleteTopicInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteTopicInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeleteTopicInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeleteTopicInput contents to a SignedRequest
struct DeleteTopicInputWriter;
impl DeleteTopicInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeleteTopicInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
/// Input for ConfirmSubscription action.
#[derive(Debug, Default)]
pub struct ConfirmSubscriptionInput {
	/// Short-lived token sent to an endpoint during the `Subscribe` action.
	pub token: token,
	/// Disallows unauthenticated unsubscribes of the subscription. If the value of
	/// this parameter is `true` and the request has an AWS signature, then only the
	/// topic owner and the subscription owner can unsubscribe the endpoint. The
	/// unsubscribe action requires AWS authentication.
	pub authenticate_on_unsubscribe: Option<authenticateOnUnsubscribe>,
	/// The ARN of the topic for which you wish to confirm a subscription.
	pub topic_arn: topicARN,
}

/// Parse ConfirmSubscriptionInput from XML
struct ConfirmSubscriptionInputParser;
impl ConfirmSubscriptionInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ConfirmSubscriptionInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ConfirmSubscriptionInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Token" {
				obj.token = try!(tokenParser::parse_xml("Token", stack));
				continue;
			}
			if current_name == "AuthenticateOnUnsubscribe" {
				obj.authenticate_on_unsubscribe = Some(try!(authenticateOnUnsubscribeParser::parse_xml("AuthenticateOnUnsubscribe", stack)));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ConfirmSubscriptionInput contents to a SignedRequest
struct ConfirmSubscriptionInputWriter;
impl ConfirmSubscriptionInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ConfirmSubscriptionInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		tokenWriter::write_params(params, &(prefix.to_string() + "Token"), &obj.token);
		if let Some(ref obj) = obj.authenticate_on_unsubscribe {
			authenticateOnUnsubscribeWriter::write_params(params, &(prefix.to_string() + "AuthenticateOnUnsubscribe"), obj);
		}
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
/// Exception error indicating platform application disabled.
#[derive(Debug, Default)]
pub struct PlatformApplicationDisabledException {
	/// Message for platform application disabled.
	pub message: string,
}

/// Parse PlatformApplicationDisabledException from XML
struct PlatformApplicationDisabledExceptionParser;
impl PlatformApplicationDisabledExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PlatformApplicationDisabledException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PlatformApplicationDisabledException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PlatformApplicationDisabledException contents to a SignedRequest
struct PlatformApplicationDisabledExceptionWriter;
impl PlatformApplicationDisabledExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PlatformApplicationDisabledException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Indicates that a request parameter does not comply with the associated
/// constraints.
#[derive(Debug, Default)]
pub struct InvalidParameterValueException {
	pub message: string,
}

/// Parse InvalidParameterValueException from XML
struct InvalidParameterValueExceptionParser;
impl InvalidParameterValueExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidParameterValueException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InvalidParameterValueException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InvalidParameterValueException contents to a SignedRequest
struct InvalidParameterValueExceptionWriter;
impl InvalidParameterValueExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InvalidParameterValueException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type authenticateOnUnsubscribe = String;
/// Parse authenticateOnUnsubscribe from XML
struct authenticateOnUnsubscribeParser;
impl authenticateOnUnsubscribeParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<authenticateOnUnsubscribe, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write authenticateOnUnsubscribe contents to a SignedRequest
struct authenticateOnUnsubscribeWriter;
impl authenticateOnUnsubscribeWriter {
	fn write_params(params: &mut Params, name: &str, obj: &authenticateOnUnsubscribe) {
		params.put(name, obj);
	}
}
pub type messageId = String;
/// Parse messageId from XML
struct messageIdParser;
impl messageIdParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<messageId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write messageId contents to a SignedRequest
struct messageIdWriter;
impl messageIdWriter {
	fn write_params(params: &mut Params, name: &str, obj: &messageId) {
		params.put(name, obj);
	}
}
/// Input for GetEndpointAttributes action.
#[derive(Debug, Default)]
pub struct GetEndpointAttributesInput {
	/// EndpointArn for GetEndpointAttributes input.
	pub endpoint_arn: String,
}

/// Parse GetEndpointAttributesInput from XML
struct GetEndpointAttributesInputParser;
impl GetEndpointAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetEndpointAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetEndpointAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "EndpointArn" {
				obj.endpoint_arn = try!(StringParser::parse_xml("EndpointArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetEndpointAttributesInput contents to a SignedRequest
struct GetEndpointAttributesInputWriter;
impl GetEndpointAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetEndpointAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for ListSubscriptionsByTopic action.
#[derive(Debug, Default)]
pub struct ListSubscriptionsByTopicInput {
	/// Token returned by the previous `ListSubscriptionsByTopic` request.
	pub next_token: Option<nextToken>,
	/// The ARN of the topic for which you wish to find subscriptions.
	pub topic_arn: topicARN,
}

/// Parse ListSubscriptionsByTopicInput from XML
struct ListSubscriptionsByTopicInputParser;
impl ListSubscriptionsByTopicInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsByTopicInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSubscriptionsByTopicInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = Some(try!(nextTokenParser::parse_xml("NextToken", stack)));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSubscriptionsByTopicInput contents to a SignedRequest
struct ListSubscriptionsByTopicInputWriter;
impl ListSubscriptionsByTopicInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsByTopicInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.next_token {
			nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
/// Response for Subscribe action.
#[derive(Debug, Default)]
pub struct SubscribeResponse {
	/// The ARN of the subscription, if the service was able to create a subscription
	/// immediately (without requiring endpoint owner confirmation).
	pub subscription_arn: subscriptionARN,
}

/// Parse SubscribeResponse from XML
struct SubscribeResponseParser;
impl SubscribeResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscribeResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SubscribeResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SubscribeResponse contents to a SignedRequest
struct SubscribeResponseWriter;
impl SubscribeResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SubscribeResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type message = String;
/// Parse message from XML
struct messageParser;
impl messageParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<message, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write message contents to a SignedRequest
struct messageWriter;
impl messageWriter {
	fn write_params(params: &mut Params, name: &str, obj: &message) {
		params.put(name, obj);
	}
}
pub type TopicsList = Vec<Topic>;
/// Parse TopicsList from XML
struct TopicsListParser;
impl TopicsListParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Topic" {
			obj.push(try!(TopicParser::parse_xml("Topic", stack)));
		}
		Ok(obj)
	}
}
/// Write TopicsList contents to a SignedRequest
struct TopicsListWriter;
impl TopicsListWriter {
	fn write_params(params: &mut Params, name: &str, obj: &TopicsList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			TopicWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Exception error indicating endpoint disabled.
#[derive(Debug, Default)]
pub struct EndpointDisabledException {
	/// Message for endpoint disabled.
	pub message: string,
}

/// Parse EndpointDisabledException from XML
struct EndpointDisabledExceptionParser;
impl EndpointDisabledExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EndpointDisabledException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = EndpointDisabledException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write EndpointDisabledException contents to a SignedRequest
struct EndpointDisabledExceptionWriter;
impl EndpointDisabledExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &EndpointDisabledException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for GetTopicAttributes action.
#[derive(Debug, Default)]
pub struct GetTopicAttributesInput {
	/// The ARN of the topic whose properties you want to get.
	pub topic_arn: topicARN,
}

/// Parse GetTopicAttributesInput from XML
struct GetTopicAttributesInputParser;
impl GetTopicAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTopicAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetTopicAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetTopicAttributesInput contents to a SignedRequest
struct GetTopicAttributesInputWriter;
impl GetTopicAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetTopicAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub type account = String;
/// Parse account from XML
struct accountParser;
impl accountParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<account, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write account contents to a SignedRequest
struct accountWriter;
impl accountWriter {
	fn write_params(params: &mut Params, name: &str, obj: &account) {
		params.put(name, obj);
	}
}
pub type endpoint = String;
/// Parse endpoint from XML
struct endpointParser;
impl endpointParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<endpoint, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write endpoint contents to a SignedRequest
struct endpointWriter;
impl endpointWriter {
	fn write_params(params: &mut Params, name: &str, obj: &endpoint) {
		params.put(name, obj);
	}
}
pub type subscriptionARN = String;
/// Parse subscriptionARN from XML
struct subscriptionARNParser;
impl subscriptionARNParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<subscriptionARN, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write subscriptionARN contents to a SignedRequest
struct subscriptionARNWriter;
impl subscriptionARNWriter {
	fn write_params(params: &mut Params, name: &str, obj: &subscriptionARN) {
		params.put(name, obj);
	}
}
/// Response for ListSubscriptionsByTopic action.
#[derive(Debug, Default)]
pub struct ListSubscriptionsByTopicResponse {
	/// Token to pass along to the next `ListSubscriptionsByTopic` request. This
	/// element is returned if there are more subscriptions to retrieve.
	pub next_token: nextToken,
	/// A list of subscriptions.
	pub subscriptions: SubscriptionsList,
}

/// Parse ListSubscriptionsByTopicResponse from XML
struct ListSubscriptionsByTopicResponseParser;
impl ListSubscriptionsByTopicResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsByTopicResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSubscriptionsByTopicResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(nextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "Subscription" {
				obj.subscriptions = try!(SubscriptionsListParser::parse_xml("Subscription", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSubscriptionsByTopicResponse contents to a SignedRequest
struct ListSubscriptionsByTopicResponseWriter;
impl ListSubscriptionsByTopicResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsByTopicResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		SubscriptionsListWriter::write_params(params, &(prefix.to_string() + "Subscription"), &obj.subscriptions);
	}
}
pub type MapStringToString = HashMap<String,String>;
/// Parse MapStringToString from XML
struct MapStringToStringParser;
impl MapStringToStringParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MapStringToString, XmlParseError> {
		let mut obj = HashMap::new();
		while try!(peek_at_name(stack)) == tag_name {
			try!(start_element(tag_name, stack));
			let key = try!(StringParser::parse_xml("String", stack));
			let value = try!(StringParser::parse_xml("String", stack));
			obj.insert(key, value);
			try!(end_element(tag_name, stack));
		}
		Ok(obj)
	}
}
/// Write MapStringToString contents to a SignedRequest
struct MapStringToStringWriter;
impl MapStringToStringWriter {
	fn write_params(params: &mut Params, name: &str, obj: &MapStringToString) {
		let mut index = 1;
		for (key,value) in obj {
			let prefix = &format!("{}.{}", name, index);
			StringWriter::write_params(params, &format!("{}.{}", prefix, "String"), &key);
			StringWriter::write_params(params, &format!("{}.{}", prefix, "String"), &value);
			index += 1;
		}
	}
}
/// Response for ListEndpointsByPlatformApplication action.
#[derive(Debug, Default)]
pub struct ListEndpointsByPlatformApplicationResponse {
	/// Endpoints returned for ListEndpointsByPlatformApplication action.
	pub endpoints: ListOfEndpoints,
	/// NextToken string is returned when calling ListEndpointsByPlatformApplication
	/// action if additional records are available after the first page results.
	pub next_token: String,
}

/// Parse ListEndpointsByPlatformApplicationResponse from XML
struct ListEndpointsByPlatformApplicationResponseParser;
impl ListEndpointsByPlatformApplicationResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEndpointsByPlatformApplicationResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListEndpointsByPlatformApplicationResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Endpoint" {
				obj.endpoints = try!(ListOfEndpointsParser::parse_xml("Endpoint", stack));
				continue;
			}
			if current_name == "NextToken" {
				obj.next_token = try!(StringParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListEndpointsByPlatformApplicationResponse contents to a SignedRequest
struct ListEndpointsByPlatformApplicationResponseWriter;
impl ListEndpointsByPlatformApplicationResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListEndpointsByPlatformApplicationResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ListOfEndpointsWriter::write_params(params, &(prefix.to_string() + "Endpoint"), &obj.endpoints);
		StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
#[derive(Debug, Default)]
pub struct AddPermissionInput {
	/// The action you want to allow for the specified principal(s).
	/// Valid values: any Amazon SNS action name.
	pub action_name: ActionsList,
	/// The AWS account IDs of the users (principals) who will be given access to the
	/// specified actions. The users must have AWS accounts, but do not need to be
	/// signed up for this service.
	pub aws_account_id: DelegatesList,
	/// The ARN of the topic whose access control policy you wish to modify.
	pub topic_arn: topicARN,
	/// A unique identifier for the new policy statement.
	pub label: label,
}

/// Parse AddPermissionInput from XML
struct AddPermissionInputParser;
impl AddPermissionInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddPermissionInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = AddPermissionInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "action" {
				obj.action_name = try!(ActionsListParser::parse_xml("action", stack));
				continue;
			}
			if current_name == "delegate" {
				obj.aws_account_id = try!(DelegatesListParser::parse_xml("delegate", stack));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			if current_name == "Label" {
				obj.label = try!(labelParser::parse_xml("Label", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write AddPermissionInput contents to a SignedRequest
struct AddPermissionInputWriter;
impl AddPermissionInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &AddPermissionInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActionsListWriter::write_params(params, &(prefix.to_string() + "action"), &obj.action_name);
		DelegatesListWriter::write_params(params, &(prefix.to_string() + "delegate"), &obj.aws_account_id);
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
		labelWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
#[derive(Debug, Default)]
pub struct ListTopicsInput {
	/// Token returned by the previous `ListTopics` request.
	pub next_token: nextToken,
}

/// Parse ListTopicsInput from XML
struct ListTopicsInputParser;
impl ListTopicsInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListTopicsInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListTopicsInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(nextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListTopicsInput contents to a SignedRequest
struct ListTopicsInputWriter;
impl ListTopicsInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListTopicsInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Input for Publish action.
#[derive(Debug, Default)]
pub struct PublishInput {
	/// Set `MessageStructure` to `json` if you want to send a different message for
	/// each protocol. For example, using one publish action, you can send a short
	/// message to your SMS subscribers and a longer message to your email
	/// subscribers. If you set `MessageStructure` to `json`, the value of the
	/// `Message` parameter must:
	///   * be a syntactically valid JSON object; and
	///   * contain at least a top-level JSON key of "default" with a value that is a string.
	/// You can define other top-level keys that define the message you want to send
	/// to a specific transport protocol (e.g., "http").
	/// For information about sending different messages for each protocol using the
	/// AWS Management Console, go to [Create Different Messages for Each
	/// Protocol](http://docs.aws.amazon.com/sns/latest/gsg/Publish.html#sns-message-
	/// formatting-by-protocol) in the _Amazon Simple Notification Service Getting
	/// Started Guide_.
	/// Valid value: `json`
	pub message_structure: Option<messageStructure>,
	/// Either TopicArn or EndpointArn, but not both.
	pub target_arn: Option<String>,
	/// The message you want to send to the topic.
	/// If you want to send the same message to all transport protocols, include the
	/// text of the message as a String value.
	/// If you want to send different messages for each transport protocol, set the
	/// value of the `MessageStructure` parameter to `json` and use a JSON object for
	/// the `Message` parameter. See the Examples section for the format of the JSON
	/// object.
	/// Constraints: Messages must be UTF-8 encoded strings at most 256 KB in size
	/// (262144 bytes, not 262144 characters).
	/// JSON-specific constraints:
	///   * Keys in the JSON object that correspond to supported transport protocols must have simple JSON string values. 
	///   * The values will be parsed (unescaped) before they are used in outgoing messages.
	///   * Outbound notifications are JSON encoded (meaning that the characters will be reescaped for sending).
	///   * Values have a minimum length of 0 (the empty string, "", is allowed).
	///   * Values have a maximum length bounded by the overall message size (so, including multiple protocols may limit message sizes).
	///   * Non-string values will cause the key to be ignored.
	///   * Keys that do not correspond to supported transport protocols are ignored.
	///   * Duplicate keys are not allowed.
	///   * Failure to parse or validate any key or value in the message will cause the `Publish` call to return an error (no partial delivery).
	pub message: message,
	/// Optional parameter to be used as the "Subject" line when the message is
	/// delivered to email endpoints. This field will also be included, if present, in
	/// the standard JSON messages delivered to other endpoints.
	/// Constraints: Subjects must be ASCII text that begins with a letter, number, or
	/// punctuation mark; must not include line breaks or control characters; and must
	/// be less than 100 characters long.
	pub subject: Option<subject>,
	/// The topic you want to publish to.
	pub topic_arn: Option<topicARN>,
	/// Message attributes for Publish action.
	pub message_attributes: Option<MessageAttributeMap>,
}

/// Parse PublishInput from XML
struct PublishInputParser;
impl PublishInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PublishInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = PublishInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "MessageStructure" {
				obj.message_structure = Some(try!(messageStructureParser::parse_xml("MessageStructure", stack)));
				continue;
			}
			if current_name == "TargetArn" {
				obj.target_arn = Some(try!(StringParser::parse_xml("TargetArn", stack)));
				continue;
			}
			if current_name == "Message" {
				obj.message = try!(messageParser::parse_xml("Message", stack));
				continue;
			}
			if current_name == "Subject" {
				obj.subject = Some(try!(subjectParser::parse_xml("Subject", stack)));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = Some(try!(topicARNParser::parse_xml("TopicArn", stack)));
				continue;
			}
			if current_name == "MessageAttributes" {
				obj.message_attributes = Some(try!(MessageAttributeMapParser::parse_xml("MessageAttributes", stack)));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write PublishInput contents to a SignedRequest
struct PublishInputWriter;
impl PublishInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &PublishInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.message_structure {
			messageStructureWriter::write_params(params, &(prefix.to_string() + "MessageStructure"), obj);
		}
		if let Some(ref obj) = obj.target_arn {
			StringWriter::write_params(params, &(prefix.to_string() + "TargetArn"), obj);
		}
		messageWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.message);
		if let Some(ref obj) = obj.subject {
			subjectWriter::write_params(params, &(prefix.to_string() + "Subject"), obj);
		}
		if let Some(ref obj) = obj.topic_arn {
			topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), obj);
		}
		if let Some(ref obj) = obj.message_attributes {
			MessageAttributeMapWriter::write_params(params, &(prefix.to_string() + "MessageAttributes"), obj);
		}
	}
}
/// A wrapper type for the topic's Amazon Resource Name (ARN). To retrieve a
/// topic's attributes, use `GetTopicAttributes`.
#[derive(Debug, Default)]
pub struct Topic {
	/// The topic's ARN.
	pub topic_arn: topicARN,
}

/// Parse Topic from XML
struct TopicParser;
impl TopicParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Topic, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Topic::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Topic contents to a SignedRequest
struct TopicWriter;
impl TopicWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Topic) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
/// Response for ListPlatformApplications action.
#[derive(Debug, Default)]
pub struct ListPlatformApplicationsResponse {
	/// NextToken string is returned when calling ListPlatformApplications action if
	/// additional records are available after the first page results.
	pub next_token: String,
	/// Platform applications returned when calling ListPlatformApplications action.
	pub platform_applications: ListOfPlatformApplications,
}

/// Parse ListPlatformApplicationsResponse from XML
struct ListPlatformApplicationsResponseParser;
impl ListPlatformApplicationsResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPlatformApplicationsResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListPlatformApplicationsResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(StringParser::parse_xml("NextToken", stack));
				continue;
			}
			if current_name == "PlatformApplication" {
				obj.platform_applications = try!(ListOfPlatformApplicationsParser::parse_xml("PlatformApplication", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListPlatformApplicationsResponse contents to a SignedRequest
struct ListPlatformApplicationsResponseWriter;
impl ListPlatformApplicationsResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListPlatformApplicationsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		ListOfPlatformApplicationsWriter::write_params(params, &(prefix.to_string() + "PlatformApplication"), &obj.platform_applications);
	}
}
/// Input for SetSubscriptionAttributes action.
#[derive(Debug, Default)]
pub struct SetSubscriptionAttributesInput {
	/// The name of the attribute you want to set. Only a subset of the subscriptions
	/// attributes are mutable.
	/// Valid values: `DeliveryPolicy` | `RawMessageDelivery`
	pub attribute_name: attributeName,
	/// The new value for the attribute in JSON format.
	pub attribute_value: Option<attributeValue>,
	/// The ARN of the subscription to modify.
	pub subscription_arn: subscriptionARN,
}

/// Parse SetSubscriptionAttributesInput from XML
struct SetSubscriptionAttributesInputParser;
impl SetSubscriptionAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetSubscriptionAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetSubscriptionAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AttributeName" {
				obj.attribute_name = try!(attributeNameParser::parse_xml("AttributeName", stack));
				continue;
			}
			if current_name == "AttributeValue" {
				obj.attribute_value = Some(try!(attributeValueParser::parse_xml("AttributeValue", stack)));
				continue;
			}
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetSubscriptionAttributesInput contents to a SignedRequest
struct SetSubscriptionAttributesInputWriter;
impl SetSubscriptionAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetSubscriptionAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		attributeNameWriter::write_params(params, &(prefix.to_string() + "AttributeName"), &obj.attribute_name);
		if let Some(ref obj) = obj.attribute_value {
			attributeValueWriter::write_params(params, &(prefix.to_string() + "AttributeValue"), obj);
		}
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type MessageAttributeMap = HashMap<String,MessageAttributeValue>;
/// Parse MessageAttributeMap from XML
struct MessageAttributeMapParser;
impl MessageAttributeMapParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeMap, XmlParseError> {
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
/// Input for DeletePlatformApplication action.
#[derive(Debug, Default)]
pub struct DeletePlatformApplicationInput {
	/// PlatformApplicationArn of platform application object to delete.
	pub platform_application_arn: String,
}

/// Parse DeletePlatformApplicationInput from XML
struct DeletePlatformApplicationInputParser;
impl DeletePlatformApplicationInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletePlatformApplicationInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = DeletePlatformApplicationInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "PlatformApplicationArn" {
				obj.platform_application_arn = try!(StringParser::parse_xml("PlatformApplicationArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write DeletePlatformApplicationInput contents to a SignedRequest
struct DeletePlatformApplicationInputWriter;
impl DeletePlatformApplicationInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &DeletePlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type delegate = String;
/// Parse delegate from XML
struct delegateParser;
impl delegateParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<delegate, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write delegate contents to a SignedRequest
struct delegateWriter;
impl delegateWriter {
	fn write_params(params: &mut Params, name: &str, obj: &delegate) {
		params.put(name, obj);
	}
}
/// Response for GetPlatformApplicationAttributes action.
#[derive(Debug, Default)]
pub struct GetPlatformApplicationAttributesResponse {
	/// Attributes include the following:
	///   * `EventEndpointCreated` \-- Topic ARN to which EndpointCreated event notifications should be sent.
	///   * `EventEndpointDeleted` \-- Topic ARN to which EndpointDeleted event notifications should be sent.
	///   * `EventEndpointUpdated` \-- Topic ARN to which EndpointUpdate event notifications should be sent.
	///   * `EventDeliveryFailure` \-- Topic ARN to which DeliveryFailure event notifications should be sent upon Direct Publish delivery failure (permanent) to one of the application's endpoints.
	pub attributes: MapStringToString,
}

/// Parse GetPlatformApplicationAttributesResponse from XML
struct GetPlatformApplicationAttributesResponseParser;
impl GetPlatformApplicationAttributesResponseParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPlatformApplicationAttributesResponse, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = GetPlatformApplicationAttributesResponse::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Attributes" {
				obj.attributes = try!(MapStringToStringParser::parse_xml("Attributes", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write GetPlatformApplicationAttributesResponse contents to a SignedRequest
struct GetPlatformApplicationAttributesResponseWriter;
impl GetPlatformApplicationAttributesResponseWriter {
	fn write_params(params: &mut Params, name: &str, obj: &GetPlatformApplicationAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for ListSubscriptions action.
#[derive(Debug, Default)]
pub struct ListSubscriptionsInput {
	/// Token returned by the previous `ListSubscriptions` request.
	pub next_token: nextToken,
}

/// Parse ListSubscriptionsInput from XML
struct ListSubscriptionsInputParser;
impl ListSubscriptionsInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = ListSubscriptionsInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "NextToken" {
				obj.next_token = try!(nextTokenParser::parse_xml("NextToken", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write ListSubscriptionsInput contents to a SignedRequest
struct ListSubscriptionsInputWriter;
impl ListSubscriptionsInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Indicates an internal service error.
#[derive(Debug, Default)]
pub struct InternalErrorException {
	pub message: string,
}

/// Parse InternalErrorException from XML
struct InternalErrorExceptionParser;
impl InternalErrorExceptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InternalErrorException, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = InternalErrorException::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "message" {
				obj.message = try!(stringParser::parse_xml("message", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write InternalErrorException contents to a SignedRequest
struct InternalErrorExceptionWriter;
impl InternalErrorExceptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &InternalErrorException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// A wrapper type for the attributes of an Amazon SNS subscription.
#[derive(Debug, Default)]
pub struct Subscription {
	/// The subscription's owner.
	pub owner: account,
	/// The subscription's endpoint (format depends on the protocol).
	pub endpoint: endpoint,
	/// The subscription's protocol.
	pub protocol: protocol,
	/// The ARN of the subscription's topic.
	pub topic_arn: topicARN,
	/// The subscription's ARN.
	pub subscription_arn: subscriptionARN,
}

/// Parse Subscription from XML
struct SubscriptionParser;
impl SubscriptionParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Subscription, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = Subscription::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Owner" {
				obj.owner = try!(accountParser::parse_xml("Owner", stack));
				continue;
			}
			if current_name == "Endpoint" {
				obj.endpoint = try!(endpointParser::parse_xml("Endpoint", stack));
				continue;
			}
			if current_name == "Protocol" {
				obj.protocol = try!(protocolParser::parse_xml("Protocol", stack));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			if current_name == "SubscriptionArn" {
				obj.subscription_arn = try!(subscriptionARNParser::parse_xml("SubscriptionArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write Subscription contents to a SignedRequest
struct SubscriptionWriter;
impl SubscriptionWriter {
	fn write_params(params: &mut Params, name: &str, obj: &Subscription) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		accountWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
		endpointWriter::write_params(params, &(prefix.to_string() + "Endpoint"), &obj.endpoint);
		protocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
/// Input for SetTopicAttributes action.
#[derive(Debug, Default)]
pub struct SetTopicAttributesInput {
	/// The name of the attribute you want to set. Only a subset of the topic's
	/// attributes are mutable.
	/// Valid values: `Policy` | `DisplayName` | `DeliveryPolicy`
	pub attribute_name: attributeName,
	/// The new value for the attribute.
	pub attribute_value: Option<attributeValue>,
	/// The ARN of the topic to modify.
	pub topic_arn: topicARN,
}

/// Parse SetTopicAttributesInput from XML
struct SetTopicAttributesInputParser;
impl SetTopicAttributesInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetTopicAttributesInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SetTopicAttributesInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "AttributeName" {
				obj.attribute_name = try!(attributeNameParser::parse_xml("AttributeName", stack));
				continue;
			}
			if current_name == "AttributeValue" {
				obj.attribute_value = Some(try!(attributeValueParser::parse_xml("AttributeValue", stack)));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SetTopicAttributesInput contents to a SignedRequest
struct SetTopicAttributesInputWriter;
impl SetTopicAttributesInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SetTopicAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		attributeNameWriter::write_params(params, &(prefix.to_string() + "AttributeName"), &obj.attribute_name);
		if let Some(ref obj) = obj.attribute_value {
			attributeValueWriter::write_params(params, &(prefix.to_string() + "AttributeValue"), obj);
		}
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub type token = String;
/// Parse token from XML
struct tokenParser;
impl tokenParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<token, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write token contents to a SignedRequest
struct tokenWriter;
impl tokenWriter {
	fn write_params(params: &mut Params, name: &str, obj: &token) {
		params.put(name, obj);
	}
}
/// Input for Subscribe action.
#[derive(Debug, Default)]
pub struct SubscribeInput {
	/// The endpoint that you want to receive notifications. Endpoints vary by
	/// protocol:
	///   * For the `http` protocol, the endpoint is an URL beginning with "http://"
	///   * For the `https` protocol, the endpoint is a URL beginning with "https://"
	///   * For the `email` protocol, the endpoint is an email address
	///   * For the `email-json` protocol, the endpoint is an email address
	///   * For the `sms` protocol, the endpoint is a phone number of an SMS-enabled device
	///   * For the `sqs` protocol, the endpoint is the ARN of an Amazon SQS queue
	///   * For the `application` protocol, the endpoint is the EndpointArn of a mobile app and device.
	pub endpoint: Option<endpoint>,
	/// The protocol you want to use. Supported protocols include:
	///   * `http` \-- delivery of JSON-encoded message via HTTP POST
	///   * `https` \-- delivery of JSON-encoded message via HTTPS POST
	///   * `email` \-- delivery of message via SMTP
	///   * `email-json` \-- delivery of JSON-encoded message via SMTP
	///   * `sms` \-- delivery of message via SMS
	///   * `sqs` \-- delivery of JSON-encoded message to an Amazon SQS queue
	///   * `application` \-- delivery of JSON-encoded message to an EndpointArn for a mobile app and device.
	pub protocol: protocol,
	/// The ARN of the topic you want to subscribe to.
	pub topic_arn: topicARN,
}

/// Parse SubscribeInput from XML
struct SubscribeInputParser;
impl SubscribeInputParser {
	fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscribeInput, XmlParseError> {
		try!(start_element(tag_name, stack));
		let mut obj = SubscribeInput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
			if current_name == "Endpoint" {
				obj.endpoint = Some(try!(endpointParser::parse_xml("Endpoint", stack)));
				continue;
			}
			if current_name == "Protocol" {
				obj.protocol = try!(protocolParser::parse_xml("Protocol", stack));
				continue;
			}
			if current_name == "TopicArn" {
				obj.topic_arn = try!(topicARNParser::parse_xml("TopicArn", stack));
				continue;
			}
			break;
		}
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write SubscribeInput contents to a SignedRequest
struct SubscribeInputWriter;
impl SubscribeInputWriter {
	fn write_params(params: &mut Params, name: &str, obj: &SubscribeInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.endpoint {
			endpointWriter::write_params(params, &(prefix.to_string() + "Endpoint"), obj);
		}
		protocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub struct SNSClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> SNSClient<'a> { 
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> SNSClient<'a> {
		SNSClient { creds: creds, region: region }
	}
	/// Lists the platform application objects for the supported push notification
	/// services, such as APNS and GCM. The results for `ListPlatformApplications` are
	/// paginated and return a limited list of applications, up to 100. If additional
	/// records are available after the first page results, then a NextToken string
	/// will be returned. To receive the next page, you call
	/// `ListPlatformApplications` using the NextToken string received from the
	/// previous call. When there are no more records to return, NextToken will be
	/// null. For more information, see [Using Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn list_platform_applications(&self, input: &ListPlatformApplicationsInput) -> Result<ListPlatformApplicationsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListPlatformApplications");
		ListPlatformApplicationsInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListPlatformApplicationsResponseParser::parse_xml("ListPlatformApplicationsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sets the attributes of the platform application object for the supported push
	/// notification services, such as APNS and GCM. For more information, see [Using
	/// Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn set_platform_application_attributes(&self, input: &SetPlatformApplicationAttributesInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetPlatformApplicationAttributes");
		SetPlatformApplicationAttributesInputWriter::write_params(&mut params, "", &input);
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
	/// Returns a list of the subscriptions to a specific topic. Each call returns a
	/// limited list of subscriptions, up to 100. If there are more subscriptions, a
	/// `NextToken` is also returned. Use the `NextToken` parameter in a new
	/// `ListSubscriptionsByTopic` call to get further results.
	pub fn list_subscriptions_by_topic(&self, input: &ListSubscriptionsByTopicInput) -> Result<ListSubscriptionsByTopicResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSubscriptionsByTopic");
		ListSubscriptionsByTopicInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListSubscriptionsByTopicResponseParser::parse_xml("ListSubscriptionsByTopicResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Returns a list of the requester's subscriptions. Each call returns a limited
	/// list of subscriptions, up to 100. If there are more subscriptions, a
	/// `NextToken` is also returned. Use the `NextToken` parameter in a new
	/// `ListSubscriptions` call to get further results.
	pub fn list_subscriptions(&self, input: &ListSubscriptionsInput) -> Result<ListSubscriptionsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSubscriptions");
		ListSubscriptionsInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListSubscriptionsResponseParser::parse_xml("ListSubscriptionsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Verifies an endpoint owner's intent to receive messages by validating the
	/// token sent to the endpoint by an earlier `Subscribe` action. If the token is
	/// valid, the action creates a new subscription and returns its Amazon Resource
	/// Name (ARN). This call requires an AWS signature only when the
	/// `AuthenticateOnUnsubscribe` flag is set to "true".
	pub fn confirm_subscription(&self, input: &ConfirmSubscriptionInput) -> Result<ConfirmSubscriptionResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ConfirmSubscription");
		ConfirmSubscriptionInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ConfirmSubscriptionResponseParser::parse_xml("ConfirmSubscriptionResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Allows a topic owner to set an attribute of the topic to a new value.
	pub fn set_topic_attributes(&self, input: &SetTopicAttributesInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetTopicAttributes");
		SetTopicAttributesInputWriter::write_params(&mut params, "", &input);
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
	/// Sets the attributes for an endpoint for a device on one of the supported push
	/// notification services, such as GCM and APNS. For more information, see [Using
	/// Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn set_endpoint_attributes(&self, input: &SetEndpointAttributesInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetEndpointAttributes");
		SetEndpointAttributesInputWriter::write_params(&mut params, "", &input);
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
	/// Adds a statement to a topic's access control policy, granting access for the
	/// specified AWS accounts to the specified actions.
	pub fn add_permission(&self, input: &AddPermissionInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "AddPermission");
		AddPermissionInputWriter::write_params(&mut params, "", &input);
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
	/// Deletes a subscription. If the subscription requires authentication for
	/// deletion, only the owner of the subscription or the topic's owner can
	/// unsubscribe, and an AWS signature is required. If the `Unsubscribe` call does
	/// not require authentication and the requester is not the subscription owner, a
	/// final cancellation message is delivered to the endpoint, so that the endpoint
	/// owner can easily resubscribe to the topic if the `Unsubscribe` request was
	/// unintended.
	pub fn unsubscribe(&self, input: &UnsubscribeInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "Unsubscribe");
		UnsubscribeInputWriter::write_params(&mut params, "", &input);
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
	/// Returns all of the properties of a subscription.
	pub fn get_subscription_attributes(&self, input: &GetSubscriptionAttributesInput) -> Result<GetSubscriptionAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetSubscriptionAttributes");
		GetSubscriptionAttributesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetSubscriptionAttributesResponseParser::parse_xml("GetSubscriptionAttributesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Creates an endpoint for a device and mobile app on one of the supported push
	/// notification services, such as GCM and APNS. `CreatePlatformEndpoint` requires
	/// the PlatformApplicationArn that is returned from `CreatePlatformApplication`.
	/// The EndpointArn that is returned when using `CreatePlatformEndpoint` can then
	/// be used by the `Publish` action to send a message to a mobile app or by the
	/// `Subscribe` action for subscription to a topic. The `CreatePlatformEndpoint`
	/// action is idempotent, so if the requester already owns an endpoint with the
	/// same device token and attributes, that endpoint's ARN is returned without
	/// creating a new endpoint. For more information, see [Using Amazon SNS Mobile
	/// Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	/// When using `CreatePlatformEndpoint` with Baidu, two attributes must be
	/// provided: ChannelId and UserId. The token field must also contain the
	/// ChannelId. For more information, see [Creating an Amazon SNS Endpoint for Baid
	/// u](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePushBaiduEndpoint.html).
	pub fn create_platform_endpoint(&self, input: &CreatePlatformEndpointInput) -> Result<CreateEndpointResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePlatformEndpoint");
		CreatePlatformEndpointInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateEndpointResponseParser::parse_xml("CreateEndpointResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Removes a statement from a topic's access control policy.
	pub fn remove_permission(&self, input: &RemovePermissionInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "RemovePermission");
		RemovePermissionInputWriter::write_params(&mut params, "", &input);
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
	/// Creates a topic to which notifications can be published. Users can create at
	/// most 3000 topics. For more information, see
	/// [http://aws.amazon.com/sns](http://aws.amazon.com/sns/). This action is
	/// idempotent, so if the requester already owns a topic with the specified name,
	/// that topic's ARN is returned without creating a new topic.
	pub fn create_topic(&self, input: &CreateTopicInput) -> Result<CreateTopicResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateTopic");
		CreateTopicInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreateTopicResponseParser::parse_xml("CreateTopicResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Allows a subscription owner to set an attribute of the topic to a new value.
	pub fn set_subscription_attributes(&self, input: &SetSubscriptionAttributesInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "SetSubscriptionAttributes");
		SetSubscriptionAttributesInputWriter::write_params(&mut params, "", &input);
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
	/// Creates a platform application object for one of the supported push
	/// notification services, such as APNS and GCM, to which devices and mobile apps
	/// may register. You must specify PlatformPrincipal and PlatformCredential
	/// attributes when using the `CreatePlatformApplication` action. The
	/// PlatformPrincipal is received from the notification service. For
	/// APNS/APNS_SANDBOX, PlatformPrincipal is "SSL certificate". For GCM,
	/// PlatformPrincipal is not applicable. For ADM, PlatformPrincipal is "client
	/// id". The PlatformCredential is also received from the notification service.
	/// For APNS/APNS_SANDBOX, PlatformCredential is "private key". For GCM,
	/// PlatformCredential is "API key". For ADM, PlatformCredential is "client
	/// secret". The PlatformApplicationArn that is returned when using
	/// `CreatePlatformApplication` is then used as an attribute for the
	/// `CreatePlatformEndpoint` action. For more information, see [Using Amazon SNS
	/// Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn create_platform_application(&self, input: &CreatePlatformApplicationInput) -> Result<CreatePlatformApplicationResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePlatformApplication");
		CreatePlatformApplicationInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(CreatePlatformApplicationResponseParser::parse_xml("CreatePlatformApplicationResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the attributes of the platform application object for the supported
	/// push notification services, such as APNS and GCM. For more information, see
	/// [Using Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn get_platform_application_attributes(&self, input: &GetPlatformApplicationAttributesInput) -> Result<GetPlatformApplicationAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetPlatformApplicationAttributes");
		GetPlatformApplicationAttributesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetPlatformApplicationAttributesResponseParser::parse_xml("GetPlatformApplicationAttributesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Sends a message to all of a topic's subscribed endpoints. When a `messageId`
	/// is returned, the message has been saved and Amazon SNS will attempt to deliver
	/// it to the topic's subscribers shortly. The format of the outgoing message to
	/// each subscribed endpoint depends on the notification protocol selected.
	/// To use the `Publish` action for sending a message to a mobile endpoint, such
	/// as an app on a Kindle device or mobile phone, you must specify the
	/// EndpointArn. The EndpointArn is returned when making a call with the
	/// `CreatePlatformEndpoint` action. The second example below shows a request and
	/// response for publishing to a mobile endpoint.
	pub fn publish(&self, input: &PublishInput) -> Result<PublishResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "Publish");
		PublishInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(PublishResponseParser::parse_xml("PublishResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Lists the endpoints and endpoint attributes for devices in a supported push
	/// notification service, such as GCM and APNS. The results for
	/// `ListEndpointsByPlatformApplication` are paginated and return a limited list
	/// of endpoints, up to 100. If additional records are available after the first
	/// page results, then a NextToken string will be returned. To receive the next
	/// page, you call `ListEndpointsByPlatformApplication` again using the NextToken
	/// string received from the previous call. When there are no more records to
	/// return, NextToken will be null. For more information, see [Using Amazon SNS
	/// Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn list_endpoints_by_platform_application(&self, input: &ListEndpointsByPlatformApplicationInput) -> Result<ListEndpointsByPlatformApplicationResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListEndpointsByPlatformApplication");
		ListEndpointsByPlatformApplicationInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListEndpointsByPlatformApplicationResponseParser::parse_xml("ListEndpointsByPlatformApplicationResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Retrieves the endpoint attributes for a device on one of the supported push
	/// notification services, such as GCM and APNS. For more information, see [Using
	/// Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn get_endpoint_attributes(&self, input: &GetEndpointAttributesInput) -> Result<GetEndpointAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetEndpointAttributes");
		GetEndpointAttributesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetEndpointAttributesResponseParser::parse_xml("GetEndpointAttributesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes the endpoint from Amazon SNS. This action is idempotent. For more
	/// information, see [Using Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn delete_endpoint(&self, input: &DeleteEndpointInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteEndpoint");
		DeleteEndpointInputWriter::write_params(&mut params, "", &input);
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
	/// Deletes a platform application object for one of the supported push
	/// notification services, such as APNS and GCM. For more information, see [Using
	/// Amazon SNS Mobile Push
	/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
	pub fn delete_platform_application(&self, input: &DeletePlatformApplicationInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeletePlatformApplication");
		DeletePlatformApplicationInputWriter::write_params(&mut params, "", &input);
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
	/// Returns all of the properties of a topic. Topic properties returned might
	/// differ based on the authorization of the user.
	pub fn get_topic_attributes(&self, input: &GetTopicAttributesInput) -> Result<GetTopicAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "GetTopicAttributes");
		GetTopicAttributesInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(GetTopicAttributesResponseParser::parse_xml("GetTopicAttributesResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Prepares to subscribe an endpoint by sending the endpoint a confirmation
	/// message. To actually create a subscription, the endpoint owner must call the
	/// `ConfirmSubscription` action with the token from the confirmation message.
	/// Confirmation tokens are valid for three days.
	pub fn subscribe(&self, input: &SubscribeInput) -> Result<SubscribeResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "Subscribe");
		SubscribeInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(SubscribeResponseParser::parse_xml("SubscribeResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
	/// Deletes a topic and all its subscriptions. Deleting a topic might prevent some
	/// messages previously sent to the topic from being delivered to subscribers.
	/// This action is idempotent, so deleting a topic that does not exist does not
	/// result in an error.
	pub fn delete_topic(&self, input: &DeleteTopicInput) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteTopic");
		DeleteTopicInputWriter::write_params(&mut params, "", &input);
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
	/// Returns a list of the requester's topics. Each call returns a limited list of
	/// topics, up to 100. If there are more topics, a `NextToken` is also returned.
	/// Use the `NextToken` parameter in a new `ListTopics` call to get further
	/// results.
	pub fn list_topics(&self, input: &ListTopicsInput) -> Result<ListTopicsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", &self.region, "/");
		let mut params = Params::new();
		params.put("Action", "ListTopics");
		ListTopicsInputWriter::write_params(&mut params, "", &input);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(&self.creds));
		let mut reader = EventReader::new(output.as_bytes());
		let mut stack = reader.events().peekable();
		stack.next();
		stack.next();
		match status {
			200 => { 
				Ok(try!(ListTopicsResponseParser::parse_xml("ListTopicsResponse", &mut stack)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
