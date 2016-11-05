#![cfg(feature = "s3")]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

#[macro_use]
extern crate rusoto;

use std::io::Read;
use std::fs::File;
use std::env::var;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Helper, S3Client, ListObjectsRequest, HeadObjectRequest, CreateBucketRequest};

fn test_bucket() -> String {
    match var("S3_TEST_BUCKET") {
        Ok(val) => val.to_owned(),
        Err(_) => "rusototester".to_owned()
    }
}

fn test_bucket_region() -> Region {
    match var("S3_TEST_BUCKET_REGION") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => "us-west-2".parse().unwrap()
    }
}

#[test]
fn list_buckets_tests() {
    let _ = env_logger::init();
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
    let response = s3.list_buckets().unwrap();
    info!("Got list of buckets: {:?}", response);
    for q in response.buckets {
        info!("Existing bucket: {:?}", q.name);
    }
}

#[test]
fn object_lifecycle_test() {
    // PUT an object
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());

    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            s3.put_object(&test_bucket(), "no_credentials", &contents).unwrap();
        }
    }

    let client = S3Client::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
    // HEAD the object that was PUT
    let size_req = HeadObjectRequest{
      bucket: test_bucket(),
      key: "no_credentials".to_string(),
      ..Default::default()
    };

    println!("{:?}", client.head_object(&size_req).unwrap());

    // GET the object
    s3.get_object(&test_bucket(), "no_credentials").unwrap();

    // DELETE the object
    s3.delete_object(&test_bucket(), "no_credentials").unwrap();    
}

#[test]
fn create_bucket_in_useast1_and_use_immediately() {
    let s3 = S3Client::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
    let create_bucket_request = CreateBucketRequest{
        bucket: "rusoto_foo_bucket".to_string(),
        ..Default::default()
    };
    let bucket_creation_result = s3.create_bucket(&create_bucket_request).unwrap();
    println!("bucket created: {:?}", bucket_creation_result);

    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let s3_helper = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
            let put_response = s3_helper.put_object("rusoto_foo_bucket", "no_credentials", &contents).unwrap();
            println!("put_response is {:?}", put_response);
        }
    }

}

#[test]
fn put_and_fetch_timestamp_named_object_test() {
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            s3.put_object(&test_bucket(), "2016-10-07T23:30:38Z", &contents).unwrap();
        }
    }
    let get_response = s3.get_object(&test_bucket(), "2016-10-07T23:30:38Z").unwrap();
    println!("Got object back: {:?}", get_response);
}

#[test]
fn list_objects_test() {
    let _ = env_logger::init();
    let bare_s3 = S3Client::new(DefaultCredentialsProvider::new().unwrap(), test_bucket_region());
    let mut list_request = ListObjectsRequest::default(); // need to set bucket
    list_request.bucket = test_bucket();
    let result = bare_s3.list_objects(&list_request).unwrap();
    println!("result is {:?}", result);
}

