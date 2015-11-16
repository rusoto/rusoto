//! Errors during AWS communication or parsing
//!
//! Wrapper around String to store the error.
//!

use std::fmt;
use xmlutil::XmlParseError;

/// Simple wrapper around a String to store the error
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

impl fmt::Display for AWSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
