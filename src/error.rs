//! Error and result types.

use std::fmt;
use std::error::Error;
use std::io::Error as IoError;
use std::str::ParseBoolError;
use std::num::ParseIntError;

use chrono::format::ParseError as ChronoParseError;
use serde_json::{Value, from_str};

use xmlutil::XmlParseError;

/// An error produced when AWS API calls are unsuccessful.
#[derive(Debug, PartialEq)]
pub struct AwsError {
    pub message: String
}

pub fn parse_json_protocol_error(body: &str) -> AwsError {
    match from_str::<Value>(body) {
        Ok(json) => {
            let error_type: &str = match json.find("__type") {
                Some(error_type) => error_type.as_string().unwrap_or("Unknown error"),
                None => "Unknown error",
            };

            let error_message: &str = match json.find("message") {
                Some(error_message) => error_message.as_string().unwrap_or(body),
                None => body,
            };

            AwsError::new(format!("{}: {}", error_type, error_message))
        }
        Err(err) => AwsError::new(
            format!("Failed to parse error as JSON: {}.\nRaw error:\n{}", err, body)
        ),
    }
}

impl AwsError {
    /// Create a new error with the given message.
    pub fn new<S>(message: S) -> AwsError where S: Into<String> {
        AwsError {
            message: message.into(),
        }
    }
}

impl Error for AwsError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<ParseBoolError> for AwsError {
    fn from(err: ParseBoolError) -> AwsError {
        AwsError::new(format!("{}", err))
    }
}

impl From<ParseIntError> for AwsError {
    fn from(err: ParseIntError) -> AwsError {
        AwsError::new(format!("{}", err))
    }
}

impl From<ChronoParseError> for AwsError {
    fn from(err: ChronoParseError) -> AwsError {
        AwsError::new(format!("{}", err))
    }
}

impl From<IoError> for AwsError {
    fn from(err: IoError) -> AwsError {
        AwsError::new(format!("{}", err))
    }
}

impl From<XmlParseError> for AwsError {
    fn from(err: XmlParseError) -> AwsError {
        AwsError::new(format!("{:?}", err))
    }
}

impl fmt::Display for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The result type produced by AWS API calls.
pub type AwsResult<T> = Result<T, AwsError>;
