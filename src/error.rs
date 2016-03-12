//! Error and result types.

use std::fmt;
use std::io::Error as IoError;

use chrono::format::ParseError as ChronoParseError;

use xmlutil::XmlParseError;

/// An error produced when AWS API calls are unsuccessful.
#[derive(Debug, PartialEq)]
pub struct AWSError(pub String);

impl AWSError {
	pub fn new<S>(msg:S) -> AWSError where S:Into<String>{
		AWSError(msg.into())
	}
}

impl From<ChronoParseError> for AWSError {
    fn from(err: ChronoParseError) -> AWSError {
        AWSError(format!("{}", err))
    }
}

impl From<IoError> for AWSError {
    fn from(err: IoError) -> AWSError {
        AWSError(format!("{}", err))
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

/// The result type produced by AWS API calls.
pub type AWSResult<T> = Result<T, AWSError>;
