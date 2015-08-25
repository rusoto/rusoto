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
use hyper::client::Response;
use std::io::Read;

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

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
	use super::ListBucketsOutputParser;
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
