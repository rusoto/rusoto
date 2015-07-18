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

/// Parse a GetSubscriptionAttributesResponse from XML
pub struct GetSubscriptionAttributesResponseParser;
impl GetSubscriptionAttributesResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSubscriptionAttributesResponse, XmlParseError> {
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
/// Write a GetSubscriptionAttributesResponse's contents to a SignedRequest
pub struct GetSubscriptionAttributesResponseWriter;
impl GetSubscriptionAttributesResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetSubscriptionAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		SubscriptionAttributesMapWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
pub type Binary = Vec<u8>;
/// Parse a Binary from XML
pub struct BinaryParser;
impl BinaryParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Binary, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack)).into_bytes();
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a Binary's contents to a SignedRequest
pub struct BinaryWriter;
impl BinaryWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Binary) {
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

/// Parse a MessageAttributeValue from XML
pub struct MessageAttributeValueParser;
impl MessageAttributeValueParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MessageAttributeValue, XmlParseError> {
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
/// Write a MessageAttributeValue's contents to a SignedRequest
pub struct MessageAttributeValueWriter;
impl MessageAttributeValueWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MessageAttributeValue) {
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
/// Deletes a subscription. If the subscription requires authentication for
/// deletion, only the owner of the subscription or the topic's owner can
/// unsubscribe, and an AWS signature is required. If the `Unsubscribe` call does
/// not require authentication and the requester is not the subscription owner, a
/// final cancellation message is delivered to the endpoint, so that the endpoint
/// owner can easily resubscribe to the topic if the `Unsubscribe` request was
/// unintended.
#[derive(Debug, Default)]
pub struct UnsubscribeInput {
	/// The ARN of the subscription to be deleted.
	pub subscription_arn: subscriptionARN,
}

/// Parse a UnsubscribeInput from XML
pub struct UnsubscribeInputParser;
impl UnsubscribeInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<UnsubscribeInput, XmlParseError> {
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
/// Write a UnsubscribeInput's contents to a SignedRequest
pub struct UnsubscribeInputWriter;
impl UnsubscribeInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &UnsubscribeInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type TopicAttributesMap = HashMap<attributeName,attributeValue>;
/// Parse a TopicAttributesMap from XML
pub struct TopicAttributesMapParser;
impl TopicAttributesMapParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicAttributesMap, XmlParseError> {
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
/// Write a TopicAttributesMap's contents to a SignedRequest
pub struct TopicAttributesMapWriter;
impl TopicAttributesMapWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &TopicAttributesMap) {
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
/// Parse a DelegatesList from XML
pub struct DelegatesListParser;
impl DelegatesListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DelegatesList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "delegate" {
			obj.push(try!(delegateParser::parse_xml("delegate", stack)));
		}
		Ok(obj)
	}
}
/// Write a DelegatesList's contents to a SignedRequest
pub struct DelegatesListWriter;
impl DelegatesListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DelegatesList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			delegateWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Input for ListEndpointsByPlatformApplication action.
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
#[derive(Debug, Default)]
pub struct ListEndpointsByPlatformApplicationInput {
	/// NextToken string is used when calling ListEndpointsByPlatformApplication
	/// action to retrieve additional records that are available after the first page
	/// results.
	pub next_token: Option<String>,
	/// PlatformApplicationArn for ListEndpointsByPlatformApplicationInput action.
	pub platform_application_arn: String,
}

/// Parse a ListEndpointsByPlatformApplicationInput from XML
pub struct ListEndpointsByPlatformApplicationInputParser;
impl ListEndpointsByPlatformApplicationInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEndpointsByPlatformApplicationInput, XmlParseError> {
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
/// Write a ListEndpointsByPlatformApplicationInput's contents to a SignedRequest
pub struct ListEndpointsByPlatformApplicationInputWriter;
impl ListEndpointsByPlatformApplicationInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListEndpointsByPlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.next_token {
			StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), obj);
		}
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type SubscriptionAttributesMap = HashMap<attributeName,attributeValue>;
/// Parse a SubscriptionAttributesMap from XML
pub struct SubscriptionAttributesMapParser;
impl SubscriptionAttributesMapParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionAttributesMap, XmlParseError> {
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
/// Write a SubscriptionAttributesMap's contents to a SignedRequest
pub struct SubscriptionAttributesMapWriter;
impl SubscriptionAttributesMapWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SubscriptionAttributesMap) {
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

/// Parse a TopicLimitExceededException from XML
pub struct TopicLimitExceededExceptionParser;
impl TopicLimitExceededExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicLimitExceededException, XmlParseError> {
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
/// Write a TopicLimitExceededException's contents to a SignedRequest
pub struct TopicLimitExceededExceptionWriter;
impl TopicLimitExceededExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &TopicLimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type SubscriptionsList = Vec<Subscription>;
/// Parse a SubscriptionsList from XML
pub struct SubscriptionsListParser;
impl SubscriptionsListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Subscription" {
			obj.push(try!(SubscriptionParser::parse_xml("Subscription", stack)));
		}
		Ok(obj)
	}
}
/// Write a SubscriptionsList's contents to a SignedRequest
pub struct SubscriptionsListWriter;
impl SubscriptionsListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SubscriptionsList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			SubscriptionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
/// Parse a String from XML
pub struct StringParser;
impl StringParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<String, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a String's contents to a SignedRequest
pub struct StringWriter;
impl StringWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &String) {
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

/// Parse a GetEndpointAttributesResponse from XML
pub struct GetEndpointAttributesResponseParser;
impl GetEndpointAttributesResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetEndpointAttributesResponse, XmlParseError> {
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
/// Write a GetEndpointAttributesResponse's contents to a SignedRequest
pub struct GetEndpointAttributesResponseWriter;
impl GetEndpointAttributesResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetEndpointAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for SetPlatformApplicationAttributes action.
/// Sets the attributes of the platform application object for the supported push
/// notification services, such as APNS and GCM. For more information, see [Using
/// Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
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

/// Parse a SetPlatformApplicationAttributesInput from XML
pub struct SetPlatformApplicationAttributesInputParser;
impl SetPlatformApplicationAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetPlatformApplicationAttributesInput, XmlParseError> {
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
/// Write a SetPlatformApplicationAttributesInput's contents to a SignedRequest
pub struct SetPlatformApplicationAttributesInputWriter;
impl SetPlatformApplicationAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SetPlatformApplicationAttributesInput) {
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

/// Parse a ListSubscriptionsResponse from XML
pub struct ListSubscriptionsResponseParser;
impl ListSubscriptionsResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsResponse, XmlParseError> {
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
/// Write a ListSubscriptionsResponse's contents to a SignedRequest
pub struct ListSubscriptionsResponseWriter;
impl ListSubscriptionsResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsResponse) {
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

/// Parse a ListTopicsResponse from XML
pub struct ListTopicsResponseParser;
impl ListTopicsResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListTopicsResponse, XmlParseError> {
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
/// Write a ListTopicsResponse's contents to a SignedRequest
pub struct ListTopicsResponseWriter;
impl ListTopicsResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListTopicsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		TopicsListWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topics);
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
pub type topicName = String;
/// Parse a topicName from XML
pub struct topicNameParser;
impl topicNameParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<topicName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a topicName's contents to a SignedRequest
pub struct topicNameWriter;
impl topicNameWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &topicName) {
		params.put(name, obj);
	}
}
pub type ListOfPlatformApplications = Vec<PlatformApplication>;
/// Parse a ListOfPlatformApplications from XML
pub struct ListOfPlatformApplicationsParser;
impl ListOfPlatformApplicationsParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOfPlatformApplications, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "PlatformApplication" {
			obj.push(try!(PlatformApplicationParser::parse_xml("PlatformApplication", stack)));
		}
		Ok(obj)
	}
}
/// Write a ListOfPlatformApplications's contents to a SignedRequest
pub struct ListOfPlatformApplicationsWriter;
impl ListOfPlatformApplicationsWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListOfPlatformApplications) {
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

/// Parse a SubscriptionLimitExceededException from XML
pub struct SubscriptionLimitExceededExceptionParser;
impl SubscriptionLimitExceededExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscriptionLimitExceededException, XmlParseError> {
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
/// Write a SubscriptionLimitExceededException's contents to a SignedRequest
pub struct SubscriptionLimitExceededExceptionWriter;
impl SubscriptionLimitExceededExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SubscriptionLimitExceededException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type string = String;
/// Parse a string from XML
pub struct stringParser;
impl stringParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<string, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a string's contents to a SignedRequest
pub struct stringWriter;
impl stringWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &string) {
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

/// Parse a GetTopicAttributesResponse from XML
pub struct GetTopicAttributesResponseParser;
impl GetTopicAttributesResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTopicAttributesResponse, XmlParseError> {
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
/// Write a GetTopicAttributesResponse's contents to a SignedRequest
pub struct GetTopicAttributesResponseWriter;
impl GetTopicAttributesResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetTopicAttributesResponse) {
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

/// Parse a CreatePlatformApplicationResponse from XML
pub struct CreatePlatformApplicationResponseParser;
impl CreatePlatformApplicationResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformApplicationResponse, XmlParseError> {
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
/// Write a CreatePlatformApplicationResponse's contents to a SignedRequest
pub struct CreatePlatformApplicationResponseWriter;
impl CreatePlatformApplicationResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformApplicationResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type topicARN = String;
/// Parse a topicARN from XML
pub struct topicARNParser;
impl topicARNParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<topicARN, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a topicARN's contents to a SignedRequest
pub struct topicARNWriter;
impl topicARNWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &topicARN) {
		params.put(name, obj);
	}
}
/// Indicates that the user has been denied access to the requested resource.
#[derive(Debug, Default)]
pub struct AuthorizationErrorException {
	pub message: string,
}

/// Parse a AuthorizationErrorException from XML
pub struct AuthorizationErrorExceptionParser;
impl AuthorizationErrorExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AuthorizationErrorException, XmlParseError> {
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
/// Write a AuthorizationErrorException's contents to a SignedRequest
pub struct AuthorizationErrorExceptionWriter;
impl AuthorizationErrorExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AuthorizationErrorException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for RemovePermission action.
/// Removes a statement from a topic's access control policy.
#[derive(Debug, Default)]
pub struct RemovePermissionInput {
	/// The ARN of the topic whose access control policy you wish to modify.
	pub topic_arn: topicARN,
	/// The unique label of the statement you want to remove.
	pub label: label,
}

/// Parse a RemovePermissionInput from XML
pub struct RemovePermissionInputParser;
impl RemovePermissionInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<RemovePermissionInput, XmlParseError> {
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
/// Write a RemovePermissionInput's contents to a SignedRequest
pub struct RemovePermissionInputWriter;
impl RemovePermissionInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &RemovePermissionInput) {
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

/// Parse a CreateTopicResponse from XML
pub struct CreateTopicResponseParser;
impl CreateTopicResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateTopicResponse, XmlParseError> {
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
/// Write a CreateTopicResponse's contents to a SignedRequest
pub struct CreateTopicResponseWriter;
impl CreateTopicResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreateTopicResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub type ListOfEndpoints = Vec<Endpoint>;
/// Parse a ListOfEndpoints from XML
pub struct ListOfEndpointsParser;
impl ListOfEndpointsParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListOfEndpoints, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Endpoint" {
			obj.push(try!(EndpointParser::parse_xml("Endpoint", stack)));
		}
		Ok(obj)
	}
}
/// Write a ListOfEndpoints's contents to a SignedRequest
pub struct ListOfEndpointsWriter;
impl ListOfEndpointsWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListOfEndpoints) {
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

/// Parse a PlatformApplication from XML
pub struct PlatformApplicationParser;
impl PlatformApplicationParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PlatformApplication, XmlParseError> {
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
/// Write a PlatformApplication's contents to a SignedRequest
pub struct PlatformApplicationWriter;
impl PlatformApplicationWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PlatformApplication) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type protocol = String;
/// Parse a protocol from XML
pub struct protocolParser;
impl protocolParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<protocol, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a protocol's contents to a SignedRequest
pub struct protocolWriter;
impl protocolWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &protocol) {
		params.put(name, obj);
	}
}
pub type label = String;
/// Parse a label from XML
pub struct labelParser;
impl labelParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<label, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a label's contents to a SignedRequest
pub struct labelWriter;
impl labelWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &label) {
		params.put(name, obj);
	}
}
/// Indicates that a request parameter does not comply with the associated
/// constraints.
#[derive(Debug, Default)]
pub struct InvalidParameterException {
	pub message: string,
}

/// Parse a InvalidParameterException from XML
pub struct InvalidParameterExceptionParser;
impl InvalidParameterExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidParameterException, XmlParseError> {
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
/// Write a InvalidParameterException's contents to a SignedRequest
pub struct InvalidParameterExceptionWriter;
impl InvalidParameterExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidParameterException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for CreatePlatformApplication action.
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

/// Parse a CreatePlatformApplicationInput from XML
pub struct CreatePlatformApplicationInputParser;
impl CreatePlatformApplicationInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformApplicationInput, XmlParseError> {
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
/// Write a CreatePlatformApplicationInput's contents to a SignedRequest
pub struct CreatePlatformApplicationInputWriter;
impl CreatePlatformApplicationInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "Platform"), &obj.platform);
		StringWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for CreateTopic action.
/// Creates a topic to which notifications can be published. Users can create at
/// most 3000 topics. For more information, see
/// [http://aws.amazon.com/sns](http://aws.amazon.com/sns/). This action is
/// idempotent, so if the requester already owns a topic with the specified name,
/// that topic's ARN is returned without creating a new topic.
#[derive(Debug, Default)]
pub struct CreateTopicInput {
	/// The name of the topic you want to create.
	/// Constraints: Topic names must be made up of only uppercase and lowercase ASCII
	/// letters, numbers, underscores, and hyphens, and must be between 1 and 256
	/// characters long.
	pub name: topicName,
}

/// Parse a CreateTopicInput from XML
pub struct CreateTopicInputParser;
impl CreateTopicInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateTopicInput, XmlParseError> {
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
/// Write a CreateTopicInput's contents to a SignedRequest
pub struct CreateTopicInputWriter;
impl CreateTopicInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreateTopicInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
	}
}
pub type action = String;
/// Parse a action from XML
pub struct actionParser;
impl actionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<action, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a action's contents to a SignedRequest
pub struct actionWriter;
impl actionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &action) {
		params.put(name, obj);
	}
}
/// Input for DeleteEndpoint action.
/// Deletes the endpoint from Amazon SNS. This action is idempotent. For more
/// information, see [Using Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
#[derive(Debug, Default)]
pub struct DeleteEndpointInput {
	/// EndpointArn of endpoint to delete.
	pub endpoint_arn: String,
}

/// Parse a DeleteEndpointInput from XML
pub struct DeleteEndpointInputParser;
impl DeleteEndpointInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteEndpointInput, XmlParseError> {
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
/// Write a DeleteEndpointInput's contents to a SignedRequest
pub struct DeleteEndpointInputWriter;
impl DeleteEndpointInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteEndpointInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for GetSubscriptionAttributes.
/// Returns all of the properties of a subscription.
#[derive(Debug, Default)]
pub struct GetSubscriptionAttributesInput {
	/// The ARN of the subscription whose properties you want to get.
	pub subscription_arn: subscriptionARN,
}

/// Parse a GetSubscriptionAttributesInput from XML
pub struct GetSubscriptionAttributesInputParser;
impl GetSubscriptionAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetSubscriptionAttributesInput, XmlParseError> {
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
/// Write a GetSubscriptionAttributesInput's contents to a SignedRequest
pub struct GetSubscriptionAttributesInputWriter;
impl GetSubscriptionAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetSubscriptionAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type messageStructure = String;
/// Parse a messageStructure from XML
pub struct messageStructureParser;
impl messageStructureParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<messageStructure, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a messageStructure's contents to a SignedRequest
pub struct messageStructureWriter;
impl messageStructureWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &messageStructure) {
		params.put(name, obj);
	}
}
pub type ActionsList = Vec<action>;
/// Parse a ActionsList from XML
pub struct ActionsListParser;
impl ActionsListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ActionsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "action" {
			obj.push(try!(actionParser::parse_xml("action", stack)));
		}
		Ok(obj)
	}
}
/// Write a ActionsList's contents to a SignedRequest
pub struct ActionsListWriter;
impl ActionsListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ActionsList) {
		let mut index = 1;
		for element in obj.iter() {
			let key = &format!("{}.{}", name, index);
			actionWriter::write_params(params, key, &element);
			index += 1;
		}
	}
}
pub type subject = String;
/// Parse a subject from XML
pub struct subjectParser;
impl subjectParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<subject, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a subject's contents to a SignedRequest
pub struct subjectWriter;
impl subjectWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &subject) {
		params.put(name, obj);
	}
}
/// Response from CreateEndpoint action.
#[derive(Debug, Default)]
pub struct CreateEndpointResponse {
	/// EndpointArn returned from CreateEndpoint action.
	pub endpoint_arn: String,
}

/// Parse a CreateEndpointResponse from XML
pub struct CreateEndpointResponseParser;
impl CreateEndpointResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreateEndpointResponse, XmlParseError> {
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
/// Write a CreateEndpointResponse's contents to a SignedRequest
pub struct CreateEndpointResponseWriter;
impl CreateEndpointResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreateEndpointResponse) {
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

/// Parse a NotFoundException from XML
pub struct NotFoundExceptionParser;
impl NotFoundExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<NotFoundException, XmlParseError> {
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
/// Write a NotFoundException's contents to a SignedRequest
pub struct NotFoundExceptionWriter;
impl NotFoundExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &NotFoundException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for SetEndpointAttributes action.
/// Sets the attributes for an endpoint for a device on one of the supported push
/// notification services, such as GCM and APNS. For more information, see [Using
/// Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
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

/// Parse a SetEndpointAttributesInput from XML
pub struct SetEndpointAttributesInputParser;
impl SetEndpointAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetEndpointAttributesInput, XmlParseError> {
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
/// Write a SetEndpointAttributesInput's contents to a SignedRequest
pub struct SetEndpointAttributesInputWriter;
impl SetEndpointAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SetEndpointAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for GetPlatformApplicationAttributes action.
/// Retrieves the attributes of the platform application object for the supported
/// push notification services, such as APNS and GCM. For more information, see
/// [Using Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
#[derive(Debug, Default)]
pub struct GetPlatformApplicationAttributesInput {
	/// PlatformApplicationArn for GetPlatformApplicationAttributesInput.
	pub platform_application_arn: String,
}

/// Parse a GetPlatformApplicationAttributesInput from XML
pub struct GetPlatformApplicationAttributesInputParser;
impl GetPlatformApplicationAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPlatformApplicationAttributesInput, XmlParseError> {
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
/// Write a GetPlatformApplicationAttributesInput's contents to a SignedRequest
pub struct GetPlatformApplicationAttributesInputWriter;
impl GetPlatformApplicationAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetPlatformApplicationAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type nextToken = String;
/// Parse a nextToken from XML
pub struct nextTokenParser;
impl nextTokenParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<nextToken, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a nextToken's contents to a SignedRequest
pub struct nextTokenWriter;
impl nextTokenWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &nextToken) {
		params.put(name, obj);
	}
}
/// Input for CreatePlatformEndpoint action.
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

/// Parse a CreatePlatformEndpointInput from XML
pub struct CreatePlatformEndpointInputParser;
impl CreatePlatformEndpointInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<CreatePlatformEndpointInput, XmlParseError> {
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
/// Write a CreatePlatformEndpointInput's contents to a SignedRequest
pub struct CreatePlatformEndpointInputWriter;
impl CreatePlatformEndpointInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &CreatePlatformEndpointInput) {
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

/// Parse a Endpoint from XML
pub struct EndpointParser;
impl EndpointParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Endpoint, XmlParseError> {
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
/// Write a Endpoint's contents to a SignedRequest
pub struct EndpointWriter;
impl EndpointWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Endpoint) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
pub type attributeValue = String;
/// Parse a attributeValue from XML
pub struct attributeValueParser;
impl attributeValueParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attributeValue, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a attributeValue's contents to a SignedRequest
pub struct attributeValueWriter;
impl attributeValueWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &attributeValue) {
		params.put(name, obj);
	}
}
/// Input for ListPlatformApplications action.
/// Lists the platform application objects for the supported push notification
/// services, such as APNS and GCM. The results for `ListPlatformApplications` are
/// paginated and return a limited list of applications, up to 100. If additional
/// records are available after the first page results, then a NextToken string
/// will be returned. To receive the next page, you call
/// `ListPlatformApplications` using the NextToken string received from the
/// previous call. When there are no more records to return, NextToken will be
/// null. For more information, see [Using Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
#[derive(Debug, Default)]
pub struct ListPlatformApplicationsInput {
	/// NextToken string is used when calling ListPlatformApplications action to
	/// retrieve additional records that are available after the first page results.
	pub next_token: String,
}

/// Parse a ListPlatformApplicationsInput from XML
pub struct ListPlatformApplicationsInputParser;
impl ListPlatformApplicationsInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPlatformApplicationsInput, XmlParseError> {
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
/// Write a ListPlatformApplicationsInput's contents to a SignedRequest
pub struct ListPlatformApplicationsInputWriter;
impl ListPlatformApplicationsInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListPlatformApplicationsInput) {
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

/// Parse a PublishResponse from XML
pub struct PublishResponseParser;
impl PublishResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PublishResponse, XmlParseError> {
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
/// Write a PublishResponse's contents to a SignedRequest
pub struct PublishResponseWriter;
impl PublishResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PublishResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		messageIdWriter::write_params(params, &(prefix.to_string() + "MessageId"), &obj.message_id);
	}
}
pub type attributeName = String;
/// Parse a attributeName from XML
pub struct attributeNameParser;
impl attributeNameParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<attributeName, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a attributeName's contents to a SignedRequest
pub struct attributeNameWriter;
impl attributeNameWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &attributeName) {
		params.put(name, obj);
	}
}
/// Response for ConfirmSubscriptions action.
#[derive(Debug, Default)]
pub struct ConfirmSubscriptionResponse {
	/// The ARN of the created subscription.
	pub subscription_arn: subscriptionARN,
}

/// Parse a ConfirmSubscriptionResponse from XML
pub struct ConfirmSubscriptionResponseParser;
impl ConfirmSubscriptionResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ConfirmSubscriptionResponse, XmlParseError> {
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
/// Write a ConfirmSubscriptionResponse's contents to a SignedRequest
pub struct ConfirmSubscriptionResponseWriter;
impl ConfirmSubscriptionResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ConfirmSubscriptionResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
/// Deletes a topic and all its subscriptions. Deleting a topic might prevent some
/// messages previously sent to the topic from being delivered to subscribers.
/// This action is idempotent, so deleting a topic that does not exist does not
/// result in an error.
#[derive(Debug, Default)]
pub struct DeleteTopicInput {
	/// The ARN of the topic you want to delete.
	pub topic_arn: topicARN,
}

/// Parse a DeleteTopicInput from XML
pub struct DeleteTopicInputParser;
impl DeleteTopicInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeleteTopicInput, XmlParseError> {
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
/// Write a DeleteTopicInput's contents to a SignedRequest
pub struct DeleteTopicInputWriter;
impl DeleteTopicInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeleteTopicInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
/// Input for ConfirmSubscription action.
/// Verifies an endpoint owner's intent to receive messages by validating the
/// token sent to the endpoint by an earlier `Subscribe` action. If the token is
/// valid, the action creates a new subscription and returns its Amazon Resource
/// Name (ARN). This call requires an AWS signature only when the
/// `AuthenticateOnUnsubscribe` flag is set to "true".
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

/// Parse a ConfirmSubscriptionInput from XML
pub struct ConfirmSubscriptionInputParser;
impl ConfirmSubscriptionInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ConfirmSubscriptionInput, XmlParseError> {
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
/// Write a ConfirmSubscriptionInput's contents to a SignedRequest
pub struct ConfirmSubscriptionInputWriter;
impl ConfirmSubscriptionInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ConfirmSubscriptionInput) {
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

/// Parse a PlatformApplicationDisabledException from XML
pub struct PlatformApplicationDisabledExceptionParser;
impl PlatformApplicationDisabledExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PlatformApplicationDisabledException, XmlParseError> {
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
/// Write a PlatformApplicationDisabledException's contents to a SignedRequest
pub struct PlatformApplicationDisabledExceptionWriter;
impl PlatformApplicationDisabledExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PlatformApplicationDisabledException) {
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

/// Parse a InvalidParameterValueException from XML
pub struct InvalidParameterValueExceptionParser;
impl InvalidParameterValueExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InvalidParameterValueException, XmlParseError> {
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
/// Write a InvalidParameterValueException's contents to a SignedRequest
pub struct InvalidParameterValueExceptionWriter;
impl InvalidParameterValueExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InvalidParameterValueException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
pub type authenticateOnUnsubscribe = String;
/// Parse a authenticateOnUnsubscribe from XML
pub struct authenticateOnUnsubscribeParser;
impl authenticateOnUnsubscribeParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<authenticateOnUnsubscribe, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a authenticateOnUnsubscribe's contents to a SignedRequest
pub struct authenticateOnUnsubscribeWriter;
impl authenticateOnUnsubscribeWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &authenticateOnUnsubscribe) {
		params.put(name, obj);
	}
}
pub type messageId = String;
/// Parse a messageId from XML
pub struct messageIdParser;
impl messageIdParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<messageId, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a messageId's contents to a SignedRequest
pub struct messageIdWriter;
impl messageIdWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &messageId) {
		params.put(name, obj);
	}
}
/// Input for GetEndpointAttributes action.
/// Retrieves the endpoint attributes for a device on one of the supported push
/// notification services, such as GCM and APNS. For more information, see [Using
/// Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
#[derive(Debug, Default)]
pub struct GetEndpointAttributesInput {
	/// EndpointArn for GetEndpointAttributes input.
	pub endpoint_arn: String,
}

/// Parse a GetEndpointAttributesInput from XML
pub struct GetEndpointAttributesInputParser;
impl GetEndpointAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetEndpointAttributesInput, XmlParseError> {
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
/// Write a GetEndpointAttributesInput's contents to a SignedRequest
pub struct GetEndpointAttributesInputWriter;
impl GetEndpointAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetEndpointAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "EndpointArn"), &obj.endpoint_arn);
	}
}
/// Input for ListSubscriptionsByTopic action.
/// Returns a list of the subscriptions to a specific topic. Each call returns a
/// limited list of subscriptions, up to 100. If there are more subscriptions, a
/// `NextToken` is also returned. Use the `NextToken` parameter in a new
/// `ListSubscriptionsByTopic` call to get further results.
#[derive(Debug, Default)]
pub struct ListSubscriptionsByTopicInput {
	/// Token returned by the previous `ListSubscriptionsByTopic` request.
	pub next_token: Option<nextToken>,
	/// The ARN of the topic for which you wish to find subscriptions.
	pub topic_arn: topicARN,
}

/// Parse a ListSubscriptionsByTopicInput from XML
pub struct ListSubscriptionsByTopicInputParser;
impl ListSubscriptionsByTopicInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsByTopicInput, XmlParseError> {
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
/// Write a ListSubscriptionsByTopicInput's contents to a SignedRequest
pub struct ListSubscriptionsByTopicInputWriter;
impl ListSubscriptionsByTopicInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsByTopicInput) {
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

/// Parse a SubscribeResponse from XML
pub struct SubscribeResponseParser;
impl SubscribeResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscribeResponse, XmlParseError> {
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
/// Write a SubscribeResponse's contents to a SignedRequest
pub struct SubscribeResponseWriter;
impl SubscribeResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SubscribeResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		subscriptionARNWriter::write_params(params, &(prefix.to_string() + "SubscriptionArn"), &obj.subscription_arn);
	}
}
pub type message = String;
/// Parse a message from XML
pub struct messageParser;
impl messageParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<message, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a message's contents to a SignedRequest
pub struct messageWriter;
impl messageWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &message) {
		params.put(name, obj);
	}
}
pub type TopicsList = Vec<Topic>;
/// Parse a TopicsList from XML
pub struct TopicsListParser;
impl TopicsListParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<TopicsList, XmlParseError> {
		let mut obj = Vec::new();
		while try!(peek_at_name(stack)) == "Topic" {
			obj.push(try!(TopicParser::parse_xml("Topic", stack)));
		}
		Ok(obj)
	}
}
/// Write a TopicsList's contents to a SignedRequest
pub struct TopicsListWriter;
impl TopicsListWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &TopicsList) {
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

/// Parse a EndpointDisabledException from XML
pub struct EndpointDisabledExceptionParser;
impl EndpointDisabledExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<EndpointDisabledException, XmlParseError> {
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
/// Write a EndpointDisabledException's contents to a SignedRequest
pub struct EndpointDisabledExceptionWriter;
impl EndpointDisabledExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &EndpointDisabledException) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		stringWriter::write_params(params, &(prefix.to_string() + "message"), &obj.message);
	}
}
/// Input for GetTopicAttributes action.
/// Returns all of the properties of a topic. Topic properties returned might
/// differ based on the authorization of the user.
#[derive(Debug, Default)]
pub struct GetTopicAttributesInput {
	/// The ARN of the topic whose properties you want to get.
	pub topic_arn: topicARN,
}

/// Parse a GetTopicAttributesInput from XML
pub struct GetTopicAttributesInputParser;
impl GetTopicAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetTopicAttributesInput, XmlParseError> {
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
/// Write a GetTopicAttributesInput's contents to a SignedRequest
pub struct GetTopicAttributesInputWriter;
impl GetTopicAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetTopicAttributesInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
pub type account = String;
/// Parse a account from XML
pub struct accountParser;
impl accountParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<account, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a account's contents to a SignedRequest
pub struct accountWriter;
impl accountWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &account) {
		params.put(name, obj);
	}
}
pub type endpoint = String;
/// Parse a endpoint from XML
pub struct endpointParser;
impl endpointParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<endpoint, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a endpoint's contents to a SignedRequest
pub struct endpointWriter;
impl endpointWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &endpoint) {
		params.put(name, obj);
	}
}
pub type subscriptionARN = String;
/// Parse a subscriptionARN from XML
pub struct subscriptionARNParser;
impl subscriptionARNParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<subscriptionARN, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a subscriptionARN's contents to a SignedRequest
pub struct subscriptionARNWriter;
impl subscriptionARNWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &subscriptionARN) {
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

/// Parse a ListSubscriptionsByTopicResponse from XML
pub struct ListSubscriptionsByTopicResponseParser;
impl ListSubscriptionsByTopicResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsByTopicResponse, XmlParseError> {
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
/// Write a ListSubscriptionsByTopicResponse's contents to a SignedRequest
pub struct ListSubscriptionsByTopicResponseWriter;
impl ListSubscriptionsByTopicResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsByTopicResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		SubscriptionsListWriter::write_params(params, &(prefix.to_string() + "Subscription"), &obj.subscriptions);
	}
}
pub type MapStringToString = HashMap<String,String>;
/// Parse a MapStringToString from XML
pub struct MapStringToStringParser;
impl MapStringToStringParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<MapStringToString, XmlParseError> {
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
/// Write a MapStringToString's contents to a SignedRequest
pub struct MapStringToStringWriter;
impl MapStringToStringWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &MapStringToString) {
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

/// Parse a ListEndpointsByPlatformApplicationResponse from XML
pub struct ListEndpointsByPlatformApplicationResponseParser;
impl ListEndpointsByPlatformApplicationResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListEndpointsByPlatformApplicationResponse, XmlParseError> {
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
/// Write a ListEndpointsByPlatformApplicationResponse's contents to a SignedRequest
pub struct ListEndpointsByPlatformApplicationResponseWriter;
impl ListEndpointsByPlatformApplicationResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListEndpointsByPlatformApplicationResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ListOfEndpointsWriter::write_params(params, &(prefix.to_string() + "Endpoint"), &obj.endpoints);
		StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Adds a statement to a topic's access control policy, granting access for the
/// specified AWS accounts to the specified actions.
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

/// Parse a AddPermissionInput from XML
pub struct AddPermissionInputParser;
impl AddPermissionInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<AddPermissionInput, XmlParseError> {
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
/// Write a AddPermissionInput's contents to a SignedRequest
pub struct AddPermissionInputWriter;
impl AddPermissionInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &AddPermissionInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		ActionsListWriter::write_params(params, &(prefix.to_string() + "action"), &obj.action_name);
		DelegatesListWriter::write_params(params, &(prefix.to_string() + "delegate"), &obj.aws_account_id);
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
		labelWriter::write_params(params, &(prefix.to_string() + "Label"), &obj.label);
	}
}
/// Returns a list of the requester's topics. Each call returns a limited list of
/// topics, up to 100. If there are more topics, a `NextToken` is also returned.
/// Use the `NextToken` parameter in a new `ListTopics` call to get further
/// results.
#[derive(Debug, Default)]
pub struct ListTopicsInput {
	/// Token returned by the previous `ListTopics` request.
	pub next_token: nextToken,
}

/// Parse a ListTopicsInput from XML
pub struct ListTopicsInputParser;
impl ListTopicsInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListTopicsInput, XmlParseError> {
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
/// Write a ListTopicsInput's contents to a SignedRequest
pub struct ListTopicsInputWriter;
impl ListTopicsInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListTopicsInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		nextTokenWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
	}
}
/// Input for Publish action.
/// Sends a message to all of a topic's subscribed endpoints. When a `messageId`
/// is returned, the message has been saved and Amazon SNS will attempt to deliver
/// it to the topic's subscribers shortly. The format of the outgoing message to
/// each subscribed endpoint depends on the notification protocol selected.
/// To use the `Publish` action for sending a message to a mobile endpoint, such
/// as an app on a Kindle device or mobile phone, you must specify the
/// EndpointArn. The EndpointArn is returned when making a call with the
/// `CreatePlatformEndpoint` action. The second example below shows a request and
/// response for publishing to a mobile endpoint.
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

/// Parse a PublishInput from XML
pub struct PublishInputParser;
impl PublishInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<PublishInput, XmlParseError> {
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
/// Write a PublishInput's contents to a SignedRequest
pub struct PublishInputWriter;
impl PublishInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &PublishInput) {
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

/// Parse a Topic from XML
pub struct TopicParser;
impl TopicParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Topic, XmlParseError> {
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
/// Write a Topic's contents to a SignedRequest
pub struct TopicWriter;
impl TopicWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Topic) {
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

/// Parse a ListPlatformApplicationsResponse from XML
pub struct ListPlatformApplicationsResponseParser;
impl ListPlatformApplicationsResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListPlatformApplicationsResponse, XmlParseError> {
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
/// Write a ListPlatformApplicationsResponse's contents to a SignedRequest
pub struct ListPlatformApplicationsResponseWriter;
impl ListPlatformApplicationsResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListPlatformApplicationsResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "NextToken"), &obj.next_token);
		ListOfPlatformApplicationsWriter::write_params(params, &(prefix.to_string() + "PlatformApplication"), &obj.platform_applications);
	}
}
/// Input for SetSubscriptionAttributes action.
/// Allows a subscription owner to set an attribute of the topic to a new value.
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

/// Parse a SetSubscriptionAttributesInput from XML
pub struct SetSubscriptionAttributesInputParser;
impl SetSubscriptionAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetSubscriptionAttributesInput, XmlParseError> {
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
/// Write a SetSubscriptionAttributesInput's contents to a SignedRequest
pub struct SetSubscriptionAttributesInputWriter;
impl SetSubscriptionAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SetSubscriptionAttributesInput) {
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
/// Parse a MessageAttributeMap from XML
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
/// Write a MessageAttributeMap's contents to a SignedRequest
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
/// Input for DeletePlatformApplication action.
/// Deletes a platform application object for one of the supported push
/// notification services, such as APNS and GCM. For more information, see [Using
/// Amazon SNS Mobile Push
/// Notifications](http://docs.aws.amazon.com/sns/latest/dg/SNSMobilePush.html).
#[derive(Debug, Default)]
pub struct DeletePlatformApplicationInput {
	/// PlatformApplicationArn of platform application object to delete.
	pub platform_application_arn: String,
}

/// Parse a DeletePlatformApplicationInput from XML
pub struct DeletePlatformApplicationInputParser;
impl DeletePlatformApplicationInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<DeletePlatformApplicationInput, XmlParseError> {
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
/// Write a DeletePlatformApplicationInput's contents to a SignedRequest
pub struct DeletePlatformApplicationInputWriter;
impl DeletePlatformApplicationInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &DeletePlatformApplicationInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		StringWriter::write_params(params, &(prefix.to_string() + "PlatformApplicationArn"), &obj.platform_application_arn);
	}
}
pub type delegate = String;
/// Parse a delegate from XML
pub struct delegateParser;
impl delegateParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<delegate, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a delegate's contents to a SignedRequest
pub struct delegateWriter;
impl delegateWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &delegate) {
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

/// Parse a GetPlatformApplicationAttributesResponse from XML
pub struct GetPlatformApplicationAttributesResponseParser;
impl GetPlatformApplicationAttributesResponseParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<GetPlatformApplicationAttributesResponse, XmlParseError> {
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
/// Write a GetPlatformApplicationAttributesResponse's contents to a SignedRequest
pub struct GetPlatformApplicationAttributesResponseWriter;
impl GetPlatformApplicationAttributesResponseWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &GetPlatformApplicationAttributesResponse) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		MapStringToStringWriter::write_params(params, &(prefix.to_string() + "Attributes"), &obj.attributes);
	}
}
/// Input for ListSubscriptions action.
/// Returns a list of the requester's subscriptions. Each call returns a limited
/// list of subscriptions, up to 100. If there are more subscriptions, a
/// `NextToken` is also returned. Use the `NextToken` parameter in a new
/// `ListSubscriptions` call to get further results.
#[derive(Debug, Default)]
pub struct ListSubscriptionsInput {
	/// Token returned by the previous `ListSubscriptions` request.
	pub next_token: nextToken,
}

/// Parse a ListSubscriptionsInput from XML
pub struct ListSubscriptionsInputParser;
impl ListSubscriptionsInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<ListSubscriptionsInput, XmlParseError> {
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
/// Write a ListSubscriptionsInput's contents to a SignedRequest
pub struct ListSubscriptionsInputWriter;
impl ListSubscriptionsInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &ListSubscriptionsInput) {
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

/// Parse a InternalErrorException from XML
pub struct InternalErrorExceptionParser;
impl InternalErrorExceptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<InternalErrorException, XmlParseError> {
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
/// Write a InternalErrorException's contents to a SignedRequest
pub struct InternalErrorExceptionWriter;
impl InternalErrorExceptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &InternalErrorException) {
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

/// Parse a Subscription from XML
pub struct SubscriptionParser;
impl SubscriptionParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<Subscription, XmlParseError> {
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
/// Write a Subscription's contents to a SignedRequest
pub struct SubscriptionWriter;
impl SubscriptionWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &Subscription) {
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
/// Allows a topic owner to set an attribute of the topic to a new value.
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

/// Parse a SetTopicAttributesInput from XML
pub struct SetTopicAttributesInputParser;
impl SetTopicAttributesInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SetTopicAttributesInput, XmlParseError> {
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
/// Write a SetTopicAttributesInput's contents to a SignedRequest
pub struct SetTopicAttributesInputWriter;
impl SetTopicAttributesInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SetTopicAttributesInput) {
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
/// Parse a token from XML
pub struct tokenParser;
impl tokenParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<token, XmlParseError> {
		try!(start_element(tag_name, stack));
		let obj = try!(characters(stack));
		try!(end_element(tag_name, stack));
		Ok(obj)
	}
}
/// Write a token's contents to a SignedRequest
pub struct tokenWriter;
impl tokenWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &token) {
		params.put(name, obj);
	}
}
/// Input for Subscribe action.
/// Prepares to subscribe an endpoint by sending the endpoint a confirmation
/// message. To actually create a subscription, the endpoint owner must call the
/// `ConfirmSubscription` action with the token from the confirmation message.
/// Confirmation tokens are valid for three days.
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

/// Parse a SubscribeInput from XML
pub struct SubscribeInputParser;
impl SubscribeInputParser {
	pub fn parse_xml<'a>(tag_name: &str, stack: &mut XmlStack) -> Result<SubscribeInput, XmlParseError> {
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
/// Write a SubscribeInput's contents to a SignedRequest
pub struct SubscribeInputWriter;
impl SubscribeInputWriter {
	pub fn write_params(params: &mut Params, name: &str, obj: &SubscribeInput) {
		let mut prefix = name.to_string();
		if prefix != "" { prefix.push_str("."); }
		if let Some(ref obj) = obj.endpoint {
			endpointWriter::write_params(params, &(prefix.to_string() + "Endpoint"), obj);
		}
		protocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
		topicARNWriter::write_params(params, &(prefix.to_string() + "TopicArn"), &obj.topic_arn);
	}
}
impl AWSRequest<ListPlatformApplicationsResponse> for ListPlatformApplicationsInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListPlatformApplicationsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListPlatformApplications");
		ListPlatformApplicationsInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for SetPlatformApplicationAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "SetPlatformApplicationAttributes");
		SetPlatformApplicationAttributesInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<ListSubscriptionsByTopicResponse> for ListSubscriptionsByTopicInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListSubscriptionsByTopicResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSubscriptionsByTopic");
		ListSubscriptionsByTopicInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<ListSubscriptionsResponse> for ListSubscriptionsInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListSubscriptionsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListSubscriptions");
		ListSubscriptionsInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<ConfirmSubscriptionResponse> for ConfirmSubscriptionInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ConfirmSubscriptionResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ConfirmSubscription");
		ConfirmSubscriptionInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for SetTopicAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "SetTopicAttributes");
		SetTopicAttributesInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<()> for SetEndpointAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "SetEndpointAttributes");
		SetEndpointAttributesInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<()> for AddPermissionInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "AddPermission");
		AddPermissionInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<()> for UnsubscribeInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "Unsubscribe");
		UnsubscribeInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<GetSubscriptionAttributesResponse> for GetSubscriptionAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetSubscriptionAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetSubscriptionAttributes");
		GetSubscriptionAttributesInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<CreateEndpointResponse> for CreatePlatformEndpointInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<CreateEndpointResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePlatformEndpoint");
		CreatePlatformEndpointInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for RemovePermissionInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "RemovePermission");
		RemovePermissionInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<CreateTopicResponse> for CreateTopicInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<CreateTopicResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "CreateTopic");
		CreateTopicInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for SetSubscriptionAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "SetSubscriptionAttributes");
		SetSubscriptionAttributesInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<CreatePlatformApplicationResponse> for CreatePlatformApplicationInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<CreatePlatformApplicationResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "CreatePlatformApplication");
		CreatePlatformApplicationInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<GetPlatformApplicationAttributesResponse> for GetPlatformApplicationAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetPlatformApplicationAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetPlatformApplicationAttributes");
		GetPlatformApplicationAttributesInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<PublishResponse> for PublishInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<PublishResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "Publish");
		PublishInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<ListEndpointsByPlatformApplicationResponse> for ListEndpointsByPlatformApplicationInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListEndpointsByPlatformApplicationResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListEndpointsByPlatformApplication");
		ListEndpointsByPlatformApplicationInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<GetEndpointAttributesResponse> for GetEndpointAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetEndpointAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetEndpointAttributes");
		GetEndpointAttributesInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for DeleteEndpointInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteEndpoint");
		DeleteEndpointInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<()> for DeletePlatformApplicationInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeletePlatformApplication");
		DeletePlatformApplicationInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<GetTopicAttributesResponse> for GetTopicAttributesInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<GetTopicAttributesResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "GetTopicAttributes");
		GetTopicAttributesInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<SubscribeResponse> for SubscribeInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<SubscribeResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "Subscribe");
		SubscribeInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
}
impl AWSRequest<()> for DeleteTopicInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<(), AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "DeleteTopic");
		DeleteTopicInputWriter::write_params(&mut params, "", self);
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
impl AWSRequest<ListTopicsResponse> for ListTopicsInput {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<ListTopicsResponse, AWSError> {
		let mut request = SignedRequest::new("POST", "sns", region, "/");
		let mut params = Params::new();
		params.put("Action", "ListTopics");
		ListTopicsInputWriter::write_params(&mut params, "", self);
		request.set_params(params);
		let (status, output) = try!(request.sign_and_execute(creds));
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
