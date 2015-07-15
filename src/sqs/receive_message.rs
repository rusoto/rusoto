use sqs::*;

impl SQSClient {
	/// Convenience method to receive a single message from the specified queue
	/// More detailed message listening should be done with SQSClient.queue_request(queue_url, ReceiveMessage)
	pub fn receive_message(&self, queue_url: &str) -> Result<ReceiveMessageResponse, SQSError> {
		let req = ReceiveMessage::default();
		self.queue_request(queue_url, req)
	}
}

impl ReceiveMessageResponse {
	pub fn get_messages(&self) -> &Vec<Message> {
		&self.receive_message_result.message
	}
}