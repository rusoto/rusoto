//! SNS bindings for Rust
#![allow(unused_variables, unused_mut, non_camel_case_types, unused_imports)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use request::*;
use xmlutil::*;
use std::str::FromStr;

// include the code generated from the SNS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sns.rs"));

pub struct SNSClient {
	creds: AWSCredentials,
	region: String
}

impl SNSClient {
	pub fn new<S>(credentials:AWSCredentials, region:S) -> SNSClient where S:Into<String> {
		SNSClient { creds: credentials, region: region.into() }
	}
	
	pub fn list_topics(&self) -> Result<ListTopicsResponse, AWSError> {
		ListTopicsInput::default().execute(&self.creds, &self.region)
	}
}	