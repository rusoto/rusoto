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
use rusoto::regions::*;
use time::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
	let provider = DefaultAWSCredentialsProviderChain::new();
	let region = Region::UsEast1;

	let provider2 = ProfileCredentialsProvider::new();

	// Creates an SQS client with its own copy of the credential provider chain:
	let mut sqs = SQSHelper::new(provider2, &region);

	match sqs_roundtrip_tests(&mut sqs) {
		Ok(_) => { println!("Everything worked."); },
		Err(err) => { println!("Got error: {:#?}", err); }
	}

	// S3 client gets its own provider chain:
	let mut s3 = S3Helper::new(provider.clone(), &region);

	match s3_list_buckets_tests(&mut s3) {
		Ok(_) => { println!("Everything worked for S3 list buckets."); },
		Err(err) => { println!("Got error in s3 list buckets: {:#?}", err); }
	}

	let mut bucket_name = format!("rusoto{}", get_time().sec);

	match s3_create_bucket_test(&mut s3, &bucket_name, &region, None) {
		Ok(_) => { println!("Everything worked for S3 create bucket."); },
		Err(err) => { println!("Got error in s3 create bucket: {:#?}", err); }
	}

	match s3_put_object_test(&mut s3, &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 put object.");
		}
		Err(err) => { println!("Got error in s3 put object: {:#?}", err); }
	}

	match s3_get_object_test(&mut s3, &bucket_name) {
		Ok(result) => {
			println!("Everything worked for S3 get object.");
			let mut f = File::create("s3-sample-creds").unwrap();
			match f.write(&(result.body)) {
				Err(why) => println!("Couldn't create file to save object from S3: {}", why),
				Ok(_) => (),
			}
		}
		Err(err) => { println!("Got error in s3 get object: {:#?}", err); }
	}

	match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {:#?}", err); }
	}

	match s3_put_object_with_reduced_redundancy_test(&mut s3, &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 put object with reduced redundancy.");
		}
		Err(err) => { println!("Got error in s3 put object with reduced redundancy: {:#?}", err); }
	}

	match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {:#?}", err); }
	}

	// Set the file in s3_multipart_upload_test and uncomment this code to test multipart upload:
	// println!("Making a large upload...");
	// match s3_multipart_upload_test(&mut s3, &bucket_name) {
	// 	Ok(_) => { println!("Everything worked for S3 multipart upload."); }
	// 	Err(err) => { println!("Got error in s3 multipart upload: {:#?}", err); }
	// }

	match s3_delete_object_test(&mut s3, &bucket_name, "join.me.zip") {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {:#?}", err); }
	}

	match s3_list_multipart_uploads(&mut s3, &bucket_name) {
		Err(why) => println!("Error listing multipart uploads: {:?}", why),
		Ok(_) => println!("yay listed."),
	}

	match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
		Ok(_) => { println!("Everything worked for S3 delete bucket."); },
		Err(err) => { println!("Got error in s3 delete bucket: {:#?}", err); }
	}

	// new bucket for canned acl testing!
	bucket_name = format!("rusoto{}", get_time().sec);

	match s3_create_bucket_test(&mut s3, &bucket_name, &region, Some(CannedAcl::AuthenticatedRead)) {
		Ok(_) => { println!("Everything worked for S3 create bucket with ACL."); },
		Err(err) => { println!("Got error in s3 create bucket: {:#?}", err); }
	}

	match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
		Ok(_) => { println!("Everything worked for S3 delete bucket."); },
		Err(err) => { println!("Got error in s3 delete bucket: {:#?}", err); }
	}
}

fn s3_list_multipart_uploads(s3: &mut S3Helper, bucket: &str) -> Result<(), AWSError> {
	match s3.list_multipart_uploads_for_bucket(bucket) {
		Err(why) => println!("Error listing multipart uploads: {:?}", why),
		Ok(result) => println!("in-progress multipart uploads: {:?}", result),
	}
	Ok(())
}

fn s3_list_buckets_tests(s3: &mut S3Helper) -> Result<(), AWSError> {
	let response = try!(s3.list_buckets());
	// println!("response is {:?}", response);
	for q in response.buckets {
		println!("Existing bucket: {:?}", q.name);
	}

	Ok(())
}

fn s3_get_object_test(s3: &mut S3Helper, bucket: &str) -> Result<GetObjectOutput, AWSError> {
	let response = try!(s3.get_object(bucket, "sample-credentials"));
	Ok(response)
}

fn s3_delete_object_test(s3: &mut S3Helper, bucket: &str, object_name: &str) -> Result<DeleteObjectOutput, AWSError> {
	let response = try!(s3.delete_object(bucket, object_name));
	Ok(response)
}

fn s3_put_object_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents : Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let response = try!(s3.put_object(bucket, "sample-credentials", &contents));
			Ok(response)
		}
	}
}

// uncomment for multipart upload testing:
// fn s3_multipart_upload_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	// Set to a > 5 MB file for testing:
	// let mut f = File::open("/Users/matthewmayer/Downloads/join.me.zip").unwrap();
	//
	// let response = try!(s3.put_multipart_object(bucket, "join.me.zip", &mut f));
	// Ok(response)
// }

fn s3_put_object_with_reduced_redundancy_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
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

fn s3_create_bucket_test(s3: &mut S3Helper, bucket: &str, region: &Region, canned_acl: Option<CannedAcl>) -> Result<(), AWSError> {
	try!(s3.create_bucket_in_region(bucket, &region, canned_acl));

	Ok(())
}

fn s3_delete_bucket_test(s3: &mut S3Helper, bucket: &str, region: &Region) -> Result<(), AWSError> {
	try!(s3.delete_bucket(bucket, &region));
	Ok(())
}

fn sqs_roundtrip_tests(sqs: &mut SQSHelper) -> Result<(), AWSError> {
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
