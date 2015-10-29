//! S3 bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use regions::*;
use std::str::FromStr;
use std::io::BufReader;
use hyper::client::Response;
use std::io::Read;
use std::ascii::AsciiExt;
use openssl::crypto::hash::Type::MD5;
use openssl::crypto::hash::hash;
use serialize::base64::{ToBase64, STANDARD};

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

const CHUNK_TO_READ: usize = 5000;
const S3_MINIMUM_PART_SIZE: usize = 5242880;
// need to sort this out, but having issues going declaring a String here, not a str.
// static S3_REDUCED_REDUNDANCY: &'static str = "REDUCED_REDUNDANCY";

/// Wraps the generated S3 client with a higher level interface
pub struct S3Helper<'a> {
	client: S3Client<'a>
}

/// Canned ACL for S3
#[derive(Debug)]
pub enum CannedAcl {
    Private,
    PublicRead,
	PublicReadWrite,
	AuthenticatedRead,
	BucketOwnerRead,
	BucketOwnerFullControl,
}

impl<'a> S3Helper<'a> {
	/// Creates a new S3 helper
	pub fn new<CP: AWSCredentialsProvider + 'a>(credentials: CP, region:&'a Region) -> S3Helper<'a> {
		S3Helper { client: S3Client::new(credentials, region) }
	}

	/// Lists buckets
	pub fn list_buckets(&mut self) -> Result<ListBucketsOutput, AWSError> {
		self.client.list_buckets()
	}

	/// Creates bucket in default us-east-1/us-standard region.
	pub fn create_bucket(&mut self, bucket_name: &str, canned_acl: Option<CannedAcl>) -> Result<CreateBucketOutput, AWSError> {
		self.create_bucket_in_region(bucket_name, &Region::UsEast1, canned_acl)
	}

	/// Creates bucket in specified region.
	pub fn create_bucket_in_region(&mut self, bucket_name: &str, region: &Region, canned_acl: Option<CannedAcl>) -> Result<CreateBucketOutput, AWSError> {
		let mut request = CreateBucketRequest::default();

		match *region {
			Region::UsEast1 => {
				// us-east-1 is us-standard, don't send a location constraint:
				request.create_bucket_configuration = None;
			}
			_ => {
				let create_config = CreateBucketConfiguration {location_constraint: region_in_aws_format(region)};
				request.create_bucket_configuration = Some(create_config);
			}
		}
		request.bucket = bucket_name.to_string();

		request.acl = canned_acl;

		let result = self.client.create_bucket(&request);
		result
	}

	/// Deletes specified bucket
	pub fn delete_bucket(&mut self, bucket_name: &str, region: &Region) -> Result<(), AWSError> {
		let mut request = DeleteBucketRequest::default();
		request.bucket = bucket_name.to_string();
		let result = self.client.delete_bucket(&request, region);
		result
	}

	/// Download a named object from bucket
	pub fn get_object(&mut self, bucket_name: &str, object_name: &str) ->  Result<GetObjectOutput, AWSError> {
		let mut request = GetObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		let result = self.client.get_object(&request);
		result
	}

	/// Upload an object to specified bucket
	pub fn put_object(&mut self, bucket_name: &str, object_name: &str, object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {
		self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, false)
	}

	/// Helper: uploads object to specified bucket using reduced redudancy storage settings
	pub fn put_object_with_reduced_redundancy(&mut self, bucket_name: &str, object_name: &str, object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {
		self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, true)
	}

	fn put_object_with_optional_reduced_redundancy(&mut self, bucket_name: &str, object_name: &str,
		object_as_bytes: &Vec<u8>, reduced_redundancy: bool) ->  Result<PutObjectOutput, AWSError> {

		let mut request = PutObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.body = Some(object_as_bytes);
		if reduced_redundancy {
			request.storage_class = Some("REDUCED_REDUNDANCY".to_string());
		}
		let result = self.put_object_with_request(&mut request);
		result
	}

	/// Uploads object to specified S3 bucket with server side encryption at rest.
	pub fn put_object_with_aws_encryption(&mut self, bucket_name: &str, object_name: &str,
		object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {

		let mut request = PutObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.body = Some(object_as_bytes);
		request.server_side_encryption = Some("AES256".to_string());

		let result = self.put_object_with_request(&mut request);
		result
	}

	/// Uploads object to specified S3 bucket using AWS KMS for key management of encryption at rest.
	pub fn put_object_with_kms_encryption(&mut self, bucket_name: &str, object_name: &str,
		object_as_bytes: &Vec<u8>, key_id: &str) ->  Result<PutObjectOutput, AWSError> {

		let mut request = PutObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.body = Some(object_as_bytes);
		request.server_side_encryption = Some("aws:kms".to_string());
		request.ssekms_key_id = Some(key_id.to_string());

		let result = self.put_object_with_request(&mut request);
		result
	}

	/// Uploads object: lets sender specify options.
	/// The most generic of put_object: caller specifies the whole request.
	pub fn put_object_with_request(&mut self, request: &mut PutObjectRequest) -> Result<PutObjectOutput, AWSError> {
		// This may be where we do some basic sanity checking: ensure we have:
		// bucket name, region, object id, payload.

		// content_md5 hashing for everyone!
		let hash = hash(MD5, request.body.unwrap()).to_base64(STANDARD);

		self.client.put_object(&request)
	}

	// TODO: does this make a copy of the object_as_reader or just transfers ownership to this?
	/// Uploads a multi-part object to specified bucket.  Allows for large file uploads.
	pub fn put_multipart_object<T: Read>(&mut self, bucket_name: &str, object_name: &str,
		object_as_reader: &mut T) -> Result<PutObjectOutput, AWSError> { // TODO: return type correct?

		// TODO: make helper function for object PUT requests that handles encryption, reduced redudancy, etc...

		let mut multipart_upload_request = CreateMultipartUploadRequest::default();
		multipart_upload_request.key = object_name.to_string();
		multipart_upload_request.bucket = bucket_name.to_string();

		// compiler warns about this line, it's not seeing its use later in this function:
		let mut upload_id : String;

		match self.client.create_multipart_upload(&multipart_upload_request) {
			Err(why) => {
				println!("Couldn't create multipart upload request: {:?}", why);
				return Err(AWSError::new("oops"));
			}
			Ok(response) => upload_id = response.upload_id.to_string(),
		}

		let mut buffered_reader = BufReader::new(object_as_reader);
		let mut parts_list : Vec<String>;

		match self.upload_chunks(&mut buffered_reader, &bucket_name, &upload_id, &object_name) {
			Err(why) => return Err(AWSError::new("oops in upload_chunks")),
			Ok(parts) => parts_list = parts,
		}

		let item_list : Vec<u8>;
		match multipart_upload_finish_xml(&parts_list) {
			Err(why) => return Err(AWSError::new("oops in multipart_upload_finish_xml")),
			Ok(parts_in_xml) => item_list = parts_in_xml,
		}
		let mut complete_upload = CompleteMultipartUploadRequest::default();

		complete_upload.key = object_name.to_string();
		complete_upload.bucket = bucket_name.to_string();
		complete_upload.upload_id = upload_id.to_string();

		complete_upload.multipart_upload = Some(&item_list);

		match self.client.complete_multipart_upload(&complete_upload) {
			Err(why) => {
				println!("Couldn't mark multipart upload as complete: {:?}", why);
				return Err(AWSError::new("oops in complete multipart upload"));
			},
			Ok(_) => (), // TODO: return object output
		}

		Ok(PutObjectOutput::default())
	}

	fn upload_chunks<T: Read>(&mut self, buffered_reader: &mut BufReader<T>,
			bucket_name: &str, upload_id: &str, object_name: &str) -> Result<Vec<String>, AWSError> {

		let mut s3_chunk : Vec<u8> = Vec::with_capacity(S3_MINIMUM_PART_SIZE + CHUNK_TO_READ);
		let mut buffer = [0u8; CHUNK_TO_READ];

		let mut total_bytes_for_debug = 0;

		let mut parts : Vec<String> = Vec::new();
		let mut part_number = 1;
		let mut more_chunks_to_go = true;
		while more_chunks_to_go {
			match buffered_reader.read(&mut buffer) {
				Err(why) => println!("Got Error in reading from buffer: {:?}", why),
				Ok(bytes_read) => {
					total_bytes_for_debug += bytes_read;
					if bytes_read == 0 {
						more_chunks_to_go = false;
					}
					let mut bytes_copied = 0;
					// This still iterates over the buffer even if we're done with it.  Need to fix.
					for i in buffer.iter() {
						if bytes_copied < bytes_read
						{
							s3_chunk.push(i.clone()); // I think this copying is unavoidable
							bytes_copied += 1;
						}
					}

					if s3_chunk.len() >= S3_MINIMUM_PART_SIZE || !more_chunks_to_go {
						match self.upload_a_part(&s3_chunk, &part_number, &bucket_name, &upload_id, &object_name) {
							Err(why) => {
								println!("Got error uploading a part: {:?}", why);
								return Err(AWSError::new("oops in upload_chunks"));
							}
							Ok(response) => {
								parts.push(response);
							}
						}
						s3_chunk.clear();
						part_number += 1;
					}
				}
			}
		}
		Ok(parts)
	}

	fn upload_a_part(&mut self, buffer: &Vec<u8>, part_number: &i32,
			bucket_name: &str, upload_id: &str, object_name: &str) -> Result<String, AWSError> {

		let mut upload_part_request = UploadPartRequest::default();
		upload_part_request.body = Some(&buffer);

		let hash = hash(MD5, upload_part_request.body.unwrap()).to_base64(STANDARD);
		upload_part_request.content_md5 = Some(hash);

		upload_part_request.bucket = bucket_name.to_string();
		upload_part_request.upload_id = upload_id.to_string();
		upload_part_request.part_number = part_number.clone();
		upload_part_request.key = object_name.to_string();

		match self.client.upload_part(&upload_part_request) {
			Err(why) => {
				println!("Error uploading part: {:?}", why);
				return Err(AWSError::new("oops in upload_a_part"));
			},
			Ok(response) => {
				return Ok(response.to_string());
			}
		}
	}

	/// Lists multipart uploads not yet completed for specified bucket
	pub fn list_multipart_uploads_for_bucket(&mut self, bucket_name: &str) -> Result<ListMultipartUploadsOutput, AWSError> {
		let mut request = ListMultipartUploadsRequest::default();
		request.bucket = bucket_name.to_string();

		match self.client.list_multipart_uploads(&request) {
			Err(why) => Err(AWSError::new(format!("Couldn't do list_multipart_uploads: {:?}", why))),
			Ok(result) => Ok(result),
		}
	}

	/// Deletes specified object from specified bucket.
	pub fn delete_object(&mut self, bucket_name: &str, object_name: &str) ->  Result<DeleteObjectOutput, AWSError> {
		let mut request = DeleteObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		self.client.delete_object(&request)
	}

	/// Abort multipart upload.
	pub fn abort_multipart_upload(&mut self, bucket_name: &str, object_name: &str, upload_id: &str) ->  Result<AbortMultipartUploadOutput, AWSError> {
		let mut request = AbortMultipartUploadRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.upload_id = upload_id.to_string();
		self.client.abort_multipart_upload(&request)
	}

	/// List parts from a multiupload request.
	pub fn multipart_upload_list_parts(&mut self, bucket_name: &str, object_name: &str, upload_id: &str) ->  Result<ListPartsOutput, AWSError> {
		let mut request = ListPartsRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.upload_id = upload_id.to_string();
		self.client.list_parts(&request)
	}
}

/// Helper function to determine if a create config is needed.
pub fn needs_create_bucket_config(region: &Region) -> bool {
	match *region {
		Region::UsEast1 => false,
		_ => true,
	}
}

// This is a bit hacky to get functionality until we figure out an XML writing util.
/// Manually writes out bucket configuration (location constraint) in XML.
pub fn create_bucket_config_xml(region: &Region) -> Vec<u8> {
	match *region {
		Region::UsEast1 => {
			Vec::new() // shouldn't actually execute this: panic! or unreachable! this?
		}
		_ => {
			let xml = format!("<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
		<LocationConstraint>{}</LocationConstraint>
		</CreateBucketConfiguration >", region_in_aws_format(region));
			xml.into_bytes()
		}
	}
}

/// Writes out XML with all the parts in it for S3 to complete.
pub fn multipart_upload_finish_xml(parts: &Vec<String>) -> Result<Vec<u8>, AWSError> {
	if parts.len() < 1 {
		return Err(AWSError::new("Can't finish upload on 0 parts."));
	}
	let mut response = String::from("<CompleteMultipartUpload>");

	let mut part_number = 1;
	for etag in parts {
		response = response + &format!("<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>", part_number, etag);
		part_number += 1;
	}

	response = response + "</CompleteMultipartUpload>";

	Ok(response.into_bytes())
}

/// Maps canned acl to AWS format.  EG public-read.
pub fn canned_acl_in_aws_format(canned_acl: &CannedAcl) -> String {
	match *canned_acl {
		CannedAcl::Private => "private".to_string(),
	    CannedAcl::PublicRead => "public-read".to_string(),
		CannedAcl::PublicReadWrite => "public-read-write".to_string(),
		CannedAcl::AuthenticatedRead => "authenticated-read".to_string(),
		CannedAcl::BucketOwnerRead => "bucket-owner-read".to_string(),
		CannedAcl::BucketOwnerFullControl => "bucket-owner-full-control".to_string(),
	}
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

	#[test]
	fn list_buckets_happy_path() {
		let file = File::open("tests/sample-data/s3_get_buckets.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse list_buckets"),
			Ok(_) => return,
		}
	}

	#[test]
	fn initiate_multipart_upload_happy_path() {
		let file = File::open("tests/sample-data/s3_initiate_multipart_upload.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = CreateMultipartUploadOutputParser::parse_xml("InitiateMultipartUploadResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse initiate_multipart_upload"),
			Ok(result) => {
				if result.bucket != "example-bucket" {
					panic!("Bucket name not right.");
				}
				if result.key != "example-object" {
					panic!("Object key not right.");
				}
				if result.upload_id != "VXBsb2FkIElEIGZvciA2aWWpbmcncyBteS1tb3ZpZS5tMnRzIHVwbG9hZA" {
					panic!("Upload ID not right.");
				}
			},
		}
	}

	#[test]
	fn complete_multipart_upload_happy_path() {
		let file = File::open("tests/sample-data/s3_complete_multipart_upload.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = CompleteMultipartUploadOutputParser::parse_xml("CompleteMultipartUploadResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse s3_complete_multipart_upload"),
			Ok(result) => {
				assert_eq!(result.bucket, "testbucket2");
				assert_eq!(result.key, "foo.zip");
				assert_eq!(result.e_tag, "\"525a81fcbc4181997bd96e4096fa7304-1\"");
			}
		}
	}

	#[test]
	fn multipart_upload_xml_looks_right() {
		let mut parts : Vec<String> = Vec::new();
		parts.push("etag1".to_string());
		parts.push("etag2".to_string());
		let response = multipart_upload_finish_xml(&parts).unwrap();

		let expected_string = "<CompleteMultipartUpload><Part><PartNumber>1</PartNumber><ETag>etag1</ETag></Part><Part><PartNumber>2</PartNumber><ETag>etag2</ETag></Part></CompleteMultipartUpload>";

		assert_eq!(expected_string,  str::from_utf8(&response).unwrap());
	}

	#[test]
	fn list_multipart_upload_happy_path() {
		let file = File::open("tests/sample-data/s3_list_multipart_uploads.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse s3_list_multipart_uploads.xml"),
			Ok(result) => {
				assert_eq!(result.bucket, "rusoto1440826511");
				let ref an_upload = result.uploads[0];
				assert_eq!(an_upload.upload_id, "eUeGzA6xR2jAH7KUhTSwrrNVfu8XPIYdoWpa7meOiceoGQLQhtKfPg_APCnuVRsyWd7bx8SS5jNssgdtTU5tTziGOz.j1URgseoqpdHqnyZRikJHTLd6iXF.GjKBEhky");
				assert_eq!(an_upload.key, "join.me.zip");

				let test_initiator = Initiator {id: "arn:aws:iam::347452556412:user/matthew".to_string(),
					display_name: "matthew".to_string() };

				assert_eq!(an_upload.initiator.id, test_initiator.id);
				assert_eq!(an_upload.initiator.display_name, test_initiator.display_name);

				assert_eq!(an_upload.initiated, "2015-09-01T19:22:56.000Z");

				let test_owner = Owner { id: "b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83".to_string(),
					display_name: "matthew".to_string() };

				assert_eq!(an_upload.owner.id, test_owner.id);
				assert_eq!(an_upload.owner.display_name, test_owner.display_name);

				assert_eq!(an_upload.storage_class, "STANDARD");
			}
		}
	}

	#[test]
	fn list_multipart_upload_parts_happy_path() {
		let file = File::open("tests/sample-data/s3_multipart_uploads_with_parts.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListPartsOutputParser::parse_xml("ListPartsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse s3_multipart_uploads_with_parts.xml"),
			Ok(result) => {
				assert_eq!(result.bucket, "rusoto1440826511");
				assert_eq!(result.upload_id, "PeePB_uORK5f2AURP_SWcQ4NO1P1oqnGNNNFK3nhFfzMeksdvG7x7nFfH1qk7a3HSossNYB7t8QhcN1Fg6ax7AXbwvAKIZ9DilB4tUcpM7qyUEgkszN4iDmMvSaImGFK");
				assert_eq!(result.key, "testfile.zip");

				let test_initiator = Initiator {id: "arn:aws:iam::347452556412:user/matthew".to_string(),
					display_name: "matthew".to_string() };

				assert_eq!(result.initiator.id, test_initiator.id);
				assert_eq!(result.initiator.display_name, test_initiator.display_name);

				let test_owner = Owner { id: "b84c6b0c308085829b6562b586f6664fc00faab6cfd441e90ad418ea916eed83".to_string(),
					display_name: "matthew".to_string() };

				assert_eq!(result.owner.id, test_owner.id);
				assert_eq!(result.owner.display_name, test_owner.display_name);

				assert_eq!(result.storage_class, "STANDARD");

				assert_eq!(result.parts.len(), 2);
				assert_eq!(result.parts[0].part_number, 1);
				assert_eq!(result.parts[0].e_tag, "\"ddcaa99616d7cd06d0a5abfef6ccebbb\"");
				assert_eq!(result.parts[0].size, 5242880);
				assert_eq!(result.parts[0].last_modified, "2015-09-08T21:02:04.000Z");

			}
		}
	}

	#[test]
	fn list_multipart_upload_no_uploads() {
		let file = File::open("tests/sample-data/s3_list_multipart_uploads_no_multipart_uploads.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse s3_list_multipart_uploads_no_multipart_uploads.xml"),
			Ok(result) => {
				assert_eq!(result.bucket, "rusoto1440826568");
				assert_eq!(result.uploads.len(), 0);
			}
		}
	}

	#[test]
	fn create_bucket_constrained_to_region() {
		let region = Region::UsWest2;
		match create_bucket_config_xml(&region).len() {
			0 => panic!("us-west-2 should have bucket constraint."),
			_ => return,
		}
	}

	#[test]
	fn create_bucket_us_east_1_no_constraints() {
		let region = Region::UsEast1;
		match create_bucket_config_xml(&region).len() {
			0 => return,
			_ => panic!("us-east-1 should not have bucket constraint."),
		}
	}

	#[test]
	fn create_bucket_constraint_needed() {
		let region = Region::UsWest2;
		match needs_create_bucket_config(&region) {
			false => panic!("us-west-2 should have bucket constraint."),
			true => return,
		}
	}

	#[test]
	fn create_bucket_no_constraint_needed() {
		let region = Region::UsEast1;
		match needs_create_bucket_config(&region) {
			true => panic!("us-east-1 should not have bucket constraint."),
			false => return,
		}
	}
}
