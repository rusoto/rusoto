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

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

const CHUNK_TO_READ: usize = 5000;
const S3_MINIMUM_PART_SIZE: usize = 5242880;

pub struct S3Helper<'a> {
	client: S3Client<'a>
}

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
	pub fn new(credentials: DefaultAWSCredentialsProviderChain, region:&'a Region) -> S3Helper<'a> {
		S3Helper { client: S3Client::new(credentials, region) }
	}

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

		// println!("Creating bucket");
		let result = self.client.create_bucket(&request);
		// println!("Result is {:?}", result);
		result
	}

	pub fn delete_bucket(&mut self, bucket_name: &str, region: &Region) -> Result<(), AWSError> {
		let mut request = DeleteBucketRequest::default();
		request.bucket = bucket_name.to_string();
		// println!("Deleting bucket");
		let result = self.client.delete_bucket(&request, region);
		// println!("Result is {:?}", result);
		result
	}

	pub fn get_object(&mut self, bucket_name: &str, object_name: &str) ->  Result<GetObjectOutput, AWSError> {
		let mut request = GetObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		let result = self.client.get_object(&request);
		// println!("Result is {:?}", result);
		result
	}

	pub fn put_object(&mut self, bucket_name: &str, object_name: &str, object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {
		self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, false)
	}

	pub fn put_object_with_reduced_redundancy(&mut self, bucket_name: &str, object_name: &str, object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {
		self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, true)
	}

	fn put_object_with_optional_reduced_redundancy(&mut self, bucket_name: &str, object_name: &str,
		object_as_bytes: &Vec<u8>, reduced_redundancy: bool) ->  Result<PutObjectOutput, AWSError> {

		let mut request = PutObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.body = Some(object_as_bytes);
		let result = self.client.put_object(&request, reduced_redundancy);
		result
	}

	// TODO: does this make a copy of the object_as_reader or just transfers ownership to this?
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

		// call complete multipart upload with the list of etags
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

		// TODO: implement md5 hash:
		upload_part_request.content_md5 = None; // string

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

	pub fn list_multipart_uploads_for_bucket(&mut self, bucket_name: &str) -> Result<String, AWSError> {
		let mut request = ListMultipartUploadsRequest::default();
		request.bucket = bucket_name.to_string();

		match self.client.list_multipart_uploads(&request) {
			Err(why) => panic!("Couldn't do list_multipart_uploads: {:?}", why),
			Ok(result) => println!("result is {:?}", result),
		}
		Ok("yay".to_string())
	}

	pub fn delete_object(&mut self, bucket_name: &str, object_name: &str) ->  Result<DeleteObjectOutput, AWSError> {
		let mut request = DeleteObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		let result = self.client.delete_object(&request);
		result
	}
}

pub fn needs_create_bucket_config(region: &Region) -> bool {
	match *region {
		Region::UsEast1 => false,
		_ => true,
	}
}

// This is a bit hacky to get functionality until we figure out an XML writing util.
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
		let file = File::open("tests/sample-data/s3-list-multipart-uploads.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse s3-list-multipart-uploads.xml"),
			Ok(result) => {
				assert_eq!(result.bucket, "rusoto1440826511");
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
