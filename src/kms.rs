//! KMS bindings for Rust
//!
//! Check [Helper](http://dualspark.github.io/rusoto/rusoto/kms/struct.KMSHelper.html) for convenience functions.
//!

#![allow(unused_variables, unused_mut, non_snake_case)]
use credentials::*;
use signature::*;
use error::*;
use regions::*;
use std::result;

// include the code generated from the KMS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/kms.rs"));

/// Easier to use KMS client: wraps KMSClient class.
pub struct KMSHelper<'a> {
	client: KMSClient<'a>
}

impl<'a> KMSHelper<'a> {
	/// Creates a new KMS helper
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> KMSHelper<'a> {
		KMSHelper { client: KMSClient::new(credentials, region) }
	}
}

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

pub type Result<T> = result::Result<T, KMSError>;

fn parse_error(body: &str) -> KMSError {
	if let Ok(decoded) = json::decode::<KMSError>(&body) {
		decoded
	} else {
		KMSError { __type: "DecodeError".to_string(), message: body.to_string() }
	}
}
