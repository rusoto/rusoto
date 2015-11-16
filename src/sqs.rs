//! SQS bindings for Rust
//!
//! Check [SQSHelper](http://dualspark.github.io/rusoto/rusoto/sqs/struct.SQSHelper.html) for convenience functions.
//!

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

/// Easier to use SQS client: wraps SQSClient class.
pub struct SQSHelper<'a> {
	client: SQSClient<'a>
}

impl<'a> SQSHelper<'a> {
	/// Creates a new SQS helper
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> SQSHelper<'a> {
		SQSHelper { client: SQSClient::new(credentials, region) }
	}

	/// Lists queues
	pub fn list_queues(&mut self) -> Result<ListQueuesResult, AWSError> {
		self.client.list_queues(&ListQueuesRequest::default())
	}

	/// Creates a new queue with given name
	pub fn create_queue(&mut self, queue_name: &str) -> Result<CreateQueueResult, AWSError> {
		let mut req = CreateQueueRequest::default();
		req.queue_name = queue_name.to_string();
		self.create_queue_with_request(&req)
	}

	/// Create queue with options specified in request
	pub fn create_queue_with_request(&mut self, request: &CreateQueueRequest) -> Result<CreateQueueResult, AWSError> {
		self.client.create_queue(&request)
	}

	/// Gets a queue URL by the queue's name
	pub fn get_queue_url(&mut self, queue_name: &str) -> Result<GetQueueUrlResult, AWSError> {
		let mut req = GetQueueUrlRequest::default();
		req.queue_name = queue_name.to_string();
		self.client.get_queue_url(&req)
	}

	/// Send message to specified queue
	pub fn send_message(&mut self, queue_url: &str, message_body: &str) -> Result<SendMessageResult, AWSError> {
		let mut req = SendMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.message_body = message_body.to_string();
		self.send_message_with_request(&req)
	}

	/// Send message with specified request options
	pub fn send_message_with_request(&mut self, request: &SendMessageRequest) -> Result<SendMessageResult, AWSError> {
		self.client.send_message(&request)
	}

	/// Receive a message from specified queue
	pub fn receive_message(&mut self, queue_url: &str) -> Result<ReceiveMessageResult, AWSError> {
		let mut req = ReceiveMessageRequest::default();
		req.queue_url = queue_url.to_string();
		self.receive_message_with_request(&req)
	}

	/// Receive message with specified request options
	pub fn receive_message_with_request(&mut self, request: &ReceiveMessageRequest)-> Result<ReceiveMessageResult, AWSError> {
		self.client.receive_message(&request)
	}

	/// Delete a message from the specified queue
	pub fn delete_message(&mut self, queue_url: &str, receipt_handle: &str) -> Result<(), AWSError> {
		let mut req = DeleteMessageRequest::default();
		req.queue_url = queue_url.to_string();
		req.receipt_handle = receipt_handle.to_string();
		self.delete_message_with_request(&req)
	}

	/// Delete message with specified request options
	pub fn delete_message_with_request(&mut self, request: &DeleteMessageRequest) -> Result<(), AWSError> {
		self.client.delete_message(&request)
	}

	/// Delete the specified queue
	pub fn delete_queue(&mut self, queue_url: &str) -> Result<(), AWSError> {
		let mut req = DeleteQueueRequest::default();
		req.queue_url = queue_url.to_string();
		self.delete_queue_with_request(&req)
	}

	/// Delete the queue with specified request options
	pub fn delete_queue_with_request(&mut self, request: &DeleteQueueRequest) -> Result<(), AWSError> {
		self.client.delete_queue(&request)
	}

}
