use sqs::*;
//use std::ops::Deref;

impl GetQueueUrl {
	pub fn new(queue_name: &str) -> GetQueueUrl {
		let mut req = GetQueueUrl::default();
		req.queue_name = queue_name.to_string();
		req
	}

	pub fn set_queue_owner(&mut self, aws_account_id: &str) {
		self.queue_owner_awsaccount_id = Some(aws_account_id.to_string());
	}
}


impl GetQueueUrlResponse {
	pub fn get_queue_url(&self) -> &str {
		&self.get_queue_url_result.queue_url
	}
}

/*
impl Deref for GetQueueUrlResponse {
	type Target = String;
	
	fn deref<'a>(&'a self) -> &'a String {
		&self.get_queue_url()
	}
}*/