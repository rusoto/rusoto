use sqs::*;

impl SQSClient {
	/// Convenience method for sending a simple message with a string body and no attributes specified.
	/// More complicated messages can be sent with SQSClient.queue_request(queue_url, SendMessage)	
	pub fn send_message(&self, queue_url: &str, body: &str) -> Result<SendMessageResponse, SQSError> {
		let mut req = SendMessage::default();
		req.message_body = body.to_string();
		self.queue_request(queue_url, req)
	}
}

impl SendMessageResponse {
	pub fn get_message_id(&self) -> &str {
		&self.send_message_result.message_id
	}
}