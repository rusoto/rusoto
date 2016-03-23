//! Error and result types.

use std::fmt;
use std::io::Error as IoError;

use chrono::format::ParseError as ChronoParseError;

use xmlutil::XmlParseError;

/// An error produced when AWS API calls are unsuccessful.
#[derive(Debug, PartialEq)]
pub struct AwsError(pub String);

impl AwsError {
	pub fn new<S>(msg:S) -> AwsError where S:Into<String>{
		AwsError(msg.into())
	}
}

impl From<ChronoParseError> for AwsError {
    fn from(err: ChronoParseError) -> AwsError {
        AwsError(format!("{}", err))
    }
}

impl From<IoError> for AwsError {
    fn from(err: IoError) -> AwsError {
        AwsError(format!("{}", err))
    }
}

impl From<XmlParseError> for AwsError {
        fn from(err: XmlParseError) -> AwsError {
                AwsError(format!("{:?}", err))
        }
}

impl fmt::Display for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// The result type produced by AWS API calls.
pub type AwsResult<T> = Result<T, AwsError>;
