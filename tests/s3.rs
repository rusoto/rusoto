#![cfg(feature = "s3")]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

#[macro_use]
extern crate rusoto;

use std::io::Read;
use std::fs::File;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Helper, S3Client, ListObjectsRequest, HeadObjectRequest};

#[test]
fn list_buckets_tests() {
    let _ = env_logger::init();
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    let response = s3.list_buckets().unwrap();
    info!("Got list of buckets: {:?}", response);
    for q in response.buckets {
        info!("Existing bucket: {:?}", q.name);
    }
}

#[test]
fn put_object_test() {
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            s3.put_object("rusototester", "no_credentials", &contents).unwrap();
        }
    }
}

#[test]
fn list_objects_test() {
    let _ = env_logger::init();
    let bare_s3 = S3Client::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    let mut list_request = ListObjectsRequest::default(); // need to set bucket
    list_request.bucket = "rusototester".to_string();
    let result = bare_s3.list_objects(&list_request).unwrap();
    println!("result is {:?}", result);
}

// Dependent on the file being there or it'll break.
#[test]
fn get_object_test() {
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    s3.get_object("rusototester", "no_credentials2").unwrap();
}

// Dependent on the file being there or it'll break.
#[test]
fn head_object_test() {
    let _ = env_logger::init();
    let s3 = S3Client::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    let size_req = HeadObjectRequest{
      bucket: "rusototester".to_string(),
      key: "no_credentials2".to_string(),
      ..Default::default()
    };
    println!("{:?}", s3.head_object(&size_req).unwrap());
}

#[test]
fn delete_object_test() {
    let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);
    s3.delete_object("rusototester", "no_credentials").unwrap();
}
