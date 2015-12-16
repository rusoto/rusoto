//! KMS bindings for Rust

#![allow(non_snake_case)]

use std::result::Result as StdResult;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/kms.rs"));

#[derive(RustcDecodable, Debug)]
pub struct KMSError {
	__type: String,
	message: String
}

impl From<AWSError> for KMSError {
	fn from(err: AWSError) -> KMSError {
		let AWSError(message) = err;
		KMSError { __type: "Unknown".to_string(), message: message.to_string() }
	}
}

fn parse_error(body: &str) -> KMSError {
	if let Ok(decoded) = json::decode::<KMSError>(&body) {
		decoded
	} else {
		KMSError { __type: "DecodeError".to_string(), message: body.to_string() }
	}
}

pub type Result<T> = StdResult<T, KMSError>;
