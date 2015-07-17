#![allow(dead_code)]
extern crate rusoto;
extern crate xml;
extern crate time;
use std::env::*;
use rusoto::credentials::*;
use rusoto::error::*;
use rusoto::sqs::*;
use time::*;

fn main() {
	if let Ok(creds) = creds_from_env() {
		match sqs_roundtrip_tests(creds) {
			Ok(_) => { println!("Everything worked."); },
			Err(err) => { println!("Got error: {:#?}", err); }
		}			
	} else {
		println!("Set AWS credentials with environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_KEY");
	}	
}

fn sqs_roundtrip_tests(creds: AWSCredentials) -> Result<(), AWSError> {
	
	let sqs = SQSClient::new(creds, "us-east-1");
	
	// list existing queues
	let response = try!(sqs.list_queues());	
	for q in response.queue_urls {
		println!("Existing queue: {}", q);
	}
	
	// create a new queue
	let q_name = &format!("test_q_{}", get_time().sec);
	let response = try!(sqs.create_queue(q_name));
	println!("Created queue {} with url {}", q_name, response.queue_url);
	
	// query it by name
	let response = try!(sqs.get_queue_url(q_name));
	let queue_url = &response.queue_url;
	println!("Verified queue url {} for queue name {}", queue_url, q_name);
	
	// send it a message
	let msg_str = "lorem ipsum dolor sit amet";
	let response = try!(sqs.send_message(queue_url, msg_str));
	println!("Send message with body '{}' and created message_id {}", msg_str, response.message_id);
	
	// receive a message
	let response = try!(sqs.receive_message(queue_url));
	for msg in response.messages {
		println!("Received message '{}' with id {}", msg.body, msg.message_id);
		try!(sqs.delete_message(queue_url, &msg.receipt_handle));
	}

	// delete the queue
	try!(sqs.delete_queue(queue_url));
	println!("Queue {} deleted", queue_url);

	Ok(())
}

fn creds_from_env() -> Result<AWSCredentials, VarError> {
	let env_key = try!(var("AWS_ACCESS_KEY_ID"));
	let env_secret = try!(var("AWS_SECRET_KEY"));

	Ok(AWSCredentials::new(&env_key, &env_secret)) 
}
