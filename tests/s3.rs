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
use rusoto::s3::{S3, S3Client, HeadObjectRequest, CopyObjectRequest, GetObjectRequest,
                 PutObjectRequest, DeleteObjectRequest, PutBucketCorsRequest, CORSConfiguration,
                 CORSRule, CreateBucketRequest, DeleteBucketRequest, CreateMultipartUploadRequest,
                 UploadPartRequest, CompleteMultipartUploadRequest, CompletedMultipartUpload,
                 CompletedPart, CompletedPartList};
use rusoto::default_tls_client;

type TestClient = S3Client<DefaultCredentialsProvider, Client>;

// Rust is in bad need of an integration test harness
// This creates the S3 resources needed for a suite of tests,
// executes those tests, and then destroys the resources
#[test]
fn test_all_the_things() {
    let _ = env_logger::init();

    let client = S3Client::new(default_tls_client().unwrap(),
                               DefaultCredentialsProvider::new().unwrap(),
                               Region::UsEast1);

    // a random number should probably be appended here too
    let test_bucket = format!("rusoto_test_bucket_{}", get_time().sec);
    let filename = format!("test_file_{}", get_time().sec);
    let binary_filename = format!("test_file_b{}", get_time().sec);
    let multipart_filename = format!("test_multipart_file_{}", get_time().sec);

    // get a list of list_buckets
    test_list_buckets(&client);

    // create a bucket for these tests
    test_create_bucket(&client, &test_bucket);

    // do a multipart upload
    test_multipart_upload(&client, &test_bucket, &multipart_filename);

    // modify the bucket's CORS properties
    test_put_bucket_cors(&client, &test_bucket);

    // PUT an object (no_credentials is an arbitrary choice)
    test_put_object_with_filename(&client,
                                  &test_bucket,
                                  &filename,
                                  &"tests/sample-data/no_credentials");

    // HEAD the object that was PUT
    test_head_object(&client, &test_bucket, &filename);

    // GET the object
    test_get_object(&client, &test_bucket, &filename);
    test_get_object_range(&client, &test_bucket, &filename);
    // copy the object to change its settings
    test_copy_object(&client, &test_bucket, &filename);

    // DELETE the object
    test_delete_object(&client, &test_bucket, &filename);

    // Binary objects:
    test_put_object_with_filename(&client,
                                  &test_bucket,
                                  &binary_filename,
                                  &"tests/sample-data/binary-file");
    test_get_object(&client, &test_bucket, &binary_filename);
    test_delete_object(&client, &test_bucket, &binary_filename);

    // delete the test bucket
    test_delete_bucket(&client, &test_bucket);
}

fn test_multipart_upload(client: &TestClient, bucket: &str, filename: &str) {
    let create_multipart_req = CreateMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    // start the multipart upload and note the upload_id generated
    let response = client.create_multipart_upload(&create_multipart_req).unwrap();
    println!("{:#?}", response);
    let upload_id = response.upload_id.unwrap();

    // create 2 upload parts
    let create_upload_part = |body: Vec<u8>, part_number: i64| -> UploadPartRequest {
        UploadPartRequest {
            body: Some(body),
            bucket: bucket.to_owned(),
            key: filename.to_owned(),
            upload_id: upload_id.to_owned(),
            part_number: part_number,
            ..Default::default()
        }
    };

    // minimum size for a non-final multipart upload part is 5MB
    let part_req1 = create_upload_part(vec!['a' as u8; 1024 * 1024 * 5], 1);
    let part_req2 = create_upload_part("foo".as_bytes().to_vec(), 2);

    // upload 2 parts and note the etags generated for them
    let mut completed_parts = CompletedPartList::new();
    for req in [part_req1, part_req2].into_iter() {
        let response = client.upload_part(&req).unwrap();
        println!("{:#?}", response);
        completed_parts.push(CompletedPart {
            e_tag: response.e_tag.clone(),
            part_number: Some(req.part_number),
        });
    }

    // complete the multipart upload with the etags of the parts
    let completed_upload = CompletedMultipartUpload { parts: Some(completed_parts) };

    let complete_req = CompleteMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        upload_id: upload_id.to_owned(),
        multipart_upload: Some(completed_upload),
        ..Default::default()
    };

    let response = client.complete_multipart_upload(&complete_req).unwrap();
    println!("{:#?}", response);

    // delete the completed file
    test_delete_object(client, bucket, filename);
}

fn test_create_bucket(client: &TestClient, bucket: &str) {
    let create_bucket_req = CreateBucketRequest { bucket: bucket.to_owned(), ..Default::default() };

    let result = client.create_bucket(&create_bucket_req).unwrap();
    println!("{:#?}", result);
}

fn test_delete_bucket(client: &TestClient, bucket: &str) {
    let delete_bucket_req = DeleteBucketRequest { bucket: bucket.to_owned(), ..Default::default() };

    let result = client.delete_bucket(&delete_bucket_req).unwrap();
    println!("{:#?}", result);
}

fn test_put_object_with_filename(client: &TestClient,
                                 bucket: &str,
                                 dest_filename: &str,
                                 local_filename: &str) {
    let mut f = File::open(local_filename).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: dest_filename.to_owned(),
                body: Some(contents),
                ..Default::default()
            };
            let result = client.put_object(&req);
            println!("{:#?}", result);
        }
    }
}

fn test_head_object(client: &TestClient, bucket: &str, filename: &str) {
    let head_req = HeadObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client.head_object(&head_req).unwrap();
    println!("{:#?}", result);
}

fn test_get_object(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client.get_object(&get_req).unwrap();
    println!("get object result: {:#?}", result);
    assert!(result.body.unwrap().len() > 0);
}

fn test_get_object_range(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        range: Some("bytes=0-1".to_owned()),
        ..Default::default()
    };

    let result = client.get_object(&get_req).unwrap();
    println!("\nget object range result: {:#?}", result);
    assert_eq!(result.content_length.unwrap(), 2);
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

    let result = client.copy_object(&req).unwrap();
    println!("{:#?}", result);
}

fn test_delete_object(client: &TestClient, bucket: &str, filename: &str) {
    let del_req = DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client.delete_object(&del_req).unwrap();
    println!("{:#?}", result);
}

fn test_list_buckets(client: &TestClient) {
    let result = client.list_buckets().unwrap();
    println!("{:#?}", result);
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

    let result = client.put_bucket_cors(&req).unwrap();
    println!("{:#?}", result);
}
