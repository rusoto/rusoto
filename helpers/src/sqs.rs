//! SQS helper types.

#![allow(unused_variables, unused_mut)]

use rusoto_core::{Region, RusotoResult};
use rusoto_sqs::SqsClient;
use rusoto_sqs::*;

/// Easier to use SQS client: wraps SqsClient class.
pub struct SqsHelper {
    client: SqsClient,
}

impl SqsHelper {
    /// Creates a new SQS helper
    pub fn new(region: Region) -> SqsHelper {
        SqsHelper { client: SqsClient::new(region) }
    }

    /// Lists queues
    pub async fn list_queues(&mut self) -> RusotoResult<ListQueuesResult, ListQueuesError> {
        self.client.list_queues(ListQueuesRequest::default()).await
    }

    /// Creates a new queue with given name
    pub async fn create_queue(&mut self, queue_name: &str) -> RusotoResult<CreateQueueResult, CreateQueueError> {
        let mut req = CreateQueueRequest::default();
        req.queue_name = queue_name.to_string();
        self.create_queue_with_request(req).await
    }

    /// Create queue with options specified in request
    pub async fn create_queue_with_request(&mut self,
                                     request: CreateQueueRequest)
                                     -> RusotoResult<CreateQueueResult, CreateQueueError> {
        self.client.create_queue(request).await
    }

    /// Gets a queue URL by the queue's name
    pub async fn get_queue_url(&mut self, queue_name: &str) -> RusotoResult<GetQueueUrlResult, GetQueueUrlError> {
        let mut req = GetQueueUrlRequest::default();
        req.queue_name = queue_name.to_string();
        self.client.get_queue_url(req).await
    }

    /// Send message to specified queue
    pub async fn send_message(&mut self,
                        queue_url: &str,
                        message_body: &str)
                        -> RusotoResult<SendMessageResult, SendMessageError> {
        let mut req = SendMessageRequest::default();
        req.queue_url = queue_url.to_string();
        req.message_body = message_body.to_string();
        self.send_message_with_request(req).await
    }

    /// Send message with specified request options
    pub async fn send_message_with_request(&mut self,
                                     request: SendMessageRequest)
                                     -> RusotoResult<SendMessageResult, SendMessageError> {
        self.client.send_message(request).await
    }

    /// Receive a message from specified queue
    pub async fn receive_message(&mut self, queue_url: &str) -> RusotoResult<ReceiveMessageResult, ReceiveMessageError> {
        let mut req = ReceiveMessageRequest::default();
        req.queue_url = queue_url.to_string();
        self.receive_message_with_request(req).await
    }

    /// Receive message with specified request options
    pub async fn receive_message_with_request(&mut self,
                                        request: ReceiveMessageRequest)
                                        -> RusotoResult<ReceiveMessageResult, ReceiveMessageError> {
        self.client.receive_message(request).await
    }

    /// Delete a message from the specified queue
    pub async fn delete_message(&mut self,
                          queue_url: &str,
                          receipt_handle: &str)
                          -> RusotoResult<(), DeleteMessageError> {
        let mut req = DeleteMessageRequest::default();
        req.queue_url = queue_url.to_string();
        req.receipt_handle = receipt_handle.to_string();
        self.delete_message_with_request(req).await
    }

    /// Delete message with specified request options
    pub async fn delete_message_with_request(&mut self,
                                       request: DeleteMessageRequest)
                                       -> RusotoResult<(), DeleteMessageError> {
        self.client.delete_message(request).await
    }

    /// Delete the specified queue
    pub async fn delete_queue(&mut self, queue_url: &str) -> RusotoResult<(), DeleteQueueError> {
        let mut req = DeleteQueueRequest::default();
        req.queue_url = queue_url.to_string();
        self.delete_queue_with_request(req).await
    }

    /// Delete the queue with specified request options
    pub async fn delete_queue_with_request(&mut self,
                                     request: DeleteQueueRequest)
                                     -> RusotoResult<(), DeleteQueueError> {
        self.client.delete_queue(request).await
    }
}
