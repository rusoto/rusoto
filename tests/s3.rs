#![cfg(feature = "s3")]
extern crate rusoto;
extern crate time;
extern crate hyper;
extern crate env_logger;
extern crate log;

use hyper::Client;
use std::fs::File;
use std::io::Read;
use time::get_time;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Client, HeadObjectRequest, CopyObjectRequest, GetObjectRequest,
                 PutObjectRequest, DeleteObjectRequest, PutBucketCorsRequest, CORSConfiguration,
                 CORSRule, CreateBucketRequest, DeleteBucketRequest };

type TestClient = S3Client<DefaultCredentialsProvider, Client>;

// Rust is in bad need of an integration test harness
// This creates the S3 resources needed for a suite of tests,
// executes those tests, and then destroys the resources
#[test]
fn test_all_the_things() {
    let client = S3Client::new(DefaultCredentialsProvider::new().unwrap(),
                               Region::UsEast1);

    // a random number should probably be appended here too
    let test_bucket = format!("rusoto_test_bucket_{}", get_time().sec);
    let filename = format!("test_file_{}", get_time().sec);

    // get a list of list_buckets
    test_list_buckets(&client);

    // create a bucket for these tests
    test_create_bucket(&client, &test_bucket);

    // modify the bucket's CORS properties
    test_put_bucket_cors(&client, &test_bucket);

    // PUT an object (no_credentials is an arbitrary choice)
    test_put_object(&client, &test_bucket, &filename);

    // HEAD the object that was PUT
    test_head_object(&client, &test_bucket, &filename);

    // GET the object
    test_get_object(&client, &test_bucket, &filename);

    // copy the object to change its settings
    test_copy_object(&client, &test_bucket, &filename);

    // DELETE the object
    test_delete_object(&client, &test_bucket, &filename);

    // delete the test bucket
    test_delete_bucket(&client, &test_bucket);    
}

fn test_create_bucket(client: &TestClient, bucket: &str) {
    let create_bucket_req = CreateBucketRequest {
        bucket: bucket.to_owned(),
        ..Default::default()
    };

    client.create_bucket(&create_bucket_req).unwrap();
}

fn test_delete_bucket(client: &TestClient, bucket: &str) {
    let delete_bucket_req = DeleteBucketRequest {
        bucket: bucket.to_owned(),
        ..Default::default()
    };

    client.delete_bucket(&delete_bucket_req).unwrap();
}

fn test_put_object(client: &TestClient, bucket: &str, filename: &str) {
    let mut f = File::open("tests/sample-data/no_credentials").unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: filename.to_owned(),
                body: Some(contents),
                ..Default::default()
            };
            let _ = client.put_object(&req);
        }
    }
}

fn test_head_object(client: &TestClient, bucket: &str, filename: &str) {
    let head_req = HeadObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    client.head_object(&head_req).unwrap();    
}

fn test_get_object(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    client.get_object(&get_req).unwrap();
}

fn test_copy_object(client: &TestClient, bucket: &str, filename: &str) {
    let req = CopyObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        copy_source: format!("{}/{}", bucket, filename),
        cache_control: Some("max-age=123".to_owned()),
        content_type: Some("application/json".to_owned()),
        metadata_directive: Some("REPLACE".to_owned()),
        ..Default::default()
    };

    client.copy_object(&req).unwrap();
}

fn test_delete_object(client: &TestClient, bucket: &str, filename: &str) {
    let del_req = DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    client.delete_object(&del_req).unwrap();    
}

fn test_list_buckets(client: &TestClient) {
    client.list_buckets().unwrap();
}

fn test_put_bucket_cors(client: &TestClient, bucket: &str) {
    let cors_rules =
        vec![CORSRule {
                 allowed_methods: vec!["PUT".to_owned(), "POST".to_owned(), "DELETE".to_owned()],
                 allowed_origins: vec!["http://www.example.com".to_owned()],
                 allowed_headers: Some(vec!["*".to_owned()]),
                 max_age_seconds: Some(3000),
                 expose_headers: Some(vec!["x-amz-server-side-encryption".to_owned()]),
                 ..Default::default()
             }];

    let cors_configuration = CORSConfiguration { cors_rules: cors_rules };

    let req = PutBucketCorsRequest {
        bucket: bucket.to_owned(),
        cors_configuration: cors_configuration,
        ..Default::default()
    };

    client.put_bucket_cors(&req).unwrap();

}
