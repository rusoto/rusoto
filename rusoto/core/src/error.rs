use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io;

use crate::credential::CredentialsError;

use super::proto::json::{Error as JsonError};
use super::proto::xml::{error::XmlError, util::XmlParseError};
use super::request::{BufferedHttpResponse, HttpDispatchError};

/// Generic error type returned by all rusoto requests.
#[derive(Debug, PartialEq)]
pub enum RusotoError<E> {
    /// A service-specific error occurred.
    Service(E),
    /// A common AWS error occurred.
    Common(AwsError),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

/// Result carrying a generic `RusotoError`.
pub type RusotoResult<T, E> = Result<T, RusotoError<E>>;

impl<E> From<XmlParseError> for RusotoError<E> {
    fn from(err: XmlParseError) -> Self {
        let XmlParseError(message) = err;
        RusotoError::ParseError(message.to_string())
    }
}

impl<E> From<serde_json::error::Error> for RusotoError<E> {
    fn from(err: serde_json::error::Error) -> Self {
        RusotoError::ParseError(err.to_string())
    }
}

impl<E> From<CredentialsError> for RusotoError<E> {
    fn from(err: CredentialsError) -> Self {
        RusotoError::Credentials(err)
    }
}

impl<E> From<HttpDispatchError> for RusotoError<E> {
    fn from(err: HttpDispatchError) -> Self {
        RusotoError::HttpDispatch(err)
    }
}

impl<E> From<io::Error> for RusotoError<E> {
    fn from(err: io::Error) -> Self {
        RusotoError::HttpDispatch(HttpDispatchError::from(err))
    }
}

impl<E: Error + 'static> fmt::Display for RusotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RusotoError::Service(ref err) => write!(f, "{}", err),
            RusotoError::Validation(ref cause) => write!(f, "{}", cause),
            RusotoError::Common(ref err) => write!(f, "{}", err),
            RusotoError::Credentials(ref err) => write!(f, "{}", err),
            RusotoError::HttpDispatch(ref dispatch_error) => write!(f, "{}", dispatch_error),
            RusotoError::ParseError(ref cause) => write!(f, "{}", cause),
            RusotoError::Unknown(ref cause) => write!(f, "{}", cause.body_as_str()),
        }
    }
}

impl<E: Error + 'static> Error for RusotoError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RusotoError::Service(ref err) => Some(err),
            RusotoError::Common(ref err) => Some(err),
            RusotoError::Credentials(ref err) => Some(err),
            RusotoError::HttpDispatch(ref err) => Some(err),
            _ => None,
        }
    }
}

/// Common errors which can occur when communicating with any AWS service.
///
/// These errors are documented for each service individually but are the same for all
/// e.g. https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/CommonErrors.html.
#[derive(Debug, PartialEq)]
pub enum AwsError {
    /// You do not have sufficient access to perform this action.
    AccessDenied(String),
    /// The request signature does not conform to AWS standards.
    IncompleteSignature(String),
    /// The request processing has failed because of an unknown error, exception or failure.
    InternalFailure(String),
    /// The action or operation requested is invalid. Verify that the action is typed correctly.
    InvalidAction(String),
    /// The X.509 certificate or AWS access key ID provided does not exist in our records.
    InvalidClientTokenId(String),
    /// Parameters that must not be used together were used together.
    InvalidParameterCombination(String),
    /// An invalid or out-of-range value was supplied for the input parameter.
    InvalidParameterValue(String),
    /// The AWS query string is malformed or does not adhere to AWS standards.
    InvalidQueryParameter(String),
    /// The query string contains a syntax error.
    MalformedQueryString(String),
    /// The request is missing an action or a required parameter.
    MissingAction(String),
    /// The request must contain either a valid (registered) AWS access key ID or X.509 certificate.
    MissingAuthenticationToken(String),
    /// A required parameter for the specified action is not supplied.
    MissingParameter(String),
    /// The AWS access key ID needs a subscription for the service.
    OptInRequired(String),
    /// The request reached the service more than 15 minutes after the date stamp on the request or more
    /// than 15 minutes after the request expiration date (such as for pre-signed URLs), or the date stamp
    /// on the request is more than 15 minutes in the future.
    RequestExpired(String),
    /// The request has failed due to a temporary failure of the server.
    ServiceUnavailable(String),
    /// The request was denied due to request throttling.
    Throttling(String),
    /// The input fails to satisfy the constraints specified by an AWS service.
    ValidationError(String),
}

impl TryFrom<JsonError> for AwsError {
    type Error = JsonError;
    fn try_from(value: JsonError) -> Result<AwsError, JsonError> {
        match &*value.typ {
            "AccessDeniedException" => Ok(AwsError::AccessDenied(value.msg)),
            "IncompleteSignature" => Ok(AwsError::IncompleteSignature(value.msg)),
            "InternalFailure" => Ok(AwsError::InternalFailure(value.msg)),
            "InvalidAction" => Ok(AwsError::InvalidAction(value.msg)),
            "InvalidClientTokenId" => Ok(AwsError::InvalidClientTokenId(value.msg)),
            "InvalidParameterCombination" => Ok(AwsError::InvalidParameterCombination(value.msg)),
            "InvalidParameterValue" => Ok(AwsError::InvalidParameterValue(value.msg)),
            "InvalidQueryParameter" => Ok(AwsError::InvalidQueryParameter(value.msg)),
            "MalformedQueryString" => Ok(AwsError::MalformedQueryString(value.msg)),
            "MissingAction" => Ok(AwsError::MissingAction(value.msg)),
            "MissingAuthenticationToken" => Ok(AwsError::MissingAuthenticationToken(value.msg)),
            "MissingParameter" => Ok(AwsError::MissingParameter(value.msg)),
            "OptInRequired" => Ok(AwsError::OptInRequired(value.msg)),
            "RequestExpired" => Ok(AwsError::RequestExpired(value.msg)),
            "ServiceUnavailable" => Ok(AwsError::ServiceUnavailable(value.msg)),
            "ThrottlingException" => Ok(AwsError::Throttling(value.msg)),
            "ValidationError" => Ok(AwsError::ValidationError(value.msg)),
            _ => Err(value),
        }
    }
}

impl TryFrom<XmlError> for AwsError {
    type Error = XmlError;
    fn try_from(value: XmlError) -> Result<AwsError, XmlError> {
        match &*value.code {
            "AccessDeniedException" => Ok(AwsError::AccessDenied(value.message)),
            "IncompleteSignature" => Ok(AwsError::IncompleteSignature(value.message)),
            "InternalFailure" => Ok(AwsError::InternalFailure(value.message)),
            "InvalidAction" => Ok(AwsError::InvalidAction(value.message)),
            "InvalidClientTokenId" => Ok(AwsError::InvalidClientTokenId(value.message)),
            "InvalidParameterCombination" => Ok(AwsError::InvalidParameterCombination(value.message)),
            "InvalidParameterValue" => Ok(AwsError::InvalidParameterValue(value.message)),
            "InvalidQueryParameter" => Ok(AwsError::InvalidQueryParameter(value.message)),
            "MalformedQueryString" => Ok(AwsError::MalformedQueryString(value.message)),
            "MissingAction" => Ok(AwsError::MissingAction(value.message)),
            "MissingAuthenticationToken" => Ok(AwsError::MissingAuthenticationToken(value.message)),
            "MissingParameter" => Ok(AwsError::MissingParameter(value.message)),
            "OptInRequired" => Ok(AwsError::OptInRequired(value.message)),
            "RequestExpired" => Ok(AwsError::RequestExpired(value.message)),
            "ServiceUnavailable" => Ok(AwsError::ServiceUnavailable(value.message)),
            "ThrottlingException" => Ok(AwsError::Throttling(value.message)),
            "ValidationError" => Ok(AwsError::ValidationError(value.message)),
            _ => Err(value),
        }
    }
}

impl fmt::Display for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AwsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AwsError::IncompleteSignature(ref cause) => write!(f, "{}", cause),
            AwsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            AwsError::InvalidAction(ref cause) => write!(f, "{}", cause),
            AwsError::InvalidClientTokenId(ref cause) => write!(f, "{}", cause),
            AwsError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            AwsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AwsError::InvalidQueryParameter(ref cause) => write!(f, "{}", cause),
            AwsError::MalformedQueryString(ref cause) => write!(f, "{}", cause),
            AwsError::MissingAction(ref cause) => write!(f, "{}", cause),
            AwsError::MissingAuthenticationToken(ref cause) => write!(f, "{}", cause),
            AwsError::MissingParameter(ref cause) => write!(f, "{}", cause),
            AwsError::OptInRequired(ref cause) => write!(f, "{}", cause),
            AwsError::RequestExpired(ref cause) => write!(f, "{}", cause),
            AwsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            AwsError::Throttling(ref cause) => write!(f, "{}", cause),
            AwsError::ValidationError(ref cause) => write!(f, "{}", cause),
        }
    }
}

impl Error for AwsError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::default::Default;

    const COMMON_ERROR_CODES: [&'static str;17] = [
            "AccessDeniedException",
            "IncompleteSignature",
            "InternalFailure",
            "InvalidAction",
            "InvalidClientTokenId",
            "InvalidParameterCombination",
            "InvalidParameterValue",
            "InvalidQueryParameter",
            "MalformedQueryString",
            "MissingAction",
            "MissingAuthenticationToken",
            "MissingParameter",
            "OptInRequired",
            "RequestExpired",
            "ServiceUnavailable",
            "ThrottlingException",
            "ValidationError",
    ];

    #[test]
    fn aws_error_try_parse_consistent_between_json_and_xml() {
        for code in COMMON_ERROR_CODES.iter() {
            let xml_error = XmlError {
                code: code.to_string(),
                ..Default::default()
            };
            let json_error = JsonError {
                typ: code.to_string(),
                msg: "".to_string(),
            };
            let from_xml_result = AwsError::try_from(xml_error).ok();
            assert!(from_xml_result.is_some());
            let from_json_result = AwsError::try_from(json_error).ok();
            assert!(from_json_result.is_some());
            assert_eq!(from_xml_result, from_json_result);
        }
    }
}
