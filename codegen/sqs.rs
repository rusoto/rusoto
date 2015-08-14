/// Error code 400. Unsupported operation.
#[derive(Debug, Default)]
pub struct UnsupportedOperation;

// 1904-3178
#[derive(Debug, Default)]
pub struct ListQueuesRequest {
	/// A string to use for filtering the list results. Only those queues whose name
	/// begins with the specified string are returned.
	pub queue_name_prefix: String,
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

pub struct SQSClient<'a> {
	creds: &'a AWSCredentials,
	region: &'a str
}

impl<'a> SQSClient<'a> {
	pub fn new(creds: &'a AWSCredentials, region: &'a str) -> SQSClient<'a> {
		SQSClient { creds: creds, region: region }
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
		// NOTE: the next four lines are important and will probably need to be changed in the code gen:
		// let mut stack = reader.events().peekable();
		let mut stack_wrapped = XmlResponseFromAws::new(reader.events().peekable());
		stack_wrapped.next();
		stack_wrapped.next();
		
		match status {
			200 => {
				Ok(try!(ListQueuesResultParser::parse_xml("ListQueuesResult", &mut stack_wrapped)))
			}
			_ => { Err(AWSError::new("error")) }
		}
	}
}
