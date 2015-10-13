//! S3 bindings for Rust
//!
//! Not all functions are yet implemented.  Check [S3Helper](http://dualspark.github.io/rusoto/rusoto/s3/struct.S3Helper.html)
//! for implemented functions and convenience functions.
//!

// TODO: don't allow non snake and non camel, fix in S3 codegen script:
#![allow(unused_variables, unused_mut, non_snake_case, non_camel_case_types)]
use credentials::*;
use xml::*;
use signature::*;
use error::*;
use xmlutil::*;
use regions::*;
use std::str::{FromStr, from_utf8};
use hyper::header::Headers;
use std::io::Read;
use std::ascii::AsciiExt;
use openssl::crypto::hash::Type::MD5;
use openssl::crypto::hash::hash;
use rustc_serialize::base64::{ToBase64, STANDARD};
use std::fmt;
use std::string::String::*;
// use std::string::String::to_bytes;

// include the code generated from the S3 botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

const CHUNK_TO_READ: usize = 5000;
const S3_MINIMUM_PART_SIZE: usize = 5242880;
// need to sort this out, but having issues going declaring a String here, not a str.
// static S3_REDUCED_REDUNDANCY: &'static str = "REDUCED_REDUNDANCY";

/// Wraps the generated S3 client with a higher level interface
pub struct S3Helper<'a> {
	client: S3Client<'a>
}

impl<'a> S3Helper<'a> {

	/// Creates a new S3 helper
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> S3Helper<'a> {
		S3Helper { client: S3Client::new(credentials, region) }
	}

	/// Lists buckets
	pub fn list_buckets(&mut self) -> Result<ListBucketsOutput, AWSError> {
		self.client.list_buckets()
	}

	/// Creates bucket
	pub fn create_bucket(&mut self, bucket_name: &str) -> Result<CreateBucketOutput, AWSError> {
		let mut request = CreateBucketRequest::default();
		request.bucket = bucket_name.to_string();
		self.client.create_bucket(&request)
	}

	pub fn create_bucket_with_request(&mut self, request: &CreateBucketRequest) -> Result<CreateBucketOutput, AWSError> {
		self.client.create_bucket(request)
	}

	/// Lists buckets
	pub fn put_object(&mut self, bucket_name: &str, object_name: &str, object_as_bytes: Vec<u8>) -> Result<PutObjectOutput, AWSError> {
		let mut request = PutObjectRequest::default();
		request.bucket = bucket_name.to_string();
		request.key = object_name.to_string();
		request.body = Some(object_as_bytes);

		println!("Sending put obj request: {:?}", request);

		self.client.put_object(&request)
	}

	// Create bucket in region

	// create bucket with canned acl

	// delete bucket

	// get object from bucket

	// put multipart upload

	// list multipart uploads

	// abort multipart upload

	// list multipart upload parts

	// delete object

}


#[cfg(test)]
mod tests {
	use xml::reader::*;
	use std::io::BufReader;
	use std::fs::File;
	use std::str;
	use super::ListBucketsOutputParser;
	use super::CreateMultipartUploadOutputParser;
	use super::CompleteMultipartUploadOutputParser;
	use super::ListMultipartUploadsOutputParser;
	use super::ListPartsOutputParser;
	use super::*;
	use xmlutil::*;
	use regions::*;

	// #[test]
	// fn list_buckets_happy_path() {
	// 	let file = File::open("tests/sample-data/s3_get_buckets.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse list_buckets"),
	// 		Ok(_) => return,
	// 	}
	// }
	//
	// #[test]
	// fn initiate_multipart_upload_happy_path() {
	// 	let file = File::open("tests/sample-data/s3_initiate_multipart_upload.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = CreateMultipartUploadOutputParser::parse_xml("InitiateMultipartUploadResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse initiate_multipart_upload"),
	// 		Ok(result) => {
	// 			if result.bucket != "example-bucket" {
	// 				panic!("Bucket name not right.");
	// 			}
	// 			if result.key != "example-object" {
	// 				panic!("Object key not right.");
	// 			}
	// 			if result.upload_id != "VXBsb2FkIElEIGZvciA2aWWpbmcncyBteS1tb3ZpZS5tMnRzIHVwbG9hZA" {
	// 				panic!("Upload ID not right.");
	// 			}
	// 		},
	// 	}
	// }
	//
	// #[test]
	// fn complete_multipart_upload_happy_path() {
	// 	let file = File::open("tests/sample-data/s3_complete_multipart_upload.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = CompleteMultipartUploadOutputParser::parse_xml("CompleteMultipartUploadResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse s3_complete_multipart_upload"),
	// 		Ok(result) => {
	// 			assert_eq!(result.bucket, "testbucket2");
	// 			assert_eq!(result.key, "foo.zip");
	// 			assert_eq!(result.e_tag, "\"525a81fcbc4181997bd96e4096fa7304-1\"");
	// 		}
	// 	}
	// }

	// #[test]
	// fn multipart_upload_xml_looks_right() {
	// 	let mut parts : Vec<String> = Vec::new();
	// 	parts.push("etag1".to_string());
	// 	parts.push("etag2".to_string());
	// 	let response = multipart_upload_finish_xml(&parts).unwrap();
	//
	// 	let expected_string = "<CompleteMultipartUpload><Part><PartNumber>1</PartNumber><ETag>etag1</ETag></Part><Part><PartNumber>2</PartNumber><ETag>etag2</ETag></Part></CompleteMultipartUpload>";
	//
	// 	assert_eq!(expected_string,  str::from_utf8(&response).unwrap());
	// }

	// #[test]
	// fn list_multipart_upload_happy_path() {
	// 	let file = File::open("tests/sample-data/s3_list_multipart_uploads.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse s3_list_multipart_uploads.xml"),
	// 		Ok(result) => {
	// 			assert_eq!(result.bucket, "rusoto1440826511");
	// 			let ref an_upload = result.uploads[0];
	// 			assert_eq!(an_upload.upload_id, "eUeGzA6xR2jAH7KUhTSwrrNVfu8XPIYdoWpa7meOiceoGQLQhtKfPg_APCnuVRsyWd7bx8SS5jNssgdtTU5tTziGOz.j1URgseoqpdHqnyZRikJHTLd6iXF.GjKBEhky");
	// 			assert_eq!(an_upload.key, "join.me.zip");
	//
	// 			let test_initiator = Initiator {id: "arn:aws:iam::347452556412:user/matthew".to_string(),
	// 				display_name: "matthew".to_string() };
	//
	// 			assert_eq!(an_upload.initiator.id, test_initiator.id);
	// 			assert_eq!(an_upload.initiator.display_name, test_initiator.display_name);
	//
	// 			assert_eq!(an_upload.initiated, "2015-09-01T19:22:56.000Z");
	//
	// 			let test_owner = Owner { id: "b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83".to_string(),
	// 				display_name: "matthew".to_string() };
	//
	// 			assert_eq!(an_upload.owner.id, test_owner.id);
	// 			assert_eq!(an_upload.owner.display_name, test_owner.display_name);
	//
	// 			assert_eq!(an_upload.storage_class, "STANDARD");
	// 		}
	// 	}
	// }
	//
	// #[test]
	// fn list_multipart_upload_parts_happy_path() {
	// 	let file = File::open("tests/sample-data/s3_multipart_uploads_with_parts.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = ListPartsOutputParser::parse_xml("ListPartsResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse s3_multipart_uploads_with_parts.xml"),
	// 		Ok(result) => {
	// 			assert_eq!(result.bucket, "rusoto1440826511");
	// 			assert_eq!(result.upload_id, "PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK");
	// 			assert_eq!(result.key, "testfile.zip");
	//
	// 			let test_initiator = Initiator {id: "arn:aws:iam::347452556412:user/matthew".to_string(),
	// 				display_name: "matthew".to_string() };
	//
	// 			assert_eq!(result.initiator.id, test_initiator.id);
	// 			assert_eq!(result.initiator.display_name, test_initiator.display_name);
	//
	// 			let test_owner = Owner { id: "b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83".to_string(),
	// 				display_name: "matthew".to_string() };
	//
	// 			assert_eq!(result.owner.id, test_owner.id);
	// 			assert_eq!(result.owner.display_name, test_owner.display_name);
	//
	// 			assert_eq!(result.storage_class, "STANDARD");
	//
	// 			assert_eq!(result.parts.len(), 2);
	// 			assert_eq!(result.parts[0].part_number, 1);
	// 			assert_eq!(result.parts[0].e_tag, "\"ddcaa99616d7cd06d0a5abfef6ccebbb\"");
	// 			assert_eq!(result.parts[0].size, 5242880);
	// 			assert_eq!(result.parts[0].last_modified, "2015-09-08T21:02:04.000Z");
	//
	// 		}
	// 	}
	// }
	//
	// #[test]
	// fn list_multipart_upload_no_uploads() {
	// 	let file = File::open("tests/sample-data/s3_list_multipart_uploads_no_multipart_uploads.xml").unwrap();
	//     let file = BufReader::new(file);
	//     let mut my_parser  = EventReader::new(file);
	//     let my_stack = my_parser.events().peekable();
	//     let mut reader = XmlResponseFromFile::new(my_stack);
	// 	reader.next(); // xml start node
	// 	let result = ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut reader);
	//
	// 	match result {
	// 		Err(_) => panic!("Couldn't parse s3_list_multipart_uploads_no_multipart_uploads.xml"),
	// 		Ok(result) => {
	// 			assert_eq!(result.bucket, "rusoto1440826568");
	// 			assert_eq!(result.uploads.len(), 0);
	// 		}
	// 	}
	// }

	// #[test]
	// fn create_bucket_constrained_to_region() {
	// 	let region = Region::UsWest2;
	// 	match create_bucket_config_xml(&region).len() {
	// 		0 => panic!("us-west-2 should have bucket constraint."),
	// 		_ => return,
	// 	}
	// }

	// #[test]
	// fn create_bucket_us_east_1_no_constraints() {
	// 	let region = Region::UsEast1;
	// 	match create_bucket_config_xml(&region).len() {
	// 		0 => return,
	// 		_ => panic!("us-east-1 should not have bucket constraint."),
	// 	}
	// }

	// #[test]
	// fn create_bucket_constraint_needed() {
	// 	let region = Region::UsWest2;
	// 	match needs_create_bucket_config(&region) {
	// 		false => panic!("us-west-2 should have bucket constraint."),
	// 		true => return,
	// 	}
	// }
	//
	// #[test]
	// fn create_bucket_no_constraint_needed() {
	// 	let region = Region::UsEast1;
	// 	match needs_create_bucket_config(&region) {
	// 		true => panic!("us-east-1 should not have bucket constraint."),
	// 		false => return,
	// 	}
	// }
}
