//! SQS bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use request::*;
use xmlutil::*;
use std::str::FromStr;

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sqs.rs"));

pub struct SQSClient {
	creds: AWSCredentials,
	region: String
}

impl SQSClient {
	pub fn new<S>(credentials:AWSCredentials, region:S) -> SQSClient where S:Into<String> {
		SQSClient { creds: credentials, region: region.into() }
	}

	pub fn list_queues(&self) -> Result<ListQueuesResult, AWSError> {
		ListQueuesRequest::default().execute(&self.creds, &self.region)
	}
	
	pub fn create_queue(&self, queue_name: &str) -> Result<CreateQueueResult, AWSError> {
		let mut req = CreateQueueRequest::default();
		req.queue_name = queue_name.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn get_queue_url(&self, queue_name: &str) -> Result<GetQueueUrlResult, AWSError> {
		let mut req = GetQueueUrlRequest::default();
		req.queue_name = queue_name.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn send_message(&self, queue_url: &str, message_body: &str) -> Result<SendMessageResult, AWSError> {
		let mut req = SendMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.message_body = message_body.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn receive_message(&self, queue_url: &str) -> Result<ReceiveMessageResult, AWSError> {
		let mut req = ReceiveMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn delete_message(&self, queue_url: &str, receipt_handle: &str) -> Result<(), AWSError> {
		let mut req = DeleteMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.receipt_handle = receipt_handle.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn delete_queue(&self, queue_url: &str) -> Result<(), AWSError> {
		let mut req = DeleteQueueRequest::default();
		req.queue_url = queue_url.to_string();
		req.execute(&self.creds, &self.region)
	}
	
	pub fn execute<T, R: AWSRequest<T>>(&self, request: R) -> Result<T, AWSError> {
		request.execute(&self.creds, &self.region)
	}
	
}
