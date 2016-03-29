#![cfg(feature = "s3")]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

#[macro_use]
extern crate rusoto;

use std::io::{Read, Write};
use std::fs::File;

use time::get_time;

use rusoto::{AwsError, ChainProvider, Region};
use rusoto::s3::{
    CannedAcl,
    DeleteObjectOutput,
    GetObjectOutput,
    PutObjectOutput,
    PutObjectRequest,
    S3Helper,
};

#[test]
fn all_s3_tests() {
    let _ = env_logger::init();
    info!("s3 integration tests starting up.");
    let region = Region::UsWest2;
    let mut s3 = S3Helper::new(ChainProvider::new().unwrap(), &region);

    match s3_list_buckets_tests(&mut s3) {
        Ok(_) => { info!("Everything worked for S3 list buckets."); },
        Err(err) => { info!("Got error in s3 list buckets: {}", err); }
    }

    // let mut bucket_name = format!("rusoto{}", get_time().sec);
    // // let bucket_name = "rusoto1440826511";
    //
    // match s3_create_bucket_test(&mut s3, &bucket_name, &region, None) {
    //     Ok(_) => { info!("Everything worked for S3 create bucket."); },
    //     Err(err) => { info!("Got error in s3 create bucket: {}", err); }
    // }
    //
    // match s3_put_object_with_request_specified_test(&mut s3, &bucket_name) {
    //     Ok(_) => info!("Everything worked for S3 put object."),
    //     Err(err) => info!("Got error in s3 put object: {}", err),
    // }
    //
    // match s3_put_object_test(&mut s3, &bucket_name) {
    //     Ok(_) => info!("Everything worked for S3 put object."),
    //     Err(err) => info!("Got error in s3 put object: {}", err),
    // }
    //
    // match s3_get_object_test(&mut s3, &bucket_name) {
    //     Ok(result) => {
    //         info!("Everything worked for S3 get object.");
    //         let mut f = File::create("s3-sample-creds").unwrap();
    //         match f.write(&(result.body)) {
    //             Err(why) => info!("Couldn't create file to save object from S3: {}", why),
    //             Ok(_) => (),
    //         }
    //     }
    //     Err(err) => { info!("Got error in s3 get object: {}", err); }
    // }
    //
    // match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
    //     Ok(_) => {
    //         info!("Everything worked for S3 delete object.");
    //     }
    //     Err(err) => { info!("Got error in s3 delete object: {}", err); }
    // }
    //
    // match s3_put_object_with_reduced_redundancy_test(&mut s3, &bucket_name) {
    //     Ok(_) => {
    //         info!("Everything worked for S3 put object with reduced redundancy.");
    //     }
    //     Err(err) => { info!("Got error in s3 put object with reduced redundancy: {}", err); }
    // }
    //
    // match s3_delete_object_test(&mut s3, &bucket_name, "sample-credentials") {
    //     Ok(_) => {
    //         info!("Everything worked for S3 delete object.");
    //     }
    //     Err(err) => { info!("Got error in s3 delete object: {}", err); }
    // }

    // Set the file in s3_multipart_upload_test and uncomment this code to test multipart upload:
    // info!("Making a large upload...");
    // match s3_multipart_upload_test(&mut s3, &bucket_name) {
    // 	Ok(_) => { info!("Everything worked for S3 multipart upload."); }
    // 	Err(err) => { info!("Got error in s3 multipart upload: {}", err); }
    // }

    // match s3_delete_object_test(&mut s3, &bucket_name, "testfile.zip") {
    // 	Ok(_) => {
    // 		info!("Everything worked for S3 delete object.");
    // 	}
    // 	Err(err) => { info!("Got error in s3 delete object: {}", err); }
    // }

    // match s3_list_multipart_uploads(&mut s3, &bucket_name) {
    //     Err(why) => info!("Error listing multipart uploads: {:?}", why),
    //     Ok(_) => (),
    // }

    // Working example, replace bucket name, file name, uploadID for your multipart upload:
    // match s3_list_multipart_upload_parts(&mut s3, &bucket_name, "testfile.zip", "PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK") {
    // 	Err(why) => info!("Error listing multipart upload parts: {:?}", why),
    // 	Ok(_) => (),
    // }

    // Working example, replace bucket name, file name, uploadID for your multipart upload:
    // match s3_abort_multipart_uploads(&mut s3, &bucket_name, "testfile.zip", "W5J7SeEor1A3vcRMMUhAb.BKrMs68.suzyhErssdb2HFAyDb4z7QhJBMyGkM_GSsoFqKJJLjbHcNSZTHa7MhTFJodewzcswshoDHd7mffXPNUH.xoRWVXbkLjakTETaO") {
    // 	Err(why) => info!("Error aborting multipart uploads: {:?}", why),
    // 	Ok(_) => (),
    // }

    // match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
    //     Ok(_) => { info!("Everything worked for S3 delete bucket."); },
    //     Err(err) => { info!("Got error in s3 delete bucket: {}", err); }
    // }
    //
    // // new bucket for canned acl testing!
    // bucket_name = format!("rusoto{}", get_time().sec);
    //
    // match s3_create_bucket_test(&mut s3, &bucket_name, &region, Some(CannedAcl::AuthenticatedRead)) {
    //     Ok(_) => { info!("Everything worked for S3 create bucket with ACL."); },
    //     Err(err) => { info!("Got error in s3 create bucket: {}", err); }
    // }
    //
    // match s3_delete_bucket_test(&mut s3, &bucket_name, &region) {
    //     Ok(_) => { info!("Everything worked for S3 delete bucket."); },
    //     Err(err) => { info!("Got error in s3 delete bucket: {}", err); }
    // }
}

fn s3_list_multipart_upload_parts(s3: &mut S3Helper, bucket: &str, object: &str, upload_id: &str) -> Result<(), AwsError> {
    match s3.multipart_upload_list_parts(bucket, object, upload_id) {
        Err(why) => info!("Error listing multipart upload parts: {:?}", why),
        Ok(result) => info!("Multipart upload parts: {:?}", result),
    }
    Ok(())
}

fn s3_list_multipart_uploads(s3: &mut S3Helper, bucket: &str) -> Result<(), AwsError> {
    match s3.list_multipart_uploads_for_bucket(bucket) {
        Err(why) => info!("Error listing multipart uploads: {:?}", why),
        Ok(result) => info!("in-progress multipart uploads: {:?}", result),
    }
    Ok(())
}

fn s3_abort_multipart_uploads(s3: &mut S3Helper, bucket: &str, object: &str, upload_id: &str) -> Result<(), AwsError> {
    match s3.abort_multipart_upload(bucket, object, upload_id) {
        Err(why) => info!("Error aborting multipart upload: {:?}", why),
        Ok(result) => info!("aborted multipart upload: {:?}", result),
    }
    Ok(())
}

fn s3_list_buckets_tests(s3: &mut S3Helper) -> Result<(), AwsError> {
    let response = try!(s3.list_buckets());
    info!("Got list of buckets: {:?}", response);
    for q in response.buckets {
        info!("Existing bucket: {:?}", q.name);
    }

    Ok(())
}

fn s3_get_object_test(s3: &mut S3Helper, bucket: &str) -> Result<GetObjectOutput, AwsError> {
    let response = try!(s3.get_object(bucket, "sample-credentials"));
    Ok(response)
}

fn s3_delete_object_test(s3: &mut S3Helper, bucket: &str, object_name: &str) -> Result<DeleteObjectOutput, AwsError> {
    let response = try!(s3.delete_object(bucket, object_name));
    Ok(response)
}

fn s3_put_object_aws_encryption_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    let mut f = File::open("src/sample-credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => return Err(AwsError::new(format!("Error opening file to send to S3: {}", why))),
        Ok(_) => {
            let response = try!(s3.put_object_with_aws_encryption(bucket, "sample-credentials", &contents));
            Ok(response)
        }
    }
}

fn s3_put_object_kms_encryption_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    let mut f = File::open("src/sample-credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => return Err(AwsError::new(format!("Error opening file to send to S3: {}", why))),
        Ok(_) => {
            let response = try!(s3.put_object_with_kms_encryption(bucket, "sample-credentials", &contents, "key-id"));
            Ok(response)
        }
    }
}

fn s3_put_object_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    let mut f = File::open("src/sample-credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => return Err(AwsError::new(format!("Error opening file to send to S3: {}", why))),
        Ok(_) => {
            let response = try!(s3.put_object(bucket, "sample-credentials", &contents));
            Ok(response)
        }
    }
}

fn s3_put_object_with_request_specified_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    let mut f = File::open("src/sample-credentials").unwrap();
    let mut contents : Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => return Err(AwsError::new(format!("Error opening file to send to S3: {}", why))),
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

fn s3_multipart_upload_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    // Set to a > 5 MB file for testing:
    let mut f = File::open("testfile.zip").unwrap();

    let response = try!(s3.put_multipart_object(bucket, "testfile.zip", &mut f));
    Ok(response)
}

fn s3_put_object_with_reduced_redundancy_test(s3: &mut S3Helper, bucket: &str) -> Result<PutObjectOutput, AwsError> {
    let mut f = File::open("src/sample-credentials").unwrap();
    let mut contents = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => return Err(AwsError::new(format!("Error opening file to send to S3: {}", why))),
        Ok(_) => {
            let response = try!(s3.put_object_with_reduced_redundancy(bucket, "sample-credentials", &contents));
            Ok(response)
        }
    }
}

fn s3_create_bucket_test(s3: &mut S3Helper, bucket: &str, region: &Region, canned_acl: Option<CannedAcl>) -> Result<(), AwsError> {
    try!(s3.create_bucket_in_region(bucket, &region, canned_acl));

    Ok(())
}

fn s3_delete_bucket_test(s3: &mut S3Helper, bucket: &str, region: &Region) -> Result<(), AwsError> {
    try!(s3.delete_bucket(bucket, &region));
    Ok(())
}
