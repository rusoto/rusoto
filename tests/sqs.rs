#![cfg(feature = "sqs")]

extern crate rusoto;
extern crate time;
extern crate env_logger;

use rusoto::sqs::{Sqs, SqsClient};
use rusoto::sqs::{ListQueuesRequest, CreateQueueRequest, GetQueueUrlRequest, SendMessageRequest};
use rusoto::sqs::{ReceiveMessageRequest, DeleteMessageRequest, DeleteQueueRequest, GetQueueAttributesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn list_queues() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sqs = SqsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = ListQueuesRequest { ..Default::default() };

    let result = sqs.list_queues(&request).expect("List queues failed");
    println!("{:#?}", result);
}

#[test]
fn sqs_roundtrip_tests() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sqs = SqsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

	// create a new queue
	let q_name = &format!("test_q_{}", time::get_time().sec);
	let q_creation_req = CreateQueueRequest {
		queue_name: q_name.clone(),
		..Default::default()
	};

	let response = sqs.create_queue(&q_creation_req).expect("Create queue failed");
	println!("Created queue {} with url {}", q_name, response
		.queue_url
		.clone()
		.expect("Queue url wasn't available in response"));
	// q_url_from_aws looks like https://sqs.us-east-1.amazonaws.com/acct_id_here/test_q_1495776719
	assert!(response.queue_url.unwrap().clone().ends_with(q_name));

	// query it by name
	let get_q_by_name_request = GetQueueUrlRequest {
		queue_name: q_name.clone(),
		..Default::default()
	};
	let response = sqs.get_queue_url(&get_q_by_name_request).expect("Get queue by URL request failed");
	let queue_url = &response.queue_url.expect("Queue url should be available from list queues");
	println!("Verified queue url {} for queue name {}", queue_url.clone(), q_name);

	// queue attributes
	let queue_attributes_req = GetQueueAttributesRequest {
		queue_url: queue_url.clone(),
		attribute_names: Some(vec!["All".to_string()]),
	};
	match sqs.get_queue_attributes(&queue_attributes_req) {
		Ok(result) => println!("Queue attributes: {:?}", result),
		Err(e) => panic!("Error getting queue attributes: {:?}", e),
	}

	// send it a message
	let msg_str = String::from("lorem ipsum dolor sit amet");
	let send_msg_request = SendMessageRequest {
		message_body: msg_str.clone(),
		queue_url: queue_url.clone(),
		..Default::default()
	};
	let response = sqs.send_message(&send_msg_request);
	println!("Sent message with body '{}' and created message_id {}", msg_str, response.unwrap().message_id.unwrap());

	// message_attribute_names is for testing https://github.com/rusoto/rusoto/issues/586
	let receive_request = ReceiveMessageRequest {
		queue_url: queue_url.clone(),
		message_attribute_names: Some(vec!["All".to_string()]),
		..Default::default()
	};

	let response = sqs.receive_message(&receive_request);
	for msg in response
		.expect("Expected to have a receive message response")
		.messages
		.expect("message should be available")
	{
		println!("Received message '{}' with id {}", msg.body.clone().unwrap(), msg.message_id.clone().unwrap());

		assert!(msg.body.unwrap().eq(&msg_str));

		let delete_message_request = DeleteMessageRequest {
			queue_url: queue_url.clone(),
			receipt_handle: msg.receipt_handle.clone().unwrap()
		};
		match sqs.delete_message(&delete_message_request) {
			Ok(_) => println!("Deleted message via receipt handle {:?}", delete_message_request.receipt_handle),
			Err(e) => panic!("Couldn't delete message: {:?}", e),
		}
	}

	let queue_deletion_req = DeleteQueueRequest {
		queue_url: queue_url.clone()
	};
	match sqs.delete_queue(&queue_deletion_req) {
		Ok(_) => (),
		Err(e) => panic!("Couldn't delete queue: {:?}", e),
	}
	println!("Queue {} deleted", queue_url.clone());
}
