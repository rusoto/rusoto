use sqs::*;

impl SQSClient {

	/// Convenience method for adding a single permission to a queue.  Multiple permissions should be added
	/// by calling SQSClient.queue_request(AddPermission)			
	pub fn add_permission(&self, queue_url: &str, account_id: &str, action: &str) -> Result<AddPermissionResponse, SQSError> {
		let mut req = AddPermission::default();
		req.add_action(account_id, action);
		self.queue_request(queue_url, req)
	}

}

impl AddPermission {
	pub fn new(label: &str) -> AddPermission {
		let mut req = AddPermission::default();
		req.label = label.to_string();
		req
	}
	
	pub fn add_action(&mut self, account_id: &str, action: &str) {
		self.awsaccount_id.push(account_id.to_string());
		self.action_name.push(action.to_string());
	} 
}
