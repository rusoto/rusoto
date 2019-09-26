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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

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

/// <p>Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortIncompleteMultipartUpload {
    /// <p>Specifies the number of days after which Amazon S3 aborts an incomplete multipart upload.</p>
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
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<String>,
}

struct AbortMultipartUploadOutputDeserializer;
impl AbortMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AbortMultipartUploadOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = AbortMultipartUploadOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortMultipartUploadRequest {
    /// <p>Name of the bucket to which the multipart upload was initiated.</p>
    pub bucket: String,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: String,
}

/// <p>Configures the transfer acceleration state for an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/transfer-acceleration.html">Amazon S3 Transfer Acceleration</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccelerateConfiguration {
    /// <p>Specifies the transfer acceleration status of the bucket.</p>
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

/// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p>Container for the bucket owner's display name and ID.</p>
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
    /// <p>Specifies the replica ownership. For default and valid values, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT bucket replication</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
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

/// <p>A conjunction (logical AND) of predicates, which is used in evaluating a metrics filter. The operator must have at least two predicates in any combination, and an object must match all of the predicates for the filter to apply.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsAndOperator {
    /// <p>The prefix to use when evaluating an AND predicate: The prefix that an object must have to be included in the metrics results.</p>
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

/// <p> Specifies the configuration and any analyses for the analytics filter of an Amazon S3 bucket.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketGETAnalyticsConfig.html">GET Bucket analytics</a> in the <i>Amazon Simple Storage Service API Reference</i>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyticsConfiguration {
    /// <p>The filter used to describe a set of objects for analyses. A filter must have exactly one prefix, one tag, or one conjunction (AnalyticsAndOperator). If no filter is provided, all objects will be considered in any analysis.</p>
    pub filter: Option<AnalyticsFilter>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: String,
    /// <p> Contains data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes. </p>
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
/// <p>Where to publish the analytics results.</p>
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
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    pub bucket: String,
    /// <p>The account ID that owns the destination bucket. If no account ID is provided, the owner will not be validated prior to exporting data.</p>
    pub bucket_account_id: Option<String>,
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    pub format: String,
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
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

/// <p>Specifies the lifecycle configuration for objects in an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html">Object Lifecycle Management</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BucketLifecycleConfiguration {
    /// <p>A lifecycle rule for individual objects in an Amazon S3 bucket.</p>
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
    /// <p><p/></p>
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
/// <p>Describes the cross-origin access configuration for objects in an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/cors.html">Enabling Cross-Origin Resource Sharing</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSConfiguration {
    /// <p>A set of allowed origins and methods.</p>
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

/// <p>Specifies a cross-origin access rule for an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CORSRule {
    /// <p>Headers that are specified in the <code>Access-Control-Request-Headers</code> header. These headers are allowed in a preflight OPTIONS request. In response to any preflight OPTIONS request, Amazon S3 returns any requested headers that are allowed.</p>
    pub allowed_headers: Option<Vec<String>>,
    /// <p>An HTTP method that you allow the origin to execute. Valid values are <code>GET</code>, <code>PUT</code>, <code>HEAD</code>, <code>POST</code>, and <code>DELETE</code>.</p>
    pub allowed_methods: Vec<String>,
    /// <p>One or more origins you want customers to be able to access the bucket from.</p>
    pub allowed_origins: Vec<String>,
    /// <p>One or more headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript <code>XMLHttpRequest</code> object).</p>
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
pub struct CompleteMultipartUploadOutput {
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

struct CompleteMultipartUploadOutputDeserializer;
impl CompleteMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        deserialize_elements::<_, CompleteMultipartUploadOutput, _>(
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

/// <p>Specifies a condition that must be met for a redirect to apply. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub http_error_code_returned_equals: Option<String>,
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for <code>ExamplePage.html</code>, the key prefix will be <code>ExamplePage.html</code>. To redirect request for all pages with the prefix <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all objects in the docs/ folder. Required when the parent element <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code> is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p>
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
pub struct CopyObjectOutput {
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
    /// <p>If present, specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version ID of the newly created copy.</p>
    pub version_id: Option<String>,
}

struct CopyObjectOutputDeserializer;
impl CopyObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopyObjectOutput, XmlParseError> {
        Ok(CopyObjectOutput {
            copy_object_result: Some(CopyObjectResultDeserializer::deserialize(
                "CopyObjectResult",
                stack,
            )?),
            ..CopyObjectOutput::default()
        })
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
    /// <p>The object lock mode that you want to apply to the copied object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when you want the copied object's object lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
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
pub struct CreateBucketOutput {
    /// <p><p/></p>
    pub location: Option<String>,
}

struct CreateBucketOutputDeserializer;
impl CreateBucketOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateBucketOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = CreateBucketOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
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
    /// <p>Specifies whether you want Amazon S3 object lock to be enabled for the new bucket.</p>
    pub object_lock_enabled_for_bucket: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateMultipartUploadOutput {
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
    /// <p>If present, specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>ID for the initiated multipart upload.</p>
    pub upload_id: Option<String>,
}

struct CreateMultipartUploadOutputDeserializer;
impl CreateMultipartUploadOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateMultipartUploadOutput, XmlParseError> {
        deserialize_elements::<_, CreateMultipartUploadOutput, _>(
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
    /// <p>Specifies the object lock mode that you want to apply to the uploaded object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>Specifies the date and time when you want the object lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
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

/// <p>The container element for specifying the default object lock retention settings for new objects placed in the specified bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultRetention {
    /// <p>The number of days that you want to specify for the default retention period.</p>
    pub days: Option<i64>,
    /// <p>The default object lock retention mode you want to apply to new objects placed in the specified bucket.</p>
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
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketCorsRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketEncryptionRequest {
    /// <p>The name of the bucket containing the server-side encryption configuration to delete.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketInventoryConfigurationRequest {
    /// <p>The name of the bucket containing the inventory configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketLifecycleRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketMetricsConfigurationRequest {
    /// <p>The name of the bucket containing the metrics configuration to delete.</p>
    pub bucket: String,
    /// <p>The ID used to identify the metrics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketPolicyRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketReplicationRequest {
    /// <p><p> The bucket name. </p> <note> <p>It can take a while to propagate the deletion of a replication configuration to all Amazon S3 systems.</p> </note></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
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
pub struct DeleteObjectOutput {
    /// <p>Specifies whether the versioned object that was permanently deleted was (true) or was not (false) a delete marker.</p>
    pub delete_marker: Option<bool>,
    pub request_charged: Option<String>,
    /// <p>Returns the version ID of the delete marker created as a result of the DELETE operation.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectOutputDeserializer;
impl DeleteObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteObjectOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Indicates whether Amazon S3 object lock should bypass governance-mode restrictions to process this operation.</p>
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
pub struct DeleteObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was removed from.</p>
    pub version_id: Option<String>,
}

struct DeleteObjectTaggingOutputDeserializer;
impl DeleteObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectTaggingOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteObjectTaggingOutput::default();

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
pub struct DeleteObjectsOutput {
    /// <p><p/></p>
    pub deleted: Option<Vec<DeletedObject>>,
    /// <p><p/></p>
    pub errors: Option<Vec<S3Error>>,
    pub request_charged: Option<String>,
}

struct DeleteObjectsOutputDeserializer;
impl DeleteObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteObjectsOutput, XmlParseError> {
        deserialize_elements::<_, DeleteObjectsOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct DeleteObjectsRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>Specifies whether you want to delete this object even if it has a Governance-type object lock in place. You must have sufficient permissions to perform this operation.</p>
    pub bypass_governance_retention: Option<bool>,
    /// <p><p/></p>
    pub delete: Delete,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: Option<String>,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePublicAccessBlockRequest {
    /// <p>The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete. </p>
    pub bucket: String,
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

/// <p>Specifies information about where to publish analysis or configuration results for an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Destination {
    /// <p>Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the AWS account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same AWS account that owns the source object.</p>
    pub access_control_translation: Option<AccessControlTranslation>,
    /// <p>Destination bucket owner account ID. In a cross-account scenario, if you direct Amazon S3 to change replica ownership to the AWS account that owns the destination bucket by specifying the <code>AccessControlTranslation</code> property, this is the account ID of the destination bucket owner. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr-change-owner.html">Cross-Region Replication Additional Configuration: Change Replica Owner</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub account: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the bucket where you want Amazon S3 to store replicas of the object identified by the rule.</p> <p>A replication configuration can replicate objects to only one destination bucket. If there are multiple rules in your replication configuration, all rules must specify the same destination bucket.</p>
    pub bucket: String,
    /// <p>A container that provides information about encryption. If <code>SourceSelectionCriteria</code> is specified, you must specify this element.</p>
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p> The storage class to use when replicating objects, such as standard or reduced redundancy. By default, Amazon S3 uses the storage class of the source object to create the object replica. </p> <p>For valid values, see the <code>StorageClass</code> element of the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTreplication.html">PUT Bucket replication</a> action in the <i>Amazon Simple Storage Service API Reference</i>.</p>
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

/// <p>Specifies encryption-related information for an Amazon S3 bucket that is a destination for replicated objects.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EncryptionConfiguration {
    /// <p>Specifies the AWS KMS Key ID (Key ARN or Alias ARN) for the destination bucket. Amazon S3 uses this key to encrypt replica objects.</p>
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

/// <p>Specifies the Amazon S3 object key name to filter on and whether to filter on the suffix or prefix of the key name.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FilterRule {
    /// <p>The object key name prefix or suffix identifying one or more objects to which the filtering rule applies. The maximum length is 1,024 characters. Overlapping prefixes and suffixes are not supported. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub name: Option<String>,
    /// <p>The value that the filter searches for in object key names.</p>
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
pub struct GetBucketAccelerateConfigurationOutput {
    /// <p>The accelerate configuration of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketAccelerateConfigurationOutputDeserializer;
impl GetBucketAccelerateConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAccelerateConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketAccelerateConfigurationOutput, _>(
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
pub struct GetBucketAccelerateConfigurationRequest {
    /// <p>Name of the bucket for which the accelerate configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
}

struct GetBucketAclOutputDeserializer;
impl GetBucketAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAclOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketAclOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct GetBucketAclRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationOutput {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

struct GetBucketAnalyticsConfigurationOutputDeserializer;
impl GetBucketAnalyticsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketAnalyticsConfigurationOutput, XmlParseError> {
        Ok(GetBucketAnalyticsConfigurationOutput {
            analytics_configuration: Some(AnalyticsConfigurationDeserializer::deserialize(
                "AnalyticsConfiguration",
                stack,
            )?),
            ..GetBucketAnalyticsConfigurationOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketAnalyticsConfigurationRequest {
    /// <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    pub bucket: String,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketCorsOutput {
    /// <p><p/></p>
    pub cors_rules: Option<Vec<CORSRule>>,
}

struct GetBucketCorsOutputDeserializer;
impl GetBucketCorsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketCorsOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketCorsOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct GetBucketCorsRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionOutput {
    /// <p><p/></p>
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

struct GetBucketEncryptionOutputDeserializer;
impl GetBucketEncryptionOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketEncryptionOutput, XmlParseError> {
        Ok(GetBucketEncryptionOutput {
            server_side_encryption_configuration: Some(
                ServerSideEncryptionConfigurationDeserializer::deserialize(
                    "ServerSideEncryptionConfiguration",
                    stack,
                )?,
            ),
            ..GetBucketEncryptionOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketEncryptionRequest {
    /// <p>The name of the bucket from which the server-side encryption configuration is retrieved.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketInventoryConfigurationOutput {
    /// <p>Specifies the inventory configuration.</p>
    pub inventory_configuration: Option<InventoryConfiguration>,
}

struct GetBucketInventoryConfigurationOutputDeserializer;
impl GetBucketInventoryConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketInventoryConfigurationOutput, XmlParseError> {
        Ok(GetBucketInventoryConfigurationOutput {
            inventory_configuration: Some(InventoryConfigurationDeserializer::deserialize(
                "InventoryConfiguration",
                stack,
            )?),
            ..GetBucketInventoryConfigurationOutput::default()
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
pub struct GetBucketLifecycleConfigurationOutput {
    /// <p><p/></p>
    pub rules: Option<Vec<LifecycleRule>>,
}

struct GetBucketLifecycleConfigurationOutputDeserializer;
impl GetBucketLifecycleConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleConfigurationOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleConfigurationOutput, _>(
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
pub struct GetBucketLifecycleConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLifecycleOutput {
    /// <p><p/></p>
    pub rules: Option<Vec<Rule>>,
}

struct GetBucketLifecycleOutputDeserializer;
impl GetBucketLifecycleOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLifecycleOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLifecycleOutput, _>(
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
pub struct GetBucketLifecycleRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationOutput {
    /// <p><p/></p>
    pub location_constraint: Option<String>,
}

struct GetBucketLocationOutputDeserializer;
impl GetBucketLocationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLocationOutput, XmlParseError> {
        let mut obj = GetBucketLocationOutput::default();
        obj.location_constraint = Some(BucketLocationConstraintDeserializer::deserialize(
            "LocationConstraint",
            stack,
        )?);
        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLocationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingOutput {
    /// <p><p/></p>
    pub logging_enabled: Option<LoggingEnabled>,
}

struct GetBucketLoggingOutputDeserializer;
impl GetBucketLoggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketLoggingOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketLoggingOutput, _>(tag_name, stack, |name, stack, obj| {
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
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketLoggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketMetricsConfigurationOutput {
    /// <p>Specifies the metrics configuration.</p>
    pub metrics_configuration: Option<MetricsConfiguration>,
}

struct GetBucketMetricsConfigurationOutputDeserializer;
impl GetBucketMetricsConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketMetricsConfigurationOutput, XmlParseError> {
        Ok(GetBucketMetricsConfigurationOutput {
            metrics_configuration: Some(MetricsConfigurationDeserializer::deserialize(
                "MetricsConfiguration",
                stack,
            )?),
            ..GetBucketMetricsConfigurationOutput::default()
        })
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
pub struct GetBucketNotificationConfigurationRequest {
    /// <p>Name of the bucket to get the notification configuration for.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyOutput {
    /// <p>The bucket policy as a JSON document.</p>
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyStatusOutput {
    /// <p>The policy status for the specified bucket.</p>
    pub policy_status: Option<PolicyStatus>,
}

struct GetBucketPolicyStatusOutputDeserializer;
impl GetBucketPolicyStatusOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketPolicyStatusOutput, XmlParseError> {
        Ok(GetBucketPolicyStatusOutput {
            policy_status: Some(PolicyStatusDeserializer::deserialize(
                "PolicyStatus",
                stack,
            )?),
            ..GetBucketPolicyStatusOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketPolicyStatusRequest {
    /// <p>The name of the Amazon S3 bucket whose policy status you want to retrieve.</p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationOutput {
    /// <p><p/></p>
    pub replication_configuration: Option<ReplicationConfiguration>,
}

struct GetBucketReplicationOutputDeserializer;
impl GetBucketReplicationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketReplicationOutput, XmlParseError> {
        Ok(GetBucketReplicationOutput {
            replication_configuration: Some(ReplicationConfigurationDeserializer::deserialize(
                "ReplicationConfiguration",
                stack,
            )?),
            ..GetBucketReplicationOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketReplicationRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketRequestPaymentOutput {
    /// <p>Specifies who pays for the download and request fees.</p>
    pub payer: Option<String>,
}

struct GetBucketRequestPaymentOutputDeserializer;
impl GetBucketRequestPaymentOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketRequestPaymentOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketRequestPaymentOutput, _>(
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
pub struct GetBucketRequestPaymentRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingOutput {
    /// <p><p/></p>
    pub tag_set: Vec<Tag>,
}

struct GetBucketTaggingOutputDeserializer;
impl GetBucketTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketTaggingOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketTaggingOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TagSet" => {
                    obj.tag_set
                        .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketVersioningOutput {
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    pub mfa_delete: Option<String>,
    /// <p>The versioning state of the bucket.</p>
    pub status: Option<String>,
}

struct GetBucketVersioningOutputDeserializer;
impl GetBucketVersioningOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketVersioningOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketVersioningOutput, _>(
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
pub struct GetBucketVersioningRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteOutput {
    /// <p><p/></p>
    pub error_document: Option<ErrorDocument>,
    /// <p><p/></p>
    pub index_document: Option<IndexDocument>,
    /// <p><p/></p>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p><p/></p>
    pub routing_rules: Option<Vec<RoutingRule>>,
}

struct GetBucketWebsiteOutputDeserializer;
impl GetBucketWebsiteOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetBucketWebsiteOutput, XmlParseError> {
        deserialize_elements::<_, GetBucketWebsiteOutput, _>(tag_name, stack, |name, stack, obj| {
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
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectAclOutput {
    /// <p>A list of grants.</p>
    pub grants: Option<Vec<Grant>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
    pub request_charged: Option<String>,
}

struct GetObjectAclOutputDeserializer;
impl GetObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectAclOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectAclOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct GetObjectLegalHoldOutput {
    /// <p>The current Legal Hold status for the specified object.</p>
    pub legal_hold: Option<ObjectLockLegalHold>,
}

struct GetObjectLegalHoldOutputDeserializer;
impl GetObjectLegalHoldOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLegalHoldOutput, XmlParseError> {
        Ok(GetObjectLegalHoldOutput {
            legal_hold: Some(ObjectLockLegalHoldDeserializer::deserialize(
                "LegalHold",
                stack,
            )?),
            ..GetObjectLegalHoldOutput::default()
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
pub struct GetObjectLockConfigurationOutput {
    /// <p>The specified bucket's object lock configuration.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

struct GetObjectLockConfigurationOutputDeserializer;
impl GetObjectLockConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectLockConfigurationOutput, XmlParseError> {
        Ok(GetObjectLockConfigurationOutput {
            object_lock_configuration: Some(ObjectLockConfigurationDeserializer::deserialize(
                "ObjectLockConfiguration",
                stack,
            )?),
            ..GetObjectLockConfigurationOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectLockConfigurationRequest {
    /// <p>The bucket whose object lock configuration you want to retrieve.</p>
    pub bucket: String,
}

#[derive(Default, Debug)]
pub struct GetObjectOutput {
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
    /// <p>The object lock mode currently in place for this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when this object's object lock will expire.</p>
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectRetentionOutput {
    /// <p>The container element for an object's retention settings.</p>
    pub retention: Option<ObjectLockRetention>,
}

struct GetObjectRetentionOutputDeserializer;
impl GetObjectRetentionOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectRetentionOutput, XmlParseError> {
        Ok(GetObjectRetentionOutput {
            retention: Some(ObjectLockRetentionDeserializer::deserialize(
                "Retention",
                stack,
            )?),
            ..GetObjectRetentionOutput::default()
        })
    }
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
pub struct GetObjectTaggingOutput {
    /// <p><p/></p>
    pub tag_set: Vec<Tag>,
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct GetObjectTaggingOutputDeserializer;
impl GetObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetObjectTaggingOutput, XmlParseError> {
        deserialize_elements::<_, GetObjectTaggingOutput, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "TagSet" => {
                    obj.tag_set
                        .extend(TagSetDeserializer::deserialize("TagSet", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
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

#[derive(Default, Debug)]
pub struct GetObjectTorrentOutput {
    /// <p><p/></p>
    pub body: Option<StreamingBody>,
    pub request_charged: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetObjectTorrentRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPublicAccessBlockOutput {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

struct GetPublicAccessBlockOutputDeserializer;
impl GetPublicAccessBlockOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetPublicAccessBlockOutput, XmlParseError> {
        Ok(GetPublicAccessBlockOutput {
            public_access_block_configuration: Some(
                PublicAccessBlockConfigurationDeserializer::deserialize(
                    "PublicAccessBlockConfiguration",
                    stack,
                )?,
            ),
            ..GetPublicAccessBlockOutput::default()
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetPublicAccessBlockRequest {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    pub bucket: String,
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
pub struct HeadObjectOutput {
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
    /// <p>The object lock mode currently in place for this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when this object's object lock expires.</p>
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

struct HeadObjectOutputDeserializer;
impl HeadObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<HeadObjectOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = HeadObjectOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
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

/// <p>Specifies the inventory configuration for an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketGETInventoryConfig.html">GET Bucket inventory</a> in the <i>Amazon Simple Storage Service API Reference</i>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InventoryConfiguration {
    /// <p>Contains information about where to publish the inventory results.</p>
    pub destination: InventoryDestination,
    /// <p>Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria.</p>
    pub filter: Option<InventoryFilter>,
    /// <p>The ID used to identify the inventory configuration.</p>
    pub id: String,
    /// <p>Object versions to include in the inventory list. If set to <code>All</code>, the list includes all the object versions, which adds the version-related fields <code>VersionId</code>, <code>IsLatest</code>, and <code>DeleteMarker</code> to the list. If set to <code>Current</code>, the list does not contain these version-related fields.</p>
    pub included_object_versions: String,
    /// <p>Specifies whether the inventory is enabled or disabled. If set to <code>True</code>, an inventory list is generated. If set to <code>False</code>, no inventory list is generated.</p>
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
    /// <p>The Amazon S3 bucket event for which to invoke the AWS Lambda function. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub events: Vec<String>,
    /// <p><p/></p>
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Lambda function that Amazon S3 invokes when the specified event type occurs.</p>
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
    /// <p><p/></p>
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// <p><p/></p>
    pub expiration: Option<LifecycleExpiration>,
    /// <p><p/></p>
    pub filter: Option<LifecycleRuleFilter>,
    /// <p>Unique identifier for the rule. The value cannot be longer than 255 characters.</p>
    pub id: Option<String>,
    /// <p><p/></p>
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
    /// <p><p/></p>
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
pub struct ListBucketAnalyticsConfigurationsOutput {
    /// <p>The list of analytics configurations for a bucket.</p>
    pub analytics_configuration_list: Option<Vec<AnalyticsConfiguration>>,
    /// <p>The ContinuationToken that represents where this request began.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of analytics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>NextContinuationToken is sent when isTruncated is true, which indicates that there are more analytics configurations to list. The next request must include this NextContinuationToken. The token is obfuscated and is not a usable value.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketAnalyticsConfigurationsOutputDeserializer;
impl ListBucketAnalyticsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketAnalyticsConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketAnalyticsConfigurationsOutput, _>(
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
pub struct ListBucketAnalyticsConfigurationsRequest {
    /// <p>The name of the bucket from which analytics configurations are retrieved.</p>
    pub bucket: String,
    /// <p>The ContinuationToken that represents a placeholder from where this request should begin.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketInventoryConfigurationsOutput {
    /// <p>If sent in the request, the marker that is used as a starting point for this inventory configuration list response.</p>
    pub continuation_token: Option<String>,
    /// <p>The list of inventory configurations for a bucket.</p>
    pub inventory_configuration_list: Option<Vec<InventoryConfiguration>>,
    /// <p>Indicates whether the returned list of inventory configurations is truncated in this response. A value of true indicates that the list is truncated.</p>
    pub is_truncated: Option<bool>,
    /// <p>The marker used to continue this inventory configuration listing. Use the NextContinuationToken from this response to continue the listing in a subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketInventoryConfigurationsOutputDeserializer;
impl ListBucketInventoryConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketInventoryConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketInventoryConfigurationsOutput, _>(
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
pub struct ListBucketInventoryConfigurationsRequest {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketMetricsConfigurationsOutput {
    /// <p>The marker that is used as a starting point for this metrics configuration list response. This value is present if it was sent in the request.</p>
    pub continuation_token: Option<String>,
    /// <p>Indicates whether the returned list of metrics configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken will be provided for a subsequent request.</p>
    pub is_truncated: Option<bool>,
    /// <p>The list of metrics configurations for a bucket.</p>
    pub metrics_configuration_list: Option<Vec<MetricsConfiguration>>,
    /// <p>The marker used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub next_continuation_token: Option<String>,
}

struct ListBucketMetricsConfigurationsOutputDeserializer;
impl ListBucketMetricsConfigurationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketMetricsConfigurationsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketMetricsConfigurationsOutput, _>(
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
pub struct ListBucketMetricsConfigurationsRequest {
    /// <p>The name of the bucket containing the metrics configurations to retrieve.</p>
    pub bucket: String,
    /// <p>The marker that is used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListBucketsOutput {
    /// <p><p/></p>
    pub buckets: Option<Vec<Bucket>>,
    /// <p><p/></p>
    pub owner: Option<Owner>,
}

struct ListBucketsOutputDeserializer;
impl ListBucketsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListBucketsOutput, XmlParseError> {
        deserialize_elements::<_, ListBucketsOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct ListMultipartUploadsOutput {
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

struct ListMultipartUploadsOutputDeserializer;
impl ListMultipartUploadsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMultipartUploadsOutput, XmlParseError> {
        deserialize_elements::<_, ListMultipartUploadsOutput, _>(
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
pub struct ListObjectVersionsOutput {
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

struct ListObjectVersionsOutputDeserializer;
impl ListObjectVersionsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectVersionsOutput, XmlParseError> {
        deserialize_elements::<_, ListObjectVersionsOutput, _>(
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
pub struct ListObjectsOutput {
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

struct ListObjectsOutputDeserializer;
impl ListObjectsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsOutput, XmlParseError> {
        deserialize_elements::<_, ListObjectsOutput, _>(tag_name, stack, |name, stack, obj| {
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
pub struct ListObjectsV2Output {
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

struct ListObjectsV2OutputDeserializer;
impl ListObjectsV2OutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListObjectsV2Output, XmlParseError> {
        deserialize_elements::<_, ListObjectsV2Output, _>(tag_name, stack, |name, stack, obj| {
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
pub struct ListPartsOutput {
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

struct ListPartsOutputDeserializer;
impl ListPartsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPartsOutput, XmlParseError> {
        deserialize_elements::<_, ListPartsOutput, _>(tag_name, stack, |name, stack, obj| {
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

/// <p>Describes where logs are stored and the prefix that Amazon S3 assigns to all log object keys for a bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlogging.html">PUT Bucket logging</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoggingEnabled {
    /// <p>Specifies the bucket where you want Amazon S3 to store server access logs. You can have your logs delivered to any bucket that you own, including the same bucket that is being logged. You can also configure multiple buckets to deliver their logs to the same target bucket. In this case you should choose a different TargetPrefix for each source bucket so that the delivered log files can be distinguished by key.</p>
    pub target_bucket: String,
    /// <p><p/></p>
    pub target_grants: Option<Vec<TargetGrant>>,
    /// <p>A prefix for all log object keys. If you store log files from multiple Amazon S3 buckets in a single bucket, you can use a prefix to distinguish which log files came from which bucket.</p>
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

/// <p>Specifies a metrics configuration for the CloudWatch request metrics (specified by the metrics configuration ID) from an Amazon S3 bucket. If you're updating an existing metrics configuration, note that this is a full replacement of the existing metrics configuration. If you don't include the elements you want to keep, they are erased. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTMetricConfiguration.html"> PUT Bucket metrics</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
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

/// <p>Container for the transition rule that describes when noncurrent objects transition to the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class at a specific period in the object's lifetime.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NoncurrentVersionTransition {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html">How Amazon S3 Calculates When an Object Became Noncurrent</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
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
    /// <p>Describes the AWS Lambda functions to invoke and the events for which to invoke them.</p>
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    /// <p>The Amazon Simple Queue Service queues to publish messages to and the events for which to publish messages.</p>
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    /// <p>The topic to which notifications are sent and the events for which notifications are generated.</p>
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        deserialize_elements::<_, NotificationConfiguration, _>(
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

struct NotificationConfigurationDeprecatedDeserializer;
impl NotificationConfigurationDeprecatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        deserialize_elements::<_, NotificationConfigurationDeprecated, _>(
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

/// <p>Specifies object key name filtering rules. For information about key name filtering, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Configuring Event Notifications</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotificationConfigurationFilter {
    /// <p><p/></p>
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
/// <p>The container element for object lock configuration parameters.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectLockConfiguration {
    /// <p>Indicates whether this bucket has an object lock configuration enabled.</p>
    pub object_lock_enabled: Option<String>,
    /// <p>The object lock rule in place for the specified object.</p>
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
    /// <p>The date on which this object lock retention expires.</p>
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

/// <p>The container element for an object lock rule.</p>
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

/// <p>Specifies the Block Public Access configuration for an Amazon S3 bucket.</p>
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
pub struct PutBucketAclRequest {
    /// <p>The canned ACL to apply to the bucket.</p>
    pub acl: Option<String>,
    /// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
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
pub struct PutBucketAnalyticsConfigurationRequest {
    /// <p>The configuration and any analyses for the analytics filter.</p>
    pub analytics_configuration: AnalyticsConfiguration,
    /// <p>The name of the bucket to which an analytics configuration is stored.</p>
    pub bucket: String,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: String,
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
pub struct PutBucketEncryptionRequest {
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or AWS KMS-managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub bucket: String,
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration. This parameter is auto-populated when using the command from the CLI.</p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
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
pub struct PutBucketLifecycleConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
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
pub struct PutBucketLoggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub bucket_logging_status: BucketLoggingStatus,
    /// <p><p/></p>
    pub content_md5: Option<String>,
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
pub struct PutBucketNotificationConfigurationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub notification_configuration: NotificationConfiguration,
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
pub struct PutBucketReplicationRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit.</p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub replication_configuration: ReplicationConfiguration,
    /// <p>A token that allows Amazon S3 object lock to be enabled for an existing bucket.</p>
    pub token: Option<String>,
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
pub struct PutBucketTaggingRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub tagging: Tagging,
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
pub struct PutBucketWebsiteRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub content_md5: Option<String>,
    /// <p><p/></p>
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<String>,
}

struct PutObjectAclOutputDeserializer;
impl PutObjectAclOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectAclOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectAclOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectAclRequest {
    /// <p>The canned ACL to apply to the object.</p>
    pub acl: Option<String>,
    /// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
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
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<String>,
}

struct PutObjectLegalHoldOutputDeserializer;
impl PutObjectLegalHoldOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLegalHoldOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectLegalHoldOutput::default();

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
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<String>,
}

struct PutObjectLockConfigurationOutputDeserializer;
impl PutObjectLockConfigurationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectLockConfigurationOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectLockConfigurationOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectLockConfigurationRequest {
    /// <p>The bucket whose object lock configuration you want to create or replace.</p>
    pub bucket: String,
    /// <p>The MD5 hash for the request body.</p>
    pub content_md5: Option<String>,
    /// <p>The object lock configuration that you want to apply to the specified bucket.</p>
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub request_payer: Option<String>,
    /// <p>A token to allow Amazon S3 object lock to be enabled for an existing bucket.</p>
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutObjectOutput {
    /// <p>Entity tag for the uploaded object.</p>
    pub e_tag: Option<String>,
    /// <p>If the object expiration is configured, this will contain the expiration date (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.</p>
    pub expiration: Option<String>,
    pub request_charged: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round trip message integrity verification of the customer-provided encryption key.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>If present, specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
    /// <p>If present, specifies the ID of the AWS Key Management Service (KMS) master encryption key that was used for the object.</p>
    pub ssekms_key_id: Option<String>,
    /// <p>The Server-side encryption algorithm used when storing this object in S3 (e.g., AES256, aws:kms).</p>
    pub server_side_encryption: Option<String>,
    /// <p>Version of the object.</p>
    pub version_id: Option<String>,
}

struct PutObjectOutputDeserializer;
impl PutObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectOutput::default();

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
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI. This parameted is required if object lock parameters are specified.</p>
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
    /// <p>The object lock mode that you want to apply to this object.</p>
    pub object_lock_mode: Option<String>,
    /// <p>The date and time when you want this object's object lock to expire.</p>
    pub object_lock_retain_until_date: Option<String>,
    pub request_payer: Option<String>,
    /// <p>Specifies the algorithm to use to when encrypting the object (e.g., AES256).</p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon does not store the encryption key. The key must be appropriate for use with the algorithm specified in the x-amz-server-side​-encryption​-customer-algorithm header.</p>
    pub sse_customer_key: Option<String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure the encryption key was transmitted without error.</p>
    pub sse_customer_key_md5: Option<String>,
    /// <p>Specifies the AWS KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub ssekms_encryption_context: Option<String>,
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
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<String>,
}

struct PutObjectRetentionOutputDeserializer;
impl PutObjectRetentionOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectRetentionOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectRetentionOutput::default();

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
pub struct PutObjectTaggingOutput {
    /// <p><p/></p>
    pub version_id: Option<String>,
}

struct PutObjectTaggingOutputDeserializer;
impl PutObjectTaggingOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutObjectTaggingOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = PutObjectTaggingOutput::default();

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
pub struct PutPublicAccessBlockRequest {
    /// <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to set.</p>
    pub bucket: String,
    /// <p>The MD5 hash of the <code>PutPublicAccessBlock</code> request body. </p>
    pub content_md5: Option<String>,
    /// <p>The <code>PublicAccessBlock</code> configuration that you want to apply to this Amazon S3 bucket. You can enable the configuration options in any combination. For more information about when Amazon S3 considers a bucket or object public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
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

/// <p>Specifies the configuration for publishing messages to an Amazon Simple Queue Service (Amazon SQS) queue when Amazon S3 detects specified events.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct QueueConfiguration {
    /// <p><p/></p>
    pub events: Vec<String>,
    /// <p><p/></p>
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SQS queue to which Amazon S3 publishes a message when it detects events of the specified type.</p>
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
/// <p>Specifies how requests are redirected. In the event of an error, you can specify a different error code to return.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Redirect {
    /// <p>The host name to use in the redirect request.</p>
    pub host_name: Option<String>,
    /// <p>The HTTP redirect code to use on the response. Not required if one of the siblings is present.</p>
    pub http_redirect_code: Option<String>,
    /// <p>Protocol to use when redirecting requests. The default is the protocol that is used in the original request.</p>
    pub protocol: Option<String>,
    /// <p>The object key prefix to use in the redirect request. For example, to redirect requests for all pages with prefix <code>docs/</code> (objects in the <code>docs/</code> folder) to <code>documents/</code>, you can set a condition block with <code>KeyPrefixEquals</code> set to <code>docs/</code> and in the Redirect set <code>ReplaceKeyPrefixWith</code> to <code>/documents</code>. Not required if one of the siblings is present. Can be present only if <code>ReplaceKeyWith</code> is not provided.</p>
    pub replace_key_prefix_with: Option<String>,
    /// <p>The specific object key to use in the redirect request. For example, redirect request to <code>error.html</code>. Not required if one of the siblings is present. Can be present only if <code>ReplaceKeyPrefixWith</code> is not provided.</p>
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

/// <p>Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RedirectAllRequestsTo {
    /// <p>Name of the host where requests are redirected.</p>
    pub host_name: String,
    /// <p>Protocol to use when redirecting requests. The default is the protocol that is used in the original request.</p>
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
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that Amazon S3 assumes when replicating objects. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr-how-setup.html">How to Set Up Cross-Region Replication</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
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

/// <p>Specifies which Amazon S3 objects to replicate and where to store the replicas.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReplicationRule {
    /// <p><p/></p>
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    /// <p>A container for information about the replication destination.</p>
    pub destination: Destination,
    /// <p><p/></p>
    pub filter: Option<ReplicationRuleFilter>,
    /// <p>A unique identifier for the rule. The maximum value is 255 characters.</p>
    pub id: Option<String>,
    /// <p>The priority associated with the rule. If you specify multiple rules in a replication configuration, Amazon S3 prioritizes the rules to prevent conflicts when filtering. If two or more rules identify the same object based on a specified filter, the rule with higher priority takes precedence. For example:</p> <ul> <li> <p>Same object quality prefix based filter criteria If prefixes you specified in multiple rules overlap </p> </li> <li> <p>Same object qualify tag based filter criteria specified in multiple rules</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    pub priority: Option<i64>,
    /// <p>A container that describes additional filters for identifying the source objects that you want to replicate. You can choose to enable or disable the replication of these objects. Currently, Amazon S3 supports only the filter that you can specify for objects created with server-side encryption using an AWS KMS-Managed Key (SSE-KMS).</p>
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    /// <p>Specifies whether the rule is enabled.</p>
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
pub struct RestoreObjectOutput {
    pub request_charged: Option<String>,
    /// <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    pub restore_output_path: Option<String>,
}

struct RestoreObjectOutputDeserializer;
impl RestoreObjectOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RestoreObjectOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = RestoreObjectOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestoreObjectRequest {
    /// <p><p/></p>
    pub bucket: String,
    /// <p><p/></p>
    pub key: String,
    pub request_payer: Option<String>,
    /// <p><p/></p>
    pub restore_request: Option<RestoreRequest>,
    /// <p><p/></p>
    pub version_id: Option<String>,
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

/// <p>Specifies the redirect behavior and when a redirect is applied.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoutingRule {
    /// <p>A container for describing a condition that must be met for the specified redirect to apply. For example, 1. If request is for pages in the <code>/docs</code> folder, redirect to the <code>/documents</code> folder. 2. If request results in HTTP error 4xx, redirect request to another host where you might process the error.</p>
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

/// <p>Specifies lifecycle rules for an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTlifecycle.html">PUT Bucket lifecycle</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rule {
    /// <p><p/></p>
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// <p><p/></p>
    pub expiration: Option<LifecycleExpiration>,
    /// <p>Unique identifier for the rule. The value can't be longer than 255 characters.</p>
    pub id: Option<String>,
    /// <p><p/></p>
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    /// <p><p/></p>
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    /// <p>Object key prefix that identifies one or more objects to which this rule applies.</p>
    pub prefix: String,
    /// <p>If <code>Enabled</code>, the rule is currently being applied. If <code>Disabled</code>, the rule is not currently being applied.</p>
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
    /// <p><p/></p>
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
    /// <p><p/></p>
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectObjectContentOutput {
    /// <p><p/></p>
    pub payload: Option<SelectObjectContentEventStream>,
}

struct SelectObjectContentOutputDeserializer;
impl SelectObjectContentOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SelectObjectContentOutput, XmlParseError> {
        Ok(SelectObjectContentOutput {
            payload: Some(SelectObjectContentEventStreamDeserializer::deserialize(
                "Payload", stack,
            )?),
            ..SelectObjectContentOutput::default()
        })
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

/// <p>Describes the default server-side encryption to apply to new objects in the bucket. If a PUT Object request doesn't specify any server-side encryption, this default encryption will be applied. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTencryption.html">PUT Bucket encryption</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionByDefault {
    /// <p>KMS master key ID to use for the default encryption. This parameter is allowed if and only if <code>SSEAlgorithm</code> is set to <code>aws:kms</code>.</p>
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

/// <p>Specifies the default server-side-encryption configuration.</p>
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

/// <p>Specifies the default server-side encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServerSideEncryptionRule {
    /// <p>Specifies the default server-side encryption to apply to new objects in the bucket. If a PUT Object request doesn't specify any server-side encryption, this default encryption will be applied.</p>
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
/// <p>A container that describes additional filters for identifying the source objects that you want to replicate. You can choose to enable or disable the replication of these objects. Currently, Amazon S3 supports only the filter that you can specify for objects created with server-side encryption using an AWS KMS-Managed Key (SSE-KMS).</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceSelectionCriteria {
    /// <p> A container for filter information for the selection of Amazon S3 objects encrypted with AWS KMS. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p>
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
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an AWS KMS-managed key.</p>
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

/// <p>Specifies data related to access patterns to be collected and made available to analyze the tradeoffs between different storage classes for an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StorageClassAnalysis {
    /// <p>Specifies how data related to the storage class analysis for an Amazon S3 bucket should be exported.</p>
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
    /// <p>The version of the output schema to use when exporting data. Must be <code>V_1</code>.</p>
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

/// <p>A container for specifying the configuration for publication of messages to an Amazon Simple Notification Service (Amazon SNS) topic when Amazon S3 detects specified events.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TopicConfiguration {
    /// <p>The Amazon S3 bucket event about which to send notifications. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html">Supported Event Types</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    pub events: Vec<String>,
    /// <p><p/></p>
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which Amazon S3 publishes a message when it detects events of the specified type.</p>
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

/// <p>Specifies when an object transitions to a specified storage class.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transition {
    /// <p>Indicates when objects are transitioned to the specified storage class. The date value must be in ISO 8601 format. The time is always midnight UTC.</p>
    pub date: Option<String>,
    /// <p>Indicates the number of days after creation when objects are transitioned to the specified storage class. The value must be a positive integer.</p>
    pub days: Option<i64>,
    /// <p>The storage class to which you want the object to transition.</p>
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
pub struct UploadPartCopyOutput {
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

struct UploadPartCopyOutputDeserializer;
impl UploadPartCopyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartCopyOutput, XmlParseError> {
        Ok(UploadPartCopyOutput {
            copy_part_result: Some(CopyPartResultDeserializer::deserialize(
                "CopyPartResult",
                stack,
            )?),
            ..UploadPartCopyOutput::default()
        })
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
pub struct UploadPartOutput {
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

struct UploadPartOutputDeserializer;
impl UploadPartOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UploadPartOutput, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UploadPartOutput::default();

        end_element(tag_name, stack)?;

        Ok(obj)
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
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI. This parameted is required if object lock parameters are specified.</p>
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

/// <p>Describes the versioning state of an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTBucketPUTVersioningStatus.html">PUT Bucket versioning</a> in the <i>Amazon Simple Storage Service API Reference</i>.</p>
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

/// <p>Specifies website configuration parameters for an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebsiteConfiguration {
    /// <p>The name of the error document for the website.</p>
    pub error_document: Option<ErrorDocument>,
    /// <p>The name of the index document for the website.</p>
    pub index_document: Option<IndexDocument>,
    /// <p><p>The redirect behavior for every request to this bucket&#39;s website endpoint.</p> <important> <p>If you specify this property, you can&#39;t specify any other property.</p> </important></p>
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    /// <p>Rules that define when a redirect is applied and the redirect behavior.</p>
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
    ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError>;

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError>;

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    fn copy_object(
        &self,
        input: CopyObjectRequest,
    ) -> RusotoFuture<CopyObjectOutput, CopyObjectError>;

    /// <p>Creates a new bucket.</p>
    fn create_bucket(
        &self,
        input: CreateBucketRequest,
    ) -> RusotoFuture<CreateBucketOutput, CreateBucketError>;

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError>;

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    fn delete_bucket(&self, input: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError>;

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p> <p>To use this operation, you must have permissions to perform the s3:PutAnalyticsConfiguration action. The bucket owner has this permission by default. The bucket owner can grant this permission to others. </p>
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError>;

    /// <p>Deletes the CORS configuration information set for the bucket.</p>
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> RusotoFuture<(), DeleteBucketCorsError>;

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> RusotoFuture<(), DeleteBucketEncryptionError>;

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError>;

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> RusotoFuture<(), DeleteBucketLifecycleError>;

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError>;

    /// <p>Deletes the policy from the bucket.</p>
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> RusotoFuture<(), DeleteBucketPolicyError>;

    /// <p> Deletes the replication configuration from the bucket. For information about replication configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> RusotoFuture<(), DeleteBucketReplicationError>;

    /// <p>Deletes the tags from the bucket.</p>
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> RusotoFuture<(), DeleteBucketTaggingError>;

    /// <p>This operation removes the website configuration from the bucket.</p>
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> RusotoFuture<(), DeleteBucketWebsiteError>;

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError>;

    /// <p>Removes the tag-set from an existing object.</p>
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError>;

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    fn delete_objects(
        &self,
        input: DeleteObjectsRequest,
    ) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError>;

    /// <p>Removes the <code>PublicAccessBlock</code> configuration from an Amazon S3 bucket.</p>
    fn delete_public_access_block(
        &self,
        input: DeletePublicAccessBlockRequest,
    ) -> RusotoFuture<(), DeletePublicAccessBlockError>;

    /// <p>Returns the accelerate configuration of a bucket.</p>
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>;

    /// <p>Gets the access control policy for the bucket.</p>
    fn get_bucket_acl(
        &self,
        input: GetBucketAclRequest,
    ) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError>;

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>;

    /// <p>Returns the CORS configuration for the bucket.</p>
    fn get_bucket_cors(
        &self,
        input: GetBucketCorsRequest,
    ) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError>;

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError>;

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>;

    /// <p> No longer used, see the GetBucketLifecycleConfiguration operation.</p>
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError>;

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>;

    /// <p>Returns the region the bucket resides in.</p>
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError>;

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError>;

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError>;

    /// <p> No longer used, see the GetBucketNotificationConfiguration operation.</p>
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError>;

    /// <p>Returns the notification configuration of a bucket.</p>
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError>;

    /// <p>Returns the policy of a specified bucket.</p>
    fn get_bucket_policy(
        &self,
        input: GetBucketPolicyRequest,
    ) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError>;

    /// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public.</p>
    fn get_bucket_policy_status(
        &self,
        input: GetBucketPolicyStatusRequest,
    ) -> RusotoFuture<GetBucketPolicyStatusOutput, GetBucketPolicyStatusError>;

    /// <p><p>Returns the replication configuration of a bucket.</p> <note> <p> It can take a while to propagate the put or delete a replication configuration to all Amazon S3 systems. Therefore, a get request soon after put or delete can return a wrong result. </p> </note></p>
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError>;

    /// <p>Returns the request payment configuration of a bucket.</p>
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError>;

    /// <p>Returns the tag set associated with the bucket.</p>
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError>;

    /// <p>Returns the versioning state of a bucket.</p>
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError>;

    /// <p>Returns the website configuration for a bucket.</p>
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError>;

    /// <p>Retrieves objects from Amazon S3.</p>
    fn get_object(&self, input: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError>;

    /// <p>Returns the access control list (ACL) of an object.</p>
    fn get_object_acl(
        &self,
        input: GetObjectAclRequest,
    ) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError>;

    /// <p>Gets an object's current Legal Hold status.</p>
    fn get_object_legal_hold(
        &self,
        input: GetObjectLegalHoldRequest,
    ) -> RusotoFuture<GetObjectLegalHoldOutput, GetObjectLegalHoldError>;

    /// <p>Gets the object lock configuration for a bucket. The rule specified in the object lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn get_object_lock_configuration(
        &self,
        input: GetObjectLockConfigurationRequest,
    ) -> RusotoFuture<GetObjectLockConfigurationOutput, GetObjectLockConfigurationError>;

    /// <p>Retrieves an object's retention settings.</p>
    fn get_object_retention(
        &self,
        input: GetObjectRetentionRequest,
    ) -> RusotoFuture<GetObjectRetentionOutput, GetObjectRetentionError>;

    /// <p>Returns the tag-set of an object.</p>
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError>;

    /// <p>Return torrent files from a bucket.</p>
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError>;

    /// <p>Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn get_public_access_block(
        &self,
        input: GetPublicAccessBlockRequest,
    ) -> RusotoFuture<GetPublicAccessBlockOutput, GetPublicAccessBlockError>;

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    fn head_bucket(&self, input: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError>;

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    fn head_object(
        &self,
        input: HeadObjectRequest,
    ) -> RusotoFuture<HeadObjectOutput, HeadObjectError>;

    /// <p>Lists the analytics configurations for the bucket.</p>
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>;

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>;

    /// <p>Lists the metrics configurations for the bucket.</p>
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>;

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError>;

    /// <p>This operation lists in-progress multipart uploads.</p>
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError>;

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    fn list_objects(
        &self,
        input: ListObjectsRequest,
    ) -> RusotoFuture<ListObjectsOutput, ListObjectsError>;

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    fn list_objects_v2(
        &self,
        input: ListObjectsV2Request,
    ) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error>;

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    fn list_parts(&self, input: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError>;

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAccelerateConfigurationError>;

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError>;

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError>;

    /// <p>Sets the CORS configuration for a bucket.</p>
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError>;

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> RusotoFuture<(), PutBucketEncryptionError>;

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketInventoryConfigurationError>;

    /// <p> No longer used, see the PutBucketLifecycleConfiguration operation.</p>
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleError>;

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleConfigurationError>;

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> RusotoFuture<(), PutBucketLoggingError>;

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketMetricsConfigurationError>;

    /// <p> No longer used, see the PutBucketNotificationConfiguration operation.</p>
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationError>;

    /// <p>Enables notifications of specified events for a bucket.</p>
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationConfigurationError>;

    /// <p>Applies an Amazon S3 bucket policy to an Amazon S3 bucket.</p>
    fn put_bucket_policy(
        &self,
        input: PutBucketPolicyRequest,
    ) -> RusotoFuture<(), PutBucketPolicyError>;

    /// <p> Creates a replication configuration or replaces an existing one. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> RusotoFuture<(), PutBucketReplicationError>;

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> RusotoFuture<(), PutBucketRequestPaymentError>;

    /// <p>Sets the tags for a bucket.</p>
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> RusotoFuture<(), PutBucketTaggingError>;

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> RusotoFuture<(), PutBucketVersioningError>;

    /// <p>Set the website configuration for a bucket.</p>
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> RusotoFuture<(), PutBucketWebsiteError>;

    /// <p>Adds an object to a bucket.</p>
    fn put_object(&self, input: PutObjectRequest) -> RusotoFuture<PutObjectOutput, PutObjectError>;

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    fn put_object_acl(
        &self,
        input: PutObjectAclRequest,
    ) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError>;

    /// <p>Applies a Legal Hold configuration to the specified object.</p>
    fn put_object_legal_hold(
        &self,
        input: PutObjectLegalHoldRequest,
    ) -> RusotoFuture<PutObjectLegalHoldOutput, PutObjectLegalHoldError>;

    /// <p>Places an object lock configuration on the specified bucket. The rule specified in the object lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    fn put_object_lock_configuration(
        &self,
        input: PutObjectLockConfigurationRequest,
    ) -> RusotoFuture<PutObjectLockConfigurationOutput, PutObjectLockConfigurationError>;

    /// <p>Places an Object Retention configuration on an object.</p>
    fn put_object_retention(
        &self,
        input: PutObjectRetentionRequest,
    ) -> RusotoFuture<PutObjectRetentionOutput, PutObjectRetentionError>;

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError>;

    /// <p>Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    fn put_public_access_block(
        &self,
        input: PutPublicAccessBlockRequest,
    ) -> RusotoFuture<(), PutPublicAccessBlockError>;

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    fn restore_object(
        &self,
        input: RestoreObjectRequest,
    ) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError>;

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError>;

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    fn upload_part(
        &self,
        input: UploadPartRequest,
    ) -> RusotoFuture<UploadPartOutput, UploadPartError>;

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    fn upload_part_copy(
        &self,
        input: UploadPartCopyRequest,
    ) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError>;
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
        Self::new_with_client(Client::shared(), region)
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
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> S3Client {
        S3Client { client, region }
    }
}

impl S3 for S3Client {
    /// <p>Aborts a multipart upload.</p> <p>To verify that all parts have been removed, so you don't get charged for the part storage, you should call the List Parts operation and ensure the parts list is empty.</p>
    #[allow(unused_variables, warnings)]
    fn abort_multipart_upload(
        &self,
        input: AbortMultipartUploadRequest,
    ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = AbortMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = AbortMultipartUploadOutputDeserializer::deserialize(
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

    /// <p>Completes a multipart upload by assembling previously uploaded parts.</p>
    #[allow(unused_variables, warnings)]
    fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest,
    ) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        if input.multipart_upload.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CompletedMultipartUploadSerializer::serialize(
                &mut writer,
                "CompleteMultipartUpload",
                input.multipart_upload.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CompleteMultipartUploadError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CompleteMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CompleteMultipartUploadOutputDeserializer::deserialize(
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

    /// <p>Creates a copy of an object that is already stored in Amazon S3.</p>
    #[allow(unused_variables, warnings)]
    fn copy_object(
        &self,
        input: CopyObjectRequest,
    ) -> RusotoFuture<CopyObjectOutput, CopyObjectError> {
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
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
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

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref metadata_directive) = input.metadata_directive {
            request.add_header("x-amz-metadata-directive", &metadata_directive.to_string());
        }

        if let Some(ref object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = input.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = input.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_encryption_context) = input.ssekms_encryption_context {
            request.add_header(
                "x-amz-server-side-encryption-context",
                &ssekms_encryption_context.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
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
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = CopyObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        CopyObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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
                if let Some(ssekms_encryption_context) =
                    response.headers.get("x-amz-server-side-encryption-context")
                {
                    let value = ssekms_encryption_context.to_owned();
                    result.ssekms_encryption_context = Some(value)
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

    /// <p>Creates a new bucket.</p>
    #[allow(unused_variables, warnings)]
    fn create_bucket(
        &self,
        input: CreateBucketRequest,
    ) -> RusotoFuture<CreateBucketOutput, CreateBucketError> {
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

        if let Some(ref object_lock_enabled_for_bucket) = input.object_lock_enabled_for_bucket {
            request.add_header(
                "x-amz-bucket-object-lock-enabled",
                &object_lock_enabled_for_bucket.to_string(),
            );
        }

        if input.create_bucket_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            CreateBucketConfigurationSerializer::serialize(
                &mut writer,
                "CreateBucketConfiguration",
                input.create_bucket_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = CreateBucketOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        CreateBucketOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(location) = response.headers.get("Location") {
                    let value = location.to_owned();
                    result.location = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Initiates a multipart upload and returns an upload ID.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    #[allow(unused_variables, warnings)]
    fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest,
    ) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError> {
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

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = input.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = input.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_encryption_context) = input.ssekms_encryption_context {
            request.add_header(
                "x-amz-server-side-encryption-context",
                &ssekms_encryption_context.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }
        let mut params = Params::new();
        params.put_key("uploads");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateMultipartUploadError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = CreateMultipartUploadOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CreateMultipartUploadOutputDeserializer::deserialize(
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
                if let Some(ssekms_encryption_context) =
                    response.headers.get("x-amz-server-side-encryption-context")
                {
                    let value = ssekms_encryption_context.to_owned();
                    result.ssekms_encryption_context = Some(value)
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

    /// <p>Deletes the bucket. All objects (including all object versions and Delete Markers) in the bucket must be deleted before the bucket itself can be deleted.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket(&self, input: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes an analytics configuration for the bucket (specified by the analytics configuration ID).</p> <p>To use this operation, you must have permissions to perform the s3:PutAnalyticsConfiguration action. The bucket owner has this permission by default. The bucket owner can grant this permission to others. </p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_analytics_configuration(
        &self,
        input: DeleteBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketAnalyticsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the CORS configuration information set for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_cors(
        &self,
        input: DeleteBucketCorsRequest,
    ) -> RusotoFuture<(), DeleteBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketCorsError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the server-side encryption configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_encryption(
        &self,
        input: DeleteBucketEncryptionRequest,
    ) -> RusotoFuture<(), DeleteBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketEncryptionError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_inventory_configuration(
        &self,
        input: DeleteBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketInventoryConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the lifecycle configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_lifecycle(
        &self,
        input: DeleteBucketLifecycleRequest,
    ) -> RusotoFuture<(), DeleteBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketLifecycleError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_metrics_configuration(
        &self,
        input: DeleteBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketMetricsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the policy from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_policy(
        &self,
        input: DeleteBucketPolicyRequest,
    ) -> RusotoFuture<(), DeleteBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBucketPolicyError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> Deletes the replication configuration from the bucket. For information about replication configuration, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_replication(
        &self,
        input: DeleteBucketReplicationRequest,
    ) -> RusotoFuture<(), DeleteBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBucketReplicationError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Deletes the tags from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_tagging(
        &self,
        input: DeleteBucketTaggingRequest,
    ) -> RusotoFuture<(), DeleteBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteBucketTaggingError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>This operation removes the website configuration from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_bucket_website(
        &self,
        input: DeleteBucketWebsiteRequest,
    ) -> RusotoFuture<(), DeleteBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteBucketWebsiteError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects.</p>
    #[allow(unused_variables, warnings)]
    fn delete_object(
        &self,
        input: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        if let Some(ref bypass_governance_retention) = input.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = DeleteObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        DeleteObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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

    /// <p>Removes the tag-set from an existing object.</p>
    #[allow(unused_variables, warnings)]
    fn delete_object_tagging(
        &self,
        input: DeleteObjectTaggingRequest,
    ) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = DeleteObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteObjectTaggingOutputDeserializer::deserialize(
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

    /// <p>This operation enables you to delete multiple objects from a bucket using a single HTTP request. You may specify up to 1000 keys.</p>
    #[allow(unused_variables, warnings)]
    fn delete_objects(
        &self,
        input: DeleteObjectsRequest,
    ) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref bypass_governance_retention) = input.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

        if let Some(ref mfa) = input.mfa {
            request.add_header("x-amz-mfa", &mfa.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("delete");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        DeleteSerializer::serialize(&mut writer, "Delete", &input.delete);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
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
                    result = DeleteObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        DeleteObjectsOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Removes the <code>PublicAccessBlock</code> configuration from an Amazon S3 bucket.</p>
    #[allow(unused_variables, warnings)]
    fn delete_public_access_block(
        &self,
        input: DeletePublicAccessBlockRequest,
    ) -> RusotoFuture<(), DeletePublicAccessBlockError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("DELETE", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeletePublicAccessBlockError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Returns the accelerate configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_accelerate_configuration(
        &self,
        input: GetBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketAccelerateConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketAccelerateConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Gets the access control policy for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_acl(
        &self,
        input: GetBucketAclRequest,
    ) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("acl");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        GetBucketAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Gets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_analytics_configuration(
        &self,
        input: GetBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketAnalyticsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketAnalyticsConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the CORS configuration for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_cors(
        &self,
        input: GetBucketCorsRequest,
    ) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketCorsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        GetBucketCorsOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the server-side encryption configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_encryption(
        &self,
        input: GetBucketEncryptionRequest,
    ) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketEncryptionOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketEncryptionOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_inventory_configuration(
        &self,
        input: GetBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketInventoryConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketInventoryConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p> No longer used, see the GetBucketLifecycleConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle(
        &self,
        input: GetBucketLifecycleRequest,
    ) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketLifecycleOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLifecycleOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the lifecycle configuration information set on the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_lifecycle_configuration(
        &self,
        input: GetBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketLifecycleConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLifecycleConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the region the bucket resides in.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest,
    ) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("location");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketLocationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLocationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_logging(
        &self,
        input: GetBucketLoggingRequest,
    ) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketLoggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketLoggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Gets a metrics configuration (specified by the metrics configuration ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_metrics_configuration(
        &self,
        input: GetBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketMetricsConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketMetricsConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketMetricsConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p> No longer used, see the GetBucketNotificationConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketNotificationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = NotificationConfigurationDeprecated::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = NotificationConfigurationDeprecatedDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the notification configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_notification_configuration(
        &self,
        input: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = NotificationConfiguration::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = NotificationConfigurationDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the policy of a specified bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_policy(
        &self,
        input: GetBucketPolicyRequest,
    ) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBucketPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().map(move |response| {
                let mut result = GetBucketPolicyOutput::default();
                result.policy = Some(String::from_utf8_lossy(response.body.as_ref()).into());

                result
            }))
        })
    }

    /// <p>Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_policy_status(
        &self,
        input: GetBucketPolicyStatusRequest,
    ) -> RusotoFuture<GetBucketPolicyStatusOutput, GetBucketPolicyStatusError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("policyStatus");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketPolicyStatusError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketPolicyStatusOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketPolicyStatusOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p><p>Returns the replication configuration of a bucket.</p> <note> <p> It can take a while to propagate the put or delete a replication configuration to all Amazon S3 systems. Therefore, a get request soon after put or delete can return a wrong result. </p> </note></p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_replication(
        &self,
        input: GetBucketReplicationRequest,
    ) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketReplicationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketReplicationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the request payment configuration of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_request_payment(
        &self,
        input: GetBucketRequestPaymentRequest,
    ) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBucketRequestPaymentError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetBucketRequestPaymentOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketRequestPaymentOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the tag set associated with the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_tagging(
        &self,
        input: GetBucketTaggingRequest,
    ) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketTaggingOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the versioning state of a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_versioning(
        &self,
        input: GetBucketVersioningRequest,
    ) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("versioning");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketVersioningOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketVersioningOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the website configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_bucket_website(
        &self,
        input: GetBucketWebsiteRequest,
    ) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetBucketWebsiteOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetBucketWebsiteOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Retrieves objects from Amazon S3.</p>
    #[allow(unused_variables, warnings)]
    fn get_object(&self, input: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError> {
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
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
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

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectError::from_response(response))),
                );
            }

            let mut result = GetObjectOutput::default();
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

    /// <p>Returns the access control list (ACL) of an object.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_acl(
        &self,
        input: GetObjectAclRequest,
    ) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        GetObjectAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Gets an object's current Legal Hold status.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_legal_hold(
        &self,
        input: GetObjectLegalHoldRequest,
    ) -> RusotoFuture<GetObjectLegalHoldOutput, GetObjectLegalHoldError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("legal-hold");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetObjectLegalHoldOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectLegalHoldOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Gets the object lock configuration for a bucket. The rule specified in the object lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_lock_configuration(
        &self,
        input: GetObjectLockConfigurationRequest,
    ) -> RusotoFuture<GetObjectLockConfigurationOutput, GetObjectLockConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("object-lock");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetObjectLockConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = GetObjectLockConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectLockConfigurationOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Retrieves an object's retention settings.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_retention(
        &self,
        input: GetObjectRetentionRequest,
    ) -> RusotoFuture<GetObjectRetentionOutput, GetObjectRetentionError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("retention");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetObjectRetentionOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectRetentionOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns the tag-set of an object.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_tagging(
        &self,
        input: GetObjectTaggingRequest,
    ) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("tagging");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetObjectTaggingOutputDeserializer::deserialize(
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

    /// <p>Return torrent files from a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_object_torrent(
        &self,
        input: GetObjectTorrentRequest,
    ) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        params.put_key("torrent");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetObjectTorrentError::from_response(response))),
                );
            }

            let mut result = GetObjectTorrentOutput::default();
            result.body = Some(response.body);
            if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                let value = request_charged.to_owned();
                result.request_charged = Some(value)
            };
            Box::new(future::ok(result))
        })
    }

    /// <p>Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    #[allow(unused_variables, warnings)]
    fn get_public_access_block(
        &self,
        input: GetPublicAccessBlockRequest,
    ) -> RusotoFuture<GetPublicAccessBlockOutput, GetPublicAccessBlockError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = GetPublicAccessBlockOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = GetPublicAccessBlockOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>This operation is useful to determine if a bucket exists and you have permission to access it.</p>
    #[allow(unused_variables, warnings)]
    fn head_bucket(&self, input: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("HEAD", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(HeadBucketError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>The HEAD operation retrieves metadata from an object without returning the object itself. This operation is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object.</p>
    #[allow(unused_variables, warnings)]
    fn head_object(
        &self,
        input: HeadObjectRequest,
    ) -> RusotoFuture<HeadObjectOutput, HeadObjectError> {
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
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        if let Some(ref x) = input.part_number {
            params.put("partNumber", x);
        }
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = HeadObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        HeadObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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

    /// <p>Lists the analytics configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_analytics_configurations(
        &self,
        input: ListBucketAnalyticsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("analytics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListBucketAnalyticsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketAnalyticsConfigurationsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of inventory configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_inventory_configurations(
        &self,
        input: ListBucketInventoryConfigurationsRequest,
    ) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("inventory");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListBucketInventoryConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketInventoryConfigurationsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Lists the metrics configurations for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_bucket_metrics_configurations(
        &self,
        input: ListBucketMetricsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>
    {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        if let Some(ref x) = input.continuation_token {
            params.put("continuation-token", x);
        }
        params.put_key("metrics");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListBucketMetricsConfigurationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListBucketMetricsConfigurationsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns a list of all buckets owned by the authenticated sender of the request.</p>
    #[allow(unused_variables, warnings)]
    fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError> {
        let request_uri = "/";

        let mut request = SignedRequest::new("GET", "s3", &self.region, &request_uri);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListBucketsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListBucketsOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>This operation lists in-progress multipart uploads.</p>
    #[allow(unused_variables, warnings)]
    fn list_multipart_uploads(
        &self,
        input: ListMultipartUploadsRequest,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListMultipartUploadsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListMultipartUploadsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns metadata about all of the versions of objects in a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_object_versions(
        &self,
        input: ListObjectVersionsRequest,
    ) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListObjectVersionsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = ListObjectVersionsOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn list_objects(
        &self,
        input: ListObjectsRequest,
    ) -> RusotoFuture<ListObjectsOutput, ListObjectsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListObjectsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListObjectsOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Returns some or all (up to 1000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. Note: ListObjectsV2 is the revised List Objects API and we recommend you use this revised API for new application development.</p>
    #[allow(unused_variables, warnings)]
    fn list_objects_v2(
        &self,
        input: ListObjectsV2Request,
    ) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListObjectsV2Output::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListObjectsV2OutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Lists the parts that have been uploaded for a specific multipart upload.</p>
    #[allow(unused_variables, warnings)]
    fn list_parts(&self, input: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError> {
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

        self.client.sign_and_dispatch(request, |response| {
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
                    result = ListPartsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        ListPartsOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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

    /// <p>Sets the accelerate configuration of an existing bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_accelerate_configuration(
        &self,
        input: PutBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAccelerateConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("accelerate");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AccelerateConfigurationSerializer::serialize(
            &mut writer,
            "AccelerateConfiguration",
            &input.accelerate_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAccelerateConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the permissions on a bucket using access control lists (ACL).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_acl(&self, input: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError> {
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
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                input.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketAclError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets an analytics configuration for the bucket (specified by the analytics configuration ID).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_analytics_configuration(
        &self,
        input: PutBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("analytics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        AnalyticsConfigurationSerializer::serialize(
            &mut writer,
            "AnalyticsConfiguration",
            &input.analytics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketAnalyticsConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the CORS configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_cors(&self, input: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("cors");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        CORSConfigurationSerializer::serialize(
            &mut writer,
            "CORSConfiguration",
            &input.cors_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketCorsError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Creates a new server-side encryption configuration (or replaces an existing one, if present).</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_encryption(
        &self,
        input: PutBucketEncryptionRequest,
    ) -> RusotoFuture<(), PutBucketEncryptionError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("encryption");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ServerSideEncryptionConfigurationSerializer::serialize(
            &mut writer,
            "ServerSideEncryptionConfiguration",
            &input.server_side_encryption_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketEncryptionError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Adds an inventory configuration (identified by the inventory ID) from the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_inventory_configuration(
        &self,
        input: PutBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketInventoryConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("inventory");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        InventoryConfigurationSerializer::serialize(
            &mut writer,
            "InventoryConfiguration",
            &input.inventory_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketInventoryConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> No longer used, see the PutBucketLifecycleConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle(
        &self,
        input: PutBucketLifecycleRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            LifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                input.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketLifecycleError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets lifecycle configuration for your bucket. If a lifecycle configuration exists, it replaces it.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_lifecycle_configuration(
        &self,
        input: PutBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("lifecycle");
        request.set_params(params);
        if input.lifecycle_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            BucketLifecycleConfigurationSerializer::serialize(
                &mut writer,
                "LifecycleConfiguration",
                input.lifecycle_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketLifecycleConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. To set the logging status of a bucket, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_logging(
        &self,
        input: PutBucketLoggingRequest,
    ) -> RusotoFuture<(), PutBucketLoggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("logging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        BucketLoggingStatusSerializer::serialize(
            &mut writer,
            "BucketLoggingStatus",
            &input.bucket_logging_status,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketLoggingError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets a metrics configuration (specified by the metrics configuration ID) for the bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_metrics_configuration(
        &self,
        input: PutBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketMetricsConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put("id", &input.id);
        params.put_key("metrics");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        MetricsConfigurationSerializer::serialize(
            &mut writer,
            "MetricsConfiguration",
            &input.metrics_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketMetricsConfigurationError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> No longer used, see the PutBucketNotificationConfiguration operation.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification(
        &self,
        input: PutBucketNotificationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationDeprecatedSerializer::serialize(
            &mut writer,
            "NotificationConfigurationDeprecated",
            &input.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Enables notifications of specified events for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_notification_configuration(
        &self,
        input: PutBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        let mut params = Params::new();
        params.put_key("notification");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        NotificationConfigurationSerializer::serialize(
            &mut writer,
            "NotificationConfiguration",
            &input.notification_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketNotificationConfigurationError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Applies an Amazon S3 bucket policy to an Amazon S3 bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_policy(
        &self,
        input: PutBucketPolicyRequest,
    ) -> RusotoFuture<(), PutBucketPolicyError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref confirm_remove_self_bucket_access) = input.confirm_remove_self_bucket_access
        {
            request.add_header(
                "x-amz-confirm-remove-self-bucket-access",
                &confirm_remove_self_bucket_access.to_string(),
            );
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("policy");
        request.set_params(params);
        request.set_payload(Some(input.policy.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketPolicyError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p> Creates a replication configuration or replaces an existing one. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/crr.html">Cross-Region Replication (CRR)</a> in the <i>Amazon S3 Developer Guide</i>. </p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_replication(
        &self,
        input: PutBucketReplicationRequest,
    ) -> RusotoFuture<(), PutBucketReplicationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref token) = input.token {
            request.add_header("x-amz-bucket-object-lock-token", &token.to_string());
        }
        let mut params = Params::new();
        params.put_key("replication");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        ReplicationConfigurationSerializer::serialize(
            &mut writer,
            "ReplicationConfiguration",
            &input.replication_configuration,
        );
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketReplicationError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. Documentation on requester pays buckets can be found at http://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_request_payment(
        &self,
        input: PutBucketRequestPaymentRequest,
    ) -> RusotoFuture<(), PutBucketRequestPaymentError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("requestPayment");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        RequestPaymentConfigurationSerializer::serialize(
            &mut writer,
            "RequestPaymentConfiguration",
            &input.request_payment_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutBucketRequestPaymentError::from_response(response))
                }));
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the tags for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_tagging(
        &self,
        input: PutBucketTaggingRequest,
    ) -> RusotoFuture<(), PutBucketTaggingError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("tagging");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        request.set_payload(Some(writer.into_inner()));
        request.set_content_md5_header();

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketTaggingError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Sets the versioning state of an existing bucket. To set the versioning state, you must be the bucket owner.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_versioning(
        &self,
        input: PutBucketVersioningRequest,
    ) -> RusotoFuture<(), PutBucketVersioningError> {
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
        let mut writer = EventWriter::new(Vec::new());
        VersioningConfigurationSerializer::serialize(
            &mut writer,
            "VersioningConfiguration",
            &input.versioning_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutBucketVersioningError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Set the website configuration for a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_bucket_website(
        &self,
        input: PutBucketWebsiteRequest,
    ) -> RusotoFuture<(), PutBucketWebsiteError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("website");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        WebsiteConfigurationSerializer::serialize(
            &mut writer,
            "WebsiteConfiguration",
            &input.website_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBucketWebsiteError::from_response(response))),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Adds an object to a bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_object(&self, input: PutObjectRequest) -> RusotoFuture<PutObjectOutput, PutObjectError> {
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

        if let Some(ref metadata) = input.metadata {
            for (header_name, header_value) in metadata.iter() {
                let header = format!("x-amz-meta-{}", header_name);
                request.add_header(header, header_value);
            }
        }

        if let Some(ref object_lock_legal_hold_status) = input.object_lock_legal_hold_status {
            request.add_header(
                "x-amz-object-lock-legal-hold",
                &object_lock_legal_hold_status.to_string(),
            );
        }

        if let Some(ref object_lock_mode) = input.object_lock_mode {
            request.add_header("x-amz-object-lock-mode", &object_lock_mode.to_string());
        }

        if let Some(ref object_lock_retain_until_date) = input.object_lock_retain_until_date {
            request.add_header(
                "x-amz-object-lock-retain-until-date",
                &object_lock_retain_until_date.to_string(),
            );
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref ssekms_encryption_context) = input.ssekms_encryption_context {
            request.add_header(
                "x-amz-server-side-encryption-context",
                &ssekms_encryption_context.to_string(),
            );
        }

        if let Some(ref ssekms_key_id) = input.ssekms_key_id {
            request.add_header(
                "x-amz-server-side-encryption-aws-kms-key-id",
                &ssekms_key_id.to_string(),
            );
        }

        if let Some(ref server_side_encryption) = input.server_side_encryption {
            request.add_header(
                "x-amz-server-side-encryption",
                &server_side_encryption.to_string(),
            );
        }

        if let Some(ref storage_class) = input.storage_class {
            request.add_header("x-amz-storage-class", &storage_class.to_string());
        }

        if let Some(ref tagging) = input.tagging {
            request.add_header("x-amz-tagging", &tagging.to_string());
        }

        if let Some(ref website_redirect_location) = input.website_redirect_location {
            request.add_header(
                "x-amz-website-redirect-location",
                &website_redirect_location.to_string(),
            );
        }

        if let Some(__body) = input.body {
            request.set_payload_stream(__body);
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = PutObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        PutObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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
                if let Some(ssekms_encryption_context) =
                    response.headers.get("x-amz-server-side-encryption-context")
                {
                    let value = ssekms_encryption_context.to_owned();
                    result.ssekms_encryption_context = Some(value)
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

    /// <p>uses the acl subresource to set the access control list (ACL) permissions for an object that already exists in a bucket</p>
    #[allow(unused_variables, warnings)]
    fn put_object_acl(
        &self,
        input: PutObjectAclRequest,
    ) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError> {
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
        if input.access_control_policy.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            AccessControlPolicySerializer::serialize(
                &mut writer,
                "AccessControlPolicy",
                input.access_control_policy.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = PutObjectAclOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        PutObjectAclOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                if let Some(request_charged) = response.headers.get("x-amz-request-charged") {
                    let value = request_charged.to_owned();
                    result.request_charged = Some(value)
                }; // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Applies a Legal Hold configuration to the specified object.</p>
    #[allow(unused_variables, warnings)]
    fn put_object_legal_hold(
        &self,
        input: PutObjectLegalHoldRequest,
    ) -> RusotoFuture<PutObjectLegalHoldOutput, PutObjectLegalHoldError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("legal-hold");
        request.set_params(params);
        if input.legal_hold.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockLegalHoldSerializer::serialize(
                &mut writer,
                "LegalHold",
                input.legal_hold.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = PutObjectLegalHoldOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectLegalHoldOutputDeserializer::deserialize(
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

    /// <p>Places an object lock configuration on the specified bucket. The rule specified in the object lock configuration will be applied by default to every new object placed in the specified bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_object_lock_configuration(
        &self,
        input: PutObjectLockConfigurationRequest,
    ) -> RusotoFuture<PutObjectLockConfigurationOutput, PutObjectLockConfigurationError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref token) = input.token {
            request.add_header("x-amz-bucket-object-lock-token", &token.to_string());
        }
        let mut params = Params::new();
        params.put_key("object-lock");
        request.set_params(params);
        if input.object_lock_configuration.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockConfigurationSerializer::serialize(
                &mut writer,
                "ObjectLockConfiguration",
                input.object_lock_configuration.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutObjectLockConfigurationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let mut result;

                if response.body.is_empty() {
                    result = PutObjectLockConfigurationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectLockConfigurationOutputDeserializer::deserialize(
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

    /// <p>Places an Object Retention configuration on an object.</p>
    #[allow(unused_variables, warnings)]
    fn put_object_retention(
        &self,
        input: PutObjectRetentionRequest,
    ) -> RusotoFuture<PutObjectRetentionOutput, PutObjectRetentionError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref bypass_governance_retention) = input.bypass_governance_retention {
            request.add_header(
                "x-amz-bypass-governance-retention",
                &bypass_governance_retention.to_string(),
            );
        }

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        params.put_key("retention");
        request.set_params(params);
        if input.retention.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            ObjectLockRetentionSerializer::serialize(
                &mut writer,
                "Retention",
                input.retention.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = PutObjectRetentionOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectRetentionOutputDeserializer::deserialize(
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

    /// <p>Sets the supplied tag-set to an object that already exists in a bucket</p>
    #[allow(unused_variables, warnings)]
    fn put_object_tagging(
        &self,
        input: PutObjectTaggingRequest,
    ) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError> {
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
        let mut writer = EventWriter::new(Vec::new());
        TaggingSerializer::serialize(&mut writer, "Tagging", &input.tagging);
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
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
                    result = PutObjectTaggingOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutObjectTaggingOutputDeserializer::deserialize(
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

    /// <p>Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket.</p>
    #[allow(unused_variables, warnings)]
    fn put_public_access_block(
        &self,
        input: PutPublicAccessBlockRequest,
    ) -> RusotoFuture<(), PutPublicAccessBlockError> {
        let request_uri = format!("/{bucket}", bucket = input.bucket);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        if let Some(ref content_md5) = input.content_md5 {
            request.add_header("Content-MD5", &content_md5.to_string());
        }
        let mut params = Params::new();
        params.put_key("publicAccessBlock");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        PublicAccessBlockConfigurationSerializer::serialize(
            &mut writer,
            "PublicAccessBlockConfiguration",
            &input.public_access_block_configuration,
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutPublicAccessBlockError::from_response(response))
                    }),
                );
            }

            Box::new(future::ok(::std::mem::drop(response)))
        })
    }

    /// <p>Restores an archived copy of an object back into Amazon S3</p>
    #[allow(unused_variables, warnings)]
    fn restore_object(
        &self,
        input: RestoreObjectRequest,
    ) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError> {
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
        if input.restore_request.is_some() {
            let mut writer = EventWriter::new(Vec::new());
            RestoreRequestSerializer::serialize(
                &mut writer,
                "RestoreRequest",
                input.restore_request.as_ref().unwrap(),
            );
            request.set_payload(Some(writer.into_inner()));
        } else {
            request.set_payload(Some(Vec::new()));
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = RestoreObjectOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        RestoreObjectOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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

    /// <p>This operation filters the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must also specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records, and returns only records that match the specified SQL expression. You must also specify the data serialization format for the response.</p>
    #[allow(unused_variables, warnings)]
    fn select_object_content(
        &self,
        input: SelectObjectContentRequest,
    ) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("POST", "s3", &self.region, &request_uri);

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put_key("select");
        params.put("select-type", "2");
        request.set_params(params);
        let mut writer = EventWriter::new(Vec::new());
        SelectObjectContentRequestSerializer::serialize(
            &mut writer,
            "SelectObjectContentRequest",
            &input,
            "http://s3.amazonaws.com/doc/2006-03-01/",
        );
        request.set_payload(Some(writer.into_inner()));

        self.client.sign_and_dispatch(request, |response| {
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
                    result = SelectObjectContentOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = SelectObjectContentOutputDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }

    /// <p>Uploads a part in a multipart upload.</p> <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
    #[allow(unused_variables, warnings)]
    fn upload_part(
        &self,
        input: UploadPartRequest,
    ) -> RusotoFuture<UploadPartOutput, UploadPartError> {
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
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);
        if let Some(__body) = input.body {
            request.set_payload_stream(__body);
        }

        self.client.sign_and_dispatch(request, |response| {
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
                    result = UploadPartOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        UploadPartOutputDeserializer::deserialize(&actual_tag_name, &mut stack)?;
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

    /// <p>Uploads a part by copying data from an existing object as data source.</p>
    #[allow(unused_variables, warnings)]
    fn upload_part_copy(
        &self,
        input: UploadPartCopyRequest,
    ) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError> {
        let request_uri = format!("/{bucket}/{key}", bucket = input.bucket, key = input.key);

        let mut request = SignedRequest::new("PUT", "s3", &self.region, &request_uri);

        request.add_header("x-amz-copy-source", &input.copy_source);

        if let Some(ref copy_source_if_match) = input.copy_source_if_match {
            request.add_header(
                "x-amz-copy-source-if-match",
                &copy_source_if_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_modified_since) = input.copy_source_if_modified_since {
            request.add_header(
                "x-amz-copy-source-if-modified-since",
                &copy_source_if_modified_since.to_string(),
            );
        }

        if let Some(ref copy_source_if_none_match) = input.copy_source_if_none_match {
            request.add_header(
                "x-amz-copy-source-if-none-match",
                &copy_source_if_none_match.to_string(),
            );
        }

        if let Some(ref copy_source_if_unmodified_since) = input.copy_source_if_unmodified_since {
            request.add_header(
                "x-amz-copy-source-if-unmodified-since",
                &copy_source_if_unmodified_since.to_string(),
            );
        }

        if let Some(ref copy_source_range) = input.copy_source_range {
            request.add_header("x-amz-copy-source-range", &copy_source_range.to_string());
        }

        if let Some(ref copy_source_sse_customer_algorithm) =
            input.copy_source_sse_customer_algorithm
        {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-algorithm",
                &copy_source_sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key) = input.copy_source_sse_customer_key {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key",
                &copy_source_sse_customer_key.to_string(),
            );
        }

        if let Some(ref copy_source_sse_customer_key_md5) = input.copy_source_sse_customer_key_md5 {
            request.add_header(
                "x-amz-copy-source-server-side-encryption-customer-key-MD5",
                &copy_source_sse_customer_key_md5.to_string(),
            );
        }

        if let Some(ref request_payer) = input.request_payer {
            request.add_header("x-amz-request-payer", &request_payer.to_string());
        }

        if let Some(ref sse_customer_algorithm) = input.sse_customer_algorithm {
            request.add_header(
                "x-amz-server-side-encryption-customer-algorithm",
                &sse_customer_algorithm.to_string(),
            );
        }

        if let Some(ref sse_customer_key) = input.sse_customer_key {
            request.add_header(
                "x-amz-server-side-encryption-customer-key",
                &sse_customer_key.to_string(),
            );
        }

        if let Some(ref sse_customer_key_md5) = input.sse_customer_key_md5 {
            request.add_header(
                "x-amz-server-side-encryption-customer-key-MD5",
                &sse_customer_key_md5.to_string(),
            );
        }
        let mut params = Params::new();
        params.put("partNumber", &input.part_number);
        params.put("uploadId", &input.upload_id);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
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
                    result = UploadPartCopyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = UploadPartCopyOutputDeserializer::deserialize(
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
