//! Errors during AWS communication or parsing
use xmlutil::XmlParseError;

/// Simple wraper around a String to store the error
#[derive(Debug, PartialEq)]
pub struct AWSError(pub String);

impl AWSError {
	pub fn new<S>(msg:S) -> AWSError where S:Into<String>{
		AWSError(msg.into())
	}
}

impl From<XmlParseError> for AWSError {
        fn from(err: XmlParseError) -> AWSError {
                AWSError(format!("{:?}", err))
        }
}
