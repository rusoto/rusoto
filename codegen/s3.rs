use std::collections::HashMap;
#[derive(Debug,PartialEq)]
pub enum ArgumentLocation {
	Header,
	Body,
	Headers,
	Querystring,
	Uri,
}
pub fn get_value_for_header(header_name: &str, headers: &Headers) -> Result<String, XmlParseError> {
	for header in headers.iter() {
		if header.name() == header_name {
			return Ok(header.value_string());
		}
	}
	Ok(String::new())
	// should be:
	// Err(AWSError::new(format!("Couldn't find field {} in headers", header_name)))
}
#[derive(Debug, Default)]
pub struct LifecycleExpiration {
	/// Indicates at what date the object is to be moved or deleted. Should be in GMT
	/// ISO 8601 Format.
	pub date: Date,
	/// Indicates the lifetime, in days, of the objects that are subject to the rule.
	/// The value must be a non-zero positive integer.
	pub days: Days,
}


/// Parse LifecycleExpiration from response
struct LifecycleExpirationParser;
impl LifecycleExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LifecycleExpiration, XmlParseError> {
		println!("in LifecycleExpirationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = LifecycleExpiration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Date and child shape is Date
			if current_name == "Date"{
				obj.date = try!(DateParser::parse_response(Some(&"Date".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Days and child shape is Days
			if current_name == "Days"{
				obj.days = try!(DaysParser::parse_response(Some(&"Days".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write LifecycleExpiration contents to a SignedRequest
struct LifecycleExpirationWriter;
impl LifecycleExpirationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LifecycleExpiration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DateWriter::write_params(request, &obj.date, Some(&ArgumentLocation::Body), &"Date".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = DaysWriter::write_params(request, &obj.days, Some(&ArgumentLocation::Body), &"Days".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketNotificationRequest {
	pub notification_configuration: NotificationConfigurationDeprecated,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketNotificationRequest contents to a SignedRequest
struct PutBucketNotificationRequestWriter;
impl PutBucketNotificationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketNotificationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = NotificationConfigurationDeprecatedWriter::write_params(request, &obj.notification_configuration, Some(&ArgumentLocation::Body), &"NotificationConfiguration".to_string());
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type S3ClientErrors = Vec<S3ClientError>;

/// Parse S3ClientErrors from response
struct S3ClientErrorsParser;
impl S3ClientErrorsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<S3ClientErrors, XmlParseError> {
		println!("in S3ClientErrorsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Error" {
//we need to iterate over members of Error
// obj.push for Error
			obj.push(try!(S3ClientErrorParser::parse_response(Some(&"Error"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct PutBucketVersioningRequest {
	/// The concatenation of the authentication device's serial number, a space, and
	/// the value that is displayed on your authentication device.
	pub mfa: Option<MFA>,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	pub versioning_configuration: VersioningConfiguration,
}

/// Write PutBucketVersioningRequest contents to a SignedRequest
struct PutBucketVersioningRequestWriter;
impl PutBucketVersioningRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketVersioningRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for mfa
		if let Some(ref obj) = obj.mfa {
			MFAWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-mfa".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = VersioningConfigurationWriter::write_params(request, &obj.versioning_configuration, Some(&ArgumentLocation::Body), &"VersioningConfiguration".to_string());
		body
	}
}
pub type CopySourceVersionId = String;

/// Parse CopySourceVersionId from response
struct CopySourceVersionIdParser;
impl CopySourceVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceVersionId, XmlParseError> {
		println!("in CopySourceVersionIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceVersionId contents to a SignedRequest
struct CopySourceVersionIdWriter;
impl CopySourceVersionIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceVersionId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceVersionId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// Container for specifying the configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Notification Service (Amazon SNS) topic.
#[derive(Debug, Default)]
pub struct TopicConfiguration {
	pub filter: Option<NotificationConfigurationFilter>,
	pub id: Option<NotificationId>,
	/// Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects
	/// events of specified type.
	pub topic_arn: TopicArn,
	pub events: EventList,
}


/// Parse TopicConfiguration from response
struct TopicConfigurationParser;
impl TopicConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TopicConfiguration, XmlParseError> {
		println!("in TopicConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = TopicConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Filter and child shape is NotificationConfigurationFilter
			if current_name == "Filter"{
				obj.filter = Some(try!(NotificationConfigurationFilterParser::parse_response(Some(&"Filter".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = Some(try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of TopicArn and child shape is TopicArn
			if current_name == "TopicArn"{
				obj.topic_arn = try!(TopicArnParser::parse_response(Some(&"TopicArn".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write TopicConfiguration contents to a SignedRequest
struct TopicConfigurationWriter;
impl TopicConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TopicConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for filter
		if let Some(ref obj) = obj.filter {
			body = NotificationConfigurationFilterWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Filter".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = NotificationIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Id".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = TopicArnWriter::write_params(request, &obj.topic_arn, Some(&ArgumentLocation::Body), &"Topic".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct Destination {
	/// Amazon resource name (ARN) of the bucket where you want Amazon S3 to store
	/// replicas of the object identified by the rule.
	pub bucket: BucketName,
	/// The class of storage used to store the object.
	pub storage_class: Option<StorageClass>,
}


/// Parse Destination from response
struct DestinationParser;
impl DestinationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Destination, XmlParseError> {
		println!("in DestinationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Destination::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Bucket and child shape is BucketName
			if current_name == "Bucket"{
				obj.bucket = try!(BucketNameParser::parse_response(Some(&"Bucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is StorageClass
			if current_name == "StorageClass"{
				obj.storage_class = Some(try!(StorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Destination contents to a SignedRequest
struct DestinationWriter;
impl DestinationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Destination, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Body), &"Bucket".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for storage_class
		if let Some(ref obj) = obj.storage_class {
			body = StorageClassWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
		}
		body
	}
}
pub type IfNoneMatch = String;

/// Parse IfNoneMatch from response
struct IfNoneMatchParser;
impl IfNoneMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IfNoneMatch, XmlParseError> {
		println!("in IfNoneMatchParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IfNoneMatch contents to a SignedRequest
struct IfNoneMatchWriter;
impl IfNoneMatchWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IfNoneMatch, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IfNoneMatch TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// The specified bucket does not exist.
#[derive(Debug, Default)]
pub struct NoSuchBucket;


/// Parse NoSuchBucket from response
struct NoSuchBucketParser;
impl NoSuchBucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoSuchBucket, XmlParseError> {
		println!("in NoSuchBucketParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NoSuchBucket::default();
		Ok(obj)
	}
}
/// Write NoSuchBucket contents to a SignedRequest
struct NoSuchBucketWriter;
impl NoSuchBucketWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoSuchBucket, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
//NEEDS ENUM for ObjectVersionStorageClass
#[derive(Debug,PartialEq)]
pub enum ObjectVersionStorageClass {
	STANDARD,
}
impl fmt::Display for ObjectVersionStorageClass {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ObjectVersionStorageClass::STANDARD => write!(f, "STANDARD"),
		}
	}
}
impl Default for ObjectVersionStorageClass{
	fn default() -> ObjectVersionStorageClass{
		ObjectVersionStorageClass::STANDARD
	}
}
impl From<String> for ObjectVersionStorageClass{
	fn from(string: String) -> ObjectVersionStorageClass{
		match string.as_ref() {
			"STANDARD" => ObjectVersionStorageClass::STANDARD,
			_ => ObjectVersionStorageClass::default(),
		}
	}
}

/// Parse ObjectVersionStorageClass from response
struct ObjectVersionStorageClassParser;
impl ObjectVersionStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
		println!("in ObjectVersionStorageClassParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ObjectVersionStorageClass = ObjectVersionStorageClass::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ObjectVersionStorageClass::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ObjectVersionStorageClass::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ObjectVersionStorageClass contents to a SignedRequest
struct ObjectVersionStorageClassWriter;
impl ObjectVersionStorageClassWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectVersionStorageClass, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ObjectVersionStorageClass TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MultipartUploadId = String;

/// Parse MultipartUploadId from response
struct MultipartUploadIdParser;
impl MultipartUploadIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MultipartUploadId, XmlParseError> {
		println!("in MultipartUploadIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MultipartUploadId contents to a SignedRequest
struct MultipartUploadIdWriter;
impl MultipartUploadIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MultipartUploadId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MultipartUploadId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Role = String;

/// Parse Role from response
struct RoleParser;
impl RoleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Role, XmlParseError> {
		println!("in RoleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Role contents to a SignedRequest
struct RoleWriter;
impl RoleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Role, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Role TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type WebsiteRedirectLocation = String;

/// Parse WebsiteRedirectLocation from response
struct WebsiteRedirectLocationParser;
impl WebsiteRedirectLocationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<WebsiteRedirectLocation, XmlParseError> {
		println!("in WebsiteRedirectLocationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write WebsiteRedirectLocation contents to a SignedRequest
struct WebsiteRedirectLocationWriter;
impl WebsiteRedirectLocationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a WebsiteRedirectLocation, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT WebsiteRedirectLocation TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for BucketVersioningStatus
#[derive(Debug,PartialEq)]
pub enum BucketVersioningStatus {
	Enabled,
	Suspended,
}
impl fmt::Display for BucketVersioningStatus {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			BucketVersioningStatus::Enabled => write!(f, "Enabled"),
			BucketVersioningStatus::Suspended => write!(f, "Suspended"),
		}
	}
}
impl Default for BucketVersioningStatus{
	fn default() -> BucketVersioningStatus{
		BucketVersioningStatus::Enabled
	}
}
impl From<String> for BucketVersioningStatus{
	fn from(string: String) -> BucketVersioningStatus{
		match string.as_ref() {
			"Enabled" => BucketVersioningStatus::Enabled,
			"Suspended" => BucketVersioningStatus::Suspended,
			_ => BucketVersioningStatus::default(),
		}
	}
}

/// Parse BucketVersioningStatus from response
struct BucketVersioningStatusParser;
impl BucketVersioningStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketVersioningStatus, XmlParseError> {
		println!("in BucketVersioningStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : BucketVersioningStatus = BucketVersioningStatus::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = BucketVersioningStatus::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = BucketVersioningStatus::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write BucketVersioningStatus contents to a SignedRequest
struct BucketVersioningStatusWriter;
impl BucketVersioningStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketVersioningStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT BucketVersioningStatus TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketReplicationRequest {
	pub replication_configuration: ReplicationConfiguration,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketReplicationRequest contents to a SignedRequest
struct PutBucketReplicationRequestWriter;
impl PutBucketReplicationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketReplicationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ReplicationConfigurationWriter::write_params(request, &obj.replication_configuration, Some(&ArgumentLocation::Body), &"ReplicationConfiguration".to_string());
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetObjectTorrentRequest {
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write GetObjectTorrentRequest contents to a SignedRequest
struct GetObjectTorrentRequestWriter;
impl GetObjectTorrentRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetObjectTorrentRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketReplicationRequest {
	pub bucket: BucketName,
}

/// Write GetBucketReplicationRequest contents to a SignedRequest
struct GetBucketReplicationRequestWriter;
impl GetBucketReplicationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketReplicationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
/// Container for the transition rule that describes when noncurrent objects
/// transition to the STANDARD_IA or GLACIER storage class. If your bucket is
/// versioning-enabled (or versioning is suspended), you can set this action to
/// request that Amazon S3 transition noncurrent object versions to the
/// STANDARD_IA or GLACIER storage class at a specific period in the object's
/// lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionTransition {
	/// Specifies the number of days an object is noncurrent before Amazon S3 can
	/// perform the associated action. For information about the noncurrent days
	/// calculations, see [How Amazon S3 Calculates When an Object Became
	/// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
	/// Storage Service Developer Guide.
	pub noncurrent_days: Days,
	/// The class of storage used to store the object.
	pub storage_class: TransitionStorageClass,
}


/// Parse NoncurrentVersionTransition from response
struct NoncurrentVersionTransitionParser;
impl NoncurrentVersionTransitionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoncurrentVersionTransition, XmlParseError> {
		println!("in NoncurrentVersionTransitionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NoncurrentVersionTransition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of NoncurrentDays and child shape is Days
			if current_name == "NoncurrentDays"{
				obj.noncurrent_days = try!(DaysParser::parse_response(Some(&"NoncurrentDays".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is TransitionStorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(TransitionStorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write NoncurrentVersionTransition contents to a SignedRequest
struct NoncurrentVersionTransitionWriter;
impl NoncurrentVersionTransitionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoncurrentVersionTransition, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DaysWriter::write_params(request, &obj.noncurrent_days, Some(&ArgumentLocation::Body), &"NoncurrentDays".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TransitionStorageClassWriter::write_params(request, &obj.storage_class, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketTaggingOutput {
	pub tag_set: TagSet,
}


/// Parse GetBucketTaggingOutput from response
struct GetBucketTaggingOutputParser;
impl GetBucketTaggingOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketTaggingOutput, XmlParseError> {
		println!("in GetBucketTaggingOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketTaggingOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of TagSet and child shape is TagSet
			if current_name == "TagSet"{
				obj.tag_set = try!(TagSetParser::parse_response(Some(&"TagSet".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type LifecycleRules = Vec<LifecycleRule>;

/// Parse LifecycleRules from response
struct LifecycleRulesParser;
impl LifecycleRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LifecycleRules, XmlParseError> {
		println!("in LifecycleRulesParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "LifecycleRule" {
//we need to iterate over members of LifecycleRule
// obj.push for LifecycleRule
			obj.push(try!(LifecycleRuleParser::parse_response(Some(&"LifecycleRule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write LifecycleRules contents to a SignedRequest
struct LifecycleRulesWriter;
impl LifecycleRulesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LifecycleRules, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for LifecycleRule
;
		body
	}
}
pub type Body = Vec<u8>;

/// Parse Body from response
struct BodyParser;
impl BodyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Body, XmlParseError> {
		println!("in BodyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Vec<u8> = Vec::with_capacity(1);
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Vec::with_capacity(1);
					},
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Body contents to a SignedRequest
struct BodyWriter;
impl BodyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Body, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is blob
		request.set_payload(Some(obj));
		body
	}
}
#[derive(Debug, Default)]
pub struct PutObjectOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	pub request_charged: RequestCharged,
	/// Version of the object.
	pub version_id: ObjectVersionId,
	/// Entity tag for the uploaded object.
	pub e_tag: ETag,
	/// If the object expiration is configured, this will contain the expiration date
	/// (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.
	pub expiration: Expiration,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
}


/// Parse PutObjectOutput from response
struct PutObjectOutputParser;
impl PutObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<PutObjectOutput, XmlParseError> {
		println!("in PutObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = PutObjectOutput::default();
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ETag and child shape is ETag
		obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expiration and child shape is Expiration
		obj.expiration = try!(ExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct PutObjectAclOutput {
	pub request_charged: RequestCharged,
}


/// Parse PutObjectAclOutput from response
struct PutObjectAclOutputParser;
impl PutObjectAclOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<PutObjectAclOutput, XmlParseError> {
		println!("in PutObjectAclOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = PutObjectAclOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
/// Container for replication rules. You can add as many as 1,000 rules. Total
/// replication configuration size can be up to 2 MB.
#[derive(Debug, Default)]
pub struct ReplicationConfiguration {
	/// Container for information about a particular replication rule. Replication
	/// configuration must have at least one rule and can contain up to 1,000 rules.
	pub rules: ReplicationRules,
	/// Amazon Resource Name (ARN) of an IAM role for Amazon S3 to assume when
	/// replicating the objects.
	pub role: Role,
}


/// Parse ReplicationConfiguration from response
struct ReplicationConfigurationParser;
impl ReplicationConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplicationConfiguration, XmlParseError> {
		println!("in ReplicationConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ReplicationConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Rules and child shape is ReplicationRules
			if current_name == "Rules"{
				obj.rules = try!(ReplicationRulesParser::parse_response(Some(&"Rules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Role and child shape is Role
			if current_name == "Role"{
				obj.role = try!(RoleParser::parse_response(Some(&"Role".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write ReplicationConfiguration contents to a SignedRequest
struct ReplicationConfigurationWriter;
impl ReplicationConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplicationConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ReplicationRulesWriter::write_params(request, &obj.rules, Some(&ArgumentLocation::Body), &"ReplicationRule".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = RoleWriter::write_params(request, &obj.role, Some(&ArgumentLocation::Body), &"Role".to_string());
		body
	}
}
/// Confirms that the requester knows that she or he will be charged for the
/// request. Bucket owners need not specify this parameter in their requests.
/// Documentation on downloading objects from requester pays buckets can be found
/// at http://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBucket
/// s.html
//NEEDS ENUM for RequestPayer
#[derive(Debug,PartialEq)]
pub enum RequestPayer {
	requester,
}
impl fmt::Display for RequestPayer {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			RequestPayer::requester => write!(f, "requester"),
		}
	}
}
impl Default for RequestPayer{
	fn default() -> RequestPayer{
		RequestPayer::requester
	}
}
impl From<String> for RequestPayer{
	fn from(string: String) -> RequestPayer{
		match string.as_ref() {
			"requester" => RequestPayer::requester,
			_ => RequestPayer::default(),
		}
	}
}

/// Parse RequestPayer from response
struct RequestPayerParser;
impl RequestPayerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RequestPayer, XmlParseError> {
		println!("in RequestPayerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : RequestPayer = RequestPayer::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = RequestPayer::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = RequestPayer::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write RequestPayer contents to a SignedRequest
struct RequestPayerWriter;
impl RequestPayerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RequestPayer, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT RequestPayer TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketWebsiteRequest {
	pub bucket: BucketName,
}

/// Write GetBucketWebsiteRequest contents to a SignedRequest
struct GetBucketWebsiteRequestWriter;
impl GetBucketWebsiteRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketWebsiteRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type Rules = Vec<Rule>;

/// Parse Rules from response
struct RulesParser;
impl RulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Rules, XmlParseError> {
		println!("in RulesParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Rule" {
//we need to iterate over members of Rule
// obj.push for Rule
			obj.push(try!(RuleParser::parse_response(Some(&"Rule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write Rules contents to a SignedRequest
struct RulesWriter;
impl RulesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Rules, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Rule
;
		body
	}
}
#[derive(Debug, Default)]
pub struct CompleteMultipartUploadRequest {
	pub multipart_upload: Option<CompletedMultipartUpload>,
	pub upload_id: MultipartUploadId,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write CompleteMultipartUploadRequest contents to a SignedRequest
struct CompleteMultipartUploadRequestWriter;
impl CompleteMultipartUploadRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CompleteMultipartUploadRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for multipart_upload
		if let Some(ref obj) = obj.multipart_upload {
			body = CompletedMultipartUploadWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"CompleteMultipartUpload".to_string());
		}
//required field: 
		MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Querystring), &"uploadId".to_string());
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CreateBucketOutput {
	pub location: Location,
}


/// Parse CreateBucketOutput from response
struct CreateBucketOutputParser;
impl CreateBucketOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CreateBucketOutput, XmlParseError> {
		println!("in CreateBucketOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CreateBucketOutput::default();
		//parser for cname of Location and child shape is Location
		obj.location = try!(LocationParser::parse_response(Some(&"Location".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct DeleteMarkerEntry {
	pub owner: Owner,
	/// Specifies whether the object is (true) or is not (false) the latest version of
	/// an object.
	pub is_latest: IsLatest,
	/// Version ID of an object.
	pub version_id: ObjectVersionId,
	/// The object key.
	pub key: ObjectKey,
	/// Date and time the object was last modified.
	pub last_modified: LastModified,
}


/// Parse DeleteMarkerEntry from response
struct DeleteMarkerEntryParser;
impl DeleteMarkerEntryParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteMarkerEntry, XmlParseError> {
		println!("in DeleteMarkerEntryParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = DeleteMarkerEntry::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsLatest and child shape is IsLatest
			if current_name == "IsLatest"{
				obj.is_latest = try!(IsLatestParser::parse_response(Some(&"IsLatest".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of VersionId and child shape is ObjectVersionId
			if current_name == "VersionId"{
				obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write DeleteMarkerEntry contents to a SignedRequest
struct DeleteMarkerEntryWriter;
impl DeleteMarkerEntryWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteMarkerEntry, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = OwnerWriter::write_params(request, &obj.owner, Some(&ArgumentLocation::Body), &"Owner".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = IsLatestWriter::write_params(request, &obj.is_latest, Some(&ArgumentLocation::Body), &"IsLatest".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectVersionIdWriter::write_params(request, &obj.version_id, Some(&ArgumentLocation::Body), &"VersionId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = LastModifiedWriter::write_params(request, &obj.last_modified, Some(&ArgumentLocation::Body), &"LastModified".to_string());
		body
	}
}
pub type TargetBucket = String;

/// Parse TargetBucket from response
struct TargetBucketParser;
impl TargetBucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TargetBucket, XmlParseError> {
		println!("in TargetBucketParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write TargetBucket contents to a SignedRequest
struct TargetBucketWriter;
impl TargetBucketWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TargetBucket, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT TargetBucket TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for MFADeleteStatus
#[derive(Debug,PartialEq)]
pub enum MFADeleteStatus {
	Enabled,
	Disabled,
}
impl fmt::Display for MFADeleteStatus {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MFADeleteStatus::Enabled => write!(f, "Enabled"),
			MFADeleteStatus::Disabled => write!(f, "Disabled"),
		}
	}
}
impl Default for MFADeleteStatus{
	fn default() -> MFADeleteStatus{
		MFADeleteStatus::Enabled
	}
}
impl From<String> for MFADeleteStatus{
	fn from(string: String) -> MFADeleteStatus{
		match string.as_ref() {
			"Enabled" => MFADeleteStatus::Enabled,
			"Disabled" => MFADeleteStatus::Disabled,
			_ => MFADeleteStatus::default(),
		}
	}
}

/// Parse MFADeleteStatus from response
struct MFADeleteStatusParser;
impl MFADeleteStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MFADeleteStatus, XmlParseError> {
		println!("in MFADeleteStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : MFADeleteStatus = MFADeleteStatus::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = MFADeleteStatus::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = MFADeleteStatus::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MFADeleteStatus contents to a SignedRequest
struct MFADeleteStatusWriter;
impl MFADeleteStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MFADeleteStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MFADeleteStatus TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MaxKeys = i32;

/// Parse MaxKeys from response
struct MaxKeysParser;
impl MaxKeysParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MaxKeys, XmlParseError> {
		println!("in MaxKeysParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MaxKeys contents to a SignedRequest
struct MaxKeysWriter;
impl MaxKeysWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MaxKeys, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MaxKeys TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Part {
	/// Date and time at which the part was uploaded.
	pub last_modified: LastModified,
	/// Part number identifying the part. This is a positive integer between 1 and
	/// 10,000.
	pub part_number: PartNumber,
	/// Entity tag returned when the part was uploaded.
	pub e_tag: ETag,
	/// Size of the uploaded part data.
	pub size: Size,
}


/// Parse Part from response
struct PartParser;
impl PartParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Part, XmlParseError> {
		println!("in PartParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Part::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of PartNumber and child shape is PartNumber
			if current_name == "PartNumber"{
				obj.part_number = try!(PartNumberParser::parse_response(Some(&"PartNumber".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Size and child shape is Size
			if current_name == "Size"{
				obj.size = try!(SizeParser::parse_response(Some(&"Size".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Part contents to a SignedRequest
struct PartWriter;
impl PartWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Part, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = LastModifiedWriter::write_params(request, &obj.last_modified, Some(&ArgumentLocation::Body), &"LastModified".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = PartNumberWriter::write_params(request, &obj.part_number, Some(&ArgumentLocation::Body), &"PartNumber".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ETagWriter::write_params(request, &obj.e_tag, Some(&ArgumentLocation::Body), &"ETag".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = SizeWriter::write_params(request, &obj.size, Some(&ArgumentLocation::Body), &"Size".to_string());
		body
	}
}
pub type TargetGrants = Vec<TargetGrant>;

/// Parse TargetGrants from response
struct TargetGrantsParser;
impl TargetGrantsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TargetGrants, XmlParseError> {
		println!("in TargetGrantsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Grant" {
//we need to iterate over members of Grant
// skip Grant.  It's a location name.
// obj.push for TargetGrant
			obj.push(try!(TargetGrantParser::parse_response(Some(&"TargetGrant"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write TargetGrants contents to a SignedRequest
struct TargetGrantsWriter;
impl TargetGrantsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TargetGrants, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for TargetGrant
;
		body
	}
}
#[derive(Debug, Default)]
pub struct ListMultipartUploadsRequest {
	/// Together with key-marker, specifies the multipart upload after which listing
	/// should begin. If key-marker is not specified, the upload-id-marker parameter
	/// is ignored.
	pub upload_id_marker: Option<UploadIdMarker>,
	pub bucket: BucketName,
	/// Character you use to group keys.
	pub delimiter: Option<Delimiter>,
	/// Lists in-progress uploads only for those keys that begin with the specified
	/// prefix.
	pub prefix: Option<Prefix>,
	/// Together with upload-id-marker, this parameter specifies the multipart upload
	/// after which listing should begin.
	pub key_marker: Option<KeyMarker>,
	/// Sets the maximum number of multipart uploads, from 1 to 1,000, to return in
	/// the response body. 1,000 is the maximum number of uploads that can be returned
	/// in a response.
	pub max_uploads: Option<MaxUploads>,
	pub encoding_type: Option<EncodingType>,
}

/// Write ListMultipartUploadsRequest contents to a SignedRequest
struct ListMultipartUploadsRequestWriter;
impl ListMultipartUploadsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ListMultipartUploadsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for upload_id_marker
		if let Some(ref obj) = obj.upload_id_marker {
			UploadIdMarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"upload-id-marker".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for delimiter
		if let Some(ref obj) = obj.delimiter {
			DelimiterWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"delimiter".to_string());
		}
		// optional writing for prefix
		if let Some(ref obj) = obj.prefix {
			PrefixWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"prefix".to_string());
		}
		// optional writing for key_marker
		if let Some(ref obj) = obj.key_marker {
			KeyMarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"key-marker".to_string());
		}
		// optional writing for max_uploads
		if let Some(ref obj) = obj.max_uploads {
			MaxUploadsWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"max-uploads".to_string());
		}
		// optional writing for encoding_type
		if let Some(ref obj) = obj.encoding_type {
			EncodingTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"encoding-type".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyRequest {
	pub bucket: BucketName,
}

/// Write GetBucketPolicyRequest contents to a SignedRequest
struct GetBucketPolicyRequestWriter;
impl GetBucketPolicyRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketPolicyRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type CloudFunction = String;

/// Parse CloudFunction from response
struct CloudFunctionParser;
impl CloudFunctionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CloudFunction, XmlParseError> {
		println!("in CloudFunctionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CloudFunction contents to a SignedRequest
struct CloudFunctionWriter;
impl CloudFunctionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CloudFunction, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CloudFunction TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketWebsiteRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketWebsiteRequest contents to a SignedRequest
struct DeleteBucketWebsiteRequestWriter;
impl DeleteBucketWebsiteRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketWebsiteRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct VersioningConfiguration {
	/// The versioning state of the bucket.
	pub status: BucketVersioningStatus,
	/// Specifies whether MFA delete is enabled in the bucket versioning
	/// configuration. This element is only returned if the bucket has been configured
	/// with MFA delete. If the bucket has never been so configured, this element is
	/// not returned.
	pub mfa_delete: MFADelete,
}


/// Parse VersioningConfiguration from response
struct VersioningConfigurationParser;
impl VersioningConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<VersioningConfiguration, XmlParseError> {
		println!("in VersioningConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = VersioningConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Status and child shape is BucketVersioningStatus
			if current_name == "Status"{
				obj.status = try!(BucketVersioningStatusParser::parse_response(Some(&"Status".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MFADelete and child shape is MFADelete
			if current_name == "MFADelete"{
				obj.mfa_delete = try!(MFADeleteParser::parse_response(Some(&"MFADelete".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write VersioningConfiguration contents to a SignedRequest
struct VersioningConfigurationWriter;
impl VersioningConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a VersioningConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketVersioningStatusWriter::write_params(request, &obj.status, Some(&ArgumentLocation::Body), &"Status".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = MFADeleteWriter::write_params(request, &obj.mfa_delete, Some(&ArgumentLocation::Body), &"MfaDelete".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketCorsRequest {
	pub bucket: BucketName,
}

/// Write GetBucketCorsRequest contents to a SignedRequest
struct GetBucketCorsRequestWriter;
impl GetBucketCorsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketCorsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct IndexDocument {
	/// A suffix that is appended to a request that is for a directory on the website
	/// endpoint (e.g. if the suffix is index.html and you make a request to
	/// samplebucket/images/ the data that is returned will be for the object with the
	/// key name images/index.html) The suffix must not be empty and must not include
	/// a slash character.
	pub suffix: Suffix,
}


/// Parse IndexDocument from response
struct IndexDocumentParser;
impl IndexDocumentParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IndexDocument, XmlParseError> {
		println!("in IndexDocumentParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = IndexDocument::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Suffix and child shape is Suffix
			if current_name == "Suffix"{
				obj.suffix = try!(SuffixParser::parse_response(Some(&"Suffix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write IndexDocument contents to a SignedRequest
struct IndexDocumentWriter;
impl IndexDocumentWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IndexDocument, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = SuffixWriter::write_params(request, &obj.suffix, Some(&ArgumentLocation::Body), &"Suffix".to_string());
		body
	}
}
pub type TopicConfigurationList = Vec<TopicConfiguration>;

/// Parse TopicConfigurationList from response
struct TopicConfigurationListParser;
impl TopicConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TopicConfigurationList, XmlParseError> {
		println!("in TopicConfigurationListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "TopicConfiguration" {
//we need to iterate over members of TopicConfiguration
// obj.push for TopicConfiguration
			obj.push(try!(TopicConfigurationParser::parse_response(Some(&"TopicConfiguration"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write TopicConfigurationList contents to a SignedRequest
struct TopicConfigurationListWriter;
impl TopicConfigurationListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TopicConfigurationList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for TopicConfiguration
;
		body
	}
}
pub type ReplaceKeyPrefixWith = String;

/// Parse ReplaceKeyPrefixWith from response
struct ReplaceKeyPrefixWithParser;
impl ReplaceKeyPrefixWithParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplaceKeyPrefixWith, XmlParseError> {
		println!("in ReplaceKeyPrefixWithParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ReplaceKeyPrefixWith contents to a SignedRequest
struct ReplaceKeyPrefixWithWriter;
impl ReplaceKeyPrefixWithWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplaceKeyPrefixWith, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ReplaceKeyPrefixWith TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CreateMultipartUploadRequest {
	pub request_payer: Option<RequestPayer>,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: Option<ContentEncoding>,
	/// The type of storage to use for the object. Defaults to 'STANDARD'.
	pub storage_class: Option<StorageClass>,
	/// Allows grantee to read the object ACL.
	pub grant_read_acp: Option<GrantReadACP>,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: Option<ServerSideEncryption>,
	/// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
	/// requests for an object protected by AWS KMS will fail if not made via SSL or
	/// using SigV4. Documentation on configuring any of the officially supported AWS
	/// SDKs and CLI can be found at
	/// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
	/// signature-version
	pub ssekms_key_id: Option<SSEKMSKeyId>,
	/// Specifies presentational information for the object.
	pub content_disposition: Option<ContentDisposition>,
	/// A map of metadata to store with the object in S3.
	pub metadata: Option<Metadata>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: Option<WebsiteRedirectLocation>,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Option<Expires>,
	pub key: ObjectKey,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: Option<CacheControl>,
	pub bucket: BucketName,
	/// Allows grantee to read the object data and its metadata.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to write the ACL for the applicable object.
	pub grant_write_acp: Option<GrantWriteACP>,
	/// The canned ACL to apply to the object.
	pub acl: Option<ObjectCannedACL>,
	/// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
	pub grant_full_control: Option<GrantFullControl>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// A standard MIME type describing the format of the object data.
	pub content_type: Option<ContentType>,
	/// The language the content is in.
	pub content_language: Option<ContentLanguage>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

/// Write CreateMultipartUploadRequest contents to a SignedRequest
struct CreateMultipartUploadRequestWriter;
impl CreateMultipartUploadRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CreateMultipartUploadRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for content_encoding
		if let Some(ref obj) = obj.content_encoding {
			ContentEncodingWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Encoding".to_string());
		}
		// optional writing for storage_class
		if let Some(ref obj) = obj.storage_class {
			StorageClassWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-storage-class".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		// optional writing for server_side_encryption
		if let Some(ref obj) = obj.server_side_encryption {
			ServerSideEncryptionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption".to_string());
		}
		// optional writing for ssekms_key_id
		if let Some(ref obj) = obj.ssekms_key_id {
			SSEKMSKeyIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-aws-kms-key-id".to_string());
		}
		// optional writing for content_disposition
		if let Some(ref obj) = obj.content_disposition {
			ContentDispositionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Disposition".to_string());
		}
		// optional writing for metadata
		if let Some(ref obj) = obj.metadata {
			MetadataWriter::write_params(request, &obj, Some(&ArgumentLocation::Headers), &"x-amz-meta-".to_string());
		}
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
		// optional writing for website_redirect_location
		if let Some(ref obj) = obj.website_redirect_location {
			WebsiteRedirectLocationWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-website-redirect-location".to_string());
		}
		// optional writing for expires
		if let Some(ref obj) = obj.expires {
			ExpiresWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Expires".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for cache_control
		if let Some(ref obj) = obj.cache_control {
			CacheControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Cache-Control".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			ObjectCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for content_type
		if let Some(ref obj) = obj.content_type {
			ContentTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Type".to_string());
		}
		// optional writing for content_language
		if let Some(ref obj) = obj.content_language {
			ContentLanguageWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Language".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketCorsRequest {
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	pub cors_configuration: CORSConfiguration,
}

/// Write PutBucketCorsRequest contents to a SignedRequest
struct PutBucketCorsRequestWriter;
impl PutBucketCorsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketCorsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = CORSConfigurationWriter::write_params(request, &obj.cors_configuration, Some(&ArgumentLocation::Body), &"CORSConfiguration".to_string());
		body
	}
}
pub type CopySourceSSECustomerAlgorithm = String;

/// Parse CopySourceSSECustomerAlgorithm from response
struct CopySourceSSECustomerAlgorithmParser;
impl CopySourceSSECustomerAlgorithmParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceSSECustomerAlgorithm, XmlParseError> {
		println!("in CopySourceSSECustomerAlgorithmParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceSSECustomerAlgorithm contents to a SignedRequest
struct CopySourceSSECustomerAlgorithmWriter;
impl CopySourceSSECustomerAlgorithmWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceSSECustomerAlgorithm, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceSSECustomerAlgorithm TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketAclOutput {
	pub owner: Owner,
	/// A list of grants.
	pub grants: Grants,
}


/// Parse GetBucketAclOutput from response
struct GetBucketAclOutputParser;
impl GetBucketAclOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketAclOutput, XmlParseError> {
		println!("in GetBucketAclOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketAclOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Grants and child shape is Grants
			if current_name == "Grants"{
				obj.grants = try!(GrantsParser::parse_response(Some(&"Grants".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type Days = i32;

/// Parse Days from response
struct DaysParser;
impl DaysParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Days, XmlParseError> {
		println!("in DaysParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Days contents to a SignedRequest
struct DaysWriter;
impl DaysWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Days, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Days TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Value = String;

/// Parse Value from response
struct ValueParser;
impl ValueParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Value, XmlParseError> {
		println!("in ValueParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Value contents to a SignedRequest
struct ValueWriter;
impl ValueWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Value, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Value TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type DeletedObjects = Vec<DeletedObject>;

/// Parse DeletedObjects from response
struct DeletedObjectsParser;
impl DeletedObjectsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeletedObjects, XmlParseError> {
		println!("in DeletedObjectsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "DeletedObject" {
//we need to iterate over members of DeletedObject
// obj.push for DeletedObject
			obj.push(try!(DeletedObjectParser::parse_response(Some(&"DeletedObject"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write DeletedObjects contents to a SignedRequest
struct DeletedObjectsWriter;
impl DeletedObjectsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeletedObjects, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for DeletedObject
;
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketLifecycleRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketLifecycleRequest contents to a SignedRequest
struct DeleteBucketLifecycleRequestWriter;
impl DeleteBucketLifecycleRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketLifecycleRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct Tag {
	/// Value of the tag.
	pub value: Value,
	/// Name of the tag.
	pub key: ObjectKey,
}


/// Parse Tag from response
struct TagParser;
impl TagParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Tag, XmlParseError> {
		println!("in TagParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Tag::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Value and child shape is Value
			if current_name == "Value"{
				obj.value = try!(ValueParser::parse_response(Some(&"Value".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Tag contents to a SignedRequest
struct TagWriter;
impl TagWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Tag, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ValueWriter::write_params(request, &obj.value, Some(&ArgumentLocation::Body), &"Value".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
		body
	}
}
pub type KeyMarker = String;

/// Parse KeyMarker from response
struct KeyMarkerParser;
impl KeyMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<KeyMarker, XmlParseError> {
		println!("in KeyMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write KeyMarker contents to a SignedRequest
struct KeyMarkerWriter;
impl KeyMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a KeyMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT KeyMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

/// Parse DeleteMarkers from response
struct DeleteMarkersParser;
impl DeleteMarkersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteMarkers, XmlParseError> {
		println!("in DeleteMarkersParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "DeleteMarkerEntry" {
//we need to iterate over members of DeleteMarkerEntry
// obj.push for DeleteMarkerEntry
			obj.push(try!(DeleteMarkerEntryParser::parse_response(Some(&"DeleteMarkerEntry"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write DeleteMarkers contents to a SignedRequest
struct DeleteMarkersWriter;
impl DeleteMarkersWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteMarkers, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for DeleteMarkerEntry
;
		body
	}
}
#[derive(Debug, Default)]
pub struct AbortMultipartUploadOutput {
	pub request_charged: RequestCharged,
}


/// Parse AbortMultipartUploadOutput from response
struct AbortMultipartUploadOutputParser;
impl AbortMultipartUploadOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AbortMultipartUploadOutput, XmlParseError> {
		println!("in AbortMultipartUploadOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = AbortMultipartUploadOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct PutBucketPolicyRequest {
	/// The bucket policy as a JSON document.
	pub policy: Policy,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketPolicyRequest contents to a SignedRequest
struct PutBucketPolicyRequestWriter;
impl PutBucketPolicyRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketPolicyRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = PolicyWriter::write_params(request, &obj.policy, Some(&ArgumentLocation::Body), &"Policy".to_string());
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type ResponseContentDisposition = String;

/// Parse ResponseContentDisposition from response
struct ResponseContentDispositionParser;
impl ResponseContentDispositionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseContentDisposition, XmlParseError> {
		println!("in ResponseContentDispositionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseContentDisposition contents to a SignedRequest
struct ResponseContentDispositionWriter;
impl ResponseContentDispositionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseContentDisposition, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseContentDisposition TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct BucketLifecycleConfiguration {
	pub rules: LifecycleRules,
}


/// Parse BucketLifecycleConfiguration from response
struct BucketLifecycleConfigurationParser;
impl BucketLifecycleConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketLifecycleConfiguration, XmlParseError> {
		println!("in BucketLifecycleConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = BucketLifecycleConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Rules and child shape is LifecycleRules
			if current_name == "Rules"{
				obj.rules = try!(LifecycleRulesParser::parse_response(Some(&"Rules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write BucketLifecycleConfiguration contents to a SignedRequest
struct BucketLifecycleConfigurationWriter;
impl BucketLifecycleConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketLifecycleConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = LifecycleRulesWriter::write_params(request, &obj.rules, Some(&ArgumentLocation::Body), &"LifecycleRule".to_string());
		body
	}
}
pub type GrantFullControl = String;

/// Parse GrantFullControl from response
struct GrantFullControlParser;
impl GrantFullControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
		println!("in GrantFullControlParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write GrantFullControl contents to a SignedRequest
struct GrantFullControlWriter;
impl GrantFullControlWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GrantFullControl, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT GrantFullControl TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetObjectAclRequest {
	/// VersionId used to reference a specific version of the object.
	pub version_id: Option<ObjectVersionId>,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write GetObjectAclRequest contents to a SignedRequest
struct GetObjectAclRequestWriter;
impl GetObjectAclRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetObjectAclRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"versionId".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
pub type Parts = Vec<Part>;

/// Parse Parts from response
struct PartsParser;
impl PartsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Parts, XmlParseError> {
		println!("in PartsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Part" {
//we need to iterate over members of Part
// obj.push for Part
			obj.push(try!(PartParser::parse_response(Some(&"Part"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write Parts contents to a SignedRequest
struct PartsWriter;
impl PartsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Parts, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Part
;
		body
	}
}
pub type CommonPrefixList = Vec<CommonPrefix>;

/// Parse CommonPrefixList from response
struct CommonPrefixListParser;
impl CommonPrefixListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CommonPrefixList, XmlParseError> {
		println!("in CommonPrefixListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "CommonPrefix" {
//we need to iterate over members of CommonPrefix
// obj.push for CommonPrefix
			obj.push(try!(CommonPrefixParser::parse_response(Some(&"CommonPrefix"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write CommonPrefixList contents to a SignedRequest
struct CommonPrefixListWriter;
impl CommonPrefixListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CommonPrefixList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for CommonPrefix
;
		body
	}
}
//NEEDS ENUM for Protocol
#[derive(Debug,PartialEq)]
pub enum Protocol {
	http,
	https,
}
impl fmt::Display for Protocol {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Protocol::http => write!(f, "http"),
			Protocol::https => write!(f, "https"),
		}
	}
}
impl Default for Protocol{
	fn default() -> Protocol{
		Protocol::http
	}
}
impl From<String> for Protocol{
	fn from(string: String) -> Protocol{
		match string.as_ref() {
			"http" => Protocol::http,
			"https" => Protocol::https,
			_ => Protocol::default(),
		}
	}
}

/// Parse Protocol from response
struct ProtocolParser;
impl ProtocolParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Protocol, XmlParseError> {
		println!("in ProtocolParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Protocol = Protocol::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Protocol::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = Protocol::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Protocol contents to a SignedRequest
struct ProtocolWriter;
impl ProtocolWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Protocol, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Protocol TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Suffix = String;

/// Parse Suffix from response
struct SuffixParser;
impl SuffixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Suffix, XmlParseError> {
		println!("in SuffixParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Suffix contents to a SignedRequest
struct SuffixWriter;
impl SuffixWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Suffix, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Suffix TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type AllowedMethod = String;

/// Parse AllowedMethod from response
struct AllowedMethodParser;
impl AllowedMethodParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedMethod, XmlParseError> {
		println!("in AllowedMethodParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write AllowedMethod contents to a SignedRequest
struct AllowedMethodWriter;
impl AllowedMethodWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedMethod, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT AllowedMethod TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// Container for specifying an configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Queue Service (Amazon SQS) queue.
#[derive(Debug, Default)]
pub struct QueueConfiguration {
	pub filter: Option<NotificationConfigurationFilter>,
	pub id: Option<NotificationId>,
	/// Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects
	/// events of specified type.
	pub queue_arn: QueueArn,
	pub events: EventList,
}


/// Parse QueueConfiguration from response
struct QueueConfigurationParser;
impl QueueConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<QueueConfiguration, XmlParseError> {
		println!("in QueueConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = QueueConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Filter and child shape is NotificationConfigurationFilter
			if current_name == "Filter"{
				obj.filter = Some(try!(NotificationConfigurationFilterParser::parse_response(Some(&"Filter".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = Some(try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of QueueArn and child shape is QueueArn
			if current_name == "QueueArn"{
				obj.queue_arn = try!(QueueArnParser::parse_response(Some(&"QueueArn".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write QueueConfiguration contents to a SignedRequest
struct QueueConfigurationWriter;
impl QueueConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a QueueConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for filter
		if let Some(ref obj) = obj.filter {
			body = NotificationConfigurationFilterWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Filter".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = NotificationIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Id".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = QueueArnWriter::write_params(request, &obj.queue_arn, Some(&ArgumentLocation::Body), &"Queue".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
//NEEDS ENUM for BucketCannedACL
#[derive(Debug,PartialEq)]
pub enum BucketCannedACL {
	private,
	public_read,
	public_read_write,
	authenticated_read,
}
impl fmt::Display for BucketCannedACL {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			BucketCannedACL::private => write!(f, "private"),
			BucketCannedACL::public_read => write!(f, "public-read"),
			BucketCannedACL::public_read_write => write!(f, "public-read-write"),
			BucketCannedACL::authenticated_read => write!(f, "authenticated-read"),
		}
	}
}
impl Default for BucketCannedACL{
	fn default() -> BucketCannedACL{
		BucketCannedACL::private
	}
}
impl From<String> for BucketCannedACL{
	fn from(string: String) -> BucketCannedACL{
		match string.as_ref() {
			"private" => BucketCannedACL::private,
			"public-read" => BucketCannedACL::public_read,
			"public-read-write" => BucketCannedACL::public_read_write,
			"authenticated-read" => BucketCannedACL::authenticated_read,
			_ => BucketCannedACL::default(),
		}
	}
}

/// Parse BucketCannedACL from response
struct BucketCannedACLParser;
impl BucketCannedACLParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketCannedACL, XmlParseError> {
		println!("in BucketCannedACLParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : BucketCannedACL = BucketCannedACL::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = BucketCannedACL::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = BucketCannedACL::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write BucketCannedACL contents to a SignedRequest
struct BucketCannedACLWriter;
impl BucketCannedACLWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketCannedACL, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT BucketCannedACL TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type SSECustomerKey = String;

/// Parse SSECustomerKey from response
struct SSECustomerKeyParser;
impl SSECustomerKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<SSECustomerKey, XmlParseError> {
		println!("in SSECustomerKeyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write SSECustomerKey contents to a SignedRequest
struct SSECustomerKeyWriter;
impl SSECustomerKeyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a SSECustomerKey, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT SSECustomerKey TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ResponseCacheControl = String;

/// Parse ResponseCacheControl from response
struct ResponseCacheControlParser;
impl ResponseCacheControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseCacheControl, XmlParseError> {
		println!("in ResponseCacheControlParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseCacheControl contents to a SignedRequest
struct ResponseCacheControlWriter;
impl ResponseCacheControlWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseCacheControl, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseCacheControl TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for FilterRuleName
#[derive(Debug,PartialEq)]
pub enum FilterRuleName {
	prefix,
	suffix,
}
impl fmt::Display for FilterRuleName {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			FilterRuleName::prefix => write!(f, "prefix"),
			FilterRuleName::suffix => write!(f, "suffix"),
		}
	}
}
impl Default for FilterRuleName{
	fn default() -> FilterRuleName{
		FilterRuleName::prefix
	}
}
impl From<String> for FilterRuleName{
	fn from(string: String) -> FilterRuleName{
		match string.as_ref() {
			"prefix" => FilterRuleName::prefix,
			"suffix" => FilterRuleName::suffix,
			_ => FilterRuleName::default(),
		}
	}
}

/// Parse FilterRuleName from response
struct FilterRuleNameParser;
impl FilterRuleNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<FilterRuleName, XmlParseError> {
		println!("in FilterRuleNameParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : FilterRuleName = FilterRuleName::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = FilterRuleName::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = FilterRuleName::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write FilterRuleName contents to a SignedRequest
struct FilterRuleNameWriter;
impl FilterRuleNameWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a FilterRuleName, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT FilterRuleName TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct HeadBucketRequest {
	pub bucket: BucketName,
}

/// Write HeadBucketRequest contents to a SignedRequest
struct HeadBucketRequestWriter;
impl HeadBucketRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a HeadBucketRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct AbortMultipartUploadRequest {
	pub upload_id: MultipartUploadId,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write AbortMultipartUploadRequest contents to a SignedRequest
struct AbortMultipartUploadRequestWriter;
impl AbortMultipartUploadRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AbortMultipartUploadRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Querystring), &"uploadId".to_string());
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
pub type Size = i32;

/// Parse Size from response
struct SizeParser;
impl SizeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Size, XmlParseError> {
		println!("in SizeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Size contents to a SignedRequest
struct SizeWriter;
impl SizeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Size, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Size TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct UploadPartCopyOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	/// The version of the source object that was copied, if you have enabled
	/// versioning on the source bucket.
	pub copy_source_version_id: CopySourceVersionId,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	pub request_charged: RequestCharged,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
	pub copy_part_result: CopyPartResult,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
}


/// Parse UploadPartCopyOutput from response
struct UploadPartCopyOutputParser;
impl UploadPartCopyOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<UploadPartCopyOutput, XmlParseError> {
		println!("in UploadPartCopyOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = UploadPartCopyOutput::default();
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of CopySourceVersionId and child shape is CopySourceVersionId
		obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_response(Some(&"CopySourceVersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CopyPartResult and child shape is CopyPartResult
			if current_name == "CopyPartResult"{
				obj.copy_part_result = try!(CopyPartResultParser::parse_response(Some(&"CopyPartResult".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct Redirect {
	/// The specific object key to use in the redirect request. For example, redirect
	/// request to error.html. Not required if one of the sibling is present. Can be
	/// present only if ReplaceKeyPrefixWith is not provided.
	pub replace_key_with: ReplaceKeyWith,
	/// The host name to use in the redirect request.
	pub host_name: HostName,
	/// Protocol to use (http, https) when redirecting requests. The default is the
	/// protocol that is used in the original request.
	pub protocol: Protocol,
	/// The object key prefix to use in the redirect request. For example, to redirect
	/// requests for all pages with prefix docs/ (objects in the docs/ folder) to
	/// documents/, you can set a condition block with KeyPrefixEquals set to docs/
	/// and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if
	/// one of the siblings is present. Can be present only if ReplaceKeyWith is not
	/// provided.
	pub replace_key_prefix_with: ReplaceKeyPrefixWith,
	/// The HTTP redirect code to use on the response. Not required if one of the
	/// siblings is present.
	pub http_redirect_code: HttpRedirectCode,
}


/// Parse Redirect from response
struct RedirectParser;
impl RedirectParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Redirect, XmlParseError> {
		println!("in RedirectParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Redirect::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of ReplaceKeyWith and child shape is ReplaceKeyWith
			if current_name == "ReplaceKeyWith"{
				obj.replace_key_with = try!(ReplaceKeyWithParser::parse_response(Some(&"ReplaceKeyWith".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of HostName and child shape is HostName
			if current_name == "HostName"{
				obj.host_name = try!(HostNameParser::parse_response(Some(&"HostName".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Protocol and child shape is Protocol
			if current_name == "Protocol"{
				obj.protocol = try!(ProtocolParser::parse_response(Some(&"Protocol".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ReplaceKeyPrefixWith and child shape is ReplaceKeyPrefixWith
			if current_name == "ReplaceKeyPrefixWith"{
				obj.replace_key_prefix_with = try!(ReplaceKeyPrefixWithParser::parse_response(Some(&"ReplaceKeyPrefixWith".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of HttpRedirectCode and child shape is HttpRedirectCode
			if current_name == "HttpRedirectCode"{
				obj.http_redirect_code = try!(HttpRedirectCodeParser::parse_response(Some(&"HttpRedirectCode".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Redirect contents to a SignedRequest
struct RedirectWriter;
impl RedirectWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Redirect, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ReplaceKeyWithWriter::write_params(request, &obj.replace_key_with, Some(&ArgumentLocation::Body), &"ReplaceKeyWith".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = HostNameWriter::write_params(request, &obj.host_name, Some(&ArgumentLocation::Body), &"HostName".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ProtocolWriter::write_params(request, &obj.protocol, Some(&ArgumentLocation::Body), &"Protocol".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ReplaceKeyPrefixWithWriter::write_params(request, &obj.replace_key_prefix_with, Some(&ArgumentLocation::Body), &"ReplaceKeyPrefixWith".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = HttpRedirectCodeWriter::write_params(request, &obj.http_redirect_code, Some(&ArgumentLocation::Body), &"HttpRedirectCode".to_string());
		body
	}
}
pub type CopySourceIfNoneMatch = String;

/// Parse CopySourceIfNoneMatch from response
struct CopySourceIfNoneMatchParser;
impl CopySourceIfNoneMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceIfNoneMatch, XmlParseError> {
		println!("in CopySourceIfNoneMatchParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceIfNoneMatch contents to a SignedRequest
struct CopySourceIfNoneMatchWriter;
impl CopySourceIfNoneMatchWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceIfNoneMatch, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceIfNoneMatch TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketCorsRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketCorsRequest contents to a SignedRequest
struct DeleteBucketCorsRequestWriter;
impl DeleteBucketCorsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketCorsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default)]
pub struct BucketAlreadyExists;


/// Parse BucketAlreadyExists from response
struct BucketAlreadyExistsParser;
impl BucketAlreadyExistsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
		println!("in BucketAlreadyExistsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = BucketAlreadyExists::default();
		Ok(obj)
	}
}
/// Write BucketAlreadyExists contents to a SignedRequest
struct BucketAlreadyExistsWriter;
impl BucketAlreadyExistsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketAlreadyExists, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
//NEEDS ENUM for BucketLocationConstraint
#[derive(Debug,PartialEq)]
pub enum BucketLocationConstraint {
	EU,
	eu_west_1,
	us_west_1,
	us_west_2,
	ap_southeast_1,
	ap_southeast_2,
	ap_northeast_1,
	sa_east_1,
	cn_north_1,
	eu_central_1,
}
impl fmt::Display for BucketLocationConstraint {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			BucketLocationConstraint::EU => write!(f, "EU"),
			BucketLocationConstraint::eu_west_1 => write!(f, "eu-west-1"),
			BucketLocationConstraint::us_west_1 => write!(f, "us-west-1"),
			BucketLocationConstraint::us_west_2 => write!(f, "us-west-2"),
			BucketLocationConstraint::ap_southeast_1 => write!(f, "ap-southeast-1"),
			BucketLocationConstraint::ap_southeast_2 => write!(f, "ap-southeast-2"),
			BucketLocationConstraint::ap_northeast_1 => write!(f, "ap-northeast-1"),
			BucketLocationConstraint::sa_east_1 => write!(f, "sa-east-1"),
			BucketLocationConstraint::cn_north_1 => write!(f, "cn-north-1"),
			BucketLocationConstraint::eu_central_1 => write!(f, "eu-central-1"),
		}
	}
}
impl Default for BucketLocationConstraint{
	fn default() -> BucketLocationConstraint{
		BucketLocationConstraint::EU
	}
}
impl From<String> for BucketLocationConstraint{
	fn from(string: String) -> BucketLocationConstraint{
		match string.as_ref() {
			"EU" => BucketLocationConstraint::EU,
			"eu-west-1" => BucketLocationConstraint::eu_west_1,
			"us-west-1" => BucketLocationConstraint::us_west_1,
			"us-west-2" => BucketLocationConstraint::us_west_2,
			"ap-southeast-1" => BucketLocationConstraint::ap_southeast_1,
			"ap-southeast-2" => BucketLocationConstraint::ap_southeast_2,
			"ap-northeast-1" => BucketLocationConstraint::ap_northeast_1,
			"sa-east-1" => BucketLocationConstraint::sa_east_1,
			"cn-north-1" => BucketLocationConstraint::cn_north_1,
			"eu-central-1" => BucketLocationConstraint::eu_central_1,
			_ => BucketLocationConstraint::default(),
		}
	}
}

/// Parse BucketLocationConstraint from response
struct BucketLocationConstraintParser;
impl BucketLocationConstraintParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
		println!("in BucketLocationConstraintParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : BucketLocationConstraint = BucketLocationConstraint::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = BucketLocationConstraint::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = BucketLocationConstraint::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write BucketLocationConstraint contents to a SignedRequest
struct BucketLocationConstraintWriter;
impl BucketLocationConstraintWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketLocationConstraint, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT BucketLocationConstraint TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketNotificationConfigurationRequest {
	/// Name of the buket to get the notification configuration for.
	pub bucket: BucketName,
}

/// Write GetBucketNotificationConfigurationRequest contents to a SignedRequest
struct GetBucketNotificationConfigurationRequestWriter;
impl GetBucketNotificationConfigurationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketNotificationConfigurationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleConfigurationOutput {
	pub rules: LifecycleRules,
}


/// Parse GetBucketLifecycleConfigurationOutput from response
struct GetBucketLifecycleConfigurationOutputParser;
impl GetBucketLifecycleConfigurationOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketLifecycleConfigurationOutput, XmlParseError> {
		println!("in GetBucketLifecycleConfigurationOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketLifecycleConfigurationOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Rules and child shape is LifecycleRules
			if current_name == "Rules"{
				obj.rules = try!(LifecycleRulesParser::parse_response(Some(&"Rules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type NextKeyMarker = String;

/// Parse NextKeyMarker from response
struct NextKeyMarkerParser;
impl NextKeyMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NextKeyMarker, XmlParseError> {
		println!("in NextKeyMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NextKeyMarker contents to a SignedRequest
struct NextKeyMarkerWriter;
impl NextKeyMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NextKeyMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NextKeyMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type AllowedMethods = Vec<AllowedMethod>;

/// Parse AllowedMethods from response
struct AllowedMethodsParser;
impl AllowedMethodsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedMethods, XmlParseError> {
		println!("in AllowedMethodsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "AllowedMethod" {
//we need to iterate over members of AllowedMethod
// obj.push for AllowedMethod
			obj.push(try!(AllowedMethodParser::parse_response(Some(&"AllowedMethod"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write AllowedMethods contents to a SignedRequest
struct AllowedMethodsWriter;
impl AllowedMethodsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedMethods, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for AllowedMethod
;
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteObjectOutput {
	/// Returns the version ID of the delete marker created as a result of the DELETE
	/// operation.
	pub version_id: ObjectVersionId,
	pub request_charged: RequestCharged,
	/// Specifies whether the versioned object that was permanently deleted was (true)
	/// or was not (false) a delete marker.
	pub delete_marker: DeleteMarker,
}


/// Parse DeleteObjectOutput from response
struct DeleteObjectOutputParser;
impl DeleteObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteObjectOutput, XmlParseError> {
		println!("in DeleteObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = DeleteObjectOutput::default();
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of DeleteMarker and child shape is DeleteMarker
		obj.delete_marker = try!(DeleteMarkerParser::parse_response(Some(&"DeleteMarker".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
pub type VersionIdMarker = String;

/// Parse VersionIdMarker from response
struct VersionIdMarkerParser;
impl VersionIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<VersionIdMarker, XmlParseError> {
		println!("in VersionIdMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write VersionIdMarker contents to a SignedRequest
struct VersionIdMarkerWriter;
impl VersionIdMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a VersionIdMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT VersionIdMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for StorageClass
#[derive(Debug,PartialEq)]
pub enum StorageClass {
	STANDARD,
	REDUCED_REDUNDANCY,
	STANDARD_IA,
}
impl fmt::Display for StorageClass {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			StorageClass::STANDARD => write!(f, "STANDARD"),
			StorageClass::REDUCED_REDUNDANCY => write!(f, "REDUCED_REDUNDANCY"),
			StorageClass::STANDARD_IA => write!(f, "STANDARD_IA"),
		}
	}
}
impl Default for StorageClass{
	fn default() -> StorageClass{
		StorageClass::STANDARD
	}
}
impl From<String> for StorageClass{
	fn from(string: String) -> StorageClass{
		match string.as_ref() {
			"STANDARD" => StorageClass::STANDARD,
			"REDUCED_REDUNDANCY" => StorageClass::REDUCED_REDUNDANCY,
			"STANDARD_IA" => StorageClass::STANDARD_IA,
			_ => StorageClass::default(),
		}
	}
}

/// Parse StorageClass from response
struct StorageClassParser;
impl StorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<StorageClass, XmlParseError> {
		println!("in StorageClassParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : StorageClass = StorageClass::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = StorageClass::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = StorageClass::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write StorageClass contents to a SignedRequest
struct StorageClassWriter;
impl StorageClassWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a StorageClass, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT StorageClass TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
	/// Specifies the region where the bucket will be created. If you don't specify a
	/// region, the bucket will be created in US Standard.
	pub location_constraint: BucketLocationConstraint,
}


/// Parse CreateBucketConfiguration from response
struct CreateBucketConfigurationParser;
impl CreateBucketConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CreateBucketConfiguration, XmlParseError> {
		println!("in CreateBucketConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CreateBucketConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LocationConstraint and child shape is BucketLocationConstraint
			if current_name == "LocationConstraint"{
				obj.location_constraint = try!(BucketLocationConstraintParser::parse_response(Some(&"LocationConstraint".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CreateBucketConfiguration contents to a SignedRequest
struct CreateBucketConfigurationWriter;
impl CreateBucketConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CreateBucketConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketLocationConstraintWriter::write_params(request, &obj.location_constraint, Some(&ArgumentLocation::Body), &"LocationConstraint".to_string());
		body
	}
}
//NEEDS ENUM for BucketLogsPermission
#[derive(Debug,PartialEq)]
pub enum BucketLogsPermission {
	FULL_CONTROL,
	READ,
	WRITE,
}
impl fmt::Display for BucketLogsPermission {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			BucketLogsPermission::FULL_CONTROL => write!(f, "FULL_CONTROL"),
			BucketLogsPermission::READ => write!(f, "READ"),
			BucketLogsPermission::WRITE => write!(f, "WRITE"),
		}
	}
}
impl Default for BucketLogsPermission{
	fn default() -> BucketLogsPermission{
		BucketLogsPermission::FULL_CONTROL
	}
}
impl From<String> for BucketLogsPermission{
	fn from(string: String) -> BucketLogsPermission{
		match string.as_ref() {
			"FULL_CONTROL" => BucketLogsPermission::FULL_CONTROL,
			"READ" => BucketLogsPermission::READ,
			"WRITE" => BucketLogsPermission::WRITE,
			_ => BucketLogsPermission::default(),
		}
	}
}

/// Parse BucketLogsPermission from response
struct BucketLogsPermissionParser;
impl BucketLogsPermissionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketLogsPermission, XmlParseError> {
		println!("in BucketLogsPermissionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : BucketLogsPermission = BucketLogsPermission::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = BucketLogsPermission::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = BucketLogsPermission::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write BucketLogsPermission contents to a SignedRequest
struct BucketLogsPermissionWriter;
impl BucketLogsPermissionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketLogsPermission, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT BucketLogsPermission TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct HeadObjectRequest {
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// Return the object only if it has not been modified since the specified time,
	/// otherwise return a 412 (precondition failed).
	pub if_unmodified_since: Option<IfUnmodifiedSince>,
	/// VersionId used to reference a specific version of the object.
	pub version_id: Option<ObjectVersionId>,
	pub request_payer: Option<RequestPayer>,
	pub bucket: BucketName,
	/// Return the object only if its entity tag (ETag) is different from the one
	/// specified, otherwise return a 304 (not modified).
	pub if_none_match: Option<IfNoneMatch>,
	/// Downloads the specified range bytes of an object. For more information about
	/// the HTTP Range header, go to
	/// http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.
	pub range: Option<Range>,
	pub key: ObjectKey,
	/// Return the object only if its entity tag (ETag) is the same as the one
	/// specified, otherwise return a 412 (precondition failed).
	pub if_match: Option<IfMatch>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
	/// Return the object only if it has been modified since the specified time,
	/// otherwise return a 304 (not modified).
	pub if_modified_since: Option<IfModifiedSince>,
}

/// Write HeadObjectRequest contents to a SignedRequest
struct HeadObjectRequestWriter;
impl HeadObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a HeadObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
		// optional writing for if_unmodified_since
		if let Some(ref obj) = obj.if_unmodified_since {
			IfUnmodifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Unmodified-Since".to_string());
		}
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"versionId".to_string());
		}
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for if_none_match
		if let Some(ref obj) = obj.if_none_match {
			IfNoneMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-None-Match".to_string());
		}
		// optional writing for range
		if let Some(ref obj) = obj.range {
			RangeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Range".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for if_match
		if let Some(ref obj) = obj.if_match {
			IfMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Match".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
		// optional writing for if_modified_since
		if let Some(ref obj) = obj.if_modified_since {
			IfModifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Modified-Since".to_string());
		}
		body
	}
}
pub type DisplayName = String;

/// Parse DisplayName from response
struct DisplayNameParser;
impl DisplayNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DisplayName, XmlParseError> {
		println!("in DisplayNameParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write DisplayName contents to a SignedRequest
struct DisplayNameWriter;
impl DisplayNameWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DisplayName, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT DisplayName TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type GrantReadACP = String;

/// Parse GrantReadACP from response
struct GrantReadACPParser;
impl GrantReadACPParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
		println!("in GrantReadACPParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write GrantReadACP contents to a SignedRequest
struct GrantReadACPWriter;
impl GrantReadACPWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GrantReadACP, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT GrantReadACP TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Grant {
	pub grantee: Grantee,
	/// Specifies the permission given to the grantee.
	pub permission: Permission,
}


/// Parse Grant from response
struct GrantParser;
impl GrantParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Grant, XmlParseError> {
		println!("in GrantParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Grant::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Grantee and child shape is Grantee
			if current_name == "Grantee"{
				obj.grantee = try!(GranteeParser::parse_response(Some(&"Grantee".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Permission and child shape is Permission
			if current_name == "Permission"{
				obj.permission = try!(PermissionParser::parse_response(Some(&"Permission".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Grant contents to a SignedRequest
struct GrantWriter;
impl GrantWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Grant, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = GranteeWriter::write_params(request, &obj.grantee, Some(&ArgumentLocation::Body), &"Grantee".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = PermissionWriter::write_params(request, &obj.permission, Some(&ArgumentLocation::Body), &"Permission".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct TopicConfigurationDeprecated {
	/// Amazon SNS topic to which Amazon S3 will publish a message to report the
	/// specified events for the bucket.
	pub topic: TopicArn,
	pub id: NotificationId,
	/// Bucket event for which to send notifications.
	pub event: Event,
	pub events: EventList,
}


/// Parse TopicConfigurationDeprecated from response
struct TopicConfigurationDeprecatedParser;
impl TopicConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TopicConfigurationDeprecated, XmlParseError> {
		println!("in TopicConfigurationDeprecatedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = TopicConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Topic and child shape is TopicArn
			if current_name == "Topic"{
				obj.topic = try!(TopicArnParser::parse_response(Some(&"Topic".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Event and child shape is Event
			if current_name == "Event"{
				obj.event = try!(EventParser::parse_response(Some(&"Event".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write TopicConfigurationDeprecated contents to a SignedRequest
struct TopicConfigurationDeprecatedWriter;
impl TopicConfigurationDeprecatedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TopicConfigurationDeprecated, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = TopicArnWriter::write_params(request, &obj.topic, Some(&ArgumentLocation::Body), &"Topic".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = NotificationIdWriter::write_params(request, &obj.id, Some(&ArgumentLocation::Body), &"Id".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventWriter::write_params(request, &obj.event, Some(&ArgumentLocation::Body), &"Event".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
pub type CopySourceIfModifiedSince = String;

/// Parse CopySourceIfModifiedSince from response
struct CopySourceIfModifiedSinceParser;
impl CopySourceIfModifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceIfModifiedSince, XmlParseError> {
		println!("in CopySourceIfModifiedSinceParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceIfModifiedSince contents to a SignedRequest
struct CopySourceIfModifiedSinceWriter;
impl CopySourceIfModifiedSinceWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceIfModifiedSince, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceIfModifiedSince TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Initiator {
	/// Name of the Principal.
	pub display_name: DisplayName,
	/// If the principal is an AWS account, it provides the Canonical User ID. If the
	/// principal is an IAM User, it provides a user ARN value.
	pub id: ID,
}


/// Parse Initiator from response
struct InitiatorParser;
impl InitiatorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Initiator, XmlParseError> {
		println!("in InitiatorParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Initiator::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of DisplayName and child shape is DisplayName
			if current_name == "DisplayName"{
				obj.display_name = try!(DisplayNameParser::parse_response(Some(&"DisplayName".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Initiator contents to a SignedRequest
struct InitiatorWriter;
impl InitiatorWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Initiator, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DisplayNameWriter::write_params(request, &obj.display_name, Some(&ArgumentLocation::Body), &"DisplayName".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = IDWriter::write_params(request, &obj.id, Some(&ArgumentLocation::Body), &"ID".to_string());
		body
	}
}
pub type HttpRedirectCode = String;

/// Parse HttpRedirectCode from response
struct HttpRedirectCodeParser;
impl HttpRedirectCodeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<HttpRedirectCode, XmlParseError> {
		println!("in HttpRedirectCodeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write HttpRedirectCode contents to a SignedRequest
struct HttpRedirectCodeWriter;
impl HttpRedirectCodeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a HttpRedirectCode, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT HttpRedirectCode TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ListObjectVersionsRequest {
	pub bucket: BucketName,
	/// Limits the response to keys that begin with the specified prefix.
	pub prefix: Option<Prefix>,
	/// Sets the maximum number of keys returned in the response. The response might
	/// contain fewer keys but will never contain more.
	pub max_keys: Option<MaxKeys>,
	/// A delimiter is a character you use to group keys.
	pub delimiter: Option<Delimiter>,
	/// Specifies the key to start with when listing objects in a bucket.
	pub key_marker: Option<KeyMarker>,
	pub encoding_type: Option<EncodingType>,
	/// Specifies the object version you want to start listing from.
	pub version_id_marker: Option<VersionIdMarker>,
}

/// Write ListObjectVersionsRequest contents to a SignedRequest
struct ListObjectVersionsRequestWriter;
impl ListObjectVersionsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ListObjectVersionsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for prefix
		if let Some(ref obj) = obj.prefix {
			PrefixWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"prefix".to_string());
		}
		// optional writing for max_keys
		if let Some(ref obj) = obj.max_keys {
			MaxKeysWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"max-keys".to_string());
		}
		// optional writing for delimiter
		if let Some(ref obj) = obj.delimiter {
			DelimiterWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"delimiter".to_string());
		}
		// optional writing for key_marker
		if let Some(ref obj) = obj.key_marker {
			KeyMarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"key-marker".to_string());
		}
		// optional writing for encoding_type
		if let Some(ref obj) = obj.encoding_type {
			EncodingTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"encoding-type".to_string());
		}
		// optional writing for version_id_marker
		if let Some(ref obj) = obj.version_id_marker {
			VersionIdMarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"version-id-marker".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketRequest contents to a SignedRequest
struct DeleteBucketRequestWriter;
impl DeleteBucketRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type TargetPrefix = String;

/// Parse TargetPrefix from response
struct TargetPrefixParser;
impl TargetPrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TargetPrefix, XmlParseError> {
		println!("in TargetPrefixParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write TargetPrefix contents to a SignedRequest
struct TargetPrefixWriter;
impl TargetPrefixWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TargetPrefix, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT TargetPrefix TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketPolicyRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketPolicyRequest contents to a SignedRequest
struct DeleteBucketPolicyRequestWriter;
impl DeleteBucketPolicyRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketPolicyRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type CloudFunctionInvocationRole = String;

/// Parse CloudFunctionInvocationRole from response
struct CloudFunctionInvocationRoleParser;
impl CloudFunctionInvocationRoleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CloudFunctionInvocationRole, XmlParseError> {
		println!("in CloudFunctionInvocationRoleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CloudFunctionInvocationRole contents to a SignedRequest
struct CloudFunctionInvocationRoleWriter;
impl CloudFunctionInvocationRoleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CloudFunctionInvocationRole, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CloudFunctionInvocationRole TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct HeadObjectOutput {
	/// Last modified date of the object
	pub last_modified: LastModified,
	pub request_charged: RequestCharged,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: ContentEncoding,
	pub replication_status: ReplicationStatus,
	pub storage_class: StorageClass,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
	/// Specifies presentational information for the object.
	pub content_disposition: ContentDisposition,
	/// A map of metadata to store with the object in S3.
	pub metadata: Metadata,
	pub accept_ranges: AcceptRanges,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: WebsiteRedirectLocation,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Expires,
	/// Specifies whether the object retrieved was (true) or was not (false) a Delete
	/// Marker. If false, this response header does not appear in the response.
	pub delete_marker: DeleteMarker,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: CacheControl,
	/// Size of the body in bytes.
	pub content_length: ContentLength,
	/// If the object expiration is configured (see PUT Bucket lifecycle), the
	/// response includes this header. It includes the expiry-date and rule-id key
	/// value pairs providing object expiration information. The value of the rule-id
	/// is URL encoded.
	pub expiration: Expiration,
	/// This is set to the number of metadata entries not returned in x-amz-meta
	/// headers. This can happen if you create metadata using an API like SOAP that
	/// supports more flexible metadata than the REST API. For example, using SOAP,
	/// you can create metadata whose values are not legal HTTP headers.
	pub missing_meta: MissingMeta,
	/// Provides information about object restoration operation and expiration time of
	/// the restored object copy.
	pub restore: Restore,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	/// A standard MIME type describing the format of the object data.
	pub content_type: ContentType,
	/// The language the content is in.
	pub content_language: ContentLanguage,
	/// Version of the object.
	pub version_id: ObjectVersionId,
	/// An ETag is an opaque identifier assigned by a web server to a specific version
	/// of a resource found at a URL
	pub e_tag: ETag,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
}


/// Parse HeadObjectOutput from response
struct HeadObjectOutputParser;
impl HeadObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<HeadObjectOutput, XmlParseError> {
		println!("in HeadObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = HeadObjectOutput::default();
		//parser for cname of LastModified and child shape is LastModified
		obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentEncoding and child shape is ContentEncoding
		obj.content_encoding = try!(ContentEncodingParser::parse_response(Some(&"ContentEncoding".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ReplicationStatus and child shape is ReplicationStatus
		obj.replication_status = try!(ReplicationStatusParser::parse_response(Some(&"ReplicationStatus".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of StorageClass and child shape is StorageClass
		obj.storage_class = try!(StorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentDisposition and child shape is ContentDisposition
		obj.content_disposition = try!(ContentDispositionParser::parse_response(Some(&"ContentDisposition".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Metadata and child shape is Metadata
		obj.metadata = try!(MetadataParser::parse_response(Some(&"Metadata".to_string()), Some(&ArgumentLocation::Headers), headers, stack));
		//parser for cname of AcceptRanges and child shape is AcceptRanges
		obj.accept_ranges = try!(AcceptRangesParser::parse_response(Some(&"AcceptRanges".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of WebsiteRedirectLocation and child shape is WebsiteRedirectLocation
		obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_response(Some(&"WebsiteRedirectLocation".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expires and child shape is Expires
		obj.expires = try!(ExpiresParser::parse_response(Some(&"Expires".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of DeleteMarker and child shape is DeleteMarker
		obj.delete_marker = try!(DeleteMarkerParser::parse_response(Some(&"DeleteMarker".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of CacheControl and child shape is CacheControl
		obj.cache_control = try!(CacheControlParser::parse_response(Some(&"CacheControl".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentLength and child shape is ContentLength
		obj.content_length = try!(ContentLengthParser::parse_response(Some(&"ContentLength".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expiration and child shape is Expiration
		obj.expiration = try!(ExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of MissingMeta and child shape is MissingMeta
		obj.missing_meta = try!(MissingMetaParser::parse_response(Some(&"MissingMeta".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Restore and child shape is Restore
		obj.restore = try!(RestoreParser::parse_response(Some(&"Restore".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentType and child shape is ContentType
		obj.content_type = try!(ContentTypeParser::parse_response(Some(&"ContentType".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentLanguage and child shape is ContentLanguage
		obj.content_language = try!(ContentLanguageParser::parse_response(Some(&"ContentLanguage".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ETag and child shape is ETag
		obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketReplicationRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketReplicationRequest contents to a SignedRequest
struct DeleteBucketReplicationRequestWriter;
impl DeleteBucketReplicationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketReplicationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type PartNumber = i32;

/// Parse PartNumber from response
struct PartNumberParser;
impl PartNumberParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<PartNumber, XmlParseError> {
		println!("in PartNumberParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write PartNumber contents to a SignedRequest
struct PartNumberWriter;
impl PartNumberWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PartNumber, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT PartNumber TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ExposeHeaders = Vec<ExposeHeader>;

/// Parse ExposeHeaders from response
struct ExposeHeadersParser;
impl ExposeHeadersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ExposeHeaders, XmlParseError> {
		println!("in ExposeHeadersParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "ExposeHeader" {
//we need to iterate over members of ExposeHeader
// obj.push for ExposeHeader
			obj.push(try!(ExposeHeaderParser::parse_response(Some(&"ExposeHeader"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write ExposeHeaders contents to a SignedRequest
struct ExposeHeadersWriter;
impl ExposeHeadersWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ExposeHeaders, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for ExposeHeader
;
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLoggingOutput {
	pub logging_enabled: LoggingEnabled,
}


/// Parse GetBucketLoggingOutput from response
struct GetBucketLoggingOutputParser;
impl GetBucketLoggingOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketLoggingOutput, XmlParseError> {
		println!("in GetBucketLoggingOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketLoggingOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LoggingEnabled and child shape is LoggingEnabled
			if current_name == "LoggingEnabled"{
				obj.logging_enabled = try!(LoggingEnabledParser::parse_response(Some(&"LoggingEnabled".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct ListObjectsRequest {
	pub bucket: BucketName,
	/// Limits the response to keys that begin with the specified prefix.
	pub prefix: Option<Prefix>,
	/// Sets the maximum number of keys returned in the response. The response might
	/// contain fewer keys but will never contain more.
	pub max_keys: Option<MaxKeys>,
	/// A delimiter is a character you use to group keys.
	pub delimiter: Option<Delimiter>,
	/// Specifies the key to start with when listing objects in a bucket.
	pub marker: Option<Marker>,
	pub encoding_type: Option<EncodingType>,
}

/// Write ListObjectsRequest contents to a SignedRequest
struct ListObjectsRequestWriter;
impl ListObjectsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ListObjectsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for prefix
		if let Some(ref obj) = obj.prefix {
			PrefixWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"prefix".to_string());
		}
		// optional writing for max_keys
		if let Some(ref obj) = obj.max_keys {
			MaxKeysWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"max-keys".to_string());
		}
		// optional writing for delimiter
		if let Some(ref obj) = obj.delimiter {
			DelimiterWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"delimiter".to_string());
		}
		// optional writing for marker
		if let Some(ref obj) = obj.marker {
			MarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"marker".to_string());
		}
		// optional writing for encoding_type
		if let Some(ref obj) = obj.encoding_type {
			EncodingTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"encoding-type".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketReplicationOutput {
	pub replication_configuration: ReplicationConfiguration,
}


/// Parse GetBucketReplicationOutput from response
struct GetBucketReplicationOutputParser;
impl GetBucketReplicationOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketReplicationOutput, XmlParseError> {
		println!("in GetBucketReplicationOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketReplicationOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of ReplicationConfiguration and child shape is ReplicationConfiguration
			if current_name == "ReplicationConfiguration"{
				obj.replication_configuration = try!(ReplicationConfigurationParser::parse_response(Some(&"ReplicationConfiguration".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type Policy = String;

/// Parse Policy from response
struct PolicyParser;
impl PolicyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Policy, XmlParseError> {
		println!("in PolicyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Policy contents to a SignedRequest
struct PolicyWriter;
impl PolicyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Policy, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Policy TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ListMultipartUploadsOutput {
	/// Upload ID after which listing began.
	pub upload_id_marker: UploadIdMarker,
	pub common_prefixes: CommonPrefixList,
	/// When a list is truncated, this element specifies the value that should be used
	/// for the key-marker request parameter in a subsequent request.
	pub next_key_marker: NextKeyMarker,
	/// Name of the bucket to which the multipart upload was initiated.
	pub bucket: BucketName,
	pub delimiter: Delimiter,
	/// When a list is truncated, this element specifies the value that should be used
	/// for the upload-id-marker request parameter in a subsequent request.
	pub next_upload_id_marker: NextUploadIdMarker,
	/// When a prefix is provided in the request, this field contains the specified
	/// prefix. The result contains only keys starting with the specified prefix.
	pub prefix: Prefix,
	pub uploads: MultipartUploadList,
	/// The key at or after which the listing began.
	pub key_marker: KeyMarker,
	/// Maximum number of multipart uploads that could have been included in the
	/// response.
	pub max_uploads: MaxUploads,
	/// Encoding type used by Amazon S3 to encode object keys in the response.
	pub encoding_type: EncodingType,
	/// Indicates whether the returned list of multipart uploads is truncated. A value
	/// of true indicates that the list was truncated. The list can be truncated if
	/// the number of multipart uploads exceeds the limit allowed or specified by max
	/// uploads.
	pub is_truncated: IsTruncated,
}


/// Parse ListMultipartUploadsOutput from response
struct ListMultipartUploadsOutputParser;
impl ListMultipartUploadsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ListMultipartUploadsOutput, XmlParseError> {
		println!("in ListMultipartUploadsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ListMultipartUploadsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of UploadIdMarker and child shape is UploadIdMarker
			if current_name == "UploadIdMarker"{
				obj.upload_id_marker = try!(UploadIdMarkerParser::parse_response(Some(&"UploadIdMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of CommonPrefixes and child shape is CommonPrefixList
			if current_name == "CommonPrefixes"{
				obj.common_prefixes = try!(CommonPrefixListParser::parse_response(Some(&"CommonPrefixes".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextKeyMarker and child shape is NextKeyMarker
			if current_name == "NextKeyMarker"{
				obj.next_key_marker = try!(NextKeyMarkerParser::parse_response(Some(&"NextKeyMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Bucket and child shape is BucketName
			if current_name == "Bucket"{
				obj.bucket = try!(BucketNameParser::parse_response(Some(&"Bucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Delimiter and child shape is Delimiter
			if current_name == "Delimiter"{
				obj.delimiter = try!(DelimiterParser::parse_response(Some(&"Delimiter".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextUploadIdMarker and child shape is NextUploadIdMarker
			if current_name == "NextUploadIdMarker"{
				obj.next_upload_id_marker = try!(NextUploadIdMarkerParser::parse_response(Some(&"NextUploadIdMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Uploads and child shape is MultipartUploadList
			if current_name == "Uploads"{
				obj.uploads = try!(MultipartUploadListParser::parse_response(Some(&"Uploads".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of KeyMarker and child shape is KeyMarker
			if current_name == "KeyMarker"{
				obj.key_marker = try!(KeyMarkerParser::parse_response(Some(&"KeyMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MaxUploads and child shape is MaxUploads
			if current_name == "MaxUploads"{
				obj.max_uploads = try!(MaxUploadsParser::parse_response(Some(&"MaxUploads".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of EncodingType and child shape is EncodingType
			if current_name == "EncodingType"{
				obj.encoding_type = try!(EncodingTypeParser::parse_response(Some(&"EncodingType".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsTruncated and child shape is IsTruncated
			if current_name == "IsTruncated"{
				obj.is_truncated = try!(IsTruncatedParser::parse_response(Some(&"IsTruncated".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type IfUnmodifiedSince = String;

/// Parse IfUnmodifiedSince from response
struct IfUnmodifiedSinceParser;
impl IfUnmodifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IfUnmodifiedSince, XmlParseError> {
		println!("in IfUnmodifiedSinceParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IfUnmodifiedSince contents to a SignedRequest
struct IfUnmodifiedSinceWriter;
impl IfUnmodifiedSinceWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IfUnmodifiedSince, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IfUnmodifiedSince TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for Permission
#[derive(Debug,PartialEq)]
pub enum Permission {
	FULL_CONTROL,
	WRITE,
	WRITE_ACP,
	READ,
	READ_ACP,
}
impl fmt::Display for Permission {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Permission::FULL_CONTROL => write!(f, "FULL_CONTROL"),
			Permission::WRITE => write!(f, "WRITE"),
			Permission::WRITE_ACP => write!(f, "WRITE_ACP"),
			Permission::READ => write!(f, "READ"),
			Permission::READ_ACP => write!(f, "READ_ACP"),
		}
	}
}
impl Default for Permission{
	fn default() -> Permission{
		Permission::FULL_CONTROL
	}
}
impl From<String> for Permission{
	fn from(string: String) -> Permission{
		match string.as_ref() {
			"FULL_CONTROL" => Permission::FULL_CONTROL,
			"WRITE" => Permission::WRITE,
			"WRITE_ACP" => Permission::WRITE_ACP,
			"READ" => Permission::READ,
			"READ_ACP" => Permission::READ_ACP,
			_ => Permission::default(),
		}
	}
}

/// Parse Permission from response
struct PermissionParser;
impl PermissionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Permission, XmlParseError> {
		println!("in PermissionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Permission = Permission::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Permission::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = Permission::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Permission contents to a SignedRequest
struct PermissionWriter;
impl PermissionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Permission, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Permission TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Metadata = HashMap<MetadataKey,MetadataValue>;

/// Parse Metadata from response
struct MetadataParser;
impl MetadataParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Metadata, XmlParseError> {
		println!("in MetadataParser");
		 // map_parser
//lol punt
		let mut obj = HashMap::new();
		Ok(obj)
	}
}
/// Write Metadata contents to a SignedRequest
struct MetadataWriter;
impl MetadataWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Metadata, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: map_writer for MetadataKey : MetadataValue
;
		body
	}
}
pub type Grants = Vec<Grant>;

/// Parse Grants from response
struct GrantsParser;
impl GrantsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Grants, XmlParseError> {
		println!("in GrantsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Grant" {
//we need to iterate over members of Grant
// skip Grant.  It's a location name.
// obj.push for Grant
			obj.push(try!(GrantParser::parse_response(Some(&"Grant"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write Grants contents to a SignedRequest
struct GrantsWriter;
impl GrantsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Grants, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Grant
;
		body
	}
}
//NEEDS ENUM for ObjectStorageClass
#[derive(Debug,PartialEq)]
pub enum ObjectStorageClass {
	STANDARD,
	REDUCED_REDUNDANCY,
	GLACIER,
}
impl fmt::Display for ObjectStorageClass {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ObjectStorageClass::STANDARD => write!(f, "STANDARD"),
			ObjectStorageClass::REDUCED_REDUNDANCY => write!(f, "REDUCED_REDUNDANCY"),
			ObjectStorageClass::GLACIER => write!(f, "GLACIER"),
		}
	}
}
impl Default for ObjectStorageClass{
	fn default() -> ObjectStorageClass{
		ObjectStorageClass::STANDARD
	}
}
impl From<String> for ObjectStorageClass{
	fn from(string: String) -> ObjectStorageClass{
		match string.as_ref() {
			"STANDARD" => ObjectStorageClass::STANDARD,
			"REDUCED_REDUNDANCY" => ObjectStorageClass::REDUCED_REDUNDANCY,
			"GLACIER" => ObjectStorageClass::GLACIER,
			_ => ObjectStorageClass::default(),
		}
	}
}

/// Parse ObjectStorageClass from response
struct ObjectStorageClassParser;
impl ObjectStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectStorageClass, XmlParseError> {
		println!("in ObjectStorageClassParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ObjectStorageClass = ObjectStorageClass::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ObjectStorageClass::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ObjectStorageClass::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ObjectStorageClass contents to a SignedRequest
struct ObjectStorageClassWriter;
impl ObjectStorageClassWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectStorageClass, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ObjectStorageClass TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type EventList = Vec<Event>;

/// Parse EventList from response
struct EventListParser;
impl EventListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<EventList, XmlParseError> {
		println!("in EventListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Event" {
//we need to iterate over members of Event
// obj.push for Event
			obj.push(try!(EventParser::parse_response(Some(&"Event"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write EventList contents to a SignedRequest
struct EventListWriter;
impl EventListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a EventList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Event
;
		body
	}
}
pub type EmailAddress = String;

/// Parse EmailAddress from response
struct EmailAddressParser;
impl EmailAddressParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<EmailAddress, XmlParseError> {
		println!("in EmailAddressParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write EmailAddress contents to a SignedRequest
struct EmailAddressWriter;
impl EmailAddressWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a EmailAddress, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT EmailAddress TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CreateMultipartUploadOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	pub request_charged: RequestCharged,
	/// Name of the bucket to which the multipart upload was initiated.
	pub bucket: BucketName,
	/// ID for the initiated multipart upload.
	pub upload_id: MultipartUploadId,
	/// Object key for which the multipart upload was initiated.
	pub key: ObjectKey,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
}


/// Parse CreateMultipartUploadOutput from response
struct CreateMultipartUploadOutputParser;
impl CreateMultipartUploadOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CreateMultipartUploadOutput, XmlParseError> {
		println!("in CreateMultipartUploadOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CreateMultipartUploadOutput::default();
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Bucket and child shape is BucketName
			if current_name == "Bucket"{
				obj.bucket = try!(BucketNameParser::parse_response(Some(&"Bucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of UploadId and child shape is MultipartUploadId
			if current_name == "UploadId"{
				obj.upload_id = try!(MultipartUploadIdParser::parse_response(Some(&"UploadId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct PutBucketWebsiteRequest {
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	pub website_configuration: WebsiteConfiguration,
}

/// Write PutBucketWebsiteRequest contents to a SignedRequest
struct PutBucketWebsiteRequestWriter;
impl PutBucketWebsiteRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketWebsiteRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = WebsiteConfigurationWriter::write_params(request, &obj.website_configuration, Some(&ArgumentLocation::Body), &"WebsiteConfiguration".to_string());
		body
	}
}
pub type IsTruncated = bool;

/// Parse IsTruncated from response
struct IsTruncatedParser;
impl IsTruncatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IsTruncated, XmlParseError> {
		println!("in IsTruncatedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : bool = bool::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = match bool::from_str(&header_str) {
							Err(_) => false,
							Ok(newbool) => newbool,
						}
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IsTruncated contents to a SignedRequest
struct IsTruncatedWriter;
impl IsTruncatedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IsTruncated, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is boolean
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IsTruncated TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CreateBucketRequest {
	/// Allows grantee the read, write, read ACP, and write ACP permissions on the
	/// bucket.
	pub grant_full_control: Option<GrantFullControl>,
	pub create_bucket_configuration: Option<CreateBucketConfiguration>,
	/// Allows grantee to write the ACL for the applicable bucket.
	pub grant_write_acp: Option<GrantWriteACP>,
	pub bucket: BucketName,
	/// The canned ACL to apply to the bucket.
	pub acl: Option<BucketCannedACL>,
	/// Allows grantee to create, overwrite, and delete any object in the bucket.
	pub grant_write: Option<GrantWrite>,
	/// Allows grantee to list the objects in the bucket.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to read the bucket ACL.
	pub grant_read_acp: Option<GrantReadACP>,
}

/// Write CreateBucketRequest contents to a SignedRequest
struct CreateBucketRequestWriter;
impl CreateBucketRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CreateBucketRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for create_bucket_configuration
		if let Some(ref obj) = obj.create_bucket_configuration {
			body = CreateBucketConfigurationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"CreateBucketConfiguration".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			BucketCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
		// optional writing for grant_write
		if let Some(ref obj) = obj.grant_write {
			GrantWriteWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write".to_string());
		}
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		body
	}
}
pub type BucketName = String;

/// Parse BucketName from response
struct BucketNameParser;
impl BucketNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketName, XmlParseError> {
		println!("in BucketNameParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write BucketName contents to a SignedRequest
struct BucketNameWriter;
impl BucketNameWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketName, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT BucketName TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct TargetGrant {
	pub grantee: Grantee,
	/// Logging permissions assigned to the Grantee for the bucket.
	pub permission: BucketLogsPermission,
}


/// Parse TargetGrant from response
struct TargetGrantParser;
impl TargetGrantParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TargetGrant, XmlParseError> {
		println!("in TargetGrantParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = TargetGrant::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Grantee and child shape is Grantee
			if current_name == "Grantee"{
				obj.grantee = try!(GranteeParser::parse_response(Some(&"Grantee".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Permission and child shape is BucketLogsPermission
			if current_name == "Permission"{
				obj.permission = try!(BucketLogsPermissionParser::parse_response(Some(&"Permission".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write TargetGrant contents to a SignedRequest
struct TargetGrantWriter;
impl TargetGrantWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TargetGrant, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = GranteeWriter::write_params(request, &obj.grantee, Some(&ArgumentLocation::Body), &"Grantee".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketLogsPermissionWriter::write_params(request, &obj.permission, Some(&ArgumentLocation::Body), &"Permission".to_string());
		body
	}
}
pub type FilterRuleValue = String;

/// Parse FilterRuleValue from response
struct FilterRuleValueParser;
impl FilterRuleValueParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<FilterRuleValue, XmlParseError> {
		println!("in FilterRuleValueParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write FilterRuleValue contents to a SignedRequest
struct FilterRuleValueWriter;
impl FilterRuleValueWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a FilterRuleValue, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT FilterRuleValue TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketRequestPaymentRequest {
	pub request_payment_configuration: RequestPaymentConfiguration,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketRequestPaymentRequest contents to a SignedRequest
struct PutBucketRequestPaymentRequestWriter;
impl PutBucketRequestPaymentRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketRequestPaymentRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = RequestPaymentConfigurationWriter::write_params(request, &obj.request_payment_configuration, Some(&ArgumentLocation::Body), &"RequestPaymentConfiguration".to_string());
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct PutObjectAclRequest {
	/// Allows grantee the read, write, read ACP, and write ACP permissions on the
	/// bucket.
	pub grant_full_control: Option<GrantFullControl>,
	/// Allows grantee to write the ACL for the applicable bucket.
	pub grant_write_acp: Option<GrantWriteACP>,
	pub key: ObjectKey,
	pub request_payer: Option<RequestPayer>,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	/// The canned ACL to apply to the object.
	pub acl: Option<ObjectCannedACL>,
	pub access_control_policy: Option<AccessControlPolicy>,
	/// Allows grantee to create, overwrite, and delete any object in the bucket.
	pub grant_write: Option<GrantWrite>,
	/// Allows grantee to list the objects in the bucket.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to read the bucket ACL.
	pub grant_read_acp: Option<GrantReadACP>,
}

/// Write PutObjectAclRequest contents to a SignedRequest
struct PutObjectAclRequestWriter;
impl PutObjectAclRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutObjectAclRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			ObjectCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for access_control_policy
		if let Some(ref obj) = obj.access_control_policy {
			body = AccessControlPolicyWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"AccessControlPolicy".to_string());
		}
		// optional writing for grant_write
		if let Some(ref obj) = obj.grant_write {
			GrantWriteWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write".to_string());
		}
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CloudFunctionConfiguration {
	pub invocation_role: CloudFunctionInvocationRole,
	pub cloud_function: CloudFunction,
	pub events: EventList,
	pub id: NotificationId,
	pub event: Event,
}


/// Parse CloudFunctionConfiguration from response
struct CloudFunctionConfigurationParser;
impl CloudFunctionConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CloudFunctionConfiguration, XmlParseError> {
		println!("in CloudFunctionConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CloudFunctionConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of InvocationRole and child shape is CloudFunctionInvocationRole
			if current_name == "InvocationRole"{
				obj.invocation_role = try!(CloudFunctionInvocationRoleParser::parse_response(Some(&"InvocationRole".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of CloudFunction and child shape is CloudFunction
			if current_name == "CloudFunction"{
				obj.cloud_function = try!(CloudFunctionParser::parse_response(Some(&"CloudFunction".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Event and child shape is Event
			if current_name == "Event"{
				obj.event = try!(EventParser::parse_response(Some(&"Event".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CloudFunctionConfiguration contents to a SignedRequest
struct CloudFunctionConfigurationWriter;
impl CloudFunctionConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CloudFunctionConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = CloudFunctionInvocationRoleWriter::write_params(request, &obj.invocation_role, Some(&ArgumentLocation::Body), &"InvocationRole".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = CloudFunctionWriter::write_params(request, &obj.cloud_function, Some(&ArgumentLocation::Body), &"CloudFunction".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = NotificationIdWriter::write_params(request, &obj.id, Some(&ArgumentLocation::Body), &"Id".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventWriter::write_params(request, &obj.event, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
pub type LambdaFunctionArn = String;

/// Parse LambdaFunctionArn from response
struct LambdaFunctionArnParser;
impl LambdaFunctionArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LambdaFunctionArn, XmlParseError> {
		println!("in LambdaFunctionArnParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write LambdaFunctionArn contents to a SignedRequest
struct LambdaFunctionArnWriter;
impl LambdaFunctionArnWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LambdaFunctionArn, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT LambdaFunctionArn TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// Container for object key name filtering rules. For information about key name
/// filtering, go to [Configuring Event Notifications](http://docs.aws.amazon.com/
/// AmazonS3/latest/dev/NotificationHowTo.html) in the Amazon Simple Storage
/// Service Developer Guide.
#[derive(Debug, Default)]
pub struct NotificationConfigurationFilter {
	pub key: S3KeyFilter,
}


/// Parse NotificationConfigurationFilter from response
struct NotificationConfigurationFilterParser;
impl NotificationConfigurationFilterParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NotificationConfigurationFilter, XmlParseError> {
		println!("in NotificationConfigurationFilterParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NotificationConfigurationFilter::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Key and child shape is S3KeyFilter
			if current_name == "Key"{
				obj.key = try!(S3KeyFilterParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write NotificationConfigurationFilter contents to a SignedRequest
struct NotificationConfigurationFilterWriter;
impl NotificationConfigurationFilterWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NotificationConfigurationFilter, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = S3KeyFilterWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"S3Key".to_string());
		body
	}
}
pub type Quiet = bool;

/// Parse Quiet from response
struct QuietParser;
impl QuietParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Quiet, XmlParseError> {
		println!("in QuietParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : bool = bool::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = match bool::from_str(&header_str) {
							Err(_) => false,
							Ok(newbool) => newbool,
						}
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Quiet contents to a SignedRequest
struct QuietWriter;
impl QuietWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Quiet, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is boolean
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Quiet TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct AccessControlPolicy {
	pub owner: Owner,
	/// A list of grants.
	pub grants: Grants,
}


/// Parse AccessControlPolicy from response
struct AccessControlPolicyParser;
impl AccessControlPolicyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AccessControlPolicy, XmlParseError> {
		println!("in AccessControlPolicyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = AccessControlPolicy::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Grants and child shape is Grants
			if current_name == "Grants"{
				obj.grants = try!(GrantsParser::parse_response(Some(&"Grants".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write AccessControlPolicy contents to a SignedRequest
struct AccessControlPolicyWriter;
impl AccessControlPolicyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AccessControlPolicy, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = OwnerWriter::write_params(request, &obj.owner, Some(&ArgumentLocation::Body), &"Owner".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = GrantsWriter::write_params(request, &obj.grants, Some(&ArgumentLocation::Body), &"Grant".to_string());
		body
	}
}
pub type Range = String;

/// Parse Range from response
struct RangeParser;
impl RangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Range, XmlParseError> {
		println!("in RangeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Range contents to a SignedRequest
struct RangeWriter;
impl RangeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Range, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Range TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type GrantRead = String;

/// Parse GrantRead from response
struct GrantReadParser;
impl GrantReadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GrantRead, XmlParseError> {
		println!("in GrantReadParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write GrantRead contents to a SignedRequest
struct GrantReadWriter;
impl GrantReadWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GrantRead, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT GrantRead TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type SSECustomerKeyMD5 = String;

/// Parse SSECustomerKeyMD5 from response
struct SSECustomerKeyMD5Parser;
impl SSECustomerKeyMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<SSECustomerKeyMD5, XmlParseError> {
		println!("in SSECustomerKeyMD5Parser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write SSECustomerKeyMD5 contents to a SignedRequest
struct SSECustomerKeyMD5Writer;
impl SSECustomerKeyMD5Writer {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a SSECustomerKeyMD5, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT SSECustomerKeyMD5 TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

/// Parse ObjectIdentifierList from response
struct ObjectIdentifierListParser;
impl ObjectIdentifierListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectIdentifierList, XmlParseError> {
		println!("in ObjectIdentifierListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "ObjectIdentifier" {
//we need to iterate over members of ObjectIdentifier
// obj.push for ObjectIdentifier
			obj.push(try!(ObjectIdentifierParser::parse_response(Some(&"ObjectIdentifier"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write ObjectIdentifierList contents to a SignedRequest
struct ObjectIdentifierListWriter;
impl ObjectIdentifierListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectIdentifierList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for ObjectIdentifier
;
		body
	}
}
pub type Restore = String;

/// Parse Restore from response
struct RestoreParser;
impl RestoreParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Restore, XmlParseError> {
		println!("in RestoreParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Restore contents to a SignedRequest
struct RestoreWriter;
impl RestoreWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Restore, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Restore TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type NextVersionIdMarker = String;

/// Parse NextVersionIdMarker from response
struct NextVersionIdMarkerParser;
impl NextVersionIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NextVersionIdMarker, XmlParseError> {
		println!("in NextVersionIdMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NextVersionIdMarker contents to a SignedRequest
struct NextVersionIdMarkerWriter;
impl NextVersionIdMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NextVersionIdMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NextVersionIdMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Delete {
	pub objects: ObjectIdentifierList,
	/// Element to enable quiet mode for the request. When you add this element, you
	/// must set its value to true.
	pub quiet: Option<Quiet>,
}


/// Parse Delete from response
struct DeleteParser;
impl DeleteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Delete, XmlParseError> {
		println!("in DeleteParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Delete::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Objects and child shape is ObjectIdentifierList
			if current_name == "Objects"{
				obj.objects = try!(ObjectIdentifierListParser::parse_response(Some(&"Objects".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Quiet and child shape is Quiet
			if current_name == "Quiet"{
				obj.quiet = Some(try!(QuietParser::parse_response(Some(&"Quiet".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Delete contents to a SignedRequest
struct DeleteWriter;
impl DeleteWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Delete, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectIdentifierListWriter::write_params(request, &obj.objects, Some(&ArgumentLocation::Body), &"ObjectIdentifier".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for quiet
		if let Some(ref obj) = obj.quiet {
			body = QuietWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Quiet".to_string());
		}
		body
	}
}
pub type ResponseContentLanguage = String;

/// Parse ResponseContentLanguage from response
struct ResponseContentLanguageParser;
impl ResponseContentLanguageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseContentLanguage, XmlParseError> {
		println!("in ResponseContentLanguageParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseContentLanguage contents to a SignedRequest
struct ResponseContentLanguageWriter;
impl ResponseContentLanguageWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseContentLanguage, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseContentLanguage TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ListObjectsOutput {
	pub name: BucketName,
	/// When response is truncated (the IsTruncated element value in the response is
	/// true), you can use the key name in this field as marker in the subsequent
	/// request to get next set of objects. Amazon S3 lists objects in alphabetical
	/// order Note: This element is returned only if you have delimiter request
	/// parameter specified. If response does not include the NextMaker and it is
	/// truncated, you can use the value of the last Key in the response as the marker
	/// in the subsequent request to get the next set of object keys.
	pub next_marker: NextMarker,
	pub delimiter: Delimiter,
	pub max_keys: MaxKeys,
	pub prefix: Prefix,
	pub marker: Marker,
	/// Encoding type used by Amazon S3 to encode object keys in the response.
	pub encoding_type: EncodingType,
	/// A flag that indicates whether or not Amazon S3 returned all of the results
	/// that satisfied the search criteria.
	pub is_truncated: IsTruncated,
	pub contents: ObjectList,
	pub common_prefixes: CommonPrefixList,
}


/// Parse ListObjectsOutput from response
struct ListObjectsOutputParser;
impl ListObjectsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ListObjectsOutput, XmlParseError> {
		println!("in ListObjectsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ListObjectsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Name and child shape is BucketName
			if current_name == "Name"{
				obj.name = try!(BucketNameParser::parse_response(Some(&"Name".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextMarker and child shape is NextMarker
			if current_name == "NextMarker"{
				obj.next_marker = try!(NextMarkerParser::parse_response(Some(&"NextMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Delimiter and child shape is Delimiter
			if current_name == "Delimiter"{
				obj.delimiter = try!(DelimiterParser::parse_response(Some(&"Delimiter".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MaxKeys and child shape is MaxKeys
			if current_name == "MaxKeys"{
				obj.max_keys = try!(MaxKeysParser::parse_response(Some(&"MaxKeys".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Marker and child shape is Marker
			if current_name == "Marker"{
				obj.marker = try!(MarkerParser::parse_response(Some(&"Marker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of EncodingType and child shape is EncodingType
			if current_name == "EncodingType"{
				obj.encoding_type = try!(EncodingTypeParser::parse_response(Some(&"EncodingType".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsTruncated and child shape is IsTruncated
			if current_name == "IsTruncated"{
				obj.is_truncated = try!(IsTruncatedParser::parse_response(Some(&"IsTruncated".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Contents and child shape is ObjectList
			if current_name == "Contents"{
				obj.contents = try!(ObjectListParser::parse_response(Some(&"Contents".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of CommonPrefixes and child shape is CommonPrefixList
			if current_name == "CommonPrefixes"{
				obj.common_prefixes = try!(CommonPrefixListParser::parse_response(Some(&"CommonPrefixes".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type GrantWriteACP = String;

/// Parse GrantWriteACP from response
struct GrantWriteACPParser;
impl GrantWriteACPParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
		println!("in GrantWriteACPParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write GrantWriteACP contents to a SignedRequest
struct GrantWriteACPWriter;
impl GrantWriteACPWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GrantWriteACP, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT GrantWriteACP TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type CORSRules = Vec<CORSRule>;

/// Parse CORSRules from response
struct CORSRulesParser;
impl CORSRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CORSRules, XmlParseError> {
		println!("in CORSRulesParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "CORSRule" {
//we need to iterate over members of CORSRule
// obj.push for CORSRule
			obj.push(try!(CORSRuleParser::parse_response(Some(&"CORSRule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write CORSRules contents to a SignedRequest
struct CORSRulesWriter;
impl CORSRulesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CORSRules, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for CORSRule
;
		body
	}
}
pub type ContentLanguage = String;

/// Parse ContentLanguage from response
struct ContentLanguageParser;
impl ContentLanguageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentLanguage, XmlParseError> {
		println!("in ContentLanguageParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentLanguage contents to a SignedRequest
struct ContentLanguageWriter;
impl ContentLanguageWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentLanguage, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentLanguage TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ListBucketsOutput {
	pub owner: Owner,
	pub buckets: Buckets,
}


/// Parse ListBucketsOutput from response
struct ListBucketsOutputParser;
impl ListBucketsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ListBucketsOutput, XmlParseError> {
		println!("in ListBucketsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ListBucketsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Buckets and child shape is Buckets
			if current_name == "Buckets"{
				obj.buckets = try!(BucketsParser::parse_response(Some(&"Buckets".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct DeleteObjectRequest {
	/// The concatenation of the authentication device's serial number, a space, and
	/// the value that is displayed on your authentication device.
	pub mfa: Option<MFA>,
	/// VersionId used to reference a specific version of the object.
	pub version_id: Option<ObjectVersionId>,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write DeleteObjectRequest contents to a SignedRequest
struct DeleteObjectRequestWriter;
impl DeleteObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for mfa
		if let Some(ref obj) = obj.mfa {
			MFAWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-mfa".to_string());
		}
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"versionId".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
pub type CompletedPartList = Vec<CompletedPart>;

/// Parse CompletedPartList from response
struct CompletedPartListParser;
impl CompletedPartListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CompletedPartList, XmlParseError> {
		println!("in CompletedPartListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "CompletedPart" {
//we need to iterate over members of CompletedPart
// obj.push for CompletedPart
			obj.push(try!(CompletedPartParser::parse_response(Some(&"CompletedPart"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write CompletedPartList contents to a SignedRequest
struct CompletedPartListWriter;
impl CompletedPartListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CompletedPartList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for CompletedPart
;
		body
	}
}
#[derive(Debug, Default)]
pub struct DeletedObject {
	pub version_id: ObjectVersionId,
	pub delete_marker_version_id: DeleteMarkerVersionId,
	pub key: ObjectKey,
	pub delete_marker: DeleteMarker,
}


/// Parse DeletedObject from response
struct DeletedObjectParser;
impl DeletedObjectParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeletedObject, XmlParseError> {
		println!("in DeletedObjectParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = DeletedObject::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of VersionId and child shape is ObjectVersionId
			if current_name == "VersionId"{
				obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of DeleteMarkerVersionId and child shape is DeleteMarkerVersionId
			if current_name == "DeleteMarkerVersionId"{
				obj.delete_marker_version_id = try!(DeleteMarkerVersionIdParser::parse_response(Some(&"DeleteMarkerVersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of DeleteMarker and child shape is DeleteMarker
			if current_name == "DeleteMarker"{
				obj.delete_marker = try!(DeleteMarkerParser::parse_response(Some(&"DeleteMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write DeletedObject contents to a SignedRequest
struct DeletedObjectWriter;
impl DeletedObjectWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeletedObject, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectVersionIdWriter::write_params(request, &obj.version_id, Some(&ArgumentLocation::Body), &"VersionId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = DeleteMarkerVersionIdWriter::write_params(request, &obj.delete_marker_version_id, Some(&ArgumentLocation::Body), &"DeleteMarkerVersionId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = DeleteMarkerWriter::write_params(request, &obj.delete_marker, Some(&ArgumentLocation::Body), &"DeleteMarker".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CORSRule {
	/// Specifies which headers are allowed in a pre-flight OPTIONS request.
	pub allowed_headers: Option<AllowedHeaders>,
	/// One or more headers in the response that you want customers to be able to
	/// access from their applications (for example, from a JavaScript XMLHttpRequest
	/// object).
	pub expose_headers: Option<ExposeHeaders>,
	/// Identifies HTTP methods that the domain/origin specified in the rule is
	/// allowed to execute.
	pub allowed_methods: AllowedMethods,
	/// The time in seconds that your browser is to cache the preflight response for
	/// the specified resource.
	pub max_age_seconds: Option<MaxAgeSeconds>,
	/// One or more origins you want customers to be able to access the bucket from.
	pub allowed_origins: AllowedOrigins,
}


/// Parse CORSRule from response
struct CORSRuleParser;
impl CORSRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CORSRule, XmlParseError> {
		println!("in CORSRuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CORSRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of AllowedHeaders and child shape is AllowedHeaders
			if current_name == "AllowedHeaders"{
				obj.allowed_headers = Some(try!(AllowedHeadersParser::parse_response(Some(&"AllowedHeaders".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of ExposeHeaders and child shape is ExposeHeaders
			if current_name == "ExposeHeaders"{
				obj.expose_headers = Some(try!(ExposeHeadersParser::parse_response(Some(&"ExposeHeaders".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of AllowedMethods and child shape is AllowedMethods
			if current_name == "AllowedMethods"{
				obj.allowed_methods = try!(AllowedMethodsParser::parse_response(Some(&"AllowedMethods".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MaxAgeSeconds and child shape is MaxAgeSeconds
			if current_name == "MaxAgeSeconds"{
				obj.max_age_seconds = Some(try!(MaxAgeSecondsParser::parse_response(Some(&"MaxAgeSeconds".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of AllowedOrigins and child shape is AllowedOrigins
			if current_name == "AllowedOrigins"{
				obj.allowed_origins = try!(AllowedOriginsParser::parse_response(Some(&"AllowedOrigins".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CORSRule contents to a SignedRequest
struct CORSRuleWriter;
impl CORSRuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CORSRule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for allowed_headers
		if let Some(ref obj) = obj.allowed_headers {
			body = AllowedHeadersWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"AllowedHeader".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for expose_headers
		if let Some(ref obj) = obj.expose_headers {
			body = ExposeHeadersWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"ExposeHeader".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = AllowedMethodsWriter::write_params(request, &obj.allowed_methods, Some(&ArgumentLocation::Body), &"AllowedMethod".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for max_age_seconds
		if let Some(ref obj) = obj.max_age_seconds {
			body = MaxAgeSecondsWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"MaxAgeSeconds".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = AllowedOriginsWriter::write_params(request, &obj.allowed_origins, Some(&ArgumentLocation::Body), &"AllowedOrigin".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct LoggingEnabled {
	/// This element lets you specify a prefix for the keys that the log files will be
	/// stored under.
	pub target_prefix: TargetPrefix,
	/// Specifies the bucket where you want Amazon S3 to store server access logs. You
	/// can have your logs delivered to any bucket that you own, including the same
	/// bucket that is being logged. You can also configure multiple buckets to
	/// deliver their logs to the same target bucket. In this case you should choose a
	/// different TargetPrefix for each source bucket so that the delivered log files
	/// can be distinguished by key.
	pub target_bucket: TargetBucket,
	pub target_grants: TargetGrants,
}


/// Parse LoggingEnabled from response
struct LoggingEnabledParser;
impl LoggingEnabledParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LoggingEnabled, XmlParseError> {
		println!("in LoggingEnabledParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = LoggingEnabled::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of TargetPrefix and child shape is TargetPrefix
			if current_name == "TargetPrefix"{
				obj.target_prefix = try!(TargetPrefixParser::parse_response(Some(&"TargetPrefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of TargetBucket and child shape is TargetBucket
			if current_name == "TargetBucket"{
				obj.target_bucket = try!(TargetBucketParser::parse_response(Some(&"TargetBucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of TargetGrants and child shape is TargetGrants
			if current_name == "TargetGrants"{
				obj.target_grants = try!(TargetGrantsParser::parse_response(Some(&"TargetGrants".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write LoggingEnabled contents to a SignedRequest
struct LoggingEnabledWriter;
impl LoggingEnabledWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LoggingEnabled, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = TargetPrefixWriter::write_params(request, &obj.target_prefix, Some(&ArgumentLocation::Body), &"TargetPrefix".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TargetBucketWriter::write_params(request, &obj.target_bucket, Some(&ArgumentLocation::Body), &"TargetBucket".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TargetGrantsWriter::write_params(request, &obj.target_grants, Some(&ArgumentLocation::Body), &"Grant".to_string());
		body
	}
}
pub type KeyPrefixEquals = String;

/// Parse KeyPrefixEquals from response
struct KeyPrefixEqualsParser;
impl KeyPrefixEqualsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<KeyPrefixEquals, XmlParseError> {
		println!("in KeyPrefixEqualsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write KeyPrefixEquals contents to a SignedRequest
struct KeyPrefixEqualsWriter;
impl KeyPrefixEqualsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a KeyPrefixEquals, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT KeyPrefixEquals TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct RedirectAllRequestsTo {
	/// Name of the host where requests will be redirected.
	pub host_name: HostName,
	/// Protocol to use (http, https) when redirecting requests. The default is the
	/// protocol that is used in the original request.
	pub protocol: Option<Protocol>,
}


/// Parse RedirectAllRequestsTo from response
struct RedirectAllRequestsToParser;
impl RedirectAllRequestsToParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RedirectAllRequestsTo, XmlParseError> {
		println!("in RedirectAllRequestsToParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = RedirectAllRequestsTo::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of HostName and child shape is HostName
			if current_name == "HostName"{
				obj.host_name = try!(HostNameParser::parse_response(Some(&"HostName".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Protocol and child shape is Protocol
			if current_name == "Protocol"{
				obj.protocol = Some(try!(ProtocolParser::parse_response(Some(&"Protocol".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write RedirectAllRequestsTo contents to a SignedRequest
struct RedirectAllRequestsToWriter;
impl RedirectAllRequestsToWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RedirectAllRequestsTo, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = HostNameWriter::write_params(request, &obj.host_name, Some(&ArgumentLocation::Body), &"HostName".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for protocol
		if let Some(ref obj) = obj.protocol {
			body = ProtocolWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Protocol".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Owner {
	pub display_name: DisplayName,
	pub id: ID,
}


/// Parse Owner from response
struct OwnerParser;
impl OwnerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Owner, XmlParseError> {
		println!("in OwnerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Owner::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of DisplayName and child shape is DisplayName
			if current_name == "DisplayName"{
				obj.display_name = try!(DisplayNameParser::parse_response(Some(&"DisplayName".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Owner contents to a SignedRequest
struct OwnerWriter;
impl OwnerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Owner, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DisplayNameWriter::write_params(request, &obj.display_name, Some(&ArgumentLocation::Body), &"DisplayName".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = IDWriter::write_params(request, &obj.id, Some(&ArgumentLocation::Body), &"ID".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectResult {
	pub last_modified: LastModified,
	pub e_tag: ETag,
}


/// Parse CopyObjectResult from response
struct CopyObjectResultParser;
impl CopyObjectResultParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopyObjectResult, XmlParseError> {
		println!("in CopyObjectResultParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CopyObjectResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type S3ClientMessage = String;

/// Parse S3ClientMessage from response
struct S3ClientMessageParser;
impl S3ClientMessageParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<S3ClientMessage, XmlParseError> {
		println!("in S3ClientMessageParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write S3ClientMessage contents to a SignedRequest
struct S3ClientMessageWriter;
impl S3ClientMessageWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a S3ClientMessage, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT S3ClientMessage TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct LifecycleRule {
	/// If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is
	/// not currently being applied.
	pub status: ExpirationStatus,
	pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
	pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
	/// Prefix identifying one or more objects to which the rule applies.
	pub prefix: Prefix,
	pub expiration: Option<LifecycleExpiration>,
	pub transitions: Option<TransitionList>,
	/// Unique identifier for the rule. The value cannot be longer than 255
	/// characters.
	pub id: Option<ID>,
}


/// Parse LifecycleRule from response
struct LifecycleRuleParser;
impl LifecycleRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LifecycleRule, XmlParseError> {
		println!("in LifecycleRuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = LifecycleRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Status and child shape is ExpirationStatus
			if current_name == "Status"{
				obj.status = try!(ExpirationStatusParser::parse_response(Some(&"Status".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NoncurrentVersionExpiration and child shape is NoncurrentVersionExpiration
			if current_name == "NoncurrentVersionExpiration"{
				obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationParser::parse_response(Some(&"NoncurrentVersionExpiration".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of NoncurrentVersionTransitions and child shape is NoncurrentVersionTransitionList
			if current_name == "NoncurrentVersionTransitions"{
				obj.noncurrent_version_transitions = Some(try!(NoncurrentVersionTransitionListParser::parse_response(Some(&"NoncurrentVersionTransitions".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Expiration and child shape is LifecycleExpiration
			if current_name == "Expiration"{
				obj.expiration = Some(try!(LifecycleExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Transitions and child shape is TransitionList
			if current_name == "Transitions"{
				obj.transitions = Some(try!(TransitionListParser::parse_response(Some(&"Transitions".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = Some(try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write LifecycleRule contents to a SignedRequest
struct LifecycleRuleWriter;
impl LifecycleRuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LifecycleRule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ExpirationStatusWriter::write_params(request, &obj.status, Some(&ArgumentLocation::Body), &"Status".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for noncurrent_version_expiration
		if let Some(ref obj) = obj.noncurrent_version_expiration {
			body = NoncurrentVersionExpirationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"NoncurrentVersionExpiration".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for noncurrent_version_transitions
		if let Some(ref obj) = obj.noncurrent_version_transitions {
			body = NoncurrentVersionTransitionListWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"NoncurrentVersionTransition".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = PrefixWriter::write_params(request, &obj.prefix, Some(&ArgumentLocation::Body), &"Prefix".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for expiration
		if let Some(ref obj) = obj.expiration {
			body = LifecycleExpirationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Expiration".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for transitions
		if let Some(ref obj) = obj.transitions {
			body = TransitionListWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Transition".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = IDWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"ID".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleOutput {
	pub rules: Rules,
}


/// Parse GetBucketLifecycleOutput from response
struct GetBucketLifecycleOutputParser;
impl GetBucketLifecycleOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketLifecycleOutput, XmlParseError> {
		println!("in GetBucketLifecycleOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketLifecycleOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Rules and child shape is Rules
			if current_name == "Rules"{
				obj.rules = try!(RulesParser::parse_response(Some(&"Rules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Bucket event for which to send notifications.
//NEEDS ENUM for Event
#[derive(Debug,PartialEq)]
pub enum Event {
	s3_ReducedRedundancyLostObject,
	s3_ObjectCreated__star,
	s3_ObjectCreated_Put,
	s3_ObjectCreated_Post,
	s3_ObjectCreated_Copy,
	s3_ObjectCreated_CompleteMultipartUpload,
	s3_ObjectRemoved__star,
	s3_ObjectRemoved_Delete,
	s3_ObjectRemoved_DeleteMarkerCreated,
}
impl fmt::Display for Event {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Event::s3_ReducedRedundancyLostObject => write!(f, "s3:ReducedRedundancyLostObject"),
			Event::s3_ObjectCreated__star => write!(f, "s3:ObjectCreated:*"),
			Event::s3_ObjectCreated_Put => write!(f, "s3:ObjectCreated:Put"),
			Event::s3_ObjectCreated_Post => write!(f, "s3:ObjectCreated:Post"),
			Event::s3_ObjectCreated_Copy => write!(f, "s3:ObjectCreated:Copy"),
			Event::s3_ObjectCreated_CompleteMultipartUpload => write!(f, "s3:ObjectCreated:CompleteMultipartUpload"),
			Event::s3_ObjectRemoved__star => write!(f, "s3:ObjectRemoved:*"),
			Event::s3_ObjectRemoved_Delete => write!(f, "s3:ObjectRemoved:Delete"),
			Event::s3_ObjectRemoved_DeleteMarkerCreated => write!(f, "s3:ObjectRemoved:DeleteMarkerCreated"),
		}
	}
}
impl Default for Event{
	fn default() -> Event{
		Event::s3_ReducedRedundancyLostObject
	}
}
impl From<String> for Event{
	fn from(string: String) -> Event{
		match string.as_ref() {
			"s3:ReducedRedundancyLostObject" => Event::s3_ReducedRedundancyLostObject,
			"s3:ObjectCreated:*" => Event::s3_ObjectCreated__star,
			"s3:ObjectCreated:Put" => Event::s3_ObjectCreated_Put,
			"s3:ObjectCreated:Post" => Event::s3_ObjectCreated_Post,
			"s3:ObjectCreated:Copy" => Event::s3_ObjectCreated_Copy,
			"s3:ObjectCreated:CompleteMultipartUpload" => Event::s3_ObjectCreated_CompleteMultipartUpload,
			"s3:ObjectRemoved:*" => Event::s3_ObjectRemoved__star,
			"s3:ObjectRemoved:Delete" => Event::s3_ObjectRemoved_Delete,
			"s3:ObjectRemoved:DeleteMarkerCreated" => Event::s3_ObjectRemoved_DeleteMarkerCreated,
			_ => Event::default(),
		}
	}
}

/// Parse Event from response
struct EventParser;
impl EventParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Event, XmlParseError> {
		println!("in EventParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Event = Event::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Event::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = Event::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Event contents to a SignedRequest
struct EventWriter;
impl EventWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Event, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Event TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ReplicationRules = Vec<ReplicationRule>;

/// Parse ReplicationRules from response
struct ReplicationRulesParser;
impl ReplicationRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplicationRules, XmlParseError> {
		println!("in ReplicationRulesParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "ReplicationRule" {
//we need to iterate over members of ReplicationRule
// obj.push for ReplicationRule
			obj.push(try!(ReplicationRuleParser::parse_response(Some(&"ReplicationRule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write ReplicationRules contents to a SignedRequest
struct ReplicationRulesWriter;
impl ReplicationRulesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplicationRules, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for ReplicationRule
;
		body
	}
}
/// Container for specifying the notification configuration of the bucket. If this
/// element is empty, notifications are turned off on the bucket.
#[derive(Debug, Default)]
pub struct NotificationConfiguration {
	pub queue_configurations: QueueConfigurationList,
	pub lambda_function_configurations: LambdaFunctionConfigurationList,
	pub topic_configurations: TopicConfigurationList,
}


/// Parse NotificationConfiguration from response
struct NotificationConfigurationParser;
impl NotificationConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NotificationConfiguration, XmlParseError> {
		println!("in NotificationConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NotificationConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of QueueConfigurations and child shape is QueueConfigurationList
			if current_name == "QueueConfigurations"{
				obj.queue_configurations = try!(QueueConfigurationListParser::parse_response(Some(&"QueueConfigurations".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of LambdaFunctionConfigurations and child shape is LambdaFunctionConfigurationList
			if current_name == "LambdaFunctionConfigurations"{
				obj.lambda_function_configurations = try!(LambdaFunctionConfigurationListParser::parse_response(Some(&"LambdaFunctionConfigurations".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of TopicConfigurations and child shape is TopicConfigurationList
			if current_name == "TopicConfigurations"{
				obj.topic_configurations = try!(TopicConfigurationListParser::parse_response(Some(&"TopicConfigurations".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write NotificationConfiguration contents to a SignedRequest
struct NotificationConfigurationWriter;
impl NotificationConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NotificationConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = QueueConfigurationListWriter::write_params(request, &obj.queue_configurations, Some(&ArgumentLocation::Body), &"QueueConfiguration".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = LambdaFunctionConfigurationListWriter::write_params(request, &obj.lambda_function_configurations, Some(&ArgumentLocation::Body), &"LambdaFunctionConfiguration".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TopicConfigurationListWriter::write_params(request, &obj.topic_configurations, Some(&ArgumentLocation::Body), &"TopicConfiguration".to_string());
		body
	}
}
pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;

/// Parse NoncurrentVersionTransitionList from response
struct NoncurrentVersionTransitionListParser;
impl NoncurrentVersionTransitionListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoncurrentVersionTransitionList, XmlParseError> {
		println!("in NoncurrentVersionTransitionListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "NoncurrentVersionTransition" {
//we need to iterate over members of NoncurrentVersionTransition
// obj.push for NoncurrentVersionTransition
			obj.push(try!(NoncurrentVersionTransitionParser::parse_response(Some(&"NoncurrentVersionTransition"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write NoncurrentVersionTransitionList contents to a SignedRequest
struct NoncurrentVersionTransitionListWriter;
impl NoncurrentVersionTransitionListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoncurrentVersionTransitionList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for NoncurrentVersionTransition
;
		body
	}
}
#[derive(Debug, Default)]
pub struct Object {
	pub last_modified: LastModified,
	pub e_tag: ETag,
	/// The class of storage used to store the object.
	pub storage_class: ObjectStorageClass,
	pub key: ObjectKey,
	pub owner: Owner,
	pub size: Size,
}


/// Parse Object from response
struct ObjectParser;
impl ObjectParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Object, XmlParseError> {
		println!("in ObjectParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Object::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is ObjectStorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(ObjectStorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Size and child shape is Size
			if current_name == "Size"{
				obj.size = try!(SizeParser::parse_response(Some(&"Size".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Object contents to a SignedRequest
struct ObjectWriter;
impl ObjectWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Object, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = LastModifiedWriter::write_params(request, &obj.last_modified, Some(&ArgumentLocation::Body), &"LastModified".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ETagWriter::write_params(request, &obj.e_tag, Some(&ArgumentLocation::Body), &"ETag".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectStorageClassWriter::write_params(request, &obj.storage_class, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = OwnerWriter::write_params(request, &obj.owner, Some(&ArgumentLocation::Body), &"Owner".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = SizeWriter::write_params(request, &obj.size, Some(&ArgumentLocation::Body), &"Size".to_string());
		body
	}
}
pub type TransitionList = Vec<Transition>;

/// Parse TransitionList from response
struct TransitionListParser;
impl TransitionListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TransitionList, XmlParseError> {
		println!("in TransitionListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Transition" {
//we need to iterate over members of Transition
// obj.push for Transition
			obj.push(try!(TransitionParser::parse_response(Some(&"Transition"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write TransitionList contents to a SignedRequest
struct TransitionListWriter;
impl TransitionListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TransitionList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Transition
;
		body
	}
}
pub type NextMarker = String;

/// Parse NextMarker from response
struct NextMarkerParser;
impl NextMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NextMarker, XmlParseError> {
		println!("in NextMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NextMarker contents to a SignedRequest
struct NextMarkerWriter;
impl NextMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NextMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NextMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for MetadataDirective
#[derive(Debug,PartialEq)]
pub enum MetadataDirective {
	COPY,
	REPLACE,
}
impl fmt::Display for MetadataDirective {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MetadataDirective::COPY => write!(f, "COPY"),
			MetadataDirective::REPLACE => write!(f, "REPLACE"),
		}
	}
}
impl Default for MetadataDirective{
	fn default() -> MetadataDirective{
		MetadataDirective::COPY
	}
}
impl From<String> for MetadataDirective{
	fn from(string: String) -> MetadataDirective{
		match string.as_ref() {
			"COPY" => MetadataDirective::COPY,
			"REPLACE" => MetadataDirective::REPLACE,
			_ => MetadataDirective::default(),
		}
	}
}

/// Parse MetadataDirective from response
struct MetadataDirectiveParser;
impl MetadataDirectiveParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MetadataDirective, XmlParseError> {
		println!("in MetadataDirectiveParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : MetadataDirective = MetadataDirective::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = MetadataDirective::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = MetadataDirective::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MetadataDirective contents to a SignedRequest
struct MetadataDirectiveWriter;
impl MetadataDirectiveWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MetadataDirective, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MetadataDirective TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ContentEncoding = String;

/// Parse ContentEncoding from response
struct ContentEncodingParser;
impl ContentEncodingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentEncoding, XmlParseError> {
		println!("in ContentEncodingParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentEncoding contents to a SignedRequest
struct ContentEncodingWriter;
impl ContentEncodingWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentEncoding, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentEncoding TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct WebsiteConfiguration {
	pub redirect_all_requests_to: RedirectAllRequestsTo,
	pub index_document: IndexDocument,
	pub error_document: ErrorDocument,
	pub routing_rules: RoutingRules,
}


/// Parse WebsiteConfiguration from response
struct WebsiteConfigurationParser;
impl WebsiteConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<WebsiteConfiguration, XmlParseError> {
		println!("in WebsiteConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = WebsiteConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of RedirectAllRequestsTo and child shape is RedirectAllRequestsTo
			if current_name == "RedirectAllRequestsTo"{
				obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_response(Some(&"RedirectAllRequestsTo".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IndexDocument and child shape is IndexDocument
			if current_name == "IndexDocument"{
				obj.index_document = try!(IndexDocumentParser::parse_response(Some(&"IndexDocument".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ErrorDocument and child shape is ErrorDocument
			if current_name == "ErrorDocument"{
				obj.error_document = try!(ErrorDocumentParser::parse_response(Some(&"ErrorDocument".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of RoutingRules and child shape is RoutingRules
			if current_name == "RoutingRules"{
				obj.routing_rules = try!(RoutingRulesParser::parse_response(Some(&"RoutingRules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write WebsiteConfiguration contents to a SignedRequest
struct WebsiteConfigurationWriter;
impl WebsiteConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a WebsiteConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = RedirectAllRequestsToWriter::write_params(request, &obj.redirect_all_requests_to, Some(&ArgumentLocation::Body), &"RedirectAllRequestsTo".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = IndexDocumentWriter::write_params(request, &obj.index_document, Some(&ArgumentLocation::Body), &"IndexDocument".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ErrorDocumentWriter::write_params(request, &obj.error_document, Some(&ArgumentLocation::Body), &"ErrorDocument".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = RoutingRulesWriter::write_params(request, &obj.routing_rules, Some(&ArgumentLocation::Body), &"RoutingRule".to_string());
		body
	}
}
//NEEDS ENUM for MFADelete
#[derive(Debug,PartialEq)]
pub enum MFADelete {
	Enabled,
	Disabled,
}
impl fmt::Display for MFADelete {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MFADelete::Enabled => write!(f, "Enabled"),
			MFADelete::Disabled => write!(f, "Disabled"),
		}
	}
}
impl Default for MFADelete{
	fn default() -> MFADelete{
		MFADelete::Enabled
	}
}
impl From<String> for MFADelete{
	fn from(string: String) -> MFADelete{
		match string.as_ref() {
			"Enabled" => MFADelete::Enabled,
			"Disabled" => MFADelete::Disabled,
			_ => MFADelete::default(),
		}
	}
}

/// Parse MFADelete from response
struct MFADeleteParser;
impl MFADeleteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MFADelete, XmlParseError> {
		println!("in MFADeleteParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : MFADelete = MFADelete::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = MFADelete::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = MFADelete::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MFADelete contents to a SignedRequest
struct MFADeleteWriter;
impl MFADeleteWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MFADelete, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MFADelete TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type CopySourceSSECustomerKey = String;

/// Parse CopySourceSSECustomerKey from response
struct CopySourceSSECustomerKeyParser;
impl CopySourceSSECustomerKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceSSECustomerKey, XmlParseError> {
		println!("in CopySourceSSECustomerKeyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceSSECustomerKey contents to a SignedRequest
struct CopySourceSSECustomerKeyWriter;
impl CopySourceSSECustomerKeyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceSSECustomerKey, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceSSECustomerKey TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ResponseContentType = String;

/// Parse ResponseContentType from response
struct ResponseContentTypeParser;
impl ResponseContentTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseContentType, XmlParseError> {
		println!("in ResponseContentTypeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseContentType contents to a SignedRequest
struct ResponseContentTypeWriter;
impl ResponseContentTypeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseContentType, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseContentType TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CompleteMultipartUploadOutput {
	pub request_charged: RequestCharged,
	pub bucket: BucketName,
	/// Version of the object.
	pub version_id: ObjectVersionId,
	/// Entity tag of the object.
	pub e_tag: ETag,
	pub location: Location,
	pub key: ObjectKey,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
	/// If the object expiration is configured, this will contain the expiration date
	/// (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.
	pub expiration: Expiration,
}


/// Parse CompleteMultipartUploadOutput from response
struct CompleteMultipartUploadOutputParser;
impl CompleteMultipartUploadOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CompleteMultipartUploadOutput, XmlParseError> {
		println!("in CompleteMultipartUploadOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CompleteMultipartUploadOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expiration and child shape is Expiration
		obj.expiration = try!(ExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Bucket and child shape is BucketName
			if current_name == "Bucket"{
				obj.bucket = try!(BucketNameParser::parse_response(Some(&"Bucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Location and child shape is Location
			if current_name == "Location"{
				obj.location = try!(LocationParser::parse_response(Some(&"Location".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type ExposeHeader = String;

/// Parse ExposeHeader from response
struct ExposeHeaderParser;
impl ExposeHeaderParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ExposeHeader, XmlParseError> {
		println!("in ExposeHeaderParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ExposeHeader contents to a SignedRequest
struct ExposeHeaderWriter;
impl ExposeHeaderWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ExposeHeader, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ExposeHeader TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CopyPartResult {
	/// Date and time at which the object was uploaded.
	pub last_modified: LastModified,
	/// Entity tag of the object.
	pub e_tag: ETag,
}


/// Parse CopyPartResult from response
struct CopyPartResultParser;
impl CopyPartResultParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopyPartResult, XmlParseError> {
		println!("in CopyPartResultParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CopyPartResult::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct GetBucketAclRequest {
	pub bucket: BucketName,
}

/// Write GetBucketAclRequest contents to a SignedRequest
struct GetBucketAclRequestWriter;
impl GetBucketAclRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketAclRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type HostName = String;

/// Parse HostName from response
struct HostNameParser;
impl HostNameParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<HostName, XmlParseError> {
		println!("in HostNameParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write HostName contents to a SignedRequest
struct HostNameWriter;
impl HostNameWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a HostName, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT HostName TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type NextUploadIdMarker = String;

/// Parse NextUploadIdMarker from response
struct NextUploadIdMarkerParser;
impl NextUploadIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NextUploadIdMarker, XmlParseError> {
		println!("in NextUploadIdMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NextUploadIdMarker contents to a SignedRequest
struct NextUploadIdMarkerWriter;
impl NextUploadIdMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NextUploadIdMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NextUploadIdMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	pub copy_source_version_id: CopySourceVersionId,
	pub request_charged: RequestCharged,
	/// Version ID of the newly created copy.
	pub version_id: ObjectVersionId,
	/// If the object expiration is configured, the response includes this header.
	pub expiration: Expiration,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	pub copy_object_result: CopyObjectResult,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
}


/// Parse CopyObjectOutput from response
struct CopyObjectOutputParser;
impl CopyObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopyObjectOutput, XmlParseError> {
		println!("in CopyObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CopyObjectOutput::default();
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of CopySourceVersionId and child shape is CopySourceVersionId
		obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_response(Some(&"CopySourceVersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expiration and child shape is Expiration
		obj.expiration = try!(ExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CopyObjectResult and child shape is CopyObjectResult
			if current_name == "CopyObjectResult"{
				obj.copy_object_result = try!(CopyObjectResultParser::parse_response(Some(&"CopyObjectResult".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type AcceptRanges = String;

/// Parse AcceptRanges from response
struct AcceptRangesParser;
impl AcceptRangesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AcceptRanges, XmlParseError> {
		println!("in AcceptRangesParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write AcceptRanges contents to a SignedRequest
struct AcceptRangesWriter;
impl AcceptRangesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AcceptRanges, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT AcceptRanges TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// This operation is not allowed against this storage tier
#[derive(Debug, Default)]
pub struct ObjectAlreadyInActiveTierError;


/// Parse ObjectAlreadyInActiveTierError from response
struct ObjectAlreadyInActiveTierErrorParser;
impl ObjectAlreadyInActiveTierErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectAlreadyInActiveTierError, XmlParseError> {
		println!("in ObjectAlreadyInActiveTierErrorParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ObjectAlreadyInActiveTierError::default();
		Ok(obj)
	}
}
/// Write ObjectAlreadyInActiveTierError contents to a SignedRequest
struct ObjectAlreadyInActiveTierErrorWriter;
impl ObjectAlreadyInActiveTierErrorWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectAlreadyInActiveTierError, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
#[derive(Debug, Default)]
pub struct CompletedMultipartUpload {
	pub parts: CompletedPartList,
}


/// Parse CompletedMultipartUpload from response
struct CompletedMultipartUploadParser;
impl CompletedMultipartUploadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CompletedMultipartUpload, XmlParseError> {
		println!("in CompletedMultipartUploadParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CompletedMultipartUpload::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Parts and child shape is CompletedPartList
			if current_name == "Parts"{
				obj.parts = try!(CompletedPartListParser::parse_response(Some(&"Parts".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CompletedMultipartUpload contents to a SignedRequest
struct CompletedMultipartUploadWriter;
impl CompletedMultipartUploadWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CompletedMultipartUpload, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = CompletedPartListWriter::write_params(request, &obj.parts, Some(&ArgumentLocation::Body), &"CompletedPart".to_string());
		body
	}
}
pub type Initiated = String;

/// Parse Initiated from response
struct InitiatedParser;
impl InitiatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Initiated, XmlParseError> {
		println!("in InitiatedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Initiated contents to a SignedRequest
struct InitiatedWriter;
impl InitiatedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Initiated, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Initiated TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketAclRequest {
	/// Allows grantee the read, write, read ACP, and write ACP permissions on the
	/// bucket.
	pub grant_full_control: Option<GrantFullControl>,
	/// Allows grantee to write the ACL for the applicable bucket.
	pub grant_write_acp: Option<GrantWriteACP>,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	/// The canned ACL to apply to the bucket.
	pub acl: Option<BucketCannedACL>,
	pub access_control_policy: Option<AccessControlPolicy>,
	/// Allows grantee to create, overwrite, and delete any object in the bucket.
	pub grant_write: Option<GrantWrite>,
	/// Allows grantee to list the objects in the bucket.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to read the bucket ACL.
	pub grant_read_acp: Option<GrantReadACP>,
}

/// Write PutBucketAclRequest contents to a SignedRequest
struct PutBucketAclRequestWriter;
impl PutBucketAclRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketAclRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			BucketCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for access_control_policy
		if let Some(ref obj) = obj.access_control_policy {
			body = AccessControlPolicyWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"AccessControlPolicy".to_string());
		}
		// optional writing for grant_write
		if let Some(ref obj) = obj.grant_write {
			GrantWriteWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write".to_string());
		}
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct UploadPartOutput {
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// Entity tag for the uploaded object.
	pub e_tag: ETag,
	pub request_charged: RequestCharged,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
}


/// Parse UploadPartOutput from response
struct UploadPartOutputParser;
impl UploadPartOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<UploadPartOutput, XmlParseError> {
		println!("in UploadPartOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = UploadPartOutput::default();
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ETag and child shape is ETag
		obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
pub type CopySource = String;

/// Parse CopySource from response
struct CopySourceParser;
impl CopySourceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySource, XmlParseError> {
		println!("in CopySourceParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySource contents to a SignedRequest
struct CopySourceWriter;
impl CopySourceWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySource, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySource TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type QueueConfigurationList = Vec<QueueConfiguration>;

/// Parse QueueConfigurationList from response
struct QueueConfigurationListParser;
impl QueueConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<QueueConfigurationList, XmlParseError> {
		println!("in QueueConfigurationListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "QueueConfiguration" {
//we need to iterate over members of QueueConfiguration
// obj.push for QueueConfiguration
			obj.push(try!(QueueConfigurationParser::parse_response(Some(&"QueueConfiguration"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write QueueConfigurationList contents to a SignedRequest
struct QueueConfigurationListWriter;
impl QueueConfigurationListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a QueueConfigurationList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for QueueConfiguration
;
		body
	}
}
/// The source object of the COPY operation is not in the active tier and is only
/// stored in Amazon Glacier.
#[derive(Debug, Default)]
pub struct ObjectNotInActiveTierError;


/// Parse ObjectNotInActiveTierError from response
struct ObjectNotInActiveTierErrorParser;
impl ObjectNotInActiveTierErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectNotInActiveTierError, XmlParseError> {
		println!("in ObjectNotInActiveTierErrorParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ObjectNotInActiveTierError::default();
		Ok(obj)
	}
}
/// Write ObjectNotInActiveTierError contents to a SignedRequest
struct ObjectNotInActiveTierErrorWriter;
impl ObjectNotInActiveTierErrorWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectNotInActiveTierError, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
//NEEDS ENUM for TransitionStorageClass
#[derive(Debug,PartialEq)]
pub enum TransitionStorageClass {
	GLACIER,
	STANDARD_IA,
}
impl fmt::Display for TransitionStorageClass {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			TransitionStorageClass::GLACIER => write!(f, "GLACIER"),
			TransitionStorageClass::STANDARD_IA => write!(f, "STANDARD_IA"),
		}
	}
}
impl Default for TransitionStorageClass{
	fn default() -> TransitionStorageClass{
		TransitionStorageClass::GLACIER
	}
}
impl From<String> for TransitionStorageClass{
	fn from(string: String) -> TransitionStorageClass{
		match string.as_ref() {
			"GLACIER" => TransitionStorageClass::GLACIER,
			"STANDARD_IA" => TransitionStorageClass::STANDARD_IA,
			_ => TransitionStorageClass::default(),
		}
	}
}

/// Parse TransitionStorageClass from response
struct TransitionStorageClassParser;
impl TransitionStorageClassParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TransitionStorageClass, XmlParseError> {
		println!("in TransitionStorageClassParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : TransitionStorageClass = TransitionStorageClass::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = TransitionStorageClass::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = TransitionStorageClass::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write TransitionStorageClass contents to a SignedRequest
struct TransitionStorageClassWriter;
impl TransitionStorageClassWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TransitionStorageClass, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT TransitionStorageClass TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type DeleteMarker = bool;

/// Parse DeleteMarker from response
struct DeleteMarkerParser;
impl DeleteMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteMarker, XmlParseError> {
		println!("in DeleteMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : bool = bool::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = match bool::from_str(&header_str) {
							Err(_) => false,
							Ok(newbool) => newbool,
						}
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write DeleteMarker contents to a SignedRequest
struct DeleteMarkerWriter;
impl DeleteMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is boolean
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT DeleteMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Rule {
	/// If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is
	/// not currently being applied.
	pub status: ExpirationStatus,
	pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
	pub transition: Option<Transition>,
	/// Prefix identifying one or more objects to which the rule applies.
	pub prefix: Prefix,
	pub expiration: Option<LifecycleExpiration>,
	pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
	/// Unique identifier for the rule. The value cannot be longer than 255
	/// characters.
	pub id: Option<ID>,
}


/// Parse Rule from response
struct RuleParser;
impl RuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Rule, XmlParseError> {
		println!("in RuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Rule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Status and child shape is ExpirationStatus
			if current_name == "Status"{
				obj.status = try!(ExpirationStatusParser::parse_response(Some(&"Status".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NoncurrentVersionExpiration and child shape is NoncurrentVersionExpiration
			if current_name == "NoncurrentVersionExpiration"{
				obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationParser::parse_response(Some(&"NoncurrentVersionExpiration".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Transition and child shape is Transition
			if current_name == "Transition"{
				obj.transition = Some(try!(TransitionParser::parse_response(Some(&"Transition".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Expiration and child shape is LifecycleExpiration
			if current_name == "Expiration"{
				obj.expiration = Some(try!(LifecycleExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of NoncurrentVersionTransition and child shape is NoncurrentVersionTransition
			if current_name == "NoncurrentVersionTransition"{
				obj.noncurrent_version_transition = Some(try!(NoncurrentVersionTransitionParser::parse_response(Some(&"NoncurrentVersionTransition".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = Some(try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Rule contents to a SignedRequest
struct RuleWriter;
impl RuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Rule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ExpirationStatusWriter::write_params(request, &obj.status, Some(&ArgumentLocation::Body), &"Status".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for noncurrent_version_expiration
		if let Some(ref obj) = obj.noncurrent_version_expiration {
			body = NoncurrentVersionExpirationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"NoncurrentVersionExpiration".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for transition
		if let Some(ref obj) = obj.transition {
			body = TransitionWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Transition".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = PrefixWriter::write_params(request, &obj.prefix, Some(&ArgumentLocation::Body), &"Prefix".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for expiration
		if let Some(ref obj) = obj.expiration {
			body = LifecycleExpirationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Expiration".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for noncurrent_version_transition
		if let Some(ref obj) = obj.noncurrent_version_transition {
			body = NoncurrentVersionTransitionWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"NoncurrentVersionTransition".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = IDWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"ID".to_string());
		}
		body
	}
}
pub type RoutingRules = Vec<RoutingRule>;

/// Parse RoutingRules from response
struct RoutingRulesParser;
impl RoutingRulesParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RoutingRules, XmlParseError> {
		println!("in RoutingRulesParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "RoutingRule" {
//we need to iterate over members of RoutingRule
// skip RoutingRule.  It's a location name.
// obj.push for RoutingRule
			obj.push(try!(RoutingRuleParser::parse_response(Some(&"RoutingRule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write RoutingRules contents to a SignedRequest
struct RoutingRulesWriter;
impl RoutingRulesWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RoutingRules, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for RoutingRule
;
		body
	}
}
#[derive(Debug, Default)]
pub struct ReplicationRule {
	/// The rule is ignored if status is not Enabled.
	pub status: ReplicationRuleStatus,
	/// Object keyname prefix identifying one or more objects to which the rule
	/// applies. Maximum prefix length can be up to 1,024 characters. Overlapping
	/// prefixes are not supported.
	pub prefix: Prefix,
	pub destination: Destination,
	/// Unique identifier for the rule. The value cannot be longer than 255
	/// characters.
	pub id: Option<ID>,
}


/// Parse ReplicationRule from response
struct ReplicationRuleParser;
impl ReplicationRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplicationRule, XmlParseError> {
		println!("in ReplicationRuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ReplicationRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Status and child shape is ReplicationRuleStatus
			if current_name == "Status"{
				obj.status = try!(ReplicationRuleStatusParser::parse_response(Some(&"Status".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Destination and child shape is Destination
			if current_name == "Destination"{
				obj.destination = try!(DestinationParser::parse_response(Some(&"Destination".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = Some(try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write ReplicationRule contents to a SignedRequest
struct ReplicationRuleWriter;
impl ReplicationRuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplicationRule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ReplicationRuleStatusWriter::write_params(request, &obj.status, Some(&ArgumentLocation::Body), &"Status".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = PrefixWriter::write_params(request, &obj.prefix, Some(&ArgumentLocation::Body), &"Prefix".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = DestinationWriter::write_params(request, &obj.destination, Some(&ArgumentLocation::Body), &"Destination".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = IDWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"ID".to_string());
		}
		body
	}
}
pub type Date = String;

/// Parse Date from response
struct DateParser;
impl DateParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Date, XmlParseError> {
		println!("in DateParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Date contents to a SignedRequest
struct DateWriter;
impl DateWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Date, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Date TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type CacheControl = String;

/// Parse CacheControl from response
struct CacheControlParser;
impl CacheControlParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CacheControl, XmlParseError> {
		println!("in CacheControlParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CacheControl contents to a SignedRequest
struct CacheControlWriter;
impl CacheControlWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CacheControl, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CacheControl TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type AllowedOrigin = String;

/// Parse AllowedOrigin from response
struct AllowedOriginParser;
impl AllowedOriginParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedOrigin, XmlParseError> {
		println!("in AllowedOriginParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write AllowedOrigin contents to a SignedRequest
struct AllowedOriginWriter;
impl AllowedOriginWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedOrigin, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT AllowedOrigin TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type IfModifiedSince = String;

/// Parse IfModifiedSince from response
struct IfModifiedSinceParser;
impl IfModifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IfModifiedSince, XmlParseError> {
		println!("in IfModifiedSinceParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IfModifiedSince contents to a SignedRequest
struct IfModifiedSinceWriter;
impl IfModifiedSinceWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IfModifiedSince, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IfModifiedSince TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Condition {
	/// The HTTP error code when the redirect is applied. In the event of an error, if
	/// the error code equals this value, then the specified redirect is applied.
	/// Required when parent element Condition is specified and sibling
	/// KeyPrefixEquals is not specified. If both are specified, then both must be
	/// true for the redirect to be applied.
	pub http_error_code_returned_equals: HttpErrorCodeReturnedEquals,
	/// The object key name prefix when the redirect is applied. For example, to
	/// redirect requests for ExamplePage.html, the key prefix will be
	/// ExamplePage.html. To redirect request for all pages with the prefix docs/, the
	/// key prefix will be /docs, which identifies all objects in the docs/ folder.
	/// Required when the parent element Condition is specified and sibling
	/// HttpErrorCodeReturnedEquals is not specified. If both conditions are
	/// specified, both must be true for the redirect to be applied.
	pub key_prefix_equals: KeyPrefixEquals,
}


/// Parse Condition from response
struct ConditionParser;
impl ConditionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Condition, XmlParseError> {
		println!("in ConditionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Condition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of HttpErrorCodeReturnedEquals and child shape is HttpErrorCodeReturnedEquals
			if current_name == "HttpErrorCodeReturnedEquals"{
				obj.http_error_code_returned_equals = try!(HttpErrorCodeReturnedEqualsParser::parse_response(Some(&"HttpErrorCodeReturnedEquals".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of KeyPrefixEquals and child shape is KeyPrefixEquals
			if current_name == "KeyPrefixEquals"{
				obj.key_prefix_equals = try!(KeyPrefixEqualsParser::parse_response(Some(&"KeyPrefixEquals".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Condition contents to a SignedRequest
struct ConditionWriter;
impl ConditionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Condition, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = HttpErrorCodeReturnedEqualsWriter::write_params(request, &obj.http_error_code_returned_equals, Some(&ArgumentLocation::Body), &"HttpErrorCodeReturnedEquals".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = KeyPrefixEqualsWriter::write_params(request, &obj.key_prefix_equals, Some(&ArgumentLocation::Body), &"KeyPrefixEquals".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteObjectsOutput {
	pub deleted: DeletedObjects,
	pub errors: S3ClientErrors,
	pub request_charged: RequestCharged,
}


/// Parse DeleteObjectsOutput from response
struct DeleteObjectsOutputParser;
impl DeleteObjectsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteObjectsOutput, XmlParseError> {
		println!("in DeleteObjectsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = DeleteObjectsOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Deleted and child shape is DeletedObjects
			if current_name == "Deleted"{
				obj.deleted = try!(DeletedObjectsParser::parse_response(Some(&"Deleted".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Errors and child shape is Errors
			if current_name == "Errors"{
				obj.errors = try!(S3ClientErrorsParser::parse_response(Some(&"Errors".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct ErrorDocument {
	/// The object key name to use when a 4XX class error occurs.
	pub key: ObjectKey,
}


/// Parse ErrorDocument from response
struct ErrorDocumentParser;
impl ErrorDocumentParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ErrorDocument, XmlParseError> {
		println!("in ErrorDocumentParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ErrorDocument::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write ErrorDocument contents to a SignedRequest
struct ErrorDocumentWriter;
impl ErrorDocumentWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ErrorDocument, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
		body
	}
}
/// Container for key value pair that defines the criteria for the filter rule.
#[derive(Debug, Default)]
pub struct FilterRule {
	/// Object key name prefix or suffix identifying one or more objects to which the
	/// filtering rule applies. Maximum prefix length can be up to 1,024 characters.
	/// Overlapping prefixes and suffixes are not supported. For more information, go
	/// to [Configuring Event Notifications](http://docs.aws.amazon.com/AmazonS3/lates
	/// t/dev/NotificationHowTo.html) in the Amazon Simple Storage Service Developer
	/// Guide.
	pub name: FilterRuleName,
	pub value: FilterRuleValue,
}


/// Parse FilterRule from response
struct FilterRuleParser;
impl FilterRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<FilterRule, XmlParseError> {
		println!("in FilterRuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = FilterRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Name and child shape is FilterRuleName
			if current_name == "Name"{
				obj.name = try!(FilterRuleNameParser::parse_response(Some(&"Name".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Value and child shape is FilterRuleValue
			if current_name == "Value"{
				obj.value = try!(FilterRuleValueParser::parse_response(Some(&"Value".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write FilterRule contents to a SignedRequest
struct FilterRuleWriter;
impl FilterRuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a FilterRule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = FilterRuleNameWriter::write_params(request, &obj.name, Some(&ArgumentLocation::Body), &"Name".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = FilterRuleValueWriter::write_params(request, &obj.value, Some(&ArgumentLocation::Body), &"Value".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketLifecycleRequest {
	pub lifecycle_configuration: Option<LifecycleConfiguration>,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketLifecycleRequest contents to a SignedRequest
struct PutBucketLifecycleRequestWriter;
impl PutBucketLifecycleRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketLifecycleRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for lifecycle_configuration
		if let Some(ref obj) = obj.lifecycle_configuration {
			body = LifecycleConfigurationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"LifecycleConfiguration".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug)]
pub struct GetObjectTorrentOutput {
	pub body: Body,
	pub request_charged: RequestCharged,
}

// will impl default here:
impl Default for GetObjectTorrentOutput {
	fn default() -> GetObjectTorrentOutput {
		GetObjectTorrentOutput{
			body: Vec::new(),
			request_charged: RequestCharged::default(),
		}
	}
}

/// Parse GetObjectTorrentOutput from response
struct GetObjectTorrentOutputParser;
impl GetObjectTorrentOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetObjectTorrentOutput, XmlParseError> {
		println!("in GetObjectTorrentOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetObjectTorrentOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Body and child shape is Body
			if current_name == "Body"{
				obj.body = try!(BodyParser::parse_response(Some(&"Body".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type ContentLength = i32;

/// Parse ContentLength from response
struct ContentLengthParser;
impl ContentLengthParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentLength, XmlParseError> {
		println!("in ContentLengthParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentLength contents to a SignedRequest
struct ContentLengthWriter;
impl ContentLengthWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentLength, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentLength TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Transition {
	/// Indicates at what date the object is to be moved or deleted. Should be in GMT
	/// ISO 8601 Format.
	pub date: Date,
	/// Indicates the lifetime, in days, of the objects that are subject to the rule.
	/// The value must be a non-zero positive integer.
	pub days: Days,
	/// The class of storage used to store the object.
	pub storage_class: TransitionStorageClass,
}


/// Parse Transition from response
struct TransitionParser;
impl TransitionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Transition, XmlParseError> {
		println!("in TransitionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Transition::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Date and child shape is Date
			if current_name == "Date"{
				obj.date = try!(DateParser::parse_response(Some(&"Date".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Days and child shape is Days
			if current_name == "Days"{
				obj.days = try!(DaysParser::parse_response(Some(&"Days".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is TransitionStorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(TransitionStorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Transition contents to a SignedRequest
struct TransitionWriter;
impl TransitionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Transition, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DateWriter::write_params(request, &obj.date, Some(&ArgumentLocation::Body), &"Date".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = DaysWriter::write_params(request, &obj.days, Some(&ArgumentLocation::Body), &"Days".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TransitionStorageClassWriter::write_params(request, &obj.storage_class, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct QueueConfigurationDeprecated {
	pub queue: QueueArn,
	pub events: EventList,
	pub id: NotificationId,
	pub event: Event,
}


/// Parse QueueConfigurationDeprecated from response
struct QueueConfigurationDeprecatedParser;
impl QueueConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<QueueConfigurationDeprecated, XmlParseError> {
		println!("in QueueConfigurationDeprecatedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = QueueConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Queue and child shape is QueueArn
			if current_name == "Queue"{
				obj.queue = try!(QueueArnParser::parse_response(Some(&"Queue".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Event and child shape is Event
			if current_name == "Event"{
				obj.event = try!(EventParser::parse_response(Some(&"Event".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write QueueConfigurationDeprecated contents to a SignedRequest
struct QueueConfigurationDeprecatedWriter;
impl QueueConfigurationDeprecatedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a QueueConfigurationDeprecated, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = QueueArnWriter::write_params(request, &obj.queue, Some(&ArgumentLocation::Body), &"Queue".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = NotificationIdWriter::write_params(request, &obj.id, Some(&ArgumentLocation::Body), &"Id".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = EventWriter::write_params(request, &obj.event, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
#[derive(Debug)]
pub struct GetObjectOutput {
	/// Last modified date of the object
	pub last_modified: LastModified,
	/// The portion of the object returned in the response.
	pub content_range: ContentRange,
	pub request_charged: RequestCharged,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: ContentEncoding,
	pub replication_status: ReplicationStatus,
	pub storage_class: StorageClass,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: ServerSideEncryption,
	/// If present, specifies the ID of the AWS Key Management Service (KMS) master
	/// encryption key that was used for the object.
	pub ssekms_key_id: SSEKMSKeyId,
	/// Specifies presentational information for the object.
	pub content_disposition: ContentDisposition,
	/// A map of metadata to store with the object in S3.
	pub metadata: Metadata,
	/// Object data.
	pub body: Body,
	pub accept_ranges: AcceptRanges,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: WebsiteRedirectLocation,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Expires,
	/// Specifies whether the object retrieved was (true) or was not (false) a Delete
	/// Marker. If false, this response header does not appear in the response.
	pub delete_marker: DeleteMarker,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: CacheControl,
	/// Size of the body in bytes.
	pub content_length: ContentLength,
	/// If the object expiration is configured (see PUT Bucket lifecycle), the
	/// response includes this header. It includes the expiry-date and rule-id key
	/// value pairs providing object expiration information. The value of the rule-id
	/// is URL encoded.
	pub expiration: Expiration,
	/// This is set to the number of metadata entries not returned in x-amz-meta
	/// headers. This can happen if you create metadata using an API like SOAP that
	/// supports more flexible metadata than the REST API. For example, using SOAP,
	/// you can create metadata whose values are not legal HTTP headers.
	pub missing_meta: MissingMeta,
	/// Provides information about object restoration operation and expiration time of
	/// the restored object copy.
	pub restore: Restore,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header confirming the encryption
	/// algorithm used.
	pub sse_customer_algorithm: SSECustomerAlgorithm,
	/// A standard MIME type describing the format of the object data.
	pub content_type: ContentType,
	/// The language the content is in.
	pub content_language: ContentLanguage,
	/// Version of the object.
	pub version_id: ObjectVersionId,
	/// An ETag is an opaque identifier assigned by a web server to a specific version
	/// of a resource found at a URL
	pub e_tag: ETag,
	/// If server-side encryption with a customer-provided encryption key was
	/// requested, the response will include this header to provide round trip message
	/// integrity verification of the customer-provided encryption key.
	pub sse_customer_key_md5: SSECustomerKeyMD5,
}

// will impl default here:
impl Default for GetObjectOutput {
	fn default() -> GetObjectOutput {
		GetObjectOutput{
			last_modified: LastModified::default(),
			content_range: ContentRange::default(),
			request_charged: RequestCharged::default(),
			content_encoding: ContentEncoding::default(),
			replication_status: ReplicationStatus::default(),
			storage_class: StorageClass::default(),
			server_side_encryption: ServerSideEncryption::default(),
			ssekms_key_id: SSEKMSKeyId::default(),
			content_disposition: ContentDisposition::default(),
			metadata: Metadata::default(),
			body: Vec::new(),
			accept_ranges: AcceptRanges::default(),
			website_redirect_location: WebsiteRedirectLocation::default(),
			expires: Expires::default(),
			delete_marker: DeleteMarker::default(),
			cache_control: CacheControl::default(),
			content_length: ContentLength::default(),
			expiration: Expiration::default(),
			missing_meta: MissingMeta::default(),
			restore: Restore::default(),
			sse_customer_algorithm: SSECustomerAlgorithm::default(),
			content_type: ContentType::default(),
			content_language: ContentLanguage::default(),
			version_id: ObjectVersionId::default(),
			e_tag: ETag::default(),
			sse_customer_key_md5: SSECustomerKeyMD5::default(),
		}
	}
}

/// Parse GetObjectOutput from response
struct GetObjectOutputParser;
impl GetObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetObjectOutput, XmlParseError> {
		println!("in GetObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetObjectOutput::default();
		//parser for cname of LastModified and child shape is LastModified
		obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentRange and child shape is ContentRange
		obj.content_range = try!(ContentRangeParser::parse_response(Some(&"ContentRange".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentEncoding and child shape is ContentEncoding
		obj.content_encoding = try!(ContentEncodingParser::parse_response(Some(&"ContentEncoding".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ReplicationStatus and child shape is ReplicationStatus
		obj.replication_status = try!(ReplicationStatusParser::parse_response(Some(&"ReplicationStatus".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of StorageClass and child shape is StorageClass
		obj.storage_class = try!(StorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ServerSideEncryption and child shape is ServerSideEncryption
		obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_response(Some(&"ServerSideEncryption".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSEKMSKeyId and child shape is SSEKMSKeyId
		obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_response(Some(&"SSEKMSKeyId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentDisposition and child shape is ContentDisposition
		obj.content_disposition = try!(ContentDispositionParser::parse_response(Some(&"ContentDisposition".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Metadata and child shape is Metadata
		obj.metadata = try!(MetadataParser::parse_response(Some(&"Metadata".to_string()), Some(&ArgumentLocation::Headers), headers, stack));
		//parser for cname of AcceptRanges and child shape is AcceptRanges
		obj.accept_ranges = try!(AcceptRangesParser::parse_response(Some(&"AcceptRanges".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of WebsiteRedirectLocation and child shape is WebsiteRedirectLocation
		obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_response(Some(&"WebsiteRedirectLocation".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expires and child shape is Expires
		obj.expires = try!(ExpiresParser::parse_response(Some(&"Expires".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of DeleteMarker and child shape is DeleteMarker
		obj.delete_marker = try!(DeleteMarkerParser::parse_response(Some(&"DeleteMarker".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of CacheControl and child shape is CacheControl
		obj.cache_control = try!(CacheControlParser::parse_response(Some(&"CacheControl".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentLength and child shape is ContentLength
		obj.content_length = try!(ContentLengthParser::parse_response(Some(&"ContentLength".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Expiration and child shape is Expiration
		obj.expiration = try!(ExpirationParser::parse_response(Some(&"Expiration".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of MissingMeta and child shape is MissingMeta
		obj.missing_meta = try!(MissingMetaParser::parse_response(Some(&"MissingMeta".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of Restore and child shape is Restore
		obj.restore = try!(RestoreParser::parse_response(Some(&"Restore".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerAlgorithm and child shape is SSECustomerAlgorithm
		obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_response(Some(&"SSECustomerAlgorithm".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentType and child shape is ContentType
		obj.content_type = try!(ContentTypeParser::parse_response(Some(&"ContentType".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ContentLanguage and child shape is ContentLanguage
		obj.content_language = try!(ContentLanguageParser::parse_response(Some(&"ContentLanguage".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of VersionId and child shape is ObjectVersionId
		obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of ETag and child shape is ETag
		obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		//parser for cname of SSECustomerKeyMD5 and child shape is SSECustomerKeyMD5
		obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_response(Some(&"SSECustomerKeyMD5".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Body and child shape is Body
			if current_name == "Body"{
				obj.body = try!(BodyParser::parse_response(Some(&"Body".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleRequest {
	pub bucket: BucketName,
}

/// Write GetBucketLifecycleRequest contents to a SignedRequest
struct GetBucketLifecycleRequestWriter;
impl GetBucketLifecycleRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketLifecycleRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type CopySourceSSECustomerKeyMD5 = String;

/// Parse CopySourceSSECustomerKeyMD5 from response
struct CopySourceSSECustomerKeyMD5Parser;
impl CopySourceSSECustomerKeyMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceSSECustomerKeyMD5, XmlParseError> {
		println!("in CopySourceSSECustomerKeyMD5Parser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceSSECustomerKeyMD5 contents to a SignedRequest
struct CopySourceSSECustomerKeyMD5Writer;
impl CopySourceSSECustomerKeyMD5Writer {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceSSECustomerKeyMD5, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceSSECustomerKeyMD5 TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type SSECustomerAlgorithm = String;

/// Parse SSECustomerAlgorithm from response
struct SSECustomerAlgorithmParser;
impl SSECustomerAlgorithmParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<SSECustomerAlgorithm, XmlParseError> {
		println!("in SSECustomerAlgorithmParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write SSECustomerAlgorithm contents to a SignedRequest
struct SSECustomerAlgorithmWriter;
impl SSECustomerAlgorithmWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a SSECustomerAlgorithm, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT SSECustomerAlgorithm TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct S3ClientError {
	pub version_id: ObjectVersionId,
	pub code: Code,
	pub message: S3ClientMessage,
	pub key: ObjectKey,
}


/// Parse S3ClientError from response
struct S3ClientErrorParser;
impl S3ClientErrorParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<S3ClientError, XmlParseError> {
		println!("in S3ClientErrorParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = S3ClientError::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of VersionId and child shape is ObjectVersionId
			if current_name == "VersionId"{
				obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Code and child shape is Code
			if current_name == "Code"{
				obj.code = try!(CodeParser::parse_response(Some(&"Code".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Message and child shape is Message
			if current_name == "Message"{
				obj.message = try!(S3ClientMessageParser::parse_response(Some(&"Message".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write S3ClientError contents to a SignedRequest
struct S3ClientErrorWriter;
impl S3ClientErrorWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a S3ClientError, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectVersionIdWriter::write_params(request, &obj.version_id, Some(&ArgumentLocation::Body), &"VersionId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = CodeWriter::write_params(request, &obj.code, Some(&ArgumentLocation::Body), &"Code".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = S3ClientMessageWriter::write_params(request, &obj.message, Some(&ArgumentLocation::Body), &"Message".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct BucketLoggingStatus {
	pub logging_enabled: LoggingEnabled,
}


/// Parse BucketLoggingStatus from response
struct BucketLoggingStatusParser;
impl BucketLoggingStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<BucketLoggingStatus, XmlParseError> {
		println!("in BucketLoggingStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = BucketLoggingStatus::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LoggingEnabled and child shape is LoggingEnabled
			if current_name == "LoggingEnabled"{
				obj.logging_enabled = try!(LoggingEnabledParser::parse_response(Some(&"LoggingEnabled".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write BucketLoggingStatus contents to a SignedRequest
struct BucketLoggingStatusWriter;
impl BucketLoggingStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a BucketLoggingStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = LoggingEnabledWriter::write_params(request, &obj.logging_enabled, Some(&ArgumentLocation::Body), &"LoggingEnabled".to_string());
		body
	}
}
pub type IsLatest = bool;

/// Parse IsLatest from response
struct IsLatestParser;
impl IsLatestParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IsLatest, XmlParseError> {
		println!("in IsLatestParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : bool = bool::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = match bool::from_str(&header_str) {
							Err(_) => false,
							Ok(newbool) => newbool,
						}
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IsLatest contents to a SignedRequest
struct IsLatestWriter;
impl IsLatestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IsLatest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is boolean
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IsLatest TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MaxUploads = i32;

/// Parse MaxUploads from response
struct MaxUploadsParser;
impl MaxUploadsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MaxUploads, XmlParseError> {
		println!("in MaxUploadsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MaxUploads contents to a SignedRequest
struct MaxUploadsWriter;
impl MaxUploadsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MaxUploads, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MaxUploads TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct RoutingRule {
	/// Container for redirect information. You can redirect requests to another host,
	/// to another page, or with another protocol. In the event of an error, you can
	/// can specify a different error code to return.
	pub redirect: Redirect,
	/// A container for describing a condition that must be met for the specified
	/// redirect to apply. For example, 1. If request is for pages in the /docs
	/// folder, redirect to the /documents folder. 2. If request results in HTTP error
	/// 4xx, redirect request to another host where you might process the error.
	pub condition: Option<Condition>,
}


/// Parse RoutingRule from response
struct RoutingRuleParser;
impl RoutingRuleParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RoutingRule, XmlParseError> {
		println!("in RoutingRuleParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = RoutingRule::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Redirect and child shape is Redirect
			if current_name == "Redirect"{
				obj.redirect = try!(RedirectParser::parse_response(Some(&"Redirect".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Condition and child shape is Condition
			if current_name == "Condition"{
				obj.condition = Some(try!(ConditionParser::parse_response(Some(&"Condition".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write RoutingRule contents to a SignedRequest
struct RoutingRuleWriter;
impl RoutingRuleWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RoutingRule, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = RedirectWriter::write_params(request, &obj.redirect, Some(&ArgumentLocation::Body), &"Redirect".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for condition
		if let Some(ref obj) = obj.condition {
			body = ConditionWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Condition".to_string());
		}
		body
	}
}
pub type MissingMeta = i32;

/// Parse MissingMeta from response
struct MissingMetaParser;
impl MissingMetaParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MissingMeta, XmlParseError> {
		println!("in MissingMetaParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MissingMeta contents to a SignedRequest
struct MissingMetaWriter;
impl MissingMetaWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MissingMeta, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MissingMeta TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type SSEKMSKeyId = String;

/// Parse SSEKMSKeyId from response
struct SSEKMSKeyIdParser;
impl SSEKMSKeyIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<SSEKMSKeyId, XmlParseError> {
		println!("in SSEKMSKeyIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write SSEKMSKeyId contents to a SignedRequest
struct SSEKMSKeyIdWriter;
impl SSEKMSKeyIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a SSEKMSKeyId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT SSEKMSKeyId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type AllowedOrigins = Vec<AllowedOrigin>;

/// Parse AllowedOrigins from response
struct AllowedOriginsParser;
impl AllowedOriginsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedOrigins, XmlParseError> {
		println!("in AllowedOriginsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "AllowedOrigin" {
//we need to iterate over members of AllowedOrigin
// obj.push for AllowedOrigin
			obj.push(try!(AllowedOriginParser::parse_response(Some(&"AllowedOrigin"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write AllowedOrigins contents to a SignedRequest
struct AllowedOriginsWriter;
impl AllowedOriginsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedOrigins, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for AllowedOrigin
;
		body
	}
}
/// The specified multipart upload does not exist.
#[derive(Debug, Default)]
pub struct NoSuchUpload;


/// Parse NoSuchUpload from response
struct NoSuchUploadParser;
impl NoSuchUploadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoSuchUpload, XmlParseError> {
		println!("in NoSuchUploadParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NoSuchUpload::default();
		Ok(obj)
	}
}
/// Write NoSuchUpload contents to a SignedRequest
struct NoSuchUploadWriter;
impl NoSuchUploadWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoSuchUpload, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
#[derive(Debug)]
pub struct PutObjectRequest {
	pub request_payer: Option<RequestPayer>,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: Option<ContentEncoding>,
	/// The type of storage to use for the object. Defaults to 'STANDARD'.
	pub storage_class: Option<StorageClass>,
	/// Allows grantee to read the object ACL.
	pub grant_read_acp: Option<GrantReadACP>,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: Option<ServerSideEncryption>,
	/// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
	/// requests for an object protected by AWS KMS will fail if not made via SSL or
	/// using SigV4. Documentation on configuring any of the officially supported AWS
	/// SDKs and CLI can be found at
	/// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
	/// signature-version
	pub ssekms_key_id: Option<SSEKMSKeyId>,
	/// Specifies presentational information for the object.
	pub content_disposition: Option<ContentDisposition>,
	/// A map of metadata to store with the object in S3.
	pub metadata: Option<Metadata>,
	/// Object data.
	pub body: Option<Body>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: Option<WebsiteRedirectLocation>,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Option<Expires>,
	pub key: ObjectKey,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: Option<CacheControl>,
	/// Size of the body in bytes. This parameter is useful when the size of the body
	/// cannot be determined automatically.
	pub content_length: Option<ContentLength>,
	pub bucket: BucketName,
	/// Allows grantee to read the object data and its metadata.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to write the ACL for the applicable object.
	pub grant_write_acp: Option<GrantWriteACP>,
	/// The canned ACL to apply to the object.
	pub acl: Option<ObjectCannedACL>,
	/// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
	pub grant_full_control: Option<GrantFullControl>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// A standard MIME type describing the format of the object data.
	pub content_type: Option<ContentType>,
	/// The language the content is in.
	pub content_language: Option<ContentLanguage>,
	pub content_md5: Option<ContentMD5>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

// will impl default here:
impl Default for PutObjectRequest {
	fn default() -> PutObjectRequest {
		PutObjectRequest{
			request_payer: None,
			content_encoding: None,
			storage_class: None,
			grant_read_acp: None,
			server_side_encryption: None,
			ssekms_key_id: None,
			content_disposition: None,
			metadata: None,
			body: None,
			sse_customer_key: None,
			website_redirect_location: None,
			expires: None,
			key: ObjectKey::default(),
			cache_control: None,
			content_length: None,
			bucket: BucketName::default(),
			grant_read: None,
			grant_write_acp: None,
			acl: None,
			grant_full_control: None,
			sse_customer_algorithm: None,
			content_type: None,
			content_language: None,
			content_md5: None,
			sse_customer_key_md5: None,
		}
	}
}
/// Write PutObjectRequest contents to a SignedRequest
struct PutObjectRequestWriter;
impl PutObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for content_encoding
		if let Some(ref obj) = obj.content_encoding {
			ContentEncodingWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Encoding".to_string());
		}
		// optional writing for storage_class
		if let Some(ref obj) = obj.storage_class {
			StorageClassWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-storage-class".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		// optional writing for server_side_encryption
		if let Some(ref obj) = obj.server_side_encryption {
			ServerSideEncryptionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption".to_string());
		}
		// optional writing for ssekms_key_id
		if let Some(ref obj) = obj.ssekms_key_id {
			SSEKMSKeyIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-aws-kms-key-id".to_string());
		}
		// optional writing for content_disposition
		if let Some(ref obj) = obj.content_disposition {
			ContentDispositionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Disposition".to_string());
		}
		// optional writing for metadata
		if let Some(ref obj) = obj.metadata {
			MetadataWriter::write_params(request, &obj, Some(&ArgumentLocation::Headers), &"x-amz-meta-".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for body
		if let Some(ref obj) = obj.body {
			body = BodyWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Body".to_string());
		}
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
		// optional writing for website_redirect_location
		if let Some(ref obj) = obj.website_redirect_location {
			WebsiteRedirectLocationWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-website-redirect-location".to_string());
		}
		// optional writing for expires
		if let Some(ref obj) = obj.expires {
			ExpiresWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Expires".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for cache_control
		if let Some(ref obj) = obj.cache_control {
			CacheControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Cache-Control".to_string());
		}
		// optional writing for content_length
		if let Some(ref obj) = obj.content_length {
			ContentLengthWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Length".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			ObjectCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for content_type
		if let Some(ref obj) = obj.content_type {
			ContentTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Type".to_string());
		}
		// optional writing for content_language
		if let Some(ref obj) = obj.content_language {
			ContentLanguageWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Language".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
		body
	}
}
pub type Code = String;

/// Parse Code from response
struct CodeParser;
impl CodeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Code, XmlParseError> {
		println!("in CodeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Code contents to a SignedRequest
struct CodeWriter;
impl CodeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Code, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Code TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for ReplicationStatus
#[derive(Debug,PartialEq)]
pub enum ReplicationStatus {
	COMPLETE,
	PENDING,
	FAILED,
	REPLICA,
}
impl fmt::Display for ReplicationStatus {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ReplicationStatus::COMPLETE => write!(f, "COMPLETE"),
			ReplicationStatus::PENDING => write!(f, "PENDING"),
			ReplicationStatus::FAILED => write!(f, "FAILED"),
			ReplicationStatus::REPLICA => write!(f, "REPLICA"),
		}
	}
}
impl Default for ReplicationStatus{
	fn default() -> ReplicationStatus{
		ReplicationStatus::COMPLETE
	}
}
impl From<String> for ReplicationStatus{
	fn from(string: String) -> ReplicationStatus{
		match string.as_ref() {
			"COMPLETE" => ReplicationStatus::COMPLETE,
			"PENDING" => ReplicationStatus::PENDING,
			"FAILED" => ReplicationStatus::FAILED,
			"REPLICA" => ReplicationStatus::REPLICA,
			_ => ReplicationStatus::default(),
		}
	}
}

/// Parse ReplicationStatus from response
struct ReplicationStatusParser;
impl ReplicationStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplicationStatus, XmlParseError> {
		println!("in ReplicationStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ReplicationStatus = ReplicationStatus::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ReplicationStatus::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ReplicationStatus::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ReplicationStatus contents to a SignedRequest
struct ReplicationStatusWriter;
impl ReplicationStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplicationStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ReplicationStatus TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type AllowedHeaders = Vec<AllowedHeader>;

/// Parse AllowedHeaders from response
struct AllowedHeadersParser;
impl AllowedHeadersParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedHeaders, XmlParseError> {
		println!("in AllowedHeadersParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "AllowedHeader" {
//we need to iterate over members of AllowedHeader
// obj.push for AllowedHeader
			obj.push(try!(AllowedHeaderParser::parse_response(Some(&"AllowedHeader"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write AllowedHeaders contents to a SignedRequest
struct AllowedHeadersWriter;
impl AllowedHeadersWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedHeaders, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for AllowedHeader
;
		body
	}
}
#[derive(Debug, Default)]
pub struct Tagging {
	pub tag_set: TagSet,
}


/// Parse Tagging from response
struct TaggingParser;
impl TaggingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Tagging, XmlParseError> {
		println!("in TaggingParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Tagging::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of TagSet and child shape is TagSet
			if current_name == "TagSet"{
				obj.tag_set = try!(TagSetParser::parse_response(Some(&"TagSet".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Tagging contents to a SignedRequest
struct TaggingWriter;
impl TaggingWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Tagging, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = TagSetWriter::write_params(request, &obj.tag_set, Some(&ArgumentLocation::Body), &"Tag".to_string());
		body
	}
}
pub type ContentMD5 = String;

/// Parse ContentMD5 from response
struct ContentMD5Parser;
impl ContentMD5Parser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentMD5, XmlParseError> {
		println!("in ContentMD5Parser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentMD5 contents to a SignedRequest
struct ContentMD5Writer;
impl ContentMD5Writer {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentMD5, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentMD5 TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ObjectVersion {
	/// Date and time the object was last modified.
	pub last_modified: LastModified,
	/// Version ID of an object.
	pub version_id: ObjectVersionId,
	pub e_tag: ETag,
	/// The class of storage used to store the object.
	pub storage_class: ObjectVersionStorageClass,
	/// The object key.
	pub key: ObjectKey,
	pub owner: Owner,
	/// Specifies whether the object is (true) or is not (false) the latest version of
	/// an object.
	pub is_latest: IsLatest,
	/// Size in bytes of the object.
	pub size: Size,
}


/// Parse ObjectVersion from response
struct ObjectVersionParser;
impl ObjectVersionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
		println!("in ObjectVersionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ObjectVersion::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LastModified and child shape is LastModified
			if current_name == "LastModified"{
				obj.last_modified = try!(LastModifiedParser::parse_response(Some(&"LastModified".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of VersionId and child shape is ObjectVersionId
			if current_name == "VersionId"{
				obj.version_id = try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is ObjectVersionStorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(ObjectVersionStorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsLatest and child shape is IsLatest
			if current_name == "IsLatest"{
				obj.is_latest = try!(IsLatestParser::parse_response(Some(&"IsLatest".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Size and child shape is Size
			if current_name == "Size"{
				obj.size = try!(SizeParser::parse_response(Some(&"Size".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write ObjectVersion contents to a SignedRequest
struct ObjectVersionWriter;
impl ObjectVersionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectVersion, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = LastModifiedWriter::write_params(request, &obj.last_modified, Some(&ArgumentLocation::Body), &"LastModified".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectVersionIdWriter::write_params(request, &obj.version_id, Some(&ArgumentLocation::Body), &"VersionId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ETagWriter::write_params(request, &obj.e_tag, Some(&ArgumentLocation::Body), &"ETag".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectVersionStorageClassWriter::write_params(request, &obj.storage_class, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = OwnerWriter::write_params(request, &obj.owner, Some(&ArgumentLocation::Body), &"Owner".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = IsLatestWriter::write_params(request, &obj.is_latest, Some(&ArgumentLocation::Body), &"IsLatest".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = SizeWriter::write_params(request, &obj.size, Some(&ArgumentLocation::Body), &"Size".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct ListObjectVersionsOutput {
	pub name: BucketName,
	pub versions: ObjectVersionList,
	pub delete_markers: DeleteMarkers,
	/// Use this value for the key marker request parameter in a subsequent request.
	pub next_key_marker: NextKeyMarker,
	pub delimiter: Delimiter,
	pub max_keys: MaxKeys,
	pub prefix: Prefix,
	/// Marks the last Key returned in a truncated response.
	pub key_marker: KeyMarker,
	/// Use this value for the next version id marker parameter in a subsequent
	/// request.
	pub next_version_id_marker: NextVersionIdMarker,
	/// Encoding type used by Amazon S3 to encode object keys in the response.
	pub encoding_type: EncodingType,
	/// A flag that indicates whether or not Amazon S3 returned all of the results
	/// that satisfied the search criteria. If your results were truncated, you can
	/// make a follow-up paginated request using the NextKeyMarker and
	/// NextVersionIdMarker response parameters as a starting place in another request
	/// to return the rest of the results.
	pub is_truncated: IsTruncated,
	pub version_id_marker: VersionIdMarker,
	pub common_prefixes: CommonPrefixList,
}


/// Parse ListObjectVersionsOutput from response
struct ListObjectVersionsOutputParser;
impl ListObjectVersionsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ListObjectVersionsOutput, XmlParseError> {
		println!("in ListObjectVersionsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ListObjectVersionsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Name and child shape is BucketName
			if current_name == "Name"{
				obj.name = try!(BucketNameParser::parse_response(Some(&"Name".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Versions and child shape is ObjectVersionList
			if current_name == "Versions"{
				obj.versions = try!(ObjectVersionListParser::parse_response(Some(&"Versions".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of DeleteMarkers and child shape is DeleteMarkers
			if current_name == "DeleteMarkers"{
				obj.delete_markers = try!(DeleteMarkersParser::parse_response(Some(&"DeleteMarkers".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextKeyMarker and child shape is NextKeyMarker
			if current_name == "NextKeyMarker"{
				obj.next_key_marker = try!(NextKeyMarkerParser::parse_response(Some(&"NextKeyMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Delimiter and child shape is Delimiter
			if current_name == "Delimiter"{
				obj.delimiter = try!(DelimiterParser::parse_response(Some(&"Delimiter".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MaxKeys and child shape is MaxKeys
			if current_name == "MaxKeys"{
				obj.max_keys = try!(MaxKeysParser::parse_response(Some(&"MaxKeys".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of KeyMarker and child shape is KeyMarker
			if current_name == "KeyMarker"{
				obj.key_marker = try!(KeyMarkerParser::parse_response(Some(&"KeyMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextVersionIdMarker and child shape is NextVersionIdMarker
			if current_name == "NextVersionIdMarker"{
				obj.next_version_id_marker = try!(NextVersionIdMarkerParser::parse_response(Some(&"NextVersionIdMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of EncodingType and child shape is EncodingType
			if current_name == "EncodingType"{
				obj.encoding_type = try!(EncodingTypeParser::parse_response(Some(&"EncodingType".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsTruncated and child shape is IsTruncated
			if current_name == "IsTruncated"{
				obj.is_truncated = try!(IsTruncatedParser::parse_response(Some(&"IsTruncated".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of VersionIdMarker and child shape is VersionIdMarker
			if current_name == "VersionIdMarker"{
				obj.version_id_marker = try!(VersionIdMarkerParser::parse_response(Some(&"VersionIdMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of CommonPrefixes and child shape is CommonPrefixList
			if current_name == "CommonPrefixes"{
				obj.common_prefixes = try!(CommonPrefixListParser::parse_response(Some(&"CommonPrefixes".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct PutBucketNotificationConfigurationRequest {
	pub notification_configuration: NotificationConfiguration,
	pub bucket: BucketName,
}

/// Write PutBucketNotificationConfigurationRequest contents to a SignedRequest
struct PutBucketNotificationConfigurationRequestWriter;
impl PutBucketNotificationConfigurationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketNotificationConfigurationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = NotificationConfigurationWriter::write_params(request, &obj.notification_configuration, Some(&ArgumentLocation::Body), &"NotificationConfiguration".to_string());
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type Prefix = String;

/// Parse Prefix from response
struct PrefixParser;
impl PrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Prefix, XmlParseError> {
		println!("in PrefixParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Prefix contents to a SignedRequest
struct PrefixWriter;
impl PrefixWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Prefix, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Prefix TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ObjectList = Vec<Object>;

/// Parse ObjectList from response
struct ObjectListParser;
impl ObjectListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectList, XmlParseError> {
		println!("in ObjectListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Object" {
//we need to iterate over members of Object
// obj.push for Object
			obj.push(try!(ObjectParser::parse_response(Some(&"Object"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write ObjectList contents to a SignedRequest
struct ObjectListWriter;
impl ObjectListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Object
;
		body
	}
}
#[derive(Debug, Default)]
pub struct CopyObjectRequest {
	pub request_payer: Option<RequestPayer>,
	/// Copies the object if it has been modified since the specified time.
	pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
	/// Copies the object if it hasn't been modified since the specified time.
	pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
	/// Specifies what content encodings have been applied to the object and thus what
	/// decoding mechanisms must be applied to obtain the media-type referenced by the
	/// Content-Type header field.
	pub content_encoding: Option<ContentEncoding>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt
	/// the source object. The encryption key provided in this header must be one that
	/// was used when the source object was created.
	pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
	/// The type of storage to use for the object. Defaults to 'STANDARD'.
	pub storage_class: Option<StorageClass>,
	/// Allows grantee to read the object ACL.
	pub grant_read_acp: Option<GrantReadACP>,
	/// The Server-side encryption algorithm used when storing this object in S3
	/// (e.g., AES256, aws:kms).
	pub server_side_encryption: Option<ServerSideEncryption>,
	/// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
	/// requests for an object protected by AWS KMS will fail if not made via SSL or
	/// using SigV4. Documentation on configuring any of the officially supported AWS
	/// SDKs and CLI can be found at
	/// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
	/// signature-version
	pub ssekms_key_id: Option<SSEKMSKeyId>,
	/// Specifies presentational information for the object.
	pub content_disposition: Option<ContentDisposition>,
	/// A map of metadata to store with the object in S3.
	pub metadata: Option<Metadata>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// If the bucket is configured as a website, redirects requests for this object
	/// to another object in the same bucket or to an external URL. Amazon S3 stores
	/// the value of this header in the object metadata.
	pub website_redirect_location: Option<WebsiteRedirectLocation>,
	/// The name of the source bucket and key name of the source object, separated by
	/// a slash (/). Must be URL-encoded.
	pub copy_source: CopySource,
	/// The date and time at which the object is no longer cacheable.
	pub expires: Option<Expires>,
	pub key: ObjectKey,
	/// Specifies caching behavior along the request/reply chain.
	pub cache_control: Option<CacheControl>,
	/// Specifies the algorithm to use when decrypting the source object (e.g.,
	/// AES256).
	pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
	pub bucket: BucketName,
	/// Allows grantee to read the object data and its metadata.
	pub grant_read: Option<GrantRead>,
	/// Allows grantee to write the ACL for the applicable object.
	pub grant_write_acp: Option<GrantWriteACP>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
	/// The canned ACL to apply to the object.
	pub acl: Option<ObjectCannedACL>,
	/// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
	pub grant_full_control: Option<GrantFullControl>,
	/// Copies the object if its entity tag (ETag) matches the specified tag.
	pub copy_source_if_match: Option<CopySourceIfMatch>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// A standard MIME type describing the format of the object data.
	pub content_type: Option<ContentType>,
	/// The language the content is in.
	pub content_language: Option<ContentLanguage>,
	/// Specifies whether the metadata is copied from the source object or replaced
	/// with metadata provided in the request.
	pub metadata_directive: Option<MetadataDirective>,
	/// Copies the object if its entity tag (ETag) is different than the specified
	/// ETag.
	pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

/// Write CopyObjectRequest contents to a SignedRequest
struct CopyObjectRequestWriter;
impl CopyObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopyObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for copy_source_if_modified_since
		if let Some(ref obj) = obj.copy_source_if_modified_since {
			CopySourceIfModifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-modified-since".to_string());
		}
		// optional writing for copy_source_if_unmodified_since
		if let Some(ref obj) = obj.copy_source_if_unmodified_since {
			CopySourceIfUnmodifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-unmodified-since".to_string());
		}
		// optional writing for content_encoding
		if let Some(ref obj) = obj.content_encoding {
			ContentEncodingWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Encoding".to_string());
		}
		// optional writing for copy_source_sse_customer_key
		if let Some(ref obj) = obj.copy_source_sse_customer_key {
			CopySourceSSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-key".to_string());
		}
		// optional writing for storage_class
		if let Some(ref obj) = obj.storage_class {
			StorageClassWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-storage-class".to_string());
		}
		// optional writing for grant_read_acp
		if let Some(ref obj) = obj.grant_read_acp {
			GrantReadACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read-acp".to_string());
		}
		// optional writing for server_side_encryption
		if let Some(ref obj) = obj.server_side_encryption {
			ServerSideEncryptionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption".to_string());
		}
		// optional writing for ssekms_key_id
		if let Some(ref obj) = obj.ssekms_key_id {
			SSEKMSKeyIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-aws-kms-key-id".to_string());
		}
		// optional writing for content_disposition
		if let Some(ref obj) = obj.content_disposition {
			ContentDispositionWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Disposition".to_string());
		}
		// optional writing for metadata
		if let Some(ref obj) = obj.metadata {
			MetadataWriter::write_params(request, &obj, Some(&ArgumentLocation::Headers), &"x-amz-meta-".to_string());
		}
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
		// optional writing for website_redirect_location
		if let Some(ref obj) = obj.website_redirect_location {
			WebsiteRedirectLocationWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-website-redirect-location".to_string());
		}
//required field: 
		CopySourceWriter::write_params(request, &obj.copy_source, Some(&ArgumentLocation::Header), &"x-amz-copy-source".to_string());
		// optional writing for expires
		if let Some(ref obj) = obj.expires {
			ExpiresWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Expires".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for cache_control
		if let Some(ref obj) = obj.cache_control {
			CacheControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Cache-Control".to_string());
		}
		// optional writing for copy_source_sse_customer_algorithm
		if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
			CopySourceSSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-algorithm".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for grant_read
		if let Some(ref obj) = obj.grant_read {
			GrantReadWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-read".to_string());
		}
		// optional writing for grant_write_acp
		if let Some(ref obj) = obj.grant_write_acp {
			GrantWriteACPWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-write-acp".to_string());
		}
		// optional writing for copy_source_sse_customer_key_md5
		if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
			CopySourceSSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-key-MD5".to_string());
		}
		// optional writing for acl
		if let Some(ref obj) = obj.acl {
			ObjectCannedACLWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-acl".to_string());
		}
		// optional writing for grant_full_control
		if let Some(ref obj) = obj.grant_full_control {
			GrantFullControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-grant-full-control".to_string());
		}
		// optional writing for copy_source_if_match
		if let Some(ref obj) = obj.copy_source_if_match {
			CopySourceIfMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-match".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for content_type
		if let Some(ref obj) = obj.content_type {
			ContentTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Type".to_string());
		}
		// optional writing for content_language
		if let Some(ref obj) = obj.content_language {
			ContentLanguageWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Language".to_string());
		}
		// optional writing for metadata_directive
		if let Some(ref obj) = obj.metadata_directive {
			MetadataDirectiveWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-metadata-directive".to_string());
		}
		// optional writing for copy_source_if_none_match
		if let Some(ref obj) = obj.copy_source_if_none_match {
			CopySourceIfNoneMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-none-match".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteObjectsRequest {
	/// The concatenation of the authentication device's serial number, a space, and
	/// the value that is displayed on your authentication device.
	pub mfa: Option<MFA>,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub delete: Delete,
}

/// Write DeleteObjectsRequest contents to a SignedRequest
struct DeleteObjectsRequestWriter;
impl DeleteObjectsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteObjectsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for mfa
		if let Some(ref obj) = obj.mfa {
			MFAWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-mfa".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = DeleteWriter::write_params(request, &obj.delete, Some(&ArgumentLocation::Body), &"Delete".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct ListPartsOutput {
	/// Identifies who initiated the multipart upload.
	pub initiator: Initiator,
	/// Name of the bucket to which the multipart upload was initiated.
	pub bucket: BucketName,
	/// When a list is truncated, this element specifies the last part in the list, as
	/// well as the value to use for the part-number-marker request parameter in a
	/// subsequent request.
	pub next_part_number_marker: NextPartNumberMarker,
	pub parts: Parts,
	/// Upload ID identifying the multipart upload whose parts are being listed.
	pub upload_id: MultipartUploadId,
	/// The class of storage used to store the object.
	pub storage_class: StorageClass,
	/// Object key for which the multipart upload was initiated.
	pub key: ObjectKey,
	pub request_charged: RequestCharged,
	pub owner: Owner,
	/// Maximum number of parts that were allowed in the response.
	pub max_parts: MaxParts,
	/// Indicates whether the returned list of parts is truncated.
	pub is_truncated: IsTruncated,
	/// Part number after which listing begins.
	pub part_number_marker: PartNumberMarker,
}


/// Parse ListPartsOutput from response
struct ListPartsOutputParser;
impl ListPartsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ListPartsOutput, XmlParseError> {
		println!("in ListPartsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ListPartsOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Initiator and child shape is Initiator
			if current_name == "Initiator"{
				obj.initiator = try!(InitiatorParser::parse_response(Some(&"Initiator".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Bucket and child shape is BucketName
			if current_name == "Bucket"{
				obj.bucket = try!(BucketNameParser::parse_response(Some(&"Bucket".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of NextPartNumberMarker and child shape is NextPartNumberMarker
			if current_name == "NextPartNumberMarker"{
				obj.next_part_number_marker = try!(NextPartNumberMarkerParser::parse_response(Some(&"NextPartNumberMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Parts and child shape is Parts
			if current_name == "Parts"{
				obj.parts = try!(PartsParser::parse_response(Some(&"Parts".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of UploadId and child shape is MultipartUploadId
			if current_name == "UploadId"{
				obj.upload_id = try!(MultipartUploadIdParser::parse_response(Some(&"UploadId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is StorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(StorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MaxParts and child shape is MaxParts
			if current_name == "MaxParts"{
				obj.max_parts = try!(MaxPartsParser::parse_response(Some(&"MaxParts".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IsTruncated and child shape is IsTruncated
			if current_name == "IsTruncated"{
				obj.is_truncated = try!(IsTruncatedParser::parse_response(Some(&"IsTruncated".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of PartNumberMarker and child shape is PartNumberMarker
			if current_name == "PartNumberMarker"{
				obj.part_number_marker = try!(PartNumberMarkerParser::parse_response(Some(&"PartNumberMarker".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type Marker = String;

/// Parse Marker from response
struct MarkerParser;
impl MarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Marker, XmlParseError> {
		println!("in MarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Marker contents to a SignedRequest
struct MarkerWriter;
impl MarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Marker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Marker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for ObjectCannedACL
#[derive(Debug,PartialEq)]
pub enum ObjectCannedACL {
	private,
	public_read,
	public_read_write,
	authenticated_read,
	bucket_owner_read,
	bucket_owner_full_control,
}
impl fmt::Display for ObjectCannedACL {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ObjectCannedACL::private => write!(f, "private"),
			ObjectCannedACL::public_read => write!(f, "public-read"),
			ObjectCannedACL::public_read_write => write!(f, "public-read-write"),
			ObjectCannedACL::authenticated_read => write!(f, "authenticated-read"),
			ObjectCannedACL::bucket_owner_read => write!(f, "bucket-owner-read"),
			ObjectCannedACL::bucket_owner_full_control => write!(f, "bucket-owner-full-control"),
		}
	}
}
impl Default for ObjectCannedACL{
	fn default() -> ObjectCannedACL{
		ObjectCannedACL::private
	}
}
impl From<String> for ObjectCannedACL{
	fn from(string: String) -> ObjectCannedACL{
		match string.as_ref() {
			"private" => ObjectCannedACL::private,
			"public-read" => ObjectCannedACL::public_read,
			"public-read-write" => ObjectCannedACL::public_read_write,
			"authenticated-read" => ObjectCannedACL::authenticated_read,
			"bucket-owner-read" => ObjectCannedACL::bucket_owner_read,
			"bucket-owner-full-control" => ObjectCannedACL::bucket_owner_full_control,
			_ => ObjectCannedACL::default(),
		}
	}
}

/// Parse ObjectCannedACL from response
struct ObjectCannedACLParser;
impl ObjectCannedACLParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
		println!("in ObjectCannedACLParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ObjectCannedACL = ObjectCannedACL::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ObjectCannedACL::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ObjectCannedACL::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ObjectCannedACL contents to a SignedRequest
struct ObjectCannedACLWriter;
impl ObjectCannedACLWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectCannedACL, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ObjectCannedACL TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct RestoreRequest {
	/// Lifetime of the active copy in days
	pub days: Days,
}

/// Write RestoreRequest contents to a SignedRequest
struct RestoreRequestWriter;
impl RestoreRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RestoreRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DaysWriter::write_params(request, &obj.days, Some(&ArgumentLocation::Body), &"Days".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CompletedPart {
	/// Part number that identifies the part. This is a positive integer between 1 and
	/// 10,000.
	pub part_number: PartNumber,
	/// Entity tag returned when the part was uploaded.
	pub e_tag: ETag,
}


/// Parse CompletedPart from response
struct CompletedPartParser;
impl CompletedPartParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CompletedPart, XmlParseError> {
		println!("in CompletedPartParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CompletedPart::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of PartNumber and child shape is PartNumber
			if current_name == "PartNumber"{
				obj.part_number = try!(PartNumberParser::parse_response(Some(&"PartNumber".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ETag and child shape is ETag
			if current_name == "ETag"{
				obj.e_tag = try!(ETagParser::parse_response(Some(&"ETag".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CompletedPart contents to a SignedRequest
struct CompletedPartWriter;
impl CompletedPartWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CompletedPart, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = PartNumberWriter::write_params(request, &obj.part_number, Some(&ArgumentLocation::Body), &"PartNumber".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ETagWriter::write_params(request, &obj.e_tag, Some(&ArgumentLocation::Body), &"ETag".to_string());
		body
	}
}
pub type QueueArn = String;

/// Parse QueueArn from response
struct QueueArnParser;
impl QueueArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<QueueArn, XmlParseError> {
		println!("in QueueArnParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write QueueArn contents to a SignedRequest
struct QueueArnWriter;
impl QueueArnWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a QueueArn, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT QueueArn TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Location = String;

/// Parse Location from response
struct LocationParser;
impl LocationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Location, XmlParseError> {
		println!("in LocationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Location contents to a SignedRequest
struct LocationWriter;
impl LocationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Location, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Location TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type HttpErrorCodeReturnedEquals = String;

/// Parse HttpErrorCodeReturnedEquals from response
struct HttpErrorCodeReturnedEqualsParser;
impl HttpErrorCodeReturnedEqualsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<HttpErrorCodeReturnedEquals, XmlParseError> {
		println!("in HttpErrorCodeReturnedEqualsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write HttpErrorCodeReturnedEquals contents to a SignedRequest
struct HttpErrorCodeReturnedEqualsWriter;
impl HttpErrorCodeReturnedEqualsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a HttpErrorCodeReturnedEquals, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT HttpErrorCodeReturnedEquals TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct NotificationConfigurationDeprecated {
	pub cloud_function_configuration: CloudFunctionConfiguration,
	pub queue_configuration: QueueConfigurationDeprecated,
	pub topic_configuration: TopicConfigurationDeprecated,
}


/// Parse NotificationConfigurationDeprecated from response
struct NotificationConfigurationDeprecatedParser;
impl NotificationConfigurationDeprecatedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
		println!("in NotificationConfigurationDeprecatedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NotificationConfigurationDeprecated::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CloudFunctionConfiguration and child shape is CloudFunctionConfiguration
			if current_name == "CloudFunctionConfiguration"{
				obj.cloud_function_configuration = try!(CloudFunctionConfigurationParser::parse_response(Some(&"CloudFunctionConfiguration".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of QueueConfiguration and child shape is QueueConfigurationDeprecated
			if current_name == "QueueConfiguration"{
				obj.queue_configuration = try!(QueueConfigurationDeprecatedParser::parse_response(Some(&"QueueConfiguration".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of TopicConfiguration and child shape is TopicConfigurationDeprecated
			if current_name == "TopicConfiguration"{
				obj.topic_configuration = try!(TopicConfigurationDeprecatedParser::parse_response(Some(&"TopicConfiguration".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write NotificationConfigurationDeprecated contents to a SignedRequest
struct NotificationConfigurationDeprecatedWriter;
impl NotificationConfigurationDeprecatedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NotificationConfigurationDeprecated, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = CloudFunctionConfigurationWriter::write_params(request, &obj.cloud_function_configuration, Some(&ArgumentLocation::Body), &"CloudFunctionConfiguration".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = QueueConfigurationDeprecatedWriter::write_params(request, &obj.queue_configuration, Some(&ArgumentLocation::Body), &"QueueConfiguration".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TopicConfigurationDeprecatedWriter::write_params(request, &obj.topic_configuration, Some(&ArgumentLocation::Body), &"TopicConfiguration".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct UploadPartCopyRequest {
	/// Copies the object if its entity tag (ETag) matches the specified tag.
	pub copy_source_if_match: Option<CopySourceIfMatch>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
	pub request_payer: Option<RequestPayer>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt
	/// the source object. The encryption key provided in this header must be one that
	/// was used when the source object was created.
	pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
	/// Specifies the algorithm to use when decrypting the source object (e.g.,
	/// AES256).
	pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
	/// The name of the source bucket and key name of the source object, separated by
	/// a slash (/). Must be URL-encoded.
	pub copy_source: CopySource,
	/// Copies the object if it has been modified since the specified time.
	pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
	pub bucket: BucketName,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header. This must be the same encryption
	/// key specified in the initiate multipart upload request.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// Copies the object if it hasn't been modified since the specified time.
	pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
	/// Copies the object if its entity tag (ETag) is different than the specified
	/// ETag.
	pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
	/// Upload ID identifying the multipart upload whose part is being copied.
	pub upload_id: MultipartUploadId,
	pub key: ObjectKey,
	/// The range of bytes to copy from the source object. The range value must use
	/// the form bytes=first-last, where the first and last are the zero-based byte
	/// offsets to copy. For example, bytes=0-9 indicates that you want to copy the
	/// first ten bytes of the source. You can copy a range only if the source object
	/// is greater than 5 GB.
	pub copy_source_range: Option<CopySourceRange>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
	/// Part number of part being copied. This is a positive integer between 1 and
	/// 10,000.
	pub part_number: PartNumber,
}

/// Write UploadPartCopyRequest contents to a SignedRequest
struct UploadPartCopyRequestWriter;
impl UploadPartCopyRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a UploadPartCopyRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for copy_source_if_match
		if let Some(ref obj) = obj.copy_source_if_match {
			CopySourceIfMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-match".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for copy_source_sse_customer_key_md5
		if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
			CopySourceSSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-key-MD5".to_string());
		}
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for copy_source_sse_customer_key
		if let Some(ref obj) = obj.copy_source_sse_customer_key {
			CopySourceSSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-key".to_string());
		}
		// optional writing for copy_source_sse_customer_algorithm
		if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
			CopySourceSSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-server-side-encryption-customer-algorithm".to_string());
		}
//required field: 
		CopySourceWriter::write_params(request, &obj.copy_source, Some(&ArgumentLocation::Header), &"x-amz-copy-source".to_string());
		// optional writing for copy_source_if_modified_since
		if let Some(ref obj) = obj.copy_source_if_modified_since {
			CopySourceIfModifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-modified-since".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
		// optional writing for copy_source_if_unmodified_since
		if let Some(ref obj) = obj.copy_source_if_unmodified_since {
			CopySourceIfUnmodifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-unmodified-since".to_string());
		}
		// optional writing for copy_source_if_none_match
		if let Some(ref obj) = obj.copy_source_if_none_match {
			CopySourceIfNoneMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-if-none-match".to_string());
		}
//required field: 
		MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Querystring), &"uploadId".to_string());
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for copy_source_range
		if let Some(ref obj) = obj.copy_source_range {
			CopySourceRangeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-copy-source-range".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
//required field: 
		PartNumberWriter::write_params(request, &obj.part_number, Some(&ArgumentLocation::Querystring), &"partNumber".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CORSConfiguration {
	pub cors_rules: CORSRules,
}


/// Parse CORSConfiguration from response
struct CORSConfigurationParser;
impl CORSConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CORSConfiguration, XmlParseError> {
		println!("in CORSConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CORSConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CORSRules and child shape is CORSRules
			if current_name == "CORSRules"{
				obj.cors_rules = try!(CORSRulesParser::parse_response(Some(&"CORSRules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CORSConfiguration contents to a SignedRequest
struct CORSConfigurationWriter;
impl CORSConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CORSConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = CORSRulesWriter::write_params(request, &obj.cors_rules, Some(&ArgumentLocation::Body), &"CORSRule".to_string());
		body
	}
}
pub type LastModified = String;

/// Parse LastModified from response
struct LastModifiedParser;
impl LastModifiedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LastModified, XmlParseError> {
		println!("in LastModifiedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write LastModified contents to a SignedRequest
struct LastModifiedWriter;
impl LastModifiedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LastModified, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT LastModified TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ContentRange = String;

/// Parse ContentRange from response
struct ContentRangeParser;
impl ContentRangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentRange, XmlParseError> {
		println!("in ContentRangeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentRange contents to a SignedRequest
struct ContentRangeWriter;
impl ContentRangeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentRange, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentRange TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Grantee {
	/// Email address of the grantee.
	pub email_address: Option<EmailAddress>,
	/// Type of grantee
	pub foo_type: Type,
	/// Screen name of the grantee.
	pub display_name: Option<DisplayName>,
	/// The canonical user ID of the grantee.
	pub id: Option<ID>,
	/// URI of the grantee group.
	pub uri: Option<URI>,
}


/// Parse Grantee from response
struct GranteeParser;
impl GranteeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Grantee, XmlParseError> {
		println!("in GranteeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Grantee::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of EmailAddress and child shape is EmailAddress
			if current_name == "EmailAddress"{
				obj.email_address = Some(try!(EmailAddressParser::parse_response(Some(&"EmailAddress".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Type and child shape is Type
			if current_name == "Type"{
				obj.foo_type = try!(TypeParser::parse_response(Some(&"Type".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of DisplayName and child shape is DisplayName
			if current_name == "DisplayName"{
				obj.display_name = Some(try!(DisplayNameParser::parse_response(Some(&"DisplayName".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of ID and child shape is ID
			if current_name == "ID"{
				obj.id = Some(try!(IDParser::parse_response(Some(&"ID".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of URI and child shape is URI
			if current_name == "URI"{
				obj.uri = Some(try!(URIParser::parse_response(Some(&"URI".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Grantee contents to a SignedRequest
struct GranteeWriter;
impl GranteeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Grantee, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for email_address
		if let Some(ref obj) = obj.email_address {
			body = EmailAddressWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"EmailAddress".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = TypeWriter::write_params(request, &obj.foo_type, Some(&ArgumentLocation::Body), &"xsi:type".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for display_name
		if let Some(ref obj) = obj.display_name {
			body = DisplayNameWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"DisplayName".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = IDWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"ID".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for uri
		if let Some(ref obj) = obj.uri {
			body = URIWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"URI".to_string());
		}
		body
	}
}
//NEEDS ENUM for ExpirationStatus
#[derive(Debug,PartialEq)]
pub enum ExpirationStatus {
	Enabled,
	Disabled,
}
impl fmt::Display for ExpirationStatus {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ExpirationStatus::Enabled => write!(f, "Enabled"),
			ExpirationStatus::Disabled => write!(f, "Disabled"),
		}
	}
}
impl Default for ExpirationStatus{
	fn default() -> ExpirationStatus{
		ExpirationStatus::Enabled
	}
}
impl From<String> for ExpirationStatus{
	fn from(string: String) -> ExpirationStatus{
		match string.as_ref() {
			"Enabled" => ExpirationStatus::Enabled,
			"Disabled" => ExpirationStatus::Disabled,
			_ => ExpirationStatus::default(),
		}
	}
}

/// Parse ExpirationStatus from response
struct ExpirationStatusParser;
impl ExpirationStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ExpirationStatus, XmlParseError> {
		println!("in ExpirationStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ExpirationStatus = ExpirationStatus::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ExpirationStatus::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ExpirationStatus::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ExpirationStatus contents to a SignedRequest
struct ExpirationStatusWriter;
impl ExpirationStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ExpirationStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ExpirationStatus TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type CopySourceIfUnmodifiedSince = String;

/// Parse CopySourceIfUnmodifiedSince from response
struct CopySourceIfUnmodifiedSinceParser;
impl CopySourceIfUnmodifiedSinceParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceIfUnmodifiedSince, XmlParseError> {
		println!("in CopySourceIfUnmodifiedSinceParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceIfUnmodifiedSince contents to a SignedRequest
struct CopySourceIfUnmodifiedSinceWriter;
impl CopySourceIfUnmodifiedSinceWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceIfUnmodifiedSince, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceIfUnmodifiedSince TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketVersioningRequest {
	pub bucket: BucketName,
}

/// Write GetBucketVersioningRequest contents to a SignedRequest
struct GetBucketVersioningRequestWriter;
impl GetBucketVersioningRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketVersioningRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct MultipartUpload {
	/// Identifies who initiated the multipart upload.
	pub initiator: Initiator,
	/// Date and time at which the multipart upload was initiated.
	pub initiated: Initiated,
	/// Upload ID that identifies the multipart upload.
	pub upload_id: MultipartUploadId,
	/// The class of storage used to store the object.
	pub storage_class: StorageClass,
	/// Key of the object for which the multipart upload was initiated.
	pub key: ObjectKey,
	pub owner: Owner,
}


/// Parse MultipartUpload from response
struct MultipartUploadParser;
impl MultipartUploadParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MultipartUpload, XmlParseError> {
		println!("in MultipartUploadParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = MultipartUpload::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Initiator and child shape is Initiator
			if current_name == "Initiator"{
				obj.initiator = try!(InitiatorParser::parse_response(Some(&"Initiator".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Initiated and child shape is Initiated
			if current_name == "Initiated"{
				obj.initiated = try!(InitiatedParser::parse_response(Some(&"Initiated".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of UploadId and child shape is MultipartUploadId
			if current_name == "UploadId"{
				obj.upload_id = try!(MultipartUploadIdParser::parse_response(Some(&"UploadId".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of StorageClass and child shape is StorageClass
			if current_name == "StorageClass"{
				obj.storage_class = try!(StorageClassParser::parse_response(Some(&"StorageClass".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write MultipartUpload contents to a SignedRequest
struct MultipartUploadWriter;
impl MultipartUploadWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MultipartUpload, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = InitiatorWriter::write_params(request, &obj.initiator, Some(&ArgumentLocation::Body), &"Initiator".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = InitiatedWriter::write_params(request, &obj.initiated, Some(&ArgumentLocation::Body), &"Initiated".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Body), &"UploadId".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = StorageClassWriter::write_params(request, &obj.storage_class, Some(&ArgumentLocation::Body), &"StorageClass".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = OwnerWriter::write_params(request, &obj.owner, Some(&ArgumentLocation::Body), &"Owner".to_string());
		body
	}
}
pub type GrantWrite = String;

/// Parse GrantWrite from response
struct GrantWriteParser;
impl GrantWriteParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GrantWrite, XmlParseError> {
		println!("in GrantWriteParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write GrantWrite contents to a SignedRequest
struct GrantWriteWriter;
impl GrantWriteWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GrantWrite, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT GrantWrite TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type TagSet = Vec<Tag>;

/// Parse TagSet from response
struct TagSetParser;
impl TagSetParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TagSet, XmlParseError> {
		println!("in TagSetParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Tag" {
//we need to iterate over members of Tag
// skip Tag.  It's a location name.
// obj.push for Tag
			obj.push(try!(TagParser::parse_response(Some(&"Tag"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write TagSet contents to a SignedRequest
struct TagSetWriter;
impl TagSetWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TagSet, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Tag
;
		body
	}
}
pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;

/// Parse LambdaFunctionConfigurationList from response
struct LambdaFunctionConfigurationListParser;
impl LambdaFunctionConfigurationListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LambdaFunctionConfigurationList, XmlParseError> {
		println!("in LambdaFunctionConfigurationListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "LambdaFunctionConfiguration" {
//we need to iterate over members of LambdaFunctionConfiguration
// obj.push for LambdaFunctionConfiguration
			obj.push(try!(LambdaFunctionConfigurationParser::parse_response(Some(&"LambdaFunctionConfiguration"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write LambdaFunctionConfigurationList contents to a SignedRequest
struct LambdaFunctionConfigurationListWriter;
impl LambdaFunctionConfigurationListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LambdaFunctionConfigurationList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for LambdaFunctionConfiguration
;
		body
	}
}
//NEEDS ENUM for ServerSideEncryption
#[derive(Debug,PartialEq)]
pub enum ServerSideEncryption {
	AES256,
	aws_kms,
}
impl fmt::Display for ServerSideEncryption {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ServerSideEncryption::AES256 => write!(f, "AES256"),
			ServerSideEncryption::aws_kms => write!(f, "aws:kms"),
		}
	}
}
impl Default for ServerSideEncryption{
	fn default() -> ServerSideEncryption{
		ServerSideEncryption::AES256
	}
}
impl From<String> for ServerSideEncryption{
	fn from(string: String) -> ServerSideEncryption{
		match string.as_ref() {
			"AES256" => ServerSideEncryption::AES256,
			"aws:kms" => ServerSideEncryption::aws_kms,
			_ => ServerSideEncryption::default(),
		}
	}
}

/// Parse ServerSideEncryption from response
struct ServerSideEncryptionParser;
impl ServerSideEncryptionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ServerSideEncryption, XmlParseError> {
		println!("in ServerSideEncryptionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ServerSideEncryption = ServerSideEncryption::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ServerSideEncryption::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ServerSideEncryption::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ServerSideEncryption contents to a SignedRequest
struct ServerSideEncryptionWriter;
impl ServerSideEncryptionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ServerSideEncryption, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ServerSideEncryption TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketVersioningOutput {
	/// The versioning state of the bucket.
	pub status: BucketVersioningStatus,
	/// Specifies whether MFA delete is enabled in the bucket versioning
	/// configuration. This element is only returned if the bucket has been configured
	/// with MFA delete. If the bucket has never been so configured, this element is
	/// not returned.
	pub mfa_delete: MFADeleteStatus,
}


/// Parse GetBucketVersioningOutput from response
struct GetBucketVersioningOutputParser;
impl GetBucketVersioningOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketVersioningOutput, XmlParseError> {
		println!("in GetBucketVersioningOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketVersioningOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Status and child shape is BucketVersioningStatus
			if current_name == "Status"{
				obj.status = try!(BucketVersioningStatusParser::parse_response(Some(&"Status".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of MFADelete and child shape is MFADeleteStatus
			if current_name == "MFADelete"{
				obj.mfa_delete = try!(MFADeleteStatusParser::parse_response(Some(&"MFADelete".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3
/// permanently deletes the noncurrent object versions. You set this lifecycle
/// configuration action on a bucket that has versioning enabled (or suspended) to
/// request that Amazon S3 delete noncurrent object versions at a specific period
/// in the object's lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionExpiration {
	/// Specifies the number of days an object is noncurrent before Amazon S3 can
	/// perform the associated action. For information about the noncurrent days
	/// calculations, see [How Amazon S3 Calculates When an Object Became
	/// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
	/// Storage Service Developer Guide.
	pub noncurrent_days: Days,
}


/// Parse NoncurrentVersionExpiration from response
struct NoncurrentVersionExpirationParser;
impl NoncurrentVersionExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoncurrentVersionExpiration, XmlParseError> {
		println!("in NoncurrentVersionExpirationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NoncurrentVersionExpiration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of NoncurrentDays and child shape is Days
			if current_name == "NoncurrentDays"{
				obj.noncurrent_days = try!(DaysParser::parse_response(Some(&"NoncurrentDays".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write NoncurrentVersionExpiration contents to a SignedRequest
struct NoncurrentVersionExpirationWriter;
impl NoncurrentVersionExpirationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoncurrentVersionExpiration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = DaysWriter::write_params(request, &obj.noncurrent_days, Some(&ArgumentLocation::Body), &"NoncurrentDays".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketRequestPaymentOutput {
	/// Specifies who pays for the download and request fees.
	pub payer: Payer,
}


/// Parse GetBucketRequestPaymentOutput from response
struct GetBucketRequestPaymentOutputParser;
impl GetBucketRequestPaymentOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
		println!("in GetBucketRequestPaymentOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketRequestPaymentOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Payer and child shape is Payer
			if current_name == "Payer"{
				obj.payer = try!(PayerParser::parse_response(Some(&"Payer".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct GetObjectRequest {
	/// Sets the Content-Encoding header of the response.
	pub response_content_encoding: Option<ResponseContentEncoding>,
	/// Sets the Content-Language header of the response.
	pub response_content_language: Option<ResponseContentLanguage>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	/// Sets the Content-Type header of the response.
	pub response_content_type: Option<ResponseContentType>,
	/// Return the object only if it has not been modified since the specified time,
	/// otherwise return a 412 (precondition failed).
	pub if_unmodified_since: Option<IfUnmodifiedSince>,
	/// VersionId used to reference a specific version of the object.
	pub version_id: Option<ObjectVersionId>,
	pub request_payer: Option<RequestPayer>,
	/// Sets the Cache-Control header of the response.
	pub response_cache_control: Option<ResponseCacheControl>,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header.
	pub sse_customer_key: Option<SSECustomerKey>,
	pub bucket: BucketName,
	/// Return the object only if its entity tag (ETag) is different from the one
	/// specified, otherwise return a 304 (not modified).
	pub if_none_match: Option<IfNoneMatch>,
	/// Sets the Content-Disposition header of the response
	pub response_content_disposition: Option<ResponseContentDisposition>,
	/// Downloads the specified range bytes of an object. For more information about
	/// the HTTP Range header, go to
	/// http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.
	pub range: Option<Range>,
	pub key: ObjectKey,
	/// Return the object only if its entity tag (ETag) is the same as the one
	/// specified, otherwise return a 412 (precondition failed).
	pub if_match: Option<IfMatch>,
	/// Sets the Expires header of the response.
	pub response_expires: Option<ResponseExpires>,
	/// Return the object only if it has been modified since the specified time,
	/// otherwise return a 304 (not modified).
	pub if_modified_since: Option<IfModifiedSince>,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

/// Write GetObjectRequest contents to a SignedRequest
struct GetObjectRequestWriter;
impl GetObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for response_content_encoding
		if let Some(ref obj) = obj.response_content_encoding {
			ResponseContentEncodingWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-content-encoding".to_string());
		}
		// optional writing for response_content_language
		if let Some(ref obj) = obj.response_content_language {
			ResponseContentLanguageWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-content-language".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for response_content_type
		if let Some(ref obj) = obj.response_content_type {
			ResponseContentTypeWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-content-type".to_string());
		}
		// optional writing for if_unmodified_since
		if let Some(ref obj) = obj.if_unmodified_since {
			IfUnmodifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Unmodified-Since".to_string());
		}
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"versionId".to_string());
		}
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for response_cache_control
		if let Some(ref obj) = obj.response_cache_control {
			ResponseCacheControlWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-cache-control".to_string());
		}
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for if_none_match
		if let Some(ref obj) = obj.if_none_match {
			IfNoneMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-None-Match".to_string());
		}
		// optional writing for response_content_disposition
		if let Some(ref obj) = obj.response_content_disposition {
			ResponseContentDispositionWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-content-disposition".to_string());
		}
		// optional writing for range
		if let Some(ref obj) = obj.range {
			RangeWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Range".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for if_match
		if let Some(ref obj) = obj.if_match {
			IfMatchWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Match".to_string());
		}
		// optional writing for response_expires
		if let Some(ref obj) = obj.response_expires {
			ResponseExpiresWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"response-expires".to_string());
		}
		// optional writing for if_modified_since
		if let Some(ref obj) = obj.if_modified_since {
			IfModifiedSinceWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"If-Modified-Since".to_string());
		}
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
		body
	}
}
pub type ContentDisposition = String;

/// Parse ContentDisposition from response
struct ContentDispositionParser;
impl ContentDispositionParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentDisposition, XmlParseError> {
		println!("in ContentDispositionParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentDisposition contents to a SignedRequest
struct ContentDispositionWriter;
impl ContentDispositionWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentDisposition, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentDisposition TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MetadataKey = String;

/// Parse MetadataKey from response
struct MetadataKeyParser;
impl MetadataKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MetadataKey, XmlParseError> {
		println!("in MetadataKeyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MetadataKey contents to a SignedRequest
struct MetadataKeyWriter;
impl MetadataKeyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MetadataKey, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MetadataKey TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ResponseContentEncoding = String;

/// Parse ResponseContentEncoding from response
struct ResponseContentEncodingParser;
impl ResponseContentEncodingParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseContentEncoding, XmlParseError> {
		println!("in ResponseContentEncodingParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseContentEncoding contents to a SignedRequest
struct ResponseContentEncodingWriter;
impl ResponseContentEncodingWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseContentEncoding, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseContentEncoding TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLoggingRequest {
	pub bucket: BucketName,
}

/// Write GetBucketLoggingRequest contents to a SignedRequest
struct GetBucketLoggingRequestWriter;
impl GetBucketLoggingRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketLoggingRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type UploadIdMarker = String;

/// Parse UploadIdMarker from response
struct UploadIdMarkerParser;
impl UploadIdMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<UploadIdMarker, XmlParseError> {
		println!("in UploadIdMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write UploadIdMarker contents to a SignedRequest
struct UploadIdMarkerWriter;
impl UploadIdMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a UploadIdMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT UploadIdMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for Payer
#[derive(Debug,PartialEq)]
pub enum Payer {
	Requester,
	BucketOwner,
}
impl fmt::Display for Payer {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Payer::Requester => write!(f, "Requester"),
			Payer::BucketOwner => write!(f, "BucketOwner"),
		}
	}
}
impl Default for Payer{
	fn default() -> Payer{
		Payer::Requester
	}
}
impl From<String> for Payer{
	fn from(string: String) -> Payer{
		match string.as_ref() {
			"Requester" => Payer::Requester,
			"BucketOwner" => Payer::BucketOwner,
			_ => Payer::default(),
		}
	}
}

/// Parse Payer from response
struct PayerParser;
impl PayerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Payer, XmlParseError> {
		println!("in PayerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Payer = Payer::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Payer::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = Payer::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Payer contents to a SignedRequest
struct PayerWriter;
impl PayerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Payer, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Payer TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for Type
#[derive(Debug,PartialEq)]
pub enum Type {
	CanonicalUser,
	AmazonCustomerByEmail,
	Group,
}
impl fmt::Display for Type {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Type::CanonicalUser => write!(f, "CanonicalUser"),
			Type::AmazonCustomerByEmail => write!(f, "AmazonCustomerByEmail"),
			Type::Group => write!(f, "Group"),
		}
	}
}
impl Default for Type{
	fn default() -> Type{
		Type::CanonicalUser
	}
}
impl From<String> for Type{
	fn from(string: String) -> Type{
		match string.as_ref() {
			"CanonicalUser" => Type::CanonicalUser,
			"AmazonCustomerByEmail" => Type::AmazonCustomerByEmail,
			"Group" => Type::Group,
			_ => Type::default(),
		}
	}
}

/// Parse Type from response
struct TypeParser;
impl TypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Type, XmlParseError> {
		println!("in TypeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : Type = Type::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = Type::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = Type::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Type contents to a SignedRequest
struct TypeWriter;
impl TypeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Type, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Type TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type Buckets = Vec<Bucket>;

/// Parse Buckets from response
struct BucketsParser;
impl BucketsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Buckets, XmlParseError> {
		println!("in BucketsParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "Bucket" {
//we need to iterate over members of Bucket
// skip Bucket.  It's a location name.
// obj.push for Bucket
			obj.push(try!(BucketParser::parse_response(Some(&"Bucket"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write Buckets contents to a SignedRequest
struct BucketsWriter;
impl BucketsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Buckets, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for Bucket
;
		body
	}
}
pub type Expires = String;

/// Parse Expires from response
struct ExpiresParser;
impl ExpiresParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Expires, XmlParseError> {
		println!("in ExpiresParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Expires contents to a SignedRequest
struct ExpiresWriter;
impl ExpiresWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Expires, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Expires TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct RestoreObjectOutput {
	pub request_charged: RequestCharged,
}


/// Parse RestoreObjectOutput from response
struct RestoreObjectOutputParser;
impl RestoreObjectOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RestoreObjectOutput, XmlParseError> {
		println!("in RestoreObjectOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = RestoreObjectOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct RestoreObjectRequest {
	pub version_id: Option<ObjectVersionId>,
	pub restore_request: Option<RestoreRequest>,
	pub bucket: BucketName,
	pub request_payer: Option<RequestPayer>,
	pub key: ObjectKey,
}

/// Write RestoreObjectRequest contents to a SignedRequest
struct RestoreObjectRequestWriter;
impl RestoreObjectRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RestoreObjectRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"versionId".to_string());
		}
// GOES IN BODY, set here from return types?
		// optional writing for restore_request
		if let Some(ref obj) = obj.restore_request {
			body = RestoreRequestWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"RestoreRequest".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLocationOutput {
	pub location_constraint: BucketLocationConstraint,
}


/// Parse GetBucketLocationOutput from response
struct GetBucketLocationOutputParser;
impl GetBucketLocationOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketLocationOutput, XmlParseError> {
		println!("in GetBucketLocationOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketLocationOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of LocationConstraint and child shape is BucketLocationConstraint
			if current_name == "LocationConstraint"{
				obj.location_constraint = try!(BucketLocationConstraintParser::parse_response(Some(&"LocationConstraint".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
#[derive(Debug, Default)]
pub struct GetObjectAclOutput {
	pub owner: Owner,
	/// A list of grants.
	pub grants: Grants,
	pub request_charged: RequestCharged,
}


/// Parse GetObjectAclOutput from response
struct GetObjectAclOutputParser;
impl GetObjectAclOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetObjectAclOutput, XmlParseError> {
		println!("in GetObjectAclOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetObjectAclOutput::default();
		//parser for cname of RequestCharged and child shape is RequestCharged
		obj.request_charged = try!(RequestChargedParser::parse_response(Some(&"RequestCharged".to_string()), Some(&ArgumentLocation::Header), headers, stack));
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Owner and child shape is Owner
			if current_name == "Owner"{
				obj.owner = try!(OwnerParser::parse_response(Some(&"Owner".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Grants and child shape is Grants
			if current_name == "Grants"{
				obj.grants = try!(GrantsParser::parse_response(Some(&"Grants".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type ReplaceKeyWith = String;

/// Parse ReplaceKeyWith from response
struct ReplaceKeyWithParser;
impl ReplaceKeyWithParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplaceKeyWith, XmlParseError> {
		println!("in ReplaceKeyWithParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ReplaceKeyWith contents to a SignedRequest
struct ReplaceKeyWithWriter;
impl ReplaceKeyWithWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplaceKeyWith, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ReplaceKeyWith TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ObjectKey = String;

/// Parse ObjectKey from response
struct ObjectKeyParser;
impl ObjectKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectKey, XmlParseError> {
		println!("in ObjectKeyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ObjectKey contents to a SignedRequest
struct ObjectKeyWriter;
impl ObjectKeyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectKey, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ObjectKey TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketTaggingRequest {
	pub bucket: BucketName,
}

/// Write GetBucketTaggingRequest contents to a SignedRequest
struct GetBucketTaggingRequestWriter;
impl GetBucketTaggingRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketTaggingRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyOutput {
	/// The bucket policy as a JSON document.
	pub policy: Policy,
}


/// Parse GetBucketPolicyOutput from response
struct GetBucketPolicyOutputParser;
impl GetBucketPolicyOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketPolicyOutput, XmlParseError> {
		println!("in GetBucketPolicyOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketPolicyOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Policy and child shape is Policy
			if current_name == "Policy"{
				obj.policy = try!(PolicyParser::parse_response(Some(&"Policy".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type MaxAgeSeconds = i32;

/// Parse MaxAgeSeconds from response
struct MaxAgeSecondsParser;
impl MaxAgeSecondsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MaxAgeSeconds, XmlParseError> {
		println!("in MaxAgeSecondsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MaxAgeSeconds contents to a SignedRequest
struct MaxAgeSecondsWriter;
impl MaxAgeSecondsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MaxAgeSeconds, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MaxAgeSeconds TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type CopySourceRange = String;

/// Parse CopySourceRange from response
struct CopySourceRangeParser;
impl CopySourceRangeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceRange, XmlParseError> {
		println!("in CopySourceRangeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceRange contents to a SignedRequest
struct CopySourceRangeWriter;
impl CopySourceRangeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceRange, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceRange TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketLifecycleConfigurationRequest {
	pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
	pub bucket: BucketName,
}

/// Write PutBucketLifecycleConfigurationRequest contents to a SignedRequest
struct PutBucketLifecycleConfigurationRequestWriter;
impl PutBucketLifecycleConfigurationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketLifecycleConfigurationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for lifecycle_configuration
		if let Some(ref obj) = obj.lifecycle_configuration {
			body = BucketLifecycleConfigurationWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"LifecycleConfiguration".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type TopicArn = String;

/// Parse TopicArn from response
struct TopicArnParser;
impl TopicArnParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<TopicArn, XmlParseError> {
		println!("in TopicArnParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write TopicArn contents to a SignedRequest
struct TopicArnWriter;
impl TopicArnWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a TopicArn, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT TopicArn TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketTaggingRequest {
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	pub tagging: Tagging,
}

/// Write PutBucketTaggingRequest contents to a SignedRequest
struct PutBucketTaggingRequestWriter;
impl PutBucketTaggingRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketTaggingRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = TaggingWriter::write_params(request, &obj.tagging, Some(&ArgumentLocation::Body), &"Tagging".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleConfigurationRequest {
	pub bucket: BucketName,
}

/// Write GetBucketLifecycleConfigurationRequest contents to a SignedRequest
struct GetBucketLifecycleConfigurationRequestWriter;
impl GetBucketLifecycleConfigurationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketLifecycleConfigurationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketRequestPaymentRequest {
	pub bucket: BucketName,
}

/// Write GetBucketRequestPaymentRequest contents to a SignedRequest
struct GetBucketRequestPaymentRequestWriter;
impl GetBucketRequestPaymentRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketRequestPaymentRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct CommonPrefix {
	pub prefix: Prefix,
}


/// Parse CommonPrefix from response
struct CommonPrefixParser;
impl CommonPrefixParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CommonPrefix, XmlParseError> {
		println!("in CommonPrefixParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = CommonPrefix::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Prefix and child shape is Prefix
			if current_name == "Prefix"{
				obj.prefix = try!(PrefixParser::parse_response(Some(&"Prefix".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write CommonPrefix contents to a SignedRequest
struct CommonPrefixWriter;
impl CommonPrefixWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CommonPrefix, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = PrefixWriter::write_params(request, &obj.prefix, Some(&ArgumentLocation::Body), &"Prefix".to_string());
		body
	}
}
/// The specified key does not exist.
#[derive(Debug, Default)]
pub struct NoSuchKey;


/// Parse NoSuchKey from response
struct NoSuchKeyParser;
impl NoSuchKeyParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NoSuchKey, XmlParseError> {
		println!("in NoSuchKeyParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = NoSuchKey::default();
		Ok(obj)
	}
}
/// Write NoSuchKey contents to a SignedRequest
struct NoSuchKeyWriter;
impl NoSuchKeyWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NoSuchKey, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		body
	}
}
#[derive(Debug)]
pub struct UploadPartRequest {
	pub body: Option<Body>,
	/// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
	pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
	pub request_payer: Option<RequestPayer>,
	/// Size of the body in bytes. This parameter is useful when the size of the body
	/// cannot be determined automatically.
	pub content_length: Option<ContentLength>,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
	/// Specifies the customer-provided encryption key for Amazon S3 to use in
	/// encrypting data. This value is used to store the object and then it is
	/// discarded; Amazon does not store the encryption key. The key must be
	/// appropriate for use with the algorithm specified in the x-amz-server-
	/// side​-encryption​-customer-algorithm header. This must be the same encryption
	/// key specified in the initiate multipart upload request.
	pub sse_customer_key: Option<SSECustomerKey>,
	/// Upload ID identifying the multipart upload whose part is being uploaded.
	pub upload_id: MultipartUploadId,
	pub key: ObjectKey,
	/// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
	/// Amazon S3 uses this header for a message integrity check to ensure the
	/// encryption key was transmitted without error.
	pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
	/// Part number of part being uploaded. This is a positive integer between 1 and
	/// 10,000.
	pub part_number: PartNumber,
}

// will impl default here:
impl Default for UploadPartRequest {
	fn default() -> UploadPartRequest {
		UploadPartRequest{
			body: None,
			sse_customer_algorithm: None,
			request_payer: None,
			content_length: None,
			content_md5: None,
			bucket: BucketName::default(),
			sse_customer_key: None,
			upload_id: MultipartUploadId::default(),
			key: ObjectKey::default(),
			sse_customer_key_md5: None,
			part_number: PartNumber::default(),
		}
	}
}
/// Write UploadPartRequest contents to a SignedRequest
struct UploadPartRequestWriter;
impl UploadPartRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a UploadPartRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for body
		if let Some(ref obj) = obj.body {
			body = BodyWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Body".to_string());
		}
		// optional writing for sse_customer_algorithm
		if let Some(ref obj) = obj.sse_customer_algorithm {
			SSECustomerAlgorithmWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-algorithm".to_string());
		}
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
		// optional writing for content_length
		if let Some(ref obj) = obj.content_length {
			ContentLengthWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-Length".to_string());
		}
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		// optional writing for sse_customer_key
		if let Some(ref obj) = obj.sse_customer_key {
			SSECustomerKeyWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key".to_string());
		}
//required field: 
		MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Querystring), &"uploadId".to_string());
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for sse_customer_key_md5
		if let Some(ref obj) = obj.sse_customer_key_md5 {
			SSECustomerKeyMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-server-side-encryption-customer-key-MD5".to_string());
		}
//required field: 
		PartNumberWriter::write_params(request, &obj.part_number, Some(&ArgumentLocation::Querystring), &"partNumber".to_string());
		body
	}
}
pub type ObjectVersionList = Vec<ObjectVersion>;

/// Parse ObjectVersionList from response
struct ObjectVersionListParser;
impl ObjectVersionListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectVersionList, XmlParseError> {
		println!("in ObjectVersionListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "ObjectVersion" {
//we need to iterate over members of ObjectVersion
// obj.push for ObjectVersion
			obj.push(try!(ObjectVersionParser::parse_response(Some(&"ObjectVersion"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write ObjectVersionList contents to a SignedRequest
struct ObjectVersionListWriter;
impl ObjectVersionListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectVersionList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for ObjectVersion
;
		body
	}
}
pub type MFA = String;

/// Parse MFA from response
struct MFAParser;
impl MFAParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MFA, XmlParseError> {
		println!("in MFAParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MFA contents to a SignedRequest
struct MFAWriter;
impl MFAWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MFA, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MFA TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MultipartUploadList = Vec<MultipartUpload>;

/// Parse MultipartUploadList from response
struct MultipartUploadListParser;
impl MultipartUploadListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MultipartUploadList, XmlParseError> {
		println!("in MultipartUploadListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "MultipartUpload" {
//we need to iterate over members of MultipartUpload
// obj.push for MultipartUpload
			obj.push(try!(MultipartUploadParser::parse_response(Some(&"MultipartUpload"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write MultipartUploadList contents to a SignedRequest
struct MultipartUploadListWriter;
impl MultipartUploadListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MultipartUploadList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for MultipartUpload
;
		body
	}
}
pub type AllowedHeader = String;

/// Parse AllowedHeader from response
struct AllowedHeaderParser;
impl AllowedHeaderParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<AllowedHeader, XmlParseError> {
		println!("in AllowedHeaderParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write AllowedHeader contents to a SignedRequest
struct AllowedHeaderWriter;
impl AllowedHeaderWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a AllowedHeader, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT AllowedHeader TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct Bucket {
	/// Date the bucket was created.
	pub creation_date: CreationDate,
	/// The name of the bucket.
	pub name: BucketName,
}


/// Parse Bucket from response
struct BucketParser;
impl BucketParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Bucket, XmlParseError> {
		println!("in BucketParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = Bucket::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CreationDate and child shape is CreationDate
			if current_name == "CreationDate"{
				obj.creation_date = try!(CreationDateParser::parse_response(Some(&"CreationDate".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Name and child shape is BucketName
			if current_name == "Name"{
				obj.name = try!(BucketNameParser::parse_response(Some(&"Name".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write Bucket contents to a SignedRequest
struct BucketWriter;
impl BucketWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Bucket, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = CreationDateWriter::write_params(request, &obj.creation_date, Some(&ArgumentLocation::Body), &"CreationDate".to_string());
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketNameWriter::write_params(request, &obj.name, Some(&ArgumentLocation::Body), &"Name".to_string());
		body
	}
}
pub type URI = String;

/// Parse URI from response
struct URIParser;
impl URIParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<URI, XmlParseError> {
		println!("in URIParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write URI contents to a SignedRequest
struct URIWriter;
impl URIWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a URI, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT URI TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// If present, indicates that the requester was successfully charged for the
/// request.
//NEEDS ENUM for RequestCharged
#[derive(Debug,PartialEq)]
pub enum RequestCharged {
	requester,
}
impl fmt::Display for RequestCharged {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			RequestCharged::requester => write!(f, "requester"),
		}
	}
}
impl Default for RequestCharged{
	fn default() -> RequestCharged{
		RequestCharged::requester
	}
}
impl From<String> for RequestCharged{
	fn from(string: String) -> RequestCharged{
		match string.as_ref() {
			"requester" => RequestCharged::requester,
			_ => RequestCharged::default(),
		}
	}
}

/// Parse RequestCharged from response
struct RequestChargedParser;
impl RequestChargedParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RequestCharged, XmlParseError> {
		println!("in RequestChargedParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : RequestCharged = RequestCharged::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = RequestCharged::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = RequestCharged::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write RequestCharged contents to a SignedRequest
struct RequestChargedWriter;
impl RequestChargedWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RequestCharged, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT RequestCharged TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct PutBucketLoggingRequest {
	pub bucket_logging_status: BucketLoggingStatus,
	pub content_md5: Option<ContentMD5>,
	pub bucket: BucketName,
}

/// Write PutBucketLoggingRequest contents to a SignedRequest
struct PutBucketLoggingRequestWriter;
impl PutBucketLoggingRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PutBucketLoggingRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = BucketLoggingStatusWriter::write_params(request, &obj.bucket_logging_status, Some(&ArgumentLocation::Body), &"BucketLoggingStatus".to_string());
		// optional writing for content_md5
		if let Some(ref obj) = obj.content_md5 {
			ContentMD5Writer::write_params(request, &obj, Some(&ArgumentLocation::Header), &"Content-MD5".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type Delimiter = String;

/// Parse Delimiter from response
struct DelimiterParser;
impl DelimiterParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Delimiter, XmlParseError> {
		println!("in DelimiterParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Delimiter contents to a SignedRequest
struct DelimiterWriter;
impl DelimiterWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Delimiter, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Delimiter TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MetadataValue = String;

/// Parse MetadataValue from response
struct MetadataValueParser;
impl MetadataValueParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MetadataValue, XmlParseError> {
		println!("in MetadataValueParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MetadataValue contents to a SignedRequest
struct MetadataValueWriter;
impl MetadataValueWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MetadataValue, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MetadataValue TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct LifecycleConfiguration {
	pub rules: Rules,
}


/// Parse LifecycleConfiguration from response
struct LifecycleConfigurationParser;
impl LifecycleConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LifecycleConfiguration, XmlParseError> {
		println!("in LifecycleConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = LifecycleConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Rules and child shape is Rules
			if current_name == "Rules"{
				obj.rules = try!(RulesParser::parse_response(Some(&"Rules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write LifecycleConfiguration contents to a SignedRequest
struct LifecycleConfigurationWriter;
impl LifecycleConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LifecycleConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = RulesWriter::write_params(request, &obj.rules, Some(&ArgumentLocation::Body), &"Rule".to_string());
		body
	}
}
pub type Expiration = String;

/// Parse Expiration from response
struct ExpirationParser;
impl ExpirationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<Expiration, XmlParseError> {
		println!("in ExpirationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write Expiration contents to a SignedRequest
struct ExpirationWriter;
impl ExpirationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a Expiration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT Expiration TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type IfMatch = String;

/// Parse IfMatch from response
struct IfMatchParser;
impl IfMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<IfMatch, XmlParseError> {
		println!("in IfMatchParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write IfMatch contents to a SignedRequest
struct IfMatchWriter;
impl IfMatchWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a IfMatch, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT IfMatch TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ResponseExpires = String;

/// Parse ResponseExpires from response
struct ResponseExpiresParser;
impl ResponseExpiresParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ResponseExpires, XmlParseError> {
		println!("in ResponseExpiresParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ResponseExpires contents to a SignedRequest
struct ResponseExpiresWriter;
impl ResponseExpiresWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ResponseExpires, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ResponseExpires TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct DeleteBucketTaggingRequest {
	pub bucket: BucketName,
}

/// Write DeleteBucketTaggingRequest contents to a SignedRequest
struct DeleteBucketTaggingRequestWriter;
impl DeleteBucketTaggingRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteBucketTaggingRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
/// Container for specifying the AWS Lambda notification configuration.
#[derive(Debug, Default)]
pub struct LambdaFunctionConfiguration {
	pub filter: Option<NotificationConfigurationFilter>,
	/// Lambda cloud function ARN that Amazon S3 can invoke when it detects events of
	/// the specified type.
	pub lambda_function_arn: LambdaFunctionArn,
	pub id: Option<NotificationId>,
	pub events: EventList,
}


/// Parse LambdaFunctionConfiguration from response
struct LambdaFunctionConfigurationParser;
impl LambdaFunctionConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<LambdaFunctionConfiguration, XmlParseError> {
		println!("in LambdaFunctionConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = LambdaFunctionConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Filter and child shape is NotificationConfigurationFilter
			if current_name == "Filter"{
				obj.filter = Some(try!(NotificationConfigurationFilterParser::parse_response(Some(&"Filter".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of LambdaFunctionArn and child shape is LambdaFunctionArn
			if current_name == "LambdaFunctionArn"{
				obj.lambda_function_arn = try!(LambdaFunctionArnParser::parse_response(Some(&"LambdaFunctionArn".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of Id and child shape is NotificationId
			if current_name == "Id"{
				obj.id = Some(try!(NotificationIdParser::parse_response(Some(&"Id".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Events and child shape is EventList
			if current_name == "Events"{
				obj.events = try!(EventListParser::parse_response(Some(&"Events".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write LambdaFunctionConfiguration contents to a SignedRequest
struct LambdaFunctionConfigurationWriter;
impl LambdaFunctionConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a LambdaFunctionConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for filter
		if let Some(ref obj) = obj.filter {
			body = NotificationConfigurationFilterWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Filter".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = LambdaFunctionArnWriter::write_params(request, &obj.lambda_function_arn, Some(&ArgumentLocation::Body), &"CloudFunction".to_string());
// GOES IN BODY, set here from return types?
		// optional writing for id
		if let Some(ref obj) = obj.id {
			body = NotificationIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"Id".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = EventListWriter::write_params(request, &obj.events, Some(&ArgumentLocation::Body), &"Event".to_string());
		body
	}
}
/// Requests Amazon S3 to encode the object keys in the response and specifies the
/// encoding method to use. An object key may contain any Unicode character;
/// however, XML 1.0 parser cannot parse some characters, such as characters with
/// an ASCII value from 0 to 10. For characters that are not supported in XML 1.0,
/// you can add this parameter to request that Amazon S3 encode the keys in the
/// response.
//NEEDS ENUM for EncodingType
#[derive(Debug,PartialEq)]
pub enum EncodingType {
	url,
}
impl fmt::Display for EncodingType {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			EncodingType::url => write!(f, "url"),
		}
	}
}
impl Default for EncodingType{
	fn default() -> EncodingType{
		EncodingType::url
	}
}
impl From<String> for EncodingType{
	fn from(string: String) -> EncodingType{
		match string.as_ref() {
			"url" => EncodingType::url,
			_ => EncodingType::default(),
		}
	}
}

/// Parse EncodingType from response
struct EncodingTypeParser;
impl EncodingTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<EncodingType, XmlParseError> {
		println!("in EncodingTypeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : EncodingType = EncodingType::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = EncodingType::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = EncodingType::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write EncodingType contents to a SignedRequest
struct EncodingTypeWriter;
impl EncodingTypeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a EncodingType, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT EncodingType TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ID = String;

/// Parse ID from response
struct IDParser;
impl IDParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ID, XmlParseError> {
		println!("in IDParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ID contents to a SignedRequest
struct IDWriter;
impl IDWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ID, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ID TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type PartNumberMarker = i32;

/// Parse PartNumberMarker from response
struct PartNumberMarkerParser;
impl PartNumberMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<PartNumberMarker, XmlParseError> {
		println!("in PartNumberMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write PartNumberMarker contents to a SignedRequest
struct PartNumberMarkerWriter;
impl PartNumberMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a PartNumberMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT PartNumberMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type DeleteMarkerVersionId = String;

/// Parse DeleteMarkerVersionId from response
struct DeleteMarkerVersionIdParser;
impl DeleteMarkerVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<DeleteMarkerVersionId, XmlParseError> {
		println!("in DeleteMarkerVersionIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write DeleteMarkerVersionId contents to a SignedRequest
struct DeleteMarkerVersionIdWriter;
impl DeleteMarkerVersionIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a DeleteMarkerVersionId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT DeleteMarkerVersionId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketWebsiteOutput {
	pub redirect_all_requests_to: RedirectAllRequestsTo,
	pub index_document: IndexDocument,
	pub error_document: ErrorDocument,
	pub routing_rules: RoutingRules,
}


/// Parse GetBucketWebsiteOutput from response
struct GetBucketWebsiteOutputParser;
impl GetBucketWebsiteOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketWebsiteOutput, XmlParseError> {
		println!("in GetBucketWebsiteOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketWebsiteOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of RedirectAllRequestsTo and child shape is RedirectAllRequestsTo
			if current_name == "RedirectAllRequestsTo"{
				obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_response(Some(&"RedirectAllRequestsTo".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of IndexDocument and child shape is IndexDocument
			if current_name == "IndexDocument"{
				obj.index_document = try!(IndexDocumentParser::parse_response(Some(&"IndexDocument".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of ErrorDocument and child shape is ErrorDocument
			if current_name == "ErrorDocument"{
				obj.error_document = try!(ErrorDocumentParser::parse_response(Some(&"ErrorDocument".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			//body parser for cname of RoutingRules and child shape is RoutingRules
			if current_name == "RoutingRules"{
				obj.routing_rules = try!(RoutingRulesParser::parse_response(Some(&"RoutingRules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
pub type CopySourceIfMatch = String;

/// Parse CopySourceIfMatch from response
struct CopySourceIfMatchParser;
impl CopySourceIfMatchParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CopySourceIfMatch, XmlParseError> {
		println!("in CopySourceIfMatchParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CopySourceIfMatch contents to a SignedRequest
struct CopySourceIfMatchWriter;
impl CopySourceIfMatchWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CopySourceIfMatch, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CopySourceIfMatch TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
//NEEDS ENUM for ReplicationRuleStatus
#[derive(Debug,PartialEq)]
pub enum ReplicationRuleStatus {
	Enabled,
	Disabled,
}
impl fmt::Display for ReplicationRuleStatus {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ReplicationRuleStatus::Enabled => write!(f, "Enabled"),
			ReplicationRuleStatus::Disabled => write!(f, "Disabled"),
		}
	}
}
impl Default for ReplicationRuleStatus{
	fn default() -> ReplicationRuleStatus{
		ReplicationRuleStatus::Enabled
	}
}
impl From<String> for ReplicationRuleStatus{
	fn from(string: String) -> ReplicationRuleStatus{
		match string.as_ref() {
			"Enabled" => ReplicationRuleStatus::Enabled,
			"Disabled" => ReplicationRuleStatus::Disabled,
			_ => ReplicationRuleStatus::default(),
		}
	}
}

/// Parse ReplicationRuleStatus from response
struct ReplicationRuleStatusParser;
impl ReplicationRuleStatusParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ReplicationRuleStatus, XmlParseError> {
		println!("in ReplicationRuleStatusParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : ReplicationRuleStatus = ReplicationRuleStatus::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = ReplicationRuleStatus::from(try!(characters(stack)));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = ReplicationRuleStatus::from(header_str);
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ReplicationRuleStatus contents to a SignedRequest
struct ReplicationRuleStatusWriter;
impl ReplicationRuleStatusWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ReplicationRuleStatus, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ReplicationRuleStatus TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ContentType = String;

/// Parse ContentType from response
struct ContentTypeParser;
impl ContentTypeParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ContentType, XmlParseError> {
		println!("in ContentTypeParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ContentType contents to a SignedRequest
struct ContentTypeWriter;
impl ContentTypeWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ContentType, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ContentType TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type ObjectVersionId = String;

/// Parse ObjectVersionId from response
struct ObjectVersionIdParser;
impl ObjectVersionIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
		println!("in ObjectVersionIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ObjectVersionId contents to a SignedRequest
struct ObjectVersionIdWriter;
impl ObjectVersionIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectVersionId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ObjectVersionId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
/// Container for object key name prefix and suffix filtering rules.
#[derive(Debug, Default)]
pub struct S3KeyFilter {
	pub filter_rules: FilterRuleList,
}


/// Parse S3KeyFilter from response
struct S3KeyFilterParser;
impl S3KeyFilterParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<S3KeyFilter, XmlParseError> {
		println!("in S3KeyFilterParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = S3KeyFilter::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of FilterRules and child shape is FilterRuleList
			if current_name == "FilterRules"{
				obj.filter_rules = try!(FilterRuleListParser::parse_response(Some(&"FilterRules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write S3KeyFilter contents to a SignedRequest
struct S3KeyFilterWriter;
impl S3KeyFilterWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a S3KeyFilter, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = FilterRuleListWriter::write_params(request, &obj.filter_rules, Some(&ArgumentLocation::Body), &"FilterRule".to_string());
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketLocationRequest {
	pub bucket: BucketName,
}

/// Write GetBucketLocationRequest contents to a SignedRequest
struct GetBucketLocationRequestWriter;
impl GetBucketLocationRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a GetBucketLocationRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
		body
	}
}
pub type NextPartNumberMarker = i32;

/// Parse NextPartNumberMarker from response
struct NextPartNumberMarkerParser;
impl NextPartNumberMarkerParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NextPartNumberMarker, XmlParseError> {
		println!("in NextPartNumberMarkerParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NextPartNumberMarker contents to a SignedRequest
struct NextPartNumberMarkerWriter;
impl NextPartNumberMarkerWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NextPartNumberMarker, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NextPartNumberMarker TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ListPartsRequest {
	pub request_payer: Option<RequestPayer>,
	pub bucket: BucketName,
	/// Upload ID identifying the multipart upload whose parts are being listed.
	pub upload_id: MultipartUploadId,
	pub key: ObjectKey,
	/// Sets the maximum number of parts to return.
	pub max_parts: Option<MaxParts>,
	/// Specifies the part after which listing should begin. Only parts with higher
	/// part numbers will be listed.
	pub part_number_marker: Option<PartNumberMarker>,
}

/// Write ListPartsRequest contents to a SignedRequest
struct ListPartsRequestWriter;
impl ListPartsRequestWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ListPartsRequest, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
		// optional writing for request_payer
		if let Some(ref obj) = obj.request_payer {
			RequestPayerWriter::write_params(request, &obj, Some(&ArgumentLocation::Header), &"x-amz-request-payer".to_string());
		}
//required field: 
		BucketNameWriter::write_params(request, &obj.bucket, Some(&ArgumentLocation::Uri), &"Bucket".to_string());
//required field: 
		MultipartUploadIdWriter::write_params(request, &obj.upload_id, Some(&ArgumentLocation::Querystring), &"uploadId".to_string());
//required field: 
		ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Uri), &"Key".to_string());
		// optional writing for max_parts
		if let Some(ref obj) = obj.max_parts {
			MaxPartsWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"max-parts".to_string());
		}
		// optional writing for part_number_marker
		if let Some(ref obj) = obj.part_number_marker {
			PartNumberMarkerWriter::write_params(request, &obj, Some(&ArgumentLocation::Querystring), &"part-number-marker".to_string());
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct RequestPaymentConfiguration {
	/// Specifies who pays for the download and request fees.
	pub payer: Payer,
}


/// Parse RequestPaymentConfiguration from response
struct RequestPaymentConfigurationParser;
impl RequestPaymentConfigurationParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<RequestPaymentConfiguration, XmlParseError> {
		println!("in RequestPaymentConfigurationParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = RequestPaymentConfiguration::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of Payer and child shape is Payer
			if current_name == "Payer"{
				obj.payer = try!(PayerParser::parse_response(Some(&"Payer".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write RequestPaymentConfiguration contents to a SignedRequest
struct RequestPaymentConfigurationWriter;
impl RequestPaymentConfigurationWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a RequestPaymentConfiguration, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
//required field: 
		body = PayerWriter::write_params(request, &obj.payer, Some(&ArgumentLocation::Body), &"Payer".to_string());
		body
	}
}
pub type ETag = String;

/// Parse ETag from response
struct ETagParser;
impl ETagParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ETag, XmlParseError> {
		println!("in ETagParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write ETag contents to a SignedRequest
struct ETagWriter;
impl ETagWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ETag, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT ETag TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct ObjectIdentifier {
	/// VersionId for the specific version of the object to delete.
	pub version_id: Option<ObjectVersionId>,
	/// Key name of the object to delete.
	pub key: ObjectKey,
}


/// Parse ObjectIdentifier from response
struct ObjectIdentifierParser;
impl ObjectIdentifierParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<ObjectIdentifier, XmlParseError> {
		println!("in ObjectIdentifierParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = ObjectIdentifier::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of VersionId and child shape is ObjectVersionId
			if current_name == "VersionId"{
				obj.version_id = Some(try!(ObjectVersionIdParser::parse_response(Some(&"VersionId".to_string()), Some(&ArgumentLocation::Body), headers, stack)));
				continue;
			}
			//body parser for cname of Key and child shape is ObjectKey
			if current_name == "Key"{
				obj.key = try!(ObjectKeyParser::parse_response(Some(&"Key".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// Write ObjectIdentifier contents to a SignedRequest
struct ObjectIdentifierWriter;
impl ObjectIdentifierWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a ObjectIdentifier, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
		// STRUCT_WRITER
// GOES IN BODY, set here from return types?
		// optional writing for version_id
		if let Some(ref obj) = obj.version_id {
			body = ObjectVersionIdWriter::write_params(request, &obj, Some(&ArgumentLocation::Body), &"VersionId".to_string());
		}
// GOES IN BODY, set here from return types?
//required field: 
		body = ObjectKeyWriter::write_params(request, &obj.key, Some(&ArgumentLocation::Body), &"Key".to_string());
		body
	}
}
/// Optional unique identifier for configurations in a notification configuration.
/// If you don't provide one, Amazon S3 will assign an ID.
pub type NotificationId = String;

/// Parse NotificationId from response
struct NotificationIdParser;
impl NotificationIdParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<NotificationId, XmlParseError> {
		println!("in NotificationIdParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write NotificationId contents to a SignedRequest
struct NotificationIdWriter;
impl NotificationIdWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a NotificationId, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is string
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT NotificationId TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
#[derive(Debug, Default)]
pub struct GetBucketCorsOutput {
	pub cors_rules: CORSRules,
}


/// Parse GetBucketCorsOutput from response
struct GetBucketCorsOutputParser;
impl GetBucketCorsOutputParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<GetBucketCorsOutput, XmlParseError> {
		println!("in GetBucketCorsOutputParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // struct_parser
		let mut obj = GetBucketCorsOutput::default();
		loop {
			let current_name = try!(peek_at_name(stack));
println!("current_name is {}", current_name);
			//body parser for cname of CORSRules and child shape is CORSRules
			if current_name == "CORSRules"{
				obj.cors_rules = try!(CORSRulesParser::parse_response(Some(&"CORSRules".to_string()), Some(&ArgumentLocation::Body), headers, stack));
				continue;
			}
			break;
		}
		Ok(obj)
	}
}
/// A list of containers for key value pair that defines the criteria for the
/// filter rule.
pub type FilterRuleList = Vec<FilterRule>;

/// Parse FilterRuleList from response
struct FilterRuleListParser;
impl FilterRuleListParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<FilterRuleList, XmlParseError> {
		println!("in FilterRuleListParser");
		 // list_parser
		let mut obj = Vec::new();
		stack.next(); // need to consume the first part of the XML to find what we're looking for.
		while try!(peek_at_name(stack)) == "FilterRule" {
//we need to iterate over members of FilterRule
// obj.push for FilterRule
			obj.push(try!(FilterRuleParser::parse_response(Some(&"FilterRule"), Some(&ArgumentLocation::Body), &headers, stack)));
		}
		Ok(obj)
	}
}
/// Write FilterRuleList contents to a SignedRequest
struct FilterRuleListWriter;
impl FilterRuleListWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a FilterRuleList, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// TODO: list_writer for FilterRule
;
		body
	}
}
pub type CreationDate = String;

/// Parse CreationDate from response
struct CreationDateParser;
impl CreationDateParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<CreationDate, XmlParseError> {
		println!("in CreationDateParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : String = String::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = try!(characters(stack));
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = header_str;
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write CreationDate contents to a SignedRequest
struct CreationDateWriter;
impl CreationDateWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a CreationDate, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is timestamp
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT CreationDate TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub type MaxParts = i32;

/// Parse MaxParts from response
struct MaxPartsParser;
impl MaxPartsParser {
	fn parse_response<'a, T: Peek + Next>(tag_name: Option<&str>, location: Option<&ArgumentLocation>, headers: &Headers, stack: &mut T) -> Result<MaxParts, XmlParseError> {
		println!("in MaxPartsParser");
		// check if tag_name is present (should be an Option type).  If so, read start element, otherwise forge ahead:
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(start_element(tag, stack)) ; },
					}
				}
			},
		}
		 // primitive_parser
		let mut obj : i32 = i32::default();
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Headers => (), // not yet implemented
					&ArgumentLocation::Body => {
						// primitive_parser
						obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
					},
					&ArgumentLocation::Header => {
						let header_str = try!(get_value_for_header(tag_name.unwrap(), headers));
						obj = try!(i32::from_str(&header_str));
					},
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => ()
					}
			}
		}
		match location {
			None => (),
			Some(loc) => {
				if loc == &ArgumentLocation::Body {
					match tag_name {
						None => (),
						Some(tag) => { try!(end_element(tag, stack)) ; },
					}
				}
			},
		}
		Ok(obj)
	}
}
/// Write MaxParts contents to a SignedRequest
struct MaxPartsWriter;
impl MaxPartsWriter {
	fn write_params<'a>(request: &mut SignedRequest<'a>, obj: &'a MaxParts, location: Option<&ArgumentLocation>, location_name: &str) -> Option<Vec<u8>> {
		let mut body : Option<Vec<u8>> = None;
// shape_type is integer
		match location{
			None => (), // noop
			Some(loc) => {
				match loc {
					&ArgumentLocation::Header => (),
					&ArgumentLocation::Body => {
//HEY PRINT MaxParts TO BODY (goes in return type)
						body = Some(obj.to_string().into_bytes());
					},
					&ArgumentLocation::Headers => (),
					&ArgumentLocation::Querystring => (),
					&ArgumentLocation::Uri => {
						// Bucket always goes in this format: bucketname.s3.amazonaws.com
						// TODO: can we optimize this out with codegen?
						if location_name == "Bucket" {
							request.set_hostname(Some(format!("{}.s3.amazonaws.com", obj)));
						} else {
							// Assume anything else in the URI goes in hostname/obj
							let existing_path = request.get_path();
							request.set_path(&format!("{}/{}", existing_path, obj));
						}
					},
				}
			},
		}
		body
	}
}
pub struct S3Client<'a> {
	creds: Box<AWSCredentialsProvider + 'a>,
	region: &'a Region
}

impl<'a> S3Client<'a> { 
	pub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> S3Client<'a> {
		S3Client { creds: Box::new(creds), region: region }
	}
	/// Returns metadata about all of the versions of objects in a bucket.
	pub fn list_object_versions(&mut self, input: &ListObjectVersionsRequest) -> Result<ListObjectVersionsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match ListObjectVersionsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(ListObjectVersionsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in list_object_versions"))
			 }
		}
	}
	/// Replaces a policy on a bucket. If the bucket already has a policy, the one in
	/// this request completely replaces it.
	pub fn put_bucket_policy(&mut self, input: &PutBucketPolicyRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketPolicyRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_policy"))
			 }
		}
	}
	/// Sets lifecycle configuration for your bucket. If a lifecycle configuration
	/// exists, it replaces it.
	pub fn put_bucket_lifecycle_configuration(&mut self, input: &PutBucketLifecycleConfigurationRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketLifecycleConfigurationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_lifecycle_configuration"))
			 }
		}
	}
	/// Returns some or all (up to 1000) of the objects in a bucket. You can use the
	/// request parameters as selection criteria to return a subset of the objects in
	/// a bucket.
	pub fn list_objects(&mut self, input: &ListObjectsRequest) -> Result<ListObjectsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match ListObjectsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(ListObjectsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in list_objects"))
			 }
		}
	}
	/// Set the website configuration for a bucket.
	pub fn put_bucket_website(&mut self, input: &PutBucketWebsiteRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketWebsiteRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_website"))
			 }
		}
	}
	/// Deprecated, see the PutBucketNotificationConfiguraiton operation.
	pub fn put_bucket_notification(&mut self, input: &PutBucketNotificationRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketNotificationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_notification"))
			 }
		}
	}
	/// Set the logging parameters for a bucket and to specify permissions for who can
	/// view and modify the logging parameters. To set the logging status of a bucket,
	/// you must be the bucket owner.
	pub fn put_bucket_logging(&mut self, input: &PutBucketLoggingRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketLoggingRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_logging"))
			 }
		}
	}
	/// Creates a new replication configuration (or replaces an existing one, if
	/// present).
	pub fn put_bucket_replication(&mut self, input: &PutBucketReplicationRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketReplicationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_replication"))
			 }
		}
	}
	/// Uploads a part in a multipart upload.
	/// **Note:** After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.
	pub fn upload_part(&mut self, input: &UploadPartRequest) -> Result<UploadPartOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match UploadPartRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(UploadPartOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in upload_part"))
			 }
		}
	}
	/// Adds an object to a bucket.
	pub fn put_object(&mut self, input: &PutObjectRequest) -> Result<PutObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(PutObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_object"))
			 }
		}
	}
	/// Deletes the cors configuration information set for the bucket.
	pub fn delete_bucket_cors(&mut self, input: &DeleteBucketCorsRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketCorsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_cors"))
			 }
		}
	}
	/// Sets the versioning state of an existing bucket. To set the versioning state,
	/// you must be the bucket owner.
	pub fn put_bucket_versioning(&mut self, input: &PutBucketVersioningRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketVersioningRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_versioning"))
			 }
		}
	}
	/// Returns the cors configuration for the bucket.
	pub fn get_bucket_cors(&mut self, input: &GetBucketCorsRequest) -> Result<GetBucketCorsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketCorsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketCorsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_cors"))
			 }
		}
	}
	/// Deprecated, see the PutBucketLifecycleConfiguration operation.
	pub fn put_bucket_lifecycle(&mut self, input: &PutBucketLifecycleRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketLifecycleRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_lifecycle"))
			 }
		}
	}
	/// Gets the access control policy for the bucket.
	pub fn get_bucket_acl(&mut self, input: &GetBucketAclRequest) -> Result<GetBucketAclOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketAclRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketAclOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_acl"))
			 }
		}
	}
	/// Returns the logging status of a bucket and the permissions users have to view
	/// and modify that status. To use GET, you must be the bucket owner.
	pub fn get_bucket_logging(&mut self, input: &GetBucketLoggingRequest) -> Result<GetBucketLoggingOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketLoggingRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketLoggingOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_logging"))
			 }
		}
	}
	/// This operation is useful to determine if a bucket exists and you have
	/// permission to access it.
	pub fn head_bucket(&mut self, input: &HeadBucketRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("HEAD", "s3", &self.region, &uri);

		match HeadBucketRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in head_bucket"))
			 }
		}
	}
	/// Sets the permissions on a bucket using access control lists (ACL).
	pub fn put_bucket_acl(&mut self, input: &PutBucketAclRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketAclRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_acl"))
			 }
		}
	}
	/// This operation removes the website configuration from the bucket.
	pub fn delete_bucket_website(&mut self, input: &DeleteBucketWebsiteRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketWebsiteRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_website"))
			 }
		}
	}
	/// Deletes the policy from the bucket.
	pub fn delete_bucket_policy(&mut self, input: &DeleteBucketPolicyRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketPolicyRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_policy"))
			 }
		}
	}
	/// Returns the notification configuration of a bucket.
	pub fn get_bucket_notification_configuration(&mut self, input: &GetBucketNotificationConfigurationRequest) -> Result<NotificationConfiguration, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketNotificationConfigurationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(NotificationConfigurationParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_notification_configuration"))
			 }
		}
	}
	/// This operation enables you to delete multiple objects from a bucket using a
	/// single HTTP request. You may specify up to 1000 keys.
	pub fn delete_objects(&mut self, input: &DeleteObjectsRequest) -> Result<DeleteObjectsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("POST", "s3", &self.region, &uri);

		match DeleteObjectsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(DeleteObjectsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_objects"))
			 }
		}
	}
	pub fn delete_bucket_replication(&mut self, input: &DeleteBucketReplicationRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketReplicationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_replication"))
			 }
		}
	}
	/// Creates a copy of an object that is already stored in Amazon S3.
	pub fn copy_object(&mut self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match CopyObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(CopyObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in copy_object"))
			 }
		}
	}
	/// Returns a list of all buckets owned by the authenticated sender of the
	/// request.
	pub fn list_buckets(&mut self) -> Result<ListBucketsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		//nothing to set
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(ListBucketsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in list_buckets"))
			 }
		}
	}
	/// Sets the request payment configuration for a bucket. By default, the bucket
	/// owner pays for downloads from the bucket. This configuration parameter enables
	/// the bucket owner (only) to specify that the person requesting the download
	/// will be charged for the download. Documentation on requester pays buckets can
	/// be found at
	/// http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html
	pub fn put_bucket_request_payment(&mut self, input: &PutBucketRequestPaymentRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketRequestPaymentRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_request_payment"))
			 }
		}
	}
	/// Enables notifications of specified events for a bucket.
	pub fn put_bucket_notification_configuration(&mut self, input: &PutBucketNotificationConfigurationRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketNotificationConfigurationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_notification_configuration"))
			 }
		}
	}
	/// The HEAD operation retrieves metadata from an object without returning the
	/// object itself. This operation is useful if you're only interested in an
	/// object's metadata. To use HEAD, you must have READ access to the object.
	pub fn head_object(&mut self, input: &HeadObjectRequest) -> Result<HeadObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("HEAD", "s3", &self.region, &uri);

		match HeadObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(HeadObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in head_object"))
			 }
		}
	}
	/// Deletes the tags from the bucket.
	pub fn delete_bucket_tagging(&mut self, input: &DeleteBucketTaggingRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketTaggingRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_tagging"))
			 }
		}
	}
	/// Return torrent files from a bucket.
	pub fn get_object_torrent(&mut self, input: &GetObjectTorrentRequest) -> Result<GetObjectTorrentOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetObjectTorrentRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetObjectTorrentOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_object_torrent"))
			 }
		}
	}
	/// Deprecated, see the GetBucketLifecycleConfiguration operation.
	pub fn get_bucket_lifecycle(&mut self, input: &GetBucketLifecycleRequest) -> Result<GetBucketLifecycleOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketLifecycleRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketLifecycleOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_lifecycle"))
			 }
		}
	}
	/// Creates a new bucket.
	pub fn create_bucket(&mut self, input: &CreateBucketRequest) -> Result<CreateBucketOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match CreateBucketRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(CreateBucketOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in create_bucket"))
			 }
		}
	}
	/// Completes a multipart upload by assembling previously uploaded parts.
	pub fn complete_multipart_upload(&mut self, input: &CompleteMultipartUploadRequest) -> Result<CompleteMultipartUploadOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("POST", "s3", &self.region, &uri);

		match CompleteMultipartUploadRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(CompleteMultipartUploadOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in complete_multipart_upload"))
			 }
		}
	}
	/// Returns the website configuration for a bucket.
	pub fn get_bucket_website(&mut self, input: &GetBucketWebsiteRequest) -> Result<GetBucketWebsiteOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketWebsiteRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketWebsiteOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_website"))
			 }
		}
	}
	/// Initiates a multipart upload and returns an upload ID.
	/// **Note:** After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.
	pub fn create_multipart_upload(&mut self, input: &CreateMultipartUploadRequest) -> Result<CreateMultipartUploadOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("POST", "s3", &self.region, &uri);

		match CreateMultipartUploadRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(CreateMultipartUploadOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in create_multipart_upload"))
			 }
		}
	}
	/// Deletes the bucket. All objects (including all object versions and Delete
	/// Markers) in the bucket must be deleted before the bucket itself can be
	/// deleted.
	pub fn delete_bucket(&mut self, input: &DeleteBucketRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket"))
			 }
		}
	}
	/// Retrieves objects from Amazon S3.
	pub fn get_object(&mut self, input: &GetObjectRequest) -> Result<GetObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_object"))
			 }
		}
	}
	/// Returns the policy of a specified bucket.
	pub fn get_bucket_policy(&mut self, input: &GetBucketPolicyRequest) -> Result<GetBucketPolicyOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketPolicyRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketPolicyOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_policy"))
			 }
		}
	}
	/// Returns the versioning state of a bucket.
	pub fn get_bucket_versioning(&mut self, input: &GetBucketVersioningRequest) -> Result<GetBucketVersioningOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketVersioningRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketVersioningOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_versioning"))
			 }
		}
	}
	/// This operation lists in-progress multipart uploads.
	pub fn list_multipart_uploads(&mut self, input: &ListMultipartUploadsRequest) -> Result<ListMultipartUploadsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match ListMultipartUploadsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(ListMultipartUploadsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in list_multipart_uploads"))
			 }
		}
	}
	/// Returns the lifecycle configuration information set on the bucket.
	pub fn get_bucket_lifecycle_configuration(&mut self, input: &GetBucketLifecycleConfigurationRequest) -> Result<GetBucketLifecycleConfigurationOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketLifecycleConfigurationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketLifecycleConfigurationOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_lifecycle_configuration"))
			 }
		}
	}
	/// Returns the request payment configuration of a bucket.
	pub fn get_bucket_request_payment(&mut self, input: &GetBucketRequestPaymentRequest) -> Result<GetBucketRequestPaymentOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketRequestPaymentRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketRequestPaymentOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_request_payment"))
			 }
		}
	}
	/// Sets the tags for a bucket.
	pub fn put_bucket_tagging(&mut self, input: &PutBucketTaggingRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketTaggingRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_tagging"))
			 }
		}
	}
	/// Returns the tag set associated with the bucket.
	pub fn get_bucket_tagging(&mut self, input: &GetBucketTaggingRequest) -> Result<GetBucketTaggingOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketTaggingRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketTaggingOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_tagging"))
			 }
		}
	}
	/// Aborts a multipart upload.
	/// To verify that all parts have been removed, so you don't get charged for the
	/// part storage, you should call the List Parts operation and ensure the parts
	/// list is empty.
	pub fn abort_multipart_upload(&mut self, input: &AbortMultipartUploadRequest) -> Result<AbortMultipartUploadOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match AbortMultipartUploadRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(AbortMultipartUploadOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in abort_multipart_upload"))
			 }
		}
	}
	/// uses the acl subresource to set the access control list (ACL) permissions for
	/// an object that already exists in a bucket
	pub fn put_object_acl(&mut self, input: &PutObjectAclRequest) -> Result<PutObjectAclOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutObjectAclRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(PutObjectAclOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_object_acl"))
			 }
		}
	}
	/// Returns the region the bucket resides in.
	pub fn get_bucket_location(&mut self, input: &GetBucketLocationRequest) -> Result<GetBucketLocationOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketLocationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketLocationOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_location"))
			 }
		}
	}
	/// Sets the cors configuration for a bucket.
	pub fn put_bucket_cors(&mut self, input: &PutBucketCorsRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match PutBucketCorsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in put_bucket_cors"))
			 }
		}
	}
	/// Deletes the lifecycle configuration from the bucket.
	pub fn delete_bucket_lifecycle(&mut self, input: &DeleteBucketLifecycleRequest) -> Result<(), AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteBucketLifecycleRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				Ok(())
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_bucket_lifecycle"))
			 }
		}
	}
	/// Deprecated, see the GetBucketNotificationConfiguration operation.
	pub fn get_bucket_notification(&mut self, input: &GetBucketNotificationConfigurationRequest) -> Result<NotificationConfigurationDeprecated, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketNotificationConfigurationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(NotificationConfigurationDeprecatedParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_notification"))
			 }
		}
	}
	/// Lists the parts that have been uploaded for a specific multipart upload.
	pub fn list_parts(&mut self, input: &ListPartsRequest) -> Result<ListPartsOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match ListPartsRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(ListPartsOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in list_parts"))
			 }
		}
	}
	/// Returns the access control list (ACL) of an object.
	pub fn get_object_acl(&mut self, input: &GetObjectAclRequest) -> Result<GetObjectAclOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetObjectAclRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetObjectAclOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_object_acl"))
			 }
		}
	}
	/// Uploads a part by copying data from an existing object as data source.
	pub fn upload_part_copy(&mut self, input: &UploadPartCopyRequest) -> Result<UploadPartCopyOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("PUT", "s3", &self.region, &uri);

		match UploadPartCopyRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(UploadPartCopyOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in upload_part_copy"))
			 }
		}
	}
	/// Removes the null version (if there is one) of an object and inserts a delete
	/// marker, which becomes the latest version of the object. If there isn't a null
	/// version, Amazon S3 does not remove any objects.
	pub fn delete_object(&mut self, input: &DeleteObjectRequest) -> Result<DeleteObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("DELETE", "s3", &self.region, &uri);

		match DeleteObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(DeleteObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in delete_object"))
			 }
		}
	}
	/// Restores an archived copy of an object back into Amazon S3
	pub fn restore_object(&mut self, input: &RestoreObjectRequest) -> Result<RestoreObjectOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("POST", "s3", &self.region, &uri);

		match RestoreObjectRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(RestoreObjectOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in restore_object"))
			 }
		}
	}
	pub fn get_bucket_replication(&mut self, input: &GetBucketReplicationRequest) -> Result<GetBucketReplicationOutput, AWSError> {
		let mut uri = String::from("");
		let mut request_body : Vec<u8>;
		let mut request = SignedRequest::new("GET", "s3", &self.region, &uri);

		match GetBucketReplicationRequestWriter::write_params(&mut request, &input, None, &"".to_string()) {
			None => (),
			Some(body_val) => {
				request_body = body_val;
				request.set_payload(Some(&request_body));
			},
		};

		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();

		match status {
			200 => { 
				let headers = result.headers.clone();
				let mut reader = EventReader::new(result);
				let mut stack = XmlResponseFromAws::new(reader.events().peekable());
				stack.next(); // xml start tag
				stack.next(); // top level result
				// tag name for top level doesn't matter:
				let aws_result = try!(GetBucketReplicationOutputParser::parse_response(None, None, &headers, &mut stack));
				Ok(aws_result)
			}
			_ => { 
				println!("Error: Status code was {}", status);
				let mut body = String::new();
				result.read_to_string(&mut body).unwrap();

				println!("Error response body: {}", body);
				Err(AWSError::new("S3 error in get_bucket_replication"))
			 }
		}
	}
}

