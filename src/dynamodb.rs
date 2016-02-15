//! DynamoDB bindings for Rust
//!
//! Check
//! [DynamoDBHelper](http://dualspark.github.io/rusoto/rusoto/dynamodb/struct.DynamoDBHelper.html)
//! for convenience functions.
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

// include the code generated from the helpers
include!(concat!(env!("OUT_DIR"), "/dynamodb_helpers.rs"));
