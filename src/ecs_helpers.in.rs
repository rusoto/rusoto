use error::AWSError;
use std::result;

/// Result type for ECS requests
pub type Result<T> = result::Result<T, ECSError>;

/// Common error type for ECS requests
#[derive(Debug, Deserialize)]
pub struct ECSError {
    __type: String,
    message: Option<String>
}

impl From<AWSError> for ECSError {
    fn from(err: AWSError) -> ECSError {
        let AWSError(message) = err;
        ECSError { __type: "Unknown".to_string(), message: Some(message.to_string()) }
    }
}

fn parse_error(body: &str) -> ECSError {
    if let Ok(decoded) = from_str::<ECSError>(&body) {
        decoded
    } else {
        ECSError { __type: "DecodeError".to_string(), message: Some(body.to_string()) }
    }
}
