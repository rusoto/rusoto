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

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use std::io::Write;
use std::str::FromStr;
use xml;
use xml::reader::ParserConfig;
use xml::EventReader;
use xml::EventWriter;

/// <p>Specifies the days since the initiation of an Incomplete Multipart Upload that Lifecycle will wait before permanently removing all parts of the upload.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortIncompleteMultipartUpload {
    /// <p>Indicates the number of days that must pass since initiation for Lifecycle to abort an Incomplete Multipart Upload.</p>
    pub days_after_initiation: Option<i64>,
}

struct AbortIncompleteMultipartUploadDeserializer;
impl AbortIncompleteMultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortIncompleteMultipartUpload, XmlParseError> {
        deserialize_elements::<_, AbortIncompleteMultipartUpload, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DaysAfterInitiation" => {
                        obj.days_after_initiation =
                            Some(DaysAfterInitiationDeserializer::deserialize(
                                "DaysAfterInitiation",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct AbortIncompleteMultipartUploadSerializer;
impl AbortIncompleteMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AbortIncompleteMultipartUpload,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days_after_initiation {
            writer.write(xml::writer::XmlEvent::start_element("DaysAfterInitiation"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortMultipartUploadRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p><p/></p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortMultipartUploadResponse {
    pub request_charged: Option<String>,
}

struct AbortMultipartUploadResponseDeserializer;
impl AbortMultipartUploadResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortMultipartUploadResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = AbortMultipartUploadResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccelerateConfiguration {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<String>,
}

pub struct AccelerateConfigurationSerializer;
impl AccelerateConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccelerateConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
}

pub struct AccessControlPolicySerializer;
impl AccessControlPolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccessControlPolicy,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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

/// <p>A container for information about access control for replicas.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessControlTranslation {
    /// <p>The override value for the owner of the replica object.</p>
    pub owner: String,
}

struct AccessControlTranslationDeserializer;
impl AccessControlTranslationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccessControlTranslation, XmlParseError> {
        deserialize_elements::<_, AccessControlTranslation, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Owner" => {
                        obj.owner = OwnerOverrideDeserializer::deserialize("Owner", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct AccessControlTranslationSerializer;
impl AccessControlTranslationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AccessControlTranslation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Owner"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.owner
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AccountIdDeserializer;
impl AccountIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AccountIdSerializer;
impl AccountIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct AllowQuotedRecordDelimiterSerializer;
impl AllowQuotedRecordDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedHeaderDeserializer;
impl AllowedHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AllowedHeaderSerializer;
impl AllowedHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedHeadersDeserializer;
impl AllowedHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(AllowedHeaderDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AllowedMethodSerializer;
impl AllowedMethodSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedMethodsDeserializer;
impl AllowedMethodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(AllowedMethodDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AllowedOriginSerializer;
impl AllowedOriginSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AllowedOriginsDeserializer;
impl AllowedOriginsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(AllowedOriginDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            AllowedOriginSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsAndOperator {
    /// <p>The prefix to use when evaluating an AND predicate.</p>
    pub prefix: Option<String>,
    /// <p>The list of tags to use when evaluating an AND predicate.</p>
    pub tags: Option<Vec<Tag>>,
}

struct AnalyticsAndOperatorDeserializer;
impl AnalyticsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsAndOperator, XmlParseError> {
        deserialize_elements::<_, AnalyticsAndOperator, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagSetDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct AnalyticsAndOperatorSerializer;
impl AnalyticsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsConfiguration {
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub filter: Option<AnalyticsFilter>,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
    /// <p>If present, it indicates that data related to access patterns will be collected and made available to analyze the tradeoffs between different storage classes.</p>
    pub storage_class_analysis: StorageClassAnalysis,
}

struct AnalyticsConfigurationDeserializer;
impl AnalyticsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsConfiguration, XmlParseError> {
        deserialize_elements::<_, AnalyticsConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Filter" => {
                    obj.filter = Some(AnalyticsFilterDeserializer::deserialize("Filter", stack)?);
                }
                "Id" => {
                    obj.id = AnalyticsIdDeserializer::deserialize("Id", stack)?;
                }
                "StorageClassAnalysis" => {
                    obj.storage_class_analysis = StorageClassAnalysisDeserializer::deserialize(
                        "StorageClassAnalysis",
                        stack,
                    )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct AnalyticsConfigurationSerializer;
impl AnalyticsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &AnalyticsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        StorageClassAnalysisSerializer::serialize(
            &mut writer,
            "StorageClassAnalysis",
            &obj.storage_class_analysis,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsConfigurationListDeserializer;
impl AnalyticsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AnalyticsConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(AnalyticsConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsExportDestination {
    /// <p>A destination signifying output to an S3 bucket.</p>
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

struct AnalyticsExportDestinationDeserializer;
impl AnalyticsExportDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsExportDestination, XmlParseError> {
        deserialize_elements::<_, AnalyticsExportDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "S3BucketDestination" => {
                        obj.s3_bucket_destination =
                            AnalyticsS3BucketDestinationDeserializer::deserialize(
                                "S3BucketDestination",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct AnalyticsExportDestinationSerializer;
impl AnalyticsExportDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsExportDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsS3BucketDestinationSerializer::serialize(
            &mut writer,
            "S3BucketDestination",
            &obj.s3_bucket_destination,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating an analytics filter. The operator must have at least two predicates.</p>
    pub and: Option<AnalyticsAndOperator>,
    /// <p>The prefix to use when evaluating an analytics filter.</p>
    pub prefix: Option<String>,
    /// <p>The tag to use when evaluating an analytics filter.</p>
    pub tag: Option<Tag>,
}

struct AnalyticsFilterDeserializer;
impl AnalyticsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsFilter, XmlParseError> {
        deserialize_elements::<_, AnalyticsFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "And" => {
                    obj.and = Some(AnalyticsAndOperatorDeserializer::deserialize("And", stack)?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct AnalyticsFilterSerializer;
impl AnalyticsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &AnalyticsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AnalyticsIdSerializer;
impl AnalyticsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsS3BucketDestination {
    /// <p>The Amazon resource name (ARN) of the bucket to which data is exported.</p>
    pub bucket: String,
    /// <p>The account ID that owns the destination bucket. If no account ID is provided, the owner will not be validated prior to exporting data.</p>
    pub bucket_account_id: Option<String>,
    /// <p>The file format used when exporting data to Amazon S3.</p>
    pub format: String,
    /// <p>The prefix to use when exporting data. The exported data begins with this prefix.</p>
    pub prefix: Option<String>,
}

struct AnalyticsS3BucketDestinationDeserializer;
impl AnalyticsS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AnalyticsS3BucketDestination, XmlParseError> {
        deserialize_elements::<_, AnalyticsS3BucketDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = BucketNameDeserializer::deserialize("Bucket", stack)?;
                    }
                    "BucketAccountId" => {
                        obj.bucket_account_id = Some(AccountIdDeserializer::deserialize(
                            "BucketAccountId",
                            stack,
                        )?);
                    }
                    "Format" => {
                        obj.format =
                            AnalyticsS3ExportFileFormatDeserializer::deserialize("Format", stack)?;
                    }
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct AnalyticsS3BucketDestinationSerializer;
impl AnalyticsS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &AnalyticsS3BucketDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.bucket_account_id {
            writer.write(xml::writer::XmlEvent::start_element("BucketAccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Format"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.format
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct AnalyticsS3ExportFileFormatDeserializer;
impl AnalyticsS3ExportFileFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct AnalyticsS3ExportFileFormatSerializer;
impl AnalyticsS3ExportFileFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub type StreamingBody = ::rusoto_core::ByteStream;
struct BodyDeserializer;
impl BodyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bytes::Bytes, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?.into();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BodySerializer;
impl BodySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bytes::Bytes,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = String::from_utf8(obj.to_vec()).expect("Not a UTF-8 string")
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bucket {
    /// <p>Date the bucket was created.</p>
    pub creation_date: Option<String>,
    /// <p>The name of the bucket.</p>
    pub name: Option<String>,
}

struct BucketDeserializer;
impl BucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
        deserialize_elements::<_, Bucket, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreationDate" => {
                    obj.creation_date = Some(CreationDateDeserializer::deserialize(
                        "CreationDate",
                        stack,
                    )?);
                }
                "Name" => {
                    obj.name = Some(BucketNameDeserializer::deserialize("Name", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct BucketAccelerateStatusDeserializer;
impl BucketAccelerateStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BucketAccelerateStatusSerializer;
impl BucketAccelerateStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BucketLifecycleConfiguration {
    /// <p><p/></p>
    pub rules: Vec<LifecycleRule>,
}

pub struct BucketLifecycleConfigurationSerializer;
impl BucketLifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &BucketLifecycleConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        LifecycleRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketLocationConstraintDeserializer;
impl BucketLocationConstraintDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BucketLocationConstraintSerializer;
impl BucketLocationConstraintSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

pub struct BucketLoggingStatusSerializer;
impl BucketLoggingStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &BucketLoggingStatus,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BucketLogsPermissionSerializer;
impl BucketLogsPermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketNameDeserializer;
impl BucketNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BucketNameSerializer;
impl BucketNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketVersioningStatusDeserializer;
impl BucketVersioningStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct BucketVersioningStatusSerializer;
impl BucketVersioningStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct BucketsDeserializer;
impl BucketsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Bucket>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Bucket" {
                obj.push(BucketDeserializer::deserialize("Bucket", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct BytesProcessedDeserializer;
impl BytesProcessedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BytesReturnedDeserializer;
impl BytesReturnedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BytesScannedDeserializer;
impl BytesScannedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSConfiguration {
    /// <p><p/></p>
    pub cors_rules: Vec<CORSRule>,
}

pub struct CORSConfigurationSerializer;
impl CORSConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CORSConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        CORSRulesSerializer::serialize(&mut writer, "CORSRule", &obj.cors_rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSRule {
    /// <p>Specifies which headers are allowed in a pre-flight OPTIONS request.</p>
    pub allowed_headers: Option<Vec<String>>,
    /// <p>Identifies HTTP methods that the domain/origin specified in the rule is allowed to execute.</p>
    pub allowed_methods: Vec<String>,
    /// <p>One or more origins you want customers to be able to access the bucket from.</p>
    pub allowed_origins: Vec<String>,
    /// <p>One or more headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript XMLHttpRequest object).</p>
    pub expose_headers: Option<Vec<String>>,
    /// <p>The time in seconds that your browser is to cache the preflight response for the specified resource.</p>
    pub max_age_seconds: Option<i64>,
}

struct CORSRuleDeserializer;
impl CORSRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CORSRule, XmlParseError> {
        deserialize_elements::<_, CORSRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllowedHeader" => {
                    obj.allowed_headers.get_or_insert(vec![]).extend(
                        AllowedHeadersDeserializer::deserialize("AllowedHeader", stack)?,
                    );
                }
                "AllowedMethod" => {
                    obj.allowed_methods
                        .extend(AllowedMethodsDeserializer::deserialize(
                            "AllowedMethod",
                            stack,
                        )?);
                }
                "AllowedOrigin" => {
                    obj.allowed_origins
                        .extend(AllowedOriginsDeserializer::deserialize(
                            "AllowedOrigin",
                            stack,
                        )?);
                }
                "ExposeHeader" => {
                    obj.expose_headers.get_or_insert(vec![]).extend(
                        ExposeHeadersDeserializer::deserialize("ExposeHeader", stack)?,
                    );
                }
                "MaxAgeSeconds" => {
                    obj.max_age_seconds = Some(MaxAgeSecondsDeserializer::deserialize(
                        "MaxAgeSeconds",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct CORSRuleSerializer;
impl CORSRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CORSRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
            writer.write(xml::writer::XmlEvent::start_element("MaxAgeSeconds"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CORSRulesDeserializer;
impl CORSRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CORSRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(CORSRuleDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CORSRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            CORSRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>Describes how a CSV-formatted input object is formatted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CSVInput {
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub allow_quoted_record_delimiter: Option<bool>,
    /// <p>The single character used to indicate a row should be ignored when present at the start of a row.</p>
    pub comments: Option<String>,
    /// <p>The value used to separate individual fields in a record.</p>
    pub field_delimiter: Option<String>,
    /// <p>Describes the first line of input. Valid values: None, Ignore, Use.</p>
    pub file_header_info: Option<String>,
    /// <p>Value used for escaping where the field delimiter is part of the value.</p>
    pub quote_character: Option<String>,
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: Option<String>,
    /// <p>The value used to separate individual records.</p>
    pub record_delimiter: Option<String>,
}

pub struct CSVInputSerializer;
impl CSVInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allow_quoted_record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element(
                "AllowQuotedRecordDelimiter",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.comments {
            writer.write(xml::writer::XmlEvent::start_element("Comments"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.field_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("FieldDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.file_header_info {
            writer.write(xml::writer::XmlEvent::start_element("FileHeaderInfo"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteEscapeCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes how CSV-formatted results are formatted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CSVOutput {
    /// <p>The value used to separate individual fields in a record.</p>
    pub field_delimiter: Option<String>,
    /// <p>The value used for escaping where the field delimiter is part of the value.</p>
    pub quote_character: Option<String>,
    /// <p>Th single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: Option<String>,
    /// <p>Indicates whether or not all output fields should be quoted.</p>
    pub quote_fields: Option<String>,
    /// <p>The value used to separate individual records.</p>
    pub record_delimiter: Option<String>,
}

pub struct CSVOutputSerializer;
impl CSVOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.field_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("FieldDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            writer.write(xml::writer::XmlEvent::start_element("QuoteEscapeCharacter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.quote_fields {
            writer.write(xml::writer::XmlEvent::start_element("QuoteFields"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CloudFunctionDeserializer;
impl CloudFunctionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct CloudFunctionSerializer;
impl CloudFunctionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CloudFunctionConfiguration {
    /// <p><p/></p>
    pub cloud_function: Option<String>,
    /// <p><p/></p>
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    /// <p><p/></p>
    pub invocation_role: Option<String>,
}

struct CloudFunctionConfigurationDeserializer;
impl CloudFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CloudFunctionConfiguration, XmlParseError> {
        deserialize_elements::<_, CloudFunctionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CloudFunction" => {
                        obj.cloud_function = Some(CloudFunctionDeserializer::deserialize(
                            "CloudFunction",
                            stack,
                        )?);
                    }
                    "Event" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Event", stack)?);
                    }
                    "Id" => {
                        obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                    }
                    "InvocationRole" => {
                        obj.invocation_role =
                            Some(CloudFunctionInvocationRoleDeserializer::deserialize(
                                "InvocationRole",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct CloudFunctionConfigurationSerializer;
impl CloudFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CloudFunctionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function {
            writer.write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.invocation_role {
            writer.write(xml::writer::XmlEvent::start_element("InvocationRole"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CloudFunctionInvocationRoleDeserializer;
impl CloudFunctionInvocationRoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct CloudFunctionInvocationRoleSerializer;
impl CloudFunctionInvocationRoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct CodeDeserializer;
impl CodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct CommentsSerializer;
impl CommentsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CommonPrefix {
    /// <p><p/></p>
    pub prefix: Option<String>,
}

struct CommonPrefixDeserializer;
impl CommonPrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CommonPrefix, XmlParseError> {
        deserialize_elements::<_, CommonPrefix, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct CommonPrefixListDeserializer;
impl CommonPrefixListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CommonPrefix>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(CommonPrefixDeserializer::deserialize(tag_name, stack)?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteMultipartUploadRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    /// <p><p/></p>
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub request_payer: Option<String>,
    /// <p><p/></p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompleteMultipartUploadResponse {
    /// <p><p/></p>
    pub bucket: Option<String>,
    /// <p>Entity tag of the object.</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    /// <p><p/></p>
    pub key: Option<String>,
    /// <p><p/></p>
    pub location: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
}

struct CompleteMultipartUploadResponseDeserializer;
impl CompleteMultipartUploadResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMultipartUploadResponse, XmlParseError> {
        deserialize_elements::<_, CompleteMultipartUploadResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = Some(BucketNameDeserializer::deserialize("Bucket", stack)?);
                    }
                    "ETag" => {
                        obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                    }
                    "Key" => {
                        obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                    }
                    "Location" => {
                        obj.location = Some(LocationDeserializer::deserialize("Location", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompletedMultipartUpload {
    /// <p><p/></p>
    pub parts: Option<Vec<CompletedPart>>,
}

pub struct CompletedMultipartUploadSerializer;
impl CompletedMultipartUploadSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CompletedMultipartUpload,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.parts {
            &CompletedPartListSerializer::serialize(&mut writer, "Part", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CompletedPart {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<String>,
    /// <p>Part number that identifies the part. This is a positive integer between 1 and 10,000.</p>
    pub part_number: Option<i64>,
}

pub struct CompletedPartSerializer;
impl CompletedPartSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CompletedPart,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.e_tag {
            writer.write(xml::writer::XmlEvent::start_element("ETag"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.part_number {
            writer.write(xml::writer::XmlEvent::start_element("PartNumber"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct CompletedPartListSerializer;
impl CompletedPartListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<CompletedPart>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            CompletedPartSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

pub struct CompressionTypeSerializer;
impl CompressionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element Condition is specified and sibling KeyPrefixEquals is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub http_error_code_returned_equals: Option<String>,
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for ExamplePage.html, the key prefix will be ExamplePage.html. To redirect request for all pages with the prefix docs/, the key prefix will be /docs, which identifies all objects in the docs/ folder. Required when the parent element Condition is specified and sibling HttpErrorCodeReturnedEquals is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p>
    pub key_prefix_equals: Option<String>,
}

struct ConditionDeserializer;
impl ConditionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Condition, XmlParseError> {
        deserialize_elements::<_, Condition, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HttpErrorCodeReturnedEquals" => {
                    obj.http_error_code_returned_equals =
                        Some(HttpErrorCodeReturnedEqualsDeserializer::deserialize(
                            "HttpErrorCodeReturnedEquals",
                            stack,
                        )?);
                }
                "KeyPrefixEquals" => {
                    obj.key_prefix_equals = Some(KeyPrefixEqualsDeserializer::deserialize(
                        "KeyPrefixEquals",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ConditionSerializer;
impl ConditionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Condition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.http_error_code_returned_equals {
            writer.write(xml::writer::XmlEvent::start_element(
                "HttpErrorCodeReturnedEquals",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.key_prefix_equals {
            writer.write(xml::writer::XmlEvent::start_element("KeyPrefixEquals"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinuationEvent {}

struct ContinuationEventDeserializer;
impl ContinuationEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContinuationEvent, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = ContinuationEvent::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded.</p>
    pub copy_source: String,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<String>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<String>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<String>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<String>,
    /// <p>Specifies the algorithm to use when decrypting the source object (e.g., AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.</p>
    pub copy_source_sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub copy_source_sse_customer_key_md5: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request.</p>
    pub metadata_directive: Option<String>,
    /// <p>Specifies whether you want to apply a Legal Hold to the copied object.</p>
    pub object_lock_legal_hold_status: Option<String>,
    /// <p>The Object Lock mode that you want to apply to the copied object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when you want the copied object's Object Lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object destination object this value must be used in conjunction with the TaggingDirective. The tag-set must be encoded as URL Query parameters</p>
    pub tagging: Option<String>,
    /// <p>Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request.</p>
    pub tagging_directive: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectResponse {
    /// <p><p/></p>
    pub copy_object_result: Option<CopyObjectResult>,
    /// <p><p/></p>
    pub copy_source_version_id: Option<String>,
    /// <p>If the object expiration is configured, the response includes this header.</p>
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version ID of the newly created copy.</p>
    pub version_id: Option<String>,
}

struct CopyObjectResponseDeserializer;
impl CopyObjectResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectResponse, XmlParseError> {
        Ok(CopyObjectResponse {
            copy_object_result: Some(CopyObjectResultDeserializer::deserialize(
                "CopyObjectResult",
                stack,
            )?),
            ..CopyObjectResponse::default()
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyObjectResult {
    /// <p><p/></p>
    pub e_tag: Option<String>,
    /// <p><p/></p>
    pub last_modified: Option<String>,
}

struct CopyObjectResultDeserializer;
impl CopyObjectResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectResult, XmlParseError> {
        deserialize_elements::<_, CopyObjectResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ETag" => {
                    obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CopyPartResult {
    /// <p>Entity tag of the object.</p>
    pub e_tag: Option<String>,
    /// <p>Date and time at which the object was uploaded.</p>
    pub last_modified: Option<String>,
}

struct CopyPartResultDeserializer;
impl CopyPartResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyPartResult, XmlParseError> {
        deserialize_elements::<_, CopyPartResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ETag" => {
                    obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketConfiguration {
    /// <p>Specifies the region where the bucket will be created. If you don't specify a region, the bucket is created in US East (N. Virginia) Region (us-east-1).</p>
    pub location_constraint: Option<String>,
}

pub struct CreateBucketConfigurationSerializer;
impl CreateBucketConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CreateBucketConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.location_constraint {
            writer.write(xml::writer::XmlEvent::start_element("LocationConstraint"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketRequest {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<String>,
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
    /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p>
    pub object_lock_enabled_for_bucket: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateBucketResponse {
    /// <p><p/></p>
    pub location: Option<String>,
}

struct CreateBucketResponseDeserializer;
impl CreateBucketResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateBucketResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateMultipartUploadRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether you want to apply a Legal Hold to the uploaded object.</p>
    pub object_lock_legal_hold_status: Option<String>,
    /// <p>Specifies the Object Lock mode that you want to apply to the uploaded object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>Specifies the date and time when you want the Object Lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters</p>
    pub tagging: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateMultipartUploadResponse {
    /// <p>Date when multipart upload will become eligible for abort operation by lifecycle.</p>
    pub abort_date: Option<String>,
    /// <p>Id of the lifecycle rule that makes a multipart upload eligible for abort operation.</p>
    pub abort_rule_id: Option<String>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: Option<String>,
}

struct CreateMultipartUploadResponseDeserializer;
impl CreateMultipartUploadResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMultipartUploadResponse, XmlParseError> {
        deserialize_elements::<_, CreateMultipartUploadResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = Some(BucketNameDeserializer::deserialize("Bucket", stack)?);
                    }
                    "Key" => {
                        obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                    }
                    "UploadId" => {
                        obj.upload_id = Some(MultipartUploadIdDeserializer::deserialize(
                            "UploadId", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct CreationDateDeserializer;
impl CreationDateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DateDeserializer;
impl DateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DateSerializer;
impl DateSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DaysDeserializer;
impl DaysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DaysSerializer;
impl DaysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DaysAfterInitiationDeserializer;
impl DaysAfterInitiationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DaysAfterInitiationSerializer;
impl DaysAfterInitiationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The container element for specifying the default Object Lock retention settings for new objects placed in the specified bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultRetention {
    /// <p>The number of days that you want to specify for the default retention period.</p>
    pub days: Option<i64>,
    /// <p>The default Object Lock retention mode you want to apply to new objects placed in the specified bucket.</p>
    pub mode: Option<String>,
    /// <p>The number of years that you want to specify for the default retention period.</p>
    pub years: Option<i64>,
}

struct DefaultRetentionDeserializer;
impl DefaultRetentionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DefaultRetention, XmlParseError> {
        deserialize_elements::<_, DefaultRetention, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Days" => {
                    obj.days = Some(DaysDeserializer::deserialize("Days", stack)?);
                }
                "Mode" => {
                    obj.mode = Some(ObjectLockRetentionModeDeserializer::deserialize(
                        "Mode", stack,
                    )?);
                }
                "Years" => {
                    obj.years = Some(YearsDeserializer::deserialize("Years", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct DefaultRetentionSerializer;
impl DefaultRetentionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DefaultRetention,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.mode {
            writer.write(xml::writer::XmlEvent::start_element("Mode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.years {
            writer.write(xml::writer::XmlEvent::start_element("Years"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Delete {
    /// <p><p/></p>
    pub objects: Vec<ObjectIdentifier>,
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to true.</p>
    pub quiet: Option<bool>,
}

pub struct DeleteSerializer;
impl DeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Delete,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ObjectIdentifierListSerializer::serialize(&mut writer, "Object", &obj.objects)?;
        if let Some(ref value) = obj.quiet {
            writer.write(xml::writer::XmlEvent::start_element("Quiet"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketAnalyticsConfigurationResponse {}

struct DeleteBucketAnalyticsConfigurationResponseDeserializer;
impl DeleteBucketAnalyticsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketAnalyticsConfigurationResponse, XmlParseError> {
        Ok(DeleteBucketAnalyticsConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketCorsRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketCorsResponse {}

struct DeleteBucketCorsResponseDeserializer;
impl DeleteBucketCorsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketCorsResponse, XmlParseError> {
        Ok(DeleteBucketCorsResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketEncryptionRequest {
    /// <p>The name of the bucket containing the server-side encryption configuration to delete.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketEncryptionResponse {}

struct DeleteBucketEncryptionResponseDeserializer;
impl DeleteBucketEncryptionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketEncryptionResponse, XmlParseError> {
        Ok(DeleteBucketEncryptionResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketInventoryConfigurationResponse {}

struct DeleteBucketInventoryConfigurationResponseDeserializer;
impl DeleteBucketInventoryConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketInventoryConfigurationResponse, XmlParseError> {
        Ok(DeleteBucketInventoryConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketLifecycleRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketLifecycleResponse {}

struct DeleteBucketLifecycleResponseDeserializer;
impl DeleteBucketLifecycleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketLifecycleResponse, XmlParseError> {
        Ok(DeleteBucketLifecycleResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket containing the metrics configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketMetricsConfigurationResponse {}

struct DeleteBucketMetricsConfigurationResponseDeserializer;
impl DeleteBucketMetricsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketMetricsConfigurationResponse, XmlParseError> {
        Ok(DeleteBucketMetricsConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketPolicyRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketPolicyResponse {}

struct DeleteBucketPolicyResponseDeserializer;
impl DeleteBucketPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketPolicyResponse, XmlParseError> {
        Ok(DeleteBucketPolicyResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketReplicationRequest {
    /// <p><p> The bucket name. </p> <note> <p>It can take a while to propagate the deletion of a replication configuration to all Amazon S3 systems.</p> </note></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketReplicationResponse {}

struct DeleteBucketReplicationResponseDeserializer;
impl DeleteBucketReplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketReplicationResponse, XmlParseError> {
        Ok(DeleteBucketReplicationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketResponse {}

struct DeleteBucketResponseDeserializer;
impl DeleteBucketResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketResponse, XmlParseError> {
        Ok(DeleteBucketResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketTaggingResponse {}

struct DeleteBucketTaggingResponseDeserializer;
impl DeleteBucketTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketTaggingResponse, XmlParseError> {
        Ok(DeleteBucketTaggingResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketWebsiteResponse {}

struct DeleteBucketWebsiteResponseDeserializer;
impl DeleteBucketWebsiteResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteBucketWebsiteResponse, XmlParseError> {
        Ok(DeleteBucketWebsiteResponse::default())
    }
}
struct DeleteMarkerDeserializer;
impl DeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMarkerEntry {
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub is_latest: Option<bool>,
    /// <p>The object key.</p>
    pub key: Option<String>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<String>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<String>,
}

struct DeleteMarkerEntryDeserializer;
impl DeleteMarkerEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMarkerEntry, XmlParseError> {
        deserialize_elements::<_, DeleteMarkerEntry, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsLatest" => {
                    obj.is_latest = Some(IsLatestDeserializer::deserialize("IsLatest", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                "VersionId" => {
                    obj.version_id = Some(ObjectVersionIdDeserializer::deserialize(
                        "VersionId",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Specifies whether Amazon S3 should replicate delete makers.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteMarkerReplication {
    /// <p><p>The status of the delete marker replication.</p> <note> <p> In the current implementation, Amazon S3 doesn&#39;t replicate the delete markers. The status must be <code>Disabled</code>. </p> </note></p>
    pub status: Option<String>,
}

struct DeleteMarkerReplicationDeserializer;
impl DeleteMarkerReplicationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteMarkerReplication, XmlParseError> {
        deserialize_elements::<_, DeleteMarkerReplication, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(DeleteMarkerReplicationStatusDeserializer::deserialize(
                            "Status", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct DeleteMarkerReplicationSerializer;
impl DeleteMarkerReplicationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &DeleteMarkerReplication,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DeleteMarkerReplicationStatusDeserializer;
impl DeleteMarkerReplicationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DeleteMarkerReplicationStatusSerializer;
impl DeleteMarkerReplicationStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DeleteMarkerVersionIdDeserializer;
impl DeleteMarkerVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DeleteMarkersDeserializer;
impl DeleteMarkersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeleteMarkerEntry>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(DeleteMarkerEntryDeserializer::deserialize(tag_name, stack)?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Indicates whether S3 Object Lock should bypass Governance-mode restrictions to process this operation.</p>
    pub bypass_governance_retention: Option<bool>,
    /// <p><p/></p>
    pub key: String,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectResponse {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was not (false) a delete marker.</p>
    pub delete_marker: Option<bool>,
    pub request_charged: Option<String>,
    /// <p>Returns the version ID of the delete marker created as a result of the DELETE operation.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectResponseDeserializer;
impl DeleteObjectResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteObjectResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    /// <p>The versionId of the object that the tag-set will be removed from.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectTaggingResponse {
    /// <p>The versionId of the object the tag-set was removed from.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectTaggingResponseDeserializer;
impl DeleteObjectTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectTaggingResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteObjectTaggingResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Specifies whether you want to delete this object even if it has a Governance-type Object Lock in place. You must have sufficient permissions to perform this operation.</p>
    pub bypass_governance_retention: Option<bool>,
    /// <p><p/></p>
    pub delete: Delete,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectsResponse {
    /// <p><p/></p>
    pub deleted: Option<Vec<DeletedObject>>,
    /// <p><p/></p>
    pub errors: Option<Vec<S3Error>>,
    pub request_charged: Option<String>,
}

struct DeleteObjectsResponseDeserializer;
impl DeleteObjectsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectsResponse, XmlParseError> {
        deserialize_elements::<_, DeleteObjectsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Deleted" => {
                    obj.deleted
                        .get_or_insert(vec![])
                        .extend(DeletedObjectsDeserializer::deserialize("Deleted", stack)?);
                }
                "Error" => {
                    obj.errors
                        .get_or_insert(vec![])
                        .extend(ErrorsDeserializer::deserialize("Error", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePublicAccessBlockRequest {
    /// <p>The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete. </p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePublicAccessBlockResponse {}

struct DeletePublicAccessBlockResponseDeserializer;
impl DeletePublicAccessBlockResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletePublicAccessBlockResponse, XmlParseError> {
        Ok(DeletePublicAccessBlockResponse::default())
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletedObject {
    /// <p><p/></p>
    pub delete_marker: Option<bool>,
    /// <p><p/></p>
    pub delete_marker_version_id: Option<String>,
    /// <p><p/></p>
    pub key: Option<String>,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct DeletedObjectDeserializer;
impl DeletedObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletedObject, XmlParseError> {
        deserialize_elements::<_, DeletedObject, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DeleteMarker" => {
                    obj.delete_marker = Some(DeleteMarkerDeserializer::deserialize(
                        "DeleteMarker",
                        stack,
                    )?);
                }
                "DeleteMarkerVersionId" => {
                    obj.delete_marker_version_id =
                        Some(DeleteMarkerVersionIdDeserializer::deserialize(
                            "DeleteMarkerVersionId",
                            stack,
                        )?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "VersionId" => {
                    obj.version_id = Some(ObjectVersionIdDeserializer::deserialize(
                        "VersionId",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DeletedObjectsDeserializer;
impl DeletedObjectsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DeletedObject>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(DeletedObjectDeserializer::deserialize(tag_name, stack)?);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DelimiterSerializer;
impl DelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct DescriptionSerializer;
impl DescriptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for information about the replication destination.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Destination {
    /// <p>A container for information about access control for replicas. </p> <p>Use this element only in a cross-account scenario where source and destination bucket owners are not the same to change replica ownership to the AWS account that owns the destination bucket. If you don't add this element to the replication configuration, the replicas are owned by same AWS account that owns the source object. </p>
    pub access_control_translation: Option<AccessControlTranslation>,
    /// <p>The account ID of the destination bucket. Currently, Amazon S3 verifies this value only if Access Control Translation is enabled. </p> <p>In a cross-account scenario, if you change replica ownership to the AWS account that owns the destination bucket by adding the <code>AccessControlTranslation</code> element, this is the account ID of the owner of the destination bucket. </p>
    pub account: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store replicas of the object identified by the rule. </p> <p> If there are multiple rules in your replication configuration, all rules must specify the same bucket as the destination. A replication configuration can replicate objects to only one destination bucket. </p>
    pub bucket: String,
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element. </p>
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p> The class of storage used to store the object. By default Amazon S3 uses storage class of the source object when creating a replica. </p>
    pub storage_class: Option<String>,
}

struct DestinationDeserializer;
impl DestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Destination, XmlParseError> {
        deserialize_elements::<_, Destination, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AccessControlTranslation" => {
                    obj.access_control_translation =
                        Some(AccessControlTranslationDeserializer::deserialize(
                            "AccessControlTranslation",
                            stack,
                        )?);
                }
                "Account" => {
                    obj.account = Some(AccountIdDeserializer::deserialize("Account", stack)?);
                }
                "Bucket" => {
                    obj.bucket = BucketNameDeserializer::deserialize("Bucket", stack)?;
                }
                "EncryptionConfiguration" => {
                    obj.encryption_configuration =
                        Some(EncryptionConfigurationDeserializer::deserialize(
                            "EncryptionConfiguration",
                            stack,
                        )?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(StorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct DestinationSerializer;
impl DestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Destination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.access_control_translation {
            &AccessControlTranslationSerializer::serialize(
                &mut writer,
                "AccessControlTranslation",
                value,
            )?;
        }
        if let Some(ref value) = obj.account {
            writer.write(xml::writer::XmlEvent::start_element("Account"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.encryption_configuration {
            &EncryptionConfigurationSerializer::serialize(
                &mut writer,
                "EncryptionConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct DisplayNameDeserializer;
impl DisplayNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct DisplayNameSerializer;
impl DisplayNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ETagDeserializer;
impl ETagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ETagSerializer;
impl ETagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EmailAddressDeserializer;
impl EmailAddressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct EmailAddressSerializer;
impl EmailAddressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct EnableRequestProgressSerializer;
impl EnableRequestProgressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EncodingTypeDeserializer;
impl EncodingTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct EncodingTypeSerializer;
impl EncodingTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes the server-side encryption that will be applied to the restore results.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Encryption {
    /// <p>The server-side encryption algorithm used when storing job results in Amazon S3 (e.g., AES256, aws:kms).</p>
    pub encryption_type: String,
    /// <p>If the encryption type is aws:kms, this optional value can be used to specify the encryption context for the restore results.</p>
    pub kms_context: Option<String>,
    /// <p>If the encryption type is aws:kms, this optional value specifies the AWS KMS key ID to use for encryption of job results.</p>
    pub kms_key_id: Option<String>,
}

pub struct EncryptionSerializer;
impl EncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Encryption,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("EncryptionType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.encryption_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.kms_context {
            writer.write(xml::writer::XmlEvent::start_element("KMSContext"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.kms_key_id {
            writer.write(xml::writer::XmlEvent::start_element("KMSKeyId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for information about the encryption-based configuration for replicas.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EncryptionConfiguration {
    /// <p>The ID of the AWS KMS key for the AWS Region where the destination bucket resides. Amazon S3 uses this key to encrypt the replica object. </p>
    pub replica_kms_key_id: Option<String>,
}

struct EncryptionConfigurationDeserializer;
impl EncryptionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EncryptionConfiguration, XmlParseError> {
        deserialize_elements::<_, EncryptionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicaKmsKeyID" => {
                        obj.replica_kms_key_id = Some(ReplicaKmsKeyIDDeserializer::deserialize(
                            "ReplicaKmsKeyID",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct EncryptionConfigurationSerializer;
impl EncryptionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &EncryptionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.replica_kms_key_id {
            writer.write(xml::writer::XmlEvent::start_element("ReplicaKmsKeyID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EndEvent {}

struct EndEventDeserializer;
impl EndEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EndEvent, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = EndEvent::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Error {
    /// <p><p/></p>
    pub code: Option<String>,
    /// <p><p/></p>
    pub key: Option<String>,
    /// <p><p/></p>
    pub message: Option<String>,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct S3ErrorDeserializer;
impl S3ErrorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Error, XmlParseError> {
        deserialize_elements::<_, S3Error, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Code" => {
                    obj.code = Some(CodeDeserializer::deserialize("Code", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "Message" => {
                    obj.message = Some(MessageDeserializer::deserialize("Message", stack)?);
                }
                "VersionId" => {
                    obj.version_id = Some(ObjectVersionIdDeserializer::deserialize(
                        "VersionId",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ErrorDocument {
    /// <p>The object key name to use when a 4XX class error occurs.</p>
    pub key: String,
}

struct ErrorDocumentDeserializer;
impl ErrorDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ErrorDocument, XmlParseError> {
        deserialize_elements::<_, ErrorDocument, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = ObjectKeyDeserializer::deserialize("Key", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ErrorDocumentSerializer;
impl ErrorDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ErrorDocument,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ErrorsDeserializer;
impl ErrorsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<S3Error>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(S3ErrorDeserializer::deserialize(tag_name, stack)?);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct EventSerializer;
impl EventSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(EventDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ExpirationStatusSerializer;
impl ExpirationStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExpiredObjectDeleteMarkerDeserializer;
impl ExpiredObjectDeleteMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ExpiredObjectDeleteMarkerSerializer;
impl ExpiredObjectDeleteMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExposeHeaderDeserializer;
impl ExposeHeaderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ExposeHeaderSerializer;
impl ExposeHeaderSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ExposeHeadersDeserializer;
impl ExposeHeadersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ExposeHeaderDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ExposeHeaderSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

pub struct ExpressionSerializer;
impl ExpressionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ExpressionTypeSerializer;
impl ExpressionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FetchOwnerSerializer;
impl FetchOwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FieldDelimiterSerializer;
impl FieldDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct FileHeaderInfoSerializer;
impl FileHeaderInfoSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for a key value pair that defines the criteria for the filter rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FilterRule {
    /// <p>The object key name prefix or suffix identifying one or more objects to which the filtering rule applies. The maximum prefix length is 1,024 characters. Overlapping prefixes and suffixes are not supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub name: Option<String>,
    /// <p><p/></p>
    pub value: Option<String>,
}

struct FilterRuleDeserializer;
impl FilterRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FilterRule, XmlParseError> {
        deserialize_elements::<_, FilterRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = Some(FilterRuleNameDeserializer::deserialize("Name", stack)?);
                }
                "Value" => {
                    obj.value = Some(FilterRuleValueDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct FilterRuleSerializer;
impl FilterRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &FilterRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.name {
            writer.write(xml::writer::XmlEvent::start_element("Name"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FilterRuleListDeserializer;
impl FilterRuleListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<FilterRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(FilterRuleDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<FilterRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct FilterRuleNameSerializer;
impl FilterRuleNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct FilterRuleValueDeserializer;
impl FilterRuleValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct FilterRuleValueSerializer;
impl FilterRuleValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAccelerateConfigurationRequest {
    /// <p>Name of the bucket for which the accelerate configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAccelerateConfigurationResponse {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketAccelerateConfigurationResponseDeserializer;
impl GetBucketAccelerateConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAccelerateConfigurationResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketAccelerateConfigurationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Status" => {
                        obj.status = Some(BucketAccelerateStatusDeserializer::deserialize(
                            "Status", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAclRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAclResponse {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
}

struct GetBucketAclResponseDeserializer;
impl GetBucketAclResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAclResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketAclResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AccessControlList" => {
                    obj.grants
                        .get_or_insert(vec![])
                        .extend(GrantsDeserializer::deserialize("AccessControlList", stack)?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationRequest {
    /// <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationResponse {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

struct GetBucketAnalyticsConfigurationResponseDeserializer;
impl GetBucketAnalyticsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAnalyticsConfigurationResponse, XmlParseError> {
        Ok(GetBucketAnalyticsConfigurationResponse {
            analytics_configuration: Some(AnalyticsConfigurationDeserializer::deserialize(
                "AnalyticsConfiguration",
                stack,
            )?),
            ..GetBucketAnalyticsConfigurationResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketCorsRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketCorsResponse {
    /// <p><p/></p>
    pub cors_rules: Option<Vec<CORSRule>>,
}

struct GetBucketCorsResponseDeserializer;
impl GetBucketCorsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketCorsResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketCorsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CORSRule" => {
                    obj.cors_rules
                        .get_or_insert(vec![])
                        .extend(CORSRulesDeserializer::deserialize("CORSRule", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionRequest {
    /// <p>The name of the bucket from which the server-side encryption configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionResponse {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

struct GetBucketEncryptionResponseDeserializer;
impl GetBucketEncryptionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketEncryptionResponse, XmlParseError> {
        Ok(GetBucketEncryptionResponse {
            server_side_encryption_configuration: Some(
                ServerSideEncryptionConfigurationDeserializer::deserialize(
                    "ServerSideEncryptionConfiguration",
                    stack,
                )?,
            ),
            ..GetBucketEncryptionResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket containing the inventory configuration to retrieve.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketInventoryConfigurationResponse {
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: Option<InventoryConfiguration>,
}

struct GetBucketInventoryConfigurationResponseDeserializer;
impl GetBucketInventoryConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketInventoryConfigurationResponse, XmlParseError> {
        Ok(GetBucketInventoryConfigurationResponse {
            inventory_configuration: Some(InventoryConfigurationDeserializer::deserialize(
                "InventoryConfiguration",
                stack,
            )?),
            ..GetBucketInventoryConfigurationResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleConfigurationResponse {
    /// <p><p/></p>
    pub rules: Option<Vec<LifecycleRule>>,
}

struct GetBucketLifecycleConfigurationResponseDeserializer;
impl GetBucketLifecycleConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleConfigurationResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleConfigurationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rule" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(LifecycleRulesDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleResponse {
    /// <p><p/></p>
    pub rules: Option<Vec<Rule>>,
}

struct GetBucketLifecycleResponseDeserializer;
impl GetBucketLifecycleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rule" => {
                        obj.rules
                            .get_or_insert(vec![])
                            .extend(RulesDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationResponse {
    /// <p><p/></p>
    pub location_constraint: Option<String>,
}

struct GetBucketLocationResponseDeserializer;
impl GetBucketLocationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLocationResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketLocationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LocationConstraint" => {
                        obj.location_constraint =
                            Some(BucketLocationConstraintDeserializer::deserialize(
                                "LocationConstraint",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingResponse {
    pub logging_enabled: Option<LoggingEnabled>,
}

struct GetBucketLoggingResponseDeserializer;
impl GetBucketLoggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLoggingResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketLoggingResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LoggingEnabled" => {
                        obj.logging_enabled = Some(LoggingEnabledDeserializer::deserialize(
                            "LoggingEnabled",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket containing the metrics configuration to retrieve.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketMetricsConfigurationResponse {
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: Option<MetricsConfiguration>,
}

struct GetBucketMetricsConfigurationResponseDeserializer;
impl GetBucketMetricsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketMetricsConfigurationResponse, XmlParseError> {
        Ok(GetBucketMetricsConfigurationResponse {
            metrics_configuration: Some(MetricsConfigurationDeserializer::deserialize(
                "MetricsConfiguration",
                stack,
            )?),
            ..GetBucketMetricsConfigurationResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketNotificationConfigurationRequest {
    /// <p>Name of the bucket to get the notification configuration for.</p>
    pub bucket: String,
}

/// <p>A container for specifying the notification configuration of the bucket. If this element is empty, notifications are turned off for the bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketNotificationConfigurationResponse {
    /// <p><p/></p>
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    /// <p><p/></p>
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    /// <p><p/></p>
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

struct GetBucketNotificationConfigurationResponseDeserializer;
impl GetBucketNotificationConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketNotificationConfigurationResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketNotificationConfigurationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CloudFunctionConfiguration" => {
                        obj.lambda_function_configurations
                            .get_or_insert(vec![])
                            .extend(LambdaFunctionConfigurationListDeserializer::deserialize(
                                "CloudFunctionConfiguration",
                                stack,
                            )?);
                    }
                    "QueueConfiguration" => {
                        obj.queue_configurations.get_or_insert(vec![]).extend(
                            QueueConfigurationListDeserializer::deserialize(
                                "QueueConfiguration",
                                stack,
                            )?,
                        );
                    }
                    "TopicConfiguration" => {
                        obj.topic_configurations.get_or_insert(vec![]).extend(
                            TopicConfigurationListDeserializer::deserialize(
                                "TopicConfiguration",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketNotificationRequest {
    /// <p>Name of the bucket to get the notification configuration for.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketNotificationResponse {
    /// <p><p/></p>
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
    /// <p><p/></p>
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    /// <p><p/></p>
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
}

struct GetBucketNotificationResponseDeserializer;
impl GetBucketNotificationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketNotificationResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketNotificationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CloudFunctionConfiguration" => {
                        obj.cloud_function_configuration =
                            Some(CloudFunctionConfigurationDeserializer::deserialize(
                                "CloudFunctionConfiguration",
                                stack,
                            )?);
                    }
                    "QueueConfiguration" => {
                        obj.queue_configuration =
                            Some(QueueConfigurationDeprecatedDeserializer::deserialize(
                                "QueueConfiguration",
                                stack,
                            )?);
                    }
                    "TopicConfiguration" => {
                        obj.topic_configuration =
                            Some(TopicConfigurationDeprecatedDeserializer::deserialize(
                                "TopicConfiguration",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyResponse {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyStatusRequest {
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyStatusResponse {
    /// <p>The policy status for the specified bucket.</p>
    pub policy_status: Option<PolicyStatus>,
}

struct GetBucketPolicyStatusResponseDeserializer;
impl GetBucketPolicyStatusResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyStatusResponse, XmlParseError> {
        Ok(GetBucketPolicyStatusResponse {
            policy_status: Some(PolicyStatusDeserializer::deserialize(
                "PolicyStatus",
                stack,
            )?),
            ..GetBucketPolicyStatusResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationResponse {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

struct GetBucketReplicationResponseDeserializer;
impl GetBucketReplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketReplicationResponse, XmlParseError> {
        Ok(GetBucketReplicationResponse {
            replication_configuration: Some(ReplicationConfigurationDeserializer::deserialize(
                "ReplicationConfiguration",
                stack,
            )?),
            ..GetBucketReplicationResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketRequestPaymentRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketRequestPaymentResponse {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Option<String>,
}

struct GetBucketRequestPaymentResponseDeserializer;
impl GetBucketRequestPaymentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketRequestPaymentResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketRequestPaymentResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Payer" => {
                        obj.payer = Some(PayerDeserializer::deserialize("Payer", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingResponse {
    /// <p><p/></p>
    pub tag_set: Vec<Tag>,
}

struct GetBucketTaggingResponseDeserializer;
impl GetBucketTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketTaggingResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketTaggingResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TagSet" => {
                        obj.tag_set
                            .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketVersioningRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketVersioningResponse {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<String>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketVersioningResponseDeserializer;
impl GetBucketVersioningResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketVersioningResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketVersioningResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MfaDelete" => {
                        obj.mfa_delete = Some(MFADeleteStatusDeserializer::deserialize(
                            "MfaDelete",
                            stack,
                        )?);
                    }
                    "Status" => {
                        obj.status = Some(BucketVersioningStatusDeserializer::deserialize(
                            "Status", stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteResponse {
    /// <p><p/></p>
    pub error_document: Option<ErrorDocument>,
    /// <p><p/></p>
    pub index_document: Option<IndexDocument>,
    /// <p><p/></p>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p><p/></p>
    pub routing_rules: Option<Vec<RoutingRule>>,
}

struct GetBucketWebsiteResponseDeserializer;
impl GetBucketWebsiteResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketWebsiteResponse, XmlParseError> {
        deserialize_elements::<_, GetBucketWebsiteResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ErrorDocument" => {
                        obj.error_document = Some(ErrorDocumentDeserializer::deserialize(
                            "ErrorDocument",
                            stack,
                        )?);
                    }
                    "IndexDocument" => {
                        obj.index_document = Some(IndexDocumentDeserializer::deserialize(
                            "IndexDocument",
                            stack,
                        )?);
                    }
                    "RedirectAllRequestsTo" => {
                        obj.redirect_all_requests_to =
                            Some(RedirectAllRequestsToDeserializer::deserialize(
                                "RedirectAllRequestsTo",
                                stack,
                            )?);
                    }
                    "RoutingRules" => {
                        obj.routing_rules.get_or_insert(vec![]).extend(
                            RoutingRulesDeserializer::deserialize("RoutingRules", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectAclRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectAclResponse {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    pub request_charged: Option<String>,
}

struct GetObjectAclResponseDeserializer;
impl GetObjectAclResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectAclResponse, XmlParseError> {
        deserialize_elements::<_, GetObjectAclResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AccessControlList" => {
                    obj.grants
                        .get_or_insert(vec![])
                        .extend(GrantsDeserializer::deserialize("AccessControlList", stack)?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectLegalHoldRequest {
    /// <p>The bucket containing the object whose Legal Hold status you want to retrieve.</p>
    pub bucket: String,
    /// <p>The key name for the object whose Legal Hold status you want to retrieve.</p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>The version ID of the object whose Legal Hold status you want to retrieve.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectLegalHoldResponse {
    /// <p>The current Legal Hold status for the specified object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
}

struct GetObjectLegalHoldResponseDeserializer;
impl GetObjectLegalHoldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLegalHoldResponse, XmlParseError> {
        Ok(GetObjectLegalHoldResponse {
            legal_hold: Some(ObjectLockLegalHoldDeserializer::deserialize(
                "LegalHold",
                stack,
            )?),
            ..GetObjectLegalHoldResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectLockConfigurationRequest {
    /// <p>The bucket whose Object Lock configuration you want to retrieve.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectLockConfigurationResponse {
    /// <p>The specified bucket's Object Lock configuration.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

struct GetObjectLockConfigurationResponseDeserializer;
impl GetObjectLockConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLockConfigurationResponse, XmlParseError> {
        Ok(GetObjectLockConfigurationResponse {
            object_lock_configuration: Some(ObjectLockConfigurationDeserializer::deserialize(
                "ObjectLockConfiguration",
                stack,
            )?),
            ..GetObjectLockConfigurationResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed).</p>
    pub if_match: Option<String>,
    /// <p>Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified).</p>
    pub if_modified_since: Option<String>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified).</p>
    pub if_none_match: Option<String>,
    /// <p>Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed).</p>
    pub if_unmodified_since: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub part_number: Option<i64>,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.</p>
    pub range: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Sets the Cache-Control header of the response.</p>
    pub response_cache_control: Option<String>,
    /// <p>Sets the Content-Disposition header of the response</p>
    pub response_content_disposition: Option<String>,
    /// <p>Sets the Content-Encoding header of the response.</p>
    pub response_content_encoding: Option<String>,
    /// <p>Sets the Content-Language header of the response.</p>
    pub response_content_language: Option<String>,
    /// <p>Sets the Content-Type header of the response.</p>
    pub response_content_type: Option<String>,
    /// <p>Sets the Expires header of the response.</p>
    pub response_expires: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug)]
pub struct GetObjectResponse {
    /// <p><p/></p>
    pub accept_ranges: Option<String>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>The portion of the object returned in the response.</p>
    pub content_range: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response.</p>
    pub delete_marker: Option<bool>,
    /// <p>An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Last modified date of the object</p>
    pub last_modified: Option<String>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers.</p>
    pub missing_meta: Option<i64>,
    /// <p>Indicates whether this object has an active legal hold. This field is only returned if you have permission to view an object's legal hold status.</p>
    pub object_lock_legal_hold_status: Option<String>,
    /// <p>The Object Lock mode currently in place for this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when this object's Object Lock will expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    /// <p>The count of parts this object has.</p>
    pub parts_count: Option<i64>,
    /// <p><p/></p>
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    /// <p>Provides information about object restoration operation and expiration time of the restored object copy.</p>
    pub restore: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p><p/></p>
    pub storage_class: Option<String>,
    /// <p>The number of tags, if any, on the object.</p>
    pub tag_count: Option<i64>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectRetentionRequest {
    /// <p>The bucket containing the object whose retention settings you want to retrieve.</p>
    pub bucket: String,
    /// <p>The key name for the object whose retention settings you want to retrieve.</p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>The version ID for the object whose retention settings you want to retrieve.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectRetentionResponse {
    /// <p>The container element for an object's retention settings.</p>
    pub retention: Option<ObjectLockRetention>,
}

struct GetObjectRetentionResponseDeserializer;
impl GetObjectRetentionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectRetentionResponse, XmlParseError> {
        Ok(GetObjectRetentionResponse {
            retention: Some(ObjectLockRetentionDeserializer::deserialize(
                "Retention",
                stack,
            )?),
            ..GetObjectRetentionResponse::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTaggingResponse {
    /// <p><p/></p>
    pub tag_set: Vec<Tag>,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct GetObjectTaggingResponseDeserializer;
impl GetObjectTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTaggingResponse, XmlParseError> {
        deserialize_elements::<_, GetObjectTaggingResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TagSet" => {
                        obj.tag_set
                            .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTorrentRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug)]
pub struct GetObjectTorrentResponse {
    /// <p><p/></p>
    pub body: Option<StreamingBody>,
    pub request_charged: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPublicAccessBlockRequest {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPublicAccessBlockResponse {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

struct GetPublicAccessBlockResponseDeserializer;
impl GetPublicAccessBlockResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicAccessBlockResponse, XmlParseError> {
        Ok(GetPublicAccessBlockResponse {
            public_access_block_configuration: Some(
                PublicAccessBlockConfigurationDeserializer::deserialize(
                    "PublicAccessBlockConfiguration",
                    stack,
                )?,
            ),
            ..GetPublicAccessBlockResponse::default()
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GlacierJobParameters {
    /// <p>Glacier retrieval tier at which the restore will be processed.</p>
    pub tier: String,
}

pub struct GlacierJobParametersSerializer;
impl GlacierJobParametersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &GlacierJobParameters,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Tier"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.tier
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Grant {
    /// <p><p/></p>
    pub grantee: Option<Grantee>,
    /// <p>Specifies the permission given to the grantee.</p>
    pub permission: Option<String>,
}

struct GrantDeserializer;
impl GrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grant, XmlParseError> {
        deserialize_elements::<_, Grant, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Grantee" => {
                    obj.grantee = Some(GranteeDeserializer::deserialize("Grantee", stack)?);
                }
                "Permission" => {
                    obj.permission =
                        Some(PermissionDeserializer::deserialize("Permission", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct GrantSerializer;
impl GrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Grant,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer.write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Grantee {
    /// <p>Screen name of the grantee.</p>
    pub display_name: Option<String>,
    /// <p>Email address of the grantee.</p>
    pub email_address: Option<String>,
    /// <p>The canonical user ID of the grantee.</p>
    pub id: Option<String>,
    /// <p>Type of grantee</p>
    pub type_: String,
    /// <p>URI of the grantee group.</p>
    pub uri: Option<String>,
}

struct GranteeDeserializer;
impl GranteeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Grantee, XmlParseError> {
        deserialize_elements::<_, Grantee, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DisplayName" => {
                    obj.display_name =
                        Some(DisplayNameDeserializer::deserialize("DisplayName", stack)?);
                }
                "EmailAddress" => {
                    obj.email_address = Some(EmailAddressDeserializer::deserialize(
                        "EmailAddress",
                        stack,
                    )?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                "xsi:type" => {
                    obj.type_ = TypeDeserializer::deserialize("xsi:type", stack)?;
                }
                "URI" => {
                    obj.uri = Some(URIDeserializer::deserialize("URI", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct GranteeSerializer;
impl GranteeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Grantee,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer.write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.email_address {
            writer.write(xml::writer::XmlEvent::start_element("EmailAddress"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("xsi:type"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.type_
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.uri {
            writer.write(xml::writer::XmlEvent::start_element("URI"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct GrantsDeserializer;
impl GrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Grant>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Grant" {
                obj.push(GrantDeserializer::deserialize("Grant", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct GrantsSerializer;
impl GrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Grant>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            GrantSerializer::serialize(writer, "Grant", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadBucketRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadBucketResponse {}

struct HeadBucketResponseDeserializer;
impl HeadBucketResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadBucketResponse, XmlParseError> {
        Ok(HeadBucketResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed).</p>
    pub if_match: Option<String>,
    /// <p>Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified).</p>
    pub if_modified_since: Option<String>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified).</p>
    pub if_none_match: Option<String>,
    /// <p>Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed).</p>
    pub if_unmodified_since: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' HEAD request for the part specified. Useful querying about the size of the part and the number of parts in this object.</p>
    pub part_number: Option<i64>,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, go to http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.</p>
    pub range: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HeadObjectResponse {
    /// <p><p/></p>
    pub accept_ranges: Option<String>,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes.</p>
    pub content_length: Option<i64>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response.</p>
    pub delete_marker: Option<bool>,
    /// <p>An ETag is an opaque identifier assigned by a web server to a specific version of a resource found at a URL</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the expiry-date and rule-id key value pairs providing object expiration information. The value of the rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Last modified date of the object</p>
    pub last_modified: Option<String>,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>This is set to the number of metadata entries not returned in x-amz-meta headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers.</p>
    pub missing_meta: Option<i64>,
    /// <p>The Legal Hold status for the specified object.</p>
    pub object_lock_legal_hold_status: Option<String>,
    /// <p>The Object Lock mode currently in place for this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when this object's Object Lock will expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    /// <p>The count of parts this object has.</p>
    pub parts_count: Option<i64>,
    /// <p><p/></p>
    pub replication_status: Option<String>,
    pub request_charged: Option<String>,
    /// <p>Provides information about object restoration operation and expiration time of the restored object copy.</p>
    pub restore: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p><p/></p>
    pub storage_class: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

struct HeadObjectResponseDeserializer;
impl HeadObjectResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadObjectResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = HeadObjectResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HostNameDeserializer;
impl HostNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct HostNameSerializer;
impl HostNameSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HttpErrorCodeReturnedEqualsDeserializer;
impl HttpErrorCodeReturnedEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct HttpErrorCodeReturnedEqualsSerializer;
impl HttpErrorCodeReturnedEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct HttpRedirectCodeDeserializer;
impl HttpRedirectCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct HttpRedirectCodeSerializer;
impl HttpRedirectCodeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IDDeserializer;
impl IDDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct IDSerializer;
impl IDSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IndexDocument {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (e.g. if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p>
    pub suffix: String,
}

struct IndexDocumentDeserializer;
impl IndexDocumentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IndexDocument, XmlParseError> {
        deserialize_elements::<_, IndexDocument, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Suffix" => {
                    obj.suffix = SuffixDeserializer::deserialize("Suffix", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct IndexDocumentSerializer;
impl IndexDocumentSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &IndexDocument,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Suffix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.suffix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InitiatedDeserializer;
impl InitiatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Initiator {
    /// <p>Name of the Principal.</p>
    pub display_name: Option<String>,
    /// <p>If the principal is an AWS account, it provides the Canonical User ID. If the principal is an IAM User, it provides a user ARN value.</p>
    pub id: Option<String>,
}

struct InitiatorDeserializer;
impl InitiatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Initiator, XmlParseError> {
        deserialize_elements::<_, Initiator, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DisplayName" => {
                    obj.display_name =
                        Some(DisplayNameDeserializer::deserialize("DisplayName", stack)?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Describes the serialization format of the object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InputSerialization {
    /// <p>Describes the serialization of a CSV-encoded object.</p>
    pub csv: Option<CSVInput>,
    /// <p>Specifies object's compression format. Valid values: NONE, GZIP, BZIP2. Default Value: NONE.</p>
    pub compression_type: Option<String>,
    /// <p>Specifies JSON as object's input serialization format.</p>
    pub json: Option<JSONInput>,
    /// <p>Specifies Parquet as object's input serialization format.</p>
    pub parquet: Option<ParquetInput>,
}

pub struct InputSerializationSerializer;
impl InputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVInputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.compression_type {
            writer.write(xml::writer::XmlEvent::start_element("CompressionType"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.json {
            &JSONInputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        if let Some(ref value) = obj.parquet {
            &ParquetInputSerializer::serialize(&mut writer, "Parquet", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryConfiguration {
    /// <p>Contains information about where to publish the inventory results.</p>
    pub destination: InventoryDestination,
    /// <p>Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria.</p>
    pub filter: Option<InventoryFilter>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
    /// <p>Specifies which object version(s) to included in the inventory results.</p>
    pub included_object_versions: String,
    /// <p>Specifies whether the inventory is enabled or disabled.</p>
    pub is_enabled: bool,
    /// <p>Contains the optional fields that are included in the inventory results.</p>
    pub optional_fields: Option<Vec<String>>,
    /// <p>Specifies the schedule for generating inventory results.</p>
    pub schedule: InventorySchedule,
}

struct InventoryConfigurationDeserializer;
impl InventoryConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryConfiguration, XmlParseError> {
        deserialize_elements::<_, InventoryConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Destination" => {
                    obj.destination =
                        InventoryDestinationDeserializer::deserialize("Destination", stack)?;
                }
                "Filter" => {
                    obj.filter = Some(InventoryFilterDeserializer::deserialize("Filter", stack)?);
                }
                "Id" => {
                    obj.id = InventoryIdDeserializer::deserialize("Id", stack)?;
                }
                "IncludedObjectVersions" => {
                    obj.included_object_versions =
                        InventoryIncludedObjectVersionsDeserializer::deserialize(
                            "IncludedObjectVersions",
                            stack,
                        )?;
                }
                "IsEnabled" => {
                    obj.is_enabled = IsEnabledDeserializer::deserialize("IsEnabled", stack)?;
                }
                "OptionalFields" => {
                    obj.optional_fields.get_or_insert(vec![]).extend(
                        InventoryOptionalFieldsDeserializer::deserialize("OptionalFields", stack)?,
                    );
                }
                "Schedule" => {
                    obj.schedule = InventoryScheduleDeserializer::deserialize("Schedule", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InventoryConfigurationSerializer;
impl InventoryConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryDestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.filter {
            &InventoryFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element(
            "IncludedObjectVersions",
        ))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.included_object_versions
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("IsEnabled"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.is_enabled
        )))?;
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<InventoryConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(InventoryConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

struct InventoryDestinationDeserializer;
impl InventoryDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryDestination, XmlParseError> {
        deserialize_elements::<_, InventoryDestination, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "S3BucketDestination" => {
                    obj.s3_bucket_destination =
                        InventoryS3BucketDestinationDeserializer::deserialize(
                            "S3BucketDestination",
                            stack,
                        )?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InventoryDestinationSerializer;
impl InventoryDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        InventoryS3BucketDestinationSerializer::serialize(
            &mut writer,
            "S3BucketDestination",
            &obj.s3_bucket_destination,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Contains the type of server-side encryption used to encrypt the inventory results.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryEncryption {
    /// <p>Specifies the use of SSE-KMS to encrypt delivered Inventory reports.</p>
    pub ssekms: Option<SSEKMS>,
    /// <p>Specifies the use of SSE-S3 to encrypt delivered Inventory reports.</p>
    pub sses3: Option<SSES3>,
}

struct InventoryEncryptionDeserializer;
impl InventoryEncryptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryEncryption, XmlParseError> {
        deserialize_elements::<_, InventoryEncryption, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "SSE-KMS" => {
                    obj.ssekms = Some(SSEKMSDeserializer::deserialize("SSE-KMS", stack)?);
                }
                "SSE-S3" => {
                    obj.sses3 = Some(SSES3Deserializer::deserialize("SSE-S3", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InventoryEncryptionSerializer;
impl InventoryEncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryEncryption,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.ssekms {
            &SSEKMSSerializer::serialize(&mut writer, "SSE-KMS", value)?;
        }
        if let Some(ref value) = obj.sses3 {
            &SSES3Serializer::serialize(&mut writer, "SSE-S3", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub prefix: String,
}

struct InventoryFilterDeserializer;
impl InventoryFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryFilter, XmlParseError> {
        deserialize_elements::<_, InventoryFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Prefix" => {
                    obj.prefix = PrefixDeserializer::deserialize("Prefix", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InventoryFilterSerializer;
impl InventoryFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryFormatDeserializer;
impl InventoryFormatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct InventoryFormatSerializer;
impl InventoryFormatSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryFrequencyDeserializer;
impl InventoryFrequencyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct InventoryFrequencySerializer;
impl InventoryFrequencySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryIdDeserializer;
impl InventoryIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct InventoryIdSerializer;
impl InventoryIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryIncludedObjectVersionsDeserializer;
impl InventoryIncludedObjectVersionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct InventoryIncludedObjectVersionsSerializer;
impl InventoryIncludedObjectVersionsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryOptionalFieldDeserializer;
impl InventoryOptionalFieldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct InventoryOptionalFieldSerializer;
impl InventoryOptionalFieldSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct InventoryOptionalFieldsDeserializer;
impl InventoryOptionalFieldsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Field" {
                obj.push(InventoryOptionalFieldDeserializer::deserialize(
                    "Field", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct InventoryOptionalFieldsSerializer;
impl InventoryOptionalFieldsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<String>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            InventoryOptionalFieldSerializer::serialize(writer, "Field", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryS3BucketDestination {
    /// <p>The ID of the account that owns the destination bucket.</p>
    pub account_id: Option<String>,
    /// <p>The Amazon resource name (ARN) of the bucket where inventory results will be published.</p>
    pub bucket: String,
    /// <p>Contains the type of server-side encryption used to encrypt the inventory results.</p>
    pub encryption: Option<InventoryEncryption>,
    /// <p>Specifies the output format of the inventory results.</p>
    pub format: String,
    /// <p>The prefix that is prepended to all inventory results.</p>
    pub prefix: Option<String>,
}

struct InventoryS3BucketDestinationDeserializer;
impl InventoryS3BucketDestinationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventoryS3BucketDestination, XmlParseError> {
        deserialize_elements::<_, InventoryS3BucketDestination, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AccountId" => {
                        obj.account_id =
                            Some(AccountIdDeserializer::deserialize("AccountId", stack)?);
                    }
                    "Bucket" => {
                        obj.bucket = BucketNameDeserializer::deserialize("Bucket", stack)?;
                    }
                    "Encryption" => {
                        obj.encryption = Some(InventoryEncryptionDeserializer::deserialize(
                            "Encryption",
                            stack,
                        )?);
                    }
                    "Format" => {
                        obj.format = InventoryFormatDeserializer::deserialize("Format", stack)?;
                    }
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct InventoryS3BucketDestinationSerializer;
impl InventoryS3BucketDestinationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventoryS3BucketDestination,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.account_id {
            writer.write(xml::writer::XmlEvent::start_element("AccountId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Bucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.encryption {
            &InventoryEncryptionSerializer::serialize(&mut writer, "Encryption", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Format"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.format
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub frequency: String,
}

struct InventoryScheduleDeserializer;
impl InventoryScheduleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InventorySchedule, XmlParseError> {
        deserialize_elements::<_, InventorySchedule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Frequency" => {
                    obj.frequency =
                        InventoryFrequencyDeserializer::deserialize("Frequency", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct InventoryScheduleSerializer;
impl InventoryScheduleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InventorySchedule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Frequency"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.frequency
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IsEnabledDeserializer;
impl IsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct IsEnabledSerializer;
impl IsEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct IsLatestDeserializer;
impl IsLatestDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IsPublicDeserializer;
impl IsPublicDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct IsTruncatedDeserializer;
impl IsTruncatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JSONInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub type_: Option<String>,
}

pub struct JSONInputSerializer;
impl JSONInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.type_ {
            writer.write(xml::writer::XmlEvent::start_element("Type"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JSONOutput {
    /// <p>The value used to separate individual records in the output.</p>
    pub record_delimiter: Option<String>,
}

pub struct JSONOutputSerializer;
impl JSONOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.record_delimiter {
            writer.write(xml::writer::XmlEvent::start_element("RecordDelimiter"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct JSONTypeSerializer;
impl JSONTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct KMSContextSerializer;
impl KMSContextSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct KeyCountDeserializer;
impl KeyCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct KeyMarkerDeserializer;
impl KeyMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct KeyMarkerSerializer;
impl KeyMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct KeyPrefixEqualsDeserializer;
impl KeyPrefixEqualsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct KeyPrefixEqualsSerializer;
impl KeyPrefixEqualsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionArnDeserializer;
impl LambdaFunctionArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct LambdaFunctionArnSerializer;
impl LambdaFunctionArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for specifying the configuration for AWS Lambda notifications.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LambdaFunctionConfiguration {
    /// <p><p/></p>
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Lambda cloud function that Amazon S3 can invoke when it detects events of the specified type.</p>
    pub lambda_function_arn: String,
}

struct LambdaFunctionConfigurationDeserializer;
impl LambdaFunctionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LambdaFunctionConfiguration, XmlParseError> {
        deserialize_elements::<_, LambdaFunctionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Event" => {
                        obj.events
                            .extend(EventListDeserializer::deserialize("Event", stack)?);
                    }
                    "Filter" => {
                        obj.filter =
                            Some(NotificationConfigurationFilterDeserializer::deserialize(
                                "Filter", stack,
                            )?);
                    }
                    "Id" => {
                        obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                    }
                    "CloudFunction" => {
                        obj.lambda_function_arn =
                            LambdaFunctionArnDeserializer::deserialize("CloudFunction", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct LambdaFunctionConfigurationSerializer;
impl LambdaFunctionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LambdaFunctionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("CloudFunction"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.lambda_function_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct LambdaFunctionConfigurationListDeserializer;
impl LambdaFunctionConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LambdaFunctionConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(LambdaFunctionConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<LambdaFunctionConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleConfiguration {
    /// <p><p/></p>
    pub rules: Vec<Rule>,
}

pub struct LifecycleConfigurationSerializer;
impl LifecycleConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        RulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleExpiration {
    /// <p>Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format.</p>
    pub date: Option<String>,
    /// <p>Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer.</p>
    pub days: Option<i64>,
    /// <p>Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If set to true, the delete marker will be expired; if set to false the policy takes no action. This cannot be specified with Days or Date in a Lifecycle Expiration Policy.</p>
    pub expired_object_delete_marker: Option<bool>,
}

struct LifecycleExpirationDeserializer;
impl LifecycleExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleExpiration, XmlParseError> {
        deserialize_elements::<_, LifecycleExpiration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Date" => {
                    obj.date = Some(DateDeserializer::deserialize("Date", stack)?);
                }
                "Days" => {
                    obj.days = Some(DaysDeserializer::deserialize("Days", stack)?);
                }
                "ExpiredObjectDeleteMarker" => {
                    obj.expired_object_delete_marker =
                        Some(ExpiredObjectDeleteMarkerDeserializer::deserialize(
                            "ExpiredObjectDeleteMarker",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct LifecycleExpirationSerializer;
impl LifecycleExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleExpiration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.expired_object_delete_marker {
            writer.write(xml::writer::XmlEvent::start_element(
                "ExpiredObjectDeleteMarker",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// <p><p/></p>
    pub expiration: Option<LifecycleExpiration>,
    pub filter: Option<LifecycleRuleFilter>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    /// <p><p/></p>
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub status: String,
    /// <p><p/></p>
    pub transitions: Option<Vec<Transition>>,
}

struct LifecycleRuleDeserializer;
impl LifecycleRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRule, XmlParseError> {
        deserialize_elements::<_, LifecycleRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AbortIncompleteMultipartUpload" => {
                    obj.abort_incomplete_multipart_upload =
                        Some(AbortIncompleteMultipartUploadDeserializer::deserialize(
                            "AbortIncompleteMultipartUpload",
                            stack,
                        )?);
                }
                "Expiration" => {
                    obj.expiration = Some(LifecycleExpirationDeserializer::deserialize(
                        "Expiration",
                        stack,
                    )?);
                }
                "Filter" => {
                    obj.filter = Some(LifecycleRuleFilterDeserializer::deserialize(
                        "Filter", stack,
                    )?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                "NoncurrentVersionExpiration" => {
                    obj.noncurrent_version_expiration =
                        Some(NoncurrentVersionExpirationDeserializer::deserialize(
                            "NoncurrentVersionExpiration",
                            stack,
                        )?);
                }
                "NoncurrentVersionTransition" => {
                    obj.noncurrent_version_transitions
                        .get_or_insert(vec![])
                        .extend(NoncurrentVersionTransitionListDeserializer::deserialize(
                            "NoncurrentVersionTransition",
                            stack,
                        )?);
                }
                "Status" => {
                    obj.status = ExpirationStatusDeserializer::deserialize("Status", stack)?;
                }
                "Transition" => {
                    obj.transitions.get_or_insert(vec![]).extend(
                        TransitionListDeserializer::deserialize("Transition", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct LifecycleRuleSerializer;
impl LifecycleRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(
                &mut writer,
                "AbortIncompleteMultipartUpload",
                value,
            )?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.filter {
            &LifecycleRuleFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(
                &mut writer,
                "NoncurrentVersionExpiration",
                value,
            )?;
        }
        if let Some(ref value) = obj.noncurrent_version_transitions {
            &NoncurrentVersionTransitionListSerializer::serialize(
                &mut writer,
                "NoncurrentVersionTransition",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.transitions {
            &TransitionListSerializer::serialize(&mut writer, "Transition", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>This is used in a Lifecycle Rule Filter to apply a logical AND to two or more predicates. The Lifecycle Rule will apply to any object matching all of the predicates configured inside the And operator.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRuleAndOperator {
    /// <p><p/></p>
    pub prefix: Option<String>,
    /// <p>All of these tags must exist in the object's tag set in order for the rule to apply.</p>
    pub tags: Option<Vec<Tag>>,
}

struct LifecycleRuleAndOperatorDeserializer;
impl LifecycleRuleAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleAndOperator, XmlParseError> {
        deserialize_elements::<_, LifecycleRuleAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    "Tag" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tag", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct LifecycleRuleAndOperatorSerializer;
impl LifecycleRuleAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRuleAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The Filter is used to identify objects that a Lifecycle Rule applies to. A Filter must have exactly one of Prefix, Tag, or And specified.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LifecycleRuleFilter {
    pub and: Option<LifecycleRuleAndOperator>,
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    pub prefix: Option<String>,
    /// <p>This tag must exist in the object's tag set in order for the rule to apply.</p>
    pub tag: Option<Tag>,
}

struct LifecycleRuleFilterDeserializer;
impl LifecycleRuleFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LifecycleRuleFilter, XmlParseError> {
        deserialize_elements::<_, LifecycleRuleFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "And" => {
                    obj.and = Some(LifecycleRuleAndOperatorDeserializer::deserialize(
                        "And", stack,
                    )?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct LifecycleRuleFilterSerializer;
impl LifecycleRuleFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LifecycleRuleFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &LifecycleRuleAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LifecycleRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(LifecycleRuleDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<LifecycleRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            LifecycleRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    /// <p>The name of the bucket from which analytics configurations are retrieved.</p>
    pub bucket: String,
    /// <p>The ContinuationToken that represents a placeholder from where this request should begin.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketAnalyticsConfigurationsResponse {
    /// <p>The list of analytics configurations for a bucket.</p>
    pub analytics_configuration_list: Option<Vec<AnalyticsConfiguration>>,
    /// <p>The ContinuationToken that represents where this request began.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of analytics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>NextContinuationToken is sent when isTruncated is true, which indicates that there are more analytics configurations to list. The next request must include this NextContinuationToken. The token is obfuscated and is not a usable value.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketAnalyticsConfigurationsResponseDeserializer;
impl ListBucketAnalyticsConfigurationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketAnalyticsConfigurationsResponse, XmlParseError> {
        deserialize_elements::<_, ListBucketAnalyticsConfigurationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AnalyticsConfiguration" => {
                        obj.analytics_configuration_list
                            .get_or_insert(vec![])
                            .extend(AnalyticsConfigurationListDeserializer::deserialize(
                                "AnalyticsConfiguration",
                                stack,
                            )?);
                    }
                    "ContinuationToken" => {
                        obj.continuation_token =
                            Some(TokenDeserializer::deserialize("ContinuationToken", stack)?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(NextTokenDeserializer::deserialize(
                            "NextContinuationToken",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketInventoryConfigurationsRequest {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketInventoryConfigurationsResponse {
    /// <p>If sent in the request, the marker that is used as a starting point for this inventory configuration list response.</p>
    pub continuation_token: Option<String>,
    /// <p>The list of inventory configurations for a bucket.</p>
    pub inventory_configuration_list: Option<Vec<InventoryConfiguration>>,
    /// <p>Indicates whether the returned list of inventory configurations is truncated in this response. A value of true indicates that the list is truncated.</p>
    pub is_truncated: Option<bool>,
    /// <p>The marker used to continue this inventory configuration listing. Use the NextContinuationToken from this response to continue the listing in a subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketInventoryConfigurationsResponseDeserializer;
impl ListBucketInventoryConfigurationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketInventoryConfigurationsResponse, XmlParseError> {
        deserialize_elements::<_, ListBucketInventoryConfigurationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ContinuationToken" => {
                        obj.continuation_token =
                            Some(TokenDeserializer::deserialize("ContinuationToken", stack)?);
                    }
                    "InventoryConfiguration" => {
                        obj.inventory_configuration_list
                            .get_or_insert(vec![])
                            .extend(InventoryConfigurationListDeserializer::deserialize(
                                "InventoryConfiguration",
                                stack,
                            )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(NextTokenDeserializer::deserialize(
                            "NextContinuationToken",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketMetricsConfigurationsRequest {
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketMetricsConfigurationsResponse {
    /// <p>The marker that is used as a starting point for this metrics configuration list response. This value is present if it was sent in the request.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of metrics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>The list of metrics configurations for a bucket.</p>
    pub metrics_configuration_list: Option<Vec<MetricsConfiguration>>,
    /// <p>The marker used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketMetricsConfigurationsResponseDeserializer;
impl ListBucketMetricsConfigurationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketMetricsConfigurationsResponse, XmlParseError> {
        deserialize_elements::<_, ListBucketMetricsConfigurationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ContinuationToken" => {
                        obj.continuation_token =
                            Some(TokenDeserializer::deserialize("ContinuationToken", stack)?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                    }
                    "MetricsConfiguration" => {
                        obj.metrics_configuration_list.get_or_insert(vec![]).extend(
                            MetricsConfigurationListDeserializer::deserialize(
                                "MetricsConfiguration",
                                stack,
                            )?,
                        );
                    }
                    "NextContinuationToken" => {
                        obj.next_continuation_token = Some(NextTokenDeserializer::deserialize(
                            "NextContinuationToken",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketsRequest {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketsResponse {
    /// <p><p/></p>
    pub buckets: Option<Vec<Bucket>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
}

struct ListBucketsResponseDeserializer;
impl ListBucketsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketsResponse, XmlParseError> {
        deserialize_elements::<_, ListBucketsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Buckets" => {
                    obj.buckets
                        .get_or_insert(vec![])
                        .extend(BucketsDeserializer::deserialize("Buckets", stack)?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMultipartUploadsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Together with upload-id-marker, this parameter specifies the multipart upload after which listing should begin.</p>
    pub key_marker: Option<String>,
    /// <p>Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response.</p>
    pub max_uploads: Option<i64>,
    /// <p>Lists in-progress uploads only for those keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored.</p>
    pub upload_id_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMultipartUploadsResponse {
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    /// <p><p/></p>
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// <p><p/></p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub is_truncated: Option<bool>,
    /// <p>The key at or after which the listing began.</p>
    pub key_marker: Option<String>,
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub max_uploads: Option<i64>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub next_key_marker: Option<String>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the upload-id-marker request parameter in a subsequent request.</p>
    pub next_upload_id_marker: Option<String>,
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Upload ID after which listing began.</p>
    pub upload_id_marker: Option<String>,
    /// <p><p/></p>
    pub uploads: Option<Vec<MultipartUpload>>,
}

struct ListMultipartUploadsResponseDeserializer;
impl ListMultipartUploadsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMultipartUploadsResponse, XmlParseError> {
        deserialize_elements::<_, ListMultipartUploadsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Bucket" => {
                        obj.bucket = Some(BucketNameDeserializer::deserialize("Bucket", stack)?);
                    }
                    "CommonPrefixes" => {
                        obj.common_prefixes.get_or_insert(vec![]).extend(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?,
                        );
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(DelimiterDeserializer::deserialize("Delimiter", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack,
                        )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                    }
                    "KeyMarker" => {
                        obj.key_marker =
                            Some(KeyMarkerDeserializer::deserialize("KeyMarker", stack)?);
                    }
                    "MaxUploads" => {
                        obj.max_uploads =
                            Some(MaxUploadsDeserializer::deserialize("MaxUploads", stack)?);
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(NextKeyMarkerDeserializer::deserialize(
                            "NextKeyMarker",
                            stack,
                        )?);
                    }
                    "NextUploadIdMarker" => {
                        obj.next_upload_id_marker =
                            Some(NextUploadIdMarkerDeserializer::deserialize(
                                "NextUploadIdMarker",
                                stack,
                            )?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    "UploadIdMarker" => {
                        obj.upload_id_marker = Some(UploadIdMarkerDeserializer::deserialize(
                            "UploadIdMarker",
                            stack,
                        )?);
                    }
                    "Upload" => {
                        obj.uploads.get_or_insert(vec![]).extend(
                            MultipartUploadListDeserializer::deserialize("Upload", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectVersionsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub key_marker: Option<String>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Specifies the object version you want to start listing from.</p>
    pub version_id_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectVersionsResponse {
    /// <p><p/></p>
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// <p><p/></p>
    pub delete_markers: Option<Vec<DeleteMarkerEntry>>,
    /// <p><p/></p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria. If your results were truncated, you can make a follow-up paginated request using the NextKeyMarker and NextVersionIdMarker response parameters as a starting place in another request to return the rest of the results.</p>
    pub is_truncated: Option<bool>,
    /// <p>Marks the last Key returned in a truncated response.</p>
    pub key_marker: Option<String>,
    /// <p><p/></p>
    pub max_keys: Option<i64>,
    /// <p><p/></p>
    pub name: Option<String>,
    /// <p>Use this value for the key marker request parameter in a subsequent request.</p>
    pub next_key_marker: Option<String>,
    /// <p>Use this value for the next version id marker parameter in a subsequent request.</p>
    pub next_version_id_marker: Option<String>,
    /// <p><p/></p>
    pub prefix: Option<String>,
    /// <p><p/></p>
    pub version_id_marker: Option<String>,
    /// <p><p/></p>
    pub versions: Option<Vec<ObjectVersion>>,
}

struct ListObjectVersionsResponseDeserializer;
impl ListObjectVersionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectVersionsResponse, XmlParseError> {
        deserialize_elements::<_, ListObjectVersionsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CommonPrefixes" => {
                        obj.common_prefixes.get_or_insert(vec![]).extend(
                            CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?,
                        );
                    }
                    "DeleteMarker" => {
                        obj.delete_markers.get_or_insert(vec![]).extend(
                            DeleteMarkersDeserializer::deserialize("DeleteMarker", stack)?,
                        );
                    }
                    "Delimiter" => {
                        obj.delimiter =
                            Some(DelimiterDeserializer::deserialize("Delimiter", stack)?);
                    }
                    "EncodingType" => {
                        obj.encoding_type = Some(EncodingTypeDeserializer::deserialize(
                            "EncodingType",
                            stack,
                        )?);
                    }
                    "IsTruncated" => {
                        obj.is_truncated =
                            Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                    }
                    "KeyMarker" => {
                        obj.key_marker =
                            Some(KeyMarkerDeserializer::deserialize("KeyMarker", stack)?);
                    }
                    "MaxKeys" => {
                        obj.max_keys = Some(MaxKeysDeserializer::deserialize("MaxKeys", stack)?);
                    }
                    "Name" => {
                        obj.name = Some(BucketNameDeserializer::deserialize("Name", stack)?);
                    }
                    "NextKeyMarker" => {
                        obj.next_key_marker = Some(NextKeyMarkerDeserializer::deserialize(
                            "NextKeyMarker",
                            stack,
                        )?);
                    }
                    "NextVersionIdMarker" => {
                        obj.next_version_id_marker =
                            Some(NextVersionIdMarkerDeserializer::deserialize(
                                "NextVersionIdMarker",
                                stack,
                            )?);
                    }
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    "VersionIdMarker" => {
                        obj.version_id_marker = Some(VersionIdMarkerDeserializer::deserialize(
                            "VersionIdMarker",
                            stack,
                        )?);
                    }
                    "Version" => {
                        obj.versions.get_or_insert(vec![]).extend(
                            ObjectVersionListDeserializer::deserialize("Version", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    pub encoding_type: Option<String>,
    /// <p>Specifies the key to start with when listing objects in a bucket.</p>
    pub marker: Option<String>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsResponse {
    /// <p><p/></p>
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// <p><p/></p>
    pub contents: Option<Vec<Object>>,
    /// <p><p/></p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria.</p>
    pub is_truncated: Option<bool>,
    /// <p><p/></p>
    pub marker: Option<String>,
    /// <p><p/></p>
    pub max_keys: Option<i64>,
    /// <p><p/></p>
    pub name: Option<String>,
    /// <p>When response is truncated (the IsTruncated element value in the response is true), you can use the key name in this field as marker in the subsequent request to get next set of objects. Amazon S3 lists objects in alphabetical order Note: This element is returned only if you have delimiter request parameter specified. If response does not include the NextMaker and it is truncated, you can use the value of the last Key in the response as the marker in the subsequent request to get the next set of object keys.</p>
    pub next_marker: Option<String>,
    /// <p><p/></p>
    pub prefix: Option<String>,
}

struct ListObjectsResponseDeserializer;
impl ListObjectsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsResponse, XmlParseError> {
        deserialize_elements::<_, ListObjectsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CommonPrefixes" => {
                    obj.common_prefixes.get_or_insert(vec![]).extend(
                        CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?,
                    );
                }
                "Contents" => {
                    obj.contents
                        .get_or_insert(vec![])
                        .extend(ObjectListDeserializer::deserialize("Contents", stack)?);
                }
                "Delimiter" => {
                    obj.delimiter = Some(DelimiterDeserializer::deserialize("Delimiter", stack)?);
                }
                "EncodingType" => {
                    obj.encoding_type = Some(EncodingTypeDeserializer::deserialize(
                        "EncodingType",
                        stack,
                    )?);
                }
                "IsTruncated" => {
                    obj.is_truncated =
                        Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                }
                "Marker" => {
                    obj.marker = Some(MarkerDeserializer::deserialize("Marker", stack)?);
                }
                "MaxKeys" => {
                    obj.max_keys = Some(MaxKeysDeserializer::deserialize("MaxKeys", stack)?);
                }
                "Name" => {
                    obj.name = Some(BucketNameDeserializer::deserialize("Name", stack)?);
                }
                "NextMarker" => {
                    obj.next_marker =
                        Some(NextMarkerDeserializer::deserialize("NextMarker", stack)?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsV2Request {
    /// <p>Name of the bucket to list.</p>
    pub bucket: String,
    /// <p>ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key</p>
    pub continuation_token: Option<String>,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>The owner field is not present in listV2 by default, if you want to return owner field with each key in the result then set the fetch owner field to true</p>
    pub fetch_owner: Option<bool>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request in V2 style. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: Option<String>,
    /// <p>StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket</p>
    pub start_after: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListObjectsV2Response {
    /// <p>CommonPrefixes contains all (if there are any) keys between Prefix and the next occurrence of the string specified by delimiter</p>
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    /// <p>Metadata about each object returned.</p>
    pub contents: Option<Vec<Object>>,
    /// <p>ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key</p>
    pub continuation_token: Option<String>,
    /// <p>A delimiter is a character you use to group keys.</p>
    pub delimiter: Option<String>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    pub encoding_type: Option<String>,
    /// <p>A flag that indicates whether or not Amazon S3 returned all of the results that satisfied the search criteria.</p>
    pub is_truncated: Option<bool>,
    /// <p>KeyCount is the number of keys returned with this request. KeyCount will always be less than equals to MaxKeys field. Say you ask for 50 keys, your result will include less than equals 50 keys </p>
    pub key_count: Option<i64>,
    /// <p>Sets the maximum number of keys returned in the response. The response might contain fewer keys but will never contain more.</p>
    pub max_keys: Option<i64>,
    /// <p>Name of the bucket to list.</p>
    pub name: Option<String>,
    /// <p>NextContinuationToken is sent when isTruncated is true which means there are more keys in the bucket that can be listed. The next list requests to Amazon S3 can be continued with this NextContinuationToken. NextContinuationToken is obfuscated and is not a real key</p>
    pub next_continuation_token: Option<String>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: Option<String>,
    /// <p>StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket</p>
    pub start_after: Option<String>,
}

struct ListObjectsV2ResponseDeserializer;
impl ListObjectsV2ResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsV2Response, XmlParseError> {
        deserialize_elements::<_, ListObjectsV2Response, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CommonPrefixes" => {
                    obj.common_prefixes.get_or_insert(vec![]).extend(
                        CommonPrefixListDeserializer::deserialize("CommonPrefixes", stack)?,
                    );
                }
                "Contents" => {
                    obj.contents
                        .get_or_insert(vec![])
                        .extend(ObjectListDeserializer::deserialize("Contents", stack)?);
                }
                "ContinuationToken" => {
                    obj.continuation_token =
                        Some(TokenDeserializer::deserialize("ContinuationToken", stack)?);
                }
                "Delimiter" => {
                    obj.delimiter = Some(DelimiterDeserializer::deserialize("Delimiter", stack)?);
                }
                "EncodingType" => {
                    obj.encoding_type = Some(EncodingTypeDeserializer::deserialize(
                        "EncodingType",
                        stack,
                    )?);
                }
                "IsTruncated" => {
                    obj.is_truncated =
                        Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                }
                "KeyCount" => {
                    obj.key_count = Some(KeyCountDeserializer::deserialize("KeyCount", stack)?);
                }
                "MaxKeys" => {
                    obj.max_keys = Some(MaxKeysDeserializer::deserialize("MaxKeys", stack)?);
                }
                "Name" => {
                    obj.name = Some(BucketNameDeserializer::deserialize("Name", stack)?);
                }
                "NextContinuationToken" => {
                    obj.next_continuation_token = Some(NextTokenDeserializer::deserialize(
                        "NextContinuationToken",
                        stack,
                    )?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "StartAfter" => {
                    obj.start_after =
                        Some(StartAfterDeserializer::deserialize("StartAfter", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPartsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    /// <p>Sets the maximum number of parts to return.</p>
    pub max_parts: Option<i64>,
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub part_number_marker: Option<i64>,
    pub request_payer: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPartsResponse {
    /// <p>Date when multipart upload will become eligible for abort operation by lifecycle.</p>
    pub abort_date: Option<String>,
    /// <p>Id of the lifecycle rule that makes a multipart upload eligible for abort operation.</p>
    pub abort_rule_id: Option<String>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: Option<String>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: Option<Initiator>,
    /// <p>Indicates whether the returned list of parts is truncated.</p>
    pub is_truncated: Option<bool>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    /// <p>Maximum number of parts that were allowed in the response.</p>
    pub max_parts: Option<i64>,
    /// <p>When a list is truncated, this element specifies the last part in the list, as well as the value to use for the part-number-marker request parameter in a subsequent request.</p>
    pub next_part_number_marker: Option<i64>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    /// <p>Part number after which listing begins.</p>
    pub part_number_marker: Option<i64>,
    /// <p><p/></p>
    pub parts: Option<Vec<Part>>,
    pub request_charged: Option<String>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub upload_id: Option<String>,
}

struct ListPartsResponseDeserializer;
impl ListPartsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPartsResponse, XmlParseError> {
        deserialize_elements::<_, ListPartsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Bucket" => {
                    obj.bucket = Some(BucketNameDeserializer::deserialize("Bucket", stack)?);
                }
                "Initiator" => {
                    obj.initiator = Some(InitiatorDeserializer::deserialize("Initiator", stack)?);
                }
                "IsTruncated" => {
                    obj.is_truncated =
                        Some(IsTruncatedDeserializer::deserialize("IsTruncated", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "MaxParts" => {
                    obj.max_parts = Some(MaxPartsDeserializer::deserialize("MaxParts", stack)?);
                }
                "NextPartNumberMarker" => {
                    obj.next_part_number_marker =
                        Some(NextPartNumberMarkerDeserializer::deserialize(
                            "NextPartNumberMarker",
                            stack,
                        )?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                "PartNumberMarker" => {
                    obj.part_number_marker = Some(PartNumberMarkerDeserializer::deserialize(
                        "PartNumberMarker",
                        stack,
                    )?);
                }
                "Part" => {
                    obj.parts
                        .get_or_insert(vec![])
                        .extend(PartsDeserializer::deserialize("Part", stack)?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(StorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                "UploadId" => {
                    obj.upload_id = Some(MultipartUploadIdDeserializer::deserialize(
                        "UploadId", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LocationDeserializer;
impl LocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct LocationPrefixSerializer;
impl LocationPrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for logging information. Presence of this element indicates that logging is enabled. Parameters TargetBucket and TargetPrefix are required in this case.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case you should choose a different TargetPrefix for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub target_bucket: String,
    /// <p><p/></p>
    pub target_grants: Option<Vec<TargetGrant>>,
    /// <p>This element lets you specify a prefix for the keys that the log files will be stored under.</p>
    pub target_prefix: String,
}

struct LoggingEnabledDeserializer;
impl LoggingEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoggingEnabled, XmlParseError> {
        deserialize_elements::<_, LoggingEnabled, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TargetBucket" => {
                    obj.target_bucket =
                        TargetBucketDeserializer::deserialize("TargetBucket", stack)?;
                }
                "TargetGrants" => {
                    obj.target_grants.get_or_insert(vec![]).extend(
                        TargetGrantsDeserializer::deserialize("TargetGrants", stack)?,
                    );
                }
                "TargetPrefix" => {
                    obj.target_prefix =
                        TargetPrefixDeserializer::deserialize("TargetPrefix", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct LoggingEnabledSerializer;
impl LoggingEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &LoggingEnabled,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("TargetBucket"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_bucket
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.target_grants {
            &TargetGrantsSerializer::serialize(&mut writer, "TargetGrants", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("TargetPrefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.target_prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MFADeleteSerializer;
impl MFADeleteSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MFADeleteStatusDeserializer;
impl MFADeleteStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MarkerDeserializer;
impl MarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MarkerSerializer;
impl MarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxAgeSecondsDeserializer;
impl MaxAgeSecondsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MaxAgeSecondsSerializer;
impl MaxAgeSecondsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxKeysDeserializer;
impl MaxKeysDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MaxKeysSerializer;
impl MaxKeysSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxPartsDeserializer;
impl MaxPartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MaxPartsSerializer;
impl MaxPartsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MaxUploadsDeserializer;
impl MaxUploadsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MaxUploadsSerializer;
impl MaxUploadsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A metadata key-value pair to store with an object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetadataEntry {
    /// <p><p/></p>
    pub name: Option<String>,
    /// <p><p/></p>
    pub value: Option<String>,
}

pub struct MetadataEntrySerializer;
impl MetadataEntrySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetadataEntry,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.name {
            writer.write(xml::writer::XmlEvent::start_element("Name"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.value {
            writer.write(xml::writer::XmlEvent::start_element("Value"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MetadataKeySerializer;
impl MetadataKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct MetadataValueSerializer;
impl MetadataValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsAndOperator {
    /// <p>The prefix used when evaluating an AND predicate.</p>
    pub prefix: Option<String>,
    /// <p>The list of tags used when evaluating an AND predicate.</p>
    pub tags: Option<Vec<Tag>>,
}

struct MetricsAndOperatorDeserializer;
impl MetricsAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsAndOperator, XmlParseError> {
        deserialize_elements::<_, MetricsAndOperator, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagSetDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct MetricsAndOperatorSerializer;
impl MetricsAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsConfiguration {
    /// <p>Specifies a metrics configuration filter. The metrics configuration will only include objects that meet the filter's criteria. A filter must be a prefix, a tag, or a conjunction (MetricsAndOperator).</p>
    pub filter: Option<MetricsFilter>,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

struct MetricsConfigurationDeserializer;
impl MetricsConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsConfiguration, XmlParseError> {
        deserialize_elements::<_, MetricsConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Filter" => {
                    obj.filter = Some(MetricsFilterDeserializer::deserialize("Filter", stack)?);
                }
                "Id" => {
                    obj.id = MetricsIdDeserializer::deserialize("Id", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct MetricsConfigurationSerializer;
impl MetricsConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter {
            &MetricsFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Id"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MetricsConfigurationListDeserializer;
impl MetricsConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricsConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(MetricsConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricsFilter {
    /// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates, and an object must match all of the predicates in order for the filter to apply.</p>
    pub and: Option<MetricsAndOperator>,
    /// <p>The prefix used when evaluating a metrics filter.</p>
    pub prefix: Option<String>,
    /// <p>The tag used when evaluating a metrics filter.</p>
    pub tag: Option<Tag>,
}

struct MetricsFilterDeserializer;
impl MetricsFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricsFilter, XmlParseError> {
        deserialize_elements::<_, MetricsFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "And" => {
                    obj.and = Some(MetricsAndOperatorDeserializer::deserialize("And", stack)?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct MetricsFilterSerializer;
impl MetricsFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &MetricsFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &MetricsAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MetricsIdSerializer;
impl MetricsIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MultipartUpload {
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub initiated: Option<String>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: Option<Initiator>,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: Option<String>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: Option<String>,
}

struct MultipartUploadDeserializer;
impl MultipartUploadDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MultipartUpload, XmlParseError> {
        deserialize_elements::<_, MultipartUpload, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Initiated" => {
                    obj.initiated = Some(InitiatedDeserializer::deserialize("Initiated", stack)?);
                }
                "Initiator" => {
                    obj.initiator = Some(InitiatorDeserializer::deserialize("Initiator", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(StorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                "UploadId" => {
                    obj.upload_id = Some(MultipartUploadIdDeserializer::deserialize(
                        "UploadId", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MultipartUploadIdDeserializer;
impl MultipartUploadIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct MultipartUploadIdSerializer;
impl MultipartUploadIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct MultipartUploadListDeserializer;
impl MultipartUploadListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MultipartUpload>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(MultipartUploadDeserializer::deserialize(tag_name, stack)?);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextMarkerDeserializer;
impl NextMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextPartNumberMarkerDeserializer;
impl NextPartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextUploadIdMarkerDeserializer;
impl NextUploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextVersionIdMarkerDeserializer;
impl NextVersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Specifies when noncurrent object versions expire. Upon expiration, Amazon S3 permanently deletes the noncurrent object versions. You set this lifecycle configuration action on a bucket that has versioning enabled (or suspended) to request that Amazon S3 delete noncurrent object versions at a specific period in the object's lifetime.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NoncurrentVersionExpiration {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide.</p>
    pub noncurrent_days: Option<i64>,
}

struct NoncurrentVersionExpirationDeserializer;
impl NoncurrentVersionExpirationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionExpiration, XmlParseError> {
        deserialize_elements::<_, NoncurrentVersionExpiration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NoncurrentDays" => {
                        obj.noncurrent_days =
                            Some(DaysDeserializer::deserialize("NoncurrentDays", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct NoncurrentVersionExpirationSerializer;
impl NoncurrentVersionExpirationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NoncurrentVersionExpiration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer.write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for the transition rule that describes when noncurrent objects transition to the STANDARD_IA, ONEZONE_IA, INTELLIGENT_TIERING, GLACIER or DEEP_ARCHIVE storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to the STANDARD_IA, ONEZONE_IA, INTELLIGENT_TIERING, GLACIER or DEEP_ARCHIVE storage class at a specific period in the object's lifetime.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NoncurrentVersionTransition {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the Amazon Simple Storage Service Developer Guide.</p>
    pub noncurrent_days: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct NoncurrentVersionTransitionDeserializer;
impl NoncurrentVersionTransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NoncurrentVersionTransition, XmlParseError> {
        deserialize_elements::<_, NoncurrentVersionTransition, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NoncurrentDays" => {
                        obj.noncurrent_days =
                            Some(DaysDeserializer::deserialize("NoncurrentDays", stack)?);
                    }
                    "StorageClass" => {
                        obj.storage_class = Some(TransitionStorageClassDeserializer::deserialize(
                            "StorageClass",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct NoncurrentVersionTransitionSerializer;
impl NoncurrentVersionTransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NoncurrentVersionTransition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.noncurrent_days {
            writer.write(xml::writer::XmlEvent::start_element("NoncurrentDays"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct NoncurrentVersionTransitionListDeserializer;
impl NoncurrentVersionTransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NoncurrentVersionTransition>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(NoncurrentVersionTransitionDeserializer::deserialize(
                    tag_name, stack,
                )?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<NoncurrentVersionTransition>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            NoncurrentVersionTransitionSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>A container for specifying the notification configuration of the bucket. If this element is empty, notifications are turned off for the bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfiguration {
    /// <p><p/></p>
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    /// <p><p/></p>
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    /// <p><p/></p>
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

pub struct NotificationConfigurationSerializer;
impl NotificationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.lambda_function_configurations {
            &LambdaFunctionConfigurationListSerializer::serialize(
                &mut writer,
                "CloudFunctionConfiguration",
                value,
            )?;
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfigurationDeprecated {
    /// <p><p/></p>
    pub cloud_function_configuration: Option<CloudFunctionConfiguration>,
    /// <p><p/></p>
    pub queue_configuration: Option<QueueConfigurationDeprecated>,
    /// <p><p/></p>
    pub topic_configuration: Option<TopicConfigurationDeprecated>,
}

pub struct NotificationConfigurationDeprecatedSerializer;
impl NotificationConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.cloud_function_configuration {
            &CloudFunctionConfigurationSerializer::serialize(
                &mut writer,
                "CloudFunctionConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.queue_configuration {
            &QueueConfigurationDeprecatedSerializer::serialize(
                &mut writer,
                "QueueConfiguration",
                value,
            )?;
        }
        if let Some(ref value) = obj.topic_configuration {
            &TopicConfigurationDeprecatedSerializer::serialize(
                &mut writer,
                "TopicConfiguration",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

struct NotificationConfigurationFilterDeserializer;
impl NotificationConfigurationFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationFilter, XmlParseError> {
        deserialize_elements::<_, NotificationConfigurationFilter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "S3Key" => {
                        obj.key = Some(S3KeyFilterDeserializer::deserialize("S3Key", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct NotificationConfigurationFilterSerializer;
impl NotificationConfigurationFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &NotificationConfigurationFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct NotificationIdSerializer;
impl NotificationIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Object {
    /// <p><p/></p>
    pub e_tag: Option<String>,
    /// <p><p/></p>
    pub key: Option<String>,
    /// <p><p/></p>
    pub last_modified: Option<String>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    /// <p><p/></p>
    pub size: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct ObjectDeserializer;
impl ObjectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Object, XmlParseError> {
        deserialize_elements::<_, Object, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ETag" => {
                    obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                "Size" => {
                    obj.size = Some(SizeDeserializer::deserialize("Size", stack)?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(ObjectStorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ObjectCannedACLSerializer;
impl ObjectCannedACLSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectIdentifier {
    /// <p>Key name of the object to delete.</p>
    pub key: String,
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub version_id: Option<String>,
}

pub struct ObjectIdentifierSerializer;
impl ObjectIdentifierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectIdentifier,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.version_id {
            writer.write(xml::writer::XmlEvent::start_element("VersionId"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ObjectIdentifierListSerializer;
impl ObjectIdentifierListSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ObjectIdentifier>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ObjectKeySerializer;
impl ObjectKeySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectListDeserializer;
impl ObjectListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Object>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ObjectDeserializer::deserialize(tag_name, stack)?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}
/// <p>The container element for Object Lock configuration parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectLockConfiguration {
    /// <p>Indicates whether this bucket has an Object Lock configuration enabled.</p>
    pub object_lock_enabled: Option<String>,
    /// <p>The Object Lock rule in place for the specified object.</p>
    pub rule: Option<ObjectLockRule>,
}

struct ObjectLockConfigurationDeserializer;
impl ObjectLockConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockConfiguration, XmlParseError> {
        deserialize_elements::<_, ObjectLockConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ObjectLockEnabled" => {
                        obj.object_lock_enabled = Some(ObjectLockEnabledDeserializer::deserialize(
                            "ObjectLockEnabled",
                            stack,
                        )?);
                    }
                    "Rule" => {
                        obj.rule = Some(ObjectLockRuleDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ObjectLockConfigurationSerializer;
impl ObjectLockConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectLockConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.object_lock_enabled {
            writer.write(xml::writer::XmlEvent::start_element("ObjectLockEnabled"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.rule {
            &ObjectLockRuleSerializer::serialize(&mut writer, "Rule", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectLockEnabledDeserializer;
impl ObjectLockEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ObjectLockEnabledSerializer;
impl ObjectLockEnabledSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A Legal Hold configuration for an object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectLockLegalHold {
    /// <p>Indicates whether the specified object has a Legal Hold in place.</p>
    pub status: Option<String>,
}

struct ObjectLockLegalHoldDeserializer;
impl ObjectLockLegalHoldDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockLegalHold, XmlParseError> {
        deserialize_elements::<_, ObjectLockLegalHold, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Status" => {
                    obj.status = Some(ObjectLockLegalHoldStatusDeserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ObjectLockLegalHoldSerializer;
impl ObjectLockLegalHoldSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectLockLegalHold,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectLockLegalHoldStatusDeserializer;
impl ObjectLockLegalHoldStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ObjectLockLegalHoldStatusSerializer;
impl ObjectLockLegalHoldStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A Retention configuration for an object.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectLockRetention {
    /// <p>Indicates the Retention mode for the specified object.</p>
    pub mode: Option<String>,
    /// <p>The date on which this Object Lock Retention will expire.</p>
    pub retain_until_date: Option<String>,
}

struct ObjectLockRetentionDeserializer;
impl ObjectLockRetentionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockRetention, XmlParseError> {
        deserialize_elements::<_, ObjectLockRetention, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Mode" => {
                    obj.mode = Some(ObjectLockRetentionModeDeserializer::deserialize(
                        "Mode", stack,
                    )?);
                }
                "RetainUntilDate" => {
                    obj.retain_until_date =
                        Some(DateDeserializer::deserialize("RetainUntilDate", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ObjectLockRetentionSerializer;
impl ObjectLockRetentionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectLockRetention,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.mode {
            writer.write(xml::writer::XmlEvent::start_element("Mode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.retain_until_date {
            writer.write(xml::writer::XmlEvent::start_element("RetainUntilDate"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectLockRetentionModeDeserializer;
impl ObjectLockRetentionModeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ObjectLockRetentionModeSerializer;
impl ObjectLockRetentionModeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The container element for an Object Lock rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectLockRule {
    /// <p>The default retention period that you want to apply to new objects placed in the specified bucket.</p>
    pub default_retention: Option<DefaultRetention>,
}

struct ObjectLockRuleDeserializer;
impl ObjectLockRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectLockRule, XmlParseError> {
        deserialize_elements::<_, ObjectLockRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultRetention" => {
                    obj.default_retention = Some(DefaultRetentionDeserializer::deserialize(
                        "DefaultRetention",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ObjectLockRuleSerializer;
impl ObjectLockRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ObjectLockRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.default_retention {
            &DefaultRetentionSerializer::serialize(&mut writer, "DefaultRetention", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectStorageClassDeserializer;
impl ObjectStorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectVersion {
    /// <p><p/></p>
    pub e_tag: Option<String>,
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub is_latest: Option<bool>,
    /// <p>The object key.</p>
    pub key: Option<String>,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: Option<String>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    /// <p>Size in bytes of the object.</p>
    pub size: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
    /// <p>Version ID of an object.</p>
    pub version_id: Option<String>,
}

struct ObjectVersionDeserializer;
impl ObjectVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ObjectVersion, XmlParseError> {
        deserialize_elements::<_, ObjectVersion, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ETag" => {
                    obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                }
                "IsLatest" => {
                    obj.is_latest = Some(IsLatestDeserializer::deserialize("IsLatest", stack)?);
                }
                "Key" => {
                    obj.key = Some(ObjectKeyDeserializer::deserialize("Key", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                "Owner" => {
                    obj.owner = Some(OwnerDeserializer::deserialize("Owner", stack)?);
                }
                "Size" => {
                    obj.size = Some(SizeDeserializer::deserialize("Size", stack)?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(ObjectVersionStorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                "VersionId" => {
                    obj.version_id = Some(ObjectVersionIdDeserializer::deserialize(
                        "VersionId",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ObjectVersionIdDeserializer;
impl ObjectVersionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ObjectVersionIdSerializer;
impl ObjectVersionIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ObjectVersionListDeserializer;
impl ObjectVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ObjectVersion>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ObjectVersionDeserializer::deserialize(tag_name, stack)?);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Describes the location where the restore job's output is stored.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub s3: Option<S3Location>,
}

pub struct OutputLocationSerializer;
impl OutputLocationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OutputLocation,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.s3 {
            &S3LocationSerializer::serialize(&mut writer, "S3", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes how results of the Select job are serialized.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub csv: Option<CSVOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub json: Option<JSONOutput>,
}

pub struct OutputSerializationSerializer;
impl OutputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OutputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVOutputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.json {
            &JSONOutputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Owner {
    /// <p><p/></p>
    pub display_name: Option<String>,
    /// <p><p/></p>
    pub id: Option<String>,
}

struct OwnerDeserializer;
impl OwnerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Owner, XmlParseError> {
        deserialize_elements::<_, Owner, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DisplayName" => {
                    obj.display_name =
                        Some(DisplayNameDeserializer::deserialize("DisplayName", stack)?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct OwnerSerializer;
impl OwnerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Owner,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.display_name {
            writer.write(xml::writer::XmlEvent::start_element("DisplayName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct OwnerOverrideDeserializer;
impl OwnerOverrideDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct OwnerOverrideSerializer;
impl OwnerOverrideSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParquetInput {}

pub struct ParquetInputSerializer;
impl ParquetInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ParquetInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Part {
    /// <p>Entity tag returned when the part was uploaded.</p>
    pub e_tag: Option<String>,
    /// <p>Date and time at which the part was uploaded.</p>
    pub last_modified: Option<String>,
    /// <p>Part number identifying the part. This is a positive integer between 1 and 10,000.</p>
    pub part_number: Option<i64>,
    /// <p>Size in bytes of the uploaded part data.</p>
    pub size: Option<i64>,
}

struct PartDeserializer;
impl PartDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Part, XmlParseError> {
        deserialize_elements::<_, Part, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ETag" => {
                    obj.e_tag = Some(ETagDeserializer::deserialize("ETag", stack)?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                "PartNumber" => {
                    obj.part_number =
                        Some(PartNumberDeserializer::deserialize("PartNumber", stack)?);
                }
                "Size" => {
                    obj.size = Some(SizeDeserializer::deserialize("Size", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PartNumberDeserializer;
impl PartNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PartNumberSerializer;
impl PartNumberSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PartNumberMarkerDeserializer;
impl PartNumberMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PartNumberMarkerSerializer;
impl PartNumberMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PartsDeserializer;
impl PartsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Part>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(PartDeserializer::deserialize(tag_name, stack)?);
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PayerSerializer;
impl PayerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PermissionDeserializer;
impl PermissionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PermissionSerializer;
impl PermissionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct PolicySerializer;
impl PolicySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>The container element for a bucket's policy status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PolicyStatus {
    /// <p>The policy status for this bucket. <code>TRUE</code> indicates that this bucket is public. <code>FALSE</code> indicates that the bucket is not public.</p>
    pub is_public: Option<bool>,
}

struct PolicyStatusDeserializer;
impl PolicyStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PolicyStatus, XmlParseError> {
        deserialize_elements::<_, PolicyStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "IsPublic" => {
                    obj.is_public = Some(IsPublicDeserializer::deserialize("IsPublic", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PrefixDeserializer;
impl PrefixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PrefixSerializer;
impl PrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct PriorityDeserializer;
impl PriorityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct PrioritySerializer;
impl PrioritySerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Progress {
    /// <p>The current number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>The current number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>The current number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Progress, XmlParseError> {
        deserialize_elements::<_, Progress, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "BytesProcessed" => {
                    obj.bytes_processed = Some(BytesProcessedDeserializer::deserialize(
                        "BytesProcessed",
                        stack,
                    )?);
                }
                "BytesReturned" => {
                    obj.bytes_returned = Some(BytesReturnedDeserializer::deserialize(
                        "BytesReturned",
                        stack,
                    )?);
                }
                "BytesScanned" => {
                    obj.bytes_scanned = Some(BytesScannedDeserializer::deserialize(
                        "BytesScanned",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgressEvent {
    /// <p>The Progress event details.</p>
    pub details: Option<Progress>,
}

struct ProgressEventDeserializer;
impl ProgressEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProgressEvent, XmlParseError> {
        deserialize_elements::<_, ProgressEvent, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Details" => {
                    obj.details = Some(ProgressDeserializer::deserialize("Details", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ProtocolDeserializer;
impl ProtocolDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ProtocolSerializer;
impl ProtocolSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PublicAccessBlockConfiguration {
    /// <p>Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes the following behavior:</p> <ul> <li> <p>PUT Bucket acl and PUT Object acl calls fail if the specified ACL is public.</p> </li> <li> <p>PUT Object calls fail if the request includes a public ACL.</p> </li> </ul> <p>Enabling this setting doesn't affect existing policies or ACLs.</p>
    pub block_public_acls: Option<bool>,
    /// <p>Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to reject calls to PUT Bucket policy if the specified bucket policy allows public access. </p> <p>Enabling this setting doesn't affect existing bucket policies.</p>
    pub block_public_policy: Option<bool>,
    /// <p>Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this bucket. Setting this element to <code>TRUE</code> causes Amazon S3 to ignore all public ACLs on this bucket and objects in this bucket.</p> <p>Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set.</p>
    pub ignore_public_acls: Option<bool>,
    /// <p>Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting this element to <code>TRUE</code> restricts access to this bucket to only AWS services and authorized users within this account if the bucket has a public policy.</p> <p>Enabling this setting doesn't affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked.</p>
    pub restrict_public_buckets: Option<bool>,
}

struct PublicAccessBlockConfigurationDeserializer;
impl PublicAccessBlockConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PublicAccessBlockConfiguration, XmlParseError> {
        deserialize_elements::<_, PublicAccessBlockConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "BlockPublicAcls" => {
                        obj.block_public_acls =
                            Some(SettingDeserializer::deserialize("BlockPublicAcls", stack)?);
                    }
                    "BlockPublicPolicy" => {
                        obj.block_public_policy = Some(SettingDeserializer::deserialize(
                            "BlockPublicPolicy",
                            stack,
                        )?);
                    }
                    "IgnorePublicAcls" => {
                        obj.ignore_public_acls =
                            Some(SettingDeserializer::deserialize("IgnorePublicAcls", stack)?);
                    }
                    "RestrictPublicBuckets" => {
                        obj.restrict_public_buckets = Some(SettingDeserializer::deserialize(
                            "RestrictPublicBuckets",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct PublicAccessBlockConfigurationSerializer;
impl PublicAccessBlockConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &PublicAccessBlockConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.block_public_acls {
            writer.write(xml::writer::XmlEvent::start_element("BlockPublicAcls"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.block_public_policy {
            writer.write(xml::writer::XmlEvent::start_element("BlockPublicPolicy"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.ignore_public_acls {
            writer.write(xml::writer::XmlEvent::start_element("IgnorePublicAcls"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.restrict_public_buckets {
            writer.write(xml::writer::XmlEvent::start_element(
                "RestrictPublicBuckets",
            ))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAccelerateConfigurationRequest {
    /// <p>Specifies the Accelerate Configuration you want to set for the bucket.</p>
    pub accelerate_configuration: AccelerateConfiguration,
    /// <p>Name of the bucket for which the accelerate configuration is set.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAccelerateConfigurationResponse {}

struct PutBucketAccelerateConfigurationResponseDeserializer;
impl PutBucketAccelerateConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAccelerateConfigurationResponse, XmlParseError> {
        Ok(PutBucketAccelerateConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAclRequest {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<String>,
    /// <p><p/></p>
    pub access_control_policy: Option<AccessControlPolicy>,
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAclResponse {}

struct PutBucketAclResponseDeserializer;
impl PutBucketAclResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAclResponse, XmlParseError> {
        Ok(PutBucketAclResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAnalyticsConfigurationRequest {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: AnalyticsConfiguration,
    /// <p>The name of the bucket to which an analytics configuration is stored.</p>
    pub bucket: String,
    /// <p>The identifier used to represent an analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketAnalyticsConfigurationResponse {}

struct PutBucketAnalyticsConfigurationResponseDeserializer;
impl PutBucketAnalyticsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketAnalyticsConfigurationResponse, XmlParseError> {
        Ok(PutBucketAnalyticsConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketCorsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub cors_configuration: CORSConfiguration,
    /// <p><p/></p>
    pub content_md5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketCorsResponse {}

struct PutBucketCorsResponseDeserializer;
impl PutBucketCorsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketCorsResponse, XmlParseError> {
        Ok(PutBucketCorsResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketEncryptionRequest {
    /// <p>The name of the bucket for which the server-side encryption configuration is set.</p>
    pub bucket: String,
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration. This parameter is auto-populated when using the command from the CLI</p>
    pub content_md5: Option<String>,
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketEncryptionResponse {}

struct PutBucketEncryptionResponseDeserializer;
impl PutBucketEncryptionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketEncryptionResponse, XmlParseError> {
        Ok(PutBucketEncryptionResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket where the inventory configuration will be stored.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: InventoryConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketInventoryConfigurationResponse {}

struct PutBucketInventoryConfigurationResponseDeserializer;
impl PutBucketInventoryConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketInventoryConfigurationResponse, XmlParseError> {
        Ok(PutBucketInventoryConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleConfigurationResponse {}

struct PutBucketLifecycleConfigurationResponseDeserializer;
impl PutBucketLifecycleConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLifecycleConfigurationResponse, XmlParseError> {
        Ok(PutBucketLifecycleConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLifecycleResponse {}

struct PutBucketLifecycleResponseDeserializer;
impl PutBucketLifecycleResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLifecycleResponse, XmlParseError> {
        Ok(PutBucketLifecycleResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLoggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub bucket_logging_status: BucketLoggingStatus,
    /// <p><p/></p>
    pub content_md5: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketLoggingResponse {}

struct PutBucketLoggingResponseDeserializer;
impl PutBucketLoggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketLoggingResponse, XmlParseError> {
        Ok(PutBucketLoggingResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket for which the metrics configuration is set.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: MetricsConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketMetricsConfigurationResponse {}

struct PutBucketMetricsConfigurationResponseDeserializer;
impl PutBucketMetricsConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketMetricsConfigurationResponse, XmlParseError> {
        Ok(PutBucketMetricsConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
    pub notification_configuration: NotificationConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationConfigurationResponse {}

struct PutBucketNotificationConfigurationResponseDeserializer;
impl PutBucketNotificationConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketNotificationConfigurationResponse, XmlParseError> {
        Ok(PutBucketNotificationConfigurationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub notification_configuration: NotificationConfigurationDeprecated,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketNotificationResponse {}

struct PutBucketNotificationResponseDeserializer;
impl PutBucketNotificationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketNotificationResponse, XmlParseError> {
        Ok(PutBucketNotificationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketPolicyRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    pub confirm_remove_self_bucket_access: Option<bool>,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketPolicyResponse {}

struct PutBucketPolicyResponseDeserializer;
impl PutBucketPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketPolicyResponse, XmlParseError> {
        Ok(PutBucketPolicyResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketReplicationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    pub replication_configuration: ReplicationConfiguration,
    /// <p><p/></p>
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketReplicationResponse {}

struct PutBucketReplicationResponseDeserializer;
impl PutBucketReplicationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketReplicationResponse, XmlParseError> {
        Ok(PutBucketReplicationResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketRequestPaymentRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub request_payment_configuration: RequestPaymentConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketRequestPaymentResponse {}

struct PutBucketRequestPaymentResponseDeserializer;
impl PutBucketRequestPaymentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketRequestPaymentResponse, XmlParseError> {
        Ok(PutBucketRequestPaymentResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub tagging: Tagging,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketTaggingResponse {}

struct PutBucketTaggingResponseDeserializer;
impl PutBucketTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketTaggingResponse, XmlParseError> {
        Ok(PutBucketTaggingResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketVersioningRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    /// <p><p/></p>
    pub versioning_configuration: VersioningConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketVersioningResponse {}

struct PutBucketVersioningResponseDeserializer;
impl PutBucketVersioningResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketVersioningResponse, XmlParseError> {
        Ok(PutBucketVersioningResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutBucketWebsiteResponse {}

struct PutBucketWebsiteResponseDeserializer;
impl PutBucketWebsiteResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutBucketWebsiteResponse, XmlParseError> {
        Ok(PutBucketWebsiteResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p><p/></p>
    pub access_control_policy: Option<AccessControlPolicy>,
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to list the objects in the bucket.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the bucket ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to create, overwrite, and delete any object in the bucket.</p>
    pub grant_write: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    pub grant_write_acp: Option<String>,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclResponse {
    pub request_charged: Option<String>,
}

struct PutObjectAclResponseDeserializer;
impl PutObjectAclResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectAclResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectAclResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectLegalHoldRequest {
    /// <p>The bucket containing the object that you want to place a Legal Hold on.</p>
    pub bucket: String,
    /// <p>The MD5 hash for the request body.</p>
    pub content_md5: Option<String>,
    /// <p>The key name for the object that you want to place a Legal Hold on.</p>
    pub key: String,
    /// <p>Container element for the Legal Hold configuration you want to apply to the specified object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
    pub request_payer: Option<String>,
    /// <p>The version ID of the object that you want to place a Legal Hold on.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectLegalHoldResponse {
    pub request_charged: Option<String>,
}

struct PutObjectLegalHoldResponseDeserializer;
impl PutObjectLegalHoldResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLegalHoldResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectLegalHoldResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectLockConfigurationRequest {
    /// <p>The bucket whose Object Lock configuration you want to create or replace.</p>
    pub bucket: String,
    /// <p>The MD5 hash for the request body.</p>
    pub content_md5: Option<String>,
    /// <p>The Object Lock configuration that you want to apply to the specified bucket.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub request_payer: Option<String>,
    /// <p>A token to allow Object Lock to be enabled for an existing bucket.</p>
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectLockConfigurationResponse {
    pub request_charged: Option<String>,
}

struct PutObjectLockConfigurationResponseDeserializer;
impl PutObjectLockConfigurationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLockConfigurationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectLockConfigurationResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug)]
pub struct PutObjectRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Name of the bucket to which the PUT operation was initiated.</p>
    pub bucket: String,
    /// <p>Specifies caching behavior along the request/reply chain.</p>
    pub cache_control: Option<String>,
    /// <p>Specifies presentational information for the object.</p>
    pub content_disposition: Option<String>,
    /// <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    pub content_encoding: Option<String>,
    /// <p>The language the content is in.</p>
    pub content_language: Option<String>,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub content_length: Option<i64>,
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI</p>
    pub content_md5: Option<String>,
    /// <p>A standard MIME type describing the format of the object data.</p>
    pub content_type: Option<String>,
    /// <p>The date and time at which the object is no longer cacheable.</p>
    pub expires: Option<String>,
    /// <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>
    pub grant_full_control: Option<String>,
    /// <p>Allows grantee to read the object data and its metadata.</p>
    pub grant_read: Option<String>,
    /// <p>Allows grantee to read the object ACL.</p>
    pub grant_read_acp: Option<String>,
    /// <p>Allows grantee to write the ACL for the applicable object.</p>
    pub grant_write_acp: Option<String>,
    /// <p>Object key for which the PUT operation was initiated.</p>
    pub key: String,
    /// <p>A map of metadata to store with the object in S3.</p>
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Legal Hold status that you want to apply to the specified object.</p>
    pub object_lock_legal_hold_status: Option<String>,
    /// <p>The Object Lock mode that you want to apply to this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when you want this object's Object Lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS key ID to use for object encryption. All GET and PUT requests for an object protected by AWS KMS will fail if not made via SSL or using SigV4. Documentation on configuring any of the officially supported AWS SDKs and CLI can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>The type of storage to use for the object. Defaults to 'STANDARD'.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters. (For example, "Key1=Value1")</p>
    pub tagging: Option<String>,
    /// <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    pub website_redirect_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectResponse {
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
}

struct PutObjectResponseDeserializer;
impl PutObjectResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectRetentionRequest {
    /// <p>The bucket that contains the object you want to apply this Object Retention configuration to.</p>
    pub bucket: String,
    /// <p>Indicates whether this operation should bypass Governance-mode restrictions.j</p>
    pub bypass_governance_retention: Option<bool>,
    /// <p>The MD5 hash for the request body.</p>
    pub content_md5: Option<String>,
    /// <p>The key name for the object that you want to apply this Object Retention configuration to.</p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>The container element for the Object Retention configuration.</p>
    pub retention: Option<ObjectLockRetention>,
    /// <p>The version ID for the object that you want to apply this Object Retention configuration to.</p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectRetentionResponse {
    pub request_charged: Option<String>,
}

struct PutObjectRetentionResponseDeserializer;
impl PutObjectRetentionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectRetentionResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectRetentionResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p><p/></p>
    pub tagging: Tagging,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectTaggingResponse {
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct PutObjectTaggingResponseDeserializer;
impl PutObjectTaggingResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectTaggingResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectTaggingResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutPublicAccessBlockRequest {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to set.</p>
    pub bucket: String,
    /// <p>The MD5 hash of the <code>PutPublicAccessBlock</code> request body. </p>
    pub content_md5: Option<String>,
    /// <p>The <code>PublicAccessBlock</code> configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. For more information about when Amazon S3 considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutPublicAccessBlockResponse {}

struct PutPublicAccessBlockResponseDeserializer;
impl PutPublicAccessBlockResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutPublicAccessBlockResponse, XmlParseError> {
        Ok(PutPublicAccessBlockResponse::default())
    }
}
struct QueueArnDeserializer;
impl QueueArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct QueueArnSerializer;
impl QueueArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for specifying the configuration for publication of messages to an Amazon Simple Queue Service (Amazon SQS) queue.when Amazon S3 detects specified events.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueueConfiguration {
    /// <p><p/></p>
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SQS queue to which Amazon S3 will publish a message when it detects events of the specified type.</p>
    pub queue_arn: String,
}

struct QueueConfigurationDeserializer;
impl QueueConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfiguration, XmlParseError> {
        deserialize_elements::<_, QueueConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Event" => {
                    obj.events
                        .extend(EventListDeserializer::deserialize("Event", stack)?);
                }
                "Filter" => {
                    obj.filter = Some(NotificationConfigurationFilterDeserializer::deserialize(
                        "Filter", stack,
                    )?);
                }
                "Id" => {
                    obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                }
                "Queue" => {
                    obj.queue_arn = QueueArnDeserializer::deserialize("Queue", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct QueueConfigurationSerializer;
impl QueueConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueueConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.queue_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueueConfigurationDeprecated {
    /// <p><p/></p>
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    /// <p><p/></p>
    pub queue: Option<String>,
}

struct QueueConfigurationDeprecatedDeserializer;
impl QueueConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<QueueConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, QueueConfigurationDeprecated, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Event" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Event", stack)?);
                    }
                    "Id" => {
                        obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                    }
                    "Queue" => {
                        obj.queue = Some(QueueArnDeserializer::deserialize("Queue", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct QueueConfigurationDeprecatedSerializer;
impl QueueConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &QueueConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.queue {
            writer.write(xml::writer::XmlEvent::start_element("Queue"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct QueueConfigurationListDeserializer;
impl QueueConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<QueueConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(QueueConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<QueueConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteCharacterSerializer;
impl QuoteCharacterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteEscapeCharacterSerializer;
impl QuoteEscapeCharacterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct QuoteFieldsSerializer;
impl QuoteFieldsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct RecordDelimiterSerializer;
impl RecordDelimiterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub payload: Option<bytes::Bytes>,
}

struct RecordsEventDeserializer;
impl RecordsEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecordsEvent, XmlParseError> {
        deserialize_elements::<_, RecordsEvent, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Payload" => {
                    obj.payload = Some(BodyDeserializer::deserialize("Payload", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Redirect {
    /// <p>The host name to use in the redirect request.</p>
    pub host_name: Option<String>,
    /// <p>The HTTP redirect code to use on the response. Not required if one of the siblings is present.</p>
    pub http_redirect_code: Option<String>,
    /// <p>Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request.</p>
    pub protocol: Option<String>,
    /// <p>The object key prefix to use in the redirect request. For example, to redirect requests for all pages with prefix docs/ (objects in the docs/ folder) to documents/, you can set a condition block with KeyPrefixEquals set to docs/ and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if one of the siblings is present. Can be present only if ReplaceKeyWith is not provided.</p>
    pub replace_key_prefix_with: Option<String>,
    /// <p>The specific object key to use in the redirect request. For example, redirect request to error.html. Not required if one of the sibling is present. Can be present only if ReplaceKeyPrefixWith is not provided.</p>
    pub replace_key_with: Option<String>,
}

struct RedirectDeserializer;
impl RedirectDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Redirect, XmlParseError> {
        deserialize_elements::<_, Redirect, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HostName" => {
                    obj.host_name = Some(HostNameDeserializer::deserialize("HostName", stack)?);
                }
                "HttpRedirectCode" => {
                    obj.http_redirect_code = Some(HttpRedirectCodeDeserializer::deserialize(
                        "HttpRedirectCode",
                        stack,
                    )?);
                }
                "Protocol" => {
                    obj.protocol = Some(ProtocolDeserializer::deserialize("Protocol", stack)?);
                }
                "ReplaceKeyPrefixWith" => {
                    obj.replace_key_prefix_with =
                        Some(ReplaceKeyPrefixWithDeserializer::deserialize(
                            "ReplaceKeyPrefixWith",
                            stack,
                        )?);
                }
                "ReplaceKeyWith" => {
                    obj.replace_key_with = Some(ReplaceKeyWithDeserializer::deserialize(
                        "ReplaceKeyWith",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct RedirectSerializer;
impl RedirectSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Redirect,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.host_name {
            writer.write(xml::writer::XmlEvent::start_element("HostName"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.http_redirect_code {
            writer.write(xml::writer::XmlEvent::start_element("HttpRedirectCode"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.protocol {
            writer.write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_prefix_with {
            writer.write(xml::writer::XmlEvent::start_element("ReplaceKeyPrefixWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.replace_key_with {
            writer.write(xml::writer::XmlEvent::start_element("ReplaceKeyWith"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RedirectAllRequestsTo {
    /// <p>Name of the host where requests will be redirected.</p>
    pub host_name: String,
    /// <p>Protocol to use (http, https) when redirecting requests. The default is the protocol that is used in the original request.</p>
    pub protocol: Option<String>,
}

struct RedirectAllRequestsToDeserializer;
impl RedirectAllRequestsToDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RedirectAllRequestsTo, XmlParseError> {
        deserialize_elements::<_, RedirectAllRequestsTo, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "HostName" => {
                    obj.host_name = HostNameDeserializer::deserialize("HostName", stack)?;
                }
                "Protocol" => {
                    obj.protocol = Some(ProtocolDeserializer::deserialize("Protocol", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct RedirectAllRequestsToSerializer;
impl RedirectAllRequestsToSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RedirectAllRequestsTo,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("HostName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.host_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.protocol {
            writer.write(xml::writer::XmlEvent::start_element("Protocol"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplaceKeyPrefixWithDeserializer;
impl ReplaceKeyPrefixWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ReplaceKeyPrefixWithSerializer;
impl ReplaceKeyPrefixWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplaceKeyWithDeserializer;
impl ReplaceKeyWithDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ReplaceKeyWithSerializer;
impl ReplaceKeyWithSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicaKmsKeyIDDeserializer;
impl ReplicaKmsKeyIDDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ReplicaKmsKeyIDSerializer;
impl ReplicaKmsKeyIDSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for replication rules. You can add up to 1,000 rules. The maximum size of a replication configuration is 2 MB.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that Amazon S3 can assume when replicating the objects.</p>
    pub role: String,
    /// <p>A container for one or more replication rules. A replication configuration must have at least one rule and can contain a maximum of 1,000 rules. </p>
    pub rules: Vec<ReplicationRule>,
}

struct ReplicationConfigurationDeserializer;
impl ReplicationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationConfiguration, XmlParseError> {
        deserialize_elements::<_, ReplicationConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Role" => {
                        obj.role = RoleDeserializer::deserialize("Role", stack)?;
                    }
                    "Rule" => {
                        obj.rules
                            .extend(ReplicationRulesDeserializer::deserialize("Rule", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ReplicationConfigurationSerializer;
impl ReplicationConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Role"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.role
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        ReplicationRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for information about a specific replication rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationRule {
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    /// <p>A container for information about the replication destination.</p>
    pub destination: Destination,
    pub filter: Option<ReplicationRuleFilter>,
    /// <p>A unique identifier for the rule. The maximum value is 255 characters.</p>
    pub id: Option<String>,
    /// <p>The priority associated with the rule. If you specify multiple rules in a replication configuration, Amazon S3 prioritizes the rules to prevent conflicts when filtering. If two or more rules identify the same object based on a specified filter, the rule with higher priority takes precedence. For example:</p> <ul> <li> <p>Same object quality prefix based filter criteria If prefixes you specified in multiple rules overlap </p> </li> <li> <p>Same object qualify tag based filter criteria specified in multiple rules</p> </li> </ul> <p>For more information, see <a href=" https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub priority: Option<i64>,
    /// <p>A container that describes additional filters for identifying the source objects that you want to replicate. You can choose to enable or disable the replication of these objects. Currently, Amazon S3 supports only the filter that you can specify for objects created with server-side encryption using an AWS KMS-Managed Key (SSE-KMS). </p> <p> If you want Amazon S3 to replicate objects created with server-side encryption using AWS KMS-Managed Keys. </p>
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    /// <p>If status isn't enabled, the rule is ignored.</p>
    pub status: String,
}

struct ReplicationRuleDeserializer;
impl ReplicationRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRule, XmlParseError> {
        deserialize_elements::<_, ReplicationRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DeleteMarkerReplication" => {
                    obj.delete_marker_replication =
                        Some(DeleteMarkerReplicationDeserializer::deserialize(
                            "DeleteMarkerReplication",
                            stack,
                        )?);
                }
                "Destination" => {
                    obj.destination = DestinationDeserializer::deserialize("Destination", stack)?;
                }
                "Filter" => {
                    obj.filter = Some(ReplicationRuleFilterDeserializer::deserialize(
                        "Filter", stack,
                    )?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                "Priority" => {
                    obj.priority = Some(PriorityDeserializer::deserialize("Priority", stack)?);
                }
                "SourceSelectionCriteria" => {
                    obj.source_selection_criteria =
                        Some(SourceSelectionCriteriaDeserializer::deserialize(
                            "SourceSelectionCriteria",
                            stack,
                        )?);
                }
                "Status" => {
                    obj.status = ReplicationRuleStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ReplicationRuleSerializer;
impl ReplicationRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.delete_marker_replication {
            &DeleteMarkerReplicationSerializer::serialize(
                &mut writer,
                "DeleteMarkerReplication",
                value,
            )?;
        }
        DestinationSerializer::serialize(&mut writer, "Destination", &obj.destination)?;
        if let Some(ref value) = obj.filter {
            &ReplicationRuleFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.priority {
            writer.write(xml::writer::XmlEvent::start_element("Priority"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.source_selection_criteria {
            &SourceSelectionCriteriaSerializer::serialize(
                &mut writer,
                "SourceSelectionCriteria",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationRuleAndOperator {
    /// <p><p/></p>
    pub prefix: Option<String>,
    /// <p><p/></p>
    pub tags: Option<Vec<Tag>>,
}

struct ReplicationRuleAndOperatorDeserializer;
impl ReplicationRuleAndOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRuleAndOperator, XmlParseError> {
        deserialize_elements::<_, ReplicationRuleAndOperator, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Prefix" => {
                        obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                    }
                    "Tag" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagSetDeserializer::deserialize("Tag", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ReplicationRuleAndOperatorSerializer;
impl ReplicationRuleAndOperatorSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationRuleAndOperator,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tags {
            &TagSetSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A filter that identifies the subset of objects to which the replication rule applies. A <code>Filter</code> must specify exactly one <code>Prefix</code>, <code>Tag</code>, or an <code>And</code> child element.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationRuleFilter {
    /// <p><p>A container for specifying rule filters. The filters determine the subset of objects to which the rule applies. This element is required only if you specify more than one filter. For example: </p> <ul> <li> <p>If you specify both a <code>Prefix</code> and a <code>Tag</code> filter, wrap these filters in an <code>And</code> tag.</p> </li> <li> <p>If you specify a filter based on multiple tags, wrap the <code>Tag</code> elements in an <code>And</code> tag.</p> </li> </ul></p>
    pub and: Option<ReplicationRuleAndOperator>,
    /// <p>An object keyname prefix that identifies the subset of objects to which the rule applies.</p>
    pub prefix: Option<String>,
    /// <p>A container for specifying a tag key and value. </p> <p>The rule applies only to objects that have the tag in their tag set.</p>
    pub tag: Option<Tag>,
}

struct ReplicationRuleFilterDeserializer;
impl ReplicationRuleFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationRuleFilter, XmlParseError> {
        deserialize_elements::<_, ReplicationRuleFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "And" => {
                    obj.and = Some(ReplicationRuleAndOperatorDeserializer::deserialize(
                        "And", stack,
                    )?);
                }
                "Prefix" => {
                    obj.prefix = Some(PrefixDeserializer::deserialize("Prefix", stack)?);
                }
                "Tag" => {
                    obj.tag = Some(TagDeserializer::deserialize("Tag", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct ReplicationRuleFilterSerializer;
impl ReplicationRuleFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ReplicationRuleFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.and {
            &ReplicationRuleAndOperatorSerializer::serialize(&mut writer, "And", value)?;
        }
        if let Some(ref value) = obj.prefix {
            writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tag {
            &TagSerializer::serialize(&mut writer, "Tag", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicationRuleStatusDeserializer;
impl ReplicationRuleStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ReplicationRuleStatusSerializer;
impl ReplicationRuleStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ReplicationRulesDeserializer;
impl ReplicationRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReplicationRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ReplicationRuleDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ReplicationRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ReplicationRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestPaymentConfiguration {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: String,
}

pub struct RequestPaymentConfigurationSerializer;
impl RequestPaymentConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RequestPaymentConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Payer"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.payer
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestProgress {
    /// <p>Specifies whether periodic QueryProgress frames should be sent. Valid values: TRUE, FALSE. Default value: FALSE.</p>
    pub enabled: Option<bool>,
}

pub struct RequestProgressSerializer;
impl RequestProgressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RequestProgress,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.enabled {
            writer.write(xml::writer::XmlEvent::start_element("Enabled"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseCacheControlSerializer;
impl ResponseCacheControlSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentDispositionSerializer;
impl ResponseContentDispositionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentEncodingSerializer;
impl ResponseContentEncodingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentLanguageSerializer;
impl ResponseContentLanguageSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseContentTypeSerializer;
impl ResponseContentTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ResponseExpiresSerializer;
impl ResponseExpiresSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
    pub restore_request: Option<RestoreRequest>,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreObjectResponse {
    pub request_charged: Option<String>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub restore_output_path: Option<String>,
}

struct RestoreObjectResponseDeserializer;
impl RestoreObjectResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreObjectResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RestoreObjectResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Container for restore job parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreRequest {
    /// <p>Lifetime of the active copy in days. Do not use with restores that specify OutputLocation.</p>
    pub days: Option<i64>,
    /// <p>The optional description for the job.</p>
    pub description: Option<String>,
    /// <p>Glacier related parameters pertaining to this job. Do not use with restores that specify OutputLocation.</p>
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    /// <p>Describes the location where the restore job's output is stored.</p>
    pub output_location: Option<OutputLocation>,
    /// <p>Describes the parameters for Select job types.</p>
    pub select_parameters: Option<SelectParameters>,
    /// <p>Glacier retrieval tier at which the restore will be processed.</p>
    pub tier: Option<String>,
    /// <p>Type of restore request.</p>
    pub type_: Option<String>,
}

pub struct RestoreRequestSerializer;
impl RestoreRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RestoreRequest,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.description {
            writer.write(xml::writer::XmlEvent::start_element("Description"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.glacier_job_parameters {
            &GlacierJobParametersSerializer::serialize(&mut writer, "GlacierJobParameters", value)?;
        }
        if let Some(ref value) = obj.output_location {
            &OutputLocationSerializer::serialize(&mut writer, "OutputLocation", value)?;
        }
        if let Some(ref value) = obj.select_parameters {
            &SelectParametersSerializer::serialize(&mut writer, "SelectParameters", value)?;
        }
        if let Some(ref value) = obj.tier {
            writer.write(xml::writer::XmlEvent::start_element("Tier"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.type_ {
            writer.write(xml::writer::XmlEvent::start_element("Type"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct RestoreRequestTypeSerializer;
impl RestoreRequestTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct RoleDeserializer;
impl RoleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct RoleSerializer;
impl RoleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoutingRule {
    /// <p>A container for describing a condition that must be met for the specified redirect to apply. For example, 1. If request is for pages in the /docs folder, redirect to the /documents folder. 2. If request results in HTTP error 4xx, redirect request to another host where you might process the error.</p>
    pub condition: Option<Condition>,
    /// <p>Container for redirect information. You can redirect requests to another host, to another page, or with another protocol. In the event of an error, you can specify a different error code to return.</p>
    pub redirect: Redirect,
}

struct RoutingRuleDeserializer;
impl RoutingRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RoutingRule, XmlParseError> {
        deserialize_elements::<_, RoutingRule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Condition" => {
                    obj.condition = Some(ConditionDeserializer::deserialize("Condition", stack)?);
                }
                "Redirect" => {
                    obj.redirect = RedirectDeserializer::deserialize("Redirect", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct RoutingRuleSerializer;
impl RoutingRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RoutingRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RoutingRule>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "RoutingRule" {
                obj.push(RoutingRuleDeserializer::deserialize("RoutingRule", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct RoutingRulesSerializer;
impl RoutingRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<RoutingRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            RoutingRuleSerializer::serialize(writer, "RoutingRule", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// <p><p/></p>
    pub expiration: Option<LifecycleExpiration>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    /// <p>Prefix identifying one or more objects to which the rule applies.</p>
    pub prefix: String,
    /// <p>If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is not currently being applied.</p>
    pub status: String,
    /// <p><p/></p>
    pub transition: Option<Transition>,
}

struct RuleDeserializer;
impl RuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rule, XmlParseError> {
        deserialize_elements::<_, Rule, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AbortIncompleteMultipartUpload" => {
                    obj.abort_incomplete_multipart_upload =
                        Some(AbortIncompleteMultipartUploadDeserializer::deserialize(
                            "AbortIncompleteMultipartUpload",
                            stack,
                        )?);
                }
                "Expiration" => {
                    obj.expiration = Some(LifecycleExpirationDeserializer::deserialize(
                        "Expiration",
                        stack,
                    )?);
                }
                "ID" => {
                    obj.id = Some(IDDeserializer::deserialize("ID", stack)?);
                }
                "NoncurrentVersionExpiration" => {
                    obj.noncurrent_version_expiration =
                        Some(NoncurrentVersionExpirationDeserializer::deserialize(
                            "NoncurrentVersionExpiration",
                            stack,
                        )?);
                }
                "NoncurrentVersionTransition" => {
                    obj.noncurrent_version_transition =
                        Some(NoncurrentVersionTransitionDeserializer::deserialize(
                            "NoncurrentVersionTransition",
                            stack,
                        )?);
                }
                "Prefix" => {
                    obj.prefix = PrefixDeserializer::deserialize("Prefix", stack)?;
                }
                "Status" => {
                    obj.status = ExpirationStatusDeserializer::deserialize("Status", stack)?;
                }
                "Transition" => {
                    obj.transition =
                        Some(TransitionDeserializer::deserialize("Transition", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct RuleSerializer;
impl RuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Rule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.abort_incomplete_multipart_upload {
            &AbortIncompleteMultipartUploadSerializer::serialize(
                &mut writer,
                "AbortIncompleteMultipartUpload",
                value,
            )?;
        }
        if let Some(ref value) = obj.expiration {
            &LifecycleExpirationSerializer::serialize(&mut writer, "Expiration", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("ID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.noncurrent_version_expiration {
            &NoncurrentVersionExpirationSerializer::serialize(
                &mut writer,
                "NoncurrentVersionExpiration",
                value,
            )?;
        }
        if let Some(ref value) = obj.noncurrent_version_transition {
            &NoncurrentVersionTransitionSerializer::serialize(
                &mut writer,
                "NoncurrentVersionTransition",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Rule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(RuleDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Rule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            RuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p>A container for object key name prefix and suffix filtering rules.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3KeyFilter {
    pub filter_rules: Option<Vec<FilterRule>>,
}

struct S3KeyFilterDeserializer;
impl S3KeyFilterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3KeyFilter, XmlParseError> {
        deserialize_elements::<_, S3KeyFilter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "FilterRule" => {
                    obj.filter_rules.get_or_insert(vec![]).extend(
                        FilterRuleListDeserializer::deserialize("FilterRule", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct S3KeyFilterSerializer;
impl S3KeyFilterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3KeyFilter,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.filter_rules {
            &FilterRuleListSerializer::serialize(&mut writer, "FilterRule", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes an S3 location that will receive the results of the restore request.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Location {
    /// <p>A list of grants that control access to the staged results.</p>
    pub access_control_list: Option<Vec<Grant>>,
    /// <p>The name of the bucket where the restore results will be placed.</p>
    pub bucket_name: String,
    /// <p>The canned ACL to apply to the restore results.</p>
    pub canned_acl: Option<String>,
    pub encryption: Option<Encryption>,
    /// <p>The prefix that is prepended to the restore results for this request.</p>
    pub prefix: String,
    /// <p>The class of storage used to store the restore results.</p>
    pub storage_class: Option<String>,
    /// <p>The tag-set that is applied to the restore results.</p>
    pub tagging: Option<Tagging>,
    /// <p>A list of metadata to store with the restore results in S3.</p>
    pub user_metadata: Option<Vec<MetadataEntry>>,
}

pub struct S3LocationSerializer;
impl S3LocationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &S3Location,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.access_control_list {
            &GrantsSerializer::serialize(&mut writer, "AccessControlList", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("BucketName"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.bucket_name
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.canned_acl {
            writer.write(xml::writer::XmlEvent::start_element("CannedACL"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.encryption {
            &EncryptionSerializer::serialize(&mut writer, "Encryption", value)?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Prefix"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.prefix
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.tagging {
            &TaggingSerializer::serialize(&mut writer, "Tagging", value)?;
        }
        if let Some(ref value) = obj.user_metadata {
            &UserMetadataSerializer::serialize(&mut writer, "UserMetadata", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Specifies the use of SSE-KMS to encrypt delivered Inventory reports.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SSEKMS {
    /// <p>Specifies the ID of the AWS Key Management Service (KMS) master encryption key to use for encrypting Inventory reports.</p>
    pub key_id: String,
}

struct SSEKMSDeserializer;
impl SSEKMSDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSEKMS, XmlParseError> {
        deserialize_elements::<_, SSEKMS, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "KeyId" => {
                    obj.key_id = SSEKMSKeyIdDeserializer::deserialize("KeyId", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct SSEKMSSerializer;
impl SSEKMSSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SSEKMS,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("KeyId"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key_id
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SSEKMSKeyIdDeserializer;
impl SSEKMSKeyIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct SSEKMSKeyIdSerializer;
impl SSEKMSKeyIdSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Specifies the use of SSE-S3 to encrypt delivered Inventory reports.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SSES3 {}

struct SSES3Deserializer;
impl SSES3Deserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSES3, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = SSES3::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct SSES3Serializer;
impl SSES3Serializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SSES3,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentEventStream {
    /// <p>The Continuation Event.</p>
    pub cont: Option<ContinuationEvent>,
    /// <p>The End Event.</p>
    pub end: Option<EndEvent>,
    /// <p>The Progress Event.</p>
    pub progress: Option<ProgressEvent>,
    /// <p>The Records Event.</p>
    pub records: Option<RecordsEvent>,
    /// <p>The Stats Event.</p>
    pub stats: Option<StatsEvent>,
}

struct SelectObjectContentEventStreamDeserializer;
impl SelectObjectContentEventStreamDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentEventStream, XmlParseError> {
        deserialize_elements::<_, SelectObjectContentEventStream, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Cont" => {
                        obj.cont = Some(ContinuationEventDeserializer::deserialize("Cont", stack)?);
                    }
                    "End" => {
                        obj.end = Some(EndEventDeserializer::deserialize("End", stack)?);
                    }
                    "Progress" => {
                        obj.progress =
                            Some(ProgressEventDeserializer::deserialize("Progress", stack)?);
                    }
                    "Records" => {
                        obj.records =
                            Some(RecordsEventDeserializer::deserialize("Records", stack)?);
                    }
                    "Stats" => {
                        obj.stats = Some(StatsEventDeserializer::deserialize("Stats", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Request to filter the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records. It returns only records that match the specified SQL expression. You must also specify the data serialization format for the response. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectSELECTContent.html">S3Select API Documentation</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentRequest {
    /// <p>The S3 bucket.</p>
    pub bucket: String,
    /// <p>The expression that is used to query the object.</p>
    pub expression: String,
    /// <p>The type of the provided expression (for example., SQL).</p>
    pub expression_type: String,
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub input_serialization: InputSerialization,
    /// <p>The object key.</p>
    pub key: String,
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub output_serialization: OutputSerialization,
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub request_progress: Option<RequestProgress>,
    /// <p>The SSE Algorithm used to encrypt the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>The SSE Customer Key. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key: Option<String>,
    /// <p>The SSE Customer Key MD5. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html"> Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key_md5: Option<String>,
}

pub struct SelectObjectContentRequestSerializer;
impl SelectObjectContentRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SelectObjectContentRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        ExpressionSerializer::serialize(&mut writer, "Expression", &obj.expression)?;
        ExpressionTypeSerializer::serialize(&mut writer, "ExpressionType", &obj.expression_type)?;
        InputSerializationSerializer::serialize(
            &mut writer,
            "InputSerialization",
            &obj.input_serialization,
        )?;
        OutputSerializationSerializer::serialize(
            &mut writer,
            "OutputSerialization",
            &obj.output_serialization,
        )?;
        if let Some(ref value) = obj.request_progress {
            &RequestProgressSerializer::serialize(&mut writer, "RequestProgress", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentResponse {
    /// <p><p/></p>
    pub payload: Option<SelectObjectContentEventStream>,
}

struct SelectObjectContentResponseDeserializer;
impl SelectObjectContentResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentResponse, XmlParseError> {
        Ok(SelectObjectContentResponse {
            payload: Some(SelectObjectContentEventStreamDeserializer::deserialize(
                "Payload", stack,
            )?),
            ..SelectObjectContentResponse::default()
        })
    }
}
/// <p>Describes the parameters for Select job types.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectParameters {
    /// <p>The expression that is used to query the object.</p>
    pub expression: String,
    /// <p>The type of the provided expression (e.g., SQL).</p>
    pub expression_type: String,
    /// <p>Describes the serialization format of the object.</p>
    pub input_serialization: InputSerialization,
    /// <p>Describes how the results of the Select job are serialized.</p>
    pub output_serialization: OutputSerialization,
}

pub struct SelectParametersSerializer;
impl SelectParametersSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SelectParameters,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Expression"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.expression
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("ExpressionType"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.expression_type
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        InputSerializationSerializer::serialize(
            &mut writer,
            "InputSerialization",
            &obj.input_serialization,
        )?;
        OutputSerializationSerializer::serialize(
            &mut writer,
            "OutputSerialization",
            &obj.output_serialization,
        )?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ServerSideEncryptionDeserializer;
impl ServerSideEncryptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ServerSideEncryptionSerializer;
impl ServerSideEncryptionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If Put Object request does not specify any server-side encryption, this default encryption will be applied.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionByDefault {
    /// <p>KMS master key ID to use for the default encryption. This parameter is allowed if SSEAlgorithm is aws:kms.</p>
    pub kms_master_key_id: Option<String>,
    /// <p>Server-side encryption algorithm to use for the default encryption.</p>
    pub sse_algorithm: String,
}

struct ServerSideEncryptionByDefaultDeserializer;
impl ServerSideEncryptionByDefaultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionByDefault, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionByDefault, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "KMSMasterKeyID" => {
                        obj.kms_master_key_id = Some(SSEKMSKeyIdDeserializer::deserialize(
                            "KMSMasterKeyID",
                            stack,
                        )?);
                    }
                    "SSEAlgorithm" => {
                        obj.sse_algorithm =
                            ServerSideEncryptionDeserializer::deserialize("SSEAlgorithm", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ServerSideEncryptionByDefaultSerializer;
impl ServerSideEncryptionByDefaultSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionByDefault,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.kms_master_key_id {
            writer.write(xml::writer::XmlEvent::start_element("KMSMasterKeyID"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("SSEAlgorithm"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.sse_algorithm
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for server-side encryption configuration rules. Currently S3 supports one rule only.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    pub rules: Vec<ServerSideEncryptionRule>,
}

struct ServerSideEncryptionConfigurationDeserializer;
impl ServerSideEncryptionConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionConfiguration, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Rule" => {
                        obj.rules
                            .extend(ServerSideEncryptionRulesDeserializer::deserialize(
                                "Rule", stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ServerSideEncryptionConfigurationSerializer;
impl ServerSideEncryptionConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        ServerSideEncryptionRulesSerializer::serialize(&mut writer, "Rule", &obj.rules)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>Container for information about a particular server-side encryption configuration rule.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionRule {
    /// <p>Describes the default server-side encryption to apply to new objects in the bucket. If Put Object request does not specify any server-side encryption, this default encryption will be applied.</p>
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

struct ServerSideEncryptionRuleDeserializer;
impl ServerSideEncryptionRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServerSideEncryptionRule, XmlParseError> {
        deserialize_elements::<_, ServerSideEncryptionRule, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ApplyServerSideEncryptionByDefault" => {
                        obj.apply_server_side_encryption_by_default =
                            Some(ServerSideEncryptionByDefaultDeserializer::deserialize(
                                "ApplyServerSideEncryptionByDefault",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct ServerSideEncryptionRuleSerializer;
impl ServerSideEncryptionRuleSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ServerSideEncryptionRule,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.apply_server_side_encryption_by_default {
            &ServerSideEncryptionByDefaultSerializer::serialize(
                &mut writer,
                "ApplyServerSideEncryptionByDefault",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct ServerSideEncryptionRulesDeserializer;
impl ServerSideEncryptionRulesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServerSideEncryptionRule>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(ServerSideEncryptionRuleDeserializer::deserialize(
                    tag_name, stack,
                )?);
            } else {
                break;
            }
        }

        Ok(obj)
    }
}

pub struct ServerSideEncryptionRulesSerializer;
impl ServerSideEncryptionRulesSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<ServerSideEncryptionRule>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            ServerSideEncryptionRuleSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

struct SettingDeserializer;
impl SettingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct SettingSerializer;
impl SettingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &bool,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SizeDeserializer;
impl SizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A container for filters that define which source objects should be replicated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceSelectionCriteria {
    /// <p> A container for filter information for the selection of S3 objects encrypted with AWS KMS. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p>
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

struct SourceSelectionCriteriaDeserializer;
impl SourceSelectionCriteriaDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceSelectionCriteria, XmlParseError> {
        deserialize_elements::<_, SourceSelectionCriteria, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SseKmsEncryptedObjects" => {
                        obj.sse_kms_encrypted_objects =
                            Some(SseKmsEncryptedObjectsDeserializer::deserialize(
                                "SseKmsEncryptedObjects",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct SourceSelectionCriteriaSerializer;
impl SourceSelectionCriteriaSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SourceSelectionCriteria,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.sse_kms_encrypted_objects {
            &SseKmsEncryptedObjectsSerializer::serialize(
                &mut writer,
                "SseKmsEncryptedObjects",
                value,
            )?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for filter information for the selection of S3 objects encrypted with AWS KMS.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SseKmsEncryptedObjects {
    /// <p> If the status is not <code>Enabled</code>, replication for S3 objects encrypted with AWS KMS is disabled.</p>
    pub status: String,
}

struct SseKmsEncryptedObjectsDeserializer;
impl SseKmsEncryptedObjectsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SseKmsEncryptedObjects, XmlParseError> {
        deserialize_elements::<_, SseKmsEncryptedObjects, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Status" => {
                    obj.status =
                        SseKmsEncryptedObjectsStatusDeserializer::deserialize("Status", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct SseKmsEncryptedObjectsSerializer;
impl SseKmsEncryptedObjectsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SseKmsEncryptedObjects,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Status"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.status
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SseKmsEncryptedObjectsStatusDeserializer;
impl SseKmsEncryptedObjectsStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct SseKmsEncryptedObjectsStatusSerializer;
impl SseKmsEncryptedObjectsStatusSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct StartAfterDeserializer;
impl StartAfterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct StartAfterSerializer;
impl StartAfterSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stats {
    /// <p>The total number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>The total number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>The total number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

struct StatsDeserializer;
impl StatsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Stats, XmlParseError> {
        deserialize_elements::<_, Stats, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "BytesProcessed" => {
                    obj.bytes_processed = Some(BytesProcessedDeserializer::deserialize(
                        "BytesProcessed",
                        stack,
                    )?);
                }
                "BytesReturned" => {
                    obj.bytes_returned = Some(BytesReturnedDeserializer::deserialize(
                        "BytesReturned",
                        stack,
                    )?);
                }
                "BytesScanned" => {
                    obj.bytes_scanned = Some(BytesScannedDeserializer::deserialize(
                        "BytesScanned",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatsEvent {
    /// <p>The Stats event details.</p>
    pub details: Option<Stats>,
}

struct StatsEventDeserializer;
impl StatsEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatsEvent, XmlParseError> {
        deserialize_elements::<_, StatsEvent, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Details" => {
                    obj.details = Some(StatsDeserializer::deserialize("Details", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StorageClassDeserializer;
impl StorageClassDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct StorageClassSerializer;
impl StorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageClassAnalysis {
    /// <p>A container used to describe how data related to the storage class analysis should be exported.</p>
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

struct StorageClassAnalysisDeserializer;
impl StorageClassAnalysisDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysis, XmlParseError> {
        deserialize_elements::<_, StorageClassAnalysis, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DataExport" => {
                    obj.data_export =
                        Some(StorageClassAnalysisDataExportDeserializer::deserialize(
                            "DataExport",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct StorageClassAnalysisSerializer;
impl StorageClassAnalysisSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StorageClassAnalysis,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.data_export {
            &StorageClassAnalysisDataExportSerializer::serialize(&mut writer, "DataExport", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageClassAnalysisDataExport {
    /// <p>The place to store the data for an analysis.</p>
    pub destination: AnalyticsExportDestination,
    /// <p>The version of the output schema to use when exporting data. Must be V_1.</p>
    pub output_schema_version: String,
}

struct StorageClassAnalysisDataExportDeserializer;
impl StorageClassAnalysisDataExportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StorageClassAnalysisDataExport, XmlParseError> {
        deserialize_elements::<_, StorageClassAnalysisDataExport, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Destination" => {
                        obj.destination = AnalyticsExportDestinationDeserializer::deserialize(
                            "Destination",
                            stack,
                        )?;
                    }
                    "OutputSchemaVersion" => {
                        obj.output_schema_version =
                            StorageClassAnalysisSchemaVersionDeserializer::deserialize(
                                "OutputSchemaVersion",
                                stack,
                            )?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct StorageClassAnalysisDataExportSerializer;
impl StorageClassAnalysisDataExportSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &StorageClassAnalysisDataExport,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        AnalyticsExportDestinationSerializer::serialize(
            &mut writer,
            "Destination",
            &obj.destination,
        )?;
        writer.write(xml::writer::XmlEvent::start_element("OutputSchemaVersion"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.output_schema_version
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct StorageClassAnalysisSchemaVersionDeserializer;
impl StorageClassAnalysisSchemaVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct StorageClassAnalysisSchemaVersionSerializer;
impl StorageClassAnalysisSchemaVersionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct SuffixDeserializer;
impl SuffixDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct SuffixSerializer;
impl SuffixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>Name of the tag.</p>
    pub key: String,
    /// <p>Value of the tag.</p>
    pub value: String,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = ObjectKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = ValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TagSerializer;
impl TagSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tag,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::start_element("Key"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.key
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::start_element("Value"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.value
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TagSetDeserializer;
impl TagSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Tag" {
                obj.push(TagDeserializer::deserialize("Tag", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct TagSetSerializer;
impl TagSetSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Tag>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            TagSerializer::serialize(writer, "Tag", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tagging {
    /// <p><p/></p>
    pub tag_set: Vec<Tag>,
}

pub struct TaggingSerializer;
impl TaggingSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Tagging,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        TagSetSerializer::serialize(&mut writer, "TagSet", &obj.tag_set)?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetBucketDeserializer;
impl TargetBucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TargetBucketSerializer;
impl TargetBucketSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TargetGrant {
    /// <p><p/></p>
    pub grantee: Option<Grantee>,
    /// <p>Logging permissions assigned to the Grantee for the bucket.</p>
    pub permission: Option<String>,
}

struct TargetGrantDeserializer;
impl TargetGrantDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TargetGrant, XmlParseError> {
        deserialize_elements::<_, TargetGrant, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Grantee" => {
                    obj.grantee = Some(GranteeDeserializer::deserialize("Grantee", stack)?);
                }
                "Permission" => {
                    obj.permission = Some(BucketLogsPermissionDeserializer::deserialize(
                        "Permission",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TargetGrantSerializer;
impl TargetGrantSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TargetGrant,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.grantee {
            &GranteeSerializer::serialize(&mut writer, "Grantee", value)?;
        }
        if let Some(ref value) = obj.permission {
            writer.write(xml::writer::XmlEvent::start_element("Permission"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TargetGrantsDeserializer;
impl TargetGrantsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TargetGrant>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Grant" {
                obj.push(TargetGrantDeserializer::deserialize("Grant", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

pub struct TargetGrantsSerializer;
impl TargetGrantsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<TargetGrant>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TargetPrefixSerializer;
impl TargetPrefixSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct TierSerializer;
impl TierSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TokenDeserializer;
impl TokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TokenSerializer;
impl TokenSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TopicArnDeserializer;
impl TopicArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TopicArnSerializer;
impl TopicArnSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p>A container for specifying the configuration for publication of messages to an Amazon Simple Notification Service (Amazon SNS) topic.when Amazon S3 detects specified events.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicConfiguration {
    /// <p><p/></p>
    pub events: Vec<String>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which Amazon S3 will publish a message when it detects events of the specified type.</p>
    pub topic_arn: String,
}

struct TopicConfigurationDeserializer;
impl TopicConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfiguration, XmlParseError> {
        deserialize_elements::<_, TopicConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Event" => {
                    obj.events
                        .extend(EventListDeserializer::deserialize("Event", stack)?);
                }
                "Filter" => {
                    obj.filter = Some(NotificationConfigurationFilterDeserializer::deserialize(
                        "Filter", stack,
                    )?);
                }
                "Id" => {
                    obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                }
                "Topic" => {
                    obj.topic_arn = TopicArnDeserializer::deserialize("Topic", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TopicConfigurationSerializer;
impl TopicConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TopicConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        EventListSerializer::serialize(&mut writer, "Event", &obj.events)?;
        if let Some(ref value) = obj.filter {
            &NotificationConfigurationFilterSerializer::serialize(&mut writer, "Filter", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.topic_arn
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicConfigurationDeprecated {
    /// <p><p/></p>
    pub events: Option<Vec<String>>,
    pub id: Option<String>,
    /// <p>Amazon SNS topic to which Amazon S3 will publish a message to report the specified events for the bucket.</p>
    pub topic: Option<String>,
}

struct TopicConfigurationDeprecatedDeserializer;
impl TopicConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TopicConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, TopicConfigurationDeprecated, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Event" => {
                        obj.events
                            .get_or_insert(vec![])
                            .extend(EventListDeserializer::deserialize("Event", stack)?);
                    }
                    "Id" => {
                        obj.id = Some(NotificationIdDeserializer::deserialize("Id", stack)?);
                    }
                    "Topic" => {
                        obj.topic = Some(TopicArnDeserializer::deserialize("Topic", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}

pub struct TopicConfigurationDeprecatedSerializer;
impl TopicConfigurationDeprecatedSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &TopicConfigurationDeprecated,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.events {
            &EventListSerializer::serialize(&mut writer, "Event", value)?;
        }
        if let Some(ref value) = obj.id {
            writer.write(xml::writer::XmlEvent::start_element("Id"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.topic {
            writer.write(xml::writer::XmlEvent::start_element("Topic"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TopicConfigurationListDeserializer;
impl TopicConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TopicConfiguration>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(TopicConfigurationDeserializer::deserialize(
                    tag_name, stack,
                )?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<TopicConfiguration>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        for element in obj {
            TopicConfigurationSerializer::serialize(writer, name, element)?;
        }
        Ok(())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transition {
    /// <p>Indicates at what date the object is to be moved or deleted. Should be in GMT ISO 8601 Format.</p>
    pub date: Option<String>,
    /// <p>Indicates the lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer.</p>
    pub days: Option<i64>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: Option<String>,
}

struct TransitionDeserializer;
impl TransitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Transition, XmlParseError> {
        deserialize_elements::<_, Transition, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Date" => {
                    obj.date = Some(DateDeserializer::deserialize("Date", stack)?);
                }
                "Days" => {
                    obj.days = Some(DaysDeserializer::deserialize("Days", stack)?);
                }
                "StorageClass" => {
                    obj.storage_class = Some(TransitionStorageClassDeserializer::deserialize(
                        "StorageClass",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

pub struct TransitionSerializer;
impl TransitionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Transition,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.date {
            writer.write(xml::writer::XmlEvent::start_element("Date"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.days {
            writer.write(xml::writer::XmlEvent::start_element("Days"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.storage_class {
            writer.write(xml::writer::XmlEvent::start_element("StorageClass"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TransitionListDeserializer;
impl TransitionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Transition>, XmlParseError> {
        let mut obj = vec![];

        loop {
            let consume_next_tag = match stack.peek() {
                Some(&Ok(xml::reader::XmlEvent::StartElement { ref name, .. })) => {
                    name.local_name == tag_name
                }
                _ => false,
            };

            if consume_next_tag {
                obj.push(TransitionDeserializer::deserialize(tag_name, stack)?);
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
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<Transition>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TransitionStorageClassSerializer;
impl TransitionStorageClassSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct TypeDeserializer;
impl TypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct TypeSerializer;
impl TypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct URIDeserializer;
impl URIDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct URISerializer;
impl URISerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct UploadIdMarkerDeserializer;
impl UploadIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct UploadIdMarkerSerializer;
impl UploadIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartCopyRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>The name of the source bucket and key name of the source object, separated by a slash (/). Must be URL-encoded.</p>
    pub copy_source: String,
    /// <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    pub copy_source_if_match: Option<String>,
    /// <p>Copies the object if it has been modified since the specified time.</p>
    pub copy_source_if_modified_since: Option<String>,
    /// <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    pub copy_source_if_none_match: Option<String>,
    /// <p>Copies the object if it hasn't been modified since the specified time.</p>
    pub copy_source_if_unmodified_since: Option<String>,
    /// <p>The range of bytes to copy from the source object. The range value must use the form bytes=first-last, where the first and last are the zero-based byte offsets to copy. For example, bytes=0-9 indicates that you want to copy the first ten bytes of the source. You can copy a range only if the source object is greater than 5 MB.</p>
    pub copy_source_range: Option<String>,
    /// <p>Specifies the algorithm to use when decrypting the source object (e.g., AES256).</p>
    pub copy_source_sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.</p>
    pub copy_source_sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub copy_source_sse_customer_key_md5: Option<String>,
    /// <p><p/></p>
    pub key: String,
    /// <p>Part number of part being copied. This is a positive integer between 1 and 10,000.</p>
    pub part_number: i64,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose part is being copied.</p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartCopyResponse {
    /// <p><p/></p>
    pub copy_part_result: Option<CopyPartResult>,
    /// <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    pub copy_source_version_id: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
}

struct UploadPartCopyResponseDeserializer;
impl UploadPartCopyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartCopyResponse, XmlParseError> {
        Ok(UploadPartCopyResponse {
            copy_part_result: Some(CopyPartResultDeserializer::deserialize(
                "CopyPartResult",
                stack,
            )?),
            ..UploadPartCopyResponse::default()
        })
    }
}
#[derive(Default, Debug)]
pub struct UploadPartRequest {
    /// <p>Object data.</p>
    pub body: Option<StreamingBody>,
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: String,
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub content_length: Option<i64>,
    /// <p>The base64-encoded 128-bit MD5 digest of the part data.</p>
    pub content_md5: Option<String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub key: String,
    /// <p>Part number of part being uploaded. This is a positive integer between 1 and 10,000.</p>
    pub part_number: i64,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Upload ID identifying the multipart upload whose part is being uploaded.</p>
    pub upload_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UploadPartResponse {
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
}

struct UploadPartResponseDeserializer;
impl UploadPartResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UploadPartResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct UserMetadataSerializer;
impl UserMetadataSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &Vec<MetadataEntry>,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        for element in obj {
            MetadataEntrySerializer::serialize(writer, "MetadataEntry", element)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

struct ValueDeserializer;
impl ValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct ValueSerializer;
impl ValueSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct VersionIdMarkerDeserializer;
impl VersionIdMarkerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct VersionIdMarkerSerializer;
impl VersionIdMarkerSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VersioningConfiguration {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<String>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<String>,
}

pub struct VersioningConfigurationSerializer;
impl VersioningConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &VersioningConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.mfa_delete {
            writer.write(xml::writer::XmlEvent::start_element("MfaDelete"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        if let Some(ref value) = obj.status {
            writer.write(xml::writer::XmlEvent::start_element("Status"))?;
            writer.write(xml::writer::XmlEvent::characters(&format!(
                "{value}",
                value = value
            )));
            writer.write(xml::writer::XmlEvent::end_element())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebsiteConfiguration {
    /// <p><p/></p>
    pub error_document: Option<ErrorDocument>,
    /// <p><p/></p>
    pub index_document: Option<IndexDocument>,
    /// <p><p/></p>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p><p/></p>
    pub routing_rules: Option<Vec<RoutingRule>>,
}

pub struct WebsiteConfigurationSerializer;
impl WebsiteConfigurationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &WebsiteConfiguration,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.error_document {
            &ErrorDocumentSerializer::serialize(&mut writer, "ErrorDocument", value)?;
        }
        if let Some(ref value) = obj.index_document {
            &IndexDocumentSerializer::serialize(&mut writer, "IndexDocument", value)?;
        }
        if let Some(ref value) = obj.redirect_all_requests_to {
            &RedirectAllRequestsToSerializer::serialize(
                &mut writer,
                "RedirectAllRequestsTo",
                value,
            )?;
        }
        if let Some(ref value) = obj.routing_rules {
            &RoutingRulesSerializer::serialize(&mut writer, "RoutingRules", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

struct YearsDeserializer;
impl YearsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

pub struct YearsSerializer;
impl YearsSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &i64,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(
            "{value}",
            value = obj.to_string()
        )))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

/// Errors returned by AbortMultipartUpload
#[derive(Debug, PartialEq)]
pub enum AbortMultipartUploadError {
    /// <p>The specified multipart upload does not exist.</p>
    NoSuchUpload(String),
}

impl AbortMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AbortMultipartUploadError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchUpload" => {
                        return RusotoError::Service(AbortMultipartUploadError::NoSuchUpload(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by CompleteMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CompleteMultipartUploadError {}

impl CompleteMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteMultipartUploadError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CompleteMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompleteMultipartUploadError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by CopyObject
#[derive(Debug, PartialEq)]
pub enum CopyObjectError {
    /// <p>The source object of the COPY operation is not in the active tier and is only stored in Amazon Glacier.</p>
    ObjectNotInActiveTierError(String),
}

impl CopyObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ObjectNotInActiveTierError" => {
                        return RusotoError::Service(CopyObjectError::ObjectNotInActiveTierError(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by CreateBucket
#[derive(Debug, PartialEq)]
pub enum CreateBucketError {
    /// <p>The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again.</p>
    BucketAlreadyExists(String),
    /// <p><p/></p>
    BucketAlreadyOwnedByYou(String),
}

impl CreateBucketError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBucketError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "BucketAlreadyExists" => {
                        return RusotoError::Service(CreateBucketError::BucketAlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "BucketAlreadyOwnedByYou" => {
                        return RusotoError::Service(CreateBucketError::BucketAlreadyOwnedByYou(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by CreateMultipartUpload
#[derive(Debug, PartialEq)]
pub enum CreateMultipartUploadError {}

impl CreateMultipartUploadError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMultipartUploadError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateMultipartUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMultipartUploadError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucket
#[derive(Debug, PartialEq)]
pub enum DeleteBucketError {}

impl DeleteBucketError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketAnalyticsConfigurationError {}

impl DeleteBucketAnalyticsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBucketAnalyticsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketCors
#[derive(Debug, PartialEq)]
pub enum DeleteBucketCorsError {}

impl DeleteBucketCorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketCorsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketCorsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketEncryption
#[derive(Debug, PartialEq)]
pub enum DeleteBucketEncryptionError {}

impl DeleteBucketEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketEncryptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketInventoryConfigurationError {}

impl DeleteBucketInventoryConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBucketInventoryConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum DeleteBucketLifecycleError {}

impl DeleteBucketLifecycleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketLifecycleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteBucketMetricsConfigurationError {}

impl DeleteBucketMetricsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBucketMetricsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteBucketPolicyError {}

impl DeleteBucketPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketPolicyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketReplication
#[derive(Debug, PartialEq)]
pub enum DeleteBucketReplicationError {}

impl DeleteBucketReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketReplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketReplicationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketTagging
#[derive(Debug, PartialEq)]
pub enum DeleteBucketTaggingError {}

impl DeleteBucketTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteBucketWebsite
#[derive(Debug, PartialEq)]
pub enum DeleteBucketWebsiteError {}

impl DeleteBucketWebsiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBucketWebsiteError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteObject
#[derive(Debug, PartialEq)]
pub enum DeleteObjectError {}

impl DeleteObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteObjectTagging
#[derive(Debug, PartialEq)]
pub enum DeleteObjectTaggingError {}

impl DeleteObjectTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteObjectTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeleteObjects
#[derive(Debug, PartialEq)]
pub enum DeleteObjectsError {}

impl DeleteObjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteObjectsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteObjectsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DeletePublicAccessBlock
#[derive(Debug, PartialEq)]
pub enum DeletePublicAccessBlockError {}

impl DeletePublicAccessBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePublicAccessBlockError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeletePublicAccessBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePublicAccessBlockError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAccelerateConfigurationError {}

impl GetBucketAccelerateConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketAccelerateConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketAcl
#[derive(Debug, PartialEq)]
pub enum GetBucketAclError {}

impl GetBucketAclError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketAclError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAclError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketAnalyticsConfigurationError {}

impl GetBucketAnalyticsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketAnalyticsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketCors
#[derive(Debug, PartialEq)]
pub enum GetBucketCorsError {}

impl GetBucketCorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketCorsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketCorsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketEncryption
#[derive(Debug, PartialEq)]
pub enum GetBucketEncryptionError {}

impl GetBucketEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketEncryptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketInventoryConfigurationError {}

impl GetBucketInventoryConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketInventoryConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleError {}

impl GetBucketLifecycleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketLifecycleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketLifecycleConfigurationError {}

impl GetBucketLifecycleConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketLifecycleConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketLocation
#[derive(Debug, PartialEq)]
pub enum GetBucketLocationError {}

impl GetBucketLocationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketLocationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLocationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketLogging
#[derive(Debug, PartialEq)]
pub enum GetBucketLoggingError {}

impl GetBucketLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketLoggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketLoggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketMetricsConfigurationError {}

impl GetBucketMetricsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketMetricsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketNotification
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationError {}

impl GetBucketNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketNotificationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetBucketNotificationConfigurationError {}

impl GetBucketNotificationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetBucketNotificationConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketPolicy
#[derive(Debug, PartialEq)]
pub enum GetBucketPolicyError {}

impl GetBucketPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketPolicyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketPolicyStatus
#[derive(Debug, PartialEq)]
pub enum GetBucketPolicyStatusError {}

impl GetBucketPolicyStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketPolicyStatusError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketPolicyStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketPolicyStatusError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketReplication
#[derive(Debug, PartialEq)]
pub enum GetBucketReplicationError {}

impl GetBucketReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketReplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketReplicationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum GetBucketRequestPaymentError {}

impl GetBucketRequestPaymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketRequestPaymentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketTagging
#[derive(Debug, PartialEq)]
pub enum GetBucketTaggingError {}

impl GetBucketTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketVersioning
#[derive(Debug, PartialEq)]
pub enum GetBucketVersioningError {}

impl GetBucketVersioningError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketVersioningError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketVersioningError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetBucketWebsite
#[derive(Debug, PartialEq)]
pub enum GetBucketWebsiteError {}

impl GetBucketWebsiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBucketWebsiteError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetObject
#[derive(Debug, PartialEq)]
pub enum GetObjectError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
}

impl GetObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchKey" => {
                        return RusotoError::Service(GetObjectError::NoSuchKey(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by GetObjectAcl
#[derive(Debug, PartialEq)]
pub enum GetObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
}

impl GetObjectAclError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectAclError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchKey" => {
                        return RusotoError::Service(GetObjectAclError::NoSuchKey(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by GetObjectLegalHold
#[derive(Debug, PartialEq)]
pub enum GetObjectLegalHoldError {}

impl GetObjectLegalHoldError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectLegalHoldError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetObjectLegalHoldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectLegalHoldError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetObjectLockConfiguration
#[derive(Debug, PartialEq)]
pub enum GetObjectLockConfigurationError {}

impl GetObjectLockConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetObjectLockConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetObjectLockConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectLockConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetObjectRetention
#[derive(Debug, PartialEq)]
pub enum GetObjectRetentionError {}

impl GetObjectRetentionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectRetentionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetObjectRetentionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectRetentionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetObjectTagging
#[derive(Debug, PartialEq)]
pub enum GetObjectTaggingError {}

impl GetObjectTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetObjectTorrent
#[derive(Debug, PartialEq)]
pub enum GetObjectTorrentError {}

impl GetObjectTorrentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetObjectTorrentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetObjectTorrentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetObjectTorrentError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetPublicAccessBlock
#[derive(Debug, PartialEq)]
pub enum GetPublicAccessBlockError {}

impl GetPublicAccessBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPublicAccessBlockError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetPublicAccessBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPublicAccessBlockError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by HeadBucket
#[derive(Debug, PartialEq)]
pub enum HeadBucketError {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
}

impl HeadBucketError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<HeadBucketError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        return RusotoError::Service(HeadBucketError::NoSuchBucket(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by HeadObject
#[derive(Debug, PartialEq)]
pub enum HeadObjectError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
}

impl HeadObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<HeadObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchKey" => {
                        return RusotoError::Service(HeadObjectError::NoSuchKey(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListBucketAnalyticsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketAnalyticsConfigurationsError {}

impl ListBucketAnalyticsConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListBucketAnalyticsConfigurationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListBucketAnalyticsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketAnalyticsConfigurationsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListBucketInventoryConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketInventoryConfigurationsError {}

impl ListBucketInventoryConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListBucketInventoryConfigurationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListBucketInventoryConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketInventoryConfigurationsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListBucketMetricsConfigurations
#[derive(Debug, PartialEq)]
pub enum ListBucketMetricsConfigurationsError {}

impl ListBucketMetricsConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListBucketMetricsConfigurationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListBucketMetricsConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketMetricsConfigurationsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListBuckets
#[derive(Debug, PartialEq)]
pub enum ListBucketsError {}

impl ListBucketsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBucketsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListBucketsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBucketsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListMultipartUploads
#[derive(Debug, PartialEq)]
pub enum ListMultipartUploadsError {}

impl ListMultipartUploadsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMultipartUploadsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListMultipartUploadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMultipartUploadsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListObjectVersions
#[derive(Debug, PartialEq)]
pub enum ListObjectVersionsError {}

impl ListObjectVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectVersionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListObjectVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListObjectVersionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListObjects
#[derive(Debug, PartialEq)]
pub enum ListObjectsError {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
}

impl ListObjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        return RusotoError::Service(ListObjectsError::NoSuchBucket(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListObjectsV2
#[derive(Debug, PartialEq)]
pub enum ListObjectsV2Error {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(String),
}

impl ListObjectsV2Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListObjectsV2Error> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchBucket" => {
                        return RusotoError::Service(ListObjectsV2Error::NoSuchBucket(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListParts
#[derive(Debug, PartialEq)]
pub enum ListPartsError {}

impl ListPartsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPartsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPartsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketAccelerateConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAccelerateConfigurationError {}

impl PutBucketAccelerateConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketAccelerateConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketAccelerateConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAccelerateConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketAcl
#[derive(Debug, PartialEq)]
pub enum PutBucketAclError {}

impl PutBucketAclError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketAclError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketAclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAclError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketAnalyticsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketAnalyticsConfigurationError {}

impl PutBucketAnalyticsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketAnalyticsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketAnalyticsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketAnalyticsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketCors
#[derive(Debug, PartialEq)]
pub enum PutBucketCorsError {}

impl PutBucketCorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketCorsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketCorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketCorsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketEncryption
#[derive(Debug, PartialEq)]
pub enum PutBucketEncryptionError {}

impl PutBucketEncryptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketEncryptionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketEncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketEncryptionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketInventoryConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketInventoryConfigurationError {}

impl PutBucketInventoryConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketInventoryConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketInventoryConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketInventoryConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketLifecycle
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleError {}

impl PutBucketLifecycleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketLifecycleError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketLifecycleConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketLifecycleConfigurationError {}

impl PutBucketLifecycleConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketLifecycleConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLifecycleConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketLogging
#[derive(Debug, PartialEq)]
pub enum PutBucketLoggingError {}

impl PutBucketLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketLoggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketLoggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketLoggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketMetricsConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketMetricsConfigurationError {}

impl PutBucketMetricsConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketMetricsConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketMetricsConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketMetricsConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketNotification
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationError {}

impl PutBucketNotificationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketNotificationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketNotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketNotificationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutBucketNotificationConfigurationError {}

impl PutBucketNotificationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutBucketNotificationConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketNotificationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketNotificationConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketPolicy
#[derive(Debug, PartialEq)]
pub enum PutBucketPolicyError {}

impl PutBucketPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketPolicyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketPolicyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketReplication
#[derive(Debug, PartialEq)]
pub enum PutBucketReplicationError {}

impl PutBucketReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketReplicationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketReplicationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketRequestPayment
#[derive(Debug, PartialEq)]
pub enum PutBucketRequestPaymentError {}

impl PutBucketRequestPaymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketRequestPaymentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketRequestPaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketRequestPaymentError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketTagging
#[derive(Debug, PartialEq)]
pub enum PutBucketTaggingError {}

impl PutBucketTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketVersioning
#[derive(Debug, PartialEq)]
pub enum PutBucketVersioningError {}

impl PutBucketVersioningError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketVersioningError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketVersioningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketVersioningError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutBucketWebsite
#[derive(Debug, PartialEq)]
pub enum PutBucketWebsiteError {}

impl PutBucketWebsiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBucketWebsiteError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutBucketWebsiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBucketWebsiteError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutObject
#[derive(Debug, PartialEq)]
pub enum PutObjectError {}

impl PutObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutObjectAcl
#[derive(Debug, PartialEq)]
pub enum PutObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(String),
}

impl PutObjectAclError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectAclError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "NoSuchKey" => {
                        return RusotoError::Service(PutObjectAclError::NoSuchKey(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by PutObjectLegalHold
#[derive(Debug, PartialEq)]
pub enum PutObjectLegalHoldError {}

impl PutObjectLegalHoldError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectLegalHoldError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutObjectLegalHoldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectLegalHoldError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutObjectLockConfiguration
#[derive(Debug, PartialEq)]
pub enum PutObjectLockConfigurationError {}

impl PutObjectLockConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutObjectLockConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutObjectLockConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectLockConfigurationError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutObjectRetention
#[derive(Debug, PartialEq)]
pub enum PutObjectRetentionError {}

impl PutObjectRetentionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectRetentionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutObjectRetentionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectRetentionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutObjectTagging
#[derive(Debug, PartialEq)]
pub enum PutObjectTaggingError {}

impl PutObjectTaggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutObjectTaggingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutObjectTaggingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutObjectTaggingError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by PutPublicAccessBlock
#[derive(Debug, PartialEq)]
pub enum PutPublicAccessBlockError {}

impl PutPublicAccessBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPublicAccessBlockError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutPublicAccessBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPublicAccessBlockError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by RestoreObject
#[derive(Debug, PartialEq)]
pub enum RestoreObjectError {
    /// <p>This operation is not allowed against this storage tier</p>
    ObjectAlreadyInActiveTierError(String),
}

impl RestoreObjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreObjectError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ObjectAlreadyInActiveTierError" => {
                        return RusotoError::Service(
                            RestoreObjectError::ObjectAlreadyInActiveTierError(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by SelectObjectContent
#[derive(Debug, PartialEq)]
pub enum SelectObjectContentError {}

impl SelectObjectContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SelectObjectContentError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SelectObjectContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SelectObjectContentError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by UploadPart
#[derive(Debug, PartialEq)]
pub enum UploadPartError {}

impl UploadPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadPartError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UploadPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by UploadPartCopy
#[derive(Debug, PartialEq)]
pub enum UploadPartCopyError {}

impl UploadPartCopyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UploadPartCopyError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UploadPartCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UploadPartCopyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Trait representing the capabilities of the Amazon S3 API. Amazon S3 clients implement this trait.
pub trait S3 {
    /// <p>Aborts a multipart upload.</p> <p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadRequest,
    ) -> Request<AbortMultipartUploadRequest>;

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> Request<CompleteMultipartUploadRequest>;

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    fn copy_object(&self, input: CopyObjectRequest) -> Request<CopyObjectRequest>;

    /// <p>Creates a new bucket.</p>
    fn create_bucket(&self, input: CreateBucketRequest) -> Request<CreateBucketRequest>;

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> Request<CreateMultipartUploadRequest>;

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    fn delete_bucket(&self, input: DeleteBucketRequest) -> Request<DeleteBucketRequest>;

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> Request<DeleteBucketAnalyticsConfigurationRequest>;

    /// <p>Deletes the CORS configuration information set for the bucket.</p>
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> Request<DeleteBucketCorsRequest>;

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> Request<DeleteBucketEncryptionRequest>;

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> Request<DeleteBucketInventoryConfigurationRequest>;

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> Request<DeleteBucketLifecycleRequest>;

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> Request<DeleteBucketMetricsConfigurationRequest>;

    /// <p>Deletes the policy from the bucket.</p>
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> Request<DeleteBucketPolicyRequest>;

    /// <p> Deletes the replication configuration from the bucket. For information about replication configuration, see <a href=" https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> Request<DeleteBucketReplicationRequest>;

    /// <p>Deletes the tags from the bucket.</p>
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> Request<DeleteBucketTaggingRequest>;

    /// <p>This operation removes the website configuration from the bucket.</p>
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> Request<DeleteBucketWebsiteRequest>;

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    fn delete_object(&self, input: DeleteObjectRequest) -> Request<DeleteObjectRequest>;

    /// <p>Removes the tag-set from an existing object.</p>
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> Request<DeleteObjectTaggingRequest>;

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    fn delete_objects(&self, input: DeleteObjectsRequest) -> Request<DeleteObjectsRequest>;

    /// <p>Removes the <code>PublicAccessBlock</code> configuration from an Amazon S3 bucket.</p>
    fn delete_public_access_block(
        &self,
        input: DeletePublicAccessBlockRequest,
    ) -> Request<DeletePublicAccessBlockRequest>;

    /// <p>Returns the accelerate configuration of a bucket.</p>
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> Request<GetBucketAccelerateConfigurationRequest>;

    /// <p>Gets the access control policy for the bucket.</p>
    fn get_bucket_acl(&self, input: GetBucketAclRequest) -> Request<GetBucketAclRequest>;

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> Request<GetBucketAnalyticsConfigurationRequest>;

    /// <p>Returns the CORS configuration for the bucket.</p>
    fn get_bucket_cors(&self, input: GetBucketCorsRequest) -> Request<GetBucketCorsRequest>;

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> Request<GetBucketEncryptionRequest>;

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> Request<GetBucketInventoryConfigurationRequest>;

    /// <p> No longer used, see the GetBucketLifecycleConfiguration operation.</p>
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> Request<GetBucketLifecycleRequest>;

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> Request<GetBucketLifecycleConfigurationRequest>;

    /// <p>Returns the region the bucket resides in.</p>
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> Request<GetBucketLocationRequest>;

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> Request<GetBucketLoggingRequest>;

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> Request<GetBucketMetricsConfigurationRequest>;

    /// <p> No longer used, see the GetBucketNotificationConfiguration operation.</p>
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationRequest,
    ) -> Request<GetBucketNotificationRequest>;

    /// <p>Returns the notification configuration of a bucket.</p>
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> Request<GetBucketNotificationConfigurationRequest>;

    /// <p>Returns the policy of a specified bucket.</p>
    fn get_bucket_policy(&self, input: GetBucketPolicyRequest) -> Request<GetBucketPolicyRequest>;

    /// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public.</p>
    fn get_bucket_policy_status(
        &self,
        input: GetBucketPolicyStatusRequest,
    ) -> Request<GetBucketPolicyStatusRequest>;

    /// <p><p>Returns the replication configuration of a bucket.</p> <note> <p> It can take a while to propagate the put or delete a replication configuration to all Amazon S3 systems. Therefore, a get request soon after put or delete can return a wrong result. </p> </note></p>
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> Request<GetBucketReplicationRequest>;

    /// <p>Returns the request payment configuration of a bucket.</p>
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> Request<GetBucketRequestPaymentRequest>;

    /// <p>Returns the tag set associated with the bucket.</p>
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> Request<GetBucketTaggingRequest>;

    /// <p>Returns the versioning state of a bucket.</p>
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> Request<GetBucketVersioningRequest>;

    /// <p>Returns the website configuration for a bucket.</p>
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> Request<GetBucketWebsiteRequest>;

    /// <p>Retrieves objects from Amazon S3.</p>
    fn get_object(&self, input: GetObjectRequest) -> Request<GetObjectRequest>;

    /// <p>Returns the access control list (ACL) of an object.</p>
    fn get_object_acl(&self, input: GetObjectAclRequest) -> Request<GetObjectAclRequest>;

    /// <p>Gets an object's current Legal Hold status.</p>
    fn get_object_legal_hold(
        &self,
        input: GetObjectLegalHoldRequest,
    ) -> Request<GetObjectLegalHoldRequest>;

    /// <p>Gets the Object Lock configuration for a bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn get_object_lock_configuration(
        &self,
        input: GetObjectLockConfigurationRequest,
    ) -> Request<GetObjectLockConfigurationRequest>;

    /// <p>Retrieves an object's retention settings.</p>
    fn get_object_retention(
        &self,
        input: GetObjectRetentionRequest,
    ) -> Request<GetObjectRetentionRequest>;

    /// <p>Returns the tag-set of an object.</p>
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> Request<GetObjectTaggingRequest>;

    /// <p>Return torrent files from a bucket.</p>
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> Request<GetObjectTorrentRequest>;

    /// <p>Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn get_public_access_block(
        &self,
        input: GetPublicAccessBlockRequest,
    ) -> Request<GetPublicAccessBlockRequest>;

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    fn head_bucket(&self, input: HeadBucketRequest) -> Request<HeadBucketRequest>;

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    fn head_object(&self, input: HeadObjectRequest) -> Request<HeadObjectRequest>;

    /// <p>Lists the analytics configurations for the bucket.</p>
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> Request<ListBucketAnalyticsConfigurationsRequest>;

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> Request<ListBucketInventoryConfigurationsRequest>;

    /// <p>Lists the metrics configurations for the bucket.</p>
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> Request<ListBucketMetricsConfigurationsRequest>;

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    fn list_buckets(&self) -> Request<ListBucketsRequest>;

    /// <p>This operation lists in-progress multipart uploads.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> Request<ListMultipartUploadsRequest>;

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> Request<ListObjectVersionsRequest>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    fn list_objects(&self, input: ListObjectsRequest) -> Request<ListObjectsRequest>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    fn list_objects_v2(&self, input: ListObjectsV2Request) -> Request<ListObjectsV2Request>;

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    fn list_parts(&self, input: ListPartsRequest) -> Request<ListPartsRequest>;

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> Request<PutBucketAccelerateConfigurationRequest>;

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> Request<PutBucketAclRequest>;

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> Request<PutBucketAnalyticsConfigurationRequest>;

    /// <p>Sets the CORS configuration for a bucket.</p>
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> Request<PutBucketCorsRequest>;

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> Request<PutBucketEncryptionRequest>;

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> Request<PutBucketInventoryConfigurationRequest>;

    /// <p> No longer used, see the PutBucketLifecycleConfiguration operation.</p>
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> Request<PutBucketLifecycleRequest>;

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> Request<PutBucketLifecycleConfigurationRequest>;

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> Request<PutBucketLoggingRequest>;

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> Request<PutBucketMetricsConfigurationRequest>;

    /// <p> No longer used, see the PutBucketNotificationConfiguration operation.</p>
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> Request<PutBucketNotificationRequest>;

    /// <p>Enables notifications of specified events for a bucket.</p>
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> Request<PutBucketNotificationConfigurationRequest>;

    /// <p>Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it.</p>
    fn put_bucket_policy(&self, input: PutBucketPolicyRequest) -> Request<PutBucketPolicyRequest>;

    /// <p> Creates a replication configuration or replaces an existing one. For more information, see <a href=" https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> Request<PutBucketReplicationRequest>;

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> Request<PutBucketRequestPaymentRequest>;

    /// <p>Sets the tags for a bucket.</p>
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> Request<PutBucketTaggingRequest>;

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> Request<PutBucketVersioningRequest>;

    /// <p>Set the website configuration for a bucket.</p>
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> Request<PutBucketWebsiteRequest>;

    /// <p>Adds an object to a bucket.</p>
    fn put_object(&self, input: PutObjectRequest) -> Request<PutObjectRequest>;

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    fn put_object_acl(&self, input: PutObjectAclRequest) -> Request<PutObjectAclRequest>;

    /// <p>Applies a Legal Hold configuration to the specified object.</p>
    fn put_object_legal_hold(
        &self,
        input: PutObjectLegalHoldRequest,
    ) -> Request<PutObjectLegalHoldRequest>;

    /// <p>Places an Object Lock configuration on the specified bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn put_object_lock_configuration(
        &self,
        input: PutObjectLockConfigurationRequest,
    ) -> Request<PutObjectLockConfigurationRequest>;

    /// <p>Places an Object Retention configuration on an object.</p>
    fn put_object_retention(
        &self,
        input: PutObjectRetentionRequest,
    ) -> Request<PutObjectRetentionRequest>;

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> Request<PutObjectTaggingRequest>;

    /// <p>Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn put_public_access_block(
        &self,
        input: PutPublicAccessBlockRequest,
    ) -> Request<PutPublicAccessBlockRequest>;

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    fn restore_object(&self, input: RestoreObjectRequest) -> Request<RestoreObjectRequest>;

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> Request<SelectObjectContentRequest>;

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn upload_part(&self, input: UploadPartRequest) -> Request<UploadPartRequest>;

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    fn upload_part_copy(&self, input: UploadPartCopyRequest) -> Request<UploadPartCopyRequest>;
}
/// A client for the Amazon S3 API.
#[derive(Clone)]
pub struct S3Client {
    client: Client,
    region: region::Region,
}

impl S3Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> S3Client {
        S3Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> S3Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        S3Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl S3 for S3Client {
    /// <p>Aborts a multipart upload.</p> <p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadRequest,
    ) -> Request<AbortMultipartUploadRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> Request<CompleteMultipartUploadRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    fn copy_object(&self, input: CopyObjectRequest) -> Request<CopyObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a new bucket.</p>
    fn create_bucket(&self, input: CreateBucketRequest) -> Request<CreateBucketRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> Request<CreateMultipartUploadRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    fn delete_bucket(&self, input: DeleteBucketRequest) -> Request<DeleteBucketRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> Request<DeleteBucketAnalyticsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the CORS configuration information set for the bucket.</p>
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> Request<DeleteBucketCorsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> Request<DeleteBucketEncryptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> Request<DeleteBucketInventoryConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> Request<DeleteBucketLifecycleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> Request<DeleteBucketMetricsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the policy from the bucket.</p>
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> Request<DeleteBucketPolicyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> Deletes the replication configuration from the bucket. For information about replication configuration, see <a href=" https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> Request<DeleteBucketReplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the tags from the bucket.</p>
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> Request<DeleteBucketTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>This operation removes the website configuration from the bucket.</p>
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> Request<DeleteBucketWebsiteRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    fn delete_object(&self, input: DeleteObjectRequest) -> Request<DeleteObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes the tag-set from an existing object.</p>
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> Request<DeleteObjectTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    fn delete_objects(&self, input: DeleteObjectsRequest) -> Request<DeleteObjectsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes the <code>PublicAccessBlock</code> configuration from an Amazon S3 bucket.</p>
    fn delete_public_access_block(
        &self,
        input: DeletePublicAccessBlockRequest,
    ) -> Request<DeletePublicAccessBlockRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the accelerate configuration of a bucket.</p>
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> Request<GetBucketAccelerateConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the access control policy for the bucket.</p>
    fn get_bucket_acl(&self, input: GetBucketAclRequest) -> Request<GetBucketAclRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> Request<GetBucketAnalyticsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the CORS configuration for the bucket.</p>
    fn get_bucket_cors(&self, input: GetBucketCorsRequest) -> Request<GetBucketCorsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> Request<GetBucketEncryptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> Request<GetBucketInventoryConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> No longer used, see the GetBucketLifecycleConfiguration operation.</p>
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> Request<GetBucketLifecycleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> Request<GetBucketLifecycleConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the region the bucket resides in.</p>
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> Request<GetBucketLocationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> Request<GetBucketLoggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> Request<GetBucketMetricsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> No longer used, see the GetBucketNotificationConfiguration operation.</p>
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationRequest,
    ) -> Request<GetBucketNotificationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the notification configuration of a bucket.</p>
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> Request<GetBucketNotificationConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the policy of a specified bucket.</p>
    fn get_bucket_policy(&self, input: GetBucketPolicyRequest) -> Request<GetBucketPolicyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public.</p>
    fn get_bucket_policy_status(
        &self,
        input: GetBucketPolicyStatusRequest,
    ) -> Request<GetBucketPolicyStatusRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Returns the replication configuration of a bucket.</p> <note> <p> It can take a while to propagate the put or delete a replication configuration to all Amazon S3 systems. Therefore, a get request soon after put or delete can return a wrong result. </p> </note></p>
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> Request<GetBucketReplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the request payment configuration of a bucket.</p>
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> Request<GetBucketRequestPaymentRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the tag set associated with the bucket.</p>
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> Request<GetBucketTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the versioning state of a bucket.</p>
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> Request<GetBucketVersioningRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the website configuration for a bucket.</p>
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> Request<GetBucketWebsiteRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves objects from Amazon S3.</p>
    fn get_object(&self, input: GetObjectRequest) -> Request<GetObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the access control list (ACL) of an object.</p>
    fn get_object_acl(&self, input: GetObjectAclRequest) -> Request<GetObjectAclRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets an object's current Legal Hold status.</p>
    fn get_object_legal_hold(
        &self,
        input: GetObjectLegalHoldRequest,
    ) -> Request<GetObjectLegalHoldRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets the Object Lock configuration for a bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn get_object_lock_configuration(
        &self,
        input: GetObjectLockConfigurationRequest,
    ) -> Request<GetObjectLockConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves an object's retention settings.</p>
    fn get_object_retention(
        &self,
        input: GetObjectRetentionRequest,
    ) -> Request<GetObjectRetentionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the tag-set of an object.</p>
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> Request<GetObjectTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Return torrent files from a bucket.</p>
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> Request<GetObjectTorrentRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn get_public_access_block(
        &self,
        input: GetPublicAccessBlockRequest,
    ) -> Request<GetPublicAccessBlockRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    fn head_bucket(&self, input: HeadBucketRequest) -> Request<HeadBucketRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    fn head_object(&self, input: HeadObjectRequest) -> Request<HeadObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the analytics configurations for the bucket.</p>
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> Request<ListBucketAnalyticsConfigurationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> Request<ListBucketInventoryConfigurationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the metrics configurations for the bucket.</p>
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> Request<ListBucketMetricsConfigurationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    fn list_buckets(&self) -> Request<ListBucketsRequest> {
        Request::new(
            ListBucketsRequest {},
            self.region.clone(),
            self.client.clone(),
        )
    }

    /// <p>This operation lists in-progress multipart uploads.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> Request<ListMultipartUploadsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> Request<ListObjectVersionsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    fn list_objects(&self, input: ListObjectsRequest) -> Request<ListObjectsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    fn list_objects_v2(&self, input: ListObjectsV2Request) -> Request<ListObjectsV2Request> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    fn list_parts(&self, input: ListPartsRequest) -> Request<ListPartsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> Request<PutBucketAccelerateConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> Request<PutBucketAclRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> Request<PutBucketAnalyticsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the CORS configuration for a bucket.</p>
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> Request<PutBucketCorsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> Request<PutBucketEncryptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> Request<PutBucketInventoryConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> No longer used, see the PutBucketLifecycleConfiguration operation.</p>
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> Request<PutBucketLifecycleRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> Request<PutBucketLifecycleConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> Request<PutBucketLoggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> Request<PutBucketMetricsConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> No longer used, see the PutBucketNotificationConfiguration operation.</p>
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> Request<PutBucketNotificationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Enables notifications of specified events for a bucket.</p>
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> Request<PutBucketNotificationConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Replaces a policy on a bucket. If the bucket already has a policy, the one in this request completely replaces it.</p>
    fn put_bucket_policy(&self, input: PutBucketPolicyRequest) -> Request<PutBucketPolicyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p> Creates a replication configuration or replaces an existing one. For more information, see <a href=" https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> Request<PutBucketReplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> Request<PutBucketRequestPaymentRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the tags for a bucket.</p>
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> Request<PutBucketTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> Request<PutBucketVersioningRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Set the website configuration for a bucket.</p>
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> Request<PutBucketWebsiteRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds an object to a bucket.</p>
    fn put_object(&self, input: PutObjectRequest) -> Request<PutObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    fn put_object_acl(&self, input: PutObjectAclRequest) -> Request<PutObjectAclRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Applies a Legal Hold configuration to the specified object.</p>
    fn put_object_legal_hold(
        &self,
        input: PutObjectLegalHoldRequest,
    ) -> Request<PutObjectLegalHoldRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Places an Object Lock configuration on the specified bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn put_object_lock_configuration(
        &self,
        input: PutObjectLockConfigurationRequest,
    ) -> Request<PutObjectLockConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Places an Object Retention configuration on an object.</p>
    fn put_object_retention(
        &self,
        input: PutObjectRetentionRequest,
    ) -> Request<PutObjectRetentionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> Request<PutObjectTaggingRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn put_public_access_block(
        &self,
        input: PutPublicAccessBlockRequest,
    ) -> Request<PutPublicAccessBlockRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    fn restore_object(&self, input: RestoreObjectRequest) -> Request<RestoreObjectRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> Request<SelectObjectContentRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn upload_part(&self, input: UploadPartRequest) -> Request<UploadPartRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    fn upload_part_copy(&self, input: UploadPartCopyRequest) -> Request<UploadPartCopyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for AbortMultipartUploadRequest {
    type Output = AbortMultipartUploadResponse;
    type Error = AbortMultipartUploadError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &self.upload_id);
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AbortMultipartUploadError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = AbortMultipartUploadResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = AbortMultipartUploadResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CompleteMultipartUploadRequest {
    type Output = CompleteMultipartUploadResponse;
    type Error = CompleteMultipartUploadError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("POST", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &self.upload_id);
        request.set_params(params);
        if self.multipart_upload.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CompletedMultipartUploadSerializer::serialize(
                &mut writer,
                "CompleteMultipartUpload",
                self.multipart_upload.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CompleteMultipartUploadError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CompleteMultipartUploadResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CompleteMultipartUploadResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(expiration) = response.headers.get("x-amz-expiration") {
                    let value = expiration.to_owned();
                    result.expiration = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CopyObjectRequest {
    type Output = CopyObjectResponse;
    type Error = CopyObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = self.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = self.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = self.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = self.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = self.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }
        request.add_header("x-amz-copy-source", &self.copy_source);

        if let Some(ref copy_source_if_match) = self.copy_source_if_match {
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = self.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = self.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = self.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            self.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = self.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = self.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref expires) = self.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = self.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref metadata_directive) = self.metadata_directive {
            request.add_header("x-amz-metadata-directive", &metadata_directive.to_string());
        }

        if let Some(ref object_lock_legal_hold_status) = self.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = self.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = self.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = self.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = self.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = self.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = self.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref tagging_directive) = self.tagging_directive {
            request.add_header("x-amz-tagging-directive", &tagging_directive.to_string());
        }

        if let Some(ref website_redirect_location) = self.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CopyObjectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CopyObjectResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        CopyObjectResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id")
                {
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
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateBucketRequest {
    type Output = CreateBucketResponse;
    type Error = CreateBucketError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = self.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref object_lock_enabled_for_bucket) = self.object_lock_enabled_for_bucket {
            request.add_header(
                "x-amz-bucket-object-lock-enabled",
                &object_lock_enabled_for_bucket.to_string(),
            );
        }

        if self.create_bucket_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CreateBucketConfigurationSerializer::serialize(
                &mut writer,
                "CreateBucketConfiguration",
                self.create_bucket_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBucketError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateBucketResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CreateBucketResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateMultipartUploadRequest {
    type Output = CreateMultipartUploadResponse;
    type Error = CreateMultipartUploadError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("POST", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = self.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = self.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = self.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = self.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_type) = self.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = self.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = self.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref object_lock_legal_hold_status) = self.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = self.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = self.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = self.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = self.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = self.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = self.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = self.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }
        let mut params = Params::new();
        params.put_key("uploads");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateMultipartUploadError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateMultipartUploadResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CreateMultipartUploadResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
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
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketRequest {
    type Output = DeleteBucketResponse;
    type Error = DeleteBucketError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketAnalyticsConfigurationRequest {
    type Output = DeleteBucketAnalyticsConfigurationResponse;
    type Error = DeleteBucketAnalyticsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("analytics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketAnalyticsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketAnalyticsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketAnalyticsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketCorsRequest {
    type Output = DeleteBucketCorsResponse;
    type Error = DeleteBucketCorsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketCorsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketCorsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketCorsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketEncryptionRequest {
    type Output = DeleteBucketEncryptionResponse;
    type Error = DeleteBucketEncryptionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketEncryptionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketEncryptionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketEncryptionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketInventoryConfigurationRequest {
    type Output = DeleteBucketInventoryConfigurationResponse;
    type Error = DeleteBucketInventoryConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("inventory");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketInventoryConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketInventoryConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketInventoryConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketLifecycleRequest {
    type Output = DeleteBucketLifecycleResponse;
    type Error = DeleteBucketLifecycleError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketLifecycleError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketLifecycleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketLifecycleResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketMetricsConfigurationRequest {
    type Output = DeleteBucketMetricsConfigurationResponse;
    type Error = DeleteBucketMetricsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("metrics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketMetricsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketMetricsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketMetricsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketPolicyRequest {
    type Output = DeleteBucketPolicyResponse;
    type Error = DeleteBucketPolicyError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketPolicyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketReplicationRequest {
    type Output = DeleteBucketReplicationResponse;
    type Error = DeleteBucketReplicationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketReplicationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketReplicationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketReplicationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketTaggingRequest {
    type Output = DeleteBucketTaggingResponse;
    type Error = DeleteBucketTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteBucketTaggingError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteBucketWebsiteRequest {
    type Output = DeleteBucketWebsiteResponse;
    type Error = DeleteBucketWebsiteError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteBucketWebsiteError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteBucketWebsiteResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteBucketWebsiteResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteObjectRequest {
    type Output = DeleteObjectResponse;
    type Error = DeleteObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        if let Some(ref bypass_governance_retention) = self.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

        if let Some(ref mfa) = self.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteObjectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteObjectResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
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
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteObjectTaggingRequest {
    type Output = DeleteObjectTaggingResponse;
    type Error = DeleteObjectTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteObjectTaggingError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteObjectTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteObjectsRequest {
    type Output = DeleteObjectsResponse;
    type Error = DeleteObjectsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("POST", "s3", region, &request_uri);

        if let Some(ref bypass_governance_retention) = self.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

        if let Some(ref mfa) = self.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("delete");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        DeleteSerializer::serialize(&mut writer, "Delete", &self.delete);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteObjectsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeleteObjectsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteObjectsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeletePublicAccessBlockRequest {
    type Output = DeletePublicAccessBlockResponse;
    type Error = DeletePublicAccessBlockError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeletePublicAccessBlockError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = DeletePublicAccessBlockResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeletePublicAccessBlockResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketAccelerateConfigurationRequest {
    type Output = GetBucketAccelerateConfigurationResponse;
    type Error = GetBucketAccelerateConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketAccelerateConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAccelerateConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketAccelerateConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketAclRequest {
    type Output = GetBucketAclResponse;
    type Error = GetBucketAclError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketAclError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAclResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketAclResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketAnalyticsConfigurationRequest {
    type Output = GetBucketAnalyticsConfigurationResponse;
    type Error = GetBucketAnalyticsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("analytics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketAnalyticsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketAnalyticsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketAnalyticsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketCorsRequest {
    type Output = GetBucketCorsResponse;
    type Error = GetBucketCorsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketCorsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketCorsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketCorsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketEncryptionRequest {
    type Output = GetBucketEncryptionResponse;
    type Error = GetBucketEncryptionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBucketEncryptionError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketEncryptionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketEncryptionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketInventoryConfigurationRequest {
    type Output = GetBucketInventoryConfigurationResponse;
    type Error = GetBucketInventoryConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("inventory");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketInventoryConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketInventoryConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketInventoryConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketLifecycleRequest {
    type Output = GetBucketLifecycleResponse;
    type Error = GetBucketLifecycleError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketLifecycleError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLifecycleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLifecycleResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketLifecycleConfigurationRequest {
    type Output = GetBucketLifecycleConfigurationResponse;
    type Error = GetBucketLifecycleConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketLifecycleConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLifecycleConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLifecycleConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketLocationRequest {
    type Output = GetBucketLocationResponse;
    type Error = GetBucketLocationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("location");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketLocationError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLocationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLocationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketLoggingRequest {
    type Output = GetBucketLoggingResponse;
    type Error = GetBucketLoggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketLoggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketLoggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLoggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketMetricsConfigurationRequest {
    type Output = GetBucketMetricsConfigurationResponse;
    type Error = GetBucketMetricsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("metrics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketMetricsConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketMetricsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketMetricsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketNotificationRequest {
    type Output = GetBucketNotificationResponse;
    type Error = GetBucketNotificationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketNotificationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketNotificationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketNotificationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketNotificationConfigurationRequest {
    type Output = GetBucketNotificationConfigurationResponse;
    type Error = GetBucketNotificationConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketNotificationConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketNotificationConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketNotificationConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketPolicyRequest {
    type Output = GetBucketPolicyResponse;
    type Error = GetBucketPolicyError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().map(move |response| {
                let mut result = GetBucketPolicyResponse::default();
                result.policy = Some(String::from_utf8_lossy(response.body.as_ref()).into());

                result
            }))
        })
    }
}

impl ServiceRequest for GetBucketPolicyStatusRequest {
    type Output = GetBucketPolicyStatusResponse;
    type Error = GetBucketPolicyStatusError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("policyStatus");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketPolicyStatusError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketPolicyStatusResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketPolicyStatusResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketReplicationRequest {
    type Output = GetBucketReplicationResponse;
    type Error = GetBucketReplicationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBucketReplicationError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketReplicationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketReplicationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketRequestPaymentRequest {
    type Output = GetBucketRequestPaymentResponse;
    type Error = GetBucketRequestPaymentError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketRequestPaymentError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketRequestPaymentResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketRequestPaymentResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketTaggingRequest {
    type Output = GetBucketTaggingResponse;
    type Error = GetBucketTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketTaggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketVersioningRequest {
    type Output = GetBucketVersioningResponse;
    type Error = GetBucketVersioningError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBucketVersioningError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketVersioningResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketVersioningResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetBucketWebsiteRequest {
    type Output = GetBucketWebsiteResponse;
    type Error = GetBucketWebsiteError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketWebsiteError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketWebsiteResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketWebsiteResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectRequest {
    type Output = GetObjectResponse;
    type Error = GetObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref if_match) = self.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = self.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = self.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = self.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = self.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        if let Some(ref x) = self.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = self.response_cache_control {
            params.put("response-cache-control", x);
        }
        if let Some(ref x) = self.response_content_disposition {
            params.put("response-content-disposition", x);
        }
        if let Some(ref x) = self.response_content_encoding {
            params.put("response-content-encoding", x);
        }
        if let Some(ref x) = self.response_content_language {
            params.put("response-content-language", x);
        }
        if let Some(ref x) = self.response_content_type {
            params.put("response-content-type", x);
        }
        if let Some(ref x) = self.response_expires {
            params.put("response-expires", x);
        }
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectError::from_response(response))),
                );
            }

            let mut result = GetObjectResponse::default();
            result.body = Some(response.body);
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
                if key.as_str().starts_with("x-amz-meta-") {
                    values.insert(
                        key.as_str()["x-amz-meta-".len()..].to_owned(),
                        value.to_owned(),
                    );
                }
            }
            result.metadata = Some(values);
            if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                let value = missing_meta.to_owned();
                result.missing_meta = Some(value.parse::<i64>().unwrap())
            };
            if let Some(object_lock_legal_hold_status) =
                response.headers.get("x-amz-object-lock-legal-hold")
            {
                let value = object_lock_legal_hold_status.to_owned();
                result.object_lock_legal_hold_status = Some(value)
            };
            if let Some(object_lock_mode) = response.headers.get("x-amz-object-lock-mode") {
                let value = object_lock_mode.to_owned();
                result.object_lock_mode = Some(value)
            };
            if let Some(object_lock_retain_until_date) =
                response.headers.get("x-amz-object-lock-retain-until-date")
            {
                let value = object_lock_retain_until_date.to_owned();
                result.object_lock_retain_until_date = Some(value)
            };
            if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                let value = parts_count.to_owned();
                result.parts_count = Some(value.parse::<i64>().unwrap())
            };
            if let Some(replication_status) = response.headers.get("x-amz-replication-status") {
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
            if let Some(sse_customer_algorithm) = response
                .headers
                .get("x-amz-server-side-encryption-customer-algorithm")
            {
                let value = sse_customer_algorithm.to_owned();
                result.sse_customer_algorithm = Some(value)
            };
            if let Some(sse_customer_key_md5) = response
                .headers
                .get("x-amz-server-side-encryption-customer-key-MD5")
            {
                let value = sse_customer_key_md5.to_owned();
                result.sse_customer_key_md5 = Some(value)
            };
            if let Some(ssekms_key_id) = response
                .headers
                .get("x-amz-server-side-encryption-aws-kms-key-id")
            {
                let value = ssekms_key_id.to_owned();
                result.ssekms_key_id = Some(value)
            };
            if let Some(server_side_encryption) =
                response.headers.get("x-amz-server-side-encryption")
            {
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
                response.headers.get("x-amz-website-redirect-location")
            {
                let value = website_redirect_location.to_owned();
                result.website_redirect_location = Some(value)
            };
            Box::new(future::ok(result))
        })
    }
}

impl ServiceRequest for GetObjectAclRequest {
    type Output = GetObjectAclResponse;
    type Error = GetObjectAclError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectAclError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectAclResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectAclResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectLegalHoldRequest {
    type Output = GetObjectLegalHoldResponse;
    type Error = GetObjectLegalHoldError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("legal-hold");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectLegalHoldError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectLegalHoldResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectLegalHoldResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectLockConfigurationRequest {
    type Output = GetObjectLockConfigurationResponse;
    type Error = GetObjectLockConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("object-lock");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectLockConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectLockConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectLockConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectRetentionRequest {
    type Output = GetObjectRetentionResponse;
    type Error = GetObjectRetentionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("retention");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectRetentionError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectRetentionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectRetentionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectTaggingRequest {
    type Output = GetObjectTaggingResponse;
    type Error = GetObjectTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectTaggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetObjectTorrentRequest {
    type Output = GetObjectTorrentResponse;
    type Error = GetObjectTorrentError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("torrent");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectTorrentError::from_response(response))),
                );
            }

            let mut result = GetObjectTorrentResponse::default();
            result.body = Some(response.body);
            if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                let value = request_charged.to_owned();
                result.request_charged = Some(value)
            };
            Box::new(future::ok(result))
        })
    }
}

impl ServiceRequest for GetPublicAccessBlockRequest {
    type Output = GetPublicAccessBlockResponse;
    type Error = GetPublicAccessBlockError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetPublicAccessBlockError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetPublicAccessBlockResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetPublicAccessBlockResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for HeadBucketRequest {
    type Output = HeadBucketResponse;
    type Error = HeadBucketError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("HEAD", "s3", region, &request_uri);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(HeadBucketError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = HeadBucketResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        HeadBucketResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for HeadObjectRequest {
    type Output = HeadObjectResponse;
    type Error = HeadObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("HEAD", "s3", region, &request_uri);

        if let Some(ref if_match) = self.if_match {
            request.add_header("If-Match", &if_match.to_string());
        }

        if let Some(ref if_modified_since) = self.if_modified_since {
            request.add_header("If-Modified-Since", &if_modified_since.to_string());
        }

        if let Some(ref if_none_match) = self.if_none_match {
            request.add_header("If-None-Match", &if_none_match.to_string());
        }

        if let Some(ref if_unmodified_since) = self.if_unmodified_since {
            request.add_header("If-Unmodified-Since", &if_unmodified_since.to_string());
        }

        if let Some(ref range) = self.range {
            request.add_header("Range", &range.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        if let Some(ref x) = self.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(HeadObjectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = HeadObjectResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        HeadObjectResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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
                    if key.as_str().starts_with("x-amz-meta-") {
                        values.insert(
                            key.as_str()["x-amz-meta-".len()..].to_owned(),
                            value.to_owned(),
                        );
                    }
                }
                result.metadata = Some(values);
                if let Some(missing_meta) = response.headers.get("x-amz-missing-meta") {
                    let value = missing_meta.to_owned();
                    result.missing_meta = Some(value.parse::<i64>().unwrap())
                };
                if let Some(object_lock_legal_hold_status) =
                    response.headers.get("x-amz-object-lock-legal-hold")
                {
                    let value = object_lock_legal_hold_status.to_owned();
                    result.object_lock_legal_hold_status = Some(value)
                };
                if let Some(object_lock_mode) = response.headers.get("x-amz-object-lock-mode") {
                    let value = object_lock_mode.to_owned();
                    result.object_lock_mode = Some(value)
                };
                if let Some(object_lock_retain_until_date) =
                    response.headers.get("x-amz-object-lock-retain-until-date")
                {
                    let value = object_lock_retain_until_date.to_owned();
                    result.object_lock_retain_until_date = Some(value)
                };
                if let Some(parts_count) = response.headers.get("x-amz-mp-parts-count") {
                    let value = parts_count.to_owned();
                    result.parts_count = Some(value.parse::<i64>().unwrap())
                };
                if let Some(replication_status) = response.headers.get("x-amz-replication-status") {
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
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
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
                    response.headers.get("x-amz-website-redirect-location")
                {
                    let value = website_redirect_location.to_owned();
                    result.website_redirect_location = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListBucketAnalyticsConfigurationsRequest {
    type Output = ListBucketAnalyticsConfigurationsResponse;
    type Error = ListBucketAnalyticsConfigurationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("analytics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketAnalyticsConfigurationsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketAnalyticsConfigurationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketAnalyticsConfigurationsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListBucketInventoryConfigurationsRequest {
    type Output = ListBucketInventoryConfigurationsResponse;
    type Error = ListBucketInventoryConfigurationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("inventory");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketInventoryConfigurationsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketInventoryConfigurationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketInventoryConfigurationsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListBucketMetricsConfigurationsRequest {
    type Output = ListBucketMetricsConfigurationsResponse;
    type Error = ListBucketMetricsConfigurationsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("metrics");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListBucketMetricsConfigurationsError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketMetricsConfigurationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketMetricsConfigurationsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListBucketsRequest {
    type Output = ListBucketsResponse;
    type Error = ListBucketsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListBucketsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListBucketsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListBucketsResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListMultipartUploadsRequest {
    type Output = ListMultipartUploadsResponse;
    type Error = ListMultipartUploadsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = self.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = self.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = self.max_uploads {
            params.put("max-uploads", x);
        }
        if let Some(ref x) = self.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = self.upload_id_marker {
            params.put("upload-id-marker", x);
        }
        params.put_key("uploads");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListMultipartUploadsError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListMultipartUploadsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListMultipartUploadsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListObjectVersionsRequest {
    type Output = ListObjectVersionsResponse;
    type Error = ListObjectVersionsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = self.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = self.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = self.key_marker {
            params.put("key-marker", x);
        }
        if let Some(ref x) = self.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = self.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = self.version_id_marker {
            params.put("version-id-marker", x);
        }
        params.put_key("versions");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectVersionsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectVersionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListObjectVersionsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListObjectsRequest {
    type Output = ListObjectsResponse;
    type Error = ListObjectsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = self.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = self.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = self.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = self.prefix {
            params.put("prefix", x);
        }
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListObjectsResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListObjectsV2Request {
    type Output = ListObjectsV2Response;
    type Error = ListObjectsV2Error;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.continuation_token {
            params.put("continuation-token", x);
        }
        if let Some(ref x) = self.delimiter {
            params.put("delimiter", x);
        }
        if let Some(ref x) = self.encoding_type {
            params.put("encoding-type", x);
        }
        if let Some(ref x) = self.fetch_owner {
            params.put("fetch-owner", x);
        }
        if let Some(ref x) = self.max_keys {
            params.put("max-keys", x);
        }
        if let Some(ref x) = self.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = self.start_after {
            params.put("start-after", x);
        }
        params.put("list-type", "2");
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListObjectsV2Error::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListObjectsV2Response::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListObjectsV2ResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListPartsRequest {
    type Output = ListPartsResponse;
    type Error = ListPartsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("GET", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.max_parts {
            params.put("max-parts", x);
        }
        if let Some(ref x) = self.part_number_marker {
            params.put("part-number-marker", x);
        }
        params.put("uploadId", &self.upload_id);
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPartsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = ListPartsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListPartsResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketAccelerateConfigurationRequest {
    type Output = PutBucketAccelerateConfigurationResponse;
    type Error = PutBucketAccelerateConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AccelerateConfigurationSerializer::serialize(
            &mut writer,
            "AccelerateConfiguration",
            &self.accelerate_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAccelerateConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketAccelerateConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketAccelerateConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketAclRequest {
    type Output = PutBucketAclResponse;
    type Error = PutBucketAclError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = self.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }
        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);
        if self.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                self.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketAclError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketAclResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketAclResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketAnalyticsConfigurationRequest {
    type Output = PutBucketAnalyticsConfigurationResponse;
    type Error = PutBucketAnalyticsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("analytics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AnalyticsConfigurationSerializer::serialize(
            &mut writer,
            "AnalyticsConfiguration",
            &self.analytics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAnalyticsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketAnalyticsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketAnalyticsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketCorsRequest {
    type Output = PutBucketCorsResponse;
    type Error = PutBucketCorsError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        CORSConfigurationSerializer::serialize(
            &mut writer,
            "CORSConfiguration",
            &self.cors_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketCorsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketCorsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketCorsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketEncryptionRequest {
    type Output = PutBucketEncryptionResponse;
    type Error = PutBucketEncryptionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ServerSideEncryptionConfigurationSerializer::serialize(
            &mut writer,
            "ServerSideEncryptionConfiguration",
            &self.server_side_encryption_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketEncryptionError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketEncryptionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketEncryptionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketInventoryConfigurationRequest {
    type Output = PutBucketInventoryConfigurationResponse;
    type Error = PutBucketInventoryConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("inventory");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        InventoryConfigurationSerializer::serialize(
            &mut writer,
            "InventoryConfiguration",
            &self.inventory_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketInventoryConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketInventoryConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketInventoryConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketLifecycleRequest {
    type Output = PutBucketLifecycleResponse;
    type Error = PutBucketLifecycleError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if self.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            LifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                self.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketLifecycleError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketLifecycleResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketLifecycleResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketLifecycleConfigurationRequest {
    type Output = PutBucketLifecycleConfigurationResponse;
    type Error = PutBucketLifecycleConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if self.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            BucketLifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                self.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketLifecycleConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketLifecycleConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketLifecycleConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketLoggingRequest {
    type Output = PutBucketLoggingResponse;
    type Error = PutBucketLoggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        BucketLoggingStatusSerializer::serialize(
            &mut writer,
            "BucketLoggingStatus",
            &self.bucket_logging_status,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketLoggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketLoggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketLoggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketMetricsConfigurationRequest {
    type Output = PutBucketMetricsConfigurationResponse;
    type Error = PutBucketMetricsConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put("id", &self.id);
        params.put_key("metrics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        MetricsConfigurationSerializer::serialize(
            &mut writer,
            "MetricsConfiguration",
            &self.metrics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketMetricsConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketMetricsConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketMetricsConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketNotificationRequest {
    type Output = PutBucketNotificationResponse;
    type Error = PutBucketNotificationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationDeprecatedSerializer::serialize(
            &mut writer,
            "NotificationConfigurationDeprecated",
            &self.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketNotificationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketNotificationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketNotificationConfigurationRequest {
    type Output = PutBucketNotificationConfigurationResponse;
    type Error = PutBucketNotificationConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationSerializer::serialize(
            &mut writer,
            "NotificationConfiguration",
            &self.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketNotificationConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketNotificationConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketPolicyRequest {
    type Output = PutBucketPolicyResponse;
    type Error = PutBucketPolicyError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref confirm_remove_self_bucket_access) = self.confirm_remove_self_bucket_access
        {
            request.add_header(
                "x-amz-confirm-remove-self-bucket-access",
                &confirm_remove_self_bucket_access.to_string(),
            );
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        PolicySerializer::serialize(&mut writer, "Policy", &self.policy);
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketPolicyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketReplicationRequest {
    type Output = PutBucketReplicationResponse;
    type Error = PutBucketReplicationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref token) = self.token {
            request.add_header("x-amz-bucket-object-lock-token", &token.to_string());
        }
        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ReplicationConfigurationSerializer::serialize(
            &mut writer,
            "ReplicationConfiguration",
            &self.replication_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketReplicationError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketReplicationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketReplicationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketRequestPaymentRequest {
    type Output = PutBucketRequestPaymentResponse;
    type Error = PutBucketRequestPaymentError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        RequestPaymentConfigurationSerializer::serialize(
            &mut writer,
            "RequestPaymentConfiguration",
            &self.request_payment_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketRequestPaymentError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketRequestPaymentResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketRequestPaymentResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketTaggingRequest {
    type Output = PutBucketTaggingResponse;
    type Error = PutBucketTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &self.tagging);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketTaggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketVersioningRequest {
    type Output = PutBucketVersioningResponse;
    type Error = PutBucketVersioningError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref mfa) = self.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }
        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        VersioningConfigurationSerializer::serialize(
            &mut writer,
            "VersioningConfiguration",
            &self.versioning_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketVersioningError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketVersioningResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketVersioningResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutBucketWebsiteRequest {
    type Output = PutBucketWebsiteResponse;
    type Error = PutBucketWebsiteError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        WebsiteConfigurationSerializer::serialize(
            &mut writer,
            "WebsiteConfiguration",
            &self.website_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketWebsiteError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutBucketWebsiteResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutBucketWebsiteResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectRequest {
    type Output = PutObjectResponse;
    type Error = PutObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref cache_control) = self.cache_control {
            request.add_header("Cache-Control", &cache_control.to_string());
        }

        if let Some(ref content_disposition) = self.content_disposition {
            request.add_header("Content-Disposition", &content_disposition.to_string());
        }

        if let Some(ref content_encoding) = self.content_encoding {
            request.add_header("Content-Encoding", &content_encoding.to_string());
        }

        if let Some(ref content_language) = self.content_language {
            request.add_header("Content-Language", &content_language.to_string());
        }

        if let Some(ref content_length) = self.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref content_type) = self.content_type {
            request.add_header("Content-Type", &content_type.to_string());
        }

        if let Some(ref expires) = self.expires {
            request.add_header("Expires", &expires.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref metadata) = self.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref object_lock_legal_hold_status) = self.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = self.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = self.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = self.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = self.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = self.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = self.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = self.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        if let Some(__body) = self.body {
            request.set_payload_stream(__body);
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        PutObjectResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                };
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectAclRequest {
    type Output = PutObjectAclResponse;
    type Error = PutObjectAclError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref acl) = self.acl {
            request.add_header("x-amz-acl", &acl.to_string());
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref grant_full_control) = self.grant_full_control {
            request.add_header("x-amz-grant-full-control", &grant_full_control.to_string());
        }

        if let Some(ref grant_read) = self.grant_read {
            request.add_header("x-amz-grant-read", &grant_read.to_string());
        }

        if let Some(ref grant_read_acp) = self.grant_read_acp {
            request.add_header("x-amz-grant-read-acp", &grant_read_acp.to_string());
        }

        if let Some(ref grant_write) = self.grant_write {
            request.add_header("x-amz-grant-write", &grant_write.to_string());
        }

        if let Some(ref grant_write_acp) = self.grant_write_acp {
            request.add_header("x-amz-grant-write-acp", &grant_write_acp.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("acl");
        request.set_params(params);
        if self.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                self.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectAclError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectAclResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectAclResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectLegalHoldRequest {
    type Output = PutObjectLegalHoldResponse;
    type Error = PutObjectLegalHoldError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("legal-hold");
        request.set_params(params);
        if self.legal_hold.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockLegalHoldSerializer::serialize(
                &mut writer,
                "LegalHold",
                self.legal_hold.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectLegalHoldError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectLegalHoldResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectLegalHoldResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectLockConfigurationRequest {
    type Output = PutObjectLockConfigurationResponse;
    type Error = PutObjectLockConfigurationError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref token) = self.token {
            request.add_header("x-amz-bucket-object-lock-token", &token.to_string());
        }
        let mut params = Params::new();
        params.put_key("object-lock");
        request.set_params(params);
        if self.object_lock_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockConfigurationSerializer::serialize(
                &mut writer,
                "ObjectLockConfiguration",
                self.object_lock_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutObjectLockConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectLockConfigurationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectLockConfigurationResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectRetentionRequest {
    type Output = PutObjectRetentionResponse;
    type Error = PutObjectRetentionError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref bypass_governance_retention) = self.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("retention");
        request.set_params(params);
        if self.retention.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockRetentionSerializer::serialize(
                &mut writer,
                "Retention",
                self.retention.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectRetentionError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectRetentionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectRetentionResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutObjectTaggingRequest {
    type Output = PutObjectTaggingResponse;
    type Error = PutObjectTaggingError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &self.tagging);
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutObjectTaggingError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectTaggingResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectTaggingResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(version_id) = response.headers.get("x-amz-version-id") {
                    let value = version_id.to_owned();
                    result.version_id = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutPublicAccessBlockRequest {
    type Output = PutPublicAccessBlockResponse;
    type Error = PutPublicAccessBlockError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}", bucket = self.bucket);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        PublicAccessBlockConfigurationSerializer::serialize(
            &mut writer,
            "PublicAccessBlockConfiguration",
            &self.public_access_block_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutPublicAccessBlockError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutPublicAccessBlockResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutPublicAccessBlockResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for RestoreObjectRequest {
    type Output = RestoreObjectResponse;
    type Error = RestoreObjectError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("POST", "s3", region, &request_uri);

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = self.version_id {
            params.put("versionId", x);
        }
        params.put_key("restore");
        request.set_params(params);
        if self.restore_request.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            RestoreRequestSerializer::serialize(
                &mut writer,
                "RestoreRequest",
                self.restore_request.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RestoreObjectError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = RestoreObjectResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = RestoreObjectResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(restore_output_path) = response.headers.get("x-amz-restore-output-path")
                {
                    let value = restore_output_path.to_owned();
                    result.restore_output_path = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for SelectObjectContentRequest {
    type Output = SelectObjectContentResponse;
    type Error = SelectObjectContentError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("POST", "s3", region, &request_uri);

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("select&select-type", "2");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        SelectObjectContentRequestSerializer::serialize(
            &mut writer,
            "SelectObjectContentRequest",
            &self,
            "http://s3.amazonaws.com/doc/2006-03-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SelectObjectContentError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = SelectObjectContentResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = SelectObjectContentResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UploadPartRequest {
    type Output = UploadPartResponse;
    type Error = UploadPartError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        if let Some(ref content_length) = self.content_length {
            request.add_header("Content-Length", &content_length.to_string());
        }

        if let Some(ref content_md5) = self.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &self.part_number);
        params.put("uploadId", &self.upload_id);
        request.set_params(params);
        if let Some(__body) = self.body {
            request.set_payload_stream(__body);
        }

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UploadPartError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UploadPartResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        UploadPartResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(e_tag) = response.headers.get("ETag") {
                    let value = e_tag.to_owned();
                    result.e_tag = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UploadPartCopyRequest {
    type Output = UploadPartCopyResponse;
    type Error = UploadPartCopyError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = format!("/{bucket}/{key}", bucket = self.bucket, key = self.key);

        let mut request = SignedRequest::new("PUT", "s3", region, &request_uri);

        request.add_header("x-amz-copy-source", &self.copy_source);

        if let Some(ref copy_source_if_match) = self.copy_source_if_match {
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = self.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = self.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = self.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_range) = self.copy_source_range {
            request.add_header("x-amz-copy-source-range", &copy_source_range.to_string());
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            self.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = self.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = self.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref request_payer) = self.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = self.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = self.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = self.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &self.part_number);
        params.put("uploadId", &self.upload_id);
        request.set_params(params);

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UploadPartCopyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = UploadPartCopyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = UploadPartCopyResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                if let Some(copy_source_version_id) =
                    response.headers.get("x-amz-copy-source-version-id")
                {
                    let value = copy_source_version_id.to_owned();
                    result.copy_source_version_id = Some(value)
                };
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                };
                if let Some(sse_customer_algorithm) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-algorithm")
                {
                    let value = sse_customer_algorithm.to_owned();
                    result.sse_customer_algorithm = Some(value)
                };
                if let Some(sse_customer_key_md5) = response
                    .headers
                    .get("x-amz-server-side-encryption-customer-key-MD5")
                {
                    let value = sse_customer_key_md5.to_owned();
                    result.sse_customer_key_md5 = Some(value)
                };
                if let Some(ssekms_key_id) = response
                    .headers
                    .get("x-amz-server-side-encryption-aws-kms-key-id")
                {
                    let value = ssekms_key_id.to_owned();
                    result.ssekms_key_id = Some(value)
                };
                if let Some(server_side_encryption) =
                    response.headers.get("x-amz-server-side-encryption")
                {
                    let value = server_side_encryption.to_owned();
                    result.server_side_encryption = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_s3_create_bucket() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "s3-create-bucket.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateBucketRequest::default();
        let result = client.create_bucket(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_error_s3_list_objects() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "s3-list-objects.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_acl() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-acl.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketAclRequest::default();
        let result = client.get_bucket_acl(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_location() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-location.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLocationRequest::default();
        let result = client.get_bucket_location(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_logging() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-logging.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketLoggingRequest::default();
        let result = client.get_bucket_logging(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_get_bucket_policy() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-get-bucket-policy.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetBucketPolicyRequest::default();
        let result = client.get_bucket_policy(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_buckets() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-buckets.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.list_buckets().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_multipart_uploads() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-multipart-uploads.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMultipartUploadsRequest::default();
        let result = client.list_multipart_uploads(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_object_versions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-object-versions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectVersionsRequest::default();
        let result = client.list_object_versions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_s3_list_objects() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "s3-list-objects.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = S3Client::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListObjectsRequest::default();
        let result = client.list_objects(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
