#![allow(dead_code)]
#[macro_use]
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
use rusoto::dynamodb::{DynamoDBHelper, CreateTableInput, AttributeDefinition, KeySchemaElement};
use rusoto::dynamodb::{DynamoDBError, PutItemInput, PutItemInputAttributeMap, AttributeValue};
use rusoto::dynamodb::{GetItemInput, GetItemOutput, Key, get_string_from_attribute};
use std::thread;
use time::*;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {

    let creds = DefaultAWSCredentialsProviderChain::new();
    let region = Region::UsWest2;

    let mut dynamodb = DynamoDBHelper::new(creds, &region);

    match dynamo_list_tables_tests(&mut dynamodb) {
        Ok(_) => {
            println!("List tables OK");
        }
        Err(err) => {
            println!("Error getting table list: {:#?}", err);
        }
    }

    let table_name = &format!("test_table_{}", get_time().sec);

    match dynamo_create_table_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Issued create table command for {}", table_name);
        }
        Err(err) => {
            println!("Error creating table {:#?}", err);
        }
    }

    match dynamo_describe_wait_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Table {} is now active", table_name);
        }
        Err(err) => {
            println!("Error waiting for table to become active {:#?}", err);
        }
    }

    let mut item = Key::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));

    match dynamo_get_item_test(&mut dynamodb, &table_name, item) {
        Ok(item_from_dynamo) => {
            println!("Got item back from Dynamo");
            match item_from_dynamo.Item {
                None => println!("nothing received from Dynamo, item may not exist"),
                Some(attributes_map) => {
                    for (column_name, value) in attributes_map {
                        println!("found column name '{}' with value of '{}'", column_name, get_string_from_attribute(&value).unwrap());
                    }
                },
            }
        },
        Err(err) => {
            println!("Error retrieving object: {:?}", err);
        }
    }

    match dynamo_put_item_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Put item to {}", table_name);
        }
        Err(err) => {
            println!("Error putting item to table {:#?}", err);
        }
    }

    println!("Trying the dynamo get again");
    item = Key::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));
    match dynamo_get_item_test(&mut dynamodb, &table_name, item) {
        Ok(item_from_dynamo) => {
            println!("Got item back from Dynamo");
            match item_from_dynamo.Item {
                None => println!("nothing received from Dynamo, item may not exist"),
                Some(attributes_map) => {
                    for (column_name, value) in attributes_map {
                        println!("found column name '{}' with value of '{}'", column_name, get_string_from_attribute(&value).unwrap());
                    }
                },
            }
        },
        Err(err) => {
            println!("Error retrieving object: {:?}", err);
        }
    }

    match dynamo_delete_table_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Deleted table {}", table_name);
        }
        Err(err) => {
            println!("Error deleting DynamoDB table {:#?}", err);
        }
    }

    let provider = DefaultAWSCredentialsProviderChain::new();
	let region = Region::UsEast1;

	let provider2 = DefaultAWSCredentialsProviderChain::new();

	// Creates an SQS client with its own copy of the credential provider chain:
	let mut sqs = SQSHelper::new(provider2, &region);

	match sqs_roundtrip_tests(&mut sqs) {
		Ok(_) => { println!("Everything worked."); },
		Err(err) => { println!("Got error: {}", err); }
	}

	// S3 client gets its own provider chain:
	let mut s3 = S3Helper::new(provider.clone(), &region);

	match s3_list_buckets_tests(&mut s3) {
		Ok(_) => { println!("Everything worked for S3 list buckets."); },
		Err(err) => { println!("Got error in s3 list buckets: {}", err); }
	}

	let mut bucket_name = format!("rusoto{}", get_time().sec);
	// let bucket_name = "rusoto1440826511";

	match s3_create_bucket_test(&mut s3, &bucket_name, &region, None) {
		Ok(_) => { println!("Everything worked for S3 create bucket."); },
		Err(err) => { println!("Got error in s3 create bucket: {}", err); }
	}

	match s3_put_object_with_request_specified_test(&mut s3, &bucket_name) {
		Ok(_) => println!("Everything worked for S3 put object."),
		Err(err) => println!("Got error in s3 put object: {}", err),
	}

	match s3_put_object_test(&mut s3, &bucket_name) {
		Ok(_) => println!("Everything worked for S3 put object."),
		Err(err) => println!("Got error in s3 put object: {}", err),
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
		Err(err) => { println!("Got error in s3 get object: {}", err); }
	}

	match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {}", err); }
	}

	match s3_put_object_with_reduced_redundancy_test(&mut s3, &bucket_name) {
		Ok(_) => {
			println!("Everything worked for S3 put object with reduced redundancy.");
		}
		Err(err) => { println!("Got error in s3 put object with reduced redundancy: {}", err); }
	}

	match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
		Ok(_) => {
			println!("Everything worked for S3 delete object.");
		}
		Err(err) => { println!("Got error in s3 delete object: {}", err); }
	}

	// Set the file in s3_multipart_upload_test and uncomment this code to test multipart upload:
	// println!("Making a large upload...");
	// match s3_multipart_upload_test(&mut s3, &bucket_name) {
	// 	Ok(_) => { println!("Everything worked for S3 multipart upload."); }
	// 	Err(err) => { println!("Got error in s3 multipart upload: {}", err); }
	// }

	// match s3_delete_object_test(&mut s3, &bucket_name, "testfile.zip") {
	// 	Ok(_) => {
	// 		println!("Everything worked for S3 delete object.");
	// 	}
	// 	Err(err) => { println!("Got error in s3 delete object: {}", err); }
	// }

	match s3_list_multipart_uploads(&mut s3, &bucket_name) {
		Err(why) => println!("Error listing multipart uploads: {:?}", why),
		Ok(_) => (),
	}

	// Working example, replace bucket name, file name, uploadID for your multipart upload:
	// match s3_list_multipart_upload_parts(&mut s3, &bucket_name, "testfile.zip", "PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK") {
	// 	Err(why) => println!("Error listing multipart upload parts: {:?}", why),
	// 	Ok(_) => (),
	// }

	// Working example, replace bucket name, file name, uploadID for your multipart upload:
	// match s3_abort_multipart_uploads(&mut s3, &bucket_name, "testfile.zip", "W5J7SeEor1A3vcRMMUhAb.BKrMs68.suzyhErssdb2HFAyDb4z7QhJBMyGkM_GSsoFqKJJLjbHcNSZTHa7MhTFJodewzcswshoDHd7mffXPNUH.xoRWVXbkLjakTETaO") {
	// 	Err(why) => println!("Error aborting multipart uploads: {:?}", why),
	// 	Ok(_) => (),
	// }

	match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
		Ok(_) => { println!("Everything worked for S3 delete bucket."); },
		Err(err) => { println!("Got error in s3 delete bucket: {}", err); }
	}

	// new bucket for canned acl testing!
	bucket_name = format!("rusoto{}", get_time().sec);

	match s3_create_bucket_test(&mut s3, &bucket_name, &region, Some(CannedAcl::AuthenticatedRead)) {
		Ok(_) => { println!("Everything worked for S3 create bucket with ACL."); },
		Err(err) => { println!("Got error in s3 create bucket: {}", err); }
	}

	match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
		Ok(_) => { println!("Everything worked for S3 delete bucket."); },
		Err(err) => { println!("Got error in s3 delete bucket: {}", err); }
	}
}

fn s3_list_multipart_upload_parts(s3: &mut S3Helper, bucket: &str, object: &str, upload_id: &str) -> Result<(), AWSError> {
	match s3.multipart_upload_list_parts(bucket, object, upload_id) {
		Err(why) => println!("Error listing multipart upload parts: {:?}", why),
		Ok(result) => println!("Multipart upload parts: {:?}", result),
	}
	Ok(())
}

fn s3_list_multipart_uploads(s3: &mut S3Helper, bucket: &str) -> Result<(), AWSError> {
	match s3.list_multipart_uploads_for_bucket(bucket) {
		Err(why) => println!("Error listing multipart uploads: {:?}", why),
		Ok(result) => println!("in-progress multipart uploads: {:?}", result),
	}
	Ok(())
}

fn s3_abort_multipart_uploads(s3: &mut S3Helper, bucket: &str, object: &str, upload_id: &str) -> Result<(), AWSError> {
	match s3.abort_multipart_upload(bucket, object, upload_id) {
		Err(why) => println!("Error aborting multipart upload: {:?}", why),
		Ok(result) => println!("aborted multipart upload: {:?}", result),
	}
	Ok(())
}

fn s3_list_buckets_tests(s3: &mut S3Helper) -> Result<(), AWSError> {
	let response = try!(s3.list_buckets());
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

fn s3_put_object_aws_encryption_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents : Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let response = try!(s3.put_object_with_aws_encryption(bucket, "sample-credentials", &contents));
			Ok(response)
		}
	}
}

fn s3_put_object_kms_encryption_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents : Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let response = try!(s3.put_object_with_kms_encryption(bucket, "sample-credentials", &contents, "key-id"));
			Ok(response)
		}
	}
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

fn s3_put_object_with_request_specified_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	let mut f = File::open("src/sample-credentials").unwrap();
	let mut contents : Vec<u8> = Vec::new();
	match f.read_to_end(&mut contents) {
		Err(why) => return Err(AWSError::new(format!("Error opening file to send to S3: {}", why))),
		Ok(_) => {
			let mut request = PutObjectRequest::default();
			request.key = "sample-credentials".to_string();
			request.bucket = bucket.to_string();
			request.body = Some(&contents);
			// request.content_md5 = Some("foo".to_string());

			let response = try!(s3.put_object_with_request(&mut request));

			Ok(response)
		}
	}
}

fn s3_multipart_upload_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AWSError> {
	// Set to a > 5 MB file for testing:
	let mut f = File::open("testfile.zip").unwrap();

	let response = try!(s3.put_multipart_object(bucket, "testfile.zip", &mut f));
	Ok(response)
}

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

fn dynamo_list_tables_tests(dynamodb: &mut DynamoDBHelper) -> Result<(), DynamoDBError> {
    let response = try!(dynamodb.list_tables());
    println!("{:#?}", response);
    Ok(())
}

fn dynamo_create_table_test(dynamodb: &mut DynamoDBHelper,
                            table_name: &str)
                            -> Result<(), DynamoDBError> {
    println!("Creating table {} ", table_name);

    let input = CreateTableInput::new()
                        .with_name(table_name)
                        .with_write_capacity(1)
                        .with_read_capacity(1)
                        .with_attributes(attributes!("string" => "S", "number" => "N"))
                        .with_key_schema(key_schema!("string" => "HASH", "number" => "RANGE"));

    let _result = try!(dynamodb.create_table(&input));
    Ok(())
}

fn dynamo_put_item_test(dynamodb: &mut DynamoDBHelper, table_name: &str) -> Result<(), DynamoDBError> {
    let mut input = PutItemInput::default();

    let mut item = PutItemInputAttributeMap::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));

    input.Item = item;
    input.TableName = table_name.to_string();

    try!(dynamodb.put_item(&input));

    Ok(())
}

fn dynamo_get_item_test(dynamodb: &mut DynamoDBHelper, table_name: &str, item_key: Key) -> Result<GetItemOutput, DynamoDBError> {
    let mut item_request = GetItemInput::default();
    item_request.Key = item_key;
    item_request.TableName = table_name.to_string();

    match dynamodb.get_item(&item_request) {
        Err(why) => Err(why),
        Ok(output) => Ok(output),
    }
}

fn dynamo_describe_wait_test(dynamodb: &mut DynamoDBHelper,
                             table_name: &str)
                             -> Result<(), DynamoDBError> {

    loop {
        let result = try!(dynamodb.describe_table(table_name));

        if let Some(ref status) = result.get_status() {
            if status == "ACTIVE" {
                break;
            } else {
                println!("\t{} not ready - {}", table_name, status);
                thread::sleep_ms(1000);
            }
        }

    }
    Ok(())
}

fn dynamo_delete_table_test(dynamodb: &mut DynamoDBHelper,
                            table_name: &str)
                            -> Result<(), DynamoDBError> {
    let _result = try!(dynamodb.delete_table(table_name));
    Ok(())
}
