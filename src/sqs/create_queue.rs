use sqs::*;

impl SQSClient {	
	/// Convenience method to create a queue with a name and default attributes.
	/// Queues can be created with specified attributes with SQSClient.service_request(CreateQueue)
	pub fn create_queue(&self, queue_name: &str) -> Result<CreateQueueResponse, SQSError> {
		let mut req = CreateQueue::default();
		req.queue_name = queue_name.to_string();
		self.service_request(req)
	}	
}

impl CreateQueueResponse {
	pub fn get_queue_url<'a>(&'a self) -> &'a str {
		&self.create_queue_result.queue_url
	}
}