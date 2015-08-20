#![allow(dead_code)]
extern crate rusoto;
extern crate xml;
extern crate time;
extern crate regex;
extern crate rustc_serialize;
use rusoto::credentials::*;
use rusoto::error::*;
use rusoto::sqs::*;
use rusoto::s3::*;
use time::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;
// use std::thread;

fn main() {
	let mut provider = DefaultAWSCredentialsProviderChain::new();

	match sqs_roundtrip_tests(&provider.get_credentials()) {
		Ok(_) => { println!("Everything worked."); },
		Err(err) => { println!("Got error: {:#?}", err); }
	}


	let bucket_name = format!("rusoto{}", get_time().sec);

	match s3_list_buckets_tests(&provider.get_credentials()) {
		Ok(_) => { println!("Everything worked for S3 list buckets."); },
		Err(err) => { println!("Got error in s3 list buckets: {:#?}", err); }
	}

	match s3_create_bucket_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => { println!("Everything worked for S3 create bucket."); },
		Err(err) => { println!("Got error in s3 create bucket: {:#?}", err); }
	}

	match s3_put_object_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 put object.");
		}
		Err(err) => { println!("Got error in s3 put object: {:#?}", err); }
	}

	match s3_get_object_test(&provider.get_credentials(), &bucket_name) {
		Ok(result) => {
			println!("Everything worked for S3 get object.");
			let mut f = File::create("s3-sample-creds").unwrap();
			match f.write(&(result.body)) {
				Err(why) => println!("Couldn't create file to save object from S3: {}", why),
				Ok(_) => println!("Pretend this is a noop"),
			}
		}
		Err(err) => { println!("Got error in s3 get object: {:#?}", err); }
	}

	match s3_delete_object_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {:#?}", err); }
	}

	match s3_put_object_with_reduced_redundancy_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 put object with reduced redundancy.");
		}
		Err(err) => { println!("Got error in s3 put object: {:#?}", err); }
	}

	match s3_delete_object_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {:#?}", err); }
	}

	match s3_delete_bucket_test(&provider.get_credentials(), &bucket_name) {
		Ok(_) => { println!("Everything worked for S3 delete bucket."); },
		Err(err) => { println!("Got error in s3 delete bucket: {:#?}", err); }
	}
}

fn s3_list_buckets_tests(creds: &AWSCredentials) -> Result<(), AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	let response = try!(s3.list_buckets());
	// println!("response is {:?}", response);
	for q in response.buckets {
		println!("Existing bucket: {:?}", q.name);
	}

	Ok(())
}

fn s3_get_object_test(creds: &AWSCredentials, bucket: &str) -> Result<GetObjectOutput, AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	let response = try!(s3.get_object(bucket, "sample-credentials"));
	// println!("get object response is {:?}", response);
	Ok(response)
}

fn s3_delete_object_test(creds: &AWSCredentials, bucket: &str) -> Result<DeleteObjectOutput, AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	let response = try!(s3.delete_object(bucket, "sample-credentials"));
	// println!("delete object response is {:?}", response);
	Ok(response)
}

fn s3_put_object_test(creds: &AWSCredentials, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let response = try!(s3.put_object(bucket, "sample-credentials", &contents));
			Ok(response)
		}
	}
}

fn s3_put_object_with_reduced_redundancy_test(creds: &AWSCredentials, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let response = try!(s3.put_object_with_reduced_redundancy(bucket, "sample-credentials", &contents));
			Ok(response)
		}
	}
}

fn s3_create_bucket_test(creds: &AWSCredentials, bucket: &str) -> Result<(), AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	try!(s3.create_bucket_in_region(bucket, "us-east-1"));

	Ok(())
}

fn s3_delete_bucket_test(creds: &AWSCredentials, bucket: &str) -> Result<(), AWSError> {
	let s3 = S3Helper::new(&creds, "us-east-1");

	try!(s3.delete_bucket(bucket, "us-east-1"));
	Ok(())
}

fn sqs_roundtrip_tests(creds: &AWSCredentials) -> Result<(), AWSError> {
	let sqs = SQSHelper::new(&creds, "us-east-1");

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
