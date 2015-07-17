use xmlutil::*;
use signature::URIParseError;

#[derive(Debug, Default)]
pub struct ErrorResponse {
        pub error: Error,
        pub request_id: String
}

impl XmlParser for ErrorResponse {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = ErrorResponse::default();
		obj.error = try!(Error::parse_xml(stack));
		obj.request_id = try!(string_field("RequestId", stack));
		Ok(obj)		
	}
}

#[derive(Debug, Default)]
pub struct Error {
        pub error_type: String,
        pub code: String,
        pub message: String,
        pub detail: String
}

impl XmlParser for Error {
	fn parse_xml<'a>(stack: &mut XmlStack) -> Result<Self, XmlParseError> {
		let mut obj = Error::default();
		obj.error_type = try!(string_field("Type", stack));
		obj.code = try!(string_field("Code", stack));
		obj.message = try!(string_field("Message", stack));
		obj.detail = try!(string_field("Detail", stack));
		Ok(obj)		
	}		
}


#[derive(Debug)]
pub struct AWSError(String);

impl AWSError {
	pub fn new<S>(msg:S) -> AWSError where S:Into<String>{
		AWSError(msg.into())
	}
}

impl From<URIParseError> for AWSError {
        fn from(err: URIParseError) -> AWSError {
                AWSError(format!("{:?}", err))
        }
}
impl From<XmlParseError> for AWSError {
        fn from(err: XmlParseError) -> AWSError {
                AWSError(format!("{:?}", err))
        }
}
