//! The AWS S3 API.

#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
#![allow(unused_variables, unused_mut)]

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;
use std::str;

use hyper::client::Response;
use openssl::crypto::hash::Type::MD5;
use openssl::crypto::hash::hash;
use rustc_serialize::base64::{ToBase64, STANDARD};
use xml::*;

use credential::ProvideAwsCredentials;
use error::AwsError;
use param::{Params, ServiceParams};
use region::Region;
use signature::SignedRequest;
use xmlutil::*;

#[derive(Debug, Default)]
pub struct LifecycleExpiration {
    /// Indicates at what date the object is to be moved or deleted. Should be in GMT
    /// ISO 8601 Format.
    pub date: Date,
    /// Indicates the lifetime, in days, of the objects that are subject to the rule.
    /// The value must be a non-zero positive integer.
    pub days: Days,
}

/// Parse `LifecycleExpiration` from XML
struct LifecycleExpirationParser;
impl LifecycleExpirationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LifecycleExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LifecycleExpiration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Date" => {
                    obj.date = try!(DateParser::parse_xml("Date", stack));
                    continue;
                },
                "Days" => {
                    obj.days = try!(DaysParser::parse_xml("Days", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LifecycleExpiration` contents to a `SignedRequest`
struct LifecycleExpirationWriter;
impl LifecycleExpirationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LifecycleExpiration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketNotificationRequest {
    pub notification_configuration: NotificationConfigurationDeprecated,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketNotificationRequest` from XML
struct PutBucketNotificationRequestParser;
impl PutBucketNotificationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketNotificationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketNotificationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "NotificationConfiguration" => {
                    obj.notification_configuration = try!(NotificationConfigurationDeprecatedParser::parse_xml("NotificationConfiguration", stack));
                    continue;
                },
                "Content-MD5" => {
                    obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                    continue;
                },
                "Bucket" => {
                    obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketNotificationRequest` contents to a `SignedRequest`
struct PutBucketNotificationRequestWriter;
impl PutBucketNotificationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketNotificationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        NotificationConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "NotificationConfiguration"), &obj.notification_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type Errors = Vec<String>;
/// Parse `Errors` from XML
struct ErrorsParser;
impl ErrorsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Errors, XmlParseError> {
        let mut obj : Vec<String> = Vec::new();
        while try!(peek_at_name(stack)) == "Error" {
            obj.push(try!(ErrorsParser::parse_single_error(stack)));
        }
        Ok(obj)
    }
    // hand crafted:
    fn parse_single_error<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
        // TODO: go back to try!

        match characters(stack) {
            Err(why) => Err(why),
            Ok(val) => Ok(val),
        }
    }
}
/// Write `Errors` contents to a `SignedRequest`
struct ErrorsWriter;
impl ErrorsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Errors) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ErrorsWriter::write_param(params, key, element);
            index += 1;
        }
    }
    // hand crafted:
    fn write_param(params: &mut Params, key: &str, value: &str) {
        params.put(key, value);
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

/// Parse `PutBucketVersioningRequest` from XML
struct PutBucketVersioningRequestParser;
impl PutBucketVersioningRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketVersioningRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketVersioningRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "VersioningConfiguration" {
                obj.versioning_configuration = try!(VersioningConfigurationParser::parse_xml("VersioningConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketVersioningRequest` contents to a `SignedRequest`
struct PutBucketVersioningRequestWriter;
impl PutBucketVersioningRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketVersioningRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        VersioningConfigurationWriter::write_params(params, &(prefix.to_string() + "VersioningConfiguration"), &obj.versioning_configuration);
    }
}
pub type CopySourceVersionId = String;
/// Parse `CopySourceVersionId` from XML
struct CopySourceVersionIdParser;
impl CopySourceVersionIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceVersionId` contents to a `SignedRequest`
struct CopySourceVersionIdWriter;
impl CopySourceVersionIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceVersionId) {
        params.put(name, obj);
    }
}
/// Container for specifying the configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Notification Service (Amazon SNS) topic.
#[derive(Debug, Default)]
pub struct TopicConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub topic_arn: TopicArn,
    pub events: EventList,
}

/// Parse `TopicConfiguration` from XML
struct TopicConfigurationParser;
impl TopicConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TopicConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Topic" {
                obj.topic_arn = try!(TopicArnParser::parse_xml("Topic", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TopicConfiguration` contents to a `SignedRequest`
struct TopicConfigurationWriter;
impl TopicConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TopicConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic_arn);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}
#[derive(Debug, Default)]
pub struct Destination {
    /// Amazon resource name (ARN) of the bucket where you want Amazon S3 to store
    /// replicas of the object identified by the rule.
    pub bucket: BucketName,
}

/// Parse `Destination` from XML
struct DestinationParser;
impl DestinationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Destination, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Destination::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Destination` contents to a `SignedRequest`
struct DestinationWriter;
impl DestinationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Destination) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type IfNoneMatch = String;
/// Parse `IfNoneMatch` from XML
struct IfNoneMatchParser;
impl IfNoneMatchParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfNoneMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IfNoneMatch` contents to a `SignedRequest`
struct IfNoneMatchWriter;
impl IfNoneMatchWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IfNoneMatch) {
        params.put(name, obj);
    }
}
/// The specified bucket does not exist.
#[derive(Debug, Default)]
pub struct NoSuchBucket;

/// Parse `NoSuchBucket` from XML
struct NoSuchBucketParser;
impl NoSuchBucketParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoSuchBucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoSuchBucket::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NoSuchBucket` contents to a `SignedRequest`
struct NoSuchBucketWriter;
impl NoSuchBucketWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NoSuchBucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
pub type ObjectVersionStorageClass = String;
/// Parse `ObjectVersionStorageClass` from XML
struct ObjectVersionStorageClassParser;
impl ObjectVersionStorageClassParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionStorageClass` contents to a `SignedRequest`
struct ObjectVersionStorageClassWriter;
impl ObjectVersionStorageClassWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionStorageClass) {
        params.put(name, obj);
    }
}
pub type MultipartUploadId = String;
/// Parse `MultipartUploadId` from XML
struct MultipartUploadIdParser;
impl MultipartUploadIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MultipartUploadId` contents to a `SignedRequest`
struct MultipartUploadIdWriter;
impl MultipartUploadIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadId) {
        params.put(name, obj);
    }
}
pub type Role = String;
/// Parse `Role` from XML
struct RoleParser;
impl RoleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Role, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Role` contents to a `SignedRequest`
struct RoleWriter;
impl RoleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Role) {
        params.put(name, obj);
    }
}
pub type WebsiteRedirectLocation = String;
/// Parse `WebsiteRedirectLocation` from XML
struct WebsiteRedirectLocationParser;
impl WebsiteRedirectLocationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<WebsiteRedirectLocation, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `WebsiteRedirectLocation` contents to a `SignedRequest`
struct WebsiteRedirectLocationWriter;
impl WebsiteRedirectLocationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &WebsiteRedirectLocation) {
        params.put(name, obj);
    }
}
pub type BucketVersioningStatus = String;
/// Parse `BucketVersioningStatus` from XML
struct BucketVersioningStatusParser;
impl BucketVersioningStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketVersioningStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketVersioningStatus` contents to a `SignedRequest`
struct BucketVersioningStatusWriter;
impl BucketVersioningStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketVersioningStatus) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketReplicationRequest {
    pub replication_configuration: ReplicationConfiguration,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketReplicationRequest` from XML
struct PutBucketReplicationRequestParser;
impl PutBucketReplicationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketReplicationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketReplicationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationConfiguration" {
                obj.replication_configuration = try!(ReplicationConfigurationParser::parse_xml("ReplicationConfiguration", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketReplicationRequest` contents to a `SignedRequest`
struct PutBucketReplicationRequestWriter;
impl PutBucketReplicationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketReplicationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationConfigurationWriter::write_params(params, &(prefix.to_string() + "ReplicationConfiguration"), &obj.replication_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct GetObjectTorrentRequest {
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

/// Parse `GetObjectTorrentRequest` from XML
struct GetObjectTorrentRequestParser;
impl GetObjectTorrentRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectTorrentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectTorrentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectTorrentRequest` contents to a `SignedRequest`
struct GetObjectTorrentRequestWriter;
impl GetObjectTorrentRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectTorrentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketReplicationRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketReplicationRequest` from XML
struct GetBucketReplicationRequestParser;
impl GetBucketReplicationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketReplicationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketReplicationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketReplicationRequest` contents to a `SignedRequest`
struct GetBucketReplicationRequestWriter;
impl GetBucketReplicationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketReplicationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
/// Container for the transition rule that describes when noncurrent objects
/// transition to the GLACIER storage class. If your bucket is versioning-enabled
/// (or versioning is suspended), you can set this action to request that Amazon
/// S3 transition noncurrent object versions to the GLACIER storage class at a
/// specific period in the object's lifetime.
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

/// Parse `NoncurrentVersionTransition` from XML
struct NoncurrentVersionTransitionParser;
impl NoncurrentVersionTransitionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoncurrentVersionTransition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoncurrentVersionTransition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "NoncurrentDays" {
                obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NoncurrentVersionTransition` contents to a `SignedRequest`
struct NoncurrentVersionTransitionWriter;
impl NoncurrentVersionTransitionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NoncurrentVersionTransition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
        TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketTaggingOutput {
    pub tag_set: TagSet,
}

/// Parse `GetBucketTaggingOutput` from XML
struct GetBucketTaggingOutputParser;
impl GetBucketTaggingOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketTaggingOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Tag" {
                obj.tag_set = try!(TagSetParser::parse_xml("Tag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketTaggingOutput` contents to a `SignedRequest`
struct GetBucketTaggingOutputWriter;
impl GetBucketTaggingOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketTaggingOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TagSetWriter::write_params(params, &(prefix.to_string() + "Tag"), &obj.tag_set);
    }
}
pub type Metadata = HashMap<MetadataKey,MetadataValue>;
/// Parse `Metadata` from XML
struct MetadataParser;
impl MetadataParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Metadata, XmlParseError> {
        let mut obj = HashMap::new();
        while try!(peek_at_name(stack)) == tag_name {
            try!(start_element(tag_name, stack));
            let key = try!(MetadataKeyParser::parse_xml("MetadataKey", stack));
            let value = try!(MetadataValueParser::parse_xml("MetadataValue", stack));
            obj.insert(key, value);
            try!(end_element(tag_name, stack));
        }
        Ok(obj)
    }
}
/// Write `Metadata` contents to a `SignedRequest`
struct MetadataWriter;
impl MetadataWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Metadata) {
        let mut index = 1;
        for (key,value) in obj {
            let prefix = &format!("{}.{}", name, index);
            MetadataKeyWriter::write_params(params, &format!("{}.{}", prefix, "MetadataKey"), key);
            MetadataValueWriter::write_params(params, &format!("{}.{}", prefix, "MetadataValue"), value);
            index += 1;
        }
    }
}
pub type Body = Vec<u8>;
/// Parse `Body` from XML
struct BodyParser;
impl BodyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Body, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack)).into_bytes();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Body` contents to a `SignedRequest`
struct BodyWriter;
impl BodyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Body) {
        params.put(name, str::from_utf8(obj).unwrap());
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

/// Parse `PutObjectOutput` from XML
struct PutObjectOutputParser;
impl PutObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutObjectOutput` contents to a `SignedRequest`
struct PutObjectOutputWriter;
impl PutObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}
#[derive(Debug, Default)]
pub struct PutObjectAclOutput {
    pub request_charged: RequestCharged,
}

/// Parse `PutObjectAclOutput` from XML
struct PutObjectAclOutputParser;
impl PutObjectAclOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutObjectAclOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutObjectAclOutput` contents to a `SignedRequest`
struct PutObjectAclOutputWriter;
impl PutObjectAclOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutObjectAclOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
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

/// Parse `ReplicationConfiguration` from XML
struct ReplicationConfigurationParser;
impl ReplicationConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ReplicationConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationRule" {
                obj.rules = try!(ReplicationRulesParser::parse_xml("ReplicationRule", stack));
                continue;
            }
            if current_name == "Role" {
                obj.role = try!(RoleParser::parse_xml("Role", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplicationConfiguration` contents to a `SignedRequest`
struct ReplicationConfigurationWriter;
impl ReplicationConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplicationConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationRulesWriter::write_params(params, &(prefix.to_string() + "ReplicationRule"), &obj.rules);
        RoleWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.role);
    }
}
/// Confirms that the requester knows that she or he will be charged for the
/// request. Bucket owners need not specify this parameter in their requests.
/// Documentation on downloading objects from requester pays buckets can be found
/// [here](http://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html).
pub type RequestPayer = String;
/// Parse `RequestPayer` from XML
struct RequestPayerParser;
impl RequestPayerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestPayer, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RequestPayer` contents to a `SignedRequest`
struct RequestPayerWriter;
impl RequestPayerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RequestPayer) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketWebsiteRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketWebsiteRequest` from XML
struct GetBucketWebsiteRequestParser;
impl GetBucketWebsiteRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketWebsiteRequest` contents to a `SignedRequest`
struct GetBucketWebsiteRequestWriter;
impl GetBucketWebsiteRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type Rules = Vec<Rule>;
/// Parse `Rules` from XML
struct RulesParser;
impl RulesParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Rule" {
            obj.push(try!(RuleParser::parse_xml("Rule", stack)));
        }
        Ok(obj)
    }
}
/// Write `Rules` contents to a `SignedRequest`
struct RulesWriter;
impl RulesWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Rules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            RuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct CompleteMultipartUploadRequest <'a> {
    pub multipart_upload: Option<&'a [u8]>,
    pub upload_id: MultipartUploadId,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct CreateBucketOutput {
    pub location: Location,
}

/// Parse `CreateBucketOutput` from XML
struct CreateBucketOutputParser;
impl CreateBucketOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateBucketOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateBucketOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Location" {
                obj.location = try!(LocationParser::parse_xml("Location", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreateBucketOutput` contents to a `SignedRequest`
struct CreateBucketOutputWriter;
impl CreateBucketOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreateBucketOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LocationWriter::write_params(params, &(prefix.to_string() + "Location"), &obj.location);
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

/// Parse `DeleteMarkerEntry` from XML
struct DeleteMarkerEntryParser;
impl DeleteMarkerEntryParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkerEntry, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteMarkerEntry::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteMarkerEntry` contents to a `SignedRequest`
struct DeleteMarkerEntryWriter;
impl DeleteMarkerEntryWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerEntry) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
    }
}
pub type TargetBucket = String;
/// Parse `TargetBucket` from XML
struct TargetBucketParser;
impl TargetBucketParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetBucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TargetBucket` contents to a `SignedRequest`
struct TargetBucketWriter;
impl TargetBucketWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TargetBucket) {
        params.put(name, obj);
    }
}
pub type MFADeleteStatus = String;
/// Parse `MFADeleteStatus` from XML
struct MFADeleteStatusParser;
impl MFADeleteStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFADeleteStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MFADeleteStatus` contents to a `SignedRequest`
struct MFADeleteStatusWriter;
impl MFADeleteStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MFADeleteStatus) {
        params.put(name, obj);
    }
}
pub type MaxKeys = i32;
/// Parse `MaxKeys` from XML
struct MaxKeysParser;
impl MaxKeysParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxKeys, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MaxKeys` contents to a `SignedRequest`
struct MaxKeysWriter;
impl MaxKeysWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MaxKeys) {
        params.put(name, &obj.to_string());
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

/// Parse `Part` from XML
struct PartParser;
impl PartParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Part, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Part::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "PartNumber" {
                obj.part_number = try!(PartNumberParser::parse_xml("PartNumber", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Part` contents to a `SignedRequest`
struct PartWriter;
impl PartWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Part) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        PartNumberWriter::write_params(params, &(prefix.to_string() + "PartNumber"), &obj.part_number);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}
pub type TargetGrants = Vec<TargetGrant>;
/// Parse `TargetGrants` from XML
struct TargetGrantsParser;
impl TargetGrantsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetGrants, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Grant" {
            obj.push(try!(TargetGrantParser::parse_xml("Grant", stack)));
        }
        Ok(obj)
    }
}
/// Write `TargetGrants` contents to a `SignedRequest`
struct TargetGrantsWriter;
impl TargetGrantsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TargetGrants) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TargetGrantWriter::write_params(params, key, element);
            index += 1;
        }
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

/// Parse `ListMultipartUploadsRequest` from XML
struct ListMultipartUploadsRequestParser;
impl ListMultipartUploadsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListMultipartUploadsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListMultipartUploadsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "upload-id-marker" {
                obj.upload_id_marker = Some(try!(UploadIdMarkerParser::parse_xml("upload-id-marker", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "key-marker" {
                obj.key_marker = Some(try!(KeyMarkerParser::parse_xml("key-marker", stack)));
                continue;
            }
            if current_name == "max-uploads" {
                obj.max_uploads = Some(try!(MaxUploadsParser::parse_xml("max-uploads", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListMultipartUploadsRequest` contents to a `SignedRequest`
struct ListMultipartUploadsRequestWriter;
impl ListMultipartUploadsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListMultipartUploadsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.upload_id_marker {
            UploadIdMarkerWriter::write_params(params, &(prefix.to_string() + "upload-id-marker"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.key_marker {
            KeyMarkerWriter::write_params(params, &(prefix.to_string() + "key-marker"), obj);
        }
        if let Some(ref obj) = obj.max_uploads {
            MaxUploadsWriter::write_params(params, &(prefix.to_string() + "max-uploads"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
    }
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketPolicyRequest` from XML
struct GetBucketPolicyRequestParser;
impl GetBucketPolicyRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketPolicyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketPolicyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketPolicyRequest` contents to a `SignedRequest`
struct GetBucketPolicyRequestWriter;
impl GetBucketPolicyRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type CloudFunction = String;
/// Parse `CloudFunction` from XML
struct CloudFunctionParser;
impl CloudFunctionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunction, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CloudFunction` contents to a `SignedRequest`
struct CloudFunctionWriter;
impl CloudFunctionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CloudFunction) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketWebsiteRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketWebsiteRequest` from XML
struct DeleteBucketWebsiteRequestParser;
impl DeleteBucketWebsiteRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketWebsiteRequest` contents to a `SignedRequest`
struct DeleteBucketWebsiteRequestWriter;
impl DeleteBucketWebsiteRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
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

/// Parse `VersioningConfiguration` from XML
struct VersioningConfigurationParser;
impl VersioningConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<VersioningConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = VersioningConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(BucketVersioningStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "MfaDelete" {
                obj.mfa_delete = try!(MFADeleteParser::parse_xml("MfaDelete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `VersioningConfiguration` contents to a `SignedRequest`
struct VersioningConfigurationWriter;
impl VersioningConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &VersioningConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketVersioningStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        MFADeleteWriter::write_params(params, &(prefix.to_string() + "MfaDelete"), &obj.mfa_delete);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketCorsRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketCorsRequest` from XML
struct GetBucketCorsRequestParser;
impl GetBucketCorsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketCorsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketCorsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketCorsRequest` contents to a `SignedRequest`
struct GetBucketCorsRequestWriter;
impl GetBucketCorsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketCorsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
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

/// Parse `IndexDocument` from XML
struct IndexDocumentParser;
impl IndexDocumentParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IndexDocument, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = IndexDocument::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Suffix" {
                obj.suffix = try!(SuffixParser::parse_xml("Suffix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IndexDocument` contents to a `SignedRequest`
struct IndexDocumentWriter;
impl IndexDocumentWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IndexDocument) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SuffixWriter::write_params(params, &(prefix.to_string() + "Suffix"), &obj.suffix);
    }
}
pub type TopicConfigurationList = Vec<TopicConfiguration>;
/// Parse `TopicConfigurationList` from XML
struct TopicConfigurationListParser;
impl TopicConfigurationListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "TopicConfiguration" {
            obj.push(try!(TopicConfigurationParser::parse_xml("TopicConfiguration", stack)));
        }
        Ok(obj)
    }
}
/// Write `TopicConfigurationList` contents to a `SignedRequest`
struct TopicConfigurationListWriter;
impl TopicConfigurationListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TopicConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TopicConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type ReplaceKeyPrefixWith = String;
/// Parse `ReplaceKeyPrefixWith` from XML
struct ReplaceKeyPrefixWithParser;
impl ReplaceKeyPrefixWithParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplaceKeyPrefixWith, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplaceKeyPrefixWith` contents to a `SignedRequest`
struct ReplaceKeyPrefixWithWriter;
impl ReplaceKeyPrefixWithWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplaceKeyPrefixWith) {
        params.put(name, obj);
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
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
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

/// Parse `CreateMultipartUploadRequest` from XML
struct CreateMultipartUploadRequestParser;
impl CreateMultipartUploadRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateMultipartUploadRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateMultipartUploadRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-acl" {
                obj.acl = Some(try!(ObjectCannedACLParser::parse_xml("x-amz-acl", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreateMultipartUploadRequest` contents to a `SignedRequest`
struct CreateMultipartUploadRequestWriter;
impl CreateMultipartUploadRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreateMultipartUploadRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.acl {
            ObjectCannedACLWriter::write_params(params, &(prefix.to_string() + "x-amz-acl"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}
#[derive(Debug, Default)]
pub struct PutBucketCorsRequest {
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    pub cors_configuration: Option<CORSConfiguration>,
}

/// Parse `PutBucketCorsRequest` from XML
struct PutBucketCorsRequestParser;
impl PutBucketCorsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketCorsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketCorsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "CORSConfiguration" {
                obj.cors_configuration = Some(try!(CORSConfigurationParser::parse_xml("CORSConfiguration", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketCorsRequest` contents to a `SignedRequest`
struct PutBucketCorsRequestWriter;
impl PutBucketCorsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketCorsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.cors_configuration {
            CORSConfigurationWriter::write_params(params, &(prefix.to_string() + "CORSConfiguration"), obj);
        }
    }
}
pub type CopySourceSSECustomerAlgorithm = String;
/// Parse `CopySourceSSECustomerAlgorithm` from XML
struct CopySourceSSECustomerAlgorithmParser;
impl CopySourceSSECustomerAlgorithmParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerAlgorithm, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceSSECustomerAlgorithm` contents to a `SignedRequest`
struct CopySourceSSECustomerAlgorithmWriter;
impl CopySourceSSECustomerAlgorithmWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerAlgorithm) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketAclOutput {
    pub owner: Owner,
    /// A list of grants.
    pub grants: Grants,
}

/// Parse `GetBucketAclOutput` from XML
struct GetBucketAclOutputParser;
impl GetBucketAclOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketAclOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketAclOutput` contents to a `SignedRequest`
struct GetBucketAclOutputWriter;
impl GetBucketAclOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketAclOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
    }
}
pub type Days = i32;
/// Parse `Days` from XML
struct DaysParser;
impl DaysParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Days, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Days` contents to a `SignedRequest`
struct DaysWriter;
impl DaysWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Days) {
        params.put(name, &obj.to_string());
    }
}
pub type Value = String;
/// Parse `Value` from XML
struct ValueParser;
impl ValueParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Value, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Value` contents to a `SignedRequest`
struct ValueWriter;
impl ValueWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Value) {
        params.put(name, obj);
    }
}
pub type DeletedObjects = Vec<DeletedObject>;
/// Parse `DeletedObjects` from XML
struct DeletedObjectsParser;
impl DeletedObjectsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObjects, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeletedObject" {
            obj.push(try!(DeletedObjectParser::parse_xml("DeletedObject", stack)));
        }
        Ok(obj)
    }
}
/// Write `DeletedObjects` contents to a `SignedRequest`
struct DeletedObjectsWriter;
impl DeletedObjectsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeletedObjects) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeletedObjectWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketLifecycleRequest` from XML
struct DeleteBucketLifecycleRequestParser;
impl DeleteBucketLifecycleRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketLifecycleRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketLifecycleRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketLifecycleRequest` contents to a `SignedRequest`
struct DeleteBucketLifecycleRequestWriter;
impl DeleteBucketLifecycleRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketLifecycleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct Tag {
    /// Value of the tag.
    pub value: Value,
    /// Name of the tag.
    pub key: ObjectKey,
}

/// Parse `Tag` from XML
struct TagParser;
impl TagParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Tag::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Value" {
                obj.value = try!(ValueParser::parse_xml("Value", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Tag` contents to a `SignedRequest`
struct TagWriter;
impl TagWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ValueWriter::write_params(params, &(prefix.to_string() + "Value"), &obj.value);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
pub type KeyMarker = String;
/// Parse `KeyMarker` from XML
struct KeyMarkerParser;
impl KeyMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<KeyMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = KeyMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `KeyMarker` contents to a `SignedRequest`
struct KeyMarkerWriter;
impl KeyMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &KeyMarker) {
        params.put(name, obj);
    }
}
pub type DeleteMarkers = Vec<DeleteMarkerEntry>;
/// Parse `DeleteMarkers` from XML
struct DeleteMarkersParser;
impl DeleteMarkersParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkers, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeleteMarkerEntry" {
            obj.push(try!(DeleteMarkerEntryParser::parse_xml("DeleteMarkerEntry", stack)));
        }
        Ok(obj)
    }
}
/// Write `DeleteMarkers` contents to a `SignedRequest`
struct DeleteMarkersWriter;
impl DeleteMarkersWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkers) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeleteMarkerEntryWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
/// Output from `AbortMultipartUpload` call
#[derive(Debug, Default)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: RequestCharged,
}

/// Write `AbortMultipartUploadOutput` contents to a `SignedRequest`
struct AbortMultipartUploadOutputWriter;
impl AbortMultipartUploadOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AbortMultipartUploadOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

/// Put bucket policy request
#[derive(Debug, Default)]
pub struct PutBucketPolicyRequest {
    /// The bucket policy as a JSON document.
    pub policy: Policy,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketPolicyRequest` from XML
struct PutBucketPolicyRequestParser;
impl PutBucketPolicyRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketPolicyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketPolicyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Policy" {
                obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketPolicyRequest` contents to a `SignedRequest`
struct PutBucketPolicyRequestWriter;
impl PutBucketPolicyRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PolicyWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policy);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type ResponseContentDisposition = String;
/// Parse `ResponseContentDisposition` from XML
struct ResponseContentDispositionParser;
impl ResponseContentDispositionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentDisposition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseContentDisposition` contents to a `SignedRequest`
struct ResponseContentDispositionWriter;
impl ResponseContentDispositionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseContentDisposition) {
        params.put(name, obj);
    }
}
pub type GrantFullControl = String;
/// Parse `GrantFullControl` from XML
struct GrantFullControlParser;
impl GrantFullControlParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantFullControl` contents to a `SignedRequest`
struct GrantFullControlWriter;
impl GrantFullControlWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantFullControl) {
        params.put(name, obj);
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

/// Parse `GetObjectAclRequest` from XML
struct GetObjectAclRequestParser;
impl GetObjectAclRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectAclRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectAclRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectAclRequest` contents to a `SignedRequest`
struct GetObjectAclRequestWriter;
impl GetObjectAclRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectAclRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
pub type SSECustomerAlgorithm = String;
/// Parse `SSECustomerAlgorithm` from XML
struct SSECustomerAlgorithmParser;
impl SSECustomerAlgorithmParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerAlgorithm, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `SSECustomerAlgorithm` contents to a `SignedRequest`
struct SSECustomerAlgorithmWriter;
impl SSECustomerAlgorithmWriter {
    fn write_params(params: &mut Params, name: &str, obj: &SSECustomerAlgorithm) {
        params.put(name, obj);
    }
}
pub type CommonPrefixList = Vec<CommonPrefix>;
/// Parse `CommonPrefixList` from XML
struct CommonPrefixListParser;
impl CommonPrefixListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CommonPrefixList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CommonPrefix" {
            obj.push(try!(CommonPrefixParser::parse_xml("CommonPrefix", stack)));
        }
        Ok(obj)
    }
}
/// Write `CommonPrefixList` contents to a `SignedRequest`
struct CommonPrefixListWriter;
impl CommonPrefixListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CommonPrefixList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CommonPrefixWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type Protocol = String;
/// Parse `Protocol` from XML
struct ProtocolParser;
impl ProtocolParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Protocol, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Protocol` contents to a `SignedRequest`
struct ProtocolWriter;
impl ProtocolWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Protocol) {
        params.put(name, obj);
    }
}
pub type Suffix = String;
/// Parse `Suffix` from XML
struct SuffixParser;
impl SuffixParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Suffix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Suffix` contents to a `SignedRequest`
struct SuffixWriter;
impl SuffixWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Suffix) {
        params.put(name, obj);
    }
}
pub type AllowedMethod = String;
/// Parse `AllowedMethod` from XML
struct AllowedMethodParser;
impl AllowedMethodParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedMethod, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AllowedMethod` contents to a `SignedRequest`
struct AllowedMethodWriter;
impl AllowedMethodWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedMethod) {
        params.put(name, obj);
    }
}
/// Container for specifying an configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Queue Service (Amazon SQS) queue.
#[derive(Debug, Default)]
pub struct QueueConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub queue_arn: QueueArn,
    pub events: EventList,
}

/// Parse `QueueConfiguration` from XML
struct QueueConfigurationParser;
impl QueueConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = QueueConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Queue" {
                obj.queue_arn = try!(QueueArnParser::parse_xml("Queue", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `QueueConfiguration` contents to a `SignedRequest`
struct QueueConfigurationWriter;
impl QueueConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &QueueConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue_arn);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}

pub type SSECustomerKey = String;
/// Parse `SSECustomerKey` from XML
struct SSECustomerKeyParser;
impl SSECustomerKeyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `SSECustomerKey` contents to a `SignedRequest`
struct SSECustomerKeyWriter;
impl SSECustomerKeyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &SSECustomerKey) {
        params.put(name, obj);
    }
}
pub type ObjectVersionId = String;
/// Parse `ObjectVersionId` from XML
struct ObjectVersionIdParser;
impl ObjectVersionIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionId` contents to a `SignedRequest`
struct ObjectVersionIdWriter;
impl ObjectVersionIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionId) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct HeadBucketRequest {
    pub bucket: BucketName,
}

/// Parse `HeadBucketRequest` from XML
struct HeadBucketRequestParser;
impl HeadBucketRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadBucketRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadBucketRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HeadBucketRequest` contents to a `SignedRequest`
struct HeadBucketRequestWriter;
impl HeadBucketRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HeadBucketRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct AbortMultipartUploadRequest {
    pub upload_id: MultipartUploadId,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

/// Parse `AbortMultipartUploadRequest` from XML
struct AbortMultipartUploadRequestParser;
impl AbortMultipartUploadRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AbortMultipartUploadRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AbortMultipartUploadRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "uploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("uploadId", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AbortMultipartUploadRequest` contents to a `SignedRequest`
struct AbortMultipartUploadRequestWriter;
impl AbortMultipartUploadRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AbortMultipartUploadRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "uploadId"), &obj.upload_id);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
pub type Size = i32;
/// Parse `Size` from XML
struct SizeParser;
impl SizeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Size, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Size` contents to a `SignedRequest`
struct SizeWriter;
impl SizeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Size) {
        params.put(name, &obj.to_string());
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

/// Parse `UploadPartCopyOutput` from XML
struct UploadPartCopyOutputParser;
impl UploadPartCopyOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<UploadPartCopyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = UploadPartCopyOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-version-id" {
                obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_xml("x-amz-copy-source-version-id", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "CopyPartResult" {
                obj.copy_part_result = try!(CopyPartResultParser::parse_xml("CopyPartResult", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `UploadPartCopyOutput` contents to a `SignedRequest`
struct UploadPartCopyOutputWriter;
impl UploadPartCopyOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &UploadPartCopyOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        CopySourceVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-version-id"), &obj.copy_source_version_id);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        CopyPartResultWriter::write_params(params, &(prefix.to_string() + "CopyPartResult"), &obj.copy_part_result);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
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

/// Parse `Redirect` from XML
struct RedirectParser;
impl RedirectParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Redirect, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Redirect::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplaceKeyWith" {
                obj.replace_key_with = try!(ReplaceKeyWithParser::parse_xml("ReplaceKeyWith", stack));
                continue;
            }
            if current_name == "HostName" {
                obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
                continue;
            }
            if current_name == "Protocol" {
                obj.protocol = try!(ProtocolParser::parse_xml("Protocol", stack));
                continue;
            }
            if current_name == "ReplaceKeyPrefixWith" {
                obj.replace_key_prefix_with = try!(ReplaceKeyPrefixWithParser::parse_xml("ReplaceKeyPrefixWith", stack));
                continue;
            }
            if current_name == "HttpRedirectCode" {
                obj.http_redirect_code = try!(HttpRedirectCodeParser::parse_xml("HttpRedirectCode", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Redirect` contents to a `SignedRequest`
struct RedirectWriter;
impl RedirectWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Redirect) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplaceKeyWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyWith"), &obj.replace_key_with);
        HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
        ProtocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
        ReplaceKeyPrefixWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyPrefixWith"), &obj.replace_key_prefix_with);
        HttpRedirectCodeWriter::write_params(params, &(prefix.to_string() + "HttpRedirectCode"), &obj.http_redirect_code);
    }
}
pub type CopySourceIfNoneMatch = String;
/// Parse `CopySourceIfNoneMatch` from XML
struct CopySourceIfNoneMatchParser;
impl CopySourceIfNoneMatchParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfNoneMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceIfNoneMatch` contents to a `SignedRequest`
struct CopySourceIfNoneMatchWriter;
impl CopySourceIfNoneMatchWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfNoneMatch) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketCorsRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketCorsRequest` from XML
struct DeleteBucketCorsRequestParser;
impl DeleteBucketCorsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketCorsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketCorsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketCorsRequest` contents to a `SignedRequest`
struct DeleteBucketCorsRequestWriter;
impl DeleteBucketCorsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketCorsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default)]
pub struct BucketAlreadyExists;

/// Parse `BucketAlreadyExists` from XML
struct BucketAlreadyExistsParser;
impl BucketAlreadyExistsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = BucketAlreadyExists::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketAlreadyExists` contents to a `SignedRequest`
struct BucketAlreadyExistsWriter;
impl BucketAlreadyExistsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketAlreadyExists) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
pub type BucketLocationConstraint = String;
/// Parse `BucketLocationConstraint` from XML
struct BucketLocationConstraintParser;
impl BucketLocationConstraintParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketLocationConstraint` contents to a `SignedRequest`
struct BucketLocationConstraintWriter;
impl BucketLocationConstraintWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketLocationConstraint) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketNotificationConfigurationRequest {
    /// Name of the buket to get the notification configuration for.
    pub bucket: BucketName,
}

/// Parse `GetBucketNotificationConfigurationRequest` from XML
struct GetBucketNotificationConfigurationRequestParser;
impl GetBucketNotificationConfigurationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketNotificationConfigurationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketNotificationConfigurationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketNotificationConfigurationRequest` contents to a `SignedRequest`
struct GetBucketNotificationConfigurationRequestWriter;
impl GetBucketNotificationConfigurationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketNotificationConfigurationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type NextKeyMarker = String;
/// Parse `NextKeyMarker` from XML
struct NextKeyMarkerParser;
impl NextKeyMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextKeyMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NextKeyMarker::default();

        if let Err(why) = characters(stack) {
            return Ok(obj); // swallow error, it's okay to be blank
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NextKeyMarker` contents to a `SignedRequest`
struct NextKeyMarkerWriter;
impl NextKeyMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NextKeyMarker) {
        params.put(name, obj);
    }
}
pub type AllowedMethods = Vec<AllowedMethod>;
/// Parse `AllowedMethods` from XML
struct AllowedMethodsParser;
impl AllowedMethodsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedMethods, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedMethod" {
            obj.push(try!(AllowedMethodParser::parse_xml("AllowedMethod", stack)));
        }
        Ok(obj)
    }
}
/// Write `AllowedMethods` contents to a `SignedRequest`
struct AllowedMethodsWriter;
impl AllowedMethodsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedMethods) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedMethodWriter::write_params(params, key, element);
            index += 1;
        }
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

/// Parse `DeleteObjectOutput` from XML
struct DeleteObjectOutputParser;
impl DeleteObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteObjectOutput` contents to a `SignedRequest`
struct DeleteObjectOutputWriter;
impl DeleteObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
    }
}
pub type VersionIdMarker = String;
/// Parse `VersionIdMarker` from XML
struct VersionIdMarkerParser;
impl VersionIdMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<VersionIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `VersionIdMarker` contents to a `SignedRequest`
struct VersionIdMarkerWriter;
impl VersionIdMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &VersionIdMarker) {
        params.put(name, obj);
    }
}
pub type StorageClass = String;
/// Parse `StorageClass` from XML
struct StorageClassParser;
impl StorageClassParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<StorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `StorageClass` contents to a `SignedRequest`
struct StorageClassWriter;
impl StorageClassWriter {
    fn write_params(params: &mut Params, name: &str, obj: &StorageClass) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
    /// Specifies the region where the bucket will be created. If you don't specify a
    /// region, the bucket will be created in US Standard.
    pub location_constraint: BucketLocationConstraint,
}

pub type BucketLogsPermission = String;
/// Parse `BucketLogsPermission` from XML
struct BucketLogsPermissionParser;
impl BucketLogsPermissionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLogsPermission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketLogsPermission` contents to a `SignedRequest`
struct BucketLogsPermissionWriter;
impl BucketLogsPermissionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketLogsPermission) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct HeadObjectRequest {
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
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

/// Parse `HeadObjectRequest` from XML
struct HeadObjectRequestParser;
impl HeadObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "If-Unmodified-Since" {
                obj.if_unmodified_since = Some(try!(IfUnmodifiedSinceParser::parse_xml("If-Unmodified-Since", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "If-None-Match" {
                obj.if_none_match = Some(try!(IfNoneMatchParser::parse_xml("If-None-Match", stack)));
                continue;
            }
            if current_name == "Range" {
                obj.range = Some(try!(RangeParser::parse_xml("Range", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "If-Match" {
                obj.if_match = Some(try!(IfMatchParser::parse_xml("If-Match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "If-Modified-Since" {
                obj.if_modified_since = Some(try!(IfModifiedSinceParser::parse_xml("If-Modified-Since", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HeadObjectRequest` contents to a `SignedRequest`
struct HeadObjectRequestWriter;
impl HeadObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HeadObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.if_unmodified_since {
            IfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Unmodified-Since"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.if_none_match {
            IfNoneMatchWriter::write_params(params, &(prefix.to_string() + "If-None-Match"), obj);
        }
        if let Some(ref obj) = obj.range {
            RangeWriter::write_params(params, &(prefix.to_string() + "Range"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.if_match {
            IfMatchWriter::write_params(params, &(prefix.to_string() + "If-Match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.if_modified_since {
            IfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Modified-Since"), obj);
        }
    }
}
pub type DisplayName = String;
/// Parse `DisplayName` from XML
struct DisplayNameParser;
impl DisplayNameParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DisplayName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DisplayName` contents to a `SignedRequest`
struct DisplayNameWriter;
impl DisplayNameWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DisplayName) {
        params.put(name, obj);
    }
}
pub type GrantReadACP = String;
/// Parse `GrantReadACP` from XML
struct GrantReadACPParser;
impl GrantReadACPParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantReadACP` contents to a `SignedRequest`
struct GrantReadACPWriter;
impl GrantReadACPWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantReadACP) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct Grant {
    pub grantee: Grantee,
    /// Specifies the permission given to the grantee.
    pub permission: Permission,
}

/// Parse `Grant` from XML
struct GrantParser;
impl GrantParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grant, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Grant::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Grantee" {
                obj.grantee = try!(GranteeParser::parse_xml("Grantee", stack));
                continue;
            }
            if current_name == "Permission" {
                obj.permission = try!(PermissionParser::parse_xml("Permission", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Grant` contents to a `SignedRequest`
struct GrantWriter;
impl GrantWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Grant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        PermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
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

/// Parse `TopicConfigurationDeprecated` from XML
struct TopicConfigurationDeprecatedParser;
impl TopicConfigurationDeprecatedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TopicConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Topic" {
                obj.topic = try!(TopicArnParser::parse_xml("Topic", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TopicConfigurationDeprecated` contents to a `SignedRequest`
struct TopicConfigurationDeprecatedWriter;
impl TopicConfigurationDeprecatedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TopicConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}
pub type CopySourceIfModifiedSince = String;
/// Parse `CopySourceIfModifiedSince` from XML
struct CopySourceIfModifiedSinceParser;
impl CopySourceIfModifiedSinceParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfModifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceIfModifiedSince` contents to a `SignedRequest`
struct CopySourceIfModifiedSinceWriter;
impl CopySourceIfModifiedSinceWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfModifiedSince) {
        params.put(name, obj);
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

/// Parse `Initiator` from XML
struct InitiatorParser;
impl InitiatorParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Initiator, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Initiator::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "DisplayName" {
                obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
                continue;
            }
            if current_name == "ID" {
                obj.id = try!(IDParser::parse_xml("ID", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Initiator` contents to a `SignedRequest`
struct InitiatorWriter;
impl InitiatorWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Initiator) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
        IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
    }
}
pub type HttpRedirectCode = String;
/// Parse `HttpRedirectCode` from XML
struct HttpRedirectCodeParser;
impl HttpRedirectCodeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HttpRedirectCode, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HttpRedirectCode` contents to a `SignedRequest`
struct HttpRedirectCodeWriter;
impl HttpRedirectCodeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HttpRedirectCode) {
        params.put(name, obj);
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

/// Parse `ListObjectVersionsRequest` from XML
struct ListObjectVersionsRequestParser;
impl ListObjectVersionsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectVersionsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectVersionsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "key-marker" {
                obj.key_marker = Some(try!(KeyMarkerParser::parse_xml("key-marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            if current_name == "version-id-marker" {
                obj.version_id_marker = Some(try!(VersionIdMarkerParser::parse_xml("version-id-marker", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListObjectVersionsRequest` contents to a `SignedRequest`
struct ListObjectVersionsRequestWriter;
impl ListObjectVersionsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListObjectVersionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.key_marker {
            KeyMarkerWriter::write_params(params, &(prefix.to_string() + "key-marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
        if let Some(ref obj) = obj.version_id_marker {
            VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "version-id-marker"), obj);
        }
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketRequest` from XML
struct DeleteBucketRequestParser;
impl DeleteBucketRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

pub type TargetPrefix = String;
/// Parse `TargetPrefix` from XML
struct TargetPrefixParser;
impl TargetPrefixParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TargetPrefix` contents to a `SignedRequest`
struct TargetPrefixWriter;
impl TargetPrefixWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TargetPrefix) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketPolicyRequest` from XML
struct DeleteBucketPolicyRequestParser;
impl DeleteBucketPolicyRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketPolicyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketPolicyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketPolicyRequest` contents to a `SignedRequest`
struct DeleteBucketPolicyRequestWriter;
impl DeleteBucketPolicyRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type CloudFunctionInvocationRole = String;
/// Parse `CloudFunctionInvocationRole` from XML
struct CloudFunctionInvocationRoleParser;
impl CloudFunctionInvocationRoleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunctionInvocationRole, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CloudFunctionInvocationRole` contents to a `SignedRequest`
struct CloudFunctionInvocationRoleWriter;
impl CloudFunctionInvocationRoleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CloudFunctionInvocationRole) {
        params.put(name, obj);
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

/// Parse `HeadObjectOutput` from XML
struct HeadObjectOutputParser;
impl HeadObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HeadObjectOutput` contents to a `SignedRequest`
struct HeadObjectOutputWriter;
impl HeadObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HeadObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketReplicationRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketReplicationRequest` from XML
struct DeleteBucketReplicationRequestParser;
impl DeleteBucketReplicationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketReplicationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketReplicationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketReplicationRequest` contents to a `SignedRequest`
struct DeleteBucketReplicationRequestWriter;
impl DeleteBucketReplicationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketReplicationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type PartNumber = i32;
/// Parse `PartNumber` from XML
struct PartNumberParser;
impl PartNumberParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PartNumber, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PartNumber` contents to a `SignedRequest`
struct PartNumberWriter;
impl PartNumberWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PartNumber) {
        params.put(name, &obj.to_string());
    }
}
pub type ExposeHeaders = Vec<ExposeHeader>;
/// Parse `ExposeHeaders` from XML
struct ExposeHeadersParser;
impl ExposeHeadersParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExposeHeaders, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ExposeHeader" {
            obj.push(try!(ExposeHeaderParser::parse_xml("ExposeHeader", stack)));
        }
        Ok(obj)
    }
}
/// Write `ExposeHeaders` contents to a `SignedRequest`
struct ExposeHeadersWriter;
impl ExposeHeadersWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ExposeHeaders) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ExposeHeaderWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: LoggingEnabled,
}

/// Parse `GetBucketLoggingOutput` from XML
struct GetBucketLoggingOutputParser;
impl GetBucketLoggingOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLoggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLoggingOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LoggingEnabled" {
                obj.logging_enabled = try!(LoggingEnabledParser::parse_xml("LoggingEnabled", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLoggingOutput` contents to a `SignedRequest`
struct GetBucketLoggingOutputWriter;
impl GetBucketLoggingOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLoggingOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LoggingEnabledWriter::write_params(params, &(prefix.to_string() + "LoggingEnabled"), &obj.logging_enabled);
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

/// Parse `ListObjectsRequest` from XML
struct ListObjectsRequestParser;
impl ListObjectsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "marker" {
                obj.marker = Some(try!(MarkerParser::parse_xml("marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListObjectsRequest` contents to a `SignedRequest`
struct ListObjectsRequestWriter;
impl ListObjectsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.marker {
            MarkerWriter::write_params(params, &(prefix.to_string() + "marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
    }
}
#[derive(Debug, Default)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: ReplicationConfiguration,
}

/// Parse `GetBucketReplicationOutput` from XML
struct GetBucketReplicationOutputParser;
impl GetBucketReplicationOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketReplicationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketReplicationOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationConfiguration" {
                obj.replication_configuration = try!(ReplicationConfigurationParser::parse_xml("ReplicationConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketReplicationOutput` contents to a `SignedRequest`
struct GetBucketReplicationOutputWriter;
impl GetBucketReplicationOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketReplicationOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationConfigurationWriter::write_params(params, &(prefix.to_string() + "ReplicationConfiguration"), &obj.replication_configuration);
    }
}
pub type Policy = String;
/// Parse `Policy` from XML
struct PolicyParser;
impl PolicyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Policy, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Policy` contents to a `SignedRequest`
struct PolicyWriter;
impl PolicyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Policy) {
        params.put(name, obj);
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

/// Parse `ListMultipartUploadsOutput` from XML
struct ListMultipartUploadsOutputParser;
impl ListMultipartUploadsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListMultipartUploadsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListMultipartUploadsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "UploadIdMarker" {
                obj.upload_id_marker = try!(UploadIdMarkerParser::parse_xml("UploadIdMarker", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            if current_name == "NextKeyMarker" {
                obj.next_key_marker = try!(NextKeyMarkerParser::parse_xml("NextKeyMarker", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "NextUploadIdMarker" {
                obj.next_upload_id_marker = try!(NextUploadIdMarkerParser::parse_xml("NextUploadIdMarker", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "MultipartUpload" {
                obj.uploads = try!(MultipartUploadListParser::parse_xml("MultipartUpload", stack));
                continue;
            }
            if current_name == "KeyMarker" {
                obj.key_marker = try!(KeyMarkerParser::parse_xml("KeyMarker", stack));
                continue;
            }
            if current_name == "MaxUploads" {
                obj.max_uploads = try!(MaxUploadsParser::parse_xml("MaxUploads", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "Upload" {
                obj.uploads.push(try!(MultipartUploadParser::parse_xml("Upload", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListMultipartUploadsOutput` contents to a `SignedRequest`
struct ListMultipartUploadsOutputWriter;
impl ListMultipartUploadsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListMultipartUploadsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        UploadIdMarkerWriter::write_params(params, &(prefix.to_string() + "UploadIdMarker"), &obj.upload_id_marker);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
        NextKeyMarkerWriter::write_params(params, &(prefix.to_string() + "NextKeyMarker"), &obj.next_key_marker);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        NextUploadIdMarkerWriter::write_params(params, &(prefix.to_string() + "NextUploadIdMarker"), &obj.next_upload_id_marker);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        MultipartUploadListWriter::write_params(params, &(prefix.to_string() + "MultipartUpload"), &obj.uploads);
        KeyMarkerWriter::write_params(params, &(prefix.to_string() + "KeyMarker"), &obj.key_marker);
        MaxUploadsWriter::write_params(params, &(prefix.to_string() + "MaxUploads"), &obj.max_uploads);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
    }
}
pub type IfUnmodifiedSince = String;
/// Parse `IfUnmodifiedSince` from XML
struct IfUnmodifiedSinceParser;
impl IfUnmodifiedSinceParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfUnmodifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IfUnmodifiedSince` contents to a `SignedRequest`
struct IfUnmodifiedSinceWriter;
impl IfUnmodifiedSinceWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IfUnmodifiedSince) {
        params.put(name, obj);
    }
}
pub type Permission = String;
/// Parse `Permission` from XML
struct PermissionParser;
impl PermissionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Permission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Permission` contents to a `SignedRequest`
struct PermissionWriter;
impl PermissionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Permission) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct RequestPaymentConfiguration {
    /// Specifies who pays for the download and request fees.
    pub payer: Payer,
}

/// Parse `RequestPaymentConfiguration` from XML
struct RequestPaymentConfigurationParser;
impl RequestPaymentConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestPaymentConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RequestPaymentConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Payer" {
                obj.payer = try!(PayerParser::parse_xml("Payer", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RequestPaymentConfiguration` contents to a `SignedRequest`
struct RequestPaymentConfigurationWriter;
impl RequestPaymentConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RequestPaymentConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PayerWriter::write_params(params, &(prefix.to_string() + "Payer"), &obj.payer);
    }
}
pub type Grants = Vec<Grant>;
/// Parse `Grants` from XML
struct GrantsParser;
impl GrantsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grants, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Grant" {
            obj.push(try!(GrantParser::parse_xml("Grant", stack)));
        }
        Ok(obj)
    }
}
/// Write `Grants` contents to a `SignedRequest`
struct GrantsWriter;
impl GrantsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Grants) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            GrantWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type ObjectStorageClass = String;
/// Parse `ObjectStorageClass` from XML
struct ObjectStorageClassParser;
impl ObjectStorageClassParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectStorageClass` contents to a `SignedRequest`
struct ObjectStorageClassWriter;
impl ObjectStorageClassWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectStorageClass) {
        params.put(name, obj);
    }
}
pub type EventList = Vec<Event>;
/// Parse `EventList` from XML
struct EventListParser;
impl EventListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EventList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Event" {
            obj.push(try!(EventParser::parse_xml("Event", stack)));
        }
        Ok(obj)
    }
}
/// Write `EventList` contents to a `SignedRequest`
struct EventListWriter;
impl EventListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &EventList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            EventWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type EmailAddress = String;
/// Parse `EmailAddress` from XML
struct EmailAddressParser;
impl EmailAddressParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EmailAddress, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `EmailAddress` contents to a `SignedRequest`
struct EmailAddressWriter;
impl EmailAddressWriter {
    fn write_params(params: &mut Params, name: &str, obj: &EmailAddress) {
        params.put(name, obj);
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

/// Parse `CreateMultipartUploadOutput` from XML
struct CreateMultipartUploadOutputParser;
impl CreateMultipartUploadOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateMultipartUploadOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreateMultipartUploadOutput` contents to a `SignedRequest`
struct CreateMultipartUploadOutputWriter;
impl CreateMultipartUploadOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreateMultipartUploadOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketWebsiteRequest {
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    pub website_configuration: WebsiteConfiguration,
}

/// Parse `PutBucketWebsiteRequest` from XML
struct PutBucketWebsiteRequestParser;
impl PutBucketWebsiteRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "WebsiteConfiguration" {
                obj.website_configuration = try!(WebsiteConfigurationParser::parse_xml("WebsiteConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketWebsiteRequest` contents to a `SignedRequest`
struct PutBucketWebsiteRequestWriter;
impl PutBucketWebsiteRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        WebsiteConfigurationWriter::write_params(params, &(prefix.to_string() + "WebsiteConfiguration"), &obj.website_configuration);
    }
}
pub type IsTruncated = bool;
/// Parse `IsTruncated` from XML
struct IsTruncatedParser;
impl IsTruncatedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IsTruncated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IsTruncated::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(ref chars) => obj = bool::from_str(chars).unwrap(),
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IsTruncated` contents to a `SignedRequest`
struct IsTruncatedWriter;
impl IsTruncatedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IsTruncated) {
        params.put(name, &obj.to_string());
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
    pub acl: Option<CannedAcl>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
}

pub type BucketName = String;
/// Parse `BucketName` from XML
struct BucketNameParser;
impl BucketNameParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketName` contents to a `SignedRequest`
struct BucketNameWriter;
impl BucketNameWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketName) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct TargetGrant {
    pub grantee: Grantee,
    /// Logging permissions assigned to the Grantee for the bucket.
    pub permission: BucketLogsPermission,
}

/// Parse `TargetGrant` from XML
struct TargetGrantParser;
impl TargetGrantParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetGrant, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TargetGrant::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Grantee" {
                obj.grantee = try!(GranteeParser::parse_xml("Grantee", stack));
                continue;
            }
            if current_name == "Permission" {
                obj.permission = try!(BucketLogsPermissionParser::parse_xml("Permission", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TargetGrant` contents to a `SignedRequest`
struct TargetGrantWriter;
impl TargetGrantWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TargetGrant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        BucketLogsPermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
    }
}
pub type MetadataDirective = String;
/// Parse `MetadataDirective` from XML
struct MetadataDirectiveParser;
impl MetadataDirectiveParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataDirective, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MetadataDirective` contents to a `SignedRequest`
struct MetadataDirectiveWriter;
impl MetadataDirectiveWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MetadataDirective) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketRequestPaymentRequest {
    pub request_payment_configuration: RequestPaymentConfiguration,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketRequestPaymentRequest` from XML
struct PutBucketRequestPaymentRequestParser;
impl PutBucketRequestPaymentRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketRequestPaymentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketRequestPaymentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RequestPaymentConfiguration" {
                obj.request_payment_configuration = try!(RequestPaymentConfigurationParser::parse_xml("RequestPaymentConfiguration", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketRequestPaymentRequest` contents to a `SignedRequest`
struct PutBucketRequestPaymentRequestWriter;
impl PutBucketRequestPaymentRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketRequestPaymentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestPaymentConfigurationWriter::write_params(params, &(prefix.to_string() + "RequestPaymentConfiguration"), &obj.request_payment_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
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
    pub acl: Option<CannedAcl>,
    pub access_control_policy: Option<AccessControlPolicy>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
}

/// Parse `PutObjectAclRequest` from XML
struct PutObjectAclRequestParser;
impl PutObjectAclRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutObjectAclRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutObjectAclRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "AccessControlPolicy" {
                obj.access_control_policy = Some(try!(AccessControlPolicyParser::parse_xml("AccessControlPolicy", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write" {
                obj.grant_write = Some(try!(GrantWriteParser::parse_xml("x-amz-grant-write", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutObjectAclRequest` contents to a `SignedRequest`
struct PutObjectAclRequestWriter;
impl PutObjectAclRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutObjectAclRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.access_control_policy {
            AccessControlPolicyWriter::write_params(params, &(prefix.to_string() + "AccessControlPolicy"), obj);
        }
        if let Some(ref obj) = obj.grant_write {
            GrantWriteWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write"), obj);
        }
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
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

/// Parse `CloudFunctionConfiguration` from XML
struct CloudFunctionConfigurationParser;
impl CloudFunctionConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CloudFunctionConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "InvocationRole" {
                obj.invocation_role = try!(CloudFunctionInvocationRoleParser::parse_xml("InvocationRole", stack));
                continue;
            }
            if current_name == "CloudFunction" {
                obj.cloud_function = try!(CloudFunctionParser::parse_xml("CloudFunction", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CloudFunctionConfiguration` contents to a `SignedRequest`
struct CloudFunctionConfigurationWriter;
impl CloudFunctionConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CloudFunctionConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CloudFunctionInvocationRoleWriter::write_params(params, &(prefix.to_string() + "InvocationRole"), &obj.invocation_role);
        CloudFunctionWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.cloud_function);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
    }
}
pub type LambdaFunctionArn = String;
/// Parse `LambdaFunctionArn` from XML
struct LambdaFunctionArnParser;
impl LambdaFunctionArnParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LambdaFunctionArn` contents to a `SignedRequest`
struct LambdaFunctionArnWriter;
impl LambdaFunctionArnWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionArn) {
        params.put(name, obj);
    }
}
pub type Quiet = bool;
/// Parse `Quiet` from XML
struct QuietParser;
impl QuietParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Quiet, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Quiet` contents to a `SignedRequest`
struct QuietWriter;
impl QuietWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Quiet) {
        params.put(name, &obj.to_string());
    }
}
#[derive(Debug, Default)]
pub struct AccessControlPolicy {
    pub owner: Owner,
    /// A list of grants.
    pub grants: Grants,
}

/// Parse `AccessControlPolicy` from XML
struct AccessControlPolicyParser;
impl AccessControlPolicyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AccessControlPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AccessControlPolicy::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AccessControlPolicy` contents to a `SignedRequest`
struct AccessControlPolicyWriter;
impl AccessControlPolicyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AccessControlPolicy) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
    }
}
pub type Range = String;
/// Parse `Range` from XML
struct RangeParser;
impl RangeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Range, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Range` contents to a `SignedRequest`
struct RangeWriter;
impl RangeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Range) {
        params.put(name, obj);
    }
}
pub type GrantRead = String;
/// Parse `GrantRead` from XML
struct GrantReadParser;
impl GrantReadParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantRead, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantRead` contents to a `SignedRequest`
struct GrantReadWriter;
impl GrantReadWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantRead) {
        params.put(name, obj);
    }
}
pub type SSECustomerKeyMD5 = String;
/// Parse `SSECustomerKeyMD`5 from XML
struct SSECustomerKeyMD5Parser;
impl SSECustomerKeyMD5Parser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerKeyMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `SSECustomerKeyMD5` contents to a `SignedRequest`
struct SSECustomerKeyMD5Writer;
impl SSECustomerKeyMD5Writer {
    fn write_params(params: &mut Params, name: &str, obj: &SSECustomerKeyMD5) {
        params.put(name, obj);
    }
}
pub type ObjectIdentifierList = Vec<ObjectIdentifier>;
/// Parse `ObjectIdentifierList` from XML
struct ObjectIdentifierListParser;
impl ObjectIdentifierListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifierList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ObjectIdentifier" {
            obj.push(try!(ObjectIdentifierParser::parse_xml("ObjectIdentifier", stack)));
        }
        Ok(obj)
    }
}
/// Write `ObjectIdentifierList` contents to a `SignedRequest`
struct ObjectIdentifierListWriter;
impl ObjectIdentifierListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifierList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectIdentifierWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type Restore = String;
/// Parse `Restore` from XML
struct RestoreParser;
impl RestoreParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Restore, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Restore` contents to a `SignedRequest`
struct RestoreWriter;
impl RestoreWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Restore) {
        params.put(name, obj);
    }
}
pub type NextVersionIdMarker = String;
/// Parse `NextVersionIdMarker` from XML
struct NextVersionIdMarkerParser;
impl NextVersionIdMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextVersionIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NextVersionIdMarker` contents to a `SignedRequest`
struct NextVersionIdMarkerWriter;
impl NextVersionIdMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NextVersionIdMarker) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct Delete {
    pub objects: ObjectIdentifierList,
    /// Element to enable quiet mode for the request. When you add this element, you
    /// must set its value to true.
    pub quiet: Option<Quiet>,
}

/// Parse `Delete` from XML
struct DeleteParser;
impl DeleteParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Delete, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Delete::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ObjectIdentifier" {
                obj.objects = try!(ObjectIdentifierListParser::parse_xml("ObjectIdentifier", stack));
                continue;
            }
            if current_name == "Quiet" {
                obj.quiet = Some(try!(QuietParser::parse_xml("Quiet", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Delete` contents to a `SignedRequest`
struct DeleteWriter;
impl DeleteWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Delete) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectIdentifierListWriter::write_params(params, &(prefix.to_string() + "ObjectIdentifier"), &obj.objects);
        if let Some(ref obj) = obj.quiet {
            QuietWriter::write_params(params, &(prefix.to_string() + "Quiet"), obj);
        }
    }
}
pub type ResponseContentLanguage = String;
/// Parse `ResponseContentLanguage` from XML
struct ResponseContentLanguageParser;
impl ResponseContentLanguageParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentLanguage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseContentLanguage` contents to a `SignedRequest`
struct ResponseContentLanguageWriter;
impl ResponseContentLanguageWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseContentLanguage) {
        params.put(name, obj);
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

/// Parse `ListObjectsOutput` from XML
struct ListObjectsOutputParser;
impl ListObjectsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            if current_name == "NextMarker" {
                obj.next_marker = try!(NextMarkerParser::parse_xml("NextMarker", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "MaxKeys" {
                obj.max_keys = try!(MaxKeysParser::parse_xml("MaxKeys", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "Marker" {
                obj.marker = try!(MarkerParser::parse_xml("Marker", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "Object" {
                obj.contents = try!(ObjectListParser::parse_xml("Object", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListObjectsOutput` contents to a `SignedRequest`
struct ListObjectsOutputWriter;
impl ListObjectsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListObjectsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
        NextMarkerWriter::write_params(params, &(prefix.to_string() + "NextMarker"), &obj.next_marker);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        MaxKeysWriter::write_params(params, &(prefix.to_string() + "MaxKeys"), &obj.max_keys);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        MarkerWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        ObjectListWriter::write_params(params, &(prefix.to_string() + "Object"), &obj.contents);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
    }
}
pub type GrantWriteACP = String;
/// Parse `GrantWriteACP` from XML
struct GrantWriteACPParser;
impl GrantWriteACPParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantWriteACP` contents to a `SignedRequest`
struct GrantWriteACPWriter;
impl GrantWriteACPWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantWriteACP) {
        params.put(name, obj);
    }
}
pub type CORSRules = Vec<CORSRule>;
/// Parse `CORSRules` from XML
struct CORSRulesParser;
impl CORSRulesParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CORSRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CORSRule" {
            obj.push(try!(CORSRuleParser::parse_xml("CORSRule", stack)));
        }
        Ok(obj)
    }
}
/// Write `CORSRules` contents to a `SignedRequest`
struct CORSRulesWriter;
impl CORSRulesWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CORSRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CORSRuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type ContentLanguage = String;
/// Parse `ContentLanguage` from XML
struct ContentLanguageParser;
impl ContentLanguageParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentLanguage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentLanguage` contents to a `SignedRequest`
struct ContentLanguageWriter;
impl ContentLanguageWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentLanguage) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct ListBucketsOutput {
    pub owner: Owner,
    pub buckets: Buckets,
}

/// Parse `ListBucketsOutput` from XML
struct ListBucketsOutputParser;
impl ListBucketsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListBucketsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListBucketsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Owner" => {
                    obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                    continue;
                },
                "Buckets" => {
                    stack.next(); // skip Buckets start and go to contents
                    // this will parse all buckets:
                    obj.buckets = try!(BucketsParser::parse_xml("Bucket", stack));
                },
                _ => break,
            }
        }
        stack.next(); // skip Buckets end
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListBucketsOutput` contents to a `SignedRequest`
struct ListBucketsOutputWriter;
impl ListBucketsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListBucketsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        BucketsWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.buckets);
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

/// Parse `DeleteObjectRequest` from XML
struct DeleteObjectRequestParser;
impl DeleteObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteObjectRequest` contents to a `SignedRequest`
struct DeleteObjectRequestWriter;
impl DeleteObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
pub type CompletedPartList = Vec<CompletedPart>;
/// Parse `CompletedPartList` from XML
struct CompletedPartListParser;
impl CompletedPartListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CompletedPartList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CompletedPart" {
            obj.push(try!(CompletedPartParser::parse_xml("CompletedPart", stack)));
        }
        Ok(obj)
    }
}
/// Write `CompletedPartList` contents to a `SignedRequest`
struct CompletedPartListWriter;
impl CompletedPartListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CompletedPartList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CompletedPartWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct DeletedObject {
    pub version_id: ObjectVersionId,
    pub delete_marker_version_id: DeleteMarkerVersionId,
    pub key: ObjectKey,
    pub delete_marker: DeleteMarker,
}

/// Parse `DeletedObject` from XML
struct DeletedObjectParser;
impl DeletedObjectParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObject, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeletedObject::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "DeleteMarkerVersionId" {
                obj.delete_marker_version_id = try!(DeleteMarkerVersionIdParser::parse_xml("DeleteMarkerVersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "DeleteMarker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("DeleteMarker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeletedObject` contents to a `SignedRequest`
struct DeletedObjectWriter;
impl DeletedObjectWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeletedObject) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        DeleteMarkerVersionIdWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerVersionId"), &obj.delete_marker_version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "DeleteMarker"), &obj.delete_marker);
    }
}
#[derive(Debug, Default)]
pub struct CORSRule {
    /// Specifies which headers are allowed in a pre-flight OPTIONS request.
    pub allowed_headers: AllowedHeaders,
    /// One or more headers in the response that you want customers to be able to
    /// access from their applications (for example, from a JavaScript XMLHttpRequest
    /// object).
    pub expose_headers: ExposeHeaders,
    /// Identifies HTTP methods that the domain/origin specified in the rule is
    /// allowed to execute.
    pub allowed_methods: AllowedMethods,
    /// The time in seconds that your browser is to cache the preflight response for
    /// the specified resource.
    pub max_age_seconds: MaxAgeSeconds,
    /// One or more origins you want customers to be able to access the bucket from.
    pub allowed_origins: AllowedOrigins,
}

/// Parse `CORSRule` from XML
struct CORSRuleParser;
impl CORSRuleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CORSRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CORSRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "AllowedHeader" {
                obj.allowed_headers = try!(AllowedHeadersParser::parse_xml("AllowedHeader", stack));
                continue;
            }
            if current_name == "ExposeHeader" {
                obj.expose_headers = try!(ExposeHeadersParser::parse_xml("ExposeHeader", stack));
                continue;
            }
            if current_name == "AllowedMethod" {
                obj.allowed_methods = try!(AllowedMethodsParser::parse_xml("AllowedMethod", stack));
                continue;
            }
            if current_name == "MaxAgeSeconds" {
                obj.max_age_seconds = try!(MaxAgeSecondsParser::parse_xml("MaxAgeSeconds", stack));
                continue;
            }
            if current_name == "AllowedOrigin" {
                obj.allowed_origins = try!(AllowedOriginsParser::parse_xml("AllowedOrigin", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CORSRule` contents to a `SignedRequest`
struct CORSRuleWriter;
impl CORSRuleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CORSRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        AllowedHeadersWriter::write_params(params, &(prefix.to_string() + "AllowedHeader"), &obj.allowed_headers);
        ExposeHeadersWriter::write_params(params, &(prefix.to_string() + "ExposeHeader"), &obj.expose_headers);
        AllowedMethodsWriter::write_params(params, &(prefix.to_string() + "AllowedMethod"), &obj.allowed_methods);
        MaxAgeSecondsWriter::write_params(params, &(prefix.to_string() + "MaxAgeSeconds"), &obj.max_age_seconds);
        AllowedOriginsWriter::write_params(params, &(prefix.to_string() + "AllowedOrigin"), &obj.allowed_origins);
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

/// Parse `LoggingEnabled` from XML
struct LoggingEnabledParser;
impl LoggingEnabledParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LoggingEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LoggingEnabled::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "TargetPrefix" {
                obj.target_prefix = try!(TargetPrefixParser::parse_xml("TargetPrefix", stack));
                continue;
            }
            if current_name == "TargetBucket" {
                obj.target_bucket = try!(TargetBucketParser::parse_xml("TargetBucket", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.target_grants = try!(TargetGrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LoggingEnabled` contents to a `SignedRequest`
struct LoggingEnabledWriter;
impl LoggingEnabledWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LoggingEnabled) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TargetPrefixWriter::write_params(params, &(prefix.to_string() + "TargetPrefix"), &obj.target_prefix);
        TargetBucketWriter::write_params(params, &(prefix.to_string() + "TargetBucket"), &obj.target_bucket);
        TargetGrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.target_grants);
    }
}
pub type KeyPrefixEquals = String;
/// Parse `KeyPrefixEquals` from XML
struct KeyPrefixEqualsParser;
impl KeyPrefixEqualsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<KeyPrefixEquals, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `KeyPrefixEquals` contents to a `SignedRequest`
struct KeyPrefixEqualsWriter;
impl KeyPrefixEqualsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &KeyPrefixEquals) {
        params.put(name, obj);
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

/// Parse `RedirectAllRequestsTo` from XML
struct RedirectAllRequestsToParser;
impl RedirectAllRequestsToParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RedirectAllRequestsTo, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RedirectAllRequestsTo::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "HostName" {
                obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
                continue;
            }
            if current_name == "Protocol" {
                obj.protocol = Some(try!(ProtocolParser::parse_xml("Protocol", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RedirectAllRequestsTo` contents to a `SignedRequest`
struct RedirectAllRequestsToWriter;
impl RedirectAllRequestsToWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RedirectAllRequestsTo) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
        if let Some(ref obj) = obj.protocol {
            ProtocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), obj);
        }
    }
}
#[derive(Debug, Default)]
pub struct Owner {
    pub display_name: DisplayName,
    pub id: ID,
}

/// Parse `Owner` from XML
struct OwnerParser;
impl OwnerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Owner, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Owner::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "DisplayName" => {
                    obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
                    continue;
                },
                "ID" => {
                    obj.id = try!(IDParser::parse_xml("ID", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Owner` contents to a `SignedRequest`
struct OwnerWriter;
impl OwnerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Owner) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
        IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
    }
}
#[derive(Debug, Default)]
pub struct CopyObjectResult {
    pub last_modified: LastModified,
    pub e_tag: ETag,
}

/// Parse `CopyObjectResult` from XML
struct CopyObjectResultParser;
impl CopyObjectResultParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectResult::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopyObjectResult` contents to a `SignedRequest`
struct CopyObjectResultWriter;
impl CopyObjectResultWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopyObjectResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
    }
}
pub type S3ClientMessage = String;
/// Parse `S`3ClientMessage from XML
struct S3ClientMessageParser;
impl S3ClientMessageParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<S3ClientMessage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `S3ClientMessage` contents to a `SignedRequest`
struct S3ClientMessageWriter;
impl S3ClientMessageWriter {
    fn write_params(params: &mut Params, name: &str, obj: &S3ClientMessage) {
        params.put(name, obj);
    }
}
pub type ObjectList = Vec<Object>;
/// Parse `ObjectList` from XML
struct ObjectListParser;
impl ObjectListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Object" {
            obj.push(try!(ObjectParser::parse_xml("Object", stack)));
        }
        Ok(obj)
    }
}
/// Write `ObjectList` contents to a `SignedRequest`
struct ObjectListWriter;
impl ObjectListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleOutput {
    pub rules: Rules,
}

/// Parse `GetBucketLifecycleOutput` from XML
struct GetBucketLifecycleOutputParser;
impl GetBucketLifecycleOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLifecycleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLifecycleOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Rule" {
                obj.rules = try!(RulesParser::parse_xml("Rule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLifecycleOutput` contents to a `SignedRequest`
struct GetBucketLifecycleOutputWriter;
impl GetBucketLifecycleOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLifecycleOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RulesWriter::write_params(params, &(prefix.to_string() + "Rule"), &obj.rules);
    }
}
/// Bucket event for which to send notifications.
pub type Event = String;
/// Parse `Event` from XML
struct EventParser;
impl EventParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Event, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Event` contents to a `SignedRequest`
struct EventWriter;
impl EventWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Event) {
        params.put(name, obj);
    }
}
pub type ReplicationRules = Vec<ReplicationRule>;
/// Parse `ReplicationRules` from XML
struct ReplicationRulesParser;
impl ReplicationRulesParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ReplicationRule" {
            obj.push(try!(ReplicationRuleParser::parse_xml("ReplicationRule", stack)));
        }
        Ok(obj)
    }
}
/// Write `ReplicationRules` contents to a `SignedRequest`
struct ReplicationRulesWriter;
impl ReplicationRulesWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplicationRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ReplicationRuleWriter::write_params(params, key, element);
            index += 1;
        }
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

/// Parse `NotificationConfiguration` from XML
struct NotificationConfigurationParser;
impl NotificationConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NotificationConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "QueueConfiguration" {
                obj.queue_configurations = try!(QueueConfigurationListParser::parse_xml("QueueConfiguration", stack));
                continue;
            }
            if current_name == "LambdaFunctionConfiguration" {
                obj.lambda_function_configurations = try!(LambdaFunctionConfigurationListParser::parse_xml("LambdaFunctionConfiguration", stack));
                continue;
            }
            if current_name == "TopicConfiguration" {
                obj.topic_configurations = try!(TopicConfigurationListParser::parse_xml("TopicConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NotificationConfiguration` contents to a `SignedRequest`
struct NotificationConfigurationWriter;
impl NotificationConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NotificationConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        QueueConfigurationListWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configurations);
        LambdaFunctionConfigurationListWriter::write_params(params, &(prefix.to_string() + "LambdaFunctionConfiguration"), &obj.lambda_function_configurations);
        TopicConfigurationListWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configurations);
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

/// Parse `Object` from XML
struct ObjectParser;
impl ObjectParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Object, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Object::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Object` contents to a `SignedRequest`
struct ObjectWriter;
impl ObjectWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Object) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}
pub type NextMarker = String;
/// Parse `NextMarker` from XML
struct NextMarkerParser;
impl NextMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NextMarker` contents to a `SignedRequest`
struct NextMarkerWriter;
impl NextMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NextMarker) {
        params.put(name, obj);
    }
}
pub type ContentEncoding = String;
/// Parse `ContentEncoding` from XML
struct ContentEncodingParser;
impl ContentEncodingParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentEncoding, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentEncoding` contents to a `SignedRequest`
struct ContentEncodingWriter;
impl ContentEncodingWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentEncoding) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct WebsiteConfiguration {
    pub redirect_all_requests_to: RedirectAllRequestsTo,
    pub index_document: IndexDocument,
    pub error_document: ErrorDocument,
    pub routing_rules: RoutingRules,
}

/// Parse `WebsiteConfiguration` from XML
struct WebsiteConfigurationParser;
impl WebsiteConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<WebsiteConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = WebsiteConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RedirectAllRequestsTo" {
                obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_xml("RedirectAllRequestsTo", stack));
                continue;
            }
            if current_name == "IndexDocument" {
                obj.index_document = try!(IndexDocumentParser::parse_xml("IndexDocument", stack));
                continue;
            }
            if current_name == "ErrorDocument" {
                obj.error_document = try!(ErrorDocumentParser::parse_xml("ErrorDocument", stack));
                continue;
            }
            if current_name == "RoutingRule" {
                obj.routing_rules = try!(RoutingRulesParser::parse_xml("RoutingRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `WebsiteConfiguration` contents to a `SignedRequest`
struct WebsiteConfigurationWriter;
impl WebsiteConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &WebsiteConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RedirectAllRequestsToWriter::write_params(params, &(prefix.to_string() + "RedirectAllRequestsTo"), &obj.redirect_all_requests_to);
        IndexDocumentWriter::write_params(params, &(prefix.to_string() + "IndexDocument"), &obj.index_document);
        ErrorDocumentWriter::write_params(params, &(prefix.to_string() + "ErrorDocument"), &obj.error_document);
        RoutingRulesWriter::write_params(params, &(prefix.to_string() + "RoutingRule"), &obj.routing_rules);
    }
}
pub type MFADelete = String;
/// Parse `MFADelete` from XML
struct MFADeleteParser;
impl MFADeleteParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFADelete, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MFADelete` contents to a `SignedRequest`
struct MFADeleteWriter;
impl MFADeleteWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MFADelete) {
        params.put(name, obj);
    }
}
pub type CopySourceSSECustomerKey = String;
/// Parse `CopySourceSSECustomerKey` from XML
struct CopySourceSSECustomerKeyParser;
impl CopySourceSSECustomerKeyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceSSECustomerKey` contents to a `SignedRequest`
struct CopySourceSSECustomerKeyWriter;
impl CopySourceSSECustomerKeyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerKey) {
        params.put(name, obj);
    }
}
pub type ResponseContentType = String;
/// Parse `ResponseContentType` from XML
struct ResponseContentTypeParser;
impl ResponseContentTypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseContentType` contents to a `SignedRequest`
struct ResponseContentTypeWriter;
impl ResponseContentTypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseContentType) {
        params.put(name, obj);
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

/// Parse `CompleteMultipartUploadOutput` from XML
struct CompleteMultipartUploadOutputParser;
impl CompleteMultipartUploadOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CompleteMultipartUploadOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "Location" {
                obj.location = try!(LocationParser::parse_xml("Location", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CompleteMultipartUploadOutput` contents to a `SignedRequest`
struct CompleteMultipartUploadOutputWriter;
impl CompleteMultipartUploadOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CompleteMultipartUploadOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        LocationWriter::write_params(params, &(prefix.to_string() + "Location"), &obj.location);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
    }
}
pub type ExposeHeader = String;
/// Parse `ExposeHeader` from XML
struct ExposeHeaderParser;
impl ExposeHeaderParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExposeHeader, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ExposeHeader` contents to a `SignedRequest`
struct ExposeHeaderWriter;
impl ExposeHeaderWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ExposeHeader) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct CopyPartResult {
    /// Date and time at which the object was uploaded.
    pub last_modified: LastModified,
    /// Entity tag of the object.
    pub e_tag: ETag,
}

/// Parse `CopyPartResult` from XML
struct CopyPartResultParser;
impl CopyPartResultParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyPartResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyPartResult::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopyPartResult` contents to a `SignedRequest`
struct CopyPartResultWriter;
impl CopyPartResultWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopyPartResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketAclRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketAclRequest` from XML
struct GetBucketAclRequestParser;
impl GetBucketAclRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketAclRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketAclRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketAclRequest` contents to a `SignedRequest`
struct GetBucketAclRequestWriter;
impl GetBucketAclRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketAclRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type HostName = String;
/// Parse `HostName` from XML
struct HostNameParser;
impl HostNameParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HostName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HostName` contents to a `SignedRequest`
struct HostNameWriter;
impl HostNameWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HostName) {
        params.put(name, obj);
    }
}
pub type NextUploadIdMarker = String;
/// Parse `NextUploadIdMarker` from XML
struct NextUploadIdMarkerParser;
impl NextUploadIdMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextUploadIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NextUploadIdMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NextUploadIdMarker` contents to a `SignedRequest`
struct NextUploadIdMarkerWriter;
impl NextUploadIdMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NextUploadIdMarker) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct CopyObjectOutput {
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    pub copy_source_version_id: CopySourceVersionId,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    pub request_charged: RequestCharged,
    /// If the object expiration is configured, the response includes this header.
    pub expiration: Expiration,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    pub copy_object_result: CopyObjectResult,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
}

/// Parse `CopyObjectOutput` from XML
struct CopyObjectOutputParser;
impl CopyObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-version-id" {
                obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_xml("x-amz-copy-source-version-id", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "CopyObjectResult" {
                obj.copy_object_result = try!(CopyObjectResultParser::parse_xml("CopyObjectResult", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopyObjectOutput` contents to a `SignedRequest`
struct CopyObjectOutputWriter;
impl CopyObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopyObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        CopySourceVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-version-id"), &obj.copy_source_version_id);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        CopyObjectResultWriter::write_params(params, &(prefix.to_string() + "CopyObjectResult"), &obj.copy_object_result);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}
pub type AcceptRanges = String;
/// Parse `AcceptRanges` from XML
struct AcceptRangesParser;
impl AcceptRangesParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AcceptRanges, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AcceptRanges` contents to a `SignedRequest`
struct AcceptRangesWriter;
impl AcceptRangesWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AcceptRanges) {
        params.put(name, obj);
    }
}
/// This operation is not allowed against this storage tier
#[derive(Debug, Default)]
pub struct ObjectAlreadyInActiveTierError;

/// Parse `ObjectAlreadyInActiveTierError` from XML
struct ObjectAlreadyInActiveTierErrorParser;
impl ObjectAlreadyInActiveTierErrorParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectAlreadyInActiveTierError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectAlreadyInActiveTierError::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectAlreadyInActiveTierError` contents to a `SignedRequest`
struct ObjectAlreadyInActiveTierErrorWriter;
impl ObjectAlreadyInActiveTierErrorWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectAlreadyInActiveTierError) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
#[derive(Debug, Default)]
pub struct CompletedMultipartUpload {
    pub parts: CompletedPartList,
}

/// Parse `CompletedMultipartUpload` from XML
struct CompletedMultipartUploadParser;
impl CompletedMultipartUploadParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CompletedMultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CompletedMultipartUpload::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CompletedPart" {
                obj.parts = try!(CompletedPartListParser::parse_xml("CompletedPart", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CompletedMultipartUpload` contents to a `SignedRequest`
struct CompletedMultipartUploadWriter;
impl CompletedMultipartUploadWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CompletedMultipartUpload) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CompletedPartListWriter::write_params(params, &(prefix.to_string() + "CompletedPart"), &obj.parts);
    }
}
pub type Initiated = String;
/// Parse `Initiated` from XML
struct InitiatedParser;
impl InitiatedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Initiated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Initiated` contents to a `SignedRequest`
struct InitiatedWriter;
impl InitiatedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Initiated) {
        params.put(name, obj);
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
    pub acl: Option<CannedAcl>,
    pub access_control_policy: Option<AccessControlPolicy>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
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

/// Parse `UploadPartOutput` from XML
struct UploadPartOutputParser;
impl UploadPartOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<UploadPartOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = UploadPartOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `UploadPartOutput` contents to a `SignedRequest`
struct UploadPartOutputWriter;
impl UploadPartOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &UploadPartOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}
pub type CopySource = String;
/// Parse `CopySource` from XML
struct CopySourceParser;
impl CopySourceParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySource, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySource` contents to a `SignedRequest`
struct CopySourceWriter;
impl CopySourceWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySource) {
        params.put(name, obj);
    }
}
pub type QueueConfigurationList = Vec<QueueConfiguration>;
/// Parse `QueueConfigurationList` from XML
struct QueueConfigurationListParser;
impl QueueConfigurationListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "QueueConfiguration" {
            obj.push(try!(QueueConfigurationParser::parse_xml("QueueConfiguration", stack)));
        }
        Ok(obj)
    }
}
/// Write `QueueConfigurationList` contents to a `SignedRequest`
struct QueueConfigurationListWriter;
impl QueueConfigurationListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &QueueConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            QueueConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
/// The source object of the COPY operation is not in the active tier and is only
/// stored in Amazon Glacier.
#[derive(Debug, Default)]
pub struct ObjectNotInActiveTierError;

/// Parse `ObjectNotInActiveTierError` from XML
struct ObjectNotInActiveTierErrorParser;
impl ObjectNotInActiveTierErrorParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectNotInActiveTierError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectNotInActiveTierError::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectNotInActiveTierError` contents to a `SignedRequest`
struct ObjectNotInActiveTierErrorWriter;
impl ObjectNotInActiveTierErrorWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectNotInActiveTierError) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
pub type TransitionStorageClass = String;
/// Parse `TransitionStorageClass` from XML
struct TransitionStorageClassParser;
impl TransitionStorageClassParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TransitionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TransitionStorageClass` contents to a `SignedRequest`
struct TransitionStorageClassWriter;
impl TransitionStorageClassWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TransitionStorageClass) {
        params.put(name, obj);
    }
}
pub type DeleteMarker = bool;
/// Parse `DeleteMarker` from XML
struct DeleteMarkerParser;
impl DeleteMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteMarker` contents to a `SignedRequest`
struct DeleteMarkerWriter;
impl DeleteMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteMarker) {
        params.put(name, &obj.to_string());
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

/// Parse `Rule` from XML
struct RuleParser;
impl RuleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Rule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(ExpirationStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "NoncurrentVersionExpiration" {
                obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationParser::parse_xml("NoncurrentVersionExpiration", stack)));
                continue;
            }
            if current_name == "Transition" {
                obj.transition = Some(try!(TransitionParser::parse_xml("Transition", stack)));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "Expiration" {
                obj.expiration = Some(try!(LifecycleExpirationParser::parse_xml("Expiration", stack)));
                continue;
            }
            if current_name == "NoncurrentVersionTransition" {
                obj.noncurrent_version_transition = Some(try!(NoncurrentVersionTransitionParser::parse_xml("NoncurrentVersionTransition", stack)));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Rule` contents to a `SignedRequest`
struct RuleWriter;
impl RuleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Rule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ExpirationStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        if let Some(ref obj) = obj.noncurrent_version_expiration {
            NoncurrentVersionExpirationWriter::write_params(params, &(prefix.to_string() + "NoncurrentVersionExpiration"), obj);
        }
        if let Some(ref obj) = obj.transition {
            TransitionWriter::write_params(params, &(prefix.to_string() + "Transition"), obj);
        }
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        if let Some(ref obj) = obj.expiration {
            LifecycleExpirationWriter::write_params(params, &(prefix.to_string() + "Expiration"), obj);
        }
        if let Some(ref obj) = obj.noncurrent_version_transition {
            NoncurrentVersionTransitionWriter::write_params(params, &(prefix.to_string() + "NoncurrentVersionTransition"), obj);
        }
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
    }
}
pub type RoutingRules = Vec<RoutingRule>;
/// Parse `RoutingRules` from XML
struct RoutingRulesParser;
impl RoutingRulesParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RoutingRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "RoutingRule" {
            obj.push(try!(RoutingRuleParser::parse_xml("RoutingRule", stack)));
        }
        Ok(obj)
    }
}
/// Write `RoutingRules` contents to a `SignedRequest`
struct RoutingRulesWriter;
impl RoutingRulesWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RoutingRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            RoutingRuleWriter::write_params(params, key, element);
            index += 1;
        }
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

/// Parse `ReplicationRule` from XML
struct ReplicationRuleParser;
impl ReplicationRuleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ReplicationRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(ReplicationRuleStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "Destination" {
                obj.destination = try!(DestinationParser::parse_xml("Destination", stack));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplicationRule` contents to a `SignedRequest`
struct ReplicationRuleWriter;
impl ReplicationRuleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplicationRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationRuleStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        DestinationWriter::write_params(params, &(prefix.to_string() + "Destination"), &obj.destination);
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
    }
}
pub type Date = String;
/// Parse `Date` from XML
struct DateParser;
impl DateParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Date, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Date` contents to a `SignedRequest`
struct DateWriter;
impl DateWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Date) {
        params.put(name, obj);
    }
}
pub type CacheControl = String;
/// Parse `CacheControl` from XML
struct CacheControlParser;
impl CacheControlParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CacheControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CacheControl` contents to a `SignedRequest`
struct CacheControlWriter;
impl CacheControlWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CacheControl) {
        params.put(name, obj);
    }
}
pub type AllowedOrigin = String;
/// Parse `AllowedOrigin` from XML
struct AllowedOriginParser;
impl AllowedOriginParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedOrigin, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AllowedOrigin` contents to a `SignedRequest`
struct AllowedOriginWriter;
impl AllowedOriginWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedOrigin) {
        params.put(name, obj);
    }
}
pub type IfModifiedSince = String;
/// Parse `IfModifiedSince` from XML
struct IfModifiedSinceParser;
impl IfModifiedSinceParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfModifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IfModifiedSince` contents to a `SignedRequest`
struct IfModifiedSinceWriter;
impl IfModifiedSinceWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IfModifiedSince) {
        params.put(name, obj);
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

/// Parse `Condition` from XML
struct ConditionParser;
impl ConditionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Condition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Condition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "HttpErrorCodeReturnedEquals" {
                obj.http_error_code_returned_equals = try!(HttpErrorCodeReturnedEqualsParser::parse_xml("HttpErrorCodeReturnedEquals", stack));
                continue;
            }
            if current_name == "KeyPrefixEquals" {
                obj.key_prefix_equals = try!(KeyPrefixEqualsParser::parse_xml("KeyPrefixEquals", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Condition` contents to a `SignedRequest`
struct ConditionWriter;
impl ConditionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Condition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        HttpErrorCodeReturnedEqualsWriter::write_params(params, &(prefix.to_string() + "HttpErrorCodeReturnedEquals"), &obj.http_error_code_returned_equals);
        KeyPrefixEqualsWriter::write_params(params, &(prefix.to_string() + "KeyPrefixEquals"), &obj.key_prefix_equals);
    }
}
#[derive(Debug, Default)]
pub struct DeleteObjectsOutput {
    pub deleted: DeletedObjects,
    pub errors: Errors,
    pub request_charged: RequestCharged,
}

/// Parse `DeleteObjectsOutput` from XML
struct DeleteObjectsOutputParser;
impl DeleteObjectsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "DeletedObject" {
                obj.deleted = try!(DeletedObjectsParser::parse_xml("DeletedObject", stack));
                continue;
            }
            if current_name == "Error" {
                obj.errors = try!(ErrorsParser::parse_xml("Error", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteObjectsOutput` contents to a `SignedRequest`
struct DeleteObjectsOutputWriter;
impl DeleteObjectsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DeletedObjectsWriter::write_params(params, &(prefix.to_string() + "DeletedObject"), &obj.deleted);
        ErrorsWriter::write_params(params, &(prefix.to_string() + "Error"), &obj.errors);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}
#[derive(Debug, Default)]
pub struct ErrorDocument {
    /// The object key name to use when a 4XX class error occurs.
    pub key: ObjectKey,
}

/// Parse `ErrorDocument` from XML
struct ErrorDocumentParser;
impl ErrorDocumentParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ErrorDocument, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ErrorDocument::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ErrorDocument` contents to a `SignedRequest`
struct ErrorDocumentWriter;
impl ErrorDocumentWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ErrorDocument) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
pub type Payer = String;
/// Parse `Payer` from XML
struct PayerParser;
impl PayerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Payer, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Payer` contents to a `SignedRequest`
struct PayerWriter;
impl PayerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Payer) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketLifecycleRequest {
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketLifecycleRequest` from XML
struct PutBucketLifecycleRequestParser;
impl PutBucketLifecycleRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketLifecycleRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketLifecycleRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LifecycleConfiguration" {
                obj.lifecycle_configuration = Some(try!(LifecycleConfigurationParser::parse_xml("LifecycleConfiguration", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketLifecycleRequest` contents to a `SignedRequest`
struct PutBucketLifecycleRequestWriter;
impl PutBucketLifecycleRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketLifecycleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.lifecycle_configuration {
            LifecycleConfigurationWriter::write_params(params, &(prefix.to_string() + "LifecycleConfiguration"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct GetObjectTorrentOutput {
    pub body: Body,
    pub request_charged: RequestCharged,
}

/// Parse `GetObjectTorrentOutput` from XML
struct GetObjectTorrentOutputParser;
impl GetObjectTorrentOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectTorrentOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectTorrentOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Body" {
                obj.body = try!(BodyParser::parse_xml("Body", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectTorrentOutput` contents to a `SignedRequest`
struct GetObjectTorrentOutputWriter;
impl GetObjectTorrentOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectTorrentOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BodyWriter::write_params(params, &(prefix.to_string() + "Body"), &obj.body);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}
pub type ContentLength = i32;
/// Parse `ContentLength` from XML
struct ContentLengthParser;
impl ContentLengthParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentLength, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentLength` contents to a `SignedRequest`
struct ContentLengthWriter;
impl ContentLengthWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentLength) {
        params.put(name, &obj.to_string());
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

/// Parse `Transition` from XML
struct TransitionParser;
impl TransitionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Transition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Transition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Date" {
                obj.date = try!(DateParser::parse_xml("Date", stack));
                continue;
            }
            if current_name == "Days" {
                obj.days = try!(DaysParser::parse_xml("Days", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Transition` contents to a `SignedRequest`
struct TransitionWriter;
impl TransitionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Transition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
        TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
    }
}
#[derive(Debug, Default)]
pub struct QueueConfigurationDeprecated {
    pub queue: QueueArn,
    pub events: EventList,
    pub id: NotificationId,
    pub event: Event,
}

/// Parse `QueueConfigurationDeprecated` from XML
struct QueueConfigurationDeprecatedParser;
impl QueueConfigurationDeprecatedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = QueueConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Queue" {
                obj.queue = try!(QueueArnParser::parse_xml("Queue", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `QueueConfigurationDeprecated` contents to a `SignedRequest`
struct QueueConfigurationDeprecatedWriter;
impl QueueConfigurationDeprecatedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &QueueConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
    }
}
#[derive(Debug, Default)]
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

/// Parse `GetObjectOutput` from XML
struct GetObjectOutputParser;
impl GetObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "Content-Range" {
                obj.content_range = try!(ContentRangeParser::parse_xml("Content-Range", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "Body" {
                obj.body = try!(BodyParser::parse_xml("Body", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectOutput` contents to a `SignedRequest`
struct GetObjectOutputWriter;
impl GetObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        ContentRangeWriter::write_params(params, &(prefix.to_string() + "Content-Range"), &obj.content_range);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        BodyWriter::write_params(params, &(prefix.to_string() + "Body"), &obj.body);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLifecycleRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketLifecycleRequest` from XML
struct GetBucketLifecycleRequestParser;
impl GetBucketLifecycleRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLifecycleRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLifecycleRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLifecycleRequest` contents to a `SignedRequest`
struct GetBucketLifecycleRequestWriter;
impl GetBucketLifecycleRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLifecycleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type CopySourceSSECustomerKeyMD5 = String;
/// Parse `CopySourceSSECustomerKeyMD`5 from XML
struct CopySourceSSECustomerKeyMD5Parser;
impl CopySourceSSECustomerKeyMD5Parser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerKeyMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceSSECustomerKeyMD5` contents to a `SignedRequest`
struct CopySourceSSECustomerKeyMD5Writer;
impl CopySourceSSECustomerKeyMD5Writer {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerKeyMD5) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct S3ClientError {
    pub version_id: ObjectVersionId,
    pub code: Code,
    pub message: S3ClientMessage,
    pub key: ObjectKey,
}

/// Parse `S`3ClientError from XML
struct S3ClientErrorParser;
impl S3ClientErrorParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<S3ClientError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = S3ClientError::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "Code" {
                obj.code = try!(CodeParser::parse_xml("Code", stack));
                continue;
            }
            if current_name == "Message" {
                obj.message = try!(S3ClientMessageParser::parse_xml("Message", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `S3ClientError` contents to a `SignedRequest`
struct S3ClientErrorWriter;
impl S3ClientErrorWriter {
    fn write_params(params: &mut Params, name: &str, obj: &S3ClientError) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        CodeWriter::write_params(params, &(prefix.to_string() + "Code"), &obj.code);
        S3ClientMessageWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.message);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
#[derive(Debug, Default)]
pub struct BucketLoggingStatus {
    pub logging_enabled: LoggingEnabled,
}

/// Parse `BucketLoggingStatus` from XML
struct BucketLoggingStatusParser;
impl BucketLoggingStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLoggingStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = BucketLoggingStatus::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LoggingEnabled" {
                obj.logging_enabled = try!(LoggingEnabledParser::parse_xml("LoggingEnabled", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketLoggingStatus` contents to a `SignedRequest`
struct BucketLoggingStatusWriter;
impl BucketLoggingStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketLoggingStatus) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LoggingEnabledWriter::write_params(params, &(prefix.to_string() + "LoggingEnabled"), &obj.logging_enabled);
    }
}
pub type IsLatest = bool;
/// Parse `IsLatest` from XML
struct IsLatestParser;
impl IsLatestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IsLatest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IsLatest` contents to a `SignedRequest`
struct IsLatestWriter;
impl IsLatestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IsLatest) {
        params.put(name, &obj.to_string());
    }
}
pub type MaxUploads = i32;
/// Parse `MaxUploads` from XML
struct MaxUploadsParser;
impl MaxUploadsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxUploads, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MaxUploads` contents to a `SignedRequest`
struct MaxUploadsWriter;
impl MaxUploadsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MaxUploads) {
        params.put(name, &obj.to_string());
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

/// Parse `RoutingRule` from XML
struct RoutingRuleParser;
impl RoutingRuleParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RoutingRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RoutingRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Redirect" {
                obj.redirect = try!(RedirectParser::parse_xml("Redirect", stack));
                continue;
            }
            if current_name == "Condition" {
                obj.condition = Some(try!(ConditionParser::parse_xml("Condition", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RoutingRule` contents to a `SignedRequest`
struct RoutingRuleWriter;
impl RoutingRuleWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RoutingRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RedirectWriter::write_params(params, &(prefix.to_string() + "Redirect"), &obj.redirect);
        if let Some(ref obj) = obj.condition {
            ConditionWriter::write_params(params, &(prefix.to_string() + "Condition"), obj);
        }
    }
}
pub type MissingMeta = i32;
/// Parse `MissingMeta` from XML
struct MissingMetaParser;
impl MissingMetaParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MissingMeta, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MissingMeta` contents to a `SignedRequest`
struct MissingMetaWriter;
impl MissingMetaWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MissingMeta) {
        params.put(name, &obj.to_string());
    }
}
pub type SSEKMSKeyId = String;
/// Parse `SSEKMSKeyId` from XML
struct SSEKMSKeyIdParser;
impl SSEKMSKeyIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSEKMSKeyId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `SSEKMSKeyId` contents to a `SignedRequest`
struct SSEKMSKeyIdWriter;
impl SSEKMSKeyIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &SSEKMSKeyId) {
        params.put(name, obj);
    }
}
pub type AllowedOrigins = Vec<AllowedOrigin>;
/// Parse `AllowedOrigins` from XML
struct AllowedOriginsParser;
impl AllowedOriginsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedOrigins, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedOrigin" {
            obj.push(try!(AllowedOriginParser::parse_xml("AllowedOrigin", stack)));
        }
        Ok(obj)
    }
}
/// Write `AllowedOrigins` contents to a `SignedRequest`
struct AllowedOriginsWriter;
impl AllowedOriginsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedOrigins) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedOriginWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
/// The specified multipart upload does not exist.
#[derive(Debug, Default)]
pub struct NoSuchUpload;

/// Parse `NoSuchUpload` from XML
struct NoSuchUploadParser;
impl NoSuchUploadParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoSuchUpload, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoSuchUpload::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NoSuchUpload` contents to a `SignedRequest`
struct NoSuchUploadWriter;
impl NoSuchUploadWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NoSuchUpload) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
#[derive(Debug, Default)]
pub struct PutObjectRequest<'a> {
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
    pub body: Option<&'a [u8]>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
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
    pub acl: Option<CannedAcl>,
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

pub type Code = String;
/// Parse `Code` from XML
struct CodeParser;
impl CodeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Code, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Code` contents to a `SignedRequest`
struct CodeWriter;
impl CodeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}
pub type ReplicationStatus = String;
/// Parse `ReplicationStatus` from XML
struct ReplicationStatusParser;
impl ReplicationStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplicationStatus` contents to a `SignedRequest`
struct ReplicationStatusWriter;
impl ReplicationStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplicationStatus) {
        params.put(name, obj);
    }
}
pub type AllowedHeaders = Vec<AllowedHeader>;
/// Parse `AllowedHeaders` from XML
struct AllowedHeadersParser;
impl AllowedHeadersParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedHeaders, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedHeader" {
            obj.push(try!(AllowedHeaderParser::parse_xml("AllowedHeader", stack)));
        }
        Ok(obj)
    }
}
/// Write `AllowedHeaders` contents to a `SignedRequest`
struct AllowedHeadersWriter;
impl AllowedHeadersWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedHeaders) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedHeaderWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
#[derive(Debug, Default)]
pub struct Tagging {
    pub tag_set: TagSet,
}

/// Parse `Tagging` from XML
struct TaggingParser;
impl TaggingParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tagging, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Tagging::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Tag" {
                obj.tag_set = try!(TagSetParser::parse_xml("Tag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Tagging` contents to a `SignedRequest`
struct TaggingWriter;
impl TaggingWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Tagging) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TagSetWriter::write_params(params, &(prefix.to_string() + "Tag"), &obj.tag_set);
    }
}
pub type ContentMD5 = String;
/// Parse `ContentMD`5 from XML
struct ContentMD5Parser;
impl ContentMD5Parser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentMD5` contents to a `SignedRequest`
struct ContentMD5Writer;
impl ContentMD5Writer {
    fn write_params(params: &mut Params, name: &str, obj: &ContentMD5) {
        params.put(name, obj);
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

/// Parse `ObjectVersion` from XML
struct ObjectVersionParser;
impl ObjectVersionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectVersion::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectVersionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersion` contents to a `SignedRequest`
struct ObjectVersionWriter;
impl ObjectVersionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectVersion) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectVersionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
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

/// Parse `ListObjectVersionsOutput` from XML
struct ListObjectVersionsOutputParser;
impl ListObjectVersionsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectVersionsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectVersionsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            if current_name == "ObjectVersion" {
                obj.versions = try!(ObjectVersionListParser::parse_xml("ObjectVersion", stack));
                continue;
            }
            if current_name == "DeleteMarkerEntry" {
                obj.delete_markers = try!(DeleteMarkersParser::parse_xml("DeleteMarkerEntry", stack));
                continue;
            }
            if current_name == "NextKeyMarker" {
                obj.next_key_marker = try!(NextKeyMarkerParser::parse_xml("NextKeyMarker", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "MaxKeys" {
                obj.max_keys = try!(MaxKeysParser::parse_xml("MaxKeys", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "KeyMarker" {
                obj.key_marker = try!(KeyMarkerParser::parse_xml("KeyMarker", stack));
                continue;
            }
            if current_name == "NextVersionIdMarker" {
                obj.next_version_id_marker = try!(NextVersionIdMarkerParser::parse_xml("NextVersionIdMarker", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "VersionIdMarker" {
                obj.version_id_marker = try!(VersionIdMarkerParser::parse_xml("VersionIdMarker", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListObjectVersionsOutput` contents to a `SignedRequest`
struct ListObjectVersionsOutputWriter;
impl ListObjectVersionsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListObjectVersionsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
        ObjectVersionListWriter::write_params(params, &(prefix.to_string() + "ObjectVersion"), &obj.versions);
        DeleteMarkersWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerEntry"), &obj.delete_markers);
        NextKeyMarkerWriter::write_params(params, &(prefix.to_string() + "NextKeyMarker"), &obj.next_key_marker);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        MaxKeysWriter::write_params(params, &(prefix.to_string() + "MaxKeys"), &obj.max_keys);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        KeyMarkerWriter::write_params(params, &(prefix.to_string() + "KeyMarker"), &obj.key_marker);
        NextVersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "NextVersionIdMarker"), &obj.next_version_id_marker);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "VersionIdMarker"), &obj.version_id_marker);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketNotificationConfigurationRequest {
    pub notification_configuration: NotificationConfiguration,
    pub bucket: BucketName,
}

/// Parse `PutBucketNotificationConfigurationRequest` from XML
struct PutBucketNotificationConfigurationRequestParser;
impl PutBucketNotificationConfigurationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketNotificationConfigurationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketNotificationConfigurationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "NotificationConfiguration" {
                obj.notification_configuration = try!(NotificationConfigurationParser::parse_xml("NotificationConfiguration", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketNotificationConfigurationRequest` contents to a `SignedRequest`
struct PutBucketNotificationConfigurationRequestWriter;
impl PutBucketNotificationConfigurationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketNotificationConfigurationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        NotificationConfigurationWriter::write_params(params, &(prefix.to_string() + "NotificationConfiguration"), &obj.notification_configuration);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type Prefix = String;
/// Parse `Prefix` from XML
struct PrefixParser;
impl PrefixParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Prefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Prefix` contents to a `SignedRequest`
struct PrefixWriter;
impl PrefixWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Prefix) {
        params.put(name, obj);
    }
}
pub type Parts = Vec<Part>;
/// Parse `Parts` from XML
struct PartsParser;
impl PartsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Parts, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Part" {
            obj.push(try!(PartParser::parse_xml("Part", stack)));
        }
        Ok(obj)
    }
}
/// Write `Parts` contents to a `SignedRequest`
struct PartsWriter;
impl PartsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Parts) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            PartWriter::write_params(params, key, element);
            index += 1;
        }
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
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
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
    pub acl: Option<CannedAcl>,
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

/// Parse `CopyObjectRequest` from XML
struct CopyObjectRequestParser;
impl CopyObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-modified-since" {
                obj.copy_source_if_modified_since = Some(try!(CopySourceIfModifiedSinceParser::parse_xml("x-amz-copy-source-if-modified-since", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-unmodified-since" {
                obj.copy_source_if_unmodified_since = Some(try!(CopySourceIfUnmodifiedSinceParser::parse_xml("x-amz-copy-source-if-unmodified-since", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key" {
                obj.copy_source_sse_customer_key = Some(try!(CopySourceSSECustomerKeyParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source" {
                obj.copy_source = try!(CopySourceParser::parse_xml("x-amz-copy-source", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-algorithm" {
                obj.copy_source_sse_customer_algorithm = Some(try!(CopySourceSSECustomerAlgorithmParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key-MD5" {
                obj.copy_source_sse_customer_key_md5 = Some(try!(CopySourceSSECustomerKeyMD5Parser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-match" {
                obj.copy_source_if_match = Some(try!(CopySourceIfMatchParser::parse_xml("x-amz-copy-source-if-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-metadata-directive" {
                obj.metadata_directive = Some(try!(MetadataDirectiveParser::parse_xml("x-amz-metadata-directive", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-none-match" {
                obj.copy_source_if_none_match = Some(try!(CopySourceIfNoneMatchParser::parse_xml("x-amz-copy-source-if-none-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopyObjectRequest` contents to a `SignedRequest`
struct CopyObjectRequestWriter;
impl CopyObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopyObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_modified_since {
            CopySourceIfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-modified-since"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_unmodified_since {
            CopySourceIfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-unmodified-since"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key {
            CopySourceSSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        CopySourceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source"), &obj.copy_source);
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
            CopySourceSSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-algorithm"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
            CopySourceSSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_match {
            CopySourceIfMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.metadata_directive {
            MetadataDirectiveWriter::write_params(params, &(prefix.to_string() + "x-amz-metadata-directive"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_none_match {
            CopySourceIfNoneMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-none-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
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

/// Parse `DeleteObjectsRequest` from XML
struct DeleteObjectsRequestParser;
impl DeleteObjectsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Delete" {
                obj.delete = try!(DeleteParser::parse_xml("Delete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteObjectsRequest` contents to a `SignedRequest`
struct DeleteObjectsRequestWriter;
impl DeleteObjectsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        DeleteWriter::write_params(params, &(prefix.to_string() + "Delete"), &obj.delete);
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

/// Parse `ListPartsOutput` from XML
struct ListPartsOutputParser;
impl ListPartsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListPartsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListPartsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Initiator" {
                obj.initiator = try!(InitiatorParser::parse_xml("Initiator", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "NextPartNumberMarker" {
                obj.next_part_number_marker = try!(NextPartNumberMarkerParser::parse_xml("NextPartNumberMarker", stack));
                continue;
            }
            if current_name == "Part" {
                obj.parts = try!(PartsParser::parse_xml("Part", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(StorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "MaxParts" {
                obj.max_parts = try!(MaxPartsParser::parse_xml("MaxParts", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "PartNumberMarker" {
                obj.part_number_marker = try!(PartNumberMarkerParser::parse_xml("PartNumberMarker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListPartsOutput` contents to a `SignedRequest`
struct ListPartsOutputWriter;
impl ListPartsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListPartsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        InitiatorWriter::write_params(params, &(prefix.to_string() + "Initiator"), &obj.initiator);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        NextPartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "NextPartNumberMarker"), &obj.next_part_number_marker);
        PartsWriter::write_params(params, &(prefix.to_string() + "Part"), &obj.parts);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        MaxPartsWriter::write_params(params, &(prefix.to_string() + "MaxParts"), &obj.max_parts);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        PartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "PartNumberMarker"), &obj.part_number_marker);
    }
}
pub type Marker = String;
/// Parse `Marker` from XML
struct MarkerParser;
impl MarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Marker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Marker` contents to a `SignedRequest`
struct MarkerWriter;
impl MarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Marker) {
        params.put(name, obj);
    }
}
pub type ObjectCannedACL = String;
/// Parse `ObjectCannedACL` from XML
struct ObjectCannedACLParser;
impl ObjectCannedACLParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectCannedACL` contents to a `SignedRequest`
struct ObjectCannedACLWriter;
impl ObjectCannedACLWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectCannedACL) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct RestoreRequest {
    /// Lifetime of the active copy in days
    pub days: Days,
}

/// Parse `RestoreRequest` from XML
struct RestoreRequestParser;
impl RestoreRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Days" {
                obj.days = try!(DaysParser::parse_xml("Days", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RestoreRequest` contents to a `SignedRequest`
struct RestoreRequestWriter;
impl RestoreRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RestoreRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
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

/// Parse `CompletedPart` from XML
struct CompletedPartParser;
impl CompletedPartParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CompletedPart, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CompletedPart::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "PartNumber" {
                obj.part_number = try!(PartNumberParser::parse_xml("PartNumber", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CompletedPart` contents to a `SignedRequest`
struct CompletedPartWriter;
impl CompletedPartWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CompletedPart) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PartNumberWriter::write_params(params, &(prefix.to_string() + "PartNumber"), &obj.part_number);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
    }
}
pub type QueueArn = String;
/// Parse `QueueArn` from XML
struct QueueArnParser;
impl QueueArnParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `QueueArn` contents to a `SignedRequest`
struct QueueArnWriter;
impl QueueArnWriter {
    fn write_params(params: &mut Params, name: &str, obj: &QueueArn) {
        params.put(name, obj);
    }
}
pub type Location = String;
/// Parse `Location` from XML
struct LocationParser;
impl LocationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Location, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Location` contents to a `SignedRequest`
struct LocationWriter;
impl LocationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Location) {
        params.put(name, obj);
    }
}
pub type HttpErrorCodeReturnedEquals = String;
/// Parse `HttpErrorCodeReturnedEquals` from XML
struct HttpErrorCodeReturnedEqualsParser;
impl HttpErrorCodeReturnedEqualsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HttpErrorCodeReturnedEquals, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `HttpErrorCodeReturnedEquals` contents to a `SignedRequest`
struct HttpErrorCodeReturnedEqualsWriter;
impl HttpErrorCodeReturnedEqualsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HttpErrorCodeReturnedEquals) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct NotificationConfigurationDeprecated {
    pub cloud_function_configuration: CloudFunctionConfiguration,
    pub queue_configuration: QueueConfigurationDeprecated,
    pub topic_configuration: TopicConfigurationDeprecated,
}

/// Parse `NotificationConfigurationDeprecated` from XML
struct NotificationConfigurationDeprecatedParser;
impl NotificationConfigurationDeprecatedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NotificationConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CloudFunctionConfiguration" {
                obj.cloud_function_configuration = try!(CloudFunctionConfigurationParser::parse_xml("CloudFunctionConfiguration", stack));
                continue;
            }
            if current_name == "QueueConfiguration" {
                obj.queue_configuration = try!(QueueConfigurationDeprecatedParser::parse_xml("QueueConfiguration", stack));
                continue;
            }
            if current_name == "TopicConfiguration" {
                obj.topic_configuration = try!(TopicConfigurationDeprecatedParser::parse_xml("TopicConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NotificationConfigurationDeprecated` contents to a `SignedRequest`
struct NotificationConfigurationDeprecatedWriter;
impl NotificationConfigurationDeprecatedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NotificationConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CloudFunctionConfigurationWriter::write_params(params, &(prefix.to_string() + "CloudFunctionConfiguration"), &obj.cloud_function_configuration);
        QueueConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configuration);
        TopicConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configuration);
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
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header. This must be the same encryption key
    /// specified in the initiate multipart upload request.
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

/// Parse `UploadPartCopyRequest` from XML
struct UploadPartCopyRequestParser;
impl UploadPartCopyRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<UploadPartCopyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = UploadPartCopyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-copy-source-if-match" {
                obj.copy_source_if_match = Some(try!(CopySourceIfMatchParser::parse_xml("x-amz-copy-source-if-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key-MD5" {
                obj.copy_source_sse_customer_key_md5 = Some(try!(CopySourceSSECustomerKeyMD5Parser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key" {
                obj.copy_source_sse_customer_key = Some(try!(CopySourceSSECustomerKeyParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-algorithm" {
                obj.copy_source_sse_customer_algorithm = Some(try!(CopySourceSSECustomerAlgorithmParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source" {
                obj.copy_source = try!(CopySourceParser::parse_xml("x-amz-copy-source", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-if-modified-since" {
                obj.copy_source_if_modified_since = Some(try!(CopySourceIfModifiedSinceParser::parse_xml("x-amz-copy-source-if-modified-since", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-unmodified-since" {
                obj.copy_source_if_unmodified_since = Some(try!(CopySourceIfUnmodifiedSinceParser::parse_xml("x-amz-copy-source-if-unmodified-since", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-none-match" {
                obj.copy_source_if_none_match = Some(try!(CopySourceIfNoneMatchParser::parse_xml("x-amz-copy-source-if-none-match", stack)));
                continue;
            }
            if current_name == "uploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("uploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-range" {
                obj.copy_source_range = Some(try!(CopySourceRangeParser::parse_xml("x-amz-copy-source-range", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "partNumber" {
                obj.part_number = try!(PartNumberParser::parse_xml("partNumber", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `UploadPartCopyRequest` contents to a `SignedRequest`
struct UploadPartCopyRequestWriter;
impl UploadPartCopyRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &UploadPartCopyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.copy_source_if_match {
            CopySourceIfMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
            CopySourceSSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key {
            CopySourceSSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
            CopySourceSSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-algorithm"), obj);
        }
        CopySourceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source"), &obj.copy_source);
        if let Some(ref obj) = obj.copy_source_if_modified_since {
            CopySourceIfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-modified-since"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_unmodified_since {
            CopySourceIfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-unmodified-since"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_none_match {
            CopySourceIfNoneMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-none-match"), obj);
        }
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "uploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.copy_source_range {
            CopySourceRangeWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-range"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
        PartNumberWriter::write_params(params, &(prefix.to_string() + "partNumber"), &obj.part_number);
    }
}
#[derive(Debug, Default)]
pub struct CORSConfiguration {
    pub cors_rules: CORSRules,
}

/// Parse `CORSConfiguration` from XML
struct CORSConfigurationParser;
impl CORSConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CORSConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CORSConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CORSRule" {
                obj.cors_rules = try!(CORSRulesParser::parse_xml("CORSRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CORSConfiguration` contents to a `SignedRequest`
struct CORSConfigurationWriter;
impl CORSConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CORSConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CORSRulesWriter::write_params(params, &(prefix.to_string() + "CORSRule"), &obj.cors_rules);
    }
}
pub type LastModified = String;
/// Parse `LastModified` from XML
struct LastModifiedParser;
impl LastModifiedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LastModified, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LastModified` contents to a `SignedRequest`
struct LastModifiedWriter;
impl LastModifiedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LastModified) {
        params.put(name, obj);
    }
}
pub type ContentRange = String;
/// Parse `ContentRange` from XML
struct ContentRangeParser;
impl ContentRangeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentRange, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentRange` contents to a `SignedRequest`
struct ContentRangeWriter;
impl ContentRangeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentRange) {
        params.put(name, obj);
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

/// Parse `Grantee` from XML
struct GranteeParser;
impl GranteeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grantee, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Grantee::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "EmailAddress" {
                obj.email_address = Some(try!(EmailAddressParser::parse_xml("EmailAddress", stack)));
                continue;
            }
            if current_name == "xsi:type" {
                obj.foo_type = try!(TypeParser::parse_xml("xsi:type", stack));
                continue;
            }
            if current_name == "DisplayName" {
                obj.display_name = Some(try!(DisplayNameParser::parse_xml("DisplayName", stack)));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            if current_name == "URI" {
                obj.uri = Some(try!(URIParser::parse_xml("URI", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Grantee` contents to a `SignedRequest`
struct GranteeWriter;
impl GranteeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Grantee) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.email_address {
            EmailAddressWriter::write_params(params, &(prefix.to_string() + "EmailAddress"), obj);
        }
        TypeWriter::write_params(params, &(prefix.to_string() + "xsi:type"), &obj.foo_type);
        if let Some(ref obj) = obj.display_name {
            DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), obj);
        }
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
        if let Some(ref obj) = obj.uri {
            URIWriter::write_params(params, &(prefix.to_string() + "URI"), obj);
        }
    }
}
pub type ExpirationStatus = String;
/// Parse `ExpirationStatus` from XML
struct ExpirationStatusParser;
impl ExpirationStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExpirationStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ExpirationStatus` contents to a `SignedRequest`
struct ExpirationStatusWriter;
impl ExpirationStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ExpirationStatus) {
        params.put(name, obj);
    }
}
pub type CopySourceIfUnmodifiedSince = String;
/// Parse `CopySourceIfUnmodifiedSince` from XML
struct CopySourceIfUnmodifiedSinceParser;
impl CopySourceIfUnmodifiedSinceParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfUnmodifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceIfUnmodifiedSince` contents to a `SignedRequest`
struct CopySourceIfUnmodifiedSinceWriter;
impl CopySourceIfUnmodifiedSinceWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfUnmodifiedSince) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketVersioningRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketVersioningRequest` from XML
struct GetBucketVersioningRequestParser;
impl GetBucketVersioningRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketVersioningRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketVersioningRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketVersioningRequest` contents to a `SignedRequest`
struct GetBucketVersioningRequestWriter;
impl GetBucketVersioningRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketVersioningRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
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

/// Parse `MultipartUpload` from XML
struct MultipartUploadParser;
impl MultipartUploadParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUpload::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Initiator" {
                obj.initiator = try!(InitiatorParser::parse_xml("Initiator", stack));
                continue;
            }
            if current_name == "Initiated" {
                obj.initiated = try!(InitiatedParser::parse_xml("Initiated", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(StorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MultipartUpload` contents to a `SignedRequest`
struct MultipartUploadWriter;
impl MultipartUploadWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MultipartUpload) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        InitiatorWriter::write_params(params, &(prefix.to_string() + "Initiator"), &obj.initiator);
        InitiatedWriter::write_params(params, &(prefix.to_string() + "Initiated"), &obj.initiated);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
    }
}
pub type GrantWrite = String;
/// Parse `GrantWrite` from XML
struct GrantWriteParser;
impl GrantWriteParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWrite, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantWrite` contents to a `SignedRequest`
struct GrantWriteWriter;
impl GrantWriteWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantWrite) {
        params.put(name, obj);
    }
}
pub type TagSet = Vec<Tag>;
/// Parse `TagSet` from XML
struct TagSetParser;
impl TagSetParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TagSet, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Tag" {
            obj.push(try!(TagParser::parse_xml("Tag", stack)));
        }
        Ok(obj)
    }
}
/// Write `TagSet` contents to a `SignedRequest`
struct TagSetWriter;
impl TagSetWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TagSet) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TagWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;
/// Parse `LambdaFunctionConfigurationList` from XML
struct LambdaFunctionConfigurationListParser;
impl LambdaFunctionConfigurationListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "LambdaFunctionConfiguration" {
            obj.push(try!(LambdaFunctionConfigurationParser::parse_xml("LambdaFunctionConfiguration", stack)));
        }
        Ok(obj)
    }
}
/// Write `LambdaFunctionConfigurationList` contents to a `SignedRequest`
struct LambdaFunctionConfigurationListWriter;
impl LambdaFunctionConfigurationListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            LambdaFunctionConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type ServerSideEncryption = String;
/// Parse `ServerSideEncryption` from XML
struct ServerSideEncryptionParser;
impl ServerSideEncryptionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ServerSideEncryption, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ServerSideEncryption` contents to a `SignedRequest`
struct ServerSideEncryptionWriter;
impl ServerSideEncryptionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ServerSideEncryption) {
        params.put(name, obj);
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

/// Parse `GetBucketVersioningOutput` from XML
struct GetBucketVersioningOutputParser;
impl GetBucketVersioningOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketVersioningOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketVersioningOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(BucketVersioningStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "MfaDelete" {
                obj.mfa_delete = try!(MFADeleteStatusParser::parse_xml("MfaDelete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketVersioningOutput` contents to a `SignedRequest`
struct GetBucketVersioningOutputWriter;
impl GetBucketVersioningOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketVersioningOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketVersioningStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        MFADeleteStatusWriter::write_params(params, &(prefix.to_string() + "MfaDelete"), &obj.mfa_delete);
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

/// Parse `NoncurrentVersionExpiration` from XML
struct NoncurrentVersionExpirationParser;
impl NoncurrentVersionExpirationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoncurrentVersionExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoncurrentVersionExpiration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "NoncurrentDays" {
                obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NoncurrentVersionExpiration` contents to a `SignedRequest`
struct NoncurrentVersionExpirationWriter;
impl NoncurrentVersionExpirationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NoncurrentVersionExpiration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketRequestPaymentOutput {
    /// Specifies who pays for the download and request fees.
    pub payer: Payer,
}

/// Parse `GetBucketRequestPaymentOutput` from XML
struct GetBucketRequestPaymentOutputParser;
impl GetBucketRequestPaymentOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketRequestPaymentOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Payer" {
                obj.payer = try!(PayerParser::parse_xml("Payer", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketRequestPaymentOutput` contents to a `SignedRequest`
struct GetBucketRequestPaymentOutputWriter;
impl GetBucketRequestPaymentOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketRequestPaymentOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PayerWriter::write_params(params, &(prefix.to_string() + "Payer"), &obj.payer);
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
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
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

/// Parse `GetObjectRequest` from XML
struct GetObjectRequestParser;
impl GetObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "response-content-encoding" {
                obj.response_content_encoding = Some(try!(ResponseContentEncodingParser::parse_xml("response-content-encoding", stack)));
                continue;
            }
            if current_name == "response-content-language" {
                obj.response_content_language = Some(try!(ResponseContentLanguageParser::parse_xml("response-content-language", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "response-content-type" {
                obj.response_content_type = Some(try!(ResponseContentTypeParser::parse_xml("response-content-type", stack)));
                continue;
            }
            if current_name == "If-Unmodified-Since" {
                obj.if_unmodified_since = Some(try!(IfUnmodifiedSinceParser::parse_xml("If-Unmodified-Since", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "response-cache-control" {
                obj.response_cache_control = Some(try!(ResponseCacheControlParser::parse_xml("response-cache-control", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "If-None-Match" {
                obj.if_none_match = Some(try!(IfNoneMatchParser::parse_xml("If-None-Match", stack)));
                continue;
            }
            if current_name == "response-content-disposition" {
                obj.response_content_disposition = Some(try!(ResponseContentDispositionParser::parse_xml("response-content-disposition", stack)));
                continue;
            }
            if current_name == "Range" {
                obj.range = Some(try!(RangeParser::parse_xml("Range", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "If-Match" {
                obj.if_match = Some(try!(IfMatchParser::parse_xml("If-Match", stack)));
                continue;
            }
            if current_name == "response-expires" {
                obj.response_expires = Some(try!(ResponseExpiresParser::parse_xml("response-expires", stack)));
                continue;
            }
            if current_name == "If-Modified-Since" {
                obj.if_modified_since = Some(try!(IfModifiedSinceParser::parse_xml("If-Modified-Since", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectRequest` contents to a `SignedRequest`
struct GetObjectRequestWriter;
impl GetObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.response_content_encoding {
            ResponseContentEncodingWriter::write_params(params, &(prefix.to_string() + "response-content-encoding"), obj);
        }
        if let Some(ref obj) = obj.response_content_language {
            ResponseContentLanguageWriter::write_params(params, &(prefix.to_string() + "response-content-language"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.response_content_type {
            ResponseContentTypeWriter::write_params(params, &(prefix.to_string() + "response-content-type"), obj);
        }
        if let Some(ref obj) = obj.if_unmodified_since {
            IfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Unmodified-Since"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.response_cache_control {
            ResponseCacheControlWriter::write_params(params, &(prefix.to_string() + "response-cache-control"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.if_none_match {
            IfNoneMatchWriter::write_params(params, &(prefix.to_string() + "If-None-Match"), obj);
        }
        if let Some(ref obj) = obj.response_content_disposition {
            ResponseContentDispositionWriter::write_params(params, &(prefix.to_string() + "response-content-disposition"), obj);
        }
        if let Some(ref obj) = obj.range {
            RangeWriter::write_params(params, &(prefix.to_string() + "Range"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.if_match {
            IfMatchWriter::write_params(params, &(prefix.to_string() + "If-Match"), obj);
        }
        if let Some(ref obj) = obj.response_expires {
            ResponseExpiresWriter::write_params(params, &(prefix.to_string() + "response-expires"), obj);
        }
        if let Some(ref obj) = obj.if_modified_since {
            IfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Modified-Since"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}
pub type ContentDisposition = String;
/// Parse `ContentDisposition` from XML
struct ContentDispositionParser;
impl ContentDispositionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentDisposition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentDisposition` contents to a `SignedRequest`
struct ContentDispositionWriter;
impl ContentDispositionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentDisposition) {
        params.put(name, obj);
    }
}
pub type MetadataKey = String;
/// Parse `MetadataKey` from XML
struct MetadataKeyParser;
impl MetadataKeyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MetadataKey` contents to a `SignedRequest`
struct MetadataKeyWriter;
impl MetadataKeyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MetadataKey) {
        params.put(name, obj);
    }
}
pub type ResponseContentEncoding = String;
/// Parse `ResponseContentEncoding` from XML
struct ResponseContentEncodingParser;
impl ResponseContentEncodingParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentEncoding, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseContentEncoding` contents to a `SignedRequest`
struct ResponseContentEncodingWriter;
impl ResponseContentEncodingWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseContentEncoding) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLoggingRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketLoggingRequest` from XML
struct GetBucketLoggingRequestParser;
impl GetBucketLoggingRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLoggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLoggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLoggingRequest` contents to a `SignedRequest`
struct GetBucketLoggingRequestWriter;
impl GetBucketLoggingRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLoggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type UploadIdMarker = String;
/// Parse `UploadIdMarker` from XML
struct UploadIdMarkerParser;
impl UploadIdMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<UploadIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = UploadIdMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `UploadIdMarker` contents to a `SignedRequest`
struct UploadIdMarkerWriter;
impl UploadIdMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &UploadIdMarker) {
        params.put(name, obj);
    }
}
pub type Type = String;
/// Parse `Type` from XML
struct TypeParser;
impl TypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Type, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Type` contents to a `SignedRequest`
struct TypeWriter;
impl TypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Type) {
        params.put(name, obj);
    }
}
pub type Buckets = Vec<Bucket>;
/// Parse `Buckets` from XML
struct BucketsParser;
impl BucketsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Buckets, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Bucket" {
            obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
        }
        Ok(obj)
    }
}
/// Write `Buckets` contents to a `SignedRequest`
struct BucketsWriter;
impl BucketsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Buckets) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            BucketWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type Expires = String;
/// Parse `Expires` from XML
struct ExpiresParser;
impl ExpiresParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Expires, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Expires` contents to a `SignedRequest`
struct ExpiresWriter;
impl ExpiresWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Expires) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct RestoreObjectOutput {
    pub request_charged: RequestCharged,
}

/// Parse `RestoreObjectOutput` from XML
struct RestoreObjectOutputParser;
impl RestoreObjectOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RestoreObjectOutput` contents to a `SignedRequest`
struct RestoreObjectOutputWriter;
impl RestoreObjectOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
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

/// Parse `RestoreObjectRequest` from XML
struct RestoreObjectRequestParser;
impl RestoreObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "RestoreRequest" {
                obj.restore_request = Some(try!(RestoreRequestParser::parse_xml("RestoreRequest", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RestoreObjectRequest` contents to a `SignedRequest`
struct RestoreObjectRequestWriter;
impl RestoreObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.restore_request {
            RestoreRequestWriter::write_params(params, &(prefix.to_string() + "RestoreRequest"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLocationOutput {
    pub location_constraint: BucketLocationConstraint,
}

/// Parse `GetBucketLocationOutput` from XML
struct GetBucketLocationOutputParser;
impl GetBucketLocationOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLocationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLocationOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LocationConstraint" {
                obj.location_constraint = try!(BucketLocationConstraintParser::parse_xml("LocationConstraint", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLocationOutput` contents to a `SignedRequest`
struct GetBucketLocationOutputWriter;
impl GetBucketLocationOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLocationOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketLocationConstraintWriter::write_params(params, &(prefix.to_string() + "LocationConstraint"), &obj.location_constraint);
    }
}
#[derive(Debug, Default)]
pub struct GetObjectAclOutput {
    pub owner: Owner,
    /// A list of grants.
    pub grants: Grants,
    pub request_charged: RequestCharged,
}

/// Parse `GetObjectAclOutput` from XML
struct GetObjectAclOutputParser;
impl GetObjectAclOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectAclOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetObjectAclOutput` contents to a `SignedRequest`
struct GetObjectAclOutputWriter;
impl GetObjectAclOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetObjectAclOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}
pub type ReplaceKeyWith = String;
/// Parse `ReplaceKeyWith` from XML
struct ReplaceKeyWithParser;
impl ReplaceKeyWithParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplaceKeyWith, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplaceKeyWith` contents to a `SignedRequest`
struct ReplaceKeyWithWriter;
impl ReplaceKeyWithWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplaceKeyWith) {
        params.put(name, obj);
    }
}
pub type ObjectKey = String;
/// Parse `ObjectKey` from XML
struct ObjectKeyParser;
impl ObjectKeyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectKey` contents to a `SignedRequest`
struct ObjectKeyWriter;
impl ObjectKeyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectKey) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketTaggingRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketTaggingRequest` from XML
struct GetBucketTaggingRequestParser;
impl GetBucketTaggingRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketTaggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketTaggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketTaggingRequest` contents to a `SignedRequest`
struct GetBucketTaggingRequestWriter;
impl GetBucketTaggingRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketTaggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketPolicyOutput {
    /// The bucket policy as a JSON document.
    pub policy: Policy,
}

/// Parse `GetBucketPolicyOutput` from XML
struct GetBucketPolicyOutputParser;
impl GetBucketPolicyOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketPolicyOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Policy" {
                obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketPolicyOutput` contents to a `SignedRequest`
struct GetBucketPolicyOutputWriter;
impl GetBucketPolicyOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketPolicyOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PolicyWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policy);
    }
}
pub type MaxAgeSeconds = i32;
/// Parse `MaxAgeSeconds` from XML
struct MaxAgeSecondsParser;
impl MaxAgeSecondsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxAgeSeconds, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MaxAgeSeconds` contents to a `SignedRequest`
struct MaxAgeSecondsWriter;
impl MaxAgeSecondsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MaxAgeSeconds) {
        params.put(name, &obj.to_string());
    }
}
pub type CopySourceRange = String;
/// Parse `CopySourceRange` from XML
struct CopySourceRangeParser;
impl CopySourceRangeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceRange, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceRange` contents to a `SignedRequest`
struct CopySourceRangeWriter;
impl CopySourceRangeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceRange) {
        params.put(name, obj);
    }
}
pub type TopicArn = String;
/// Parse `TopicArn` from XML
struct TopicArnParser;
impl TopicArnParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `TopicArn` contents to a `SignedRequest`
struct TopicArnWriter;
impl TopicArnWriter {
    fn write_params(params: &mut Params, name: &str, obj: &TopicArn) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketTaggingRequest {
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    pub tagging: Tagging,
}

/// Parse `PutBucketTaggingRequest` from XML
struct PutBucketTaggingRequestParser;
impl PutBucketTaggingRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketTaggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketTaggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "Tagging" {
                obj.tagging = try!(TaggingParser::parse_xml("Tagging", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketTaggingRequest` contents to a `SignedRequest`
struct PutBucketTaggingRequestWriter;
impl PutBucketTaggingRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketTaggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        TaggingWriter::write_params(params, &(prefix.to_string() + "Tagging"), &obj.tagging);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketRequestPaymentRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketRequestPaymentRequest` from XML
struct GetBucketRequestPaymentRequestParser;
impl GetBucketRequestPaymentRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketRequestPaymentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketRequestPaymentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketRequestPaymentRequest` contents to a `SignedRequest`
struct GetBucketRequestPaymentRequestWriter;
impl GetBucketRequestPaymentRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketRequestPaymentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
#[derive(Debug, Default)]
pub struct CommonPrefix {
    pub prefix: Prefix,
}

/// Parse `CommonPrefix` from XML
struct CommonPrefixParser;
impl CommonPrefixParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CommonPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CommonPrefix::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CommonPrefix` contents to a `SignedRequest`
struct CommonPrefixWriter;
impl CommonPrefixWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CommonPrefix) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
    }
}
/// The specified key does not exist.
#[derive(Debug, Default)]
pub struct NoSuchKey;

/// Parse `NoSuchKey` from XML
struct NoSuchKeyParser;
impl NoSuchKeyParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoSuchKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoSuchKey::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NoSuchKey` contents to a `SignedRequest`
struct NoSuchKeyWriter;
impl NoSuchKeyWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NoSuchKey) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
#[derive(Debug, Default)]
pub struct UploadPartRequest <'a> {
    pub body: Option<&'a [u8]>,
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
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header. This must be the same encryption key
    /// specified in the initiate multipart upload request.
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


pub type ObjectVersionList = Vec<ObjectVersion>;
/// Parse `ObjectVersionList` from XML
struct ObjectVersionListParser;
impl ObjectVersionListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ObjectVersion" {
            obj.push(try!(ObjectVersionParser::parse_xml("ObjectVersion", stack)));
        }
        Ok(obj)
    }
}
/// Write `ObjectVersionList` contents to a `SignedRequest`
struct ObjectVersionListWriter;
impl ObjectVersionListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectVersionWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type MFA = String;
/// Parse `MFA` from XML
struct MFAParser;
impl MFAParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFA, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MFA` contents to a `SignedRequest`
struct MFAWriter;
impl MFAWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MFA) {
        params.put(name, obj);
    }
}
pub type MultipartUploadList = Vec<MultipartUpload>;
/// Parse `MultipartUploadList` from XML
struct MultipartUploadListParser;
impl MultipartUploadListParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "MultipartUpload" {
            obj.push(try!(MultipartUploadParser::parse_xml("MultipartUpload", stack)));
        }
        Ok(obj)
    }
}
/// Write `MultipartUploadList` contents to a `SignedRequest`
struct MultipartUploadListWriter;
impl MultipartUploadListWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            MultipartUploadWriter::write_params(params, key, element);
            index += 1;
        }
    }
}
pub type AllowedHeader = String;
/// Parse `AllowedHeader` from XML
struct AllowedHeaderParser;
impl AllowedHeaderParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedHeader, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `AllowedHeader` contents to a `SignedRequest`
struct AllowedHeaderWriter;
impl AllowedHeaderWriter {
    fn write_params(params: &mut Params, name: &str, obj: &AllowedHeader) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct Bucket {
    /// Date the bucket was created.
    pub creation_date: CreationDate,
    /// The name of the bucket.
    pub name: BucketName,
}

/// Parse `Bucket` from XML
struct BucketParser;
impl BucketParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Bucket::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CreationDate" {
                obj.creation_date = try!(CreationDateParser::parse_xml("CreationDate", stack));
                continue;
            }
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Bucket` contents to a `SignedRequest`
struct BucketWriter;
impl BucketWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Bucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
    }
}
pub type URI = String;
/// Parse `URI` from XML
struct URIParser;
impl URIParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<URI, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `URI` contents to a `SignedRequest`
struct URIWriter;
impl URIWriter {
    fn write_params(params: &mut Params, name: &str, obj: &URI) {
        params.put(name, obj);
    }
}
/// If present, indicates that the requester was successfully charged for the
/// request.
pub type RequestCharged = String;
/// Parse `RequestCharged` from XML
struct RequestChargedParser;
impl RequestChargedParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestCharged, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `RequestCharged` contents to a `SignedRequest`
struct RequestChargedWriter;
impl RequestChargedWriter {
    fn write_params(params: &mut Params, name: &str, obj: &RequestCharged) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct PutBucketLoggingRequest {
    pub bucket_logging_status: BucketLoggingStatus,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketLoggingRequest` from XML
struct PutBucketLoggingRequestParser;
impl PutBucketLoggingRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketLoggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketLoggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "BucketLoggingStatus" {
                obj.bucket_logging_status = try!(BucketLoggingStatusParser::parse_xml("BucketLoggingStatus", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PutBucketLoggingRequest` contents to a `SignedRequest`
struct PutBucketLoggingRequestWriter;
impl PutBucketLoggingRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketLoggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketLoggingStatusWriter::write_params(params, &(prefix.to_string() + "BucketLoggingStatus"), &obj.bucket_logging_status);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type Delimiter = String;
/// Parse `Delimiter` from XML
struct DelimiterParser;
impl DelimiterParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Delimiter, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Delimiter` contents to a `SignedRequest`
struct DelimiterWriter;
impl DelimiterWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Delimiter) {
        params.put(name, obj);
    }
}
pub type MetadataValue = String;
/// Parse `MetadataValue` from XML
struct MetadataValueParser;
impl MetadataValueParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataValue, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MetadataValue` contents to a `SignedRequest`
struct MetadataValueWriter;
impl MetadataValueWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MetadataValue) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct LifecycleConfiguration {
    pub rules: Rules,
}

/// Parse `LifecycleConfiguration` from XML
struct LifecycleConfigurationParser;
impl LifecycleConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LifecycleConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LifecycleConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Rule" {
                obj.rules = try!(RulesParser::parse_xml("Rule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LifecycleConfiguration` contents to a `SignedRequest`
struct LifecycleConfigurationWriter;
impl LifecycleConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LifecycleConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RulesWriter::write_params(params, &(prefix.to_string() + "Rule"), &obj.rules);
    }
}
pub type Expiration = String;
/// Parse `Expiration` from XML
struct ExpirationParser;
impl ExpirationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Expiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Expiration` contents to a `SignedRequest`
struct ExpirationWriter;
impl ExpirationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Expiration) {
        params.put(name, obj);
    }
}
pub type IfMatch = String;
/// Parse `IfMatch` from XML
struct IfMatchParser;
impl IfMatchParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IfMatch` contents to a `SignedRequest`
struct IfMatchWriter;
impl IfMatchWriter {
    fn write_params(params: &mut Params, name: &str, obj: &IfMatch) {
        params.put(name, obj);
    }
}
pub type ResponseExpires = String;
/// Parse `ResponseExpires` from XML
struct ResponseExpiresParser;
impl ResponseExpiresParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseExpires, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseExpires` contents to a `SignedRequest`
struct ResponseExpiresWriter;
impl ResponseExpiresWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseExpires) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct DeleteBucketTaggingRequest {
    pub bucket: BucketName,
}

/// Parse `DeleteBucketTaggingRequest` from XML
struct DeleteBucketTaggingRequestParser;
impl DeleteBucketTaggingRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketTaggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketTaggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteBucketTaggingRequest` contents to a `SignedRequest`
struct DeleteBucketTaggingRequestWriter;
impl DeleteBucketTaggingRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketTaggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
/// Container for specifying the AWS Lambda notification configuration.
#[derive(Debug, Default)]
pub struct LambdaFunctionConfiguration {
    /// Lambda cloud function ARN that Amazon S3 can invoke when it detects events of
    /// the specified type.
    pub lambda_function_arn: LambdaFunctionArn,
    pub id: Option<NotificationId>,
    pub events: EventList,
}

/// Parse `LambdaFunctionConfiguration` from XML
struct LambdaFunctionConfigurationParser;
impl LambdaFunctionConfigurationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LambdaFunctionConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CloudFunction" {
                obj.lambda_function_arn = try!(LambdaFunctionArnParser::parse_xml("CloudFunction", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LambdaFunctionConfiguration` contents to a `SignedRequest`
struct LambdaFunctionConfigurationWriter;
impl LambdaFunctionConfigurationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LambdaFunctionArnWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.lambda_function_arn);
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}
/// Requests Amazon S3 to encode the object keys in the response and specifies the
/// encoding method to use. An object key may contain any Unicode character;
/// however, XML 1.0 parser cannot parse some characters, such as characters with
/// an ASCII value from 0 to 10. For characters that are not supported in XML 1.0,
/// you can add this parameter to request that Amazon S3 encode the keys in the
/// response.
pub type EncodingType = String;
/// Parse `EncodingType` from XML
struct EncodingTypeParser;
impl EncodingTypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EncodingType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `EncodingType` contents to a `SignedRequest`
struct EncodingTypeWriter;
impl EncodingTypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &EncodingType) {
        params.put(name, obj);
    }
}
pub type ID = String;
/// Parse `ID` from XML
struct IDParser;
impl IDParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ID, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ID` contents to a `SignedRequest`
struct IDWriter;
impl IDWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ID) {
        params.put(name, obj);
    }
}
pub type PartNumberMarker = i32;
/// Parse `PartNumberMarker` from XML
struct PartNumberMarkerParser;
impl PartNumberMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PartNumberMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `PartNumberMarker` contents to a `SignedRequest`
struct PartNumberMarkerWriter;
impl PartNumberMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PartNumberMarker) {
        params.put(name, &obj.to_string());
    }
}
pub type DeleteMarkerVersionId = String;
/// Parse `DeleteMarkerVersionId` from XML
struct DeleteMarkerVersionIdParser;
impl DeleteMarkerVersionIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkerVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DeleteMarkerVersionId` contents to a `SignedRequest`
struct DeleteMarkerVersionIdWriter;
impl DeleteMarkerVersionIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerVersionId) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketWebsiteOutput {
    pub redirect_all_requests_to: RedirectAllRequestsTo,
    pub index_document: IndexDocument,
    pub error_document: ErrorDocument,
    pub routing_rules: RoutingRules,
}

/// Parse `GetBucketWebsiteOutput` from XML
struct GetBucketWebsiteOutputParser;
impl GetBucketWebsiteOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketWebsiteOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketWebsiteOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RedirectAllRequestsTo" {
                obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_xml("RedirectAllRequestsTo", stack));
                continue;
            }
            if current_name == "IndexDocument" {
                obj.index_document = try!(IndexDocumentParser::parse_xml("IndexDocument", stack));
                continue;
            }
            if current_name == "ErrorDocument" {
                obj.error_document = try!(ErrorDocumentParser::parse_xml("ErrorDocument", stack));
                continue;
            }
            if current_name == "RoutingRule" {
                obj.routing_rules = try!(RoutingRulesParser::parse_xml("RoutingRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketWebsiteOutput` contents to a `SignedRequest`
struct GetBucketWebsiteOutputWriter;
impl GetBucketWebsiteOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketWebsiteOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RedirectAllRequestsToWriter::write_params(params, &(prefix.to_string() + "RedirectAllRequestsTo"), &obj.redirect_all_requests_to);
        IndexDocumentWriter::write_params(params, &(prefix.to_string() + "IndexDocument"), &obj.index_document);
        ErrorDocumentWriter::write_params(params, &(prefix.to_string() + "ErrorDocument"), &obj.error_document);
        RoutingRulesWriter::write_params(params, &(prefix.to_string() + "RoutingRule"), &obj.routing_rules);
    }
}
pub type CopySourceIfMatch = String;
/// Parse `CopySourceIfMatch` from XML
struct CopySourceIfMatchParser;
impl CopySourceIfMatchParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CopySourceIfMatch` contents to a `SignedRequest`
struct CopySourceIfMatchWriter;
impl CopySourceIfMatchWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfMatch) {
        params.put(name, obj);
    }
}
pub type ReplicationRuleStatus = String;
/// Parse `ReplicationRuleStatus` from XML
struct ReplicationRuleStatusParser;
impl ReplicationRuleStatusParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRuleStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ReplicationRuleStatus` contents to a `SignedRequest`
struct ReplicationRuleStatusWriter;
impl ReplicationRuleStatusWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ReplicationRuleStatus) {
        params.put(name, obj);
    }
}
pub type ContentType = String;
/// Parse `ContentType` from XML
struct ContentTypeParser;
impl ContentTypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentType` contents to a `SignedRequest`
struct ContentTypeWriter;
impl ContentTypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentType) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketLocationRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketLocationRequest` from XML
struct GetBucketLocationRequestParser;
impl GetBucketLocationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLocationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLocationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketLocationRequest` contents to a `SignedRequest`
struct GetBucketLocationRequestWriter;
impl GetBucketLocationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketLocationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
pub type NextPartNumberMarker = i32;
/// Parse `NextPartNumberMarker` from XML
struct NextPartNumberMarkerParser;
impl NextPartNumberMarkerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextPartNumberMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NextPartNumberMarker` contents to a `SignedRequest`
struct NextPartNumberMarkerWriter;
impl NextPartNumberMarkerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NextPartNumberMarker) {
        params.put(name, &obj.to_string());
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

/// Parse `ListPartsRequest` from XML
struct ListPartsRequestParser;
impl ListPartsRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListPartsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListPartsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "uploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("uploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "max-parts" {
                obj.max_parts = Some(try!(MaxPartsParser::parse_xml("max-parts", stack)));
                continue;
            }
            if current_name == "part-number-marker" {
                obj.part_number_marker = Some(try!(PartNumberMarkerParser::parse_xml("part-number-marker", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListPartsRequest` contents to a `SignedRequest`
struct ListPartsRequestWriter;
impl ListPartsRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListPartsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "uploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.max_parts {
            MaxPartsWriter::write_params(params, &(prefix.to_string() + "max-parts"), obj);
        }
        if let Some(ref obj) = obj.part_number_marker {
            PartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "part-number-marker"), obj);
        }
    }
}
pub type ResponseCacheControl = String;
/// Parse `ResponseCacheControl` from XML
struct ResponseCacheControlParser;
impl ResponseCacheControlParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseCacheControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ResponseCacheControl` contents to a `SignedRequest`
struct ResponseCacheControlWriter;
impl ResponseCacheControlWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ResponseCacheControl) {
        params.put(name, obj);
    }
}
pub type ETag = String;
/// Parse `ETag` from XML
struct ETagParser;
impl ETagParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ETag, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ETag` contents to a `SignedRequest`
struct ETagWriter;
impl ETagWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ETag) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct ObjectIdentifier {
    /// VersionId for the specific version of the object to delete.
    pub version_id: Option<ObjectVersionId>,
    /// Key name of the object to delete.
    pub key: ObjectKey,
}

/// Parse `ObjectIdentifier` from XML
struct ObjectIdentifierParser;
impl ObjectIdentifierParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifier, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectIdentifier::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("VersionId", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectIdentifier` contents to a `SignedRequest`
struct ObjectIdentifierWriter;
impl ObjectIdentifierWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifier) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
/// Optional unique identifier for configurations in a notification configuration.
/// If you don't provide one, Amazon S3 will assign an ID.
pub type NotificationId = String;
/// Parse `NotificationId` from XML
struct NotificationIdParser;
impl NotificationIdParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `NotificationId` contents to a `SignedRequest`
struct NotificationIdWriter;
impl NotificationIdWriter {
    fn write_params(params: &mut Params, name: &str, obj: &NotificationId) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct GetBucketCorsOutput {
    pub cors_rules: CORSRules,
}

/// Parse `GetBucketCorsOutput` from XML
struct GetBucketCorsOutputParser;
impl GetBucketCorsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketCorsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketCorsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CORSRule" {
                obj.cors_rules = try!(CORSRulesParser::parse_xml("CORSRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GetBucketCorsOutput` contents to a `SignedRequest`
struct GetBucketCorsOutputWriter;
impl GetBucketCorsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GetBucketCorsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CORSRulesWriter::write_params(params, &(prefix.to_string() + "CORSRule"), &obj.cors_rules);
    }
}
pub type CreationDate = String;
/// Parse `CreationDate` from XML
struct CreationDateParser;
impl CreationDateParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreationDate, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreationDate` contents to a `SignedRequest`
struct CreationDateWriter;
impl CreationDateWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreationDate) {
        params.put(name, obj);
    }
}
pub type MaxParts = i32;
/// Parse `MaxParts` from XML
struct MaxPartsParser;
impl MaxPartsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxParts, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MaxParts` contents to a `SignedRequest`
struct MaxPartsWriter;
impl MaxPartsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &MaxParts) {
        params.put(name, &obj.to_string());
    }
}
pub struct S3Client<P> where P: ProvideAwsCredentials {
    credentials_provider: P,
    region: Region,
}

impl<P> S3Client<P> where P: ProvideAwsCredentials {
    pub fn new(credentials_provider: P, region: Region) -> S3Client<P> {
        S3Client { credentials_provider: credentials_provider, region: region }
    }

    /// Returns metadata about all of the versions of objects in a bucket.
    pub fn list_object_versions(&self, input: &ListObjectVersionsRequest) -> Result<ListObjectVersionsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?versions");
        let mut params = Params::new();
        params.put("Action", "ListObjectVersions");
        ListObjectVersionsRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(ListObjectVersionsOutputParser::parse_xml("ListObjectVersionsOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Replaces a policy on a bucket. If the bucket already has a policy, the one in
    /// this request completely replaces it.
    pub fn put_bucket_policy(&self, input: &PutBucketPolicyRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?policy");
        let mut params = Params::new();
        params.put("Action", "PutBucketPolicy");
        PutBucketPolicyRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns some or all (up to 1000) of the objects in a bucket. You can use the
    /// request parameters as selection criteria to return a subset of the objects in
    /// a bucket.
    pub fn list_objects(&self, input: &ListObjectsRequest) -> Result<ListObjectsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}");
        let mut params = Params::new();
        params.put("Action", "ListObjects");
        ListObjectsRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(ListObjectsOutputParser::parse_xml("ListObjectsOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Set the website configuration for a bucket.
    pub fn put_bucket_website(&self, input: &PutBucketWebsiteRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?website");
        let mut params = Params::new();
        params.put("Action", "PutBucketWebsite");
        PutBucketWebsiteRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deprecated, see the PutBucketNotificationConfiguraiton operation.
    pub fn put_bucket_notification(&self, input: &PutBucketNotificationRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?notification");
        let mut params = Params::new();
        params.put("Action", "PutBucketNotification");
        PutBucketNotificationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Set the logging parameters for a bucket and to specify permissions for who can
    /// view and modify the logging parameters. To set the logging status of a bucket,
    /// you must be the bucket owner.
    pub fn put_bucket_logging(&self, input: &PutBucketLoggingRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?logging");
        let mut params = Params::new();
        params.put("Action", "PutBucketLogging");
        PutBucketLoggingRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Creates a new replication configuration (or replaces an existing one, if
    /// present).
    pub fn put_bucket_replication(&self, input: &PutBucketReplicationRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?replication");
        let mut params = Params::new();
        params.put("Action", "PutBucketReplication");
        PutBucketReplicationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Uploads a part in a multipart upload.
    /// **Note:** After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.
    pub fn upload_part(&self, input: &UploadPartRequest) -> Result<String, AwsError> {
        let object_id = &input.key;
        let mut request = SignedRequest::new("PUT", "s3", self.region, &format!("/{}", object_id));

        request.set_payload(input.body);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        if let Some(ref md5) = input.content_md5 {
            request.add_header("Content-MD5", md5);
        }

        let mut params = Params::new();
        let upload_id = &input.upload_id;
        let part_number = &input.part_number;
        params.put("partNumber", &format!("{}", part_number));
        params.put("uploadId", upload_id);
        request.set_params(params);

        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                for header in result.headers.iter() {
                    if header.name() == "ETag" {
                        return Ok(header.value_string());
                    }
                }
                Err(AwsError::new("Couldn't find etag in response headers."))
            }
            _ => {
                println!("Error: Status code was {}", status);
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                println!("Error response body: {}", body);
                Err(AwsError::new("error: didn't get a 200."))
            }
        }
    }
    /// Adds an object to a bucket.
    pub fn put_object(&self, input: &PutObjectRequest) -> Result<PutObjectOutput, AwsError> {
        let mut uri = String::from("/");
        uri = uri +  &input.key.to_string();
        let mut request = SignedRequest::new("PUT", "s3", self.region, &uri);

        if let Some(ref class) = input.storage_class {
            request.add_header("x-amz-storage-class", class);
        }

        if let Some(ref sse) = input.server_side_encryption {
            if sse.to_string().to_ascii_lowercase() == "aes256" {
                request.add_header("x-amz-server-side-encryption", sse);
            } else {
                match input.ssekms_key_id {
                    Some(ref key_id) => request.add_header("x-amz-server-side-encryption-aws-kms-key-id", key_id),
                    None => return Err(AwsError::new("KMS key specified but no key id provided.")),
                }
                request.add_header("x-amz-server-side-encryption", "aws:kms");
            }
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", cache_control);
        }

        if let Some(ref md5) = input.content_md5 {
            request.add_header("Content-MD5", md5);
        }

        if let Some(ref metadata) = input.metadata {
            for (key, value) in metadata {
                request.add_header(&format!("x-amz-meta-{}", key), value);
            }
        }

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &canned_acl_in_aws_format(acl));
        }

        match input.content_type {
            Some(ref content_type) => request.set_content_type(content_type.to_string()),

            // binary/octet-stream is default per the S3 API docs
            None => request.set_content_type("binary/octet-stream".to_string())
        };

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));
        request.set_payload(input.body);

        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                let mut put_result = PutObjectOutput::default();

                Ok(put_result)
            }
            _ => {
                println!("Error: Status code was {}", status);
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                println!("Error response body: {}", body);

                Err(AwsError::new("error uploading object to S3"))
            }
        }
    }
    /// Deletes the cors configuration information set for the bucket.
    pub fn delete_bucket_cors(&self, input: &DeleteBucketCorsRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?cors");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketCors");
        DeleteBucketCorsRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Sets the versioning state of an existing bucket. To set the versioning state,
    /// you must be the bucket owner.
    pub fn put_bucket_versioning(&self, input: &PutBucketVersioningRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?versioning");
        let mut params = Params::new();
        params.put("Action", "PutBucketVersioning");
        PutBucketVersioningRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the cors configuration for the bucket.
    pub fn get_bucket_cors(&self, input: &GetBucketCorsRequest) -> Result<GetBucketCorsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?cors");
        let mut params = Params::new();
        params.put("Action", "GetBucketCors");
        GetBucketCorsRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketCorsOutputParser::parse_xml("GetBucketCorsOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Sets lifecycle configuration for your bucket. If a lifecycle configuration
    /// exists, it replaces it.
    pub fn put_bucket_lifecycle(&self, input: &PutBucketLifecycleRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?lifecycle");
        let mut params = Params::new();
        params.put("Action", "PutBucketLifecycle");
        PutBucketLifecycleRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Gets the access control policy for the bucket.
    pub fn get_bucket_acl(&self, input: &GetBucketAclRequest) -> Result<GetBucketAclOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?acl");
        let mut params = Params::new();
        params.put("Action", "GetBucketAcl");
        GetBucketAclRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketAclOutputParser::parse_xml("GetBucketAclOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the logging status of a bucket and the permissions users have to view
    /// and modify that status. To use GET, you must be the bucket owner.
    pub fn get_bucket_logging(&self, input: &GetBucketLoggingRequest) -> Result<GetBucketLoggingOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?logging");
        let mut params = Params::new();
        params.put("Action", "GetBucketLogging");
        GetBucketLoggingRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketLoggingOutputParser::parse_xml("GetBucketLoggingOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// This operation is useful to determine if a bucket exists and you have
    /// permission to access it.
    pub fn head_bucket(&self, input: &HeadBucketRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("HEAD", "s3", self.region, "/{Bucket}");
        let mut params = Params::new();
        params.put("Action", "HeadBucket");
        HeadBucketRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Sets the permissions on a bucket using access control lists (ACL).
    pub fn put_bucket_acl(&self, input: &PutBucketAclRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?acl");
        let mut params = Params::new();
        params.put("Action", "PutBucketAcl");
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// This operation removes the website configuration from the bucket.
    pub fn delete_bucket_website(&self, input: &DeleteBucketWebsiteRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?website");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketWebsite");
        DeleteBucketWebsiteRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deletes the policy from the bucket.
    pub fn delete_bucket_policy(&self, input: &DeleteBucketPolicyRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?policy");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketPolicy");
        DeleteBucketPolicyRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the notification configuration of a bucket.
    pub fn get_bucket_notification_configuration(&self, input: &GetBucketNotificationConfigurationRequest) -> Result<NotificationConfiguration, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?notification");
        let mut params = Params::new();
        params.put("Action", "GetBucketNotificationConfiguration");
        GetBucketNotificationConfigurationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(NotificationConfigurationParser::parse_xml("NotificationConfiguration", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// This operation enables you to delete multiple objects from a bucket using a
    /// single HTTP request. You may specify up to 1000 keys.
    pub fn delete_objects(&self, input: &DeleteObjectsRequest) -> Result<DeleteObjectsOutput, AwsError> {
        // let mut uri = String::from("/");
        // uri = uri +  &input.key.to_string();
        // let mut request = SignedRequest::new("DELETE", "s3", self.region, &uri);
        // let mut params = Params::new();
        //
        // let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        // request.set_hostname(Some(hostname));
        //
        // params.put("Action", "DeleteObjects");
        // DeleteObjectsRequestWriter::write_params(&mut params, "", input);
        // request.set_params(params);
        // let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        // let status = result.status.to_u16();
        // match status {
        //  200 => {
        //      Ok(try!(DeleteObjectsOutputParser::parse_xml("DeleteObjectsOutput", &mut stack)))
        //  }
        //  _ => { Err(AwsError::new("error")) }
        // }
        Err(AwsError::new("not implemented"))
    }
    pub fn delete_bucket_replication(&self, input: &DeleteBucketReplicationRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?replication");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketReplication");
        DeleteBucketReplicationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Creates a copy of an object that is already stored in Amazon S3.
    pub fn copy_object(&self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}/{Key+}");
        let mut params = Params::new();
        params.put("Action", "CopyObject");
        CopyObjectRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(CopyObjectOutputParser::parse_xml("CopyObjectOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns a list of all buckets owned by the authenticated sender of the
    /// request.
    pub fn list_buckets(&self) -> Result<ListBucketsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/");
        let mut params = Params::new();
        params.put("Action", "ListBuckets");
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());

        stack.next(); // xml start tag

        match status {
            200 => {
                // was "ListBucketsOutput"
                Ok(try!(ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut stack)))
            }
            _ => { Err(AwsError::new("error in list_buckets")) }
        }
    }
    /// Sets the request payment configuration for a bucket. By default, the bucket
    /// owner pays for downloads from the bucket. This configuration parameter enables
    /// the bucket owner (only) to specify that the person requesting the download
    /// will be charged for the download. Documentation on requester pays buckets can
    /// be found at
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html
    pub fn put_bucket_request_payment(&self, input: &PutBucketRequestPaymentRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?requestPayment");
        let mut params = Params::new();
        params.put("Action", "PutBucketRequestPayment");
        PutBucketRequestPaymentRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Enables notifications of specified events for a bucket.
    pub fn put_bucket_notification_configuration(&self, input: &PutBucketNotificationConfigurationRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?notification");
        let mut params = Params::new();
        params.put("Action", "PutBucketNotificationConfiguration");
        PutBucketNotificationConfigurationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// The HEAD operation retrieves metadata from an object without returning the
    /// object itself. This operation is useful if you're only interested in an
    /// object's metadata. To use HEAD, you must have READ access to the object.
    pub fn head_object(&self, input: &HeadObjectRequest) -> Result<HeadObjectOutput, AwsError> {
        let mut request = SignedRequest::new("HEAD", "s3", self.region, "/{Bucket}/{Key+}");
        let mut params = Params::new();
        params.put("Action", "HeadObject");
        HeadObjectRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(HeadObjectOutputParser::parse_xml("HeadObjectOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deletes the tags from the bucket.
    pub fn delete_bucket_tagging(&self, input: &DeleteBucketTaggingRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?tagging");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketTagging");
        DeleteBucketTaggingRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Return torrent files from a bucket.
    pub fn get_object_torrent(&self, input: &GetObjectTorrentRequest) -> Result<GetObjectTorrentOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}/{Key+}?torrent");
        let mut params = Params::new();
        params.put("Action", "GetObjectTorrent");
        GetObjectTorrentRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetObjectTorrentOutputParser::parse_xml("GetObjectTorrentOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the lifecycle configuration information set on the bucket.
    pub fn get_bucket_lifecycle(&self, input: &GetBucketLifecycleRequest) -> Result<GetBucketLifecycleOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?lifecycle");
        let mut params = Params::new();
        params.put("Action", "GetBucketLifecycle");
        GetBucketLifecycleRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketLifecycleOutputParser::parse_xml("GetBucketLifecycleOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Creates a new bucket.
    /// All requests go to the us-east-1/us-standard endpoint, but can create buckets anywhere.
    pub fn create_bucket(&self, input: &CreateBucketRequest) -> Result<CreateBucketOutput, AwsError> {
        let region = Region::UsEast1;
        let mut create_config : Vec<u8>;
        let mut request = SignedRequest::new("PUT", "s3", region, "");
        let hostname = format!("{}.s3.amazonaws.com", input.bucket);
        request.set_hostname(Some(hostname));

        if needs_create_bucket_config(self.region) {
            create_config = create_bucket_config_xml(self.region);
            request.set_payload(Some(&create_config));
        }

        match input.acl {
            None => (),
            Some(ref canned_acl) => request.add_header("x-amz-acl", &canned_acl_in_aws_format(canned_acl)),
        }

        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                for header in result.headers.iter() {
                    if header.name() == "Location" {
                        return Ok(CreateBucketOutput{location: header.value_string()});
                    }
                }
                Err(AwsError::new("Something went wrong when creating a bucket."))
            }
            _ => {
                Err(AwsError::new("error in create_bucket"))
            }
        }
    }
    /// Completes a multipart upload by assembling previously uploaded parts.
    pub fn complete_multipart_upload(&self, input: &CompleteMultipartUploadRequest) -> Result<CompleteMultipartUploadOutput, AwsError> {
        let mut request = SignedRequest::new("POST", "s3", self.region,
            &format!("/{}", input.key));

        let mut params = Params::new();
        params.put("uploadId", &input.upload_id.to_string());
        request.set_params(params);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        request.set_payload(input.multipart_upload);

        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                let mut reader = EventReader::new(result);
                let mut stack = XmlResponseFromAws::new(reader.events().peekable());
                stack.next(); // xml start tag

                Ok(try!(CompleteMultipartUploadOutputParser::parse_xml("CompleteMultipartUploadResult", &mut stack)))
            }
            _ => {
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                Err(AwsError::new("error in complete_multipart_upload"))
            }
        }
    }
    /// Returns the website configuration for a bucket.
    pub fn get_bucket_website(&self, input: &GetBucketWebsiteRequest) -> Result<GetBucketWebsiteOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?website");
        let mut params = Params::new();
        params.put("Action", "GetBucketWebsite");
        GetBucketWebsiteRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketWebsiteOutputParser::parse_xml("GetBucketWebsiteOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Initiates a multipart upload and returns an upload ID.
    /// **Note:** After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.
    pub fn create_multipart_upload(&self, input: &CreateMultipartUploadRequest) -> Result<CreateMultipartUploadOutput, AwsError> {

        let object_name = &input.key;
        let mut request = SignedRequest::new("POST", "s3", self.region, &format!("/{}", object_name));

        let mut params = Params::new();
        params.put("uploads", "");
        request.set_params(params);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        match status {
            200 => {
                Ok(try!(CreateMultipartUploadOutputParser::parse_xml("InitiateMultipartUploadResult", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deletes the bucket. All objects (including all object versions and Delete
    /// Markers) in the bucket must be deleted before the bucket itself can be
    /// deleted.
    pub fn delete_bucket(&self, input: &DeleteBucketRequest, region: Region) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", region, "");

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        match status {
            204 => {
                Ok(())
            }
            _ => {
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                println!("resposne body: {}", body);
                Err(AwsError::new(format!("delete bucket error, status was {}", status)))
            }
        }
    }

    pub fn get_value_for_header(header_name: String, response: &Response) -> Result<String, AwsError> {
        for header in response.headers.iter() {
            if header.name() == header_name {
                return Ok(header.value_string());
            }
        }
        Ok(String::new())
        // Err(AwsError::new(format!("Couldn't find field {} in headers", header_name)))
    }

    /// Use the Hyper resposne to populate the GetObjectOutput
    // This would be a great candidate for some codegen magicks.
    pub fn get_object_from_response(response: &mut Response) -> Result<GetObjectOutput, AwsError> {
        // get all the goodies for GetObjectOutput
        let delete_marker_string = try!(S3Client::<P>::get_value_for_header("x-amz-delete-marker".to_string(), &response));
        let delete_marker : bool;
        if delete_marker_string.is_empty() {
            delete_marker = false;
        } else {
            delete_marker = try!(bool::from_str(&delete_marker_string));
        }
        let accept_ranges = try!(S3Client::<P>::get_value_for_header("accept-ranges".to_string(), response));
        let last_modified = try!(S3Client::<P>::get_value_for_header("Last-Modified".to_string(), response));
        let content_range = try!(S3Client::<P>::get_value_for_header("Content-Range".to_string(), response));
        let request_charged = try!(S3Client::<P>::get_value_for_header("x-amz-request-charged".to_string(), response));
        let content_encoding = try!(S3Client::<P>::get_value_for_header("Content-Encoding".to_string(), response));
        let replication_status = try!(S3Client::<P>::get_value_for_header("x-amz-replication-status".to_string(), response));
        let storage_class = try!(S3Client::<P>::get_value_for_header("x-amz-storage-class".to_string(), response));
        let server_side_encryption = try!(S3Client::<P>::get_value_for_header("x-amz-server-side-encryption".to_string(), response));
        let ssekms_key_id = try!(S3Client::<P>::get_value_for_header("x-amz-server-side-encryption-aws-kms-key-id".to_string(), response));
        let content_disposition = try!(S3Client::<P>::get_value_for_header("Content-Disposition".to_string(), response));
        let metadata = try!(S3Client::<P>::get_value_for_header("x-amz-meta-".to_string(), response));
        let website_redirect_location = try!(S3Client::<P>::get_value_for_header("x-amz-website-redirect-location".to_string(), response));
        let expires = try!(S3Client::<P>::get_value_for_header("Expires".to_string(), response));
        let cache_control = try!(S3Client::<P>::get_value_for_header("Cache-Control".to_string(), response));
        let content_length_string = try!(S3Client::<P>::get_value_for_header("Content-Length".to_string(), response));
        let content_length = try!(content_length_string.parse::<i32>());
        let expiration = try!(S3Client::<P>::get_value_for_header("x-amz-expiration".to_string(), response));
        let missing_meta_string = try!(S3Client::<P>::get_value_for_header("x-amz-missing-meta".to_string(), response));
        let missing_meta : i32;
        if missing_meta_string.is_empty() {
            missing_meta = 0;
        } else {
            missing_meta = try!(missing_meta_string.parse::<i32>());
        }
        let restore = try!(S3Client::<P>::get_value_for_header("x-amz-restore".to_string(), response));
        let sse_customer_algorithm = try!(S3Client::<P>::get_value_for_header("x-amz-server-side-encryption-customer-algorithm".to_string(), response));
        let content_type = try!(S3Client::<P>::get_value_for_header("Content-Type".to_string(), response));
        let content_language = try!(S3Client::<P>::get_value_for_header("Content-Language".to_string(), response));
        let version_id = try!(S3Client::<P>::get_value_for_header("x-amz-version-id".to_string(), response));
        let e_tag = try!(S3Client::<P>::get_value_for_header("ETag".to_string(), response));
        let sse_customer_key_md5 = try!(S3Client::<P>::get_value_for_header("x-amz-server-side-encryption-customer-key-MD5".to_string(), response));
        let mut body : Vec<u8> = Vec::new();
        try!(response.read_to_end(&mut body));
        // make the object to return
        let s3_object = GetObjectOutput {
            delete_marker: delete_marker,
            accept_ranges: accept_ranges,
            last_modified: last_modified,
            content_range: content_range,
            request_charged: request_charged,
            content_encoding: content_encoding,
            replication_status: replication_status,
            storage_class: storage_class,
            server_side_encryption: server_side_encryption,
            ssekms_key_id: ssekms_key_id,
            content_disposition: content_disposition,
            metadata: HashMap::new(),
            body: body,
            website_redirect_location: website_redirect_location,
            expires: expires,
            cache_control: cache_control,
            content_length: content_length,
            expiration: expiration,
            missing_meta: missing_meta,
            restore: restore,
            sse_customer_algorithm: sse_customer_algorithm,
            content_type: content_type,
            content_language: content_language,
            version_id: version_id,
            e_tag: e_tag,
            sse_customer_key_md5: sse_customer_key_md5,
        };
        Ok(s3_object)
    }

    /// Retrieves objects from Amazon S3.
    pub fn get_object(&self, input: &GetObjectRequest) -> Result<GetObjectOutput, AwsError> {
        let mut uri = String::from("/");
        uri = uri +  &input.key.to_string();
        let mut request = SignedRequest::new("GET", "s3", self.region, &uri);
        let mut params = Params::new();

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        params.put("Action", "GetObject");
        GetObjectRequestWriter::write_params(&mut params, "", input);

        request.set_params(params);
        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                let s3_object = try!(S3Client::<P>::get_object_from_response(&mut result));

                Ok(s3_object)
            }
            _ => {
                println!("Error: Status code was {}", status);
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                println!("Error response body: {}", body);
                Err(AwsError::new("error in get_object"))
            }
        }
    }

    /// Returns the policy of a specified bucket.
    pub fn get_bucket_policy(&self, input: &GetBucketPolicyRequest) -> Result<GetBucketPolicyOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?policy");
        let mut params = Params::new();
        params.put("Action", "GetBucketPolicy");
        GetBucketPolicyRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketPolicyOutputParser::parse_xml("GetBucketPolicyOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the versioning state of a bucket.
    pub fn get_bucket_versioning(&self, input: &GetBucketVersioningRequest) -> Result<GetBucketVersioningOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?versioning");
        let mut params = Params::new();
        params.put("Action", "GetBucketVersioning");
        GetBucketVersioningRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketVersioningOutputParser::parse_xml("GetBucketVersioningOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// This operation lists in-progress multipart uploads.
    pub fn list_multipart_uploads(&self, input: &ListMultipartUploadsRequest) -> Result<ListMultipartUploadsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/");

        let mut params = Params::new();
        params.put("uploads", "");
        request.set_params(params);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(ListMultipartUploadsOutputParser::parse_xml("ListMultipartUploadsResult", &mut stack)))
            }
            _ => {
                Err(AwsError::new("error"))
            }
        }
    }
    /// Returns the request payment configuration of a bucket.
    pub fn get_bucket_request_payment(&self, input: &GetBucketRequestPaymentRequest) -> Result<GetBucketRequestPaymentOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?requestPayment");
        let mut params = Params::new();
        params.put("Action", "GetBucketRequestPayment");
        GetBucketRequestPaymentRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketRequestPaymentOutputParser::parse_xml("GetBucketRequestPaymentOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Sets the tags for a bucket.
    pub fn put_bucket_tagging(&self, input: &PutBucketTaggingRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?tagging");
        let mut params = Params::new();
        params.put("Action", "PutBucketTagging");
        PutBucketTaggingRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the tag set associated with the bucket.
    pub fn get_bucket_tagging(&self, input: &GetBucketTaggingRequest) -> Result<GetBucketTaggingOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?tagging");
        let mut params = Params::new();
        params.put("Action", "GetBucketTagging");
        GetBucketTaggingRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketTaggingOutputParser::parse_xml("GetBucketTaggingOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Aborts a multipart upload.
    /// To verify that all parts have been removed, so you don't get charged for the
    /// part storage, you should call the List Parts operation and ensure the parts
    /// list is empty.
    pub fn abort_multipart_upload(&self, input: &AbortMultipartUploadRequest) -> Result<AbortMultipartUploadOutput, AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, &format!("/{}", input.key));

        let mut params = Params::new();
        params.put("uploadId", &input.upload_id.to_string());
        request.set_params(params);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            204 => {
                Ok(AbortMultipartUploadOutput::default())
            }
            _ => { Err(AwsError::new(format!("error, got return code {}", status))) }
        }
    }
    /// uses the acl subresource to set the access control list (ACL) permissions for
    /// an object that already exists in a bucket
    pub fn put_object_acl(&self, input: &PutObjectAclRequest) -> Result<PutObjectAclOutput, AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}/{Key+}?acl");
        let mut params = Params::new();
        params.put("Action", "PutObjectAcl");
        PutObjectAclRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(PutObjectAclOutputParser::parse_xml("PutObjectAclOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Returns the region the bucket resides in.
    pub fn get_bucket_location(&self, input: &GetBucketLocationRequest) -> Result<GetBucketLocationOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?location");
        let mut params = Params::new();
        params.put("Action", "GetBucketLocation");
        GetBucketLocationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketLocationOutputParser::parse_xml("GetBucketLocationOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Sets the cors configuration for a bucket.
    pub fn put_bucket_cors(&self, input: &PutBucketCorsRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("PUT", "s3", self.region, "/{Bucket}?cors");
        let mut params = Params::new();
        params.put("Action", "PutBucketCors");
        PutBucketCorsRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deletes the lifecycle configuration from the bucket.
    pub fn delete_bucket_lifecycle(&self, input: &DeleteBucketLifecycleRequest) -> Result<(), AwsError> {
        let mut request = SignedRequest::new("DELETE", "s3", self.region, "/{Bucket}?lifecycle");
        let mut params = Params::new();
        params.put("Action", "DeleteBucketLifecycle");
        DeleteBucketLifecycleRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(())
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Deprecated, see the GetBucketNotificationConfiguration operation.
    pub fn get_bucket_notification(&self, input: &GetBucketNotificationConfigurationRequest) -> Result<NotificationConfigurationDeprecated, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?notification");
        let mut params = Params::new();
        params.put("Action", "GetBucketNotification");
        GetBucketNotificationConfigurationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(NotificationConfigurationDeprecatedParser::parse_xml("NotificationConfigurationDeprecated", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Lists the parts that have been uploaded for a specific multipart upload.
    pub fn list_parts(&self, input: &ListPartsRequest) -> Result<ListPartsOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, &format!("/{}", input.key));

        let mut params = Params::new();
        params.put("uploadId", &input.upload_id.to_string());
        request.set_params(params);

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            200 => {
                let mut reader = EventReader::new(result);
                let mut stack = XmlResponseFromAws::new(reader.events().peekable());
                stack.next(); // xml start tag

                Ok(try!(ListPartsOutputParser::parse_xml("ListPartsResult", &mut stack)))
            }
            _ => {
                let mut body = String::new();
                try!(result.read_to_string(&mut body));
                println!("Error response body: {}", body);

                Err(AwsError::new("error in list_parts"))
            }
        }
    }
    /// Returns the access control list (ACL) of an object.
    pub fn get_object_acl(&self, input: &GetObjectAclRequest) -> Result<GetObjectAclOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}/{Key+}?acl");
        let mut params = Params::new();
        params.put("Action", "GetObjectAcl");
        GetObjectAclRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetObjectAclOutputParser::parse_xml("GetObjectAclOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    /// Uploads a part by copying data from an existing object as data source.
    // pub fn upload_part_copy(&self, input: &UploadPartCopyRequest) -> Result<bool, AwsError> {
    //  let ref part_number = input.part_number;
    //  let ref upload_id = input.upload_id;
    //  let ref object_id = input.key;
    //  let mut request = SignedRequest::new("PUT", "s3", self.region, &format!("/{}?partNumber={}&uploadId={}",
    //      object_id, part_number, upload_id));
    //
    //  let result = request.sign_and_execute(&self.credentials_provider.credentials());
    //  let status = result.status.to_u16();
    //
    //  match status {
    //      200 => {
    //          Ok(true)
    //      }
    //      _ => { Err(AwsError::new("error")) }
    //  }
    // }
    /// Removes the null version (if there is one) of an object and inserts a delete
    /// marker, which becomes the latest version of the object. If there isn't a null
    /// version, Amazon S3 does not remove any objects.
    pub fn delete_object(&self, input: &DeleteObjectRequest) -> Result<DeleteObjectOutput, AwsError> {
        let mut uri = String::from("/");
        uri = uri +  &input.key.to_string();
        let mut request = SignedRequest::new("DELETE", "s3", self.region, &uri);
        let mut params = Params::new();

        let hostname = (&input.bucket).to_string() + ".s3.amazonaws.com";
        request.set_hostname(Some(hostname));

        params.put("Action", "DeleteObject");
        DeleteObjectRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();

        match status {
            204 => {
                Ok(DeleteObjectOutput::default())
                // Ok(try!(DeleteObjectOutputParser::parse_xml("DeleteObjectOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("delete object error")) }
        }
    }
    /// Restores an archived copy of an object back into Amazon S3
    pub fn restore_object(&self, input: &RestoreObjectRequest) -> Result<RestoreObjectOutput, AwsError> {
        let mut request = SignedRequest::new("POST", "s3", self.region, "/{Bucket}/{Key+}?restore");
        let mut params = Params::new();
        params.put("Action", "RestoreObject");
        RestoreObjectRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(RestoreObjectOutputParser::parse_xml("RestoreObjectOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
    pub fn get_bucket_replication(&self, input: &GetBucketReplicationRequest) -> Result<GetBucketReplicationOutput, AwsError> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/{Bucket}?replication");
        let mut params = Params::new();
        params.put("Action", "GetBucketReplication");
        GetBucketReplicationRequestWriter::write_params(&mut params, "", input);
        request.set_params(params);
        let result = request.sign_and_execute(try!(self.credentials_provider.credentials()));
        let status = result.status.to_u16();
        let mut reader = EventReader::new(result);
        let mut stack = XmlResponseFromAws::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();
        match status {
            200 => {
                Ok(try!(GetBucketReplicationOutputParser::parse_xml("GetBucketReplicationOutput", &mut stack)))
            }
            _ => { Err(AwsError::new("error")) }
        }
    }
}

const CHUNK_TO_READ: usize = 5000;
const S3_MINIMUM_PART_SIZE: usize = 5242880;
// need to sort this out, but having issues going declaring a String here, not a str.
// static S3_REDUCED_REDUNDANCY: &'static str = "REDUCED_REDUNDANCY";

/// Wraps the generated S3 client with a higher level interface
pub struct S3Helper<P> where P: ProvideAwsCredentials {
    client: S3Client<P>,
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

impl<P> S3Helper<P> where P: ProvideAwsCredentials {
    /// Creates a new S3 helper
    pub fn new(credentials_provider: P, region: Region) -> S3Helper<P> {
        S3Helper {
            client: S3Client::new(credentials_provider, region)
        }
    }

    /// Lists buckets
    pub fn list_buckets(&self) -> Result<ListBucketsOutput, AwsError> {
        self.client.list_buckets()
    }

    /// Creates bucket in default us-east-1/us-standard region.
    pub fn create_bucket(&self, bucket_name: &str, canned_acl: Option<CannedAcl>) -> Result<CreateBucketOutput, AwsError> {
        self.create_bucket_in_region(bucket_name, Region::UsEast1, canned_acl)
    }

    /// Creates bucket in specified region.
    pub fn create_bucket_in_region(&self, bucket_name: &str, region: Region, canned_acl: Option<CannedAcl>) -> Result<CreateBucketOutput, AwsError> {
        let mut request = CreateBucketRequest::default();

        match region {
            Region::UsEast1 => {
                // us-east-1 is us-standard, don't send a location constraint:
                request.create_bucket_configuration = None;
            }
            _ => {
                let create_config = CreateBucketConfiguration {
                    location_constraint: region.to_string()
                };
                request.create_bucket_configuration = Some(create_config);
            }
        }
        request.bucket = bucket_name.to_string();

        request.acl = canned_acl;

        self.client.create_bucket(&request)
    }

    /// Deletes specified bucket
    pub fn delete_bucket(&self, bucket_name: &str, region: Region) -> Result<(), AwsError> {
        let mut request = DeleteBucketRequest::default();
        request.bucket = bucket_name.to_string();
        self.client.delete_bucket(&request, region)
    }

    /// Download a named object from bucket
    pub fn get_object(&self, bucket_name: &str, object_name: &str) ->  Result<GetObjectOutput, AwsError> {
        let mut request = GetObjectRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        self.client.get_object(&request)
    }

    /// Upload an object to specified bucket
    pub fn put_object(&self, bucket_name: &str, object_name: &str, object_as_bytes: &[u8]) ->  Result<PutObjectOutput, AwsError> {
        self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, false)
    }

    /// Helper: uploads object to specified bucket using reduced redudancy storage settings
    pub fn put_object_with_reduced_redundancy(&self, bucket_name: &str, object_name: &str, object_as_bytes: &[u8]) ->  Result<PutObjectOutput, AwsError> {
        self.put_object_with_optional_reduced_redundancy(bucket_name, object_name, object_as_bytes, true)
    }

    fn put_object_with_optional_reduced_redundancy(&self, bucket_name: &str, object_name: &str,
        object_as_bytes: &[u8], reduced_redundancy: bool) ->  Result<PutObjectOutput, AwsError> {

        let mut request = PutObjectRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        request.body = Some(object_as_bytes);
        if reduced_redundancy {
            request.storage_class = Some("REDUCED_REDUNDANCY".to_string());
        }
        self.put_object_with_request(&mut request)
    }

    /// Uploads object to specified S3 bucket with server side encryption at rest.
    pub fn put_object_with_aws_encryption(&self, bucket_name: &str, object_name: &str,
        object_as_bytes: &[u8]) ->  Result<PutObjectOutput, AwsError> {

        let mut request = PutObjectRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        request.body = Some(object_as_bytes);
        request.server_side_encryption = Some("AES256".to_string());

        self.put_object_with_request(&mut request)
    }

    /// Uploads object to specified S3 bucket using AWS KMS for key management of encryption at rest.
    pub fn put_object_with_kms_encryption(&self, bucket_name: &str, object_name: &str,
        object_as_bytes: &[u8], key_id: &str) ->  Result<PutObjectOutput, AwsError> {

        let mut request = PutObjectRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        request.body = Some(object_as_bytes);
        request.server_side_encryption = Some("aws:kms".to_string());
        request.ssekms_key_id = Some(key_id.to_string());

        self.put_object_with_request(&mut request)
    }

    /// Uploads object: lets sender specify options.
    /// The most generic of put_object: caller specifies the whole request.
    pub fn put_object_with_request(&self, request: &mut PutObjectRequest) -> Result<PutObjectOutput, AwsError> {
        // This may be where we do some basic sanity checking: ensure we have:
        // bucket name, region, object id, payload.

        // content_md5 hashing for everyone!
        let hash = hash(MD5, request.body.unwrap()).to_base64(STANDARD);

        self.client.put_object(request)
    }

    // TODO: does this make a copy of the object_as_reader or just transfers ownership to this?
    /// Uploads a multi-part object to specified bucket.  Allows for large file uploads.
    pub fn put_multipart_object<T: Read>(&self, bucket_name: &str, object_name: &str,
        object_as_reader: &mut T) -> Result<PutObjectOutput, AwsError> { // TODO: return type correct?

        // TODO: make helper function for object PUT requests that handles encryption, reduced redudancy, etc...

        let mut multipart_upload_request = CreateMultipartUploadRequest::default();
        multipart_upload_request.key = object_name.to_string();
        multipart_upload_request.bucket = bucket_name.to_string();

        // compiler warns about this line, it's not seeing its use later in this function:
        let mut upload_id : String;

        match self.client.create_multipart_upload(&multipart_upload_request) {
            Err(why) => {
                println!("Couldn't create multipart upload request: {:?}", why);
                return Err(AwsError::new("oops"));
            }
            Ok(response) => upload_id = response.upload_id.to_string(),
        }

        let mut buffered_reader = BufReader::new(object_as_reader);
        let mut parts_list : Vec<String>;

        match self.upload_chunks(&mut buffered_reader, bucket_name, &upload_id, object_name) {
            Err(why) => return Err(AwsError::new("oops in upload_chunks")),
            Ok(parts) => parts_list = parts,
        }

        let item_list : Vec<u8>;
        match multipart_upload_finish_xml(&parts_list) {
            Err(why) => return Err(AwsError::new("oops in multipart_upload_finish_xml")),
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
                return Err(AwsError::new("oops in complete multipart upload"));
            },
            Ok(_) => (), // TODO: return object output
        }

        Ok(PutObjectOutput::default())
    }

    fn upload_chunks<T: Read>(&self, buffered_reader: &mut BufReader<T>,
            bucket_name: &str, upload_id: &str, object_name: &str) -> Result<Vec<String>, AwsError> {

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
                            s3_chunk.push(*i);
                            bytes_copied += 1;
                        }
                    }

                    if s3_chunk.len() >= S3_MINIMUM_PART_SIZE || !more_chunks_to_go {
                        match self.upload_a_part(&s3_chunk, &part_number, bucket_name, upload_id, object_name) {
                            Err(why) => {
                                println!("Got error uploading a part: {:?}", why);
                                return Err(AwsError::new("oops in upload_chunks"));
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

    fn upload_a_part(&self, buffer: &[u8], part_number: &i32,
            bucket_name: &str, upload_id: &str, object_name: &str) -> Result<String, AwsError> {

        let mut upload_part_request = UploadPartRequest::default();
        upload_part_request.body = Some(buffer);

        let hash = hash(MD5, upload_part_request.body.unwrap()).to_base64(STANDARD);
        upload_part_request.content_md5 = Some(hash);

        upload_part_request.bucket = bucket_name.to_string();
        upload_part_request.upload_id = upload_id.to_string();
        upload_part_request.part_number = *part_number;
        upload_part_request.key = object_name.to_string();

        match self.client.upload_part(&upload_part_request) {
            Err(why) => {
                println!("Error uploading part: {:?}", why);
                Err(AwsError::new("oops in upload_a_part"))
            },
            Ok(response) => {
                Ok(response.to_string())
            }
        }
    }

    /// Lists multipart uploads not yet completed for specified bucket
    pub fn list_multipart_uploads_for_bucket(&self, bucket_name: &str) -> Result<ListMultipartUploadsOutput, AwsError> {
        let mut request = ListMultipartUploadsRequest::default();
        request.bucket = bucket_name.to_string();

        match self.client.list_multipart_uploads(&request) {
            Err(why) => Err(AwsError::new(format!("Couldn't do list_multipart_uploads: {:?}", why))),
            Ok(result) => Ok(result),
        }
    }

    /// Deletes specified object from specified bucket.
    pub fn delete_object(&self, bucket_name: &str, object_name: &str) ->  Result<DeleteObjectOutput, AwsError> {
        let mut request = DeleteObjectRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        self.client.delete_object(&request)
    }

    /// Abort multipart upload.
    pub fn abort_multipart_upload(&self, bucket_name: &str, object_name: &str, upload_id: &str) ->  Result<AbortMultipartUploadOutput, AwsError> {
        let mut request = AbortMultipartUploadRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        request.upload_id = upload_id.to_string();
        self.client.abort_multipart_upload(&request)
    }

    /// List parts from a multiupload request.
    pub fn multipart_upload_list_parts(&self, bucket_name: &str, object_name: &str, upload_id: &str) ->  Result<ListPartsOutput, AwsError> {
        let mut request = ListPartsRequest::default();
        request.key = object_name.to_string();
        request.bucket = bucket_name.to_string();
        request.upload_id = upload_id.to_string();
        self.client.list_parts(&request)
    }
}

/// Helper function to determine if a create config is needed.
pub fn needs_create_bucket_config(region: Region) -> bool {
    match region {
        Region::UsEast1 => false,
        _ => true,
    }
}

// This is a bit hacky to get functionality until we figure out an XML writing util.
/// Manually writes out bucket configuration (location constraint) in XML.
pub fn create_bucket_config_xml(region: Region) -> Vec<u8> {
    match region {
        Region::UsEast1 => {
            Vec::new() // shouldn't actually execute this: panic! or unreachable! this?
        }
        _ => {
            let xml = format!("<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
        <LocationConstraint>{}</LocationConstraint>
        </CreateBucketConfiguration >", region);
            xml.into_bytes()
        }
    }
}

/// Writes out XML with all the parts in it for S3 to complete.
pub fn multipart_upload_finish_xml(parts: &[String]) -> Result<Vec<u8>, AwsError> {
    if parts.len() < 1 {
        return Err(AwsError::new("Can't finish upload on 0 parts."));
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
    use std::io::BufReader;
    use std::fs::File;
    use std::str;

    use xml::reader::*;

    use region::Region;
    use super::*;
    use super::CompleteMultipartUploadOutputParser;
    use super::CreateMultipartUploadOutputParser;
    use super::ListBucketsOutputParser;
    use super::ListMultipartUploadsOutputParser;
    use super::ListPartsOutputParser;
    use xmlutil::*;

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
                let an_upload = &result.uploads[0];
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
        match create_bucket_config_xml(Region::UsWest2).len() {
            0 => panic!("us-west-2 should have bucket constraint."),
            _ => return,
        }
    }

    #[test]
    fn create_bucket_us_east_1_no_constraints() {
        match create_bucket_config_xml(Region::UsEast1).len() {
            0 => return,
            _ => panic!("us-east-1 should not have bucket constraint."),
        }
    }

    #[test]
    fn create_bucket_constraint_needed() {
        if !needs_create_bucket_config(Region::UsWest2) {
            panic!("us-west-2 should have bucket constraint.");
        }
    }

    #[test]
    fn create_bucket_no_constraint_needed() {
        if needs_create_bucket_config(Region::UsEast1) {
            panic!("us-east-1 should not have bucket constraint.");
        }
    }
}
