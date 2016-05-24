//! SQS helper types.

#![allow(unused_variables, unused_mut)]

use rusoto::{AwsResult, ProvideAwsCredentials, Region};
use rusoto::sqs::SqsClient;
use rusoto::sqs::{GetQueueUrlRequest, GetQueueUrlResult, SendMessageRequest, SendMessageResult};
use rusoto::sqs::{ReceiveMessageRequest, ReceiveMessageResult, ListQueuesRequest, ListQueuesResult};
use rusoto::sqs::{CreateQueueRequest, CreateQueueResult, DeleteMessageRequest, DeleteQueueRequest};

/// Easier to use SQS client: wraps SqsClient class.
pub struct SqsHelper<P> where P: ProvideAwsCredentials {
    client: SqsClient<P>,
}

impl <P: ProvideAwsCredentials> SqsHelper<P> {
    /// Creates a new SQS helper
    pub fn new(credentials: P, region: Region) -> SqsHelper<P> {
        SqsHelper { client: SqsClient::new(credentials, region) }
    }

    /// Lists queues
    pub fn list_queues(&mut self) -> AwsResult<ListQueuesResult> {
        self.client.list_queues(&ListQueuesRequest::default())
    }

    /// Creates a new queue with given name
    pub fn create_queue(&mut self, queue_name: &str) -> AwsResult<CreateQueueResult> {
        let mut req = CreateQueueRequest::default();
        req.queue_name = queue_name.to_string();
        self.create_queue_with_request(&req)
    }

    /// Create queue with options specified in request
    pub fn create_queue_with_request(&mut self,
                                     request: &CreateQueueRequest)
                                     -> AwsResult<CreateQueueResult> {
        self.client.create_queue(&request)
    }

    /// Gets a queue URL by the queue's name
    pub fn get_queue_url(&mut self, queue_name: &str) -> AwsResult<GetQueueUrlResult> {
        let mut req = GetQueueUrlRequest::default();
        req.queue_name = queue_name.to_string();
        self.client.get_queue_url(&req)
    }

    /// Send message to specified queue
    pub fn send_message(&mut self,
                        queue_url: &str,
                        message_body: &str)
                        -> AwsResult<SendMessageResult> {
        let mut req = SendMessageRequest::default();
        req.queue_url = queue_url.to_string();
        req.message_body = message_body.to_string();
        self.send_message_with_request(&req)
    }

    /// Send message with specified request options
    pub fn send_message_with_request(&mut self,
                                     request: &SendMessageRequest)
                                     -> AwsResult<SendMessageResult> {
        self.client.send_message(&request)
    }

    /// Receive a message from specified queue
    pub fn receive_message(&mut self, queue_url: &str) -> AwsResult<ReceiveMessageResult> {
        let mut req = ReceiveMessageRequest::default();
        req.queue_url = queue_url.to_string();
        self.receive_message_with_request(&req)
    }

    /// Receive message with specified request options
    pub fn receive_message_with_request(&mut self,
                                        request: &ReceiveMessageRequest)
                                        -> AwsResult<ReceiveMessageResult> {
        self.client.receive_message(&request)
    }

    /// Delete a message from the specified queue
    pub fn delete_message(&mut self,
                          queue_url: &str,
                          receipt_handle: &str)
                          -> AwsResult<()> {
        let mut req = DeleteMessageRequest::default();
        req.queue_url = queue_url.to_string();
        req.receipt_handle = receipt_handle.to_string();
        self.delete_message_with_request(&req)
    }

    /// Delete message with specified request options
    pub fn delete_message_with_request(&mut self,
                                       request: &DeleteMessageRequest)
                                       -> AwsResult<()> {
        self.client.delete_message(&request)
    }

    /// Delete the specified queue
    pub fn delete_queue(&mut self, queue_url: &str) -> AwsResult<()> {
        let mut req = DeleteQueueRequest::default();
        req.queue_url = queue_url.to_string();
        self.delete_queue_with_request(&req)
    }

    /// Delete the queue with specified request options
    pub fn delete_queue_with_request(&mut self,
                                     request: &DeleteQueueRequest)
                                     -> AwsResult<()> {
        self.client.delete_queue(&request)
    }
}
