//! SQS bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use std::str::FromStr;
use regions::*;

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sqs.rs"));

pub struct SQSHelper<'a> {
	client: SQSClient<'a>
}

impl<'a> SQSHelper<'a> {
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> SQSHelper<'a> {
		SQSHelper { client: SQSClient::new(credentials, region) }
	}

	pub fn list_queues(&mut self) -> Result<ListQueuesResult, AWSError> {
		self.client.list_queues(&ListQueuesRequest::default())
	}

	pub fn create_queue(&mut self, queue_name: &str) -> Result<CreateQueueResult, AWSError> {
		let mut req = CreateQueueRequest::default();
		req.queue_name = queue_name.to_string();
		self.client.create_queue(&req)
	}

	pub fn get_queue_url(&mut self, queue_name: &str) -> Result<GetQueueUrlResult, AWSError> {
		let mut req = GetQueueUrlRequest::default();
		req.queue_name = queue_name.to_string();
		self.client.get_queue_url(&req)
	}

	pub fn send_message(&mut self, queue_url: &str, message_body: &str) -> Result<SendMessageResult, AWSError> {
		let mut req = SendMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.message_body = message_body.to_string();
		self.client.send_message(&req)
	}

	pub fn receive_message(&mut self, queue_url: &str) -> Result<ReceiveMessageResult, AWSError> {
		let mut req = ReceiveMessageRequest::default();
		req.queue_url = queue_url.to_string();
		self.client.receive_message(&req)
	}

	pub fn delete_message(&mut self, queue_url: &str, receipt_handle: &str) -> Result<(), AWSError> {
		let mut req = DeleteMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.receipt_handle = receipt_handle.to_string();
		self.client.delete_message(&req)
	}

	pub fn delete_queue(&mut self, queue_url: &str) -> Result<(), AWSError> {
		let mut req = DeleteQueueRequest::default();
		req.queue_url = queue_url.to_string();
		self.client.delete_queue(&req)
	}

}
