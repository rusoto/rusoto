use xmlutil::*;
use signature::URIParseError;

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
