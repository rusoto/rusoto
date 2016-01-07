//! DynamoDB bindings for Rust
//!
//! Check [DynamoDBHelper](http://dualspark.github.io/rusoto/rusoto/dynamodb/struct.DynamoDBHelper.html) for convenience functions.
//!

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::result;
use std::str;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;

// include the code generated from the DynamoDB botocore templates
include!(concat!(env!("OUT_DIR"), "/dynamodb.rs"));


#[derive(RustcDecodable, Debug)]
pub struct DynamoDBError {
	__type: String,
	message: String
}

impl From<AWSError> for DynamoDBError {
	fn from(err: AWSError) -> DynamoDBError {
		let AWSError(message) = err;
		DynamoDBError { __type: "Unknown".to_string(), message: message.to_string() }
	}
}

pub type Result<T> = result::Result<T, DynamoDBError>;

fn parse_error(body: &str) -> DynamoDBError {
	if let Ok(decoded) = json::decode::<DynamoDBError>(&body) {
		decoded
	} else {
		DynamoDBError { __type: "DecodeError".to_string(), message: body.to_string() }
	}
}
