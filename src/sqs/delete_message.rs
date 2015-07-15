use sqs::*;

impl SQSClient {
	/// Delete a message from a queue by handle
	pub fn delete_message(&self, queue_url: &str, receipt_handle: &str) -> Result<DeleteMessageResponse, SQSError> {
		let mut req = DeleteMessage::default();
		req.receipt_handle = receipt_handle.to_string();
		self.queue_request(queue_url, req)
	}
}