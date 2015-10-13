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
use rusoto::dynamodb::DynamoDBHelper;
use rusoto::dynamodb::CreateTableInput;
use rusoto::dynamodb::AttributeDefinition;
use rusoto::dynamodb::KeySchemaElement;
use rusoto::dynamodb::DynamoDBError;
use rusoto::dynamodb::PutItemInput;
use rusoto::dynamodb::PutItemInputAttributeMap;
use rusoto::dynamodb::AttributeValue;
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

    match dynamo_put_item_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Put item to {}", table_name);
        }
        Err(err) => {
            println!("Error putting item to table {:#?}", err);
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
	// let mut sqs = SQSHelper::new(provider2, &region);

	// match sqs_roundtrip_tests(&mut sqs) {
	// 	Ok(_) => { println!("Everything worked."); },
	// 	Err(err) => { println!("Got error: {}", err); }
	// }

	// S3 client gets its own provider chain:
	let mut s3 = S3Helper::new(provider.clone(), &region);

	// works with codegen
	match s3_list_buckets_tests(&mut s3) {
		Ok(_) => { println!("Everything worked for S3 list buckets."); },
		Err(err) => { println!("Got error in s3 list buckets: {}", err); }
	}

	let mut bucket_name = format!("rusoto{}", get_time().sec);
	// let bucket_name = "rusoto1440826511";

	// works with codegen
	// match s3.create_bucket(&bucket_name) {
	// 	Err(why) => {
	// 		println!("got error back...");
	// 		println!("Error: {:?}", why);
	// 	},
	// 	Ok(_) => println!("bucket created."),
	// }

	let mut new_bucket_req = CreateBucketRequest::default();
	new_bucket_req.bucket = bucket_name;
	let mut bucket_config = CreateBucketConfiguration::default();
	bucket_config.location_constraint = BucketLocationConstraint::us_west_2;
	new_bucket_req.create_bucket_configuration = Some(bucket_config);

	match s3.create_bucket_with_request(&new_bucket_req) {
		Err(why) => {
			println!("got error back...");
			println!("Error: {:?}", why);
		},
		Ok(_) => println!("bucket created."),
	}


	// let mut f = File::open("../src/sample-credentials").unwrap();
	// let mut contents : Vec<u8> = Vec::new();
	//
	// match f.read_to_end(&mut contents) {
	// 	Err(why) => panic!(AWSError::new(format!("Error opening file to send to S3: {}", why))),
	// 	Ok(_) => {
	// 		let response = s3.put_object(bucket_name, "sample-credentials", contents).unwrap();
	// 		println!("yay: {:?}", response);
	// 	}
	// }

	println!("Exiting cleanly");

}

fn s3_list_buckets_tests(s3: &mut S3Helper) -> Result<(), AWSError> {
	let response = try!(s3.list_buckets());
	println!("we's good");
	for q in response.buckets {
		println!("Existing bucket: {:?}", q.name);
	}

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
