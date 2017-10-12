
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};


use std::str::FromStr;
use std::io::Write;
use xml::reader::ParserConfig;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use xml;
use xml::EventReader;
use xml::EventWriter;
use xml::reader::XmlEvent;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::xmlutil::{peek_at_name, characters, end_element, start_element, skip_tree};
enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
use md5;
use base64;
#[doc="Specifies the days since the initiation of an Incomplete Multipart Upload that Lifecycle will wait before permanently removing all parts of the upload."]
#[derive(Default,Debug)]
pub struct AbortIncompleteMultipartUpload {
    #[doc="Indicates the number of days that must pass since initiation for Lifecycle to abort an Incomplete Multipart Upload."]
    pub days_after_initiation: Option<i64>,
}

struct AbortIncompleteMultipartUploadDeserializer;
impl AbortIncompleteMultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AbortIncompleteMultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AbortIncompleteMultipartUpload::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DaysAfterInitiation" => {
                            obj.days_after_initiation =
                                Some(try!(DaysAfterInitiationDeserializer::deserialize("DaysAfterInitiation",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AbortIncompleteMultipartUploadSerializer;
impl AbortIncompleteMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AbortIncompleteMultipartUpload)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days_after_initiation {
            writer
                .write(xml::writer::XmlEvent::start_element("DaysAfterInitiation"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<String>,
}

struct AbortMultipartUploadOutputDeserializer;
impl AbortMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AbortMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = AbortMultipartUploadOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct AbortMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    pub upload_id: String,
}

#[derive(Default,Debug)]
pub struct AccelerateConfiguration {
    #[doc="The accelerate configuration of the bucket."]
    pub status: Option<String>,
}


pub struct AccelerateConfigurationSerializer;
impl AccelerateConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AccelerateConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.status {
            writer
                .write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct AccessControlPolicy {
    #[doc="A list of grants."]
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
}


pub struct AccessControlPolicySerializer;
impl AccessControlPolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AccessControlPolicy)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grants {
            &GrantsSerializer::serialize(&mut writer, "AccessControlList", value)?;
        }
        if let Some(ref value) = obj.owner {
            &OwnerSerializer::serialize(&mut writer, "Owner", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AccountIdDeserializer;
impl AccountIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AccountIdSerializer;
impl AccountIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct AllowedHeaderDeserializer;
impl AllowedHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AllowedHeaderSerializer;
impl AllowedHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct AllowedHeadersDeserializer;
impl AllowedHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedHeaderDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct AllowedHeadersSerializer;
impl AllowedHeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            AllowedHeaderSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct AllowedMethodDeserializer;
impl AllowedMethodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AllowedMethodSerializer;
impl AllowedMethodSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedMethodDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct AllowedMethodsSerializer;
impl AllowedMethodsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            AllowedMethodSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct AllowedOriginDeserializer;
impl AllowedOriginDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AllowedOriginSerializer;
impl AllowedOriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct AllowedOriginsDeserializer;
impl AllowedOriginsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AllowedOriginDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct AllowedOriginsSerializer;
impl AllowedOriginsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            AllowedOriginSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct AnalyticsAndOperator {
    #[doc="The prefix to use when evaluating an AND predicate."]
    pub prefix: Option<String>,
    #[doc="The list of tags to use when evaluating an AND predicate."]
    pub tags: Option<Vec<Tag>>,
}

struct AnalyticsAndOperatorDeserializer;
impl AnalyticsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AnalyticsAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsAndOperatorSerializer;
impl AnalyticsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AnalyticsAndOperator)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct AnalyticsConfiguration {
    #[doc="The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis."]
    pub filter: Option<AnalyticsFilter>,
    #[doc="The identifier used to represent an analytics configuration."]
    pub id: String,
    #[doc="If present, it indicates that data related to access patterns will be collected and made available to analyze the tradeoffs between different storage classes."]
    pub storage_class_analysis: StorageClassAnalysis,
}

struct AnalyticsConfigurationDeserializer;
impl AnalyticsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AnalyticsConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Filter" => {
                            obj.filter =
                                Some(try!(AnalyticsFilterDeserializer::deserialize("Filter",
                                                                                   stack)));
                        }
                        "Id" => {
                            obj.id = try!(AnalyticsIdDeserializer::deserialize("Id", stack));
                        }
                        "StorageClassAnalysis" => {
                            obj.storage_class_analysis =
                                try!(StorageClassAnalysisDeserializer::deserialize("StorageClassAnalysis",
                                                                                   stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsConfigurationSerializer;
impl AnalyticsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AnalyticsConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &AnalyticsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.id)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        StorageClassAnalysisSerializer::serialize(&mut writer,
                                                  "StorageClassAnalysis",
                                                  &obj.storage_class_analysis)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsConfigurationListDeserializer;
impl AnalyticsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<AnalyticsConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(AnalyticsConfigurationDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct AnalyticsExportDestination {
    #[doc="A destination signifying output to an S3 bucket."]
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

struct AnalyticsExportDestinationDeserializer;
impl AnalyticsExportDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AnalyticsExportDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsExportDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "S3BucketDestination" => {
                            obj.s3_bucket_destination =
                                try!(AnalyticsS3BucketDestinationDeserializer::deserialize("S3BucketDestination",
                                                                                           stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsExportDestinationSerializer;
impl AnalyticsExportDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AnalyticsExportDestination)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsS3BucketDestinationSerializer::serialize(&mut writer,
                                                          "S3BucketDestination",
                                                          &obj.s3_bucket_destination)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct AnalyticsFilter {
    #[doc="A conjunction (logical AND) of predicates, which is used in evaluating an analytics filter. The operator must have at least two predicates."]
    pub and: Option<AnalyticsAndOperator>,
    #[doc="The prefix to use when evaluating an analytics filter."]
    pub prefix: Option<String>,
    #[doc="The tag to use when evaluating an analytics filter."]
    pub tag: Option<Tag>,
}

struct AnalyticsFilterDeserializer;
impl AnalyticsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AnalyticsFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "And" => {
                            obj.and =
                                Some(try!(AnalyticsAndOperatorDeserializer::deserialize("And",
                                                                                        stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsFilterSerializer;
impl AnalyticsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AnalyticsFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &AnalyticsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsIdDeserializer;
impl AnalyticsIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsIdSerializer;
impl AnalyticsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct AnalyticsS3BucketDestination {
    #[doc="The Amazon resource name (ARN) of the bucket to which data is exported."]
    pub bucket: String,
    #[doc="The account ID that owns the destination bucket. If no account ID is provided, the owner will not be validated prior to exporting data."]
    pub bucket_account_id: Option<String>,
    #[doc="The file format used when exporting data to Amazon S3."]
    pub format: String,
    #[doc="The prefix to use when exporting data. The exported data begins with this prefix."]
    pub prefix: Option<String>,
}

struct AnalyticsS3BucketDestinationDeserializer;
impl AnalyticsS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AnalyticsS3BucketDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AnalyticsS3BucketDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                        }
                        "BucketAccountId" => {
                            obj.bucket_account_id =
                                Some(try!(AccountIdDeserializer::deserialize("BucketAccountId",
                                                                             stack)));
                        }
                        "Format" => {
                            obj.format =
                                try!(AnalyticsS3ExportFileFormatDeserializer::deserialize("Format",
                                                                                          stack));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsS3BucketDestinationSerializer;
impl AnalyticsS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &AnalyticsS3BucketDestination)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.bucket)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.bucket_account_id {
            writer
                .write(xml::writer::XmlEvent::start_element("BucketAccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("Format"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.format)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsS3ExportFileFormatDeserializer;
impl AnalyticsS3ExportFileFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct AnalyticsS3ExportFileFormatSerializer;
impl AnalyticsS3ExportFileFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

pub struct StreamingBody(Box<Read>);

impl fmt::Debug for StreamingBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Body: streaming content>")
    }
}

impl ::std::ops::Deref for StreamingBody {
    type Target = Box<Read>;

    fn deref(&self) -> &Box<Read> {
        &self.0
    }
}

impl ::std::ops::DerefMut for StreamingBody {
    fn deref_mut(&mut self) -> &mut Box<Read> {
        &mut self.0
    }
}

pub struct BodySerializer;
impl BodySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<u8>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = String::from_utf8(obj.to_vec()).expect("Not a UTF-8 string"))))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Bucket {
    #[doc="Date the bucket was created."]
    pub creation_date: Option<String>,
    #[doc="The name of the bucket."]
    pub name: Option<String>,
}

struct BucketDeserializer;
impl BucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Bucket::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CreationDate" => {
                            obj.creation_date =
                                Some(try!(CreationDateDeserializer::deserialize("CreationDate",
                                                                                stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(BucketNameDeserializer::deserialize("Name",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct BucketAccelerateStatusDeserializer;
impl BucketAccelerateStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct BucketAccelerateStatusSerializer;
impl BucketAccelerateStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct BucketLifecycleConfiguration {
    pub rules: Vec<LifecycleRule>,
}


pub struct BucketLifecycleConfigurationSerializer;
impl BucketLifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &BucketLifecycleConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        LifecycleRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketLocationConstraintDeserializer;
impl BucketLocationConstraintDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct BucketLocationConstraintSerializer;
impl BucketLocationConstraintSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}


pub struct BucketLoggingStatusSerializer;
impl BucketLoggingStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &BucketLoggingStatus)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.logging_enabled {
            &LoggingEnabledSerializer::serialize(&mut writer, "LoggingEnabled", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketLogsPermissionDeserializer;
impl BucketLogsPermissionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct BucketLogsPermissionSerializer;
impl BucketLogsPermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct BucketNameDeserializer;
impl BucketNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct BucketNameSerializer;
impl BucketNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct BucketVersioningStatusDeserializer;
impl BucketVersioningStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct BucketVersioningStatusSerializer;
impl BucketVersioningStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct BucketsDeserializer;
impl BucketsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Bucket>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Bucket" {
                        obj.push(try!(BucketDeserializer::deserialize("Bucket", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CORSConfiguration {
    pub cors_rules: Vec<CORSRule>,
}


pub struct CORSConfigurationSerializer;
impl CORSConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CORSConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        CORSRulesSerializer::serialize(&mut writer, "CORSRule", &obj.cors_rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct CORSRule {
    #[doc="Specifies which headers are allowed in a pre-flight OPTIONS request."]
    pub allowed_headers: Option<Vec<String>>,
    #[doc="Identifies HTTP methods that the domain/origin specified in the rule is allowed to execute."]
    pub allowed_methods: Vec<String>,
    #[doc="One or more origins you want customers to be able to access the bucket from."]
    pub allowed_origins: Vec<String>,
    #[doc="One or more headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript XMLHttpRequest object)."]
    pub expose_headers: Option<Vec<String>>,
    #[doc="The time in seconds that your browser is to cache the preflight response for the specified resource."]
    pub max_age_seconds: Option<i64>,
}

struct CORSRuleDeserializer;
impl CORSRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CORSRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CORSRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AllowedHeader" => {
                            obj.allowed_headers =
                                Some(try!(AllowedHeadersDeserializer::deserialize("AllowedHeader",
                                                                                  stack)));
                        }
                        "AllowedMethod" => {
                            obj.allowed_methods =
                                try!(AllowedMethodsDeserializer::deserialize("AllowedMethod",
                                                                             stack));
                        }
                        "AllowedOrigin" => {
                            obj.allowed_origins =
                                try!(AllowedOriginsDeserializer::deserialize("AllowedOrigin",
                                                                             stack));
                        }
                        "ExposeHeader" => {
                            obj.expose_headers =
                                Some(try!(ExposeHeadersDeserializer::deserialize("ExposeHeader",
                                                                                 stack)));
                        }
                        "MaxAgeSeconds" => {
                            obj.max_age_seconds =
                                Some(try!(MaxAgeSecondsDeserializer::deserialize("MaxAgeSeconds",
                                                                                 stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct CORSRuleSerializer;
impl CORSRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CORSRule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allowed_headers {
            &AllowedHeadersSerializer::serialize(&mut writer, "AllowedHeader", value)?;
        }
        AllowedMethodsSerializer::serialize(&mut writer, "AllowedMethod", &obj.allowed_methods)?;
        AllowedOriginsSerializer::serialize(&mut writer, "AllowedOrigin", &obj.allowed_origins)?;
        if let Some(ref value) = obj.expose_headers {
            &ExposeHeadersSerializer::serialize(&mut writer, "ExposeHeader", value)?;
        }
        if let Some(ref value) = obj.max_age_seconds {
            writer
                .write(xml::writer::XmlEvent::start_element("MaxAgeSeconds"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CORSRulesDeserializer;
impl CORSRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<CORSRule>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(CORSRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct CORSRulesSerializer;
impl CORSRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<CORSRule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            CORSRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct CloudFunctionDeserializer;
impl CloudFunctionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct CloudFunctionSerializer;
impl CloudFunctionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct CloudFunctionConfiguration {
    pub cloud_function: Option<String>,
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    pub invocation_role: Option<String>,
}

struct CloudFunctionConfigurationDeserializer;
impl CloudFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CloudFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CloudFunctionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CloudFunction" => {
                            obj.cloud_function =
                                Some(try!(CloudFunctionDeserializer::deserialize("CloudFunction",
                                                                                 stack)));
                        }
                        "Event" => {
                            obj.events = Some(try!(EventListDeserializer::deserialize("Event",
                                                                                      stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "InvocationRole" => {
                            obj.invocation_role = Some(try!(CloudFunctionInvocationRoleDeserializer::deserialize("InvocationRole", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct CloudFunctionConfigurationSerializer;
impl CloudFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CloudFunctionConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function {
            writer
                .write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.invocation_role {
            writer
                .write(xml::writer::XmlEvent::start_element("InvocationRole"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CloudFunctionInvocationRoleDeserializer;
impl CloudFunctionInvocationRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct CloudFunctionInvocationRoleSerializer;
impl CloudFunctionInvocationRoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct CodeDeserializer;
impl CodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CommonPrefix {
    pub prefix: Option<String>,
}

struct CommonPrefixDeserializer;
impl CommonPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CommonPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CommonPrefix::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct CommonPrefixListDeserializer;
impl CommonPrefixListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<CommonPrefix>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(CommonPrefixDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CompleteMultipartUploadOutput {
    pub bucket: Option<String>,
    #[doc="Entity tag of the object."]
    pub e_tag: Option<String>,
    #[doc="If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded."]
    pub expiration: Option<String>,
    pub key: Option<String>,
    pub location: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="Version of the object."]
    pub version_id: Option<String>,
}

struct CompleteMultipartUploadOutputDeserializer;
impl CompleteMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CompleteMultipartUploadOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = Some(try!(BucketNameDeserializer::deserialize("Bucket",
                                                                                       stack)));
                        }
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "Location" => {
                            obj.location = Some(try!(LocationDeserializer::deserialize("Location",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CompleteMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub request_payer: Option<String>,
    pub upload_id: String,
}

#[derive(Default,Debug)]
pub struct CompletedMultipartUpload {
    pub parts: Option<Vec<CompletedPart>>,
}


pub struct CompletedMultipartUploadSerializer;
impl CompletedMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CompletedMultipartUpload)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.parts {
            &CompletedPartListSerializer::serialize(&mut writer, "Part", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct CompletedPart {
    #[doc="Entity tag returned when the part was uploaded."]
    pub e_tag: Option<String>,
    #[doc="Part number that identifies the part. This is a positive integer between 1 and 10,000."]
    pub part_number: Option<i64>,
}


pub struct CompletedPartSerializer;
impl CompletedPartSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CompletedPart)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.e_tag {
            writer.write(xml::writer::XmlEvent::start_element("ETag"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.part_number {
            writer
                .write(xml::writer::XmlEvent::start_element("PartNumber"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}


pub struct CompletedPartListSerializer;
impl CompletedPartListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<CompletedPart>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            CompletedPartSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct Condition {
    #[doc="The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element Condition is specified and sibling KeyPrefixEquals is not specified. If both are specified, then both must be true for the redirect to be applied."]
    pub http_error_code_returned_equals: Option<String>,
    #[doc="The object key name prefix when the redirect is applied. For example, to redirect requests for ExamplePage.html, the key prefix will be ExamplePage.html. To redirect request for all pages with the prefix docs/, the key prefix will be /docs, which identifies all objects in the docs/ folder. Required when the parent element Condition is specified and sibling HttpErrorCodeReturnedEquals is not specified. If both conditions are specified, both must be true for the redirect to be applied."]
    pub key_prefix_equals: Option<String>,
}

struct ConditionDeserializer;
impl ConditionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Condition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Condition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HttpErrorCodeReturnedEquals" => {
                            obj.http_error_code_returned_equals = Some(try!(HttpErrorCodeReturnedEqualsDeserializer::deserialize("HttpErrorCodeReturnedEquals", stack)));
                        }
                        "KeyPrefixEquals" => {
                            obj.key_prefix_equals =
                                Some(try!(KeyPrefixEqualsDeserializer::deserialize("KeyPrefixEquals",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ConditionSerializer;
impl ConditionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Condition)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.http_error_code_returned_equals {
            writer
                .write(xml::writer::XmlEvent::start_element("HttpErrorCodeReturnedEquals"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.key_prefix_equals {
            writer
                .write(xml::writer::XmlEvent::start_element("KeyPrefixEquals"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct CopyObjectOutput {
    pub copy_object_result: Option<CopyObjectResult>,
    pub copy_source_version_id: Option<String>,
    #[doc="If the object expiration is configured, the response includes this header."]
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="Version ID of the newly created copy."]
    pub version_id: Option<String>,
}

struct CopyObjectOutputDeserializer;
impl CopyObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CopyObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyObjectOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CopyObjectResult" => {
                            obj.copy_object_result =
                                Some(try!(CopyObjectResultDeserializer::deserialize("CopyObjectResult",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CopyObjectRequest {
    #[doc="The canned ACL to apply to the object."]
    pub acl: Option<String>,
    pub bucket: String,
    #[doc="Specifies caching behavior along the request/reply chain."]
    pub cache_control: Option<String>,
    #[doc="Specifies presentational information for the object."]
    pub content_disposition: Option<String>,
    #[doc="Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field."]
    pub content_encoding: Option<String>,
    #[doc="The language the content is in."]
    pub content_language: Option<String>,
    #[doc="A standard MIME type describing the format of the object data."]
    pub content_type: Option<String>,
    #[doc="The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded."]
    pub copy_source: String,
    #[doc="Copies the object if its entity tag (ETag) matches the specified tag."]
    pub copy_source_if_match: Option<String>,
    #[doc="Copies the object if it has been modified since the specified time."]
    pub copy_source_if_modified_since: Option<String>,
    #[doc="Copies the object if its entity tag (ETag) is different than the specified ETag."]
    pub copy_source_if_none_match: Option<String>,
    #[doc="Copies the object if it hasn't been modified since the specified time."]
    pub copy_source_if_unmodified_since: Option<String>,
    #[doc="Specifies the algorithm to use when decrypting the source object (e.g., AES256)."]
    pub copy_source_sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created."]
    pub copy_source_sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub copy_source_sse_customer_key_md5: Option<String>,
    #[doc="The date and time at which the object is no longer cacheable."]
    pub expires: Option<String>,
    #[doc="Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to read the object data and its metadata."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the object ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable object."]
    pub grant_write_acp: Option<String>,
    pub key: String,
    #[doc="A map of metadata to store with the object in S3."]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[doc="Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request."]
    pub metadata_directive: Option<String>,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version"]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="The type of storage to use for the object. Defaults to 'STANDARD'."]
    pub storage_class: Option<String>,
    #[doc="The tag-set for the object destination object this value must be used in conjunction with the TaggingDirective. The tag-set must be encoded as URL Query parameters"]
    pub tagging: Option<String>,
    #[doc="Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request."]
    pub tagging_directive: Option<String>,
    #[doc="If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata."]
    pub website_redirect_location: Option<String>,
}

#[derive(Default,Debug)]
pub struct CopyObjectResult {
    pub e_tag: Option<String>,
    pub last_modified: Option<String>,
}

struct CopyObjectResultDeserializer;
impl CopyObjectResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CopyObjectResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyObjectResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CopyPartResult {
    #[doc="Entity tag of the object."]
    pub e_tag: Option<String>,
    #[doc="Date and time at which the object was uploaded."]
    pub last_modified: Option<String>,
}

struct CopyPartResultDeserializer;
impl CopyPartResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CopyPartResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopyPartResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CreateBucketConfiguration {
    #[doc="Specifies the region where the bucket will be created. If you don't specify a region, the bucket will be created in US Standard."]
    pub location_constraint: Option<String>,
}


pub struct CreateBucketConfigurationSerializer;
impl CreateBucketConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &CreateBucketConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.location_constraint {
            writer
                .write(xml::writer::XmlEvent::start_element("LocationConstraint"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct CreateBucketOutput {
    pub location: Option<String>,
}

struct CreateBucketOutputDeserializer;
impl CreateBucketOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateBucketOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = CreateBucketOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CreateBucketRequest {
    #[doc="The canned ACL to apply to the bucket."]
    pub acl: Option<String>,
    pub bucket: String,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    #[doc="Allows grantee the read, write, read ACP, and write ACP permissions on the bucket."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to list the objects in the bucket."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the bucket ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to create, overwrite, and delete any object in the bucket."]
    pub grant_write: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable bucket."]
    pub grant_write_acp: Option<String>,
}

#[derive(Default,Debug)]
pub struct CreateMultipartUploadOutput {
    #[doc="Date when multipart upload will become eligible for abort operation by lifecycle."]
    pub abort_date: Option<String>,
    #[doc="Id of the lifecycle rule that makes a multipart upload eligible for abort operation."]
    pub abort_rule_id: Option<String>,
    #[doc="Name of the bucket to which the multipart upload was initiated."]
    pub bucket: Option<String>,
    #[doc="Object key for which the multipart upload was initiated."]
    pub key: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="ID for the initiated multipart upload."]
    pub upload_id: Option<String>,
}

struct CreateMultipartUploadOutputDeserializer;
impl CreateMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateMultipartUploadOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = Some(try!(BucketNameDeserializer::deserialize("Bucket",
                                                                                       stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "UploadId" => {
                            obj.upload_id =
                                Some(try!(MultipartUploadIdDeserializer::deserialize("UploadId",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct CreateMultipartUploadRequest {
    #[doc="The canned ACL to apply to the object."]
    pub acl: Option<String>,
    pub bucket: String,
    #[doc="Specifies caching behavior along the request/reply chain."]
    pub cache_control: Option<String>,
    #[doc="Specifies presentational information for the object."]
    pub content_disposition: Option<String>,
    #[doc="Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field."]
    pub content_encoding: Option<String>,
    #[doc="The language the content is in."]
    pub content_language: Option<String>,
    #[doc="A standard MIME type describing the format of the object data."]
    pub content_type: Option<String>,
    #[doc="The date and time at which the object is no longer cacheable."]
    pub expires: Option<String>,
    #[doc="Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to read the object data and its metadata."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the object ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable object."]
    pub grant_write_acp: Option<String>,
    pub key: String,
    #[doc="A map of metadata to store with the object in S3."]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version"]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="The type of storage to use for the object. Defaults to 'STANDARD'."]
    pub storage_class: Option<String>,
    #[doc="The tag-set for the object. The tag-set must be encoded as URL Query parameters"]
    pub tagging: Option<String>,
    #[doc="If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata."]
    pub website_redirect_location: Option<String>,
}

struct CreationDateDeserializer;
impl CreationDateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct DateDeserializer;
impl DateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DateSerializer;
impl DateSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct DaysDeserializer;
impl DaysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DaysSerializer;
impl DaysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct DaysAfterInitiationDeserializer;
impl DaysAfterInitiationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DaysAfterInitiationSerializer;
impl DaysAfterInitiationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Delete {
    pub objects: Vec<ObjectIdentifier>,
    #[doc="Element to enable quiet mode for the request. When you add this element, you must set its value to true."]
    pub quiet: Option<bool>,
}


pub struct DeleteSerializer;
impl DeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Delete)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ObjectIdentifierListSerializer::serialize(&mut writer, "Object", &obj.objects)?;
        if let Some(ref value) = obj.quiet {
            writer.write(xml::writer::XmlEvent::start_element("Quiet"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    #[doc="The name of the bucket from which an analytics configuration is deleted."]
    pub bucket: String,
    #[doc="The identifier used to represent an analytics configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketCorsRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketInventoryConfigurationRequest {
    #[doc="The name of the bucket containing the inventory configuration to delete."]
    pub bucket: String,
    #[doc="The ID used to identify the inventory configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketMetricsConfigurationRequest {
    #[doc="The name of the bucket containing the metrics configuration to delete."]
    pub bucket: String,
    #[doc="The ID used to identify the metrics configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketReplicationRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketTaggingRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct DeleteBucketWebsiteRequest {
    pub bucket: String,
}

struct DeleteMarkerDeserializer;
impl DeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct DeleteMarkerEntry {
    #[doc="Specifies whether the object is (true) or is not (false) the latest version of an object."]
    pub is_latest: Option<bool>,
    #[doc="The object key."]
    pub key: Option<String>,
    #[doc="Date and time the object was last modified."]
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    #[doc="Version ID of an object."]
    pub version_id: Option<String>,
}

struct DeleteMarkerEntryDeserializer;
impl DeleteMarkerEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteMarkerEntry, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteMarkerEntry::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IsLatest" => {
                            obj.is_latest = Some(try!(IsLatestDeserializer::deserialize("IsLatest",
                                                                                        stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        "VersionId" => {
                            obj.version_id =
                                Some(try!(ObjectVersionIdDeserializer::deserialize("VersionId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct DeleteMarkerVersionIdDeserializer;
impl DeleteMarkerVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct DeleteMarkersDeserializer;
impl DeleteMarkersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<DeleteMarkerEntry>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(DeleteMarkerEntryDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct DeleteObjectOutput {
    #[doc="Specifies whether the versioned object that was permanently deleted was (true) or was not (false) a delete marker."]
    pub delete_marker: Option<bool>,
    pub request_charged: Option<String>,
    #[doc="Returns the version ID of the delete marker created as a result of the DELETE operation."]
    pub version_id: Option<String>,
}

struct DeleteObjectOutputDeserializer;
impl DeleteObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct DeleteObjectRequest {
    pub bucket: String,
    pub key: String,
    #[doc="The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device."]
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
    #[doc="VersionId used to reference a specific version of the object."]
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct DeleteObjectTaggingOutput {
    #[doc="The versionId of the object the tag-set was removed from."]
    pub version_id: Option<String>,
}

struct DeleteObjectTaggingOutputDeserializer;
impl DeleteObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteObjectTaggingOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct DeleteObjectTaggingRequest {
    pub bucket: String,
    pub key: String,
    #[doc="The versionId of the object that the tag-set will be removed from."]
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct DeleteObjectsOutput {
    pub deleted: Option<Vec<DeletedObject>>,
    pub errors: Option<Vec<S3Error>>,
    pub request_charged: Option<String>,
}

struct DeleteObjectsOutputDeserializer;
impl DeleteObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteObjectsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Deleted" => {
                            obj.deleted =
                                Some(try!(DeletedObjectsDeserializer::deserialize("Deleted",
                                                                                  stack)));
                        }
                        "Error" => {
                            obj.errors = Some(try!(ErrorsDeserializer::deserialize("Error",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct DeleteObjectsRequest {
    pub bucket: String,
    pub delete: Delete,
    #[doc="The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device."]
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
}

#[derive(Default,Debug)]
pub struct DeletedObject {
    pub delete_marker: Option<bool>,
    pub delete_marker_version_id: Option<String>,
    pub key: Option<String>,
    pub version_id: Option<String>,
}

struct DeletedObjectDeserializer;
impl DeletedObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeletedObject, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeletedObject::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DeleteMarker" => {
                            obj.delete_marker =
                                Some(try!(DeleteMarkerDeserializer::deserialize("DeleteMarker",
                                                                                stack)));
                        }
                        "DeleteMarkerVersionId" => {
                            obj.delete_marker_version_id =
                                Some(try!(DeleteMarkerVersionIdDeserializer::deserialize("DeleteMarkerVersionId",
                                                                                         stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "VersionId" => {
                            obj.version_id =
                                Some(try!(ObjectVersionIdDeserializer::deserialize("VersionId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct DeletedObjectsDeserializer;
impl DeletedObjectsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<DeletedObject>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(DeletedObjectDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct DelimiterDeserializer;
impl DelimiterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DelimiterSerializer;
impl DelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Destination {
    #[doc="Amazon resource name (ARN) of the bucket where you want Amazon S3 to store replicas of the object identified by the rule."]
    pub bucket: String,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
}

struct DestinationDeserializer;
impl DestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Destination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Destination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(StorageClassDeserializer::deserialize("StorageClass",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DestinationSerializer;
impl DestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Destination)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.bucket)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.storage_class {
            writer
                .write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DisplayNameDeserializer;
impl DisplayNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct DisplayNameSerializer;
impl DisplayNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ETagDeserializer;
impl ETagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ETagSerializer;
impl ETagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct EmailAddressDeserializer;
impl EmailAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct EmailAddressSerializer;
impl EmailAddressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct EncodingTypeDeserializer;
impl EncodingTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct EncodingTypeSerializer;
impl EncodingTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct S3Error {
    pub code: Option<String>,
    pub key: Option<String>,
    pub message: Option<String>,
    pub version_id: Option<String>,
}

struct S3ErrorDeserializer;
impl S3ErrorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<S3Error, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3Error::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Code" => {
                            obj.code = Some(try!(CodeDeserializer::deserialize("Code", stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "Message" => {
                            obj.message = Some(try!(MessageDeserializer::deserialize("Message",
                                                                                     stack)));
                        }
                        "VersionId" => {
                            obj.version_id =
                                Some(try!(ObjectVersionIdDeserializer::deserialize("VersionId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ErrorDocument {
    #[doc="The object key name to use when a 4XX class error occurs."]
    pub key: String,
}

struct ErrorDocumentDeserializer;
impl ErrorDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ErrorDocument, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ErrorDocument::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Key" => {
                            obj.key = try!(ObjectKeyDeserializer::deserialize("Key", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ErrorDocumentSerializer;
impl ErrorDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &ErrorDocument)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.key)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ErrorsDeserializer;
impl ErrorsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<S3Error>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(S3ErrorDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct EventSerializer;
impl EventSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(EventDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct EventListSerializer;
impl EventListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            EventSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct ExpirationStatusDeserializer;
impl ExpirationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ExpirationStatusSerializer;
impl ExpirationStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ExpiredObjectDeleteMarkerDeserializer;
impl ExpiredObjectDeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ExpiredObjectDeleteMarkerSerializer;
impl ExpiredObjectDeleteMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &bool)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ExposeHeaderDeserializer;
impl ExposeHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ExposeHeaderSerializer;
impl ExposeHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ExposeHeadersDeserializer;
impl ExposeHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ExposeHeaderDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct ExposeHeadersSerializer;
impl ExposeHeadersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            ExposeHeaderSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}


pub struct FetchOwnerSerializer;
impl FetchOwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &bool)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[doc="Container for key value pair that defines the criteria for the filter rule."]
#[derive(Default,Debug)]
pub struct FilterRule {
    #[doc="Object key name prefix or suffix identifying one or more objects to which the filtering rule applies. Maximum prefix length can be up to 1,024 characters. Overlapping prefixes and suffixes are not supported. For more information, go to <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html\">Configuring Event Notifications</a> in the Amazon Simple Storage Service Developer Guide."]
    pub name: Option<String>,
    pub value: Option<String>,
}

struct FilterRuleDeserializer;
impl FilterRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<FilterRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = FilterRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Name" => {
                            obj.name = Some(try!(FilterRuleNameDeserializer::deserialize("Name",
                                                                                         stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(FilterRuleValueDeserializer::deserialize("Value",
                                                                                           stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct FilterRuleSerializer;
impl FilterRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &FilterRule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.name {
            writer.write(xml::writer::XmlEvent::start_element("Name"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FilterRuleListDeserializer;
impl FilterRuleListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<FilterRule>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(FilterRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct FilterRuleListSerializer;
impl FilterRuleListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<FilterRule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            FilterRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct FilterRuleNameDeserializer;
impl FilterRuleNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct FilterRuleNameSerializer;
impl FilterRuleNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct FilterRuleValueDeserializer;
impl FilterRuleValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct FilterRuleValueSerializer;
impl FilterRuleValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct GetBucketAccelerateConfigurationOutput {
    #[doc="The accelerate configuration of the bucket."]
    pub status: Option<String>,
}

struct GetBucketAccelerateConfigurationOutputDeserializer;
impl GetBucketAccelerateConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<GetBucketAccelerateConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAccelerateConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Status" => {
                            obj.status =
                                Some(try!(BucketAccelerateStatusDeserializer::deserialize("Status",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketAccelerateConfigurationRequest {
    #[doc="Name of the bucket for which the accelerate configuration is retrieved."]
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketAclOutput {
    #[doc="A list of grants."]
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
}

struct GetBucketAclOutputDeserializer;
impl GetBucketAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAclOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AccessControlList" => {
                            obj.grants = Some(try!(GrantsDeserializer::deserialize("AccessControlList",
                                                                                   stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketAclRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketAnalyticsConfigurationOutput {
    #[doc="The configuration and any analyses for the analytics filter."]
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

struct GetBucketAnalyticsConfigurationOutputDeserializer;
impl GetBucketAnalyticsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<GetBucketAnalyticsConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketAnalyticsConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AnalyticsConfiguration" => {
                            obj.analytics_configuration =
                                Some(try!(AnalyticsConfigurationDeserializer::deserialize("AnalyticsConfiguration",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketAnalyticsConfigurationRequest {
    #[doc="The name of the bucket from which an analytics configuration is retrieved."]
    pub bucket: String,
    #[doc="The identifier used to represent an analytics configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Option<Vec<CORSRule>>,
}

struct GetBucketCorsOutputDeserializer;
impl GetBucketCorsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketCorsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketCorsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CORSRule" => {
                            obj.cors_rules = Some(try!(CORSRulesDeserializer::deserialize("CORSRule",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketCorsRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketInventoryConfigurationOutput {
    #[doc="Specifies the inventory configuration."]
    pub inventory_configuration: Option<InventoryConfiguration>,
}

struct GetBucketInventoryConfigurationOutputDeserializer;
impl GetBucketInventoryConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<GetBucketInventoryConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketInventoryConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "InventoryConfiguration" => {
                            obj.inventory_configuration =
                                Some(try!(InventoryConfigurationDeserializer::deserialize("InventoryConfiguration",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketInventoryConfigurationRequest {
    #[doc="The name of the bucket containing the inventory configuration to retrieve."]
    pub bucket: String,
    #[doc="The ID used to identify the inventory configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct GetBucketLifecycleConfigurationOutput {
    pub rules: Option<Vec<LifecycleRule>>,
}

struct GetBucketLifecycleConfigurationOutputDeserializer;
impl GetBucketLifecycleConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<GetBucketLifecycleConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLifecycleConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Rule" => {
                            obj.rules = Some(try!(LifecycleRulesDeserializer::deserialize("Rule",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketLifecycleOutput {
    pub rules: Option<Vec<Rule>>,
}

struct GetBucketLifecycleOutputDeserializer;
impl GetBucketLifecycleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketLifecycleOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLifecycleOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Rule" => {
                            obj.rules = Some(try!(RulesDeserializer::deserialize("Rule", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketLifecycleRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<String>,
}

struct GetBucketLocationOutputDeserializer;
impl GetBucketLocationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketLocationOutput, XmlParseError> {
        let mut obj = GetBucketLocationOutput::default();
        obj.location_constraint =
            Some(try!(BucketLocationConstraintDeserializer::deserialize("LocationConstraint",
                                                                        stack)));
        Ok(obj)
    }
}
#[derive(Default,Debug)]
pub struct GetBucketLocationRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

struct GetBucketLoggingOutputDeserializer;
impl GetBucketLoggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketLoggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketLoggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "LoggingEnabled" => {
                            obj.logging_enabled =
                                Some(try!(LoggingEnabledDeserializer::deserialize("LoggingEnabled",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketLoggingRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketMetricsConfigurationOutput {
    #[doc="Specifies the metrics configuration."]
    pub metrics_configuration: Option<MetricsConfiguration>,
}

struct GetBucketMetricsConfigurationOutputDeserializer;
impl GetBucketMetricsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<GetBucketMetricsConfigurationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketMetricsConfigurationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "MetricsConfiguration" => {
                            obj.metrics_configuration =
                                Some(try!(MetricsConfigurationDeserializer::deserialize("MetricsConfiguration",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketMetricsConfigurationRequest {
    #[doc="The name of the bucket containing the metrics configuration to retrieve."]
    pub bucket: String,
    #[doc="The ID used to identify the metrics configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct GetBucketNotificationConfigurationRequest {
    #[doc="Name of the bucket to get the notification configuration for."]
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketPolicyOutput {
    #[doc="The bucket policy as a JSON document."]
    pub policy: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetBucketPolicyRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

struct GetBucketReplicationOutputDeserializer;
impl GetBucketReplicationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketReplicationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketReplicationOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ReplicationConfiguration" => {
                            obj.replication_configuration = Some(try!(ReplicationConfigurationDeserializer::deserialize("ReplicationConfiguration", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketReplicationRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketRequestPaymentOutput {
    #[doc="Specifies who pays for the download and request fees."]
    pub payer: Option<String>,
}

struct GetBucketRequestPaymentOutputDeserializer;
impl GetBucketRequestPaymentOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketRequestPaymentOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Payer" => {
                            obj.payer = Some(try!(PayerDeserializer::deserialize("Payer", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketRequestPaymentRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketTaggingOutput {
    pub tag_set: Vec<Tag>,
}

struct GetBucketTaggingOutputDeserializer;
impl GetBucketTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketTaggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TagSet" => {
                            obj.tag_set = try!(TagSetDeserializer::deserialize("TagSet", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketTaggingRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketVersioningOutput {
    #[doc="Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned."]
    pub mfa_delete: Option<String>,
    #[doc="The versioning state of the bucket."]
    pub status: Option<String>,
}

struct GetBucketVersioningOutputDeserializer;
impl GetBucketVersioningOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketVersioningOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketVersioningOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "MfaDelete" => {
                            obj.mfa_delete =
                                Some(try!(MFADeleteStatusDeserializer::deserialize("MfaDelete",
                                                                                   stack)));
                        }
                        "Status" => {
                            obj.status =
                                Some(try!(BucketVersioningStatusDeserializer::deserialize("Status",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketVersioningRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetBucketWebsiteOutput {
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub routing_rules: Option<Vec<RoutingRule>>,
}

struct GetBucketWebsiteOutputDeserializer;
impl GetBucketWebsiteOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetBucketWebsiteOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetBucketWebsiteOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ErrorDocument" => {
                            obj.error_document =
                                Some(try!(ErrorDocumentDeserializer::deserialize("ErrorDocument",
                                                                                 stack)));
                        }
                        "IndexDocument" => {
                            obj.index_document =
                                Some(try!(IndexDocumentDeserializer::deserialize("IndexDocument",
                                                                                 stack)));
                        }
                        "RedirectAllRequestsTo" => {
                            obj.redirect_all_requests_to =
                                Some(try!(RedirectAllRequestsToDeserializer::deserialize("RedirectAllRequestsTo",
                                                                                         stack)));
                        }
                        "RoutingRules" => {
                            obj.routing_rules =
                                Some(try!(RoutingRulesDeserializer::deserialize("RoutingRules",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetBucketWebsiteRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct GetObjectAclOutput {
    #[doc="A list of grants."]
    pub grants: Option<Vec<Grant>>,
    pub owner: Option<Owner>,
    pub request_charged: Option<String>,
}

struct GetObjectAclOutputDeserializer;
impl GetObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetObjectAclOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AccessControlList" => {
                            obj.grants = Some(try!(GrantsDeserializer::deserialize("AccessControlList",
                                                                                   stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetObjectAclRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    #[doc="VersionId used to reference a specific version of the object."]
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetObjectOutput {
    pub accept_ranges: Option<String>,
    #[doc="Object data."]
    pub body: Option<StreamingBody>,
    #[doc="Specifies caching behavior along the request/reply chain."]
    pub cache_control: Option<String>,
    #[doc="Specifies presentational information for the object."]
    pub content_disposition: Option<String>,
    #[doc="Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field."]
    pub content_encoding: Option<String>,
    #[doc="The language the content is in."]
    pub content_language: Option<String>,
    #[doc="Size of the body in bytes."]
    pub content_length: Option<i64>,
    #[doc="The portion of the object returned in the response."]
    pub content_range: Option<String>,
    #[doc="A standard MIME type describing the format of the object data."]
    pub content_type: Option<String>,
    #[doc="Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response."]
    pub delete_marker: Option<bool>,
    #[doc="An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL"]
    pub e_tag: Option<String>,
    #[doc="If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded."]
    pub expiration: Option<String>,
    #[doc="The date and time at which the object is no longer cacheable."]
    pub expires: Option<String>,
    #[doc="Last modified date of the object"]
    pub last_modified: Option<String>,
    #[doc="A map of metadata to store with the object in S3."]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[doc="This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers."]
    pub missing_meta: Option<i64>,
    #[doc="The count of parts this object has."]
    pub parts_count: Option<i64>,
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    #[doc="Provides information about object restoration operation and expiration time of the restored object copy."]
    pub restore: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    pub storage_class: Option<String>,
    #[doc="The number of tags, if any, on the object."]
    pub tag_count: Option<i64>,
    #[doc="Version of the object."]
    pub version_id: Option<String>,
    #[doc="If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata."]
    pub website_redirect_location: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetObjectRequest {
    pub bucket: String,
    #[doc="Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed)."]
    pub if_match: Option<String>,
    #[doc="Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified)."]
    pub if_modified_since: Option<String>,
    #[doc="Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified)."]
    pub if_none_match: Option<String>,
    #[doc="Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed)."]
    pub if_unmodified_since: Option<String>,
    pub key: String,
    #[doc="Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object."]
    pub part_number: Option<i64>,
    #[doc="Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35."]
    pub range: Option<String>,
    pub request_payer: Option<String>,
    #[doc="Sets the Cache-Control header of the response."]
    pub response_cache_control: Option<String>,
    #[doc="Sets the Content-Disposition header of the response"]
    pub response_content_disposition: Option<String>,
    #[doc="Sets the Content-Encoding header of the response."]
    pub response_content_encoding: Option<String>,
    #[doc="Sets the Content-Language header of the response."]
    pub response_content_language: Option<String>,
    #[doc="Sets the Content-Type header of the response."]
    pub response_content_type: Option<String>,
    #[doc="Sets the Expires header of the response."]
    pub response_expires: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="VersionId used to reference a specific version of the object."]
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetObjectTaggingOutput {
    pub tag_set: Vec<Tag>,
    pub version_id: Option<String>,
}

struct GetObjectTaggingOutputDeserializer;
impl GetObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetObjectTaggingOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TagSet" => {
                            obj.tag_set = try!(TagSetDeserializer::deserialize("TagSet", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct GetObjectTaggingRequest {
    pub bucket: String,
    pub key: String,
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetObjectTorrentOutput {
    pub body: Option<StreamingBody>,
    pub request_charged: Option<String>,
}

#[derive(Default,Debug)]
pub struct GetObjectTorrentRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
}

#[derive(Default,Debug)]
pub struct GlacierJobParameters {
    #[doc="Glacier retrieval tier at which the restore will be processed."]
    pub tier: String,
}


pub struct GlacierJobParametersSerializer;
impl GlacierJobParametersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &GlacierJobParameters)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Tier"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.tier)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct Grant {
    pub grantee: Option<Grantee>,
    #[doc="Specifies the permission given to the grantee."]
    pub permission: Option<String>,
}

struct GrantDeserializer;
impl GrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Grant, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Grant::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Grantee" => {
                            obj.grantee = Some(try!(GranteeDeserializer::deserialize("Grantee",
                                                                                     stack)));
                        }
                        "Permission" => {
                            obj.permission = Some(try!(PermissionDeserializer::deserialize("Permission",
                                                                                           stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct GrantSerializer;
impl GrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Grant)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer
                .write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct Grantee {
    #[doc="Screen name of the grantee."]
    pub display_name: Option<String>,
    #[doc="Email address of the grantee."]
    pub email_address: Option<String>,
    #[doc="The canonical user ID of the grantee."]
    pub id: Option<String>,
    #[doc="Type of grantee"]
    pub type_: String,
    #[doc="URI of the grantee group."]
    pub uri: Option<String>,
}

struct GranteeDeserializer;
impl GranteeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Grantee, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Grantee::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DisplayName" => {
                            obj.display_name =
                                Some(try!(DisplayNameDeserializer::deserialize("DisplayName",
                                                                               stack)));
                        }
                        "EmailAddress" => {
                            obj.email_address =
                                Some(try!(EmailAddressDeserializer::deserialize("EmailAddress",
                                                                                stack)));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        "xsi:type" => {
                            obj.type_ = try!(TypeDeserializer::deserialize("xsi:type", stack));
                        }
                        "URI" => {
                            obj.uri = Some(try!(URIDeserializer::deserialize("URI", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct GranteeSerializer;
impl GranteeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Grantee)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer
                .write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.email_address {
            writer
                .write(xml::writer::XmlEvent::start_element("EmailAddress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("xsi:type"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.type_)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.uri {
            writer.write(xml::writer::XmlEvent::start_element("URI"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct GrantsDeserializer;
impl GrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Grant>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Grant" {
                        obj.push(try!(GrantDeserializer::deserialize("Grant", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}

pub struct GrantsSerializer;
impl GrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<Grant>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            GrantSerializer::serialize(writer, "Grant", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct HeadBucketRequest {
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct HeadObjectOutput {
    pub accept_ranges: Option<String>,
    #[doc="Specifies caching behavior along the request/reply chain."]
    pub cache_control: Option<String>,
    #[doc="Specifies presentational information for the object."]
    pub content_disposition: Option<String>,
    #[doc="Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field."]
    pub content_encoding: Option<String>,
    #[doc="The language the content is in."]
    pub content_language: Option<String>,
    #[doc="Size of the body in bytes."]
    pub content_length: Option<i64>,
    #[doc="A standard MIME type describing the format of the object data."]
    pub content_type: Option<String>,
    #[doc="Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response."]
    pub delete_marker: Option<bool>,
    #[doc="An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL"]
    pub e_tag: Option<String>,
    #[doc="If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded."]
    pub expiration: Option<String>,
    #[doc="The date and time at which the object is no longer cacheable."]
    pub expires: Option<String>,
    #[doc="Last modified date of the object"]
    pub last_modified: Option<String>,
    #[doc="A map of metadata to store with the object in S3."]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[doc="This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers."]
    pub missing_meta: Option<i64>,
    #[doc="The count of parts this object has."]
    pub parts_count: Option<i64>,
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    #[doc="Provides information about object restoration operation and expiration time of the restored object copy."]
    pub restore: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    pub storage_class: Option<String>,
    #[doc="Version of the object."]
    pub version_id: Option<String>,
    #[doc="If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata."]
    pub website_redirect_location: Option<String>,
}

struct HeadObjectOutputDeserializer;
impl HeadObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<HeadObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = HeadObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct HeadObjectRequest {
    pub bucket: String,
    #[doc="Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed)."]
    pub if_match: Option<String>,
    #[doc="Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified)."]
    pub if_modified_since: Option<String>,
    #[doc="Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified)."]
    pub if_none_match: Option<String>,
    #[doc="Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed)."]
    pub if_unmodified_since: Option<String>,
    pub key: String,
    #[doc="Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' HEAD request for the part specified. Useful querying about the size of the part and the number of parts in this object."]
    pub part_number: Option<i64>,
    #[doc="Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35."]
    pub range: Option<String>,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="VersionId used to reference a specific version of the object."]
    pub version_id: Option<String>,
}

struct HostNameDeserializer;
impl HostNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct HostNameSerializer;
impl HostNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct HttpErrorCodeReturnedEqualsDeserializer;
impl HttpErrorCodeReturnedEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct HttpErrorCodeReturnedEqualsSerializer;
impl HttpErrorCodeReturnedEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct HttpRedirectCodeDeserializer;
impl HttpRedirectCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct HttpRedirectCodeSerializer;
impl HttpRedirectCodeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct IDDeserializer;
impl IDDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct IDSerializer;
impl IDSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct IndexDocument {
    #[doc="A suffix that is appended to a request that is for a directory on the website endpoint (e.g. if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character."]
    pub suffix: String,
}

struct IndexDocumentDeserializer;
impl IndexDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<IndexDocument, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IndexDocument::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Suffix" => {
                            obj.suffix = try!(SuffixDeserializer::deserialize("Suffix", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct IndexDocumentSerializer;
impl IndexDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &IndexDocument)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("Suffix"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.suffix)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InitiatedDeserializer;
impl InitiatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct Initiator {
    #[doc="Name of the Principal."]
    pub display_name: Option<String>,
    #[doc="If the principal is an AWS account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value."]
    pub id: Option<String>,
}

struct InitiatorDeserializer;
impl InitiatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Initiator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Initiator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DisplayName" => {
                            obj.display_name =
                                Some(try!(DisplayNameDeserializer::deserialize("DisplayName",
                                                                               stack)));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct InventoryConfiguration {
    #[doc="Contains information about where to publish the inventory results."]
    pub destination: InventoryDestination,
    #[doc="Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria."]
    pub filter: Option<InventoryFilter>,
    #[doc="The ID used to identify the inventory configuration."]
    pub id: String,
    #[doc="Specifies which object version(s) to included in the inventory results."]
    pub included_object_versions: String,
    #[doc="Specifies whether the inventory is enabled or disabled."]
    pub is_enabled: bool,
    #[doc="Contains the optional fields that are included in the inventory results."]
    pub optional_fields: Option<Vec<String>>,
    #[doc="Specifies the schedule for generating inventory results."]
    pub schedule: InventorySchedule,
}

struct InventoryConfigurationDeserializer;
impl InventoryConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<InventoryConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Destination" => {
                            obj.destination =
                                try!(InventoryDestinationDeserializer::deserialize("Destination",
                                                                                   stack));
                        }
                        "Filter" => {
                            obj.filter =
                                Some(try!(InventoryFilterDeserializer::deserialize("Filter",
                                                                                   stack)));
                        }
                        "Id" => {
                            obj.id = try!(InventoryIdDeserializer::deserialize("Id", stack));
                        }
                        "IncludedObjectVersions" => {
                            obj.included_object_versions = try!(InventoryIncludedObjectVersionsDeserializer::deserialize("IncludedObjectVersions", stack));
                        }
                        "IsEnabled" => {
                            obj.is_enabled = try!(IsEnabledDeserializer::deserialize("IsEnabled",
                                                                                     stack));
                        }
                        "OptionalFields" => {
                            obj.optional_fields =
                                Some(try!(InventoryOptionalFieldsDeserializer::deserialize("OptionalFields",
                                                                                           stack)));
                        }
                        "Schedule" => {
                            obj.schedule = try!(InventoryScheduleDeserializer::deserialize("Schedule",
                                                                                           stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryConfigurationSerializer;
impl InventoryConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &InventoryConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryDestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.filter {
            &InventoryFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.id)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer
            .write(xml::writer::XmlEvent::start_element("IncludedObjectVersions"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}",
                                                             value =
                                                                 obj.included_object_versions)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer
            .write(xml::writer::XmlEvent::start_element("IsEnabled"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.is_enabled)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.optional_fields {
            &InventoryOptionalFieldsSerializer::serialize(&mut writer, "OptionalFields", value)?;
        }
        InventoryScheduleSerializer::serialize(&mut writer, "Schedule", &obj.schedule)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryConfigurationListDeserializer;
impl InventoryConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<InventoryConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(InventoryConfigurationDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct InventoryDestination {
    #[doc="Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published."]
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

struct InventoryDestinationDeserializer;
impl InventoryDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<InventoryDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "S3BucketDestination" => {
                            obj.s3_bucket_destination =
                                try!(InventoryS3BucketDestinationDeserializer::deserialize("S3BucketDestination",
                                                                                           stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryDestinationSerializer;
impl InventoryDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &InventoryDestination)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryS3BucketDestinationSerializer::serialize(&mut writer,
                                                          "S3BucketDestination",
                                                          &obj.s3_bucket_destination)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct InventoryFilter {
    #[doc="The prefix that an object must have to be included in the inventory results."]
    pub prefix: String,
}

struct InventoryFilterDeserializer;
impl InventoryFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<InventoryFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Prefix" => {
                            obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryFilterSerializer;
impl InventoryFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &InventoryFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.prefix)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryFormatDeserializer;
impl InventoryFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryFormatSerializer;
impl InventoryFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct InventoryFrequencyDeserializer;
impl InventoryFrequencyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryFrequencySerializer;
impl InventoryFrequencySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct InventoryIdDeserializer;
impl InventoryIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryIdSerializer;
impl InventoryIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct InventoryIncludedObjectVersionsDeserializer;
impl InventoryIncludedObjectVersionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryIncludedObjectVersionsSerializer;
impl InventoryIncludedObjectVersionsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct InventoryOptionalFieldDeserializer;
impl InventoryOptionalFieldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryOptionalFieldSerializer;
impl InventoryOptionalFieldSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct InventoryOptionalFieldsDeserializer;
impl InventoryOptionalFieldsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<String>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Field" {
                        obj.push(try!(InventoryOptionalFieldDeserializer::deserialize("Field",
                                                                                      stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}

pub struct InventoryOptionalFieldsSerializer;
impl InventoryOptionalFieldsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<String>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            InventoryOptionalFieldSerializer::serialize(writer, "Field", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct InventoryS3BucketDestination {
    #[doc="The ID of the account that owns the destination bucket."]
    pub account_id: Option<String>,
    #[doc="The Amazon resource name (ARN) of the bucket where inventory results will be published."]
    pub bucket: String,
    #[doc="Specifies the output format of the inventory results."]
    pub format: String,
    #[doc="The prefix that is prepended to all inventory results."]
    pub prefix: Option<String>,
}

struct InventoryS3BucketDestinationDeserializer;
impl InventoryS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<InventoryS3BucketDestination, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventoryS3BucketDestination::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AccountId" => {
                            obj.account_id = Some(try!(AccountIdDeserializer::deserialize("AccountId",
                                                                                          stack)));
                        }
                        "Bucket" => {
                            obj.bucket = try!(BucketNameDeserializer::deserialize("Bucket", stack));
                        }
                        "Format" => {
                            obj.format = try!(InventoryFormatDeserializer::deserialize("Format",
                                                                                       stack));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryS3BucketDestinationSerializer;
impl InventoryS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &InventoryS3BucketDestination)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.account_id {
            writer
                .write(xml::writer::XmlEvent::start_element("AccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.bucket)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer
            .write(xml::writer::XmlEvent::start_element("Format"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.format)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct InventorySchedule {
    #[doc="Specifies how frequently inventory results are produced."]
    pub frequency: String,
}

struct InventoryScheduleDeserializer;
impl InventoryScheduleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<InventorySchedule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InventorySchedule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Frequency" => {
                            obj.frequency =
                                try!(InventoryFrequencyDeserializer::deserialize("Frequency",
                                                                                 stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct InventoryScheduleSerializer;
impl InventoryScheduleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &InventorySchedule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("Frequency"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.frequency)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IsEnabledDeserializer;
impl IsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct IsEnabledSerializer;
impl IsEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &bool)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct IsLatestDeserializer;
impl IsLatestDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct IsTruncatedDeserializer;
impl IsTruncatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct KeyCountDeserializer;
impl KeyCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct KeyMarkerDeserializer;
impl KeyMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct KeyMarkerSerializer;
impl KeyMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct KeyPrefixEqualsDeserializer;
impl KeyPrefixEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct KeyPrefixEqualsSerializer;
impl KeyPrefixEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct LambdaFunctionArnDeserializer;
impl LambdaFunctionArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LambdaFunctionArnSerializer;
impl LambdaFunctionArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[doc="Container for specifying the AWS Lambda notification configuration."]
#[derive(Default,Debug)]
pub struct LambdaFunctionConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    #[doc="Lambda cloud function ARN that Amazon S3 can invoke when it detects events of the specified type."]
    pub lambda_function_arn: String,
}

struct LambdaFunctionConfigurationDeserializer;
impl LambdaFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LambdaFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LambdaFunctionConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Event" => {
                            obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                        }
                        "Filter" => {
                            obj.filter = Some(try!(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "CloudFunction" => {
                            obj.lambda_function_arn =
                                try!(LambdaFunctionArnDeserializer::deserialize("CloudFunction",
                                                                                stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LambdaFunctionConfigurationSerializer;
impl LambdaFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LambdaFunctionConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}",
                                                             value = obj.lambda_function_arn)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionConfigurationListDeserializer;
impl LambdaFunctionConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<Vec<LambdaFunctionConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(LambdaFunctionConfigurationDeserializer::deserialize(tag_name,
                                                                                   stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct LambdaFunctionConfigurationListSerializer;
impl LambdaFunctionConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<LambdaFunctionConfiguration>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            LambdaFunctionConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct LastModifiedDeserializer;
impl LastModifiedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct LifecycleConfiguration {
    pub rules: Vec<Rule>,
}


pub struct LifecycleConfigurationSerializer;
impl LifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LifecycleConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        RulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct LifecycleExpiration {
    #[doc="Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format."]
    pub date: Option<String>,
    #[doc="Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer."]
    pub days: Option<i64>,
    #[doc="Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If set to true, the delete marker will be expired; if set to false the policy takes no action. This cannot be specified with Days or Date in a Lifecycle Expiration Policy."]
    pub expired_object_delete_marker: Option<bool>,
}

struct LifecycleExpirationDeserializer;
impl LifecycleExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LifecycleExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleExpiration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Date" => {
                            obj.date = Some(try!(DateDeserializer::deserialize("Date", stack)));
                        }
                        "Days" => {
                            obj.days = Some(try!(DaysDeserializer::deserialize("Days", stack)));
                        }
                        "ExpiredObjectDeleteMarker" => {
                            obj.expired_object_delete_marker = Some(try!(ExpiredObjectDeleteMarkerDeserializer::deserialize("ExpiredObjectDeleteMarker", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LifecycleExpirationSerializer;
impl LifecycleExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LifecycleExpiration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.expired_object_delete_marker {
            writer
                .write(xml::writer::XmlEvent::start_element("ExpiredObjectDeleteMarker"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    pub filter: Option<LifecycleRuleFilter>,
    #[doc="Unique identifier for the rule. The value cannot be longer than 255 characters."]
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,
    #[doc="If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied."]
    pub status: String,
    pub transitions: Option<Vec<Transition>>,
}

struct LifecycleRuleDeserializer;
impl LifecycleRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LifecycleRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AbortIncompleteMultipartUpload" => {
                            obj.abort_incomplete_multipart_upload = Some(try!(AbortIncompleteMultipartUploadDeserializer::deserialize("AbortIncompleteMultipartUpload", stack)));
                        }
                        "Expiration" => {
                            obj.expiration =
                                Some(try!(LifecycleExpirationDeserializer::deserialize("Expiration",
                                                                                       stack)));
                        }
                        "Filter" => {
                            obj.filter =
                                Some(try!(LifecycleRuleFilterDeserializer::deserialize("Filter",
                                                                                       stack)));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        "NoncurrentVersionExpiration" => {
                            obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationDeserializer::deserialize("NoncurrentVersionExpiration", stack)));
                        }
                        "NoncurrentVersionTransition" => {
                            obj.noncurrent_version_transitions = Some(try!(NoncurrentVersionTransitionListDeserializer::deserialize("NoncurrentVersionTransition", stack)));
                        }
                        "Status" => {
                            obj.status = try!(ExpirationStatusDeserializer::deserialize("Status",
                                                                                        stack));
                        }
                        "Transition" => {
                            obj.transitions =
                                Some(try!(TransitionListDeserializer::deserialize("Transition",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LifecycleRuleSerializer;
impl LifecycleRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LifecycleRule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(&mut writer,
                                                                 "AbortIncompleteMultipartUpload",
                                                                 value)?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.filter {
            &LifecycleRuleFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(&mut writer,
                                                              "NoncurrentVersionExpiration",
                                                              value)?;
        }
        if let Some(ref value) = obj.noncurrent_version_transitions {
            &NoncurrentVersionTransitionListSerializer::serialize(&mut writer,
                                                                  "NoncurrentVersionTransition",
                                                                  value)?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("Status"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.status)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.transitions {
            &TransitionListSerializer::serialize(&mut writer, "Transition", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[doc="This is used in a Lifecycle Rule Filter to apply a logical AND to two or more predicates. The Lifecycle Rule will apply to any object matching all of the predicates configured inside the And operator."]
#[derive(Default,Debug)]
pub struct LifecycleRuleAndOperator {
    pub prefix: Option<String>,
    #[doc="All of these tags must exist in the object's tag set in order for the rule to apply."]
    pub tags: Option<Vec<Tag>>,
}

struct LifecycleRuleAndOperatorDeserializer;
impl LifecycleRuleAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LifecycleRuleAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRuleAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LifecycleRuleAndOperatorSerializer;
impl LifecycleRuleAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LifecycleRuleAndOperator)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[doc="The Filter is used to identify objects that a Lifecycle Rule applies to. A Filter must have exactly one of Prefix, Tag, or And specified."]
#[derive(Default,Debug)]
pub struct LifecycleRuleFilter {
    pub and: Option<LifecycleRuleAndOperator>,
    #[doc="Prefix identifying one or more objects to which the rule applies."]
    pub prefix: Option<String>,
    #[doc="This tag must exist in the object's tag set in order for the rule to apply."]
    pub tag: Option<Tag>,
}

struct LifecycleRuleFilterDeserializer;
impl LifecycleRuleFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LifecycleRuleFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LifecycleRuleFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "And" => {
                            obj.and = Some(try!(LifecycleRuleAndOperatorDeserializer::deserialize("And", stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LifecycleRuleFilterSerializer;
impl LifecycleRuleFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LifecycleRuleFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &LifecycleRuleAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LifecycleRulesDeserializer;
impl LifecycleRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<LifecycleRule>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(LifecycleRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct LifecycleRulesSerializer;
impl LifecycleRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<LifecycleRule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            LifecycleRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    #[doc="The list of analytics configurations for a bucket."]
    pub analytics_configuration_list: Option<Vec<AnalyticsConfiguration>>,
    #[doc="The ContinuationToken that represents where this request began."]
    pub continuation_token: Option<String>,
    #[doc="Indicates whether the returned list of analytics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request."]
    pub is_truncated: Option<bool>,
    #[doc="NextContinuationToken is sent when isTruncated is true, which indicates that there are more analytics configurations to list. The next request must include this NextContinuationToken. The token is obfuscated and is not a usable value."]
    pub next_continuation_token: Option<String>,
}

struct ListBucketAnalyticsConfigurationsOutputDeserializer;
impl ListBucketAnalyticsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<ListBucketAnalyticsConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketAnalyticsConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AnalyticsConfiguration" => {
                            obj.analytics_configuration_list = Some(try!(AnalyticsConfigurationListDeserializer::deserialize("AnalyticsConfiguration", stack)));
                        }
                        "ContinuationToken" => {
                            obj.continuation_token =
                                Some(try!(TokenDeserializer::deserialize("ContinuationToken",
                                                                         stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "NextContinuationToken" => {
                            obj.next_continuation_token =
                                Some(try!(NextTokenDeserializer::deserialize("NextContinuationToken",
                                                                             stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    #[doc="The name of the bucket from which analytics configurations are retrieved."]
    pub bucket: String,
    #[doc="The ContinuationToken that represents a placeholder from where this request should begin."]
    pub continuation_token: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListBucketInventoryConfigurationsOutput {
    #[doc="If sent in the request, the marker that is used as a starting point for this inventory configuration list response."]
    pub continuation_token: Option<String>,
    #[doc="The list of inventory configurations for a bucket."]
    pub inventory_configuration_list: Option<Vec<InventoryConfiguration>>,
    #[doc="Indicates whether the returned list of inventory configurations is truncated in this response. A value of true indicates that the list is truncated."]
    pub is_truncated: Option<bool>,
    #[doc="The marker used to continue this inventory configuration listing. Use the NextContinuationToken from this response to continue the listing in a subsequent request. The continuation token is an opaque value that Amazon S3 understands."]
    pub next_continuation_token: Option<String>,
}

struct ListBucketInventoryConfigurationsOutputDeserializer;
impl ListBucketInventoryConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<ListBucketInventoryConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketInventoryConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ContinuationToken" => {
                            obj.continuation_token =
                                Some(try!(TokenDeserializer::deserialize("ContinuationToken",
                                                                         stack)));
                        }
                        "InventoryConfiguration" => {
                            obj.inventory_configuration_list = Some(try!(InventoryConfigurationListDeserializer::deserialize("InventoryConfiguration", stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "NextContinuationToken" => {
                            obj.next_continuation_token =
                                Some(try!(NextTokenDeserializer::deserialize("NextContinuationToken",
                                                                             stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListBucketInventoryConfigurationsRequest {
    #[doc="The name of the bucket containing the inventory configurations to retrieve."]
    pub bucket: String,
    #[doc="The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands."]
    pub continuation_token: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListBucketMetricsConfigurationsOutput {
    #[doc="The marker that is used as a starting point for this metrics configuration list response. This value is present if it was sent in the request."]
    pub continuation_token: Option<String>,
    #[doc="Indicates whether the returned list of metrics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request."]
    pub is_truncated: Option<bool>,
    #[doc="The list of metrics configurations for a bucket."]
    pub metrics_configuration_list: Option<Vec<MetricsConfiguration>>,
    #[doc="The marker used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands."]
    pub next_continuation_token: Option<String>,
}

struct ListBucketMetricsConfigurationsOutputDeserializer;
impl ListBucketMetricsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<ListBucketMetricsConfigurationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketMetricsConfigurationsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ContinuationToken" => {
                            obj.continuation_token =
                                Some(try!(TokenDeserializer::deserialize("ContinuationToken",
                                                                         stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "MetricsConfiguration" => {
                            obj.metrics_configuration_list = Some(try!(MetricsConfigurationListDeserializer::deserialize("MetricsConfiguration", stack)));
                        }
                        "NextContinuationToken" => {
                            obj.next_continuation_token =
                                Some(try!(NextTokenDeserializer::deserialize("NextContinuationToken",
                                                                             stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListBucketMetricsConfigurationsRequest {
    #[doc="The name of the bucket containing the metrics configurations to retrieve."]
    pub bucket: String,
    #[doc="The marker that is used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands."]
    pub continuation_token: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListBucketsOutput {
    pub buckets: Option<Vec<Bucket>>,
    pub owner: Option<Owner>,
}

struct ListBucketsOutputDeserializer;
impl ListBucketsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListBucketsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListBucketsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Buckets" => {
                            obj.buckets = Some(try!(BucketsDeserializer::deserialize("Buckets",
                                                                                     stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListMultipartUploadsOutput {
    #[doc="Name of the bucket to which the multipart upload was initiated."]
    pub bucket: Option<String>,
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub delimiter: Option<String>,
    #[doc="Encoding type used by Amazon S3 to encode object keys in the response."]
    pub encoding_type: Option<String>,
    #[doc="Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads."]
    pub is_truncated: Option<bool>,
    #[doc="The key at or after which the listing began."]
    pub key_marker: Option<String>,
    #[doc="Maximum number of multipart uploads that could have been included in the response."]
    pub max_uploads: Option<i64>,
    #[doc="When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request."]
    pub next_key_marker: Option<String>,
    #[doc="When a list is truncated, this element specifies the value that should be used for the upload-id-marker request parameter in a subsequent request."]
    pub next_upload_id_marker: Option<String>,
    #[doc="When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="Upload ID after which listing began."]
    pub upload_id_marker: Option<String>,
    pub uploads: Option<Vec<MultipartUpload>>,
}

struct ListMultipartUploadsOutputDeserializer;
impl ListMultipartUploadsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListMultipartUploadsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListMultipartUploadsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = Some(try!(BucketNameDeserializer::deserialize("Bucket",
                                                                                       stack)));
                        }
                        "CommonPrefixes" => {
                            obj.common_prefixes =
                                Some(try!(CommonPrefixListDeserializer::deserialize("CommonPrefixes",
                                                                                    stack)));
                        }
                        "Delimiter" => {
                            obj.delimiter = Some(try!(DelimiterDeserializer::deserialize("Delimiter",
                                                                                         stack)));
                        }
                        "EncodingType" => {
                            obj.encoding_type =
                                Some(try!(EncodingTypeDeserializer::deserialize("EncodingType",
                                                                                stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "KeyMarker" => {
                            obj.key_marker = Some(try!(KeyMarkerDeserializer::deserialize("KeyMarker",
                                                                                          stack)));
                        }
                        "MaxUploads" => {
                            obj.max_uploads =
                                Some(try!(MaxUploadsDeserializer::deserialize("MaxUploads",
                                                                              stack)));
                        }
                        "NextKeyMarker" => {
                            obj.next_key_marker =
                                Some(try!(NextKeyMarkerDeserializer::deserialize("NextKeyMarker",
                                                                                 stack)));
                        }
                        "NextUploadIdMarker" => {
                            obj.next_upload_id_marker =
                                Some(try!(NextUploadIdMarkerDeserializer::deserialize("NextUploadIdMarker",
                                                                                      stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "UploadIdMarker" => {
                            obj.upload_id_marker =
                                Some(try!(UploadIdMarkerDeserializer::deserialize("UploadIdMarker",
                                                                                  stack)));
                        }
                        "Upload" => {
                            obj.uploads =
                                Some(try!(MultipartUploadListDeserializer::deserialize("Upload",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListMultipartUploadsRequest {
    pub bucket: String,
    #[doc="Character you use to group keys."]
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    #[doc="Together with upload-id-marker, this parameter specifies the multipart upload after which listing should begin."]
    pub key_marker: Option<String>,
    #[doc="Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response."]
    pub max_uploads: Option<i64>,
    #[doc="Lists in-progress uploads only for those keys that begin with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored."]
    pub upload_id_marker: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListObjectVersionsOutput {
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub delete_markers: Option<Vec<DeleteMarkerEntry>>,
    pub delimiter: Option<String>,
    #[doc="Encoding type used by Amazon S3 to encode object keys in the response."]
    pub encoding_type: Option<String>,
    #[doc="A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria. If your results were truncated, you can make a follow-up paginated request using the NextKeyMarker and NextVersionIdMarker response parameters as a starting place in another request to return the rest of the results."]
    pub is_truncated: Option<bool>,
    #[doc="Marks the last Key returned in a truncated response."]
    pub key_marker: Option<String>,
    pub max_keys: Option<i64>,
    pub name: Option<String>,
    #[doc="Use this value for the key marker request parameter in a subsequent request."]
    pub next_key_marker: Option<String>,
    #[doc="Use this value for the next version id marker parameter in a subsequent request."]
    pub next_version_id_marker: Option<String>,
    pub prefix: Option<String>,
    pub version_id_marker: Option<String>,
    pub versions: Option<Vec<ObjectVersion>>,
}

struct ListObjectVersionsOutputDeserializer;
impl ListObjectVersionsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListObjectVersionsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectVersionsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CommonPrefixes" => {
                            obj.common_prefixes =
                                Some(try!(CommonPrefixListDeserializer::deserialize("CommonPrefixes",
                                                                                    stack)));
                        }
                        "DeleteMarker" => {
                            obj.delete_markers =
                                Some(try!(DeleteMarkersDeserializer::deserialize("DeleteMarker",
                                                                                 stack)));
                        }
                        "Delimiter" => {
                            obj.delimiter = Some(try!(DelimiterDeserializer::deserialize("Delimiter",
                                                                                         stack)));
                        }
                        "EncodingType" => {
                            obj.encoding_type =
                                Some(try!(EncodingTypeDeserializer::deserialize("EncodingType",
                                                                                stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "KeyMarker" => {
                            obj.key_marker = Some(try!(KeyMarkerDeserializer::deserialize("KeyMarker",
                                                                                          stack)));
                        }
                        "MaxKeys" => {
                            obj.max_keys = Some(try!(MaxKeysDeserializer::deserialize("MaxKeys",
                                                                                      stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(BucketNameDeserializer::deserialize("Name",
                                                                                     stack)));
                        }
                        "NextKeyMarker" => {
                            obj.next_key_marker =
                                Some(try!(NextKeyMarkerDeserializer::deserialize("NextKeyMarker",
                                                                                 stack)));
                        }
                        "NextVersionIdMarker" => {
                            obj.next_version_id_marker =
                                Some(try!(NextVersionIdMarkerDeserializer::deserialize("NextVersionIdMarker",
                                                                                       stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "VersionIdMarker" => {
                            obj.version_id_marker =
                                Some(try!(VersionIdMarkerDeserializer::deserialize("VersionIdMarker",
                                                                                   stack)));
                        }
                        "Version" => {
                            obj.versions =
                                Some(try!(ObjectVersionListDeserializer::deserialize("Version",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListObjectVersionsRequest {
    pub bucket: String,
    #[doc="A delimiter is a character you use to group keys."]
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    #[doc="Specifies the key to start with when listing objects in a bucket."]
    pub key_marker: Option<String>,
    #[doc="Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more."]
    pub max_keys: Option<i64>,
    #[doc="Limits the response to keys that begin with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="Specifies the object version you want to start listing from."]
    pub version_id_marker: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListObjectsOutput {
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    pub contents: Option<Vec<Object>>,
    pub delimiter: Option<String>,
    #[doc="Encoding type used by Amazon S3 to encode object keys in the response."]
    pub encoding_type: Option<String>,
    #[doc="A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria."]
    pub is_truncated: Option<bool>,
    pub marker: Option<String>,
    pub max_keys: Option<i64>,
    pub name: Option<String>,
    #[doc="When response is truncated (the IsTruncated element value in the response is true), you can use the key name in this field as marker in the subsequent request to get next set of objects. Amazon S3 lists objects in alphabetical order Note: This element is returned only if you have delimiter request parameter specified. If response does not include the NextMaker and it is truncated, you can use the value of the last Key in the response as the marker in the subsequent request to get the next set of object keys."]
    pub next_marker: Option<String>,
    pub prefix: Option<String>,
}

struct ListObjectsOutputDeserializer;
impl ListObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CommonPrefixes" => {
                            obj.common_prefixes =
                                Some(try!(CommonPrefixListDeserializer::deserialize("CommonPrefixes",
                                                                                    stack)));
                        }
                        "Contents" => {
                            obj.contents = Some(try!(ObjectListDeserializer::deserialize("Contents",
                                                                                         stack)));
                        }
                        "Delimiter" => {
                            obj.delimiter = Some(try!(DelimiterDeserializer::deserialize("Delimiter",
                                                                                         stack)));
                        }
                        "EncodingType" => {
                            obj.encoding_type =
                                Some(try!(EncodingTypeDeserializer::deserialize("EncodingType",
                                                                                stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(MarkerDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "MaxKeys" => {
                            obj.max_keys = Some(try!(MaxKeysDeserializer::deserialize("MaxKeys",
                                                                                      stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(BucketNameDeserializer::deserialize("Name",
                                                                                     stack)));
                        }
                        "NextMarker" => {
                            obj.next_marker =
                                Some(try!(NextMarkerDeserializer::deserialize("NextMarker",
                                                                              stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListObjectsRequest {
    pub bucket: String,
    #[doc="A delimiter is a character you use to group keys."]
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    #[doc="Specifies the key to start with when listing objects in a bucket."]
    pub marker: Option<String>,
    #[doc="Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more."]
    pub max_keys: Option<i64>,
    #[doc="Limits the response to keys that begin with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests."]
    pub request_payer: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListObjectsV2Output {
    #[doc="CommonPrefixes contains all (if there are any) keys between Prefix and the next occurrence of the string specified by delimiter"]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    #[doc="Metadata about each object returned."]
    pub contents: Option<Vec<Object>>,
    #[doc="ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key"]
    pub continuation_token: Option<String>,
    #[doc="A delimiter is a character you use to group keys."]
    pub delimiter: Option<String>,
    #[doc="Encoding type used by Amazon S3 to encode object keys in the response."]
    pub encoding_type: Option<String>,
    #[doc="A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria."]
    pub is_truncated: Option<bool>,
    #[doc="KeyCount is the number of keys returned with this request. KeyCount will always be less than equals to MaxKeys field. Say you ask for 50 keys, your result will include less than equals 50 keys"]
    pub key_count: Option<i64>,
    #[doc="Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more."]
    pub max_keys: Option<i64>,
    #[doc="Name of the bucket to list."]
    pub name: Option<String>,
    #[doc="NextContinuationToken is sent when isTruncated is true which means there are more keys in the bucket that can be listed. The next list requests to Amazon S3 can be continued with this NextContinuationToken. NextContinuationToken is obfuscated and is not a real key"]
    pub next_continuation_token: Option<String>,
    #[doc="Limits the response to keys that begin with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket"]
    pub start_after: Option<String>,
}

struct ListObjectsV2OutputDeserializer;
impl ListObjectsV2OutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListObjectsV2Output, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListObjectsV2Output::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CommonPrefixes" => {
                            obj.common_prefixes =
                                Some(try!(CommonPrefixListDeserializer::deserialize("CommonPrefixes",
                                                                                    stack)));
                        }
                        "Contents" => {
                            obj.contents = Some(try!(ObjectListDeserializer::deserialize("Contents",
                                                                                         stack)));
                        }
                        "ContinuationToken" => {
                            obj.continuation_token =
                                Some(try!(TokenDeserializer::deserialize("ContinuationToken",
                                                                         stack)));
                        }
                        "Delimiter" => {
                            obj.delimiter = Some(try!(DelimiterDeserializer::deserialize("Delimiter",
                                                                                         stack)));
                        }
                        "EncodingType" => {
                            obj.encoding_type =
                                Some(try!(EncodingTypeDeserializer::deserialize("EncodingType",
                                                                                stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "KeyCount" => {
                            obj.key_count = Some(try!(KeyCountDeserializer::deserialize("KeyCount",
                                                                                        stack)));
                        }
                        "MaxKeys" => {
                            obj.max_keys = Some(try!(MaxKeysDeserializer::deserialize("MaxKeys",
                                                                                      stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(BucketNameDeserializer::deserialize("Name",
                                                                                     stack)));
                        }
                        "NextContinuationToken" => {
                            obj.next_continuation_token =
                                Some(try!(NextTokenDeserializer::deserialize("NextContinuationToken",
                                                                             stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "StartAfter" => {
                            obj.start_after =
                                Some(try!(StartAfterDeserializer::deserialize("StartAfter",
                                                                              stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListObjectsV2Request {
    #[doc="Name of the bucket to list."]
    pub bucket: String,
    #[doc="ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key"]
    pub continuation_token: Option<String>,
    #[doc="A delimiter is a character you use to group keys."]
    pub delimiter: Option<String>,
    #[doc="Encoding type used by Amazon S3 to encode object keys in the response."]
    pub encoding_type: Option<String>,
    #[doc="The owner field is not present in listV2 by default, if you want to return owner field with each key in the result then set the fetch owner field to true"]
    pub fetch_owner: Option<bool>,
    #[doc="Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more."]
    pub max_keys: Option<i64>,
    #[doc="Limits the response to keys that begin with the specified prefix."]
    pub prefix: Option<String>,
    #[doc="Confirms that the requester knows that she or he will be charged for the list objects request in V2 style. Bucket owners need not specify this parameter in their requests."]
    pub request_payer: Option<String>,
    #[doc="StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket"]
    pub start_after: Option<String>,
}

#[derive(Default,Debug)]
pub struct ListPartsOutput {
    #[doc="Date when multipart upload will become eligible for abort operation by lifecycle."]
    pub abort_date: Option<String>,
    #[doc="Id of the lifecycle rule that makes a multipart upload eligible for abort operation."]
    pub abort_rule_id: Option<String>,
    #[doc="Name of the bucket to which the multipart upload was initiated."]
    pub bucket: Option<String>,
    #[doc="Identifies who initiated the multipart upload."]
    pub initiator: Option<Initiator>,
    #[doc="Indicates whether the returned list of parts is truncated."]
    pub is_truncated: Option<bool>,
    #[doc="Object key for which the multipart upload was initiated."]
    pub key: Option<String>,
    #[doc="Maximum number of parts that were allowed in the response."]
    pub max_parts: Option<i64>,
    #[doc="When a list is truncated, this element specifies the last part in the list, as well as the value to use for the part-number-marker request parameter in a subsequent request."]
    pub next_part_number_marker: Option<i64>,
    pub owner: Option<Owner>,
    #[doc="Part number after which listing begins."]
    pub part_number_marker: Option<i64>,
    pub parts: Option<Vec<Part>>,
    pub request_charged: Option<String>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
    #[doc="Upload ID identifying the multipart upload whose parts are being listed."]
    pub upload_id: Option<String>,
}

struct ListPartsOutputDeserializer;
impl ListPartsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListPartsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListPartsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Bucket" => {
                            obj.bucket = Some(try!(BucketNameDeserializer::deserialize("Bucket",
                                                                                       stack)));
                        }
                        "Initiator" => {
                            obj.initiator = Some(try!(InitiatorDeserializer::deserialize("Initiator",
                                                                                         stack)));
                        }
                        "IsTruncated" => {
                            obj.is_truncated =
                                Some(try!(IsTruncatedDeserializer::deserialize("IsTruncated",
                                                                               stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "MaxParts" => {
                            obj.max_parts = Some(try!(MaxPartsDeserializer::deserialize("MaxParts",
                                                                                        stack)));
                        }
                        "NextPartNumberMarker" => {
                            obj.next_part_number_marker =
                                Some(try!(NextPartNumberMarkerDeserializer::deserialize("NextPartNumberMarker",
                                                                                        stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        "PartNumberMarker" => {
                            obj.part_number_marker =
                                Some(try!(PartNumberMarkerDeserializer::deserialize("PartNumberMarker",
                                                                                    stack)));
                        }
                        "Part" => {
                            obj.parts = Some(try!(PartsDeserializer::deserialize("Part", stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(StorageClassDeserializer::deserialize("StorageClass",
                                                                                stack)));
                        }
                        "UploadId" => {
                            obj.upload_id =
                                Some(try!(MultipartUploadIdDeserializer::deserialize("UploadId",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ListPartsRequest {
    pub bucket: String,
    pub key: String,
    #[doc="Sets the maximum number of parts to return."]
    pub max_parts: Option<i64>,
    #[doc="Specifies the part after which listing should begin. Only parts with higher part numbers will be listed."]
    pub part_number_marker: Option<i64>,
    pub request_payer: Option<String>,
    #[doc="Upload ID identifying the multipart upload whose parts are being listed."]
    pub upload_id: String,
}

struct LocationDeserializer;
impl LocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct LoggingEnabled {
    #[doc="Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case you should choose a different TargetPrefix for each source bucket so that the delivered log files can be distinguished by key."]
    pub target_bucket: Option<String>,
    pub target_grants: Option<Vec<TargetGrant>>,
    #[doc="This element lets you specify a prefix for the keys that the log files will be stored under."]
    pub target_prefix: Option<String>,
}

struct LoggingEnabledDeserializer;
impl LoggingEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<LoggingEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoggingEnabled::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TargetBucket" => {
                            obj.target_bucket =
                                Some(try!(TargetBucketDeserializer::deserialize("TargetBucket",
                                                                                stack)));
                        }
                        "TargetGrants" => {
                            obj.target_grants =
                                Some(try!(TargetGrantsDeserializer::deserialize("TargetGrants",
                                                                                stack)));
                        }
                        "TargetPrefix" => {
                            obj.target_prefix =
                                Some(try!(TargetPrefixDeserializer::deserialize("TargetPrefix",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct LoggingEnabledSerializer;
impl LoggingEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &LoggingEnabled)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.target_bucket {
            writer
                .write(xml::writer::XmlEvent::start_element("TargetBucket"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.target_grants {
            &TargetGrantsSerializer::serialize(&mut writer, "TargetGrants", value)?;
        }
        if let Some(ref value) = obj.target_prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("TargetPrefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}


pub struct MFADeleteSerializer;
impl MFADeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MFADeleteStatusDeserializer;
impl MFADeleteStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct MarkerDeserializer;
impl MarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MarkerSerializer;
impl MarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MaxAgeSecondsDeserializer;
impl MaxAgeSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MaxAgeSecondsSerializer;
impl MaxAgeSecondsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MaxKeysDeserializer;
impl MaxKeysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MaxKeysSerializer;
impl MaxKeysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MaxPartsDeserializer;
impl MaxPartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MaxPartsSerializer;
impl MaxPartsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MaxUploadsDeserializer;
impl MaxUploadsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MaxUploadsSerializer;
impl MaxUploadsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct MetricsAndOperator {
    #[doc="The prefix used when evaluating an AND predicate."]
    pub prefix: Option<String>,
    #[doc="The list of tags used when evaluating an AND predicate."]
    pub tags: Option<Vec<Tag>>,
}

struct MetricsAndOperatorDeserializer;
impl MetricsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricsAndOperator, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsAndOperator::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tags = Some(try!(TagSetDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MetricsAndOperatorSerializer;
impl MetricsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &MetricsAndOperator)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct MetricsConfiguration {
    #[doc="Specifies a metrics configuration filter. The metrics configuration will only include objects that meet the filter's criteria. A filter must be a prefix, a tag, or a conjunction (MetricsAndOperator)."]
    pub filter: Option<MetricsFilter>,
    #[doc="The ID used to identify the metrics configuration."]
    pub id: String,
}

struct MetricsConfigurationDeserializer;
impl MetricsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricsConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Filter" => {
                            obj.filter = Some(try!(MetricsFilterDeserializer::deserialize("Filter",
                                                                                          stack)));
                        }
                        "Id" => {
                            obj.id = try!(MetricsIdDeserializer::deserialize("Id", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MetricsConfigurationSerializer;
impl MetricsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &MetricsConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &MetricsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.id)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MetricsConfigurationListDeserializer;
impl MetricsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<MetricsConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(MetricsConfigurationDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct MetricsFilter {
    #[doc="A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply."]
    pub and: Option<MetricsAndOperator>,
    #[doc="The prefix used when evaluating a metrics filter."]
    pub prefix: Option<String>,
    #[doc="The tag used when evaluating a metrics filter."]
    pub tag: Option<Tag>,
}

struct MetricsFilterDeserializer;
impl MetricsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MetricsFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MetricsFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "And" => {
                            obj.and =
                                Some(try!(MetricsAndOperatorDeserializer::deserialize("And",
                                                                                      stack)));
                        }
                        "Prefix" => {
                            obj.prefix = Some(try!(PrefixDeserializer::deserialize("Prefix",
                                                                                   stack)));
                        }
                        "Tag" => {
                            obj.tag = Some(try!(TagDeserializer::deserialize("Tag", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MetricsFilterSerializer;
impl MetricsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &MetricsFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &MetricsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer
                .write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MetricsIdDeserializer;
impl MetricsIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MetricsIdSerializer;
impl MetricsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct MultipartUpload {
    #[doc="Date and time at which the multipart upload was initiated."]
    pub initiated: Option<String>,
    #[doc="Identifies who initiated the multipart upload."]
    pub initiator: Option<Initiator>,
    #[doc="Key of the object for which the multipart upload was initiated."]
    pub key: Option<String>,
    pub owner: Option<Owner>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
    #[doc="Upload ID that identifies the multipart upload."]
    pub upload_id: Option<String>,
}

struct MultipartUploadDeserializer;
impl MultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<MultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MultipartUpload::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Initiated" => {
                            obj.initiated = Some(try!(InitiatedDeserializer::deserialize("Initiated",
                                                                                         stack)));
                        }
                        "Initiator" => {
                            obj.initiator = Some(try!(InitiatorDeserializer::deserialize("Initiator",
                                                                                         stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(StorageClassDeserializer::deserialize("StorageClass",
                                                                                stack)));
                        }
                        "UploadId" => {
                            obj.upload_id =
                                Some(try!(MultipartUploadIdDeserializer::deserialize("UploadId",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct MultipartUploadIdDeserializer;
impl MultipartUploadIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct MultipartUploadIdSerializer;
impl MultipartUploadIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct MultipartUploadListDeserializer;
impl MultipartUploadListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<MultipartUpload>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(MultipartUploadDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct NextKeyMarkerDeserializer;
impl NextKeyMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct NextMarkerDeserializer;
impl NextMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct NextPartNumberMarkerDeserializer;
impl NextPartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct NextUploadIdMarkerDeserializer;
impl NextUploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct NextVersionIdMarkerDeserializer;
impl NextVersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime."]
#[derive(Default,Debug)]
pub struct NoncurrentVersionExpiration {
    #[doc="Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html\">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide."]
    pub noncurrent_days: Option<i64>,
}

struct NoncurrentVersionExpirationDeserializer;
impl NoncurrentVersionExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NoncurrentVersionExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NoncurrentVersionExpiration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "NoncurrentDays" => {
                            obj.noncurrent_days = Some(try!(DaysDeserializer::deserialize("NoncurrentDays",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NoncurrentVersionExpirationSerializer;
impl NoncurrentVersionExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &NoncurrentVersionExpiration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer
                .write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[doc="Container for the transition rule that describes when noncurrent objects transition to the STANDARD_IA or GLACIER storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to the STANDARD_IA or GLACIER storage class at a specific period in the object's lifetime."]
#[derive(Default,Debug)]
pub struct NoncurrentVersionTransition {
    #[doc="Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html\">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide."]
    pub noncurrent_days: Option<i64>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
}

struct NoncurrentVersionTransitionDeserializer;
impl NoncurrentVersionTransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NoncurrentVersionTransition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NoncurrentVersionTransition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "NoncurrentDays" => {
                            obj.noncurrent_days = Some(try!(DaysDeserializer::deserialize("NoncurrentDays",
                                                                                          stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(TransitionStorageClassDeserializer::deserialize("StorageClass",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NoncurrentVersionTransitionSerializer;
impl NoncurrentVersionTransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &NoncurrentVersionTransition)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer
                .write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer
                .write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct NoncurrentVersionTransitionListDeserializer;
impl NoncurrentVersionTransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<Vec<NoncurrentVersionTransition>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(NoncurrentVersionTransitionDeserializer::deserialize(tag_name,
                                                                                   stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct NoncurrentVersionTransitionListSerializer;
impl NoncurrentVersionTransitionListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<NoncurrentVersionTransition>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            NoncurrentVersionTransitionSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[doc="Container for specifying the notification configuration of the bucket. If this element is empty, notifications are turned off on the bucket."]
#[derive(Default,Debug)]
pub struct NotificationConfiguration {
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CloudFunctionConfiguration" => {
                            obj.lambda_function_configurations = Some(try!(LambdaFunctionConfigurationListDeserializer::deserialize("CloudFunctionConfiguration", stack)));
                        }
                        "QueueConfiguration" => {
                            obj.queue_configurations =
                                Some(try!(QueueConfigurationListDeserializer::deserialize("QueueConfiguration",
                                                                                          stack)));
                        }
                        "TopicConfiguration" => {
                            obj.topic_configurations =
                                Some(try!(TopicConfigurationListDeserializer::deserialize("TopicConfiguration",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NotificationConfigurationSerializer;
impl NotificationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &NotificationConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.lambda_function_configurations {
            &LambdaFunctionConfigurationListSerializer::serialize(&mut writer,
                                                                  "CloudFunctionConfiguration",
                                                                  value)?;
        }
        if let Some(ref value) = obj.queue_configurations {
            &QueueConfigurationListSerializer::serialize(&mut writer, "QueueConfiguration", value)?;
        }
        if let Some(ref value) = obj.topic_configurations {
            &TopicConfigurationListSerializer::serialize(&mut writer, "TopicConfiguration", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct NotificationConfigurationDeprecated {
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
}

struct NotificationConfigurationDeprecatedDeserializer;
impl NotificationConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CloudFunctionConfiguration" => {
                            obj.cloud_function_configuration = Some(try!(CloudFunctionConfigurationDeserializer::deserialize("CloudFunctionConfiguration", stack)));
                        }
                        "QueueConfiguration" => {
                            obj.queue_configuration = Some(try!(QueueConfigurationDeprecatedDeserializer::deserialize("QueueConfiguration", stack)));
                        }
                        "TopicConfiguration" => {
                            obj.topic_configuration = Some(try!(TopicConfigurationDeprecatedDeserializer::deserialize("TopicConfiguration", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NotificationConfigurationDeprecatedSerializer;
impl NotificationConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &NotificationConfigurationDeprecated)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function_configuration {
            &CloudFunctionConfigurationSerializer::serialize(&mut writer,
                                                             "CloudFunctionConfiguration",
                                                             value)?;
        }
        if let Some(ref value) = obj.queue_configuration {
            &QueueConfigurationDeprecatedSerializer::serialize(&mut writer,
                                                               "QueueConfiguration",
                                                               value)?;
        }
        if let Some(ref value) = obj.topic_configuration {
            &TopicConfigurationDeprecatedSerializer::serialize(&mut writer,
                                                               "TopicConfiguration",
                                                               value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[doc="Container for object key name filtering rules. For information about key name filtering, go to <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html\">Configuring Event Notifications</a> in the Amazon Simple Storage Service Developer Guide."]
#[derive(Default,Debug)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

struct NotificationConfigurationFilterDeserializer;
impl NotificationConfigurationFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<NotificationConfigurationFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfigurationFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "S3Key" => {
                            obj.key = Some(try!(S3KeyFilterDeserializer::deserialize("S3Key",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NotificationConfigurationFilterSerializer;
impl NotificationConfigurationFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &NotificationConfigurationFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.key {
            &S3KeyFilterSerializer::serialize(&mut writer, "S3Key", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct NotificationIdDeserializer;
impl NotificationIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct NotificationIdSerializer;
impl NotificationIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Object {
    pub e_tag: Option<String>,
    pub key: Option<String>,
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    pub size: Option<i64>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
}

struct ObjectDeserializer;
impl ObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Object, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Object::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        "Size" => {
                            obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(ObjectStorageClassDeserializer::deserialize("StorageClass",
                                                                                      stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ObjectIdentifier {
    #[doc="Key name of the object to delete."]
    pub key: String,
    #[doc="VersionId for the specific version of the object to delete."]
    pub version_id: Option<String>,
}


pub struct ObjectIdentifierSerializer;
impl ObjectIdentifierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &ObjectIdentifier)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.key)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.version_id {
            writer
                .write(xml::writer::XmlEvent::start_element("VersionId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}


pub struct ObjectIdentifierListSerializer;
impl ObjectIdentifierListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<ObjectIdentifier>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            ObjectIdentifierSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct ObjectKeyDeserializer;
impl ObjectKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ObjectKeySerializer;
impl ObjectKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ObjectListDeserializer;
impl ObjectListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Object>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ObjectDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct ObjectStorageClassDeserializer;
impl ObjectStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct ObjectVersion {
    pub e_tag: Option<String>,
    #[doc="Specifies whether the object is (true) or is not (false) the latest version of an object."]
    pub is_latest: Option<bool>,
    #[doc="The object key."]
    pub key: Option<String>,
    #[doc="Date and time the object was last modified."]
    pub last_modified: Option<String>,
    pub owner: Option<Owner>,
    #[doc="Size in bytes of the object."]
    pub size: Option<i64>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
    #[doc="Version ID of an object."]
    pub version_id: Option<String>,
}

struct ObjectVersionDeserializer;
impl ObjectVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ObjectVersion::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "IsLatest" => {
                            obj.is_latest = Some(try!(IsLatestDeserializer::deserialize("IsLatest",
                                                                                        stack)));
                        }
                        "Key" => {
                            obj.key = Some(try!(ObjectKeyDeserializer::deserialize("Key", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        "Owner" => {
                            obj.owner = Some(try!(OwnerDeserializer::deserialize("Owner", stack)));
                        }
                        "Size" => {
                            obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class = Some(try!(ObjectVersionStorageClassDeserializer::deserialize("StorageClass", stack)));
                        }
                        "VersionId" => {
                            obj.version_id =
                                Some(try!(ObjectVersionIdDeserializer::deserialize("VersionId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct ObjectVersionIdDeserializer;
impl ObjectVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ObjectVersionIdSerializer;
impl ObjectVersionIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ObjectVersionListDeserializer;
impl ObjectVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<ObjectVersion>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ObjectVersionDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct ObjectVersionStorageClassDeserializer;
impl ObjectVersionStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct Owner {
    pub display_name: Option<String>,
    pub id: Option<String>,
}

struct OwnerDeserializer;
impl OwnerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Owner, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Owner::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DisplayName" => {
                            obj.display_name =
                                Some(try!(DisplayNameDeserializer::deserialize("DisplayName",
                                                                               stack)));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct OwnerSerializer;
impl OwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Owner)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer
                .write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct Part {
    #[doc="Entity tag returned when the part was uploaded."]
    pub e_tag: Option<String>,
    #[doc="Date and time at which the part was uploaded."]
    pub last_modified: Option<String>,
    #[doc="Part number identifying the part. This is a positive integer between 1 and 10,000."]
    pub part_number: Option<i64>,
    #[doc="Size of the uploaded part data."]
    pub size: Option<i64>,
}

struct PartDeserializer;
impl PartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Part, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Part::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ETag" => {
                            obj.e_tag = Some(try!(ETagDeserializer::deserialize("ETag", stack)));
                        }
                        "LastModified" => {
                            obj.last_modified =
                                Some(try!(LastModifiedDeserializer::deserialize("LastModified",
                                                                                stack)));
                        }
                        "PartNumber" => {
                            obj.part_number =
                                Some(try!(PartNumberDeserializer::deserialize("PartNumber",
                                                                              stack)));
                        }
                        "Size" => {
                            obj.size = Some(try!(SizeDeserializer::deserialize("Size", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct PartNumberDeserializer;
impl PartNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct PartNumberSerializer;
impl PartNumberSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct PartNumberMarkerDeserializer;
impl PartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct PartNumberMarkerSerializer;
impl PartNumberMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &i64)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct PartsDeserializer;
impl PartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Part>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(PartDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}
struct PayerDeserializer;
impl PayerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct PayerSerializer;
impl PayerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct PermissionDeserializer;
impl PermissionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct PermissionSerializer;
impl PermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct PolicySerializer;
impl PolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct PrefixDeserializer;
impl PrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct PrefixSerializer;
impl PrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ProtocolDeserializer;
impl ProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ProtocolSerializer;
impl ProtocolSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct PutBucketAccelerateConfigurationRequest {
    #[doc="Specifies the Accelerate Configuration you want to set for the bucket."]
    pub accelerate_configuration: AccelerateConfiguration,
    #[doc="Name of the bucket for which the accelerate configuration is set."]
    pub bucket: String,
}

#[derive(Default,Debug)]
pub struct PutBucketAclRequest {
    #[doc="The canned ACL to apply to the bucket."]
    pub acl: Option<String>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub bucket: String,
    pub content_md5: Option<String>,
    #[doc="Allows grantee the read, write, read ACP, and write ACP permissions on the bucket."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to list the objects in the bucket."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the bucket ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to create, overwrite, and delete any object in the bucket."]
    pub grant_write: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable bucket."]
    pub grant_write_acp: Option<String>,
}

#[derive(Default,Debug)]
pub struct PutBucketAnalyticsConfigurationRequest {
    #[doc="The configuration and any analyses for the analytics filter."]
    pub analytics_configuration: AnalyticsConfiguration,
    #[doc="The name of the bucket to which an analytics configuration is stored."]
    pub bucket: String,
    #[doc="The identifier used to represent an analytics configuration."]
    pub id: String,
}

#[derive(Default,Debug)]
pub struct PutBucketCorsRequest {
    pub bucket: String,
    pub cors_configuration: CORSConfiguration,
    pub content_md5: Option<String>,
}

#[derive(Default,Debug)]
pub struct PutBucketInventoryConfigurationRequest {
    #[doc="The name of the bucket where the inventory configuration will be stored."]
    pub bucket: String,
    #[doc="The ID used to identify the inventory configuration."]
    pub id: String,
    #[doc="Specifies the inventory configuration."]
    pub inventory_configuration: InventoryConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub bucket: String,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}

#[derive(Default,Debug)]
pub struct PutBucketLifecycleRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
}

#[derive(Default,Debug)]
pub struct PutBucketLoggingRequest {
    pub bucket: String,
    pub bucket_logging_status: BucketLoggingStatus,
    pub content_md5: Option<String>,
}

#[derive(Default,Debug)]
pub struct PutBucketMetricsConfigurationRequest {
    #[doc="The name of the bucket for which the metrics configuration is set."]
    pub bucket: String,
    #[doc="The ID used to identify the metrics configuration."]
    pub id: String,
    #[doc="Specifies the metrics configuration."]
    pub metrics_configuration: MetricsConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketNotificationConfigurationRequest {
    pub bucket: String,
    pub notification_configuration: NotificationConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketNotificationRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub notification_configuration: NotificationConfigurationDeprecated,
}

#[derive(Default,Debug)]
pub struct PutBucketPolicyRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    #[doc="The bucket policy as a JSON document."]
    pub policy: String,
}

#[derive(Default,Debug)]
pub struct PutBucketReplicationRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub replication_configuration: ReplicationConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketRequestPaymentRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub request_payment_configuration: RequestPaymentConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketTaggingRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub tagging: Tagging,
}

#[derive(Default,Debug)]
pub struct PutBucketVersioningRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    #[doc="The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device."]
    pub mfa: Option<String>,
    pub versioning_configuration: VersioningConfiguration,
}

#[derive(Default,Debug)]
pub struct PutBucketWebsiteRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Default,Debug)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<String>,
}

struct PutObjectAclOutputDeserializer;
impl PutObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<PutObjectAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectAclOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct PutObjectAclRequest {
    #[doc="The canned ACL to apply to the object."]
    pub acl: Option<String>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub bucket: String,
    pub content_md5: Option<String>,
    #[doc="Allows grantee the read, write, read ACP, and write ACP permissions on the bucket."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to list the objects in the bucket."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the bucket ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to create, overwrite, and delete any object in the bucket."]
    pub grant_write: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable bucket."]
    pub grant_write_acp: Option<String>,
    pub key: String,
    pub request_payer: Option<String>,
    #[doc="VersionId used to reference a specific version of the object."]
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct PutObjectOutput {
    #[doc="Entity tag for the uploaded object."]
    pub e_tag: Option<String>,
    #[doc="If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded."]
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="Version of the object."]
    pub version_id: Option<String>,
}

struct PutObjectOutputDeserializer;
impl PutObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<PutObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct PutObjectRequest {
    #[doc="The canned ACL to apply to the object."]
    pub acl: Option<String>,
    #[doc="Object data."]
    pub body: Option<Vec<u8>>,
    #[doc="Name of the bucket to which the PUT operation was initiated."]
    pub bucket: String,
    #[doc="Specifies caching behavior along the request/reply chain."]
    pub cache_control: Option<String>,
    #[doc="Specifies presentational information for the object."]
    pub content_disposition: Option<String>,
    #[doc="Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field."]
    pub content_encoding: Option<String>,
    #[doc="The language the content is in."]
    pub content_language: Option<String>,
    #[doc="Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically."]
    pub content_length: Option<i64>,
    #[doc="The base64-encoded 128-bit MD5 digest of the part data."]
    pub content_md5: Option<String>,
    #[doc="A standard MIME type describing the format of the object data."]
    pub content_type: Option<String>,
    #[doc="The date and time at which the object is no longer cacheable."]
    pub expires: Option<String>,
    #[doc="Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object."]
    pub grant_full_control: Option<String>,
    #[doc="Allows grantee to read the object data and its metadata."]
    pub grant_read: Option<String>,
    #[doc="Allows grantee to read the object ACL."]
    pub grant_read_acp: Option<String>,
    #[doc="Allows grantee to write the ACL for the applicable object."]
    pub grant_write_acp: Option<String>,
    #[doc="Object key for which the PUT operation was initiated."]
    pub key: String,
    #[doc="A map of metadata to store with the object in S3."]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version"]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
    #[doc="The type of storage to use for the object. Defaults to 'STANDARD'."]
    pub storage_class: Option<String>,
    #[doc="The tag-set for the object. The tag-set must be encoded as URL Query parameters"]
    pub tagging: Option<String>,
    #[doc="If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata."]
    pub website_redirect_location: Option<String>,
}

#[derive(Default,Debug)]
pub struct PutObjectTaggingOutput {
    pub version_id: Option<String>,
}

struct PutObjectTaggingOutputDeserializer;
impl PutObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<PutObjectTaggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = PutObjectTaggingOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct PutObjectTaggingRequest {
    pub bucket: String,
    pub content_md5: Option<String>,
    pub key: String,
    pub tagging: Tagging,
    pub version_id: Option<String>,
}

struct QueueArnDeserializer;
impl QueueArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct QueueArnSerializer;
impl QueueArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[doc="Container for specifying an configuration when you want Amazon S3 to publish events to an Amazon Simple Queue Service (Amazon SQS) queue."]
#[derive(Default,Debug)]
pub struct QueueConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    #[doc="Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects events of specified type."]
    pub queue_arn: String,
}

struct QueueConfigurationDeserializer;
impl QueueConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<QueueConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueueConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Event" => {
                            obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                        }
                        "Filter" => {
                            obj.filter = Some(try!(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "Queue" => {
                            obj.queue_arn = try!(QueueArnDeserializer::deserialize("Queue", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct QueueConfigurationSerializer;
impl QueueConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &QueueConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.queue_arn)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct QueueConfigurationDeprecated {
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    pub queue: Option<String>,
}

struct QueueConfigurationDeprecatedDeserializer;
impl QueueConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<QueueConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = QueueConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Event" => {
                            obj.events = Some(try!(EventListDeserializer::deserialize("Event",
                                                                                      stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "Queue" => {
                            obj.queue = Some(try!(QueueArnDeserializer::deserialize("Queue",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct QueueConfigurationDeprecatedSerializer;
impl QueueConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &QueueConfigurationDeprecated)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.queue {
            writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct QueueConfigurationListDeserializer;
impl QueueConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<QueueConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(QueueConfigurationDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct QueueConfigurationListSerializer;
impl QueueConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<QueueConfiguration>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            QueueConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}


pub struct QuietSerializer;
impl QuietSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &bool)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Redirect {
    #[doc="The host name to use in the redirect request."]
    pub host_name: Option<String>,
    #[doc="The HTTP redirect code to use on the response. Not required if one of the siblings is present."]
    pub http_redirect_code: Option<String>,
    #[doc="Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request."]
    pub protocol: Option<String>,
    #[doc="The object key prefix to use in the redirect request. For example, to redirect requests for all pages with prefix docs/ (objects in the docs/ folder) to documents/, you can set a condition block with KeyPrefixEquals set to docs/ and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if one of the siblings is present. Can be present only if ReplaceKeyWith is not provided."]
    pub replace_key_prefix_with: Option<String>,
    #[doc="The specific object key to use in the redirect request. For example, redirect request to error.html. Not required if one of the sibling is present. Can be present only if ReplaceKeyPrefixWith is not provided."]
    pub replace_key_with: Option<String>,
}

struct RedirectDeserializer;
impl RedirectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Redirect, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Redirect::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostName" => {
                            obj.host_name = Some(try!(HostNameDeserializer::deserialize("HostName",
                                                                                        stack)));
                        }
                        "HttpRedirectCode" => {
                            obj.http_redirect_code =
                                Some(try!(HttpRedirectCodeDeserializer::deserialize("HttpRedirectCode",
                                                                                    stack)));
                        }
                        "Protocol" => {
                            obj.protocol = Some(try!(ProtocolDeserializer::deserialize("Protocol",
                                                                                       stack)));
                        }
                        "ReplaceKeyPrefixWith" => {
                            obj.replace_key_prefix_with =
                                Some(try!(ReplaceKeyPrefixWithDeserializer::deserialize("ReplaceKeyPrefixWith",
                                                                                        stack)));
                        }
                        "ReplaceKeyWith" => {
                            obj.replace_key_with =
                                Some(try!(ReplaceKeyWithDeserializer::deserialize("ReplaceKeyWith",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct RedirectSerializer;
impl RedirectSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Redirect)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.host_name {
            writer
                .write(xml::writer::XmlEvent::start_element("HostName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.http_redirect_code {
            writer
                .write(xml::writer::XmlEvent::start_element("HttpRedirectCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.protocol {
            writer
                .write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_prefix_with {
            writer
                .write(xml::writer::XmlEvent::start_element("ReplaceKeyPrefixWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_with {
            writer
                .write(xml::writer::XmlEvent::start_element("ReplaceKeyWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct RedirectAllRequestsTo {
    #[doc="Name of the host where requests will be redirected."]
    pub host_name: String,
    #[doc="Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request."]
    pub protocol: Option<String>,
}

struct RedirectAllRequestsToDeserializer;
impl RedirectAllRequestsToDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RedirectAllRequestsTo, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RedirectAllRequestsTo::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostName" => {
                            obj.host_name = try!(HostNameDeserializer::deserialize("HostName",
                                                                                   stack));
                        }
                        "Protocol" => {
                            obj.protocol = Some(try!(ProtocolDeserializer::deserialize("Protocol",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct RedirectAllRequestsToSerializer;
impl RedirectAllRequestsToSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &RedirectAllRequestsTo)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer
            .write(xml::writer::XmlEvent::start_element("HostName"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.host_name)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.protocol {
            writer
                .write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplaceKeyPrefixWithDeserializer;
impl ReplaceKeyPrefixWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ReplaceKeyPrefixWithSerializer;
impl ReplaceKeyPrefixWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ReplaceKeyWithDeserializer;
impl ReplaceKeyWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ReplaceKeyWithSerializer;
impl ReplaceKeyWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[doc="Container for replication rules. You can add as many as 1,000 rules. Total replication configuration size can be up to 2 MB."]
#[derive(Default,Debug)]
pub struct ReplicationConfiguration {
    #[doc="Amazon Resource Name (ARN) of an IAM role for Amazon S3 to assume when replicating the objects."]
    pub role: String,
    #[doc="Container for information about a particular replication rule. Replication configuration must have at least one rule and can contain up to 1,000 rules."]
    pub rules: Vec<ReplicationRule>,
}

struct ReplicationConfigurationDeserializer;
impl ReplicationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReplicationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Role" => {
                            obj.role = try!(RoleDeserializer::deserialize("Role", stack));
                        }
                        "Rule" => {
                            obj.rules = try!(ReplicationRulesDeserializer::deserialize("Rule",
                                                                                       stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ReplicationConfigurationSerializer;
impl ReplicationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &ReplicationConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Role"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.role)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        ReplicationRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct ReplicationRule {
    pub destination: Destination,
    #[doc="Unique identifier for the rule. The value cannot be longer than 255 characters."]
    pub id: Option<String>,
    #[doc="Object keyname prefix identifying one or more objects to which the rule applies. Maximum prefix length can be up to 1,024 characters. Overlapping prefixes are not supported."]
    pub prefix: String,
    #[doc="The rule is ignored if status is not Enabled."]
    pub status: String,
}

struct ReplicationRuleDeserializer;
impl ReplicationRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReplicationRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Destination" => {
                            obj.destination = try!(DestinationDeserializer::deserialize("Destination",
                                                                                        stack));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        "Prefix" => {
                            obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                        }
                        "Status" => {
                            obj.status =
                                try!(ReplicationRuleStatusDeserializer::deserialize("Status",
                                                                                    stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ReplicationRuleSerializer;
impl ReplicationRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &ReplicationRule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        DestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.prefix)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer
            .write(xml::writer::XmlEvent::start_element("Status"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.status)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicationRuleStatusDeserializer;
impl ReplicationRuleStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ReplicationRuleStatusSerializer;
impl ReplicationRuleStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct ReplicationRulesDeserializer;
impl ReplicationRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<ReplicationRule>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(ReplicationRuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct ReplicationRulesSerializer;
impl ReplicationRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<ReplicationRule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            ReplicationRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct RequestPaymentConfiguration {
    #[doc="Specifies who pays for the download and request fees."]
    pub payer: String,
}


pub struct RequestPaymentConfigurationSerializer;
impl RequestPaymentConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &RequestPaymentConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Payer"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.payer)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}


pub struct ResponseCacheControlSerializer;
impl ResponseCacheControlSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct ResponseContentDispositionSerializer;
impl ResponseContentDispositionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct ResponseContentEncodingSerializer;
impl ResponseContentEncodingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct ResponseContentLanguageSerializer;
impl ResponseContentLanguageSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct ResponseContentTypeSerializer;
impl ResponseContentTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct ResponseExpiresSerializer;
impl ResponseExpiresSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct RestoreObjectOutput {
    pub request_charged: Option<String>,
}

struct RestoreObjectOutputDeserializer;
impl RestoreObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RestoreObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = RestoreObjectOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct RestoreObjectRequest {
    pub bucket: String,
    pub key: String,
    pub request_payer: Option<String>,
    pub restore_request: Option<RestoreRequest>,
    pub version_id: Option<String>,
}

#[derive(Default,Debug)]
pub struct RestoreRequest {
    #[doc="Lifetime of the active copy in days"]
    pub days: i64,
    #[doc="Glacier related prameters pertaining to this job."]
    pub glacier_job_parameters: Option<GlacierJobParameters>,
}


pub struct RestoreRequestSerializer;
impl RestoreRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &RestoreRequest)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Days"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.days)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.glacier_job_parameters {
            &GlacierJobParametersSerializer::serialize(&mut writer, "GlacierJobParameters", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RoleDeserializer;
impl RoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct RoleSerializer;
impl RoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct RoutingRule {
    #[doc="A container for describing a condition that must be met for the specified redirect to apply. For example, 1. If request is for pages in the /docs folder, redirect to the /documents folder. 2. If request results in HTTP error 4xx, redirect request to another host where you might process the error."]
    pub condition: Option<Condition>,
    #[doc="Container for redirect information. You can redirect requests to another host, to another page, or with another protocol. In the event of an error, you can can specify a different error code to return."]
    pub redirect: Redirect,
}

struct RoutingRuleDeserializer;
impl RoutingRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RoutingRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RoutingRule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Condition" => {
                            obj.condition = Some(try!(ConditionDeserializer::deserialize("Condition",
                                                                                         stack)));
                        }
                        "Redirect" => {
                            obj.redirect = try!(RedirectDeserializer::deserialize("Redirect",
                                                                                  stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct RoutingRuleSerializer;
impl RoutingRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &RoutingRule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.condition {
            &ConditionSerializer::serialize(&mut writer, "Condition", value)?;
        }
        RedirectSerializer::serialize(&mut writer, "Redirect", &obj.redirect)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RoutingRulesDeserializer;
impl RoutingRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<RoutingRule>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "RoutingRule" {
                        obj.push(try!(RoutingRuleDeserializer::deserialize("RoutingRule", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}

pub struct RoutingRulesSerializer;
impl RoutingRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<RoutingRule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            RoutingRuleSerializer::serialize(writer, "RoutingRule", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct Rule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    #[doc="Unique identifier for the rule. The value cannot be longer than 255 characters."]
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    #[doc="Prefix identifying one or more objects to which the rule applies."]
    pub prefix: String,
    #[doc="If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied."]
    pub status: String,
    pub transition: Option<Transition>,
}

struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Rule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Rule::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AbortIncompleteMultipartUpload" => {
                            obj.abort_incomplete_multipart_upload = Some(try!(AbortIncompleteMultipartUploadDeserializer::deserialize("AbortIncompleteMultipartUpload", stack)));
                        }
                        "Expiration" => {
                            obj.expiration =
                                Some(try!(LifecycleExpirationDeserializer::deserialize("Expiration",
                                                                                       stack)));
                        }
                        "ID" => {
                            obj.id = Some(try!(IDDeserializer::deserialize("ID", stack)));
                        }
                        "NoncurrentVersionExpiration" => {
                            obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationDeserializer::deserialize("NoncurrentVersionExpiration", stack)));
                        }
                        "NoncurrentVersionTransition" => {
                            obj.noncurrent_version_transition = Some(try!(NoncurrentVersionTransitionDeserializer::deserialize("NoncurrentVersionTransition", stack)));
                        }
                        "Prefix" => {
                            obj.prefix = try!(PrefixDeserializer::deserialize("Prefix", stack));
                        }
                        "Status" => {
                            obj.status = try!(ExpirationStatusDeserializer::deserialize("Status",
                                                                                        stack));
                        }
                        "Transition" => {
                            obj.transition = Some(try!(TransitionDeserializer::deserialize("Transition",
                                                                                           stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct RuleSerializer;
impl RuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Rule)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(&mut writer,
                                                                 "AbortIncompleteMultipartUpload",
                                                                 value)?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(&mut writer,
                                                              "NoncurrentVersionExpiration",
                                                              value)?;
        }
        if let Some(ref value) = obj.noncurrent_version_transition {
            &NoncurrentVersionTransitionSerializer::serialize(&mut writer,
                                                              "NoncurrentVersionTransition",
                                                              value)?;
        }
        writer
            .write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.prefix)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer
            .write(xml::writer::XmlEvent::start_element("Status"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.status)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.transition {
            &TransitionSerializer::serialize(&mut writer, "Transition", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RulesDeserializer;
impl RulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Rule>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(RuleDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct RulesSerializer;
impl RulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<Rule>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            RuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[doc="Container for object key name prefix and suffix filtering rules."]
#[derive(Default,Debug)]
pub struct S3KeyFilter {
    pub filter_rules: Option<Vec<FilterRule>>,
}

struct S3KeyFilterDeserializer;
impl S3KeyFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<S3KeyFilter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3KeyFilter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "FilterRule" => {
                            obj.filter_rules =
                                Some(try!(FilterRuleListDeserializer::deserialize("FilterRule",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct S3KeyFilterSerializer;
impl S3KeyFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &S3KeyFilter)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter_rules {
            &FilterRuleListSerializer::serialize(&mut writer, "FilterRule", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SizeDeserializer;
impl SizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct StartAfterDeserializer;
impl StartAfterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct StartAfterSerializer;
impl StartAfterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct StorageClassDeserializer;
impl StorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct StorageClassSerializer;
impl StorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct StorageClassAnalysis {
    #[doc="A container used to describe how data related to the storage class analysis should be exported."]
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

struct StorageClassAnalysisDeserializer;
impl StorageClassAnalysisDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StorageClassAnalysis, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StorageClassAnalysis::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DataExport" => {
                            obj.data_export = Some(try!(StorageClassAnalysisDataExportDeserializer::deserialize("DataExport", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct StorageClassAnalysisSerializer;
impl StorageClassAnalysisSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &StorageClassAnalysis)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.data_export {
            &StorageClassAnalysisDataExportSerializer::serialize(&mut writer, "DataExport", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct StorageClassAnalysisDataExport {
    #[doc="The place to store the data for an analysis."]
    pub destination: AnalyticsExportDestination,
    #[doc="The version of the output schema to use when exporting data. Must be V_1."]
    pub output_schema_version: String,
}

struct StorageClassAnalysisDataExportDeserializer;
impl StorageClassAnalysisDataExportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StorageClassAnalysisDataExport, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StorageClassAnalysisDataExport::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Destination" => {
                            obj.destination =
                                try!(AnalyticsExportDestinationDeserializer::deserialize("Destination",
                                                                                         stack));
                        }
                        "OutputSchemaVersion" => {
                            obj.output_schema_version = try!(StorageClassAnalysisSchemaVersionDeserializer::deserialize("OutputSchemaVersion", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct StorageClassAnalysisDataExportSerializer;
impl StorageClassAnalysisDataExportSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &StorageClassAnalysisDataExport)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsExportDestinationSerializer::serialize(&mut writer,
                                                        "Destination",
                                                        &obj.destination)?;
        writer
            .write(xml::writer::XmlEvent::start_element("OutputSchemaVersion"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}",
                                                             value = obj.output_schema_version)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct StorageClassAnalysisSchemaVersionDeserializer;
impl StorageClassAnalysisSchemaVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct StorageClassAnalysisSchemaVersionSerializer;
impl StorageClassAnalysisSchemaVersionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct SuffixDeserializer;
impl SuffixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct SuffixSerializer;
impl SuffixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct Tag {
    #[doc="Name of the tag."]
    pub key: String,
    #[doc="Value of the tag."]
    pub value: String,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Key" => {
                            obj.key = try!(ObjectKeyDeserializer::deserialize("Key", stack));
                        }
                        "Value" => {
                            obj.value = try!(ValueDeserializer::deserialize("Value", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TagSerializer;
impl TagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Tag)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.key)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Value"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.value)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TagSetDeserializer;
impl TagSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Tag>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}

pub struct TagSetSerializer;
impl TagSetSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<Tag>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TagSerializer::serialize(writer, "Tag", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct Tagging {
    pub tag_set: Vec<Tag>,
}


pub struct TaggingSerializer;
impl TaggingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Tagging)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        TagSetSerializer::serialize(&mut writer, "TagSet", &obj.tag_set)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetBucketDeserializer;
impl TargetBucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TargetBucketSerializer;
impl TargetBucketSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct TargetGrant {
    pub grantee: Option<Grantee>,
    #[doc="Logging permissions assigned to the Grantee for the bucket."]
    pub permission: Option<String>,
}

struct TargetGrantDeserializer;
impl TargetGrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TargetGrant, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TargetGrant::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Grantee" => {
                            obj.grantee = Some(try!(GranteeDeserializer::deserialize("Grantee",
                                                                                     stack)));
                        }
                        "Permission" => {
                            obj.permission =
                                Some(try!(BucketLogsPermissionDeserializer::deserialize("Permission",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TargetGrantSerializer;
impl TargetGrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &TargetGrant)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer
                .write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetGrantsDeserializer;
impl TargetGrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<TargetGrant>, XmlParseError> {

        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Grant" {
                        obj.push(try!(TargetGrantDeserializer::deserialize("Grant", stack)));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)

    }
}

pub struct TargetGrantsSerializer;
impl TargetGrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<TargetGrant>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TargetGrantSerializer::serialize(writer, "Grant", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct TargetPrefixDeserializer;
impl TargetPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TargetPrefixSerializer;
impl TargetPrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}


pub struct TierSerializer;
impl TierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct TokenDeserializer;
impl TokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TokenSerializer;
impl TokenSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct TopicArnDeserializer;
impl TopicArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TopicArnSerializer;
impl TopicArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[doc="Container for specifying the configuration when you want Amazon S3 to publish events to an Amazon Simple Notification Service (Amazon SNS) topic."]
#[derive(Default,Debug)]
pub struct TopicConfiguration {
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    #[doc="Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects events of specified type."]
    pub topic_arn: String,
}

struct TopicConfigurationDeserializer;
impl TopicConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TopicConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TopicConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Event" => {
                            obj.events = try!(EventListDeserializer::deserialize("Event", stack));
                        }
                        "Filter" => {
                            obj.filter = Some(try!(NotificationConfigurationFilterDeserializer::deserialize("Filter", stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "Topic" => {
                            obj.topic_arn = try!(TopicArnDeserializer::deserialize("Topic", stack));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TopicConfigurationSerializer;
impl TopicConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &TopicConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
        writer
            .write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.topic_arn)))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct TopicConfigurationDeprecated {
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    #[doc="Amazon SNS topic to which Amazon S3 will publish a message to report the specified events for the bucket."]
    pub topic: Option<String>,
}

struct TopicConfigurationDeprecatedDeserializer;
impl TopicConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TopicConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TopicConfigurationDeprecated::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Event" => {
                            obj.events = Some(try!(EventListDeserializer::deserialize("Event",
                                                                                      stack)));
                        }
                        "Id" => {
                            obj.id = Some(try!(NotificationIdDeserializer::deserialize("Id",
                                                                                       stack)));
                        }
                        "Topic" => {
                            obj.topic = Some(try!(TopicArnDeserializer::deserialize("Topic",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TopicConfigurationDeprecatedSerializer;
impl TopicConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &TopicConfigurationDeprecated)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.topic {
            writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TopicConfigurationListDeserializer;
impl TopicConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<TopicConfiguration>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(TopicConfigurationDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct TopicConfigurationListSerializer;
impl TopicConfigurationListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<TopicConfiguration>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            TopicConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default,Debug)]
pub struct Transition {
    #[doc="Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format."]
    pub date: Option<String>,
    #[doc="Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer."]
    pub days: Option<i64>,
    #[doc="The class of storage used to store the object."]
    pub storage_class: Option<String>,
}

struct TransitionDeserializer;
impl TransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Transition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Transition::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Date" => {
                            obj.date = Some(try!(DateDeserializer::deserialize("Date", stack)));
                        }
                        "Days" => {
                            obj.days = Some(try!(DaysDeserializer::deserialize("Days", stack)));
                        }
                        "StorageClass" => {
                            obj.storage_class =
                                Some(try!(TransitionStorageClassDeserializer::deserialize("StorageClass",
                                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TransitionSerializer;
impl TransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Transition)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer
                .write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TransitionListDeserializer;
impl TransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Transition>, XmlParseError> {

        let mut obj = vec![];

        loop {

            let consume_next_tag = match stack.peek() {
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => name.local_name == tag_name,
                _ => false,
            };

            if consume_next_tag {
                obj.push(try!(TransitionDeserializer::deserialize(tag_name, stack)));
            } else {
                break;
            }

        }

        Ok(obj)

    }
}

pub struct TransitionListSerializer;
impl TransitionListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &Vec<Transition>)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        for element in obj {
            TransitionSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct TransitionStorageClassDeserializer;
impl TransitionStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TransitionStorageClassSerializer;
impl TransitionStorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct TypeDeserializer;
impl TypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct TypeSerializer;
impl TypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct URIDeserializer;
impl URIDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct URISerializer;
impl URISerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct UploadIdMarkerDeserializer;
impl UploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct UploadIdMarkerSerializer;
impl UploadIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct UploadPartCopyOutput {
    pub copy_part_result: Option<CopyPartResult>,
    #[doc="The version of the source object that was copied, if you have enabled versioning on the source bucket."]
    pub copy_source_version_id: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
}

struct UploadPartCopyOutputDeserializer;
impl UploadPartCopyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<UploadPartCopyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UploadPartCopyOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CopyPartResult" => {
                            obj.copy_part_result =
                                Some(try!(CopyPartResultDeserializer::deserialize("CopyPartResult",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct UploadPartCopyRequest {
    pub bucket: String,
    #[doc="The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded."]
    pub copy_source: String,
    #[doc="Copies the object if its entity tag (ETag) matches the specified tag."]
    pub copy_source_if_match: Option<String>,
    #[doc="Copies the object if it has been modified since the specified time."]
    pub copy_source_if_modified_since: Option<String>,
    #[doc="Copies the object if its entity tag (ETag) is different than the specified ETag."]
    pub copy_source_if_none_match: Option<String>,
    #[doc="Copies the object if it hasn't been modified since the specified time."]
    pub copy_source_if_unmodified_since: Option<String>,
    #[doc="The range of bytes to copy from the source object. The range value must use the form bytes=first-last, where the first and last are the zero-based byte offsets to copy. For example, bytes=0-9 indicates that you want to copy the first ten bytes of the source. You can copy a range only if the source object is greater than 5 GB."]
    pub copy_source_range: Option<String>,
    #[doc="Specifies the algorithm to use when decrypting the source object (e.g., AES256)."]
    pub copy_source_sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created."]
    pub copy_source_sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub copy_source_sse_customer_key_md5: Option<String>,
    pub key: String,
    #[doc="Part number of part being copied. This is a positive integer between 1 and 10,000."]
    pub part_number: i64,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="Upload ID identifying the multipart upload whose part is being copied."]
    pub upload_id: String,
}

#[derive(Default,Debug)]
pub struct UploadPartOutput {
    #[doc="Entity tag for the uploaded object."]
    pub e_tag: Option<String>,
    pub request_charged: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object."]
    pub ssekms_key_id: Option<String>,
    #[doc="The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms)."]
    pub server_side_encryption: Option<String>,
}

struct UploadPartOutputDeserializer;
impl UploadPartOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<UploadPartOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = UploadPartOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[derive(Default,Debug)]
pub struct UploadPartRequest {
    #[doc="Object data."]
    pub body: Option<Vec<u8>>,
    #[doc="Name of the bucket to which the multipart upload was initiated."]
    pub bucket: String,
    #[doc="Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically."]
    pub content_length: Option<i64>,
    #[doc="The base64-encoded 128-bit MD5 digest of the part data."]
    pub content_md5: Option<String>,
    #[doc="Object key for which the multipart upload was initiated."]
    pub key: String,
    #[doc="Part number of part being uploaded. This is a positive integer between 1 and 10,000."]
    pub part_number: i64,
    pub request_payer: Option<String>,
    #[doc="Specifies the algorithm to use to when encrypting the object (e.g., AES256)."]
    pub sse_customer_algorithm: Option<String>,
    #[doc="Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request."]
    pub sse_customer_key: Option<String>,
    #[doc="Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error."]
    pub sse_customer_key_md5: Option<String>,
    #[doc="Upload ID identifying the multipart upload whose part is being uploaded."]
    pub upload_id: String,
}

struct ValueDeserializer;
impl ValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct ValueSerializer;
impl ValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

struct VersionIdMarkerDeserializer;
impl VersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}

pub struct VersionIdMarkerSerializer;
impl VersionIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &String)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {

        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = obj.to_string())))?;
        writer.write(xml::writer::XmlEvent::end_element())

    }
}

#[derive(Default,Debug)]
pub struct VersioningConfiguration {
    #[doc="Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned."]
    pub mfa_delete: Option<String>,
    #[doc="The versioning state of the bucket."]
    pub status: Option<String>,
}


pub struct VersioningConfigurationSerializer;
impl VersioningConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &VersioningConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.mfa_delete {
            writer
                .write(xml::writer::XmlEvent::start_element("MfaDelete"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.status {
            writer
                .write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!("{value}", value = value)));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default,Debug)]
pub struct WebsiteConfiguration {
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub routing_rules: Option<Vec<RoutingRule>>,
}


pub struct WebsiteConfigurationSerializer;
impl WebsiteConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(mut writer: &mut EventWriter<W>,
                        name: &str,
                        obj: &WebsiteConfiguration)
                        -> Result<(), xml::writer::Error>
        where W: Write
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.error_document {
            &ErrorDocumentSerializer::serialize(&mut writer, "ErrorDocument", value)?;
        }
        if let Some(ref value) = obj.index_document {
            &IndexDocumentSerializer::serialize(&mut writer, "IndexDocument", value)?;
        }
        if let Some(ref value) = obj.redirect_all_requests_to {
            &RedirectAllRequestsToSerializer::serialize(&mut writer,
                                                        "RedirectAllRequestsTo",
                                                        value)?;
        }
        if let Some(ref value) = obj.routing_rules {
            &RoutingRulesSerializer::serialize(&mut writer, "RoutingRules", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// Errors returned by AbortMultipartUpload
#[derive(Debug, PartialEq)]
pub enum AbortMultipartUploadError {
    ///The specified multipart upload does not exist.
    NoSuchUpload(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AbortMultipartUploadError {
    pub fn from_body(body: &str) -> AbortMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchUpload" => {
                        AbortMultipartUploadError::NoSuchUpload(String::from(parsed_error.message))
                    }
                    _ => AbortMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => AbortMultipartUploadError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for AbortMultipartUploadError {
    fn from(err: XmlParseError) -> AbortMultipartUploadError {
        let XmlParseError(message) = err;
        AbortMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AbortMultipartUploadError {
    fn from(err: CredentialsError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortMultipartUploadError {
    fn from(err: HttpDispatchError) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortMultipartUploadError {
    fn from(err: io::Error) -> AbortMultipartUploadError {
        AbortMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            AbortMultipartUploadError::NoSuchUpload(ref cause) => cause,
            AbortMultipartUploadError::Validation(ref cause) => cause,
            AbortMultipartUploadError::Credentials(ref err) => err.description(),
            AbortMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AbortMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CompleteMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CompleteMultipartUploadError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CompleteMultipartUploadError {
    pub fn from_body(body: &str) -> CompleteMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => CompleteMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => CompleteMultipartUploadError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CompleteMultipartUploadError {
    fn from(err: XmlParseError) -> CompleteMultipartUploadError {
        let XmlParseError(message) = err;
        CompleteMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CompleteMultipartUploadError {
    fn from(err: CredentialsError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompleteMultipartUploadError {
    fn from(err: HttpDispatchError) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CompleteMultipartUploadError {
    fn from(err: io::Error) -> CompleteMultipartUploadError {
        CompleteMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CompleteMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            CompleteMultipartUploadError::Validation(ref cause) => cause,
            CompleteMultipartUploadError::Credentials(ref err) => err.description(),
            CompleteMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CompleteMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CopyObject
#[derive(Debug, PartialEq)]
pub enum CopyObjectError {
    ///The source object of the COPY operation is not in the active tier and is only stored in Amazon Glacier.
    ObjectNotInActiveTierError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CopyObjectError {
    pub fn from_body(body: &str) -> CopyObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ObjectNotInActiveTierError" => CopyObjectError::ObjectNotInActiveTierError(String::from(parsed_error.message)),
                    _ => CopyObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CopyObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CopyObjectError {
    fn from(err: XmlParseError) -> CopyObjectError {
        let XmlParseError(message) = err;
        CopyObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CopyObjectError {
    fn from(err: CredentialsError) -> CopyObjectError {
        CopyObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopyObjectError {
    fn from(err: HttpDispatchError) -> CopyObjectError {
        CopyObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CopyObjectError {
    fn from(err: io::Error) -> CopyObjectError {
        CopyObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CopyObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyObjectError {
    fn description(&self) -> &str {
        match *self {
            CopyObjectError::ObjectNotInActiveTierError(ref cause) => cause,
            CopyObjectError::Validation(ref cause) => cause,
            CopyObjectError::Credentials(ref err) => err.description(),
            CopyObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CopyObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBucket
#[derive(Debug, PartialEq)]
pub enum CreateBucketError {
    ///The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again.
    BucketAlreadyExists(String),
    ///
    BucketAlreadyOwnedByYou(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateBucketError {
    pub fn from_body(body: &str) -> CreateBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "BucketAlreadyExists" => {
                        CreateBucketError::BucketAlreadyExists(String::from(parsed_error.message))
                    }
                    "BucketAlreadyOwnedByYou" => CreateBucketError::BucketAlreadyOwnedByYou(String::from(parsed_error.message)),
                    _ => CreateBucketError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBucketError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateBucketError {
    fn from(err: XmlParseError) -> CreateBucketError {
        let XmlParseError(message) = err;
        CreateBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateBucketError {
    fn from(err: CredentialsError) -> CreateBucketError {
        CreateBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBucketError {
    fn from(err: HttpDispatchError) -> CreateBucketError {
        CreateBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBucketError {
    fn from(err: io::Error) -> CreateBucketError {
        CreateBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBucketError {
    fn description(&self) -> &str {
        match *self {
            CreateBucketError::BucketAlreadyExists(ref cause) => cause,
            CreateBucketError::BucketAlreadyOwnedByYou(ref cause) => cause,
            CreateBucketError::Validation(ref cause) => cause,
            CreateBucketError::Credentials(ref err) => err.description(),
            CreateBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CreateMultipartUploadError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateMultipartUploadError {
    pub fn from_body(body: &str) -> CreateMultipartUploadError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => CreateMultipartUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMultipartUploadError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateMultipartUploadError {
    fn from(err: XmlParseError) -> CreateMultipartUploadError {
        let XmlParseError(message) = err;
        CreateMultipartUploadError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateMultipartUploadError {
    fn from(err: CredentialsError) -> CreateMultipartUploadError {
        CreateMultipartUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMultipartUploadError {
    fn from(err: HttpDispatchError) -> CreateMultipartUploadError {
        CreateMultipartUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMultipartUploadError {
    fn from(err: io::Error) -> CreateMultipartUploadError {
        CreateMultipartUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMultipartUploadError {
    fn description(&self) -> &str {
        match *self {
            CreateMultipartUploadError::Validation(ref cause) => cause,
            CreateMultipartUploadError::Credentials(ref err) => err.description(),
            CreateMultipartUploadError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMultipartUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucket
#[derive(Debug, PartialEq)]
pub enum DeleteBucketError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketError {
    pub fn from_body(body: &str) -> DeleteBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketError {
    fn from(err: XmlParseError) -> DeleteBucketError {
        let XmlParseError(message) = err;
        DeleteBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketError {
    fn from(err: CredentialsError) -> DeleteBucketError {
        DeleteBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketError {
    fn from(err: HttpDispatchError) -> DeleteBucketError {
        DeleteBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketError {
    fn from(err: io::Error) -> DeleteBucketError {
        DeleteBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketError::Validation(ref cause) => cause,
            DeleteBucketError::Credentials(ref err) => err.description(),
            DeleteBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketAnalyticsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> DeleteBucketAnalyticsConfigurationError {
        DeleteBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            DeleteBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketCors
#[derive(Debug, PartialEq)]
pub enum DeleteBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketCorsError {
    pub fn from_body(body: &str) -> DeleteBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketCorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketCorsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketCorsError {
    fn from(err: XmlParseError) -> DeleteBucketCorsError {
        let XmlParseError(message) = err;
        DeleteBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketCorsError {
    fn from(err: CredentialsError) -> DeleteBucketCorsError {
        DeleteBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketCorsError {
    fn from(err: HttpDispatchError) -> DeleteBucketCorsError {
        DeleteBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketCorsError {
    fn from(err: io::Error) -> DeleteBucketCorsError {
        DeleteBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketCorsError::Validation(ref cause) => cause,
            DeleteBucketCorsError::Credentials(ref err) => err.description(),
            DeleteBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketInventoryConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketInventoryConfigurationError {
    fn from(err: io::Error) -> DeleteBucketInventoryConfigurationError {
        DeleteBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketInventoryConfigurationError::Validation(ref cause) => cause,
            DeleteBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum DeleteBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketLifecycleError {
    pub fn from_body(body: &str) -> DeleteBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketLifecycleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketLifecycleError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketLifecycleError {
    fn from(err: XmlParseError) -> DeleteBucketLifecycleError {
        let XmlParseError(message) = err;
        DeleteBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketLifecycleError {
    fn from(err: CredentialsError) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketLifecycleError {
    fn from(err: HttpDispatchError) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketLifecycleError {
    fn from(err: io::Error) -> DeleteBucketLifecycleError {
        DeleteBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketLifecycleError::Validation(ref cause) => cause,
            DeleteBucketLifecycleError::Credentials(ref err) => err.description(),
            DeleteBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> DeleteBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketMetricsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> DeleteBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        DeleteBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketMetricsConfigurationError {
    fn from(err: io::Error) -> DeleteBucketMetricsConfigurationError {
        DeleteBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketMetricsConfigurationError::Validation(ref cause) => cause,
            DeleteBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            DeleteBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketPolicyError {
    pub fn from_body(body: &str) -> DeleteBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketPolicyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketPolicyError {
    fn from(err: XmlParseError) -> DeleteBucketPolicyError {
        let XmlParseError(message) = err;
        DeleteBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketPolicyError {
    fn from(err: CredentialsError) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketPolicyError {
    fn from(err: HttpDispatchError) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketPolicyError {
    fn from(err: io::Error) -> DeleteBucketPolicyError {
        DeleteBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketPolicyError::Validation(ref cause) => cause,
            DeleteBucketPolicyError::Credentials(ref err) => err.description(),
            DeleteBucketPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketReplication
#[derive(Debug, PartialEq)]
pub enum DeleteBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketReplicationError {
    pub fn from_body(body: &str) -> DeleteBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketReplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketReplicationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketReplicationError {
    fn from(err: XmlParseError) -> DeleteBucketReplicationError {
        let XmlParseError(message) = err;
        DeleteBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketReplicationError {
    fn from(err: CredentialsError) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketReplicationError {
    fn from(err: HttpDispatchError) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketReplicationError {
    fn from(err: io::Error) -> DeleteBucketReplicationError {
        DeleteBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketReplicationError::Validation(ref cause) => cause,
            DeleteBucketReplicationError::Credentials(ref err) => err.description(),
            DeleteBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketTagging
#[derive(Debug, PartialEq)]
pub enum DeleteBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketTaggingError {
    pub fn from_body(body: &str) -> DeleteBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketTaggingError {
    fn from(err: XmlParseError) -> DeleteBucketTaggingError {
        let XmlParseError(message) = err;
        DeleteBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketTaggingError {
    fn from(err: CredentialsError) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketTaggingError {
    fn from(err: HttpDispatchError) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketTaggingError {
    fn from(err: io::Error) -> DeleteBucketTaggingError {
        DeleteBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketTaggingError::Validation(ref cause) => cause,
            DeleteBucketTaggingError::Credentials(ref err) => err.description(),
            DeleteBucketTaggingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBucketWebsite
#[derive(Debug, PartialEq)]
pub enum DeleteBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteBucketWebsiteError {
    pub fn from_body(body: &str) -> DeleteBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteBucketWebsiteError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBucketWebsiteError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteBucketWebsiteError {
    fn from(err: XmlParseError) -> DeleteBucketWebsiteError {
        let XmlParseError(message) = err;
        DeleteBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteBucketWebsiteError {
    fn from(err: CredentialsError) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBucketWebsiteError {
    fn from(err: HttpDispatchError) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBucketWebsiteError {
    fn from(err: io::Error) -> DeleteBucketWebsiteError {
        DeleteBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            DeleteBucketWebsiteError::Validation(ref cause) => cause,
            DeleteBucketWebsiteError::Credentials(ref err) => err.description(),
            DeleteBucketWebsiteError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteObjectError {
    pub fn from_body(body: &str) -> DeleteObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteObjectError {
    fn from(err: XmlParseError) -> DeleteObjectError {
        let XmlParseError(message) = err;
        DeleteObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectError {
    fn from(err: CredentialsError) -> DeleteObjectError {
        DeleteObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectError {
    fn from(err: HttpDispatchError) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectError {
    fn from(err: io::Error) -> DeleteObjectError {
        DeleteObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectError::Validation(ref cause) => cause,
            DeleteObjectError::Credentials(ref err) => err.description(),
            DeleteObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObjectTagging
#[derive(Debug, PartialEq)]
pub enum DeleteObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteObjectTaggingError {
    pub fn from_body(body: &str) -> DeleteObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteObjectTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteObjectTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteObjectTaggingError {
    fn from(err: XmlParseError) -> DeleteObjectTaggingError {
        let XmlParseError(message) = err;
        DeleteObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectTaggingError {
    fn from(err: CredentialsError) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectTaggingError {
    fn from(err: HttpDispatchError) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectTaggingError {
    fn from(err: io::Error) -> DeleteObjectTaggingError {
        DeleteObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectTaggingError::Validation(ref cause) => cause,
            DeleteObjectTaggingError::Credentials(ref err) => err.description(),
            DeleteObjectTaggingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteObjects
#[derive(Debug, PartialEq)]
pub enum DeleteObjectsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteObjectsError {
    pub fn from_body(body: &str) -> DeleteObjectsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DeleteObjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteObjectsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteObjectsError {
    fn from(err: XmlParseError) -> DeleteObjectsError {
        let XmlParseError(message) = err;
        DeleteObjectsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteObjectsError {
    fn from(err: CredentialsError) -> DeleteObjectsError {
        DeleteObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteObjectsError {
    fn from(err: HttpDispatchError) -> DeleteObjectsError {
        DeleteObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteObjectsError {
    fn from(err: io::Error) -> DeleteObjectsError {
        DeleteObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectsError {
    fn description(&self) -> &str {
        match *self {
            DeleteObjectsError::Validation(ref cause) => cause,
            DeleteObjectsError::Credentials(ref err) => err.description(),
            DeleteObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAccelerateConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketAccelerateConfigurationError {
    pub fn from_body(body: &str) -> GetBucketAccelerateConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketAccelerateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketAccelerateConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketAccelerateConfigurationError {
    fn from(err: XmlParseError) -> GetBucketAccelerateConfigurationError {
        let XmlParseError(message) = err;
        GetBucketAccelerateConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAccelerateConfigurationError {
    fn from(err: CredentialsError) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAccelerateConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAccelerateConfigurationError {
    fn from(err: io::Error) -> GetBucketAccelerateConfigurationError {
        GetBucketAccelerateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAccelerateConfigurationError::Validation(ref cause) => cause,
            GetBucketAccelerateConfigurationError::Credentials(ref err) => err.description(),
            GetBucketAccelerateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketAccelerateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAcl
#[derive(Debug, PartialEq)]
pub enum GetBucketAclError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketAclError {
    pub fn from_body(body: &str) -> GetBucketAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketAclError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketAclError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketAclError {
    fn from(err: XmlParseError) -> GetBucketAclError {
        let XmlParseError(message) = err;
        GetBucketAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAclError {
    fn from(err: CredentialsError) -> GetBucketAclError {
        GetBucketAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAclError {
    fn from(err: HttpDispatchError) -> GetBucketAclError {
        GetBucketAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAclError {
    fn from(err: io::Error) -> GetBucketAclError {
        GetBucketAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAclError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAclError::Validation(ref cause) => cause,
            GetBucketAclError::Credentials(ref err) => err.description(),
            GetBucketAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> GetBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketAnalyticsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> GetBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        GetBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> GetBucketAnalyticsConfigurationError {
        GetBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            GetBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            GetBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketCors
#[derive(Debug, PartialEq)]
pub enum GetBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketCorsError {
    pub fn from_body(body: &str) -> GetBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketCorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketCorsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketCorsError {
    fn from(err: XmlParseError) -> GetBucketCorsError {
        let XmlParseError(message) = err;
        GetBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketCorsError {
    fn from(err: CredentialsError) -> GetBucketCorsError {
        GetBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketCorsError {
    fn from(err: HttpDispatchError) -> GetBucketCorsError {
        GetBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketCorsError {
    fn from(err: io::Error) -> GetBucketCorsError {
        GetBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            GetBucketCorsError::Validation(ref cause) => cause,
            GetBucketCorsError::Credentials(ref err) => err.description(),
            GetBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> GetBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketInventoryConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> GetBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        GetBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketInventoryConfigurationError {
    fn from(err: io::Error) -> GetBucketInventoryConfigurationError {
        GetBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketInventoryConfigurationError::Validation(ref cause) => cause,
            GetBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            GetBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketLifecycleError {
    pub fn from_body(body: &str) -> GetBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketLifecycleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketLifecycleError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketLifecycleError {
    fn from(err: XmlParseError) -> GetBucketLifecycleError {
        let XmlParseError(message) = err;
        GetBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLifecycleError {
    fn from(err: CredentialsError) -> GetBucketLifecycleError {
        GetBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLifecycleError {
    fn from(err: HttpDispatchError) -> GetBucketLifecycleError {
        GetBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLifecycleError {
    fn from(err: io::Error) -> GetBucketLifecycleError {
        GetBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLifecycleError::Validation(ref cause) => cause,
            GetBucketLifecycleError::Credentials(ref err) => err.description(),
            GetBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketLifecycleConfigurationError {
    pub fn from_body(body: &str) -> GetBucketLifecycleConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketLifecycleConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketLifecycleConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketLifecycleConfigurationError {
    fn from(err: XmlParseError) -> GetBucketLifecycleConfigurationError {
        let XmlParseError(message) = err;
        GetBucketLifecycleConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLifecycleConfigurationError {
    fn from(err: CredentialsError) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLifecycleConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLifecycleConfigurationError {
    fn from(err: io::Error) -> GetBucketLifecycleConfigurationError {
        GetBucketLifecycleConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLifecycleConfigurationError::Validation(ref cause) => cause,
            GetBucketLifecycleConfigurationError::Credentials(ref err) => err.description(),
            GetBucketLifecycleConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLifecycleConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLocation
#[derive(Debug, PartialEq)]
pub enum GetBucketLocationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketLocationError {
    pub fn from_body(body: &str) -> GetBucketLocationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketLocationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketLocationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketLocationError {
    fn from(err: XmlParseError) -> GetBucketLocationError {
        let XmlParseError(message) = err;
        GetBucketLocationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLocationError {
    fn from(err: CredentialsError) -> GetBucketLocationError {
        GetBucketLocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLocationError {
    fn from(err: HttpDispatchError) -> GetBucketLocationError {
        GetBucketLocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLocationError {
    fn from(err: io::Error) -> GetBucketLocationError {
        GetBucketLocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLocationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLocationError::Validation(ref cause) => cause,
            GetBucketLocationError::Credentials(ref err) => err.description(),
            GetBucketLocationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketLocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketLogging
#[derive(Debug, PartialEq)]
pub enum GetBucketLoggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketLoggingError {
    pub fn from_body(body: &str) -> GetBucketLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketLoggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketLoggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketLoggingError {
    fn from(err: XmlParseError) -> GetBucketLoggingError {
        let XmlParseError(message) = err;
        GetBucketLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketLoggingError {
    fn from(err: CredentialsError) -> GetBucketLoggingError {
        GetBucketLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketLoggingError {
    fn from(err: HttpDispatchError) -> GetBucketLoggingError {
        GetBucketLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketLoggingError {
    fn from(err: io::Error) -> GetBucketLoggingError {
        GetBucketLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLoggingError {
    fn description(&self) -> &str {
        match *self {
            GetBucketLoggingError::Validation(ref cause) => cause,
            GetBucketLoggingError::Credentials(ref err) => err.description(),
            GetBucketLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> GetBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketMetricsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> GetBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        GetBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketMetricsConfigurationError {
    fn from(err: io::Error) -> GetBucketMetricsConfigurationError {
        GetBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketMetricsConfigurationError::Validation(ref cause) => cause,
            GetBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            GetBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketNotification
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketNotificationError {
    pub fn from_body(body: &str) -> GetBucketNotificationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketNotificationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketNotificationError {
    fn from(err: XmlParseError) -> GetBucketNotificationError {
        let XmlParseError(message) = err;
        GetBucketNotificationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketNotificationError {
    fn from(err: CredentialsError) -> GetBucketNotificationError {
        GetBucketNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketNotificationError {
    fn from(err: HttpDispatchError) -> GetBucketNotificationError {
        GetBucketNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketNotificationError {
    fn from(err: io::Error) -> GetBucketNotificationError {
        GetBucketNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketNotificationError::Validation(ref cause) => cause,
            GetBucketNotificationError::Credentials(ref err) => err.description(),
            GetBucketNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketNotificationConfigurationError {
    pub fn from_body(body: &str) -> GetBucketNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketNotificationConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketNotificationConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketNotificationConfigurationError {
    fn from(err: XmlParseError) -> GetBucketNotificationConfigurationError {
        let XmlParseError(message) = err;
        GetBucketNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketNotificationConfigurationError {
    fn from(err: CredentialsError) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketNotificationConfigurationError {
    fn from(err: io::Error) -> GetBucketNotificationConfigurationError {
        GetBucketNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketNotificationConfigurationError::Validation(ref cause) => cause,
            GetBucketNotificationConfigurationError::Credentials(ref err) => err.description(),
            GetBucketNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketPolicy
#[derive(Debug, PartialEq)]
pub enum GetBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketPolicyError {
    pub fn from_body(body: &str) -> GetBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketPolicyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketPolicyError {
    fn from(err: XmlParseError) -> GetBucketPolicyError {
        let XmlParseError(message) = err;
        GetBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketPolicyError {
    fn from(err: CredentialsError) -> GetBucketPolicyError {
        GetBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketPolicyError {
    fn from(err: HttpDispatchError) -> GetBucketPolicyError {
        GetBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketPolicyError {
    fn from(err: io::Error) -> GetBucketPolicyError {
        GetBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetBucketPolicyError::Validation(ref cause) => cause,
            GetBucketPolicyError::Credentials(ref err) => err.description(),
            GetBucketPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketReplication
#[derive(Debug, PartialEq)]
pub enum GetBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketReplicationError {
    pub fn from_body(body: &str) -> GetBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketReplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketReplicationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketReplicationError {
    fn from(err: XmlParseError) -> GetBucketReplicationError {
        let XmlParseError(message) = err;
        GetBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketReplicationError {
    fn from(err: CredentialsError) -> GetBucketReplicationError {
        GetBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketReplicationError {
    fn from(err: HttpDispatchError) -> GetBucketReplicationError {
        GetBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketReplicationError {
    fn from(err: io::Error) -> GetBucketReplicationError {
        GetBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            GetBucketReplicationError::Validation(ref cause) => cause,
            GetBucketReplicationError::Credentials(ref err) => err.description(),
            GetBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum GetBucketRequestPaymentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketRequestPaymentError {
    pub fn from_body(body: &str) -> GetBucketRequestPaymentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketRequestPaymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketRequestPaymentError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketRequestPaymentError {
    fn from(err: XmlParseError) -> GetBucketRequestPaymentError {
        let XmlParseError(message) = err;
        GetBucketRequestPaymentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketRequestPaymentError {
    fn from(err: CredentialsError) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketRequestPaymentError {
    fn from(err: HttpDispatchError) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketRequestPaymentError {
    fn from(err: io::Error) -> GetBucketRequestPaymentError {
        GetBucketRequestPaymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {
            GetBucketRequestPaymentError::Validation(ref cause) => cause,
            GetBucketRequestPaymentError::Credentials(ref err) => err.description(),
            GetBucketRequestPaymentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketRequestPaymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketTagging
#[derive(Debug, PartialEq)]
pub enum GetBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketTaggingError {
    pub fn from_body(body: &str) -> GetBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketTaggingError {
    fn from(err: XmlParseError) -> GetBucketTaggingError {
        let XmlParseError(message) = err;
        GetBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketTaggingError {
    fn from(err: CredentialsError) -> GetBucketTaggingError {
        GetBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketTaggingError {
    fn from(err: HttpDispatchError) -> GetBucketTaggingError {
        GetBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketTaggingError {
    fn from(err: io::Error) -> GetBucketTaggingError {
        GetBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            GetBucketTaggingError::Validation(ref cause) => cause,
            GetBucketTaggingError::Credentials(ref err) => err.description(),
            GetBucketTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketVersioning
#[derive(Debug, PartialEq)]
pub enum GetBucketVersioningError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketVersioningError {
    pub fn from_body(body: &str) -> GetBucketVersioningError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketVersioningError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketVersioningError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketVersioningError {
    fn from(err: XmlParseError) -> GetBucketVersioningError {
        let XmlParseError(message) = err;
        GetBucketVersioningError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketVersioningError {
    fn from(err: CredentialsError) -> GetBucketVersioningError {
        GetBucketVersioningError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketVersioningError {
    fn from(err: HttpDispatchError) -> GetBucketVersioningError {
        GetBucketVersioningError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketVersioningError {
    fn from(err: io::Error) -> GetBucketVersioningError {
        GetBucketVersioningError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketVersioningError {
    fn description(&self) -> &str {
        match *self {
            GetBucketVersioningError::Validation(ref cause) => cause,
            GetBucketVersioningError::Credentials(ref err) => err.description(),
            GetBucketVersioningError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBucketVersioningError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBucketWebsite
#[derive(Debug, PartialEq)]
pub enum GetBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBucketWebsiteError {
    pub fn from_body(body: &str) -> GetBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetBucketWebsiteError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBucketWebsiteError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetBucketWebsiteError {
    fn from(err: XmlParseError) -> GetBucketWebsiteError {
        let XmlParseError(message) = err;
        GetBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetBucketWebsiteError {
    fn from(err: CredentialsError) -> GetBucketWebsiteError {
        GetBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBucketWebsiteError {
    fn from(err: HttpDispatchError) -> GetBucketWebsiteError {
        GetBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBucketWebsiteError {
    fn from(err: io::Error) -> GetBucketWebsiteError {
        GetBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            GetBucketWebsiteError::Validation(ref cause) => cause,
            GetBucketWebsiteError::Credentials(ref err) => err.description(),
            GetBucketWebsiteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObject
#[derive(Debug, PartialEq)]
pub enum GetObjectError {
    ///The specified key does not exist.
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetObjectError {
    pub fn from_body(body: &str) -> GetObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchKey" => GetObjectError::NoSuchKey(String::from(parsed_error.message)),
                    _ => GetObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetObjectError {
    fn from(err: XmlParseError) -> GetObjectError {
        let XmlParseError(message) = err;
        GetObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectError {
    fn from(err: CredentialsError) -> GetObjectError {
        GetObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectError {
    fn from(err: HttpDispatchError) -> GetObjectError {
        GetObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectError {
    fn from(err: io::Error) -> GetObjectError {
        GetObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectError {
    fn description(&self) -> &str {
        match *self {
            GetObjectError::NoSuchKey(ref cause) => cause,
            GetObjectError::Validation(ref cause) => cause,
            GetObjectError::Credentials(ref err) => err.description(),
            GetObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectAcl
#[derive(Debug, PartialEq)]
pub enum GetObjectAclError {
    ///The specified key does not exist.
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetObjectAclError {
    pub fn from_body(body: &str) -> GetObjectAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchKey" => GetObjectAclError::NoSuchKey(String::from(parsed_error.message)),
                    _ => GetObjectAclError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetObjectAclError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetObjectAclError {
    fn from(err: XmlParseError) -> GetObjectAclError {
        let XmlParseError(message) = err;
        GetObjectAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectAclError {
    fn from(err: CredentialsError) -> GetObjectAclError {
        GetObjectAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectAclError {
    fn from(err: HttpDispatchError) -> GetObjectAclError {
        GetObjectAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectAclError {
    fn from(err: io::Error) -> GetObjectAclError {
        GetObjectAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectAclError {
    fn description(&self) -> &str {
        match *self {
            GetObjectAclError::NoSuchKey(ref cause) => cause,
            GetObjectAclError::Validation(ref cause) => cause,
            GetObjectAclError::Credentials(ref err) => err.description(),
            GetObjectAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectTagging
#[derive(Debug, PartialEq)]
pub enum GetObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetObjectTaggingError {
    pub fn from_body(body: &str) -> GetObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetObjectTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetObjectTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetObjectTaggingError {
    fn from(err: XmlParseError) -> GetObjectTaggingError {
        let XmlParseError(message) = err;
        GetObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectTaggingError {
    fn from(err: CredentialsError) -> GetObjectTaggingError {
        GetObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectTaggingError {
    fn from(err: HttpDispatchError) -> GetObjectTaggingError {
        GetObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectTaggingError {
    fn from(err: io::Error) -> GetObjectTaggingError {
        GetObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            GetObjectTaggingError::Validation(ref cause) => cause,
            GetObjectTaggingError::Credentials(ref err) => err.description(),
            GetObjectTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetObjectTorrent
#[derive(Debug, PartialEq)]
pub enum GetObjectTorrentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetObjectTorrentError {
    pub fn from_body(body: &str) -> GetObjectTorrentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetObjectTorrentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetObjectTorrentError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetObjectTorrentError {
    fn from(err: XmlParseError) -> GetObjectTorrentError {
        let XmlParseError(message) = err;
        GetObjectTorrentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetObjectTorrentError {
    fn from(err: CredentialsError) -> GetObjectTorrentError {
        GetObjectTorrentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetObjectTorrentError {
    fn from(err: HttpDispatchError) -> GetObjectTorrentError {
        GetObjectTorrentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetObjectTorrentError {
    fn from(err: io::Error) -> GetObjectTorrentError {
        GetObjectTorrentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetObjectTorrentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTorrentError {
    fn description(&self) -> &str {
        match *self {
            GetObjectTorrentError::Validation(ref cause) => cause,
            GetObjectTorrentError::Credentials(ref err) => err.description(),
            GetObjectTorrentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetObjectTorrentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by HeadBucket
#[derive(Debug, PartialEq)]
pub enum HeadBucketError {
    ///The specified bucket does not exist.
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl HeadBucketError {
    pub fn from_body(body: &str) -> HeadBucketError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        HeadBucketError::NoSuchBucket(String::from(parsed_error.message))
                    }
                    _ => HeadBucketError::Unknown(String::from(body)),
                }
            }
            Err(_) => HeadBucketError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for HeadBucketError {
    fn from(err: XmlParseError) -> HeadBucketError {
        let XmlParseError(message) = err;
        HeadBucketError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for HeadBucketError {
    fn from(err: CredentialsError) -> HeadBucketError {
        HeadBucketError::Credentials(err)
    }
}
impl From<HttpDispatchError> for HeadBucketError {
    fn from(err: HttpDispatchError) -> HeadBucketError {
        HeadBucketError::HttpDispatch(err)
    }
}
impl From<io::Error> for HeadBucketError {
    fn from(err: io::Error) -> HeadBucketError {
        HeadBucketError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for HeadBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for HeadBucketError {
    fn description(&self) -> &str {
        match *self {
            HeadBucketError::NoSuchBucket(ref cause) => cause,
            HeadBucketError::Validation(ref cause) => cause,
            HeadBucketError::Credentials(ref err) => err.description(),
            HeadBucketError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            HeadBucketError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by HeadObject
#[derive(Debug, PartialEq)]
pub enum HeadObjectError {
    ///The specified key does not exist.
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl HeadObjectError {
    pub fn from_body(body: &str) -> HeadObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchKey" => HeadObjectError::NoSuchKey(String::from(parsed_error.message)),
                    _ => HeadObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => HeadObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for HeadObjectError {
    fn from(err: XmlParseError) -> HeadObjectError {
        let XmlParseError(message) = err;
        HeadObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for HeadObjectError {
    fn from(err: CredentialsError) -> HeadObjectError {
        HeadObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for HeadObjectError {
    fn from(err: HttpDispatchError) -> HeadObjectError {
        HeadObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for HeadObjectError {
    fn from(err: io::Error) -> HeadObjectError {
        HeadObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for HeadObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for HeadObjectError {
    fn description(&self) -> &str {
        match *self {
            HeadObjectError::NoSuchKey(ref cause) => cause,
            HeadObjectError::Validation(ref cause) => cause,
            HeadObjectError::Credentials(ref err) => err.description(),
            HeadObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            HeadObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketAnalyticsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketAnalyticsConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBucketAnalyticsConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketAnalyticsConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListBucketAnalyticsConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBucketAnalyticsConfigurationsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketAnalyticsConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketAnalyticsConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketAnalyticsConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketAnalyticsConfigurationsError {
    fn from(err: io::Error) -> ListBucketAnalyticsConfigurationsError {
        ListBucketAnalyticsConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketAnalyticsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketAnalyticsConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketAnalyticsConfigurationsError::Validation(ref cause) => cause,
            ListBucketAnalyticsConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketAnalyticsConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketAnalyticsConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketInventoryConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketInventoryConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBucketInventoryConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketInventoryConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListBucketInventoryConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBucketInventoryConfigurationsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListBucketInventoryConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketInventoryConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketInventoryConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketInventoryConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketInventoryConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketInventoryConfigurationsError {
    fn from(err: io::Error) -> ListBucketInventoryConfigurationsError {
        ListBucketInventoryConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketInventoryConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketInventoryConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketInventoryConfigurationsError::Validation(ref cause) => cause,
            ListBucketInventoryConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketInventoryConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketInventoryConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBucketMetricsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketMetricsConfigurationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBucketMetricsConfigurationsError {
    pub fn from_body(body: &str) -> ListBucketMetricsConfigurationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListBucketMetricsConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBucketMetricsConfigurationsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListBucketMetricsConfigurationsError {
    fn from(err: XmlParseError) -> ListBucketMetricsConfigurationsError {
        let XmlParseError(message) = err;
        ListBucketMetricsConfigurationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketMetricsConfigurationsError {
    fn from(err: CredentialsError) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketMetricsConfigurationsError {
    fn from(err: HttpDispatchError) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketMetricsConfigurationsError {
    fn from(err: io::Error) -> ListBucketMetricsConfigurationsError {
        ListBucketMetricsConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketMetricsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketMetricsConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketMetricsConfigurationsError::Validation(ref cause) => cause,
            ListBucketMetricsConfigurationsError::Credentials(ref err) => err.description(),
            ListBucketMetricsConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBucketMetricsConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuckets
#[derive(Debug, PartialEq)]
pub enum ListBucketsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBucketsError {
    pub fn from_body(body: &str) -> ListBucketsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListBucketsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBucketsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListBucketsError {
    fn from(err: XmlParseError) -> ListBucketsError {
        let XmlParseError(message) = err;
        ListBucketsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListBucketsError {
    fn from(err: CredentialsError) -> ListBucketsError {
        ListBucketsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBucketsError {
    fn from(err: HttpDispatchError) -> ListBucketsError {
        ListBucketsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBucketsError {
    fn from(err: io::Error) -> ListBucketsError {
        ListBucketsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBucketsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketsError {
    fn description(&self) -> &str {
        match *self {
            ListBucketsError::Validation(ref cause) => cause,
            ListBucketsError::Credentials(ref err) => err.description(),
            ListBucketsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBucketsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMultipartUploads
#[derive(Debug, PartialEq)]
pub enum ListMultipartUploadsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListMultipartUploadsError {
    pub fn from_body(body: &str) -> ListMultipartUploadsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListMultipartUploadsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListMultipartUploadsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListMultipartUploadsError {
    fn from(err: XmlParseError) -> ListMultipartUploadsError {
        let XmlParseError(message) = err;
        ListMultipartUploadsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListMultipartUploadsError {
    fn from(err: CredentialsError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMultipartUploadsError {
    fn from(err: HttpDispatchError) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMultipartUploadsError {
    fn from(err: io::Error) -> ListMultipartUploadsError {
        ListMultipartUploadsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMultipartUploadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMultipartUploadsError {
    fn description(&self) -> &str {
        match *self {
            ListMultipartUploadsError::Validation(ref cause) => cause,
            ListMultipartUploadsError::Credentials(ref err) => err.description(),
            ListMultipartUploadsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListMultipartUploadsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectVersions
#[derive(Debug, PartialEq)]
pub enum ListObjectVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectVersionsError {
    pub fn from_body(body: &str) -> ListObjectVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListObjectVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectVersionsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListObjectVersionsError {
    fn from(err: XmlParseError) -> ListObjectVersionsError {
        let XmlParseError(message) = err;
        ListObjectVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectVersionsError {
    fn from(err: CredentialsError) -> ListObjectVersionsError {
        ListObjectVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectVersionsError {
    fn from(err: HttpDispatchError) -> ListObjectVersionsError {
        ListObjectVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectVersionsError {
    fn from(err: io::Error) -> ListObjectVersionsError {
        ListObjectVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectVersionsError::Validation(ref cause) => cause,
            ListObjectVersionsError::Credentials(ref err) => err.description(),
            ListObjectVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListObjectVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjects
#[derive(Debug, PartialEq)]
pub enum ListObjectsError {
    ///The specified bucket does not exist.
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectsError {
    pub fn from_body(body: &str) -> ListObjectsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        ListObjectsError::NoSuchBucket(String::from(parsed_error.message))
                    }
                    _ => ListObjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListObjectsError {
    fn from(err: XmlParseError) -> ListObjectsError {
        let XmlParseError(message) = err;
        ListObjectsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectsError {
    fn from(err: CredentialsError) -> ListObjectsError {
        ListObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectsError {
    fn from(err: HttpDispatchError) -> ListObjectsError {
        ListObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectsError {
    fn from(err: io::Error) -> ListObjectsError {
        ListObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectsError {
    fn description(&self) -> &str {
        match *self {
            ListObjectsError::NoSuchBucket(ref cause) => cause,
            ListObjectsError::Validation(ref cause) => cause,
            ListObjectsError::Credentials(ref err) => err.description(),
            ListObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListObjectsV2
#[derive(Debug, PartialEq)]
pub enum ListObjectsV2Error {
    ///The specified bucket does not exist.
    NoSuchBucket(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListObjectsV2Error {
    pub fn from_body(body: &str) -> ListObjectsV2Error {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        ListObjectsV2Error::NoSuchBucket(String::from(parsed_error.message))
                    }
                    _ => ListObjectsV2Error::Unknown(String::from(body)),
                }
            }
            Err(_) => ListObjectsV2Error::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListObjectsV2Error {
    fn from(err: XmlParseError) -> ListObjectsV2Error {
        let XmlParseError(message) = err;
        ListObjectsV2Error::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListObjectsV2Error {
    fn from(err: CredentialsError) -> ListObjectsV2Error {
        ListObjectsV2Error::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListObjectsV2Error {
    fn from(err: HttpDispatchError) -> ListObjectsV2Error {
        ListObjectsV2Error::HttpDispatch(err)
    }
}
impl From<io::Error> for ListObjectsV2Error {
    fn from(err: io::Error) -> ListObjectsV2Error {
        ListObjectsV2Error::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListObjectsV2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectsV2Error {
    fn description(&self) -> &str {
        match *self {
            ListObjectsV2Error::NoSuchBucket(ref cause) => cause,
            ListObjectsV2Error::Validation(ref cause) => cause,
            ListObjectsV2Error::Credentials(ref err) => err.description(),
            ListObjectsV2Error::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListObjectsV2Error::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListParts
#[derive(Debug, PartialEq)]
pub enum ListPartsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPartsError {
    pub fn from_body(body: &str) -> ListPartsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListPartsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPartsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListPartsError {
    fn from(err: XmlParseError) -> ListPartsError {
        let XmlParseError(message) = err;
        ListPartsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListPartsError {
    fn from(err: CredentialsError) -> ListPartsError {
        ListPartsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPartsError {
    fn from(err: HttpDispatchError) -> ListPartsError {
        ListPartsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPartsError {
    fn from(err: io::Error) -> ListPartsError {
        ListPartsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPartsError {
    fn description(&self) -> &str {
        match *self {
            ListPartsError::Validation(ref cause) => cause,
            ListPartsError::Credentials(ref err) => err.description(),
            ListPartsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPartsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAccelerateConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketAccelerateConfigurationError {
    pub fn from_body(body: &str) -> PutBucketAccelerateConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketAccelerateConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketAccelerateConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketAccelerateConfigurationError {
    fn from(err: XmlParseError) -> PutBucketAccelerateConfigurationError {
        let XmlParseError(message) = err;
        PutBucketAccelerateConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAccelerateConfigurationError {
    fn from(err: CredentialsError) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAccelerateConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAccelerateConfigurationError {
    fn from(err: io::Error) -> PutBucketAccelerateConfigurationError {
        PutBucketAccelerateConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAccelerateConfigurationError::Validation(ref cause) => cause,
            PutBucketAccelerateConfigurationError::Credentials(ref err) => err.description(),
            PutBucketAccelerateConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketAccelerateConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAcl
#[derive(Debug, PartialEq)]
pub enum PutBucketAclError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketAclError {
    pub fn from_body(body: &str) -> PutBucketAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketAclError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketAclError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketAclError {
    fn from(err: XmlParseError) -> PutBucketAclError {
        let XmlParseError(message) = err;
        PutBucketAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAclError {
    fn from(err: CredentialsError) -> PutBucketAclError {
        PutBucketAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAclError {
    fn from(err: HttpDispatchError) -> PutBucketAclError {
        PutBucketAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAclError {
    fn from(err: io::Error) -> PutBucketAclError {
        PutBucketAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAclError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAclError::Validation(ref cause) => cause,
            PutBucketAclError::Credentials(ref err) => err.description(),
            PutBucketAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAnalyticsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketAnalyticsConfigurationError {
    pub fn from_body(body: &str) -> PutBucketAnalyticsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketAnalyticsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketAnalyticsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketAnalyticsConfigurationError {
    fn from(err: XmlParseError) -> PutBucketAnalyticsConfigurationError {
        let XmlParseError(message) = err;
        PutBucketAnalyticsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketAnalyticsConfigurationError {
    fn from(err: CredentialsError) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketAnalyticsConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketAnalyticsConfigurationError {
    fn from(err: io::Error) -> PutBucketAnalyticsConfigurationError {
        PutBucketAnalyticsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketAnalyticsConfigurationError::Validation(ref cause) => cause,
            PutBucketAnalyticsConfigurationError::Credentials(ref err) => err.description(),
            PutBucketAnalyticsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketAnalyticsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketCors
#[derive(Debug, PartialEq)]
pub enum PutBucketCorsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketCorsError {
    pub fn from_body(body: &str) -> PutBucketCorsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketCorsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketCorsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketCorsError {
    fn from(err: XmlParseError) -> PutBucketCorsError {
        let XmlParseError(message) = err;
        PutBucketCorsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketCorsError {
    fn from(err: CredentialsError) -> PutBucketCorsError {
        PutBucketCorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketCorsError {
    fn from(err: HttpDispatchError) -> PutBucketCorsError {
        PutBucketCorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketCorsError {
    fn from(err: io::Error) -> PutBucketCorsError {
        PutBucketCorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketCorsError {
    fn description(&self) -> &str {
        match *self {
            PutBucketCorsError::Validation(ref cause) => cause,
            PutBucketCorsError::Credentials(ref err) => err.description(),
            PutBucketCorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketCorsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketInventoryConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketInventoryConfigurationError {
    pub fn from_body(body: &str) -> PutBucketInventoryConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketInventoryConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketInventoryConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketInventoryConfigurationError {
    fn from(err: XmlParseError) -> PutBucketInventoryConfigurationError {
        let XmlParseError(message) = err;
        PutBucketInventoryConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketInventoryConfigurationError {
    fn from(err: CredentialsError) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketInventoryConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketInventoryConfigurationError {
    fn from(err: io::Error) -> PutBucketInventoryConfigurationError {
        PutBucketInventoryConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketInventoryConfigurationError::Validation(ref cause) => cause,
            PutBucketInventoryConfigurationError::Credentials(ref err) => err.description(),
            PutBucketInventoryConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketInventoryConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketLifecycleError {
    pub fn from_body(body: &str) -> PutBucketLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketLifecycleError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketLifecycleError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketLifecycleError {
    fn from(err: XmlParseError) -> PutBucketLifecycleError {
        let XmlParseError(message) = err;
        PutBucketLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLifecycleError {
    fn from(err: CredentialsError) -> PutBucketLifecycleError {
        PutBucketLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLifecycleError {
    fn from(err: HttpDispatchError) -> PutBucketLifecycleError {
        PutBucketLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLifecycleError {
    fn from(err: io::Error) -> PutBucketLifecycleError {
        PutBucketLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLifecycleError::Validation(ref cause) => cause,
            PutBucketLifecycleError::Credentials(ref err) => err.description(),
            PutBucketLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketLifecycleConfigurationError {
    pub fn from_body(body: &str) -> PutBucketLifecycleConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketLifecycleConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketLifecycleConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketLifecycleConfigurationError {
    fn from(err: XmlParseError) -> PutBucketLifecycleConfigurationError {
        let XmlParseError(message) = err;
        PutBucketLifecycleConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLifecycleConfigurationError {
    fn from(err: CredentialsError) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLifecycleConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLifecycleConfigurationError {
    fn from(err: io::Error) -> PutBucketLifecycleConfigurationError {
        PutBucketLifecycleConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLifecycleConfigurationError::Validation(ref cause) => cause,
            PutBucketLifecycleConfigurationError::Credentials(ref err) => err.description(),
            PutBucketLifecycleConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketLifecycleConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketLogging
#[derive(Debug, PartialEq)]
pub enum PutBucketLoggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketLoggingError {
    pub fn from_body(body: &str) -> PutBucketLoggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketLoggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketLoggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketLoggingError {
    fn from(err: XmlParseError) -> PutBucketLoggingError {
        let XmlParseError(message) = err;
        PutBucketLoggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketLoggingError {
    fn from(err: CredentialsError) -> PutBucketLoggingError {
        PutBucketLoggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketLoggingError {
    fn from(err: HttpDispatchError) -> PutBucketLoggingError {
        PutBucketLoggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketLoggingError {
    fn from(err: io::Error) -> PutBucketLoggingError {
        PutBucketLoggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLoggingError {
    fn description(&self) -> &str {
        match *self {
            PutBucketLoggingError::Validation(ref cause) => cause,
            PutBucketLoggingError::Credentials(ref err) => err.description(),
            PutBucketLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketLoggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketMetricsConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketMetricsConfigurationError {
    pub fn from_body(body: &str) -> PutBucketMetricsConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketMetricsConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketMetricsConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketMetricsConfigurationError {
    fn from(err: XmlParseError) -> PutBucketMetricsConfigurationError {
        let XmlParseError(message) = err;
        PutBucketMetricsConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketMetricsConfigurationError {
    fn from(err: CredentialsError) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketMetricsConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketMetricsConfigurationError {
    fn from(err: io::Error) -> PutBucketMetricsConfigurationError {
        PutBucketMetricsConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketMetricsConfigurationError::Validation(ref cause) => cause,
            PutBucketMetricsConfigurationError::Credentials(ref err) => err.description(),
            PutBucketMetricsConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketMetricsConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketNotification
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketNotificationError {
    pub fn from_body(body: &str) -> PutBucketNotificationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketNotificationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketNotificationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketNotificationError {
    fn from(err: XmlParseError) -> PutBucketNotificationError {
        let XmlParseError(message) = err;
        PutBucketNotificationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketNotificationError {
    fn from(err: CredentialsError) -> PutBucketNotificationError {
        PutBucketNotificationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketNotificationError {
    fn from(err: HttpDispatchError) -> PutBucketNotificationError {
        PutBucketNotificationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketNotificationError {
    fn from(err: io::Error) -> PutBucketNotificationError {
        PutBucketNotificationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketNotificationError::Validation(ref cause) => cause,
            PutBucketNotificationError::Credentials(ref err) => err.description(),
            PutBucketNotificationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketNotificationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketNotificationConfigurationError {
    pub fn from_body(body: &str) -> PutBucketNotificationConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketNotificationConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketNotificationConfigurationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketNotificationConfigurationError {
    fn from(err: XmlParseError) -> PutBucketNotificationConfigurationError {
        let XmlParseError(message) = err;
        PutBucketNotificationConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketNotificationConfigurationError {
    fn from(err: CredentialsError) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketNotificationConfigurationError {
    fn from(err: HttpDispatchError) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketNotificationConfigurationError {
    fn from(err: io::Error) -> PutBucketNotificationConfigurationError {
        PutBucketNotificationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketNotificationConfigurationError::Validation(ref cause) => cause,
            PutBucketNotificationConfigurationError::Credentials(ref err) => err.description(),
            PutBucketNotificationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketNotificationConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketPolicy
#[derive(Debug, PartialEq)]
pub enum PutBucketPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketPolicyError {
    pub fn from_body(body: &str) -> PutBucketPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketPolicyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketPolicyError {
    fn from(err: XmlParseError) -> PutBucketPolicyError {
        let XmlParseError(message) = err;
        PutBucketPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketPolicyError {
    fn from(err: CredentialsError) -> PutBucketPolicyError {
        PutBucketPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketPolicyError {
    fn from(err: HttpDispatchError) -> PutBucketPolicyError {
        PutBucketPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketPolicyError {
    fn from(err: io::Error) -> PutBucketPolicyError {
        PutBucketPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutBucketPolicyError::Validation(ref cause) => cause,
            PutBucketPolicyError::Credentials(ref err) => err.description(),
            PutBucketPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketReplication
#[derive(Debug, PartialEq)]
pub enum PutBucketReplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketReplicationError {
    pub fn from_body(body: &str) -> PutBucketReplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketReplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketReplicationError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketReplicationError {
    fn from(err: XmlParseError) -> PutBucketReplicationError {
        let XmlParseError(message) = err;
        PutBucketReplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketReplicationError {
    fn from(err: CredentialsError) -> PutBucketReplicationError {
        PutBucketReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketReplicationError {
    fn from(err: HttpDispatchError) -> PutBucketReplicationError {
        PutBucketReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketReplicationError {
    fn from(err: io::Error) -> PutBucketReplicationError {
        PutBucketReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketReplicationError {
    fn description(&self) -> &str {
        match *self {
            PutBucketReplicationError::Validation(ref cause) => cause,
            PutBucketReplicationError::Credentials(ref err) => err.description(),
            PutBucketReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketReplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum PutBucketRequestPaymentError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketRequestPaymentError {
    pub fn from_body(body: &str) -> PutBucketRequestPaymentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketRequestPaymentError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketRequestPaymentError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketRequestPaymentError {
    fn from(err: XmlParseError) -> PutBucketRequestPaymentError {
        let XmlParseError(message) = err;
        PutBucketRequestPaymentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketRequestPaymentError {
    fn from(err: CredentialsError) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketRequestPaymentError {
    fn from(err: HttpDispatchError) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketRequestPaymentError {
    fn from(err: io::Error) -> PutBucketRequestPaymentError {
        PutBucketRequestPaymentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {
            PutBucketRequestPaymentError::Validation(ref cause) => cause,
            PutBucketRequestPaymentError::Credentials(ref err) => err.description(),
            PutBucketRequestPaymentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketRequestPaymentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketTagging
#[derive(Debug, PartialEq)]
pub enum PutBucketTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketTaggingError {
    pub fn from_body(body: &str) -> PutBucketTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketTaggingError {
    fn from(err: XmlParseError) -> PutBucketTaggingError {
        let XmlParseError(message) = err;
        PutBucketTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketTaggingError {
    fn from(err: CredentialsError) -> PutBucketTaggingError {
        PutBucketTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketTaggingError {
    fn from(err: HttpDispatchError) -> PutBucketTaggingError {
        PutBucketTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketTaggingError {
    fn from(err: io::Error) -> PutBucketTaggingError {
        PutBucketTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketTaggingError {
    fn description(&self) -> &str {
        match *self {
            PutBucketTaggingError::Validation(ref cause) => cause,
            PutBucketTaggingError::Credentials(ref err) => err.description(),
            PutBucketTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketVersioning
#[derive(Debug, PartialEq)]
pub enum PutBucketVersioningError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketVersioningError {
    pub fn from_body(body: &str) -> PutBucketVersioningError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketVersioningError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketVersioningError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketVersioningError {
    fn from(err: XmlParseError) -> PutBucketVersioningError {
        let XmlParseError(message) = err;
        PutBucketVersioningError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketVersioningError {
    fn from(err: CredentialsError) -> PutBucketVersioningError {
        PutBucketVersioningError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketVersioningError {
    fn from(err: HttpDispatchError) -> PutBucketVersioningError {
        PutBucketVersioningError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketVersioningError {
    fn from(err: io::Error) -> PutBucketVersioningError {
        PutBucketVersioningError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketVersioningError {
    fn description(&self) -> &str {
        match *self {
            PutBucketVersioningError::Validation(ref cause) => cause,
            PutBucketVersioningError::Credentials(ref err) => err.description(),
            PutBucketVersioningError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutBucketVersioningError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutBucketWebsite
#[derive(Debug, PartialEq)]
pub enum PutBucketWebsiteError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutBucketWebsiteError {
    pub fn from_body(body: &str) -> PutBucketWebsiteError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutBucketWebsiteError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutBucketWebsiteError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutBucketWebsiteError {
    fn from(err: XmlParseError) -> PutBucketWebsiteError {
        let XmlParseError(message) = err;
        PutBucketWebsiteError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutBucketWebsiteError {
    fn from(err: CredentialsError) -> PutBucketWebsiteError {
        PutBucketWebsiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBucketWebsiteError {
    fn from(err: HttpDispatchError) -> PutBucketWebsiteError {
        PutBucketWebsiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBucketWebsiteError {
    fn from(err: io::Error) -> PutBucketWebsiteError {
        PutBucketWebsiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {
            PutBucketWebsiteError::Validation(ref cause) => cause,
            PutBucketWebsiteError::Credentials(ref err) => err.description(),
            PutBucketWebsiteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBucketWebsiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObject
#[derive(Debug, PartialEq)]
pub enum PutObjectError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutObjectError {
    pub fn from_body(body: &str) -> PutObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutObjectError {
    fn from(err: XmlParseError) -> PutObjectError {
        let XmlParseError(message) = err;
        PutObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectError {
    fn from(err: CredentialsError) -> PutObjectError {
        PutObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectError {
    fn from(err: HttpDispatchError) -> PutObjectError {
        PutObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectError {
    fn from(err: io::Error) -> PutObjectError {
        PutObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectError {
    fn description(&self) -> &str {
        match *self {
            PutObjectError::Validation(ref cause) => cause,
            PutObjectError::Credentials(ref err) => err.description(),
            PutObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObjectAcl
#[derive(Debug, PartialEq)]
pub enum PutObjectAclError {
    ///The specified key does not exist.
    NoSuchKey(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutObjectAclError {
    pub fn from_body(body: &str) -> PutObjectAclError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "NoSuchKey" => PutObjectAclError::NoSuchKey(String::from(parsed_error.message)),
                    _ => PutObjectAclError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutObjectAclError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutObjectAclError {
    fn from(err: XmlParseError) -> PutObjectAclError {
        let XmlParseError(message) = err;
        PutObjectAclError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectAclError {
    fn from(err: CredentialsError) -> PutObjectAclError {
        PutObjectAclError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectAclError {
    fn from(err: HttpDispatchError) -> PutObjectAclError {
        PutObjectAclError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectAclError {
    fn from(err: io::Error) -> PutObjectAclError {
        PutObjectAclError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectAclError {
    fn description(&self) -> &str {
        match *self {
            PutObjectAclError::NoSuchKey(ref cause) => cause,
            PutObjectAclError::Validation(ref cause) => cause,
            PutObjectAclError::Credentials(ref err) => err.description(),
            PutObjectAclError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectAclError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutObjectTagging
#[derive(Debug, PartialEq)]
pub enum PutObjectTaggingError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutObjectTaggingError {
    pub fn from_body(body: &str) -> PutObjectTaggingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => PutObjectTaggingError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutObjectTaggingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PutObjectTaggingError {
    fn from(err: XmlParseError) -> PutObjectTaggingError {
        let XmlParseError(message) = err;
        PutObjectTaggingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PutObjectTaggingError {
    fn from(err: CredentialsError) -> PutObjectTaggingError {
        PutObjectTaggingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutObjectTaggingError {
    fn from(err: HttpDispatchError) -> PutObjectTaggingError {
        PutObjectTaggingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutObjectTaggingError {
    fn from(err: io::Error) -> PutObjectTaggingError {
        PutObjectTaggingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectTaggingError {
    fn description(&self) -> &str {
        match *self {
            PutObjectTaggingError::Validation(ref cause) => cause,
            PutObjectTaggingError::Credentials(ref err) => err.description(),
            PutObjectTaggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutObjectTaggingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreObject
#[derive(Debug, PartialEq)]
pub enum RestoreObjectError {
    ///This operation is not allowed against this storage tier
    ObjectAlreadyInActiveTierError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RestoreObjectError {
    pub fn from_body(body: &str) -> RestoreObjectError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ObjectAlreadyInActiveTierError" => RestoreObjectError::ObjectAlreadyInActiveTierError(String::from(parsed_error.message)),
                    _ => RestoreObjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreObjectError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for RestoreObjectError {
    fn from(err: XmlParseError) -> RestoreObjectError {
        let XmlParseError(message) = err;
        RestoreObjectError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RestoreObjectError {
    fn from(err: CredentialsError) -> RestoreObjectError {
        RestoreObjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreObjectError {
    fn from(err: HttpDispatchError) -> RestoreObjectError {
        RestoreObjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreObjectError {
    fn from(err: io::Error) -> RestoreObjectError {
        RestoreObjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreObjectError {
    fn description(&self) -> &str {
        match *self {
            RestoreObjectError::ObjectAlreadyInActiveTierError(ref cause) => cause,
            RestoreObjectError::Validation(ref cause) => cause,
            RestoreObjectError::Credentials(ref err) => err.description(),
            RestoreObjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RestoreObjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadPart
#[derive(Debug, PartialEq)]
pub enum UploadPartError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UploadPartError {
    pub fn from_body(body: &str) -> UploadPartError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => UploadPartError::Unknown(String::from(body)),
                }
            }
            Err(_) => UploadPartError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for UploadPartError {
    fn from(err: XmlParseError) -> UploadPartError {
        let XmlParseError(message) = err;
        UploadPartError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UploadPartError {
    fn from(err: CredentialsError) -> UploadPartError {
        UploadPartError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadPartError {
    fn from(err: HttpDispatchError) -> UploadPartError {
        UploadPartError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadPartError {
    fn from(err: io::Error) -> UploadPartError {
        UploadPartError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartError {
    fn description(&self) -> &str {
        match *self {
            UploadPartError::Validation(ref cause) => cause,
            UploadPartError::Credentials(ref err) => err.description(),
            UploadPartError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadPartError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UploadPartCopy
#[derive(Debug, PartialEq)]
pub enum UploadPartCopyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UploadPartCopyError {
    pub fn from_body(body: &str) -> UploadPartCopyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => UploadPartCopyError::Unknown(String::from(body)),
                }
            }
            Err(_) => UploadPartCopyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for UploadPartCopyError {
    fn from(err: XmlParseError) -> UploadPartCopyError {
        let XmlParseError(message) = err;
        UploadPartCopyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UploadPartCopyError {
    fn from(err: CredentialsError) -> UploadPartCopyError {
        UploadPartCopyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UploadPartCopyError {
    fn from(err: HttpDispatchError) -> UploadPartCopyError {
        UploadPartCopyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UploadPartCopyError {
    fn from(err: io::Error) -> UploadPartCopyError {
        UploadPartCopyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UploadPartCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartCopyError {
    fn description(&self) -> &str {
        match *self {
            UploadPartCopyError::Validation(ref cause) => cause,
            UploadPartCopyError::Credentials(ref err) => err.description(),
            UploadPartCopyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UploadPartCopyError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon S3 API. Amazon S3 clients implement this trait.
pub trait S3 {
    #[doc="<p>Aborts a multipart upload.</p><p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>"]
    fn abort_multipart_upload(&self,
                              input: &AbortMultipartUploadRequest)
                              -> Result<AbortMultipartUploadOutput, AbortMultipartUploadError>;


    #[doc="Completes a multipart upload by assembling previously uploaded parts."]
    fn complete_multipart_upload
        (&self,
         input: &CompleteMultipartUploadRequest)
         -> Result<CompleteMultipartUploadOutput, CompleteMultipartUploadError>;


    #[doc="Creates a copy of an object that is already stored in Amazon S3."]
    fn copy_object(&self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, CopyObjectError>;


    #[doc="Creates a new bucket."]
    fn create_bucket(&self,
                     input: &CreateBucketRequest)
                     -> Result<CreateBucketOutput, CreateBucketError>;


    #[doc="<p>Initiates a multipart upload and returns an upload ID.</p><p><b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>"]
    fn create_multipart_upload
        (&self,
         input: &CreateMultipartUploadRequest)
         -> Result<CreateMultipartUploadOutput, CreateMultipartUploadError>;


    #[doc="Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted."]
    fn delete_bucket(&self, input: &DeleteBucketRequest) -> Result<(), DeleteBucketError>;


    #[doc="Deletes an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    fn delete_bucket_analytics_configuration
        (&self,
         input: &DeleteBucketAnalyticsConfigurationRequest)
         -> Result<(), DeleteBucketAnalyticsConfigurationError>;


    #[doc="Deletes the cors configuration information set for the bucket."]
    fn delete_bucket_cors(&self,
                          input: &DeleteBucketCorsRequest)
                          -> Result<(), DeleteBucketCorsError>;


    #[doc="Deletes an inventory configuration (identified by the inventory ID) from the bucket."]
    fn delete_bucket_inventory_configuration
        (&self,
         input: &DeleteBucketInventoryConfigurationRequest)
         -> Result<(), DeleteBucketInventoryConfigurationError>;


    #[doc="Deletes the lifecycle configuration from the bucket."]
    fn delete_bucket_lifecycle(&self,
                               input: &DeleteBucketLifecycleRequest)
                               -> Result<(), DeleteBucketLifecycleError>;


    #[doc="Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket."]
    fn delete_bucket_metrics_configuration(&self,
                                           input: &DeleteBucketMetricsConfigurationRequest)
                                           -> Result<(), DeleteBucketMetricsConfigurationError>;


    #[doc="Deletes the policy from the bucket."]
    fn delete_bucket_policy(&self,
                            input: &DeleteBucketPolicyRequest)
                            -> Result<(), DeleteBucketPolicyError>;


    #[doc="Deletes the replication configuration from the bucket."]
    fn delete_bucket_replication(&self,
                                 input: &DeleteBucketReplicationRequest)
                                 -> Result<(), DeleteBucketReplicationError>;


    #[doc="Deletes the tags from the bucket."]
    fn delete_bucket_tagging(&self,
                             input: &DeleteBucketTaggingRequest)
                             -> Result<(), DeleteBucketTaggingError>;


    #[doc="This operation removes the website configuration from the bucket."]
    fn delete_bucket_website(&self,
                             input: &DeleteBucketWebsiteRequest)
                             -> Result<(), DeleteBucketWebsiteError>;


    #[doc="Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects."]
    fn delete_object(&self,
                     input: &DeleteObjectRequest)
                     -> Result<DeleteObjectOutput, DeleteObjectError>;


    #[doc="Removes the tag-set from an existing object."]
    fn delete_object_tagging(&self,
                             input: &DeleteObjectTaggingRequest)
                             -> Result<DeleteObjectTaggingOutput, DeleteObjectTaggingError>;


    #[doc="This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys."]
    fn delete_objects(&self,
                      input: &DeleteObjectsRequest)
                      -> Result<DeleteObjectsOutput, DeleteObjectsError>;


    #[doc="Returns the accelerate configuration of a bucket."]
    fn get_bucket_accelerate_configuration
        (&self,
         input: &GetBucketAccelerateConfigurationRequest)
         -> Result<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>;


    #[doc="Gets the access control policy for the bucket."]
    fn get_bucket_acl(&self,
                      input: &GetBucketAclRequest)
                      -> Result<GetBucketAclOutput, GetBucketAclError>;


    #[doc="Gets an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    fn get_bucket_analytics_configuration
        (&self,
         input: &GetBucketAnalyticsConfigurationRequest)
         -> Result<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>;


    #[doc="Returns the cors configuration for the bucket."]
    fn get_bucket_cors(&self,
                       input: &GetBucketCorsRequest)
                       -> Result<GetBucketCorsOutput, GetBucketCorsError>;


    #[doc="Returns an inventory configuration (identified by the inventory ID) from the bucket."]
    fn get_bucket_inventory_configuration
        (&self,
         input: &GetBucketInventoryConfigurationRequest)
         -> Result<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>;


    #[doc="Deprecated, see the GetBucketLifecycleConfiguration operation."]
    fn get_bucket_lifecycle(&self,
                            input: &GetBucketLifecycleRequest)
                            -> Result<GetBucketLifecycleOutput, GetBucketLifecycleError>;


    #[doc="Returns the lifecycle configuration information set on the bucket."]
    fn get_bucket_lifecycle_configuration
        (&self,
         input: &GetBucketLifecycleConfigurationRequest)
         -> Result<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>;


    #[doc="Returns the region the bucket resides in."]
    fn get_bucket_location(&self,
                           input: &GetBucketLocationRequest)
                           -> Result<GetBucketLocationOutput, GetBucketLocationError>;


    #[doc="Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner."]
    fn get_bucket_logging(&self,
                          input: &GetBucketLoggingRequest)
                          -> Result<GetBucketLoggingOutput, GetBucketLoggingError>;


    #[doc="Gets a metrics configuration (specified by the metrics configuration ID) from the bucket."]
    fn get_bucket_metrics_configuration
        (&self,
         input: &GetBucketMetricsConfigurationRequest)
         -> Result<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError>;


    #[doc="Deprecated, see the GetBucketNotificationConfiguration operation."]
    fn get_bucket_notification
        (&self,
         input: &GetBucketNotificationConfigurationRequest)
         -> Result<NotificationConfigurationDeprecated, GetBucketNotificationError>;


    #[doc="Returns the notification configuration of a bucket."]
    fn get_bucket_notification_configuration
        (&self,
         input: &GetBucketNotificationConfigurationRequest)
         -> Result<NotificationConfiguration, GetBucketNotificationConfigurationError>;


    #[doc="Returns the policy of a specified bucket."]
    fn get_bucket_policy(&self,
                         input: &GetBucketPolicyRequest)
                         -> Result<GetBucketPolicyOutput, GetBucketPolicyError>;


    #[doc="Returns the replication configuration of a bucket."]
    fn get_bucket_replication(&self,
                              input: &GetBucketReplicationRequest)
                              -> Result<GetBucketReplicationOutput, GetBucketReplicationError>;


    #[doc="Returns the request payment configuration of a bucket."]
    fn get_bucket_request_payment
        (&self,
         input: &GetBucketRequestPaymentRequest)
         -> Result<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError>;


    #[doc="Returns the tag set associated with the bucket."]
    fn get_bucket_tagging(&self,
                          input: &GetBucketTaggingRequest)
                          -> Result<GetBucketTaggingOutput, GetBucketTaggingError>;


    #[doc="Returns the versioning state of a bucket."]
    fn get_bucket_versioning(&self,
                             input: &GetBucketVersioningRequest)
                             -> Result<GetBucketVersioningOutput, GetBucketVersioningError>;


    #[doc="Returns the website configuration for a bucket."]
    fn get_bucket_website(&self,
                          input: &GetBucketWebsiteRequest)
                          -> Result<GetBucketWebsiteOutput, GetBucketWebsiteError>;


    #[doc="Retrieves objects from Amazon S3."]
    fn get_object(&self, input: &GetObjectRequest) -> Result<GetObjectOutput, GetObjectError>;


    #[doc="Returns the access control list (ACL) of an object."]
    fn get_object_acl(&self,
                      input: &GetObjectAclRequest)
                      -> Result<GetObjectAclOutput, GetObjectAclError>;


    #[doc="Returns the tag-set of an object."]
    fn get_object_tagging(&self,
                          input: &GetObjectTaggingRequest)
                          -> Result<GetObjectTaggingOutput, GetObjectTaggingError>;


    #[doc="Return torrent files from a bucket."]
    fn get_object_torrent(&self,
                          input: &GetObjectTorrentRequest)
                          -> Result<GetObjectTorrentOutput, GetObjectTorrentError>;


    #[doc="This operation is useful to determine if a bucket exists and you have permission to access it."]
    fn head_bucket(&self, input: &HeadBucketRequest) -> Result<(), HeadBucketError>;


    #[doc="The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object."]
    fn head_object(&self, input: &HeadObjectRequest) -> Result<HeadObjectOutput, HeadObjectError>;


    #[doc="Lists the analytics configurations for the bucket."]
    fn list_bucket_analytics_configurations
        (&self,
         input: &ListBucketAnalyticsConfigurationsRequest)
         -> Result<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>;


    #[doc="Returns a list of inventory configurations for the bucket."]
    fn list_bucket_inventory_configurations
        (&self,
         input: &ListBucketInventoryConfigurationsRequest)
         -> Result<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>;


    #[doc="Lists the metrics configurations for the bucket."]
    fn list_bucket_metrics_configurations
        (&self,
         input: &ListBucketMetricsConfigurationsRequest)
         -> Result<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>;


    #[doc="Returns a list of all buckets owned by the authenticated sender of the request."]
    fn list_buckets(&self) -> Result<ListBucketsOutput, ListBucketsError>;


    #[doc="This operation lists in-progress multipart uploads."]
    fn list_multipart_uploads(&self,
                              input: &ListMultipartUploadsRequest)
                              -> Result<ListMultipartUploadsOutput, ListMultipartUploadsError>;


    #[doc="Returns metadata about all of the versions of objects in a bucket."]
    fn list_object_versions(&self,
                            input: &ListObjectVersionsRequest)
                            -> Result<ListObjectVersionsOutput, ListObjectVersionsError>;


    #[doc="Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket."]
    fn list_objects(&self,
                    input: &ListObjectsRequest)
                    -> Result<ListObjectsOutput, ListObjectsError>;


    #[doc="Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development."]
    fn list_objects_v2(&self,
                       input: &ListObjectsV2Request)
                       -> Result<ListObjectsV2Output, ListObjectsV2Error>;


    #[doc="Lists the parts that have been uploaded for a specific multipart upload."]
    fn list_parts(&self, input: &ListPartsRequest) -> Result<ListPartsOutput, ListPartsError>;


    #[doc="Sets the accelerate configuration of an existing bucket."]
    fn put_bucket_accelerate_configuration(&self,
                                           input: &PutBucketAccelerateConfigurationRequest)
                                           -> Result<(), PutBucketAccelerateConfigurationError>;


    #[doc="Sets the permissions on a bucket using access control lists (ACL)."]
    fn put_bucket_acl(&self, input: &PutBucketAclRequest) -> Result<(), PutBucketAclError>;


    #[doc="Sets an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    fn put_bucket_analytics_configuration(&self,
                                          input: &PutBucketAnalyticsConfigurationRequest)
                                          -> Result<(), PutBucketAnalyticsConfigurationError>;


    #[doc="Sets the cors configuration for a bucket."]
    fn put_bucket_cors(&self, input: &PutBucketCorsRequest) -> Result<(), PutBucketCorsError>;


    #[doc="Adds an inventory configuration (identified by the inventory ID) from the bucket."]
    fn put_bucket_inventory_configuration(&self,
                                          input: &PutBucketInventoryConfigurationRequest)
                                          -> Result<(), PutBucketInventoryConfigurationError>;


    #[doc="Deprecated, see the PutBucketLifecycleConfiguration operation."]
    fn put_bucket_lifecycle(&self,
                            input: &PutBucketLifecycleRequest)
                            -> Result<(), PutBucketLifecycleError>;


    #[doc="Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it."]
    fn put_bucket_lifecycle_configuration(&self,
                                          input: &PutBucketLifecycleConfigurationRequest)
                                          -> Result<(), PutBucketLifecycleConfigurationError>;


    #[doc="Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner."]
    fn put_bucket_logging(&self,
                          input: &PutBucketLoggingRequest)
                          -> Result<(), PutBucketLoggingError>;


    #[doc="Sets a metrics configuration (specified by the metrics configuration ID) for the bucket."]
    fn put_bucket_metrics_configuration(&self,
                                        input: &PutBucketMetricsConfigurationRequest)
                                        -> Result<(), PutBucketMetricsConfigurationError>;


    #[doc="Deprecated, see the PutBucketNotificationConfiguraiton operation."]
    fn put_bucket_notification(&self,
                               input: &PutBucketNotificationRequest)
                               -> Result<(), PutBucketNotificationError>;


    #[doc="Enables notifications of specified events for a bucket."]
    fn put_bucket_notification_configuration
        (&self,
         input: &PutBucketNotificationConfigurationRequest)
         -> Result<(), PutBucketNotificationConfigurationError>;


    #[doc="Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it."]
    fn put_bucket_policy(&self,
                         input: &PutBucketPolicyRequest)
                         -> Result<(), PutBucketPolicyError>;


    #[doc="Creates a new replication configuration (or replaces an existing one, if present)."]
    fn put_bucket_replication(&self,
                              input: &PutBucketReplicationRequest)
                              -> Result<(), PutBucketReplicationError>;


    #[doc="Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html"]
    fn put_bucket_request_payment(&self,
                                  input: &PutBucketRequestPaymentRequest)
                                  -> Result<(), PutBucketRequestPaymentError>;


    #[doc="Sets the tags for a bucket."]
    fn put_bucket_tagging(&self,
                          input: &PutBucketTaggingRequest)
                          -> Result<(), PutBucketTaggingError>;


    #[doc="Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner."]
    fn put_bucket_versioning(&self,
                             input: &PutBucketVersioningRequest)
                             -> Result<(), PutBucketVersioningError>;


    #[doc="Set the website configuration for a bucket."]
    fn put_bucket_website(&self,
                          input: &PutBucketWebsiteRequest)
                          -> Result<(), PutBucketWebsiteError>;


    #[doc="Adds an object to a bucket."]
    fn put_object(&self, input: &PutObjectRequest) -> Result<PutObjectOutput, PutObjectError>;


    #[doc="uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket"]
    fn put_object_acl(&self,
                      input: &PutObjectAclRequest)
                      -> Result<PutObjectAclOutput, PutObjectAclError>;


    #[doc="Sets the supplied tag-set to an object that already exists in a bucket"]
    fn put_object_tagging(&self,
                          input: &PutObjectTaggingRequest)
                          -> Result<PutObjectTaggingOutput, PutObjectTaggingError>;


    #[doc="Restores an archived copy of an object back into Amazon S3"]
    fn restore_object(&self,
                      input: &RestoreObjectRequest)
                      -> Result<RestoreObjectOutput, RestoreObjectError>;


    #[doc="<p>Uploads a part in a multipart upload.</p><p><b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>"]
    fn upload_part(&self, input: &UploadPartRequest) -> Result<UploadPartOutput, UploadPartError>;


    #[doc="Uploads a part by copying data from an existing object as data source."]
    fn upload_part_copy(&self,
                        input: &UploadPartCopyRequest)
                        -> Result<UploadPartCopyOutput, UploadPartCopyError>;
}
/// A client for the Amazon S3 API.
pub struct S3Client<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> S3Client<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        S3Client {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> S3 for S3Client<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Aborts a multipart upload.</p><p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>"]
    #[allow(unused_variables, warnings)]
    fn abort_multipart_upload(&self,
                              input: &AbortMultipartUploadRequest)
                              -> Result<AbortMultipartUploadOutput, AbortMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = AbortMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(AbortMultipartUploadOutputDeserializer::deserialize(&actual_tag_name,
                                                                                 &mut stack));
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AbortMultipartUploadError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Completes a multipart upload by assembling previously uploaded parts."]
    #[allow(unused_variables, warnings)]
    fn complete_multipart_upload
        (&self,
         input: &CompleteMultipartUploadRequest)
         -> Result<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.multipart_upload.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CompletedMultipartUploadSerializer::serialize(&mut writer,
                                                          "CompleteMultipartUpload",
                                                          input.multipart_upload.as_ref().unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CompleteMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CompleteMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CompleteMultipartUploadError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }

    #[doc="Creates a copy of an object that is already stored in Amazon S3."]
    #[allow(unused_variables, warnings)]
    fn copy_object(&self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, CopyObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }
        request.add_header("x-amz-copy-source", &input.copy_source);

        if let Some(ref copy_source_if_match) = input.copy_source_if_match {
            request.add_header("x-amz-copy-source-if-match",
                               &copy_source_if_match.to_string());
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header("x-amz-copy-source-if-modified-since",
                               &copy_source_if_modified_since.to_string());
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header("x-amz-copy-source-if-none-match",
                               &copy_source_if_none_match.to_string());
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header("x-amz-copy-source-if-unmodified-since",
                               &copy_source_if_unmodified_since.to_string());
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-algorithm",
                               &copy_source_sse_customer_algorithm.to_string());
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-key",
                               &copy_source_sse_customer_key.to_string());
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-key-MD5",
                               &copy_source_sse_customer_key_md5.to_string());
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata_directive) = input.metadata_directive {
            request.add_header("x-amz-metadata-directive", &metadata_directive.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header("x-amz-server-side-encryption-aws-kms-key-id",
                               &ssekms_key_id.to_string());
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header("x-amz-server-side-encryption",
                               &server_side_encryption.to_string());
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref tagging_directive) = input.tagging_directive {
            request.add_header("x-amz-tagging-directive", &tagging_directive.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header("x-amz-website-redirect-location",
                               &website_redirect_location.to_string());
        }



        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CopyObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CopyObjectOutputDeserializer::deserialize(&actual_tag_name,
                                                                            &mut stack));
                }
                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id") {
                    let value = copy_source_version_id.to_owned();
                    result.copy_source_version_id = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CopyObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Creates a new bucket."]
    #[allow(unused_variables, warnings)]
    fn create_bucket(&self,
                     input: &CreateBucketRequest)
                     -> Result<CreateBucketOutput, CreateBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        let mut payload: Vec<u8>;
        if input.create_bucket_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CreateBucketConfigurationSerializer::serialize(&mut writer,
                                                           "CreateBucketConfiguration",
                                                           input
                                                               .create_bucket_configuration
                                                               .as_ref()
                                                               .unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CreateBucketOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateBucketOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateBucketError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="<p>Initiates a multipart upload and returns an upload ID.</p><p><b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>"]
    #[allow(unused_variables, warnings)]
    fn create_multipart_upload
        (&self,
         input: &CreateMultipartUploadRequest)
         -> Result<CreateMultipartUploadOutput, CreateMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header("x-amz-server-side-encryption-aws-kms-key-id",
                               &ssekms_key_id.to_string());
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header("x-amz-server-side-encryption",
                               &server_side_encryption.to_string());
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header("x-amz-website-redirect-location",
                               &website_redirect_location.to_string());
        }
        let mut params = Params::new();
        params.put_key("uploads");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CreateMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(CreateMultipartUploadOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }
                if let Some(abort_date) = response.headers.get("x-amz-abort-date") {
                    let value = abort_date.to_owned();
                    result.abort_date = Some(value)
                };
                if let Some(abort_rule_id) = response.headers.get("x-amz-abort-rule-id") {
                    let value = abort_rule_id.to_owned();
                    result.abort_rule_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateMultipartUploadError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket(&self, input: &DeleteBucketRequest) -> Result<(), DeleteBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);





        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_analytics_configuration
        (&self,
         input: &DeleteBucketAnalyticsConfigurationRequest)
         -> Result<(), DeleteBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketAnalyticsConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes the cors configuration information set for the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_cors(&self,
                          input: &DeleteBucketCorsRequest)
                          -> Result<(), DeleteBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketCorsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes an inventory configuration (identified by the inventory ID) from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_inventory_configuration
        (&self,
         input: &DeleteBucketInventoryConfigurationRequest)
         -> Result<(), DeleteBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketInventoryConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes the lifecycle configuration from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_lifecycle(&self,
                               input: &DeleteBucketLifecycleRequest)
                               -> Result<(), DeleteBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketLifecycleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_metrics_configuration(&self,
                                           input: &DeleteBucketMetricsConfigurationRequest)
                                           -> Result<(), DeleteBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketMetricsConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes the policy from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_policy(&self,
                            input: &DeleteBucketPolicyRequest)
                            -> Result<(), DeleteBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Deletes the replication configuration from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_replication(&self,
                                 input: &DeleteBucketReplicationRequest)
                                 -> Result<(), DeleteBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketReplicationError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }

    #[doc="Deletes the tags from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_tagging(&self,
                             input: &DeleteBucketTaggingRequest)
                             -> Result<(), DeleteBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="This operation removes the website configuration from the bucket."]
    #[allow(unused_variables, warnings)]
    fn delete_bucket_website(&self,
                             input: &DeleteBucketWebsiteRequest)
                             -> Result<(), DeleteBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteBucketWebsiteError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects."]
    #[allow(unused_variables, warnings)]
    fn delete_object(&self,
                     input: &DeleteObjectRequest)
                     -> Result<DeleteObjectOutput, DeleteObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DeleteObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteObjectOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }
                if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                    let value = delete_marker.to_owned();
                    result.delete_marker = Some(value.parse::<bool>().unwrap())
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Removes the tag-set from an existing object."]
    #[allow(unused_variables, warnings)]
    fn delete_object_tagging(&self,
                             input: &DeleteObjectTaggingRequest)
                             -> Result<DeleteObjectTaggingOutput, DeleteObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DeleteObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(DeleteObjectTaggingOutputDeserializer::deserialize(&actual_tag_name,
                                                                                &mut stack));
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteObjectTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys."]
    #[allow(unused_variables, warnings)]
    fn delete_objects(&self,
                      input: &DeleteObjectsRequest)
                      -> Result<DeleteObjectsOutput, DeleteObjectsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);


        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("delete");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        DeleteSerializer::serialize(&mut writer, "Delete", &input.delete);
        payload = writer.into_inner();
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DeleteObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(DeleteObjectsOutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteObjectsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the accelerate configuration of a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_accelerate_configuration
        (&self,
         input: &GetBucketAccelerateConfigurationRequest)
         -> Result<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketAccelerateConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketAccelerateConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketAccelerateConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Gets the access control policy for the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_acl(&self,
                      input: &GetBucketAclRequest)
                      -> Result<GetBucketAclOutput, GetBucketAclError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketAclOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketAclError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Gets an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_analytics_configuration
        (&self,
         input: &GetBucketAnalyticsConfigurationRequest)
         -> Result<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketAnalyticsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketAnalyticsConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketAnalyticsConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Returns the cors configuration for the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_cors(&self,
                       input: &GetBucketCorsRequest)
                       -> Result<GetBucketCorsOutput, GetBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketCorsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketCorsOutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketCorsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns an inventory configuration (identified by the inventory ID) from the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_inventory_configuration
        (&self,
         input: &GetBucketInventoryConfigurationRequest)
         -> Result<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketInventoryConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketInventoryConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketInventoryConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Deprecated, see the GetBucketLifecycleConfiguration operation."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle(&self,
                            input: &GetBucketLifecycleRequest)
                            -> Result<GetBucketLifecycleOutput, GetBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketLifecycleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketLifecycleOutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketLifecycleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the lifecycle configuration information set on the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle_configuration
        (&self,
         input: &GetBucketLifecycleConfigurationRequest)
         -> Result<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketLifecycleConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketLifecycleConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketLifecycleConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Returns the region the bucket resides in."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_location(&self,
                           input: &GetBucketLocationRequest)
                           -> Result<GetBucketLocationOutput, GetBucketLocationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("location");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketLocationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketLocationOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketLocationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_logging(&self,
                          input: &GetBucketLoggingRequest)
                          -> Result<GetBucketLoggingOutput, GetBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketLoggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketLoggingOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketLoggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Gets a metrics configuration (specified by the metrics configuration ID) from the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_metrics_configuration
        (&self,
         input: &GetBucketMetricsConfigurationRequest)
         -> Result<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketMetricsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketMetricsConfigurationOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketMetricsConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                      .as_ref()))
            }
        }
    }

    #[doc="Deprecated, see the GetBucketNotificationConfiguration operation."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification
        (&self,
         input: &GetBucketNotificationConfigurationRequest)
         -> Result<NotificationConfigurationDeprecated, GetBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = NotificationConfigurationDeprecated::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(NotificationConfigurationDeprecatedDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketNotificationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the notification configuration of a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification_configuration
        (&self,
         input: &GetBucketNotificationConfigurationRequest)
         -> Result<NotificationConfiguration, GetBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = NotificationConfiguration::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(NotificationConfigurationDeserializer::deserialize(&actual_tag_name,
                                                                                &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketNotificationConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the policy of a specified bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_policy(&self,
                         input: &GetBucketPolicyRequest)
                         -> Result<GetBucketPolicyOutput, GetBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                let mut result = GetBucketPolicyOutput::default();
                result.policy = Some(String::from_utf8_lossy(&body).into());


                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the replication configuration of a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_replication(&self,
                              input: &GetBucketReplicationRequest)
                              -> Result<GetBucketReplicationOutput, GetBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketReplicationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketReplicationOutputDeserializer::deserialize(&actual_tag_name,
                                                                                 &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketReplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the request payment configuration of a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_request_payment
        (&self,
         input: &GetBucketRequestPaymentRequest)
         -> Result<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketRequestPaymentOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetBucketRequestPaymentOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketRequestPaymentError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }

    #[doc="Returns the tag set associated with the bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_tagging(&self,
                          input: &GetBucketTaggingRequest)
                          -> Result<GetBucketTaggingOutput, GetBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketTaggingOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the versioning state of a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_versioning(&self,
                             input: &GetBucketVersioningRequest)
                             -> Result<GetBucketVersioningOutput, GetBucketVersioningError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketVersioningOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketVersioningOutputDeserializer::deserialize(&actual_tag_name,
                                                                                &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketVersioningError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the website configuration for a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_bucket_website(&self,
                          input: &GetBucketWebsiteRequest)
                          -> Result<GetBucketWebsiteOutput, GetBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetBucketWebsiteOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetBucketWebsiteOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBucketWebsiteError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Retrieves objects from Amazon S3."]
    #[allow(unused_variables, warnings)]
    fn get_object(&self, input: &GetObjectRequest) -> Result<GetObjectOutput, GetObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = input.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = input.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = input.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = input.response_cache_control {
            params.put("response-cache-control", x);
        }
        if let Some(ref x) = input.response_content_disposition {
            params.put("response-content-disposition", x);
        }
        if let Some(ref x) = input.response_content_encoding {
            params.put("response-content-encoding", x);
        }
        if let Some(ref x) = input.response_content_language {
            params.put("response-content-language", x);
        }
        if let Some(ref x) = input.response_content_type {
            params.put("response-content-type", x);
        }
        if let Some(ref x) = input.response_expires {
            params.put("response-expires", x);
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {



                let mut result = GetObjectOutput::default();
                result.body = Some(StreamingBody(response.body));

                if let Some(accept_ranges) = response.headers.get("accept-ranges") {
                    let value = accept_ranges.to_owned();
                    result.accept_ranges = Some(value)
                };
                if let Some(cache_control) = response.headers.get("Cache-Control") {
                    let value = cache_control.to_owned();
                    result.cache_control = Some(value)
                };
                if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                    let value = content_disposition.to_owned();
                    result.content_disposition = Some(value)
                };
                if let Some(content_encoding) = response.headers.get("Content-Encoding") {
                    let value = content_encoding.to_owned();
                    result.content_encoding = Some(value)
                };
                if let Some(content_language) = response.headers.get("Content-Language") {
                    let value = content_language.to_owned();
                    result.content_language = Some(value)
                };
                if let Some(content_length) = response.headers.get("Content-Length") {
                    let value = content_length.to_owned();
                    result.content_length = Some(value.parse::<i64>().unwrap())
                };
                if let Some(content_range) = response.headers.get("Content-Range") {
                    let value = content_range.to_owned();
                    result.content_range = Some(value)
                };
                if let Some(content_type) = response.headers.get("Content-Type") {
                    let value = content_type.to_owned();
                    result.content_type = Some(value)
                };
                if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                    let value = delete_marker.to_owned();
                    result.delete_marker = Some(value.parse::<bool>().unwrap())
                };
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(expires) = response.headers.get("Expires") {
                    let value = expires.to_owned();
                    result.expires = Some(value)
                };
                if let Some(last_modified) = response.headers.get("Last-Modified") {
                    let value = last_modified.to_owned();
                    result.last_modified = Some(value)
                };
                let mut values = ::std::collections::HashMap::new();
                for (key, value) in response.headers.iter() {
                    if key.starts_with("x-amz-meta-") {
                        values.insert(key.replace("x-amz-meta-", ""), value.to_owned());
                    }
                }
                result.metadata = Some(values);
                if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                    let value = missing_meta.to_owned();
                    result.missing_meta = Some(value.parse::<i64>().unwrap())
                };
                if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                    let value = parts_count.to_owned();
                    result.parts_count = Some(value.parse::<i64>().unwrap())
                };
                if let Some(replication_status) =
                    response.headers.get("x-amz-replication-status") {
                    let value = replication_status.to_owned();
                    result.replication_status = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(restore) = response.headers.get("x-amz-restore") {
                    let value = restore.to_owned();
                    result.restore = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(storage_class) = response.headers.get("x-amz-storage-class") {
                    let value = storage_class.to_owned();
                    result.storage_class = Some(value)
                };
                if let Some(tag_count) = response.headers.get("x-amz-tagging-count") {
                    let value = tag_count.to_owned();
                    result.tag_count = Some(value.parse::<i64>().unwrap())
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                if let Some(website_redirect_location) =
                    response.headers.get("x-amz-website-redirect-location") {
                    let value = website_redirect_location.to_owned();
                    result.website_redirect_location = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the access control list (ACL) of an object."]
    #[allow(unused_variables, warnings)]
    fn get_object_acl(&self,
                      input: &GetObjectAclRequest)
                      -> Result<GetObjectAclOutput, GetObjectAclError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(GetObjectAclOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetObjectAclError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns the tag-set of an object."]
    #[allow(unused_variables, warnings)]
    fn get_object_tagging(&self,
                          input: &GetObjectTaggingRequest)
                          -> Result<GetObjectTaggingOutput, GetObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(GetObjectTaggingOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetObjectTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Return torrent files from a bucket."]
    #[allow(unused_variables, warnings)]
    fn get_object_torrent(&self,
                          input: &GetObjectTorrentRequest)
                          -> Result<GetObjectTorrentOutput, GetObjectTorrentError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("torrent");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {



                let mut result = GetObjectTorrentOutput::default();
                result.body = Some(StreamingBody(response.body));

                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetObjectTorrentError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="This operation is useful to determine if a bucket exists and you have permission to access it."]
    #[allow(unused_variables, warnings)]
    fn head_bucket(&self, input: &HeadBucketRequest) -> Result<(), HeadBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("HEAD", "s3", &self.region, &request_uri);





        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(HeadBucketError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object."]
    #[allow(unused_variables, warnings)]
    fn head_object(&self, input: &HeadObjectRequest) -> Result<HeadObjectOutput, HeadObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("HEAD", "s3", &self.region, &request_uri);


        if let Some(ref if_match) = input.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = input.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = input.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = input.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = input.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = HeadObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(HeadObjectOutputDeserializer::deserialize(&actual_tag_name,
                                                                            &mut stack));
                }
                if let Some(accept_ranges) = response.headers.get("accept-ranges") {
                    let value = accept_ranges.to_owned();
                    result.accept_ranges = Some(value)
                };
                if let Some(cache_control) = response.headers.get("Cache-Control") {
                    let value = cache_control.to_owned();
                    result.cache_control = Some(value)
                };
                if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                    let value = content_disposition.to_owned();
                    result.content_disposition = Some(value)
                };
                if let Some(content_encoding) = response.headers.get("Content-Encoding") {
                    let value = content_encoding.to_owned();
                    result.content_encoding = Some(value)
                };
                if let Some(content_language) = response.headers.get("Content-Language") {
                    let value = content_language.to_owned();
                    result.content_language = Some(value)
                };
                if let Some(content_length) = response.headers.get("Content-Length") {
                    let value = content_length.to_owned();
                    result.content_length = Some(value.parse::<i64>().unwrap())
                };
                if let Some(content_type) = response.headers.get("Content-Type") {
                    let value = content_type.to_owned();
                    result.content_type = Some(value)
                };
                if let Some(delete_marker) = response.headers.get("x-amz-delete-marker") {
                    let value = delete_marker.to_owned();
                    result.delete_marker = Some(value.parse::<bool>().unwrap())
                };
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(expires) = response.headers.get("Expires") {
                    let value = expires.to_owned();
                    result.expires = Some(value)
                };
                if let Some(last_modified) = response.headers.get("Last-Modified") {
                    let value = last_modified.to_owned();
                    result.last_modified = Some(value)
                };
                let mut values = ::std::collections::HashMap::new();
                for (key, value) in response.headers.iter() {
                    if key.starts_with("x-amz-meta-") {
                        values.insert(key.replace("x-amz-meta-", ""), value.to_owned());
                    }
                }
                result.metadata = Some(values);
                if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                    let value = missing_meta.to_owned();
                    result.missing_meta = Some(value.parse::<i64>().unwrap())
                };
                if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                    let value = parts_count.to_owned();
                    result.parts_count = Some(value.parse::<i64>().unwrap())
                };
                if let Some(replication_status) =
                    response.headers.get("x-amz-replication-status") {
                    let value = replication_status.to_owned();
                    result.replication_status = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(restore) = response.headers.get("x-amz-restore") {
                    let value = restore.to_owned();
                    result.restore = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(storage_class) = response.headers.get("x-amz-storage-class") {
                    let value = storage_class.to_owned();
                    result.storage_class = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                if let Some(website_redirect_location) =
                    response.headers.get("x-amz-website-redirect-location") {
                    let value = website_redirect_location.to_owned();
                    result.website_redirect_location = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(HeadObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Lists the analytics configurations for the bucket."]
    #[allow(unused_variables, warnings)]
    fn list_bucket_analytics_configurations
        (&self,
         input: &ListBucketAnalyticsConfigurationsRequest)
         -> Result<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("analytics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListBucketAnalyticsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListBucketAnalyticsConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBucketAnalyticsConfigurationsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns a list of inventory configurations for the bucket."]
    #[allow(unused_variables, warnings)]
    fn list_bucket_inventory_configurations
        (&self,
         input: &ListBucketInventoryConfigurationsRequest)
         -> Result<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("inventory");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListBucketInventoryConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListBucketInventoryConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBucketInventoryConfigurationsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Lists the metrics configurations for the bucket."]
    #[allow(unused_variables, warnings)]
    fn list_bucket_metrics_configurations
        (&self,
         input: &ListBucketMetricsConfigurationsRequest)
         -> Result<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("metrics");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListBucketMetricsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListBucketMetricsConfigurationsOutputDeserializer::deserialize(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBucketMetricsConfigurationsError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Returns a list of all buckets owned by the authenticated sender of the request."]
    #[allow(unused_variables, warnings)]
    fn list_buckets(&self) -> Result<ListBucketsOutput, ListBucketsError> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);





        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListBucketsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListBucketsOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListBucketsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="This operation lists in-progress multipart uploads."]
    #[allow(unused_variables, warnings)]
    fn list_multipart_uploads(&self,
                              input: &ListMultipartUploadsRequest)
                              -> Result<ListMultipartUploadsOutput, ListMultipartUploadsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = input.max_uploads {
            params.put("max-uploads", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.upload_id_marker {
            params.put("upload-id-marker", x);
        }
        params.put_key("uploads");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListMultipartUploadsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(ListMultipartUploadsOutputDeserializer::deserialize(&actual_tag_name,
                                                                                 &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListMultipartUploadsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns metadata about all of the versions of objects in a bucket."]
    #[allow(unused_variables, warnings)]
    fn list_object_versions(&self,
                            input: &ListObjectVersionsRequest)
                            -> Result<ListObjectVersionsOutput, ListObjectVersionsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.version_id_marker {
            params.put("version-id-marker", x);
        }
        params.put_key("versions");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListObjectVersionsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(ListObjectVersionsOutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectVersionsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket."]
    #[allow(unused_variables, warnings)]
    fn list_objects(&self,
                    input: &ListObjectsRequest)
                    -> Result<ListObjectsOutput, ListObjectsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListObjectsOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development."]
    #[allow(unused_variables, warnings)]
    fn list_objects_v2(&self,
                       input: &ListObjectsV2Request)
                       -> Result<ListObjectsV2Output, ListObjectsV2Error> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        if let Some(ref x) = input.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = input.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = input.fetch_owner {
            params.put("fetch-owner", x);
        }
        if let Some(ref x) = input.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.start_after {
            params.put("start-after", x);
        }
        params.put("list-type", "2");
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), true);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListObjectsV2Output::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListObjectsV2OutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListObjectsV2Error::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Lists the parts that have been uploaded for a specific multipart upload."]
    #[allow(unused_variables, warnings)]
    fn list_parts(&self, input: &ListPartsRequest) -> Result<ListPartsOutput, ListPartsError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.max_parts {
            params.put("max-parts", x);
        }
        if let Some(ref x) = input.part_number_marker {
            params.put("part-number-marker", x);
        }
        params.put("uploadId", &input.upload_id);
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListPartsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(ListPartsOutputDeserializer::deserialize(&actual_tag_name,
                                                                           &mut stack));
                }
                if let Some(abort_date) = response.headers.get("x-amz-abort-date") {
                    let value = abort_date.to_owned();
                    result.abort_date = Some(value)
                };
                if let Some(abort_rule_id) = response.headers.get("x-amz-abort-rule-id") {
                    let value = abort_rule_id.to_owned();
                    result.abort_rule_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPartsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets the accelerate configuration of an existing bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_accelerate_configuration(&self,
                                           input: &PutBucketAccelerateConfigurationRequest)
                                           -> Result<(), PutBucketAccelerateConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        AccelerateConfigurationSerializer::serialize(&mut writer,
                                                     "AccelerateConfiguration",
                                                     &input.accelerate_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketAccelerateConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets the permissions on a bucket using access control lists (ACL)."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_acl(&self, input: &PutBucketAclRequest) -> Result<(), PutBucketAclError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }
        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(&mut writer,
                                                     "AccessControlPolicy",
                                                     input.access_control_policy.as_ref().unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketAclError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets an analytics configuration for the bucket (specified by the analytics configuration ID)."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_analytics_configuration(&self,
                                          input: &PutBucketAnalyticsConfigurationRequest)
                                          -> Result<(), PutBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        AnalyticsConfigurationSerializer::serialize(&mut writer,
                                                    "AnalyticsConfiguration",
                                                    &input.analytics_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketAnalyticsConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Sets the cors configuration for a bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_cors(&self, input: &PutBucketCorsRequest) -> Result<(), PutBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        CORSConfigurationSerializer::serialize(&mut writer,
                                               "CORSConfiguration",
                                               &input.cors_configuration);
        payload = writer.into_inner();
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketCorsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Adds an inventory configuration (identified by the inventory ID) from the bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_inventory_configuration(&self,
                                          input: &PutBucketInventoryConfigurationRequest)
                                          -> Result<(), PutBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        InventoryConfigurationSerializer::serialize(&mut writer,
                                                    "InventoryConfiguration",
                                                    &input.inventory_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketInventoryConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Deprecated, see the PutBucketLifecycleConfiguration operation."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle(&self,
                            input: &PutBucketLifecycleRequest)
                            -> Result<(), PutBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            LifecycleConfigurationSerializer::serialize(&mut writer,
                                                        "LifecycleConfiguration",
                                                        input
                                                            .lifecycle_configuration
                                                            .as_ref()
                                                            .unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketLifecycleError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle_configuration(&self,
                                          input: &PutBucketLifecycleConfigurationRequest)
                                          -> Result<(), PutBucketLifecycleConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            BucketLifecycleConfigurationSerializer::serialize(&mut writer,
                                                              "LifecycleConfiguration",
                                                              input
                                                                  .lifecycle_configuration
                                                                  .as_ref()
                                                                  .unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketLifecycleConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                        .as_ref()))
            }
        }
    }

    #[doc="Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_logging(&self,
                          input: &PutBucketLoggingRequest)
                          -> Result<(), PutBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        BucketLoggingStatusSerializer::serialize(&mut writer,
                                                 "BucketLoggingStatus",
                                                 &input.bucket_logging_status);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketLoggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets a metrics configuration (specified by the metrics configuration ID) for the bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_metrics_configuration(&self,
                                        input: &PutBucketMetricsConfigurationRequest)
                                        -> Result<(), PutBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        MetricsConfigurationSerializer::serialize(&mut writer,
                                                  "MetricsConfiguration",
                                                  &input.metrics_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketMetricsConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                      .as_ref()))
            }
        }
    }

    #[doc="Deprecated, see the PutBucketNotificationConfiguraiton operation."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification(&self,
                               input: &PutBucketNotificationRequest)
                               -> Result<(), PutBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationDeprecatedSerializer::serialize(&mut writer, "NotificationConfigurationDeprecated", &input.notification_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketNotificationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Enables notifications of specified events for a bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification_configuration
        (&self,
         input: &PutBucketNotificationConfigurationRequest)
         -> Result<(), PutBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationSerializer::serialize(&mut writer,
                                                       "NotificationConfiguration",
                                                       &input.notification_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketNotificationConfigurationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_policy(&self,
                         input: &PutBucketPolicyRequest)
                         -> Result<(), PutBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        PolicySerializer::serialize(&mut writer, "Policy", &input.policy);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Creates a new replication configuration (or replaces an existing one, if present)."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_replication(&self,
                              input: &PutBucketReplicationRequest)
                              -> Result<(), PutBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        ReplicationConfigurationSerializer::serialize(&mut writer,
                                                      "ReplicationConfiguration",
                                                      &input.replication_configuration);
        payload = writer.into_inner();
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketReplicationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html"]
    #[allow(unused_variables, warnings)]
    fn put_bucket_request_payment(&self,
                                  input: &PutBucketRequestPaymentRequest)
                                  -> Result<(), PutBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        RequestPaymentConfigurationSerializer::serialize(&mut writer,
                                                         "RequestPaymentConfiguration",
                                                         &input.request_payment_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketRequestPaymentError::from_body(String::from_utf8_lossy(&body)
                                                                .as_ref()))
            }
        }
    }

    #[doc="Sets the tags for a bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_tagging(&self,
                          input: &PutBucketTaggingRequest)
                          -> Result<(), PutBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        payload = writer.into_inner();
        let digest = md5::compute(&payload);
        // need to deref digest and then pass that reference:
        request.add_header("Content-MD5", &base64::encode(&(*digest)));
        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_versioning(&self,
                             input: &PutBucketVersioningRequest)
                             -> Result<(), PutBucketVersioningError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }
        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        VersioningConfigurationSerializer::serialize(&mut writer,
                                                     "VersioningConfiguration",
                                                     &input.versioning_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketVersioningError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Set the website configuration for a bucket."]
    #[allow(unused_variables, warnings)]
    fn put_bucket_website(&self,
                          input: &PutBucketWebsiteRequest)
                          -> Result<(), PutBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        WebsiteConfigurationSerializer::serialize(&mut writer,
                                                  "WebsiteConfiguration",
                                                  &input.website_configuration);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutBucketWebsiteError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Adds an object to a bucket."]
    #[allow(unused_variables, warnings)]
    fn put_object(&self, input: &PutObjectRequest) -> Result<PutObjectOutput, PutObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = input.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = input.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = input.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_length) = input.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref content_type) = input.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = input.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header("x-amz-server-side-encryption-aws-kms-key-id",
                               &ssekms_key_id.to_string());
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header("x-amz-server-side-encryption",
                               &server_side_encryption.to_string());
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header("x-amz-website-redirect-location",
                               &website_redirect_location.to_string());
        }

        let mut payload: Vec<u8>;
        payload = input.body.clone().unwrap();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = PutObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(PutObjectOutputDeserializer::deserialize(&actual_tag_name,
                                                                           &mut stack));
                }
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket"]
    #[allow(unused_variables, warnings)]
    fn put_object_acl(&self,
                      input: &PutObjectAclRequest)
                      -> Result<PutObjectAclOutput, PutObjectAclError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = input.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = input.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = input.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = input.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = input.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(&mut writer,
                                                     "AccessControlPolicy",
                                                     input.access_control_policy.as_ref().unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = PutObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(PutObjectAclOutputDeserializer::deserialize(&actual_tag_name,
                                                                              &mut stack));
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutObjectAclError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Sets the supplied tag-set to an object that already exists in a bucket"]
    #[allow(unused_variables, warnings)]
    fn put_object_tagging(&self,
                          input: &PutObjectTaggingRequest)
                          -> Result<PutObjectTaggingOutput, PutObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);
        let mut payload: Vec<u8>;
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        payload = writer.into_inner();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = PutObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result =
                        try!(PutObjectTaggingOutputDeserializer::deserialize(&actual_tag_name,
                                                                             &mut stack));
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutObjectTaggingError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Restores an archived copy of an object back into Amazon S3"]
    #[allow(unused_variables, warnings)]
    fn restore_object(&self,
                      input: &RestoreObjectRequest)
                      -> Result<RestoreObjectOutput, RestoreObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);


        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("restore");
        request.set_params(params);
        let mut payload: Vec<u8>;
        if input.restore_request.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            RestoreRequestSerializer::serialize(&mut writer,
                                                "RestoreRequest",
                                                input.restore_request.as_ref().unwrap());
            payload = writer.into_inner();
        } else {
            payload = Vec::new();
        }

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = RestoreObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(RestoreObjectOutputDeserializer::deserialize(&actual_tag_name,
                                                                               &mut stack));
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RestoreObjectError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="<p>Uploads a part in a multipart upload.</p><p><b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>"]
    #[allow(unused_variables, warnings)]
    fn upload_part(&self, input: &UploadPartRequest) -> Result<UploadPartOutput, UploadPartError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);


        if let Some(ref content_length) = input.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        let mut payload: Vec<u8>;
        payload = input.body.clone().unwrap();

        request.set_payload(Some(payload));

        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = UploadPartOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UploadPartOutputDeserializer::deserialize(&actual_tag_name,
                                                                            &mut stack));
                }
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UploadPartError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }

    #[doc="Uploads a part by copying data from an existing object as data source."]
    #[allow(unused_variables, warnings)]
    fn upload_part_copy(&self,
                        input: &UploadPartCopyRequest)
                        -> Result<UploadPartCopyOutput, UploadPartCopyError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        request.add_header("x-amz-copy-source", &input.copy_source);

        if let Some(ref copy_source_if_match) = input.copy_source_if_match {
            request.add_header("x-amz-copy-source-if-match",
                               &copy_source_if_match.to_string());
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header("x-amz-copy-source-if-modified-since",
                               &copy_source_if_modified_since.to_string());
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header("x-amz-copy-source-if-none-match",
                               &copy_source_if_none_match.to_string());
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header("x-amz-copy-source-if-unmodified-since",
                               &copy_source_if_unmodified_since.to_string());
        }

        if let Some(ref copy_source_range) = input.copy_source_range {
            request.add_header("x-amz-copy-source-range", &copy_source_range.to_string());
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-algorithm",
                               &copy_source_sse_customer_algorithm.to_string());
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-key",
                               &copy_source_sse_customer_key.to_string());
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header("x-amz-copy-source-server-side-encryption-customer-key-MD5",
                               &copy_source_sse_customer_key_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header("x-amz-server-side-encryption-customer-algorithm",
                               &sse_customer_algorithm.to_string());
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header("x-amz-server-side-encryption-customer-key",
                               &sse_customer_key.to_string());
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header("x-amz-server-side-encryption-customer-key-MD5",
                               &sse_customer_key_md5.to_string());
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);


        request.sign_with_plus(&try!(self.credentials_provider.credentials()), false);

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok |
            StatusCode::NoContent |
            StatusCode::PartialContent => {

                let mut result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = UploadPartCopyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    result = try!(UploadPartCopyOutputDeserializer::deserialize(&actual_tag_name,
                                                                                &mut stack));
                }
                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id") {
                    let value = copy_source_version_id.to_owned();
                    result.copy_source_version_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-algorithm") {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-customer-key-MD5") {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) =
                    response
                        .headers
                        .get("x-amz-server-side-encryption-aws-kms-key-id") {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption") {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UploadPartCopyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use super::*;
    use self::rusoto_mock::*;
    use rusoto_core::Region as rusoto_region;


    #[test]
    fn test_parse_error_s3_create_bucket() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/error",
                                                              "s3-create-bucket.xml");
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateBucketRequest::default();
        let result = client.create_bucket(&request);
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_error_s3_list_objects() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/error",
                                                              "s3-list-objects.xml");
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(&request);
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_acl() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-get-bucket-acl.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketAclRequest::default();
        let result = client.get_bucket_acl(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_get_bucket_location() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-get-bucket-location.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLocationRequest::default();
        let result = client.get_bucket_location(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_get_bucket_logging() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-get-bucket-logging.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLoggingRequest::default();
        let result = client.get_bucket_logging(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_get_bucket_policy() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-get-bucket-policy.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketPolicyRequest::default();
        let result = client.get_bucket_policy(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_list_buckets() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-list-buckets.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.list_buckets();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_list_multipart_uploads() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-list-multipart-uploads.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMultipartUploadsRequest::default();
        let result = client.list_multipart_uploads(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_list_object_versions() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-list-object-versions.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectVersionsRequest::default();
        let result = client.list_object_versions(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_s3_list_objects() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "s3-list-objects.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
