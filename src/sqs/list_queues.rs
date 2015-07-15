use sqs::*;

impl SQSClient {
	/// Convenience method to return a list of all queues	
	pub fn list_queues(&self) -> Result<ListQueuesResponse, SQSError> {
		self.service_request(ListQueues::default())
	}
}

impl ListQueuesResponse {
	pub fn get_queues(&self) -> &Vec<String> {
		&self.list_queues_result.queue_url
	}
	
}