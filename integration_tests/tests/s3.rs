#![cfg(feature = "s3")]
extern crate env_logger;
extern crate futures;
extern crate futures_fs;
extern crate log;
extern crate reqwest;
extern crate http;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate time;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::time::Duration;
use time::get_time;

use futures::{Future, Stream};
use futures_fs::FsPool;
use rusoto_core::credential::{AwsCredentials, DefaultCredentialsProvider};
use rusoto_core::{Region, ProvideAwsCredentials, RusotoError};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{
    CORSConfiguration, CORSRule, CompleteMultipartUploadRequest, CompletedMultipartUpload,
    CompletedPart, CopyObjectRequest, CreateBucketRequest, CreateMultipartUploadRequest,
    DeleteBucketRequest, DeleteObjectRequest, GetObjectError, GetObjectRequest,
    HeadObjectRequest, ListObjectsRequest, ListObjectsV2Request, PutBucketCorsRequest,
    PutObjectRequest, S3Client, StreamingBody, UploadPartCopyRequest, UploadPartRequest, S3,
};

type TestClient = S3Client;

// Rust is in bad need of an integration test harness
// This creates the S3 resources needed for a suite of tests,
// executes those tests, and then destroys the resources
#[test]
fn test_all_the_things() {
    let _ = env_logger::try_init();

    let region = if let Ok(endpoint) = env::var("S3_ENDPOINT") {
        let region = Region::Custom {
            name: "us-east-1".to_owned(),
            endpoint: endpoint.to_owned(),
        };
        println!(
            "picked up non-standard endpoint {:?} from S3_ENDPOINT env. variable",
            region
        );
        region
    } else {
        Region::UsEast1
    };

    let client = S3Client::new(region.clone());
    let credentials = DefaultCredentialsProvider::new()
        .unwrap()
        .credentials()
        .wait()
        .unwrap();

    let test_bucket = format!("rusoto-test-bucket-{}", get_time().sec);
    let filename = format!("test_file_{}", get_time().sec);
    let utf8_filename = format!("test[über]file@{}", get_time().sec);
    let binary_filename = format!("test_file_b{}", get_time().sec);
    let multipart_filename = format!("test_multipart_file_{}", get_time().sec);
    let metadata_filename = format!("test_metadata_file_{}", get_time().sec);

    // get a list of list_buckets
    test_list_buckets(&client);

    // create a bucket for these tests
    test_create_bucket(&client, &test_bucket);

    // list items v2
    list_items_in_bucket(&client, &test_bucket);

    // do a multipart upload
    test_multipart_upload(&client, &region, &credentials, &test_bucket, &multipart_filename);

    // modify the bucket's CORS properties
    if cfg!(not(feature = "disable_minio_unsupported")) {
        // Minio support: CORS is not implemented by Minio
        test_put_bucket_cors(&client, &test_bucket);
    }

    // PUT an object via buffer (no_credentials is an arbitrary choice)
    test_put_object_with_filename(
        &client,
        &test_bucket,
        &filename,
        &"tests/sample-data/no_credentials",
    );

    // HEAD the object that was PUT
    test_head_object(&client, &test_bucket, &filename);

    // GET the object
    test_get_object(&client, &test_bucket, &filename);
    test_get_object_range(&client, &test_bucket, &filename);

    // copy the object to change its settings
    test_copy_object(&client, &test_bucket, &filename);

    // UTF8 filenames
    test_put_object_with_filename(
        &client,
        &test_bucket,
        &utf8_filename,
        &"tests/sample-data/no_credentials",
    );

    test_copy_object_utf8(&client, &test_bucket, &utf8_filename);

    test_delete_object(&client, &test_bucket, &utf8_filename);

    // test failure responses
    test_get_object_no_such_object(&client, &test_bucket, &binary_filename);

    // Binary objects:
    test_put_object_with_filename(
        &client,
        &test_bucket,
        &binary_filename,
        &"tests/sample-data/binary-file",
    );
    test_get_object(&client, &test_bucket, &binary_filename);
    test_get_object_blocking_read(&client, &test_bucket, &binary_filename);

    // PUT an object via stream
    let another_filename = format!("streaming{}", filename);
    test_put_object_stream_with_filename(
        &client,
        &test_bucket,
        &another_filename,
        &"tests/sample-data/binary-file",
    );

    // metadata tests
    let mut metadata = HashMap::<String, String>::new();
    metadata.insert(
        "rusoto-metadata-some".to_string(),
        "some-test-value".to_string(),
    );
    metadata.insert("rusoto-metadata-none".to_string(), "".to_string());

    test_put_object_with_metadata(
        &client,
        &test_bucket,
        &metadata_filename,
        &"tests/sample-data/no_credentials",
        &metadata,
    );

    test_head_object_with_metadata(&client, &test_bucket, &metadata_filename, &metadata);
    test_get_object_with_metadata(&client, &test_bucket, &metadata_filename, &metadata);

    // list items with paging using list object API v1
    list_items_in_bucket_paged_v1(&client, &test_bucket);

    // list items with paging using list object API v2
    if cfg!(not(feature = "disable_ceph_unsupported")) {
        // Ceph support: this test depends on the list object v2 API which is not implemented by Ceph
        list_items_in_bucket_paged_v2(&client, &test_bucket);
    }

    test_delete_object(&client, &test_bucket, &metadata_filename);
    test_delete_object(&client, &test_bucket, &binary_filename);
    test_delete_object(&client, &test_bucket, &another_filename);

    // DELETE the object
    test_delete_object(&client, &test_bucket, &filename);

    let filename = format!("{}_for_presigned", filename);
    // PUT an object for presigned url
    test_put_object_with_filename(
        &client,
        &test_bucket,
        &filename,
        &"tests/sample-data/no_credentials",
    );
    // generate a presigned url
    test_get_object_with_presigned_url(&region, &credentials, &test_bucket, &filename);
    test_get_object_with_expired_presigned_url(&region, &credentials, &test_bucket, &filename);
    test_put_object_with_presigned_url(&region, &credentials, &test_bucket, &filename);
    test_delete_object_with_presigned_url(&region, &credentials, &test_bucket, &filename);

    let utf8_filename = format!("{}_for_presigned", utf8_filename);
    // UTF8 filenames for presigned url
    test_put_object_with_filename(
        &client,
        &test_bucket,
        &utf8_filename,
        &"tests/sample-data/no_credentials",
    );
    // generate a presigned url
    test_get_object_with_presigned_url(&region, &credentials, &test_bucket, &utf8_filename);
    test_get_object_with_expired_presigned_url(&region, &credentials, &test_bucket, &utf8_filename);
    test_put_object_with_presigned_url(&region, &credentials, &test_bucket, &utf8_filename);
    test_delete_object_with_presigned_url(&region, &credentials, &test_bucket, &utf8_filename);

    // delete the test bucket
    test_delete_bucket(&client, &test_bucket);
}

fn test_multipart_upload(client: &TestClient, region: &Region, credentials: &AwsCredentials, bucket: &str, filename: &str) {
    let create_multipart_req = CreateMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    // start the multipart upload and note the upload_id generated
    let response = client
        .create_multipart_upload(create_multipart_req)
        .sync()
        .expect("Couldn't create multipart upload");
    println!("{:#?}", response);
    let upload_id = response.upload_id.unwrap();

    // create 2 upload parts
    let create_upload_part = |body: Vec<u8>, part_number: i64| -> UploadPartRequest {
        UploadPartRequest {
            body: Some(body.into()),
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

    // upload the parts and note the etags generated for them
    let mut completed_parts = Vec::new();
    {
        let part_number = part_req1.part_number;
        let response = client
            .upload_part(part_req1)
            .sync()
            .expect("Couldn't upload a file part");
        println!("{:#?}", response);
        completed_parts.push(CompletedPart {
            e_tag: response.e_tag.clone(),
            part_number: Some(part_number),
        });
    }
    // upload the second part via a pre-signed url
    {
        let options = PreSignedRequestOption {
            expires_in: Duration::from_secs(60 * 30),
        };
        let presigned_multipart_put = part_req2.get_presigned_url(region, credentials, &options);
        println!("presigned multipart put: {:#?}", presigned_multipart_put);
        let client = reqwest::Client::new();
        let res = client
            .put(&presigned_multipart_put)
            .body(String::from("foo"))
            .send()
            .expect("Multipart put with presigned url failed");
        assert_eq!(res.status(), http::StatusCode::OK);
        let e_tag = res.headers().get("ETAG").unwrap().to_str().unwrap();
        completed_parts.push(CompletedPart {
            e_tag: Some(e_tag.to_string()),
            part_number: Some(part_req2.part_number),
        });
    }

    // complete the multipart upload with the etags of the parts
    let completed_upload = CompletedMultipartUpload {
        parts: Some(completed_parts),
    };

    let complete_req = CompleteMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        upload_id: upload_id.to_owned(),
        multipart_upload: Some(completed_upload),
        ..Default::default()
    };

    let response = client
        .complete_multipart_upload(complete_req)
        .sync()
        .expect("Couldn't complete multipart upload");
    println!("{:#?}", response);

    // Add copy upload part to this test
    // https://docs.aws.amazon.com/AmazonS3/latest/API/mpUploadUploadPartCopy.html
    let create_multipart_req2 = CreateMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };
    let upload_multi_response = client
        .create_multipart_upload(create_multipart_req2)
        .sync()
        .expect("Couldn't create multipart upload2");
    println!("{:#?}", upload_multi_response);
    let upload_id2 = upload_multi_response.upload_id.unwrap();
    let upload_part_copy_req = UploadPartCopyRequest {
        key: filename.to_owned(),
        bucket: bucket.to_owned(),
        part_number: 1,
        upload_id: upload_id2.clone(),
        copy_source: format!("{}/{}", bucket, filename).to_owned(),
        ..Default::default()
    };
    let copy_response = client
        .upload_part_copy(upload_part_copy_req)
        .sync()
        .expect("Should have had copy part work");
    println!("copy response: {:#?}", copy_response);

    let upload_part_copy_req2 = UploadPartCopyRequest {
        key: filename.to_owned(),
        bucket: bucket.to_owned(),
        part_number: 2,
        upload_id: upload_id2.clone(),
        copy_source: format!("{}/{}", bucket, filename).to_owned(),
        ..Default::default()
    };
    let copy_response2 = client
        .upload_part_copy(upload_part_copy_req2)
        .sync()
        .expect("Should have had copy part work");
    println!("copy response2: {:#?}", copy_response2);

    // complete the upload_part_copy upload:
    let completed_parts_2 = vec![
        CompletedPart {
            e_tag: copy_response.copy_part_result.unwrap().e_tag.clone(),
            part_number: Some(1),
        },
        CompletedPart {
            e_tag: copy_response2.copy_part_result.unwrap().e_tag.clone(),
            part_number: Some(2),
        },
    ];
    let completed_upload2 = CompletedMultipartUpload {
        parts: Some(completed_parts_2),
    };
    let complete_req2 = CompleteMultipartUploadRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        upload_id: upload_id2.to_owned(),
        multipart_upload: Some(completed_upload2),
        ..Default::default()
    };

    let response2 = client
        .complete_multipart_upload(complete_req2)
        .sync()
        .expect("Couldn't complete multipart upload2");
    println!("{:#?}", response2);

    // delete the completed file
    test_delete_object(client, bucket, filename);
}

fn test_create_bucket(client: &TestClient, bucket: &str) {
    let create_bucket_req = CreateBucketRequest {
        bucket: bucket.to_owned(),
        ..Default::default()
    };

    let result = client
        .create_bucket(create_bucket_req)
        .sync()
        .expect("Couldn't create bucket");
    println!("{:#?}", result);
}

fn test_delete_bucket(client: &TestClient, bucket: &str) {
    let delete_bucket_req = DeleteBucketRequest {
        bucket: bucket.to_owned(),
        ..Default::default()
    };

    let result = client.delete_bucket(delete_bucket_req).sync();
    println!("{:#?}", result);
    match result {
        Err(e) => match e {
            RusotoError::Unknown(ref e) => panic!(
                "Couldn't delete bucket because: {}",
                str::from_utf8(&e.body).unwrap()
            ),
            _ => panic!("Error from S3 different than expected"),
        },
        Ok(_) => (),
    }
}

fn test_put_object_with_filename(
    client: &TestClient,
    bucket: &str,
    dest_filename: &str,
    local_filename: &str,
) {
    let mut f = File::open(local_filename).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: dest_filename.to_owned(),
                body: Some(contents.into()),
                ..Default::default()
            };
            let result = client.put_object(req).sync().expect("Couldn't PUT object");
            println!("{:#?}", result);
        }
    }
}

fn test_put_object_stream_with_filename(
    client: &TestClient,
    bucket: &str,
    dest_filename: &str,
    local_filename: &str,
) {
    let meta = ::std::fs::metadata(local_filename).unwrap();
    let fs = FsPool::default();
    let read_stream = fs.read(local_filename.to_owned());
    let req = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: dest_filename.to_owned(),
        content_length: Some(meta.len() as i64),
        body: Some(StreamingBody::new(read_stream.map(|bytes| bytes.to_vec()))),
        ..Default::default()
    };
    let result = client.put_object(req).sync().expect("Couldn't PUT object");
    println!("{:#?}", result);
}

fn test_head_object(client: &TestClient, bucket: &str, filename: &str) {
    let head_req = HeadObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .head_object(head_req)
        .sync()
        .expect("Couldn't HEAD object");
    println!("{:#?}", result);
}

fn test_get_object(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET object");
    println!("get object result: {:#?}", result);

    let stream = result.body.unwrap();
    let body = stream.concat2().wait().unwrap();

    assert!(body.len() > 0);
}

fn test_get_object_blocking_read(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET object");
    println!("get object result: {:#?}", result);

    let mut stream = result.body.unwrap().into_blocking_read();
    let mut body = Vec::new();
    stream.read_to_end(&mut body).unwrap();

    assert!(body.len() > 0);
}

fn test_get_object_no_such_object(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    match client.get_object(get_req).sync() {
        Err(RusotoError::Service(GetObjectError::NoSuchKey(_))) => (),
        r => panic!("unexpected response {:?}", r),
    };
}

fn test_get_object_range(client: &TestClient, bucket: &str, filename: &str) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        range: Some("bytes=0-1".to_owned()),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET object (range)");
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

    let result = client
        .copy_object(req)
        .sync()
        .expect("Couldn't copy object");
    println!("{:#?}", result);
}

fn test_copy_object_utf8(client: &TestClient, bucket: &str, filename: &str) {
    let req = CopyObjectRequest {
        bucket: bucket.to_owned(),
        key: format!("{}", filename.to_owned()),
        copy_source: rusoto_s3::util::encode_key(format!("{}/{}", bucket, filename)),
        cache_control: Some("max-age=123".to_owned()),
        content_type: Some("application/json".to_owned()),
        metadata_directive: Some("REPLACE".to_owned()),
        ..Default::default()
    };

    let result = client
        .copy_object(req)
        .sync()
        .expect("Couldn't copy object (utf8)");
    println!("{:#?}", result);
}

fn test_delete_object(client: &TestClient, bucket: &str, filename: &str) {
    let del_req = DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .delete_object(del_req)
        .sync()
        .expect("Couldn't delete object");
    println!("{:#?}", result);
}

fn test_list_buckets(client: &TestClient) {
    let result = client.list_buckets().sync().expect("Couldn't list buckets");
    println!("\nbuckets available: {:#?}", result);
}

fn list_items_in_bucket(client: &TestClient, bucket: &str) {
    let list_obj_req = ListObjectsV2Request {
        bucket: bucket.to_owned(),
        start_after: Some("foo".to_owned()),
        ..Default::default()
    };
    let result = client
        .list_objects_v2(list_obj_req)
        .sync()
        .expect("Couldn't list items in bucket (v2)");
    println!("Items in bucket: {:#?}", result);
}

fn list_items_in_bucket_paged_v1(client: &TestClient, bucket: &str) {
    let mut list_request = ListObjectsRequest {
        delimiter: Some("/".to_owned()),
        bucket: bucket.to_owned(),
        max_keys: Some(2),
        ..Default::default()
    };

    let response1 = client
        .list_objects(list_request.clone())
        .sync()
        .expect("list objects failed");
    println!("Items in bucket, page 1: {:#?}", response1);
    let contents1 = response1.contents.unwrap();
    assert!(response1.is_truncated.unwrap());
    assert_eq!(contents1.len(), 2);

    list_request.marker = Some(response1.next_marker.unwrap());
    list_request.max_keys = Some(1000);
    let response2 = client
        .list_objects(list_request)
        .sync()
        .expect("list objects failed");
    println!("Items in buckut, page 2: {:#?}", response2);
    let contents2 = response2.contents.unwrap();
    assert!(!response2.is_truncated.unwrap());
    assert!(contents1[1].key.as_ref().unwrap() < contents2[0].key.as_ref().unwrap());
}

// Assuming there's already more than three item in our test bucket:
fn list_items_in_bucket_paged_v2(client: &TestClient, bucket: &str) {
    let mut list_obj_req = ListObjectsV2Request {
        bucket: bucket.to_owned(),
        max_keys: Some(1),
        ..Default::default()
    };
    let result1 = client
        .list_objects_v2(list_obj_req.clone())
        .sync()
        .expect("list objects v2 failed");
    println!("Items in bucket, page 1: {:#?}", result1);
    assert!(result1.next_continuation_token.is_some());

    list_obj_req.continuation_token = result1.next_continuation_token;
    let result2 = client
        .list_objects_v2(list_obj_req)
        .sync()
        .expect("list objects v2 paging failed");
    println!("Items in bucket, page 2: {:#?}", result2);
    // For the second call it the token is in `continuation_token` not `next_continuation_token`
    assert!(result2.continuation_token.is_some());
    assert!(
        result1.contents.unwrap()[0].key.as_ref().unwrap()
            < result2.contents.unwrap()[0].key.as_ref().unwrap()
    );
}

fn test_put_bucket_cors(client: &TestClient, bucket: &str) {
    let cors_rules = vec![CORSRule {
        allowed_methods: vec!["PUT".to_owned(), "POST".to_owned(), "DELETE".to_owned()],
        allowed_origins: vec!["http://www.example.com".to_owned()],
        allowed_headers: Some(vec!["*".to_owned()]),
        max_age_seconds: Some(3000),
        expose_headers: Some(vec!["x-amz-server-side-encryption".to_owned()]),
        ..Default::default()
    }];

    let cors_configuration = CORSConfiguration {
        cors_rules: cors_rules,
    };

    let req = PutBucketCorsRequest {
        bucket: bucket.to_owned(),
        cors_configuration: cors_configuration,
        ..Default::default()
    };

    let result = client
        .put_bucket_cors(req)
        .sync()
        .expect("Couldn't apply bucket CORS");
    println!("{:#?}", result);
}

fn test_put_object_with_metadata(
    client: &TestClient,
    bucket: &str,
    dest_filename: &str,
    local_filename: &str,
    metadata: &HashMap<String, String>,
) {
    let mut f = File::open(local_filename).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: dest_filename.to_owned(),
                body: Some(contents.into()),
                metadata: Some(metadata.clone()),
                ..Default::default()
            };
            let result = client.put_object(req).sync().expect("Couldn't PUT object");
            println!("{:#?}", result);
        }
    }
}

fn test_head_object_with_metadata(
    client: &TestClient,
    bucket: &str,
    filename: &str,
    metadata: &HashMap<String, String>,
) {
    let head_req = HeadObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .head_object(head_req)
        .sync()
        .expect("Couldn't HEAD object");
    println!("{:#?}", result);

    let head_metadata = result.metadata.as_ref().expect("No metadata available");
    assert_eq!(metadata, head_metadata);
}

fn test_get_object_with_metadata(
    client: &TestClient,
    bucket: &str,
    filename: &str,
    metadata: &HashMap<String, String>,
) {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET object");
    println!("get object result: {:#?}", result);

    let head_metadata = result.metadata.as_ref().expect("No metadata available");
    assert_eq!(metadata, head_metadata);
}

fn test_get_object_with_presigned_url(
    region: &Region,
    credentials: &AwsCredentials,
    bucket: &str,
    filename: &str,
) {
    let req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(region, credentials, &Default::default());
    println!("get object presigned url: {:#?}", presigned_url);
    let mut res = reqwest::get(&presigned_url).expect("Couldn't get object via presigned url");
    assert_eq!(res.status(), http::StatusCode::OK);
    let size = res.content_length()
        .unwrap_or(0);
    assert!(size > 0);
    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf).expect("Copying failed");
    assert!(buf.len() > 0);
}

fn test_get_object_with_expired_presigned_url(
    region: &Region,
    credentials: &AwsCredentials,
    bucket: &str,
    filename: &str,
) {
    let req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };
    let opt = PreSignedRequestOption {
        expires_in: ::std::time::Duration::from_secs(1),
    };
    let presigned_url = req.get_presigned_url(region, credentials, &opt);
    ::std::thread::sleep(::std::time::Duration::from_secs(2));
    println!("get object presigned url: {:#?}", presigned_url);
    let res = reqwest::get(&presigned_url).expect("Presigned url failure");
    assert_eq!(res.status(), http::StatusCode::FORBIDDEN);
}

fn test_put_object_with_presigned_url(
    region: &Region,
    credentials: &AwsCredentials,
    bucket: &str,
    filename: &str,
) {
    let req = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(region, credentials, &Default::default());
    println!("put object presigned url: {:#?}", presigned_url);
    let mut map = HashMap::new();
    map.insert("test", "data");
    let client = reqwest::Client::new();
    let res = client
        .put(&presigned_url)
        .json(&map)
        .send()
        .expect("Put obj with presigned url failed");
    assert_eq!(res.status(), http::StatusCode::OK);
}

fn test_delete_object_with_presigned_url(
    region: &Region,
    credentials: &AwsCredentials,
    bucket: &str,
    filename: &str,
) {
    let req = DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };
    let presigned_url = req.get_presigned_url(region, credentials, &Default::default());
    println!("delete object presigned url: {:#?}", presigned_url);
    let client = reqwest::Client::new();
    let res = client
        .delete(&presigned_url)
        .send()
        .expect("Delete of presigned url obj failed");
    assert_eq!(res.status(), http::StatusCode::NO_CONTENT);
}
