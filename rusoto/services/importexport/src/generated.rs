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
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use rusoto_core::xmlerror::*;
use rusoto_core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
/// <p>A discrete item that contains the description and URL of an artifact (such as a PDF).</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Artifact {
    pub description: Option<String>,
    pub url: Option<String>,
}

struct ArtifactDeserializer;
impl ArtifactDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Artifact, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Artifact::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "URL" => {
                        obj.url = Some(try!(URLDeserializer::deserialize("URL", stack)));
                    }
                    _ => skip_tree(stack),
                },
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
struct ArtifactListDeserializer;
impl ArtifactListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Artifact>, XmlParseError> {
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
                    if name == "member" {
                        obj.push(try!(ArtifactDeserializer::deserialize("member", stack)));
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
/// <p>Input structure for the CancelJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CancelJobInput {
    pub api_version: Option<String>,
    pub job_id: String,
}

/// Serialize `CancelJobInput` contents to a `SignedRequest`.
struct CancelJobInputSerializer;
impl CancelJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CancelJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "JobId"),
            &obj.job_id.replace("+", "%2B"),
        );
    }
}

/// <p>Output structure for the CancelJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CancelJobOutput {
    pub success: Option<bool>,
}

struct CancelJobOutputDeserializer;
impl CancelJobOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CancelJobOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CancelJobOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "Success" => {
                        obj.success =
                            Some(try!(SuccessDeserializer::deserialize("Success", stack)));
                    }
                    _ => skip_tree(stack),
                },
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
struct CarrierDeserializer;
impl CarrierDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Input structure for the CreateJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateJobInput {
    pub api_version: Option<String>,
    pub job_type: String,
    pub manifest: String,
    pub manifest_addendum: Option<String>,
    pub validate_only: bool,
}

/// Serialize `CreateJobInput` contents to a `SignedRequest`.
struct CreateJobInputSerializer;
impl CreateJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "JobType"),
            &obj.job_type.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Manifest"),
            &obj.manifest.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.manifest_addendum {
            params.put(
                &format!("{}{}", prefix, "ManifestAddendum"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "ValidateOnly"),
            &obj.validate_only.to_string().replace("+", "%2B"),
        );
    }
}

/// <p>Output structure for the CreateJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateJobOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
    pub signature: Option<String>,
    pub signature_file_contents: Option<String>,
    pub warning_message: Option<String>,
}

struct CreateJobOutputDeserializer;
impl CreateJobOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateJobOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateJobOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ArtifactList" => {
                        obj.artifact_list = Some(try!(ArtifactListDeserializer::deserialize(
                            "ArtifactList",
                            stack
                        )));
                    }
                    "JobId" => {
                        obj.job_id = Some(try!(JobIdDeserializer::deserialize("JobId", stack)));
                    }
                    "JobType" => {
                        obj.job_type =
                            Some(try!(JobTypeDeserializer::deserialize("JobType", stack)));
                    }
                    "Signature" => {
                        obj.signature =
                            Some(try!(SignatureDeserializer::deserialize("Signature", stack)));
                    }
                    "SignatureFileContents" => {
                        obj.signature_file_contents =
                            Some(try!(SignatureFileContentsDeserializer::deserialize(
                                "SignatureFileContents",
                                stack
                            )));
                    }
                    "WarningMessage" => {
                        obj.warning_message = Some(try!(WarningMessageDeserializer::deserialize(
                            "WarningMessage",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
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
struct CreationDateDeserializer;
impl CreationDateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct CurrentManifestDeserializer;
impl CurrentManifestDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DescriptionDeserializer;
impl DescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ErrorCountDeserializer;
impl ErrorCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<i64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct GenericStringDeserializer;
impl GenericStringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetShippingLabelInput {
    pub api_version: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub job_ids: Vec<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: Option<String>,
    pub state_or_province: Option<String>,
    pub street_1: Option<String>,
    pub street_2: Option<String>,
    pub street_3: Option<String>,
}

/// Serialize `GetShippingLabelInput` contents to a `SignedRequest`.
struct GetShippingLabelInputSerializer;
impl GetShippingLabelInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetShippingLabelInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.city {
            params.put(
                &format!("{}{}", prefix, "city"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.company {
            params.put(
                &format!("{}{}", prefix, "company"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.country {
            params.put(
                &format!("{}{}", prefix, "country"),
                &field_value.replace("+", "%2B"),
            );
        }
        JobIdListSerializer::serialize(params, &format!("{}{}", prefix, "jobIds"), &obj.job_ids);
        if let Some(ref field_value) = obj.name {
            params.put(
                &format!("{}{}", prefix, "name"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.phone_number {
            params.put(
                &format!("{}{}", prefix, "phoneNumber"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.postal_code {
            params.put(
                &format!("{}{}", prefix, "postalCode"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.state_or_province {
            params.put(
                &format!("{}{}", prefix, "stateOrProvince"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.street_1 {
            params.put(
                &format!("{}{}", prefix, "street1"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.street_2 {
            params.put(
                &format!("{}{}", prefix, "street2"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.street_3 {
            params.put(
                &format!("{}{}", prefix, "street3"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetShippingLabelOutput {
    pub shipping_label_url: Option<String>,
    pub warning: Option<String>,
}

struct GetShippingLabelOutputDeserializer;
impl GetShippingLabelOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetShippingLabelOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetShippingLabelOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ShippingLabelURL" => {
                        obj.shipping_label_url = Some(try!(
                            GenericStringDeserializer::deserialize("ShippingLabelURL", stack)
                        ));
                    }
                    "Warning" => {
                        obj.warning = Some(try!(GenericStringDeserializer::deserialize(
                            "Warning", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
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
/// <p>Input structure for the GetStatus operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStatusInput {
    pub api_version: Option<String>,
    pub job_id: String,
}

/// Serialize `GetStatusInput` contents to a `SignedRequest`.
struct GetStatusInputSerializer;
impl GetStatusInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetStatusInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "JobId"),
            &obj.job_id.replace("+", "%2B"),
        );
    }
}

/// <p>Output structure for the GetStatus operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStatusOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub carrier: Option<String>,
    pub creation_date: Option<String>,
    pub current_manifest: Option<String>,
    pub error_count: Option<i64>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
    pub location_code: Option<String>,
    pub location_message: Option<String>,
    pub log_bucket: Option<String>,
    pub log_key: Option<String>,
    pub progress_code: Option<String>,
    pub progress_message: Option<String>,
    pub signature: Option<String>,
    pub signature_file_contents: Option<String>,
    pub tracking_number: Option<String>,
}

struct GetStatusOutputDeserializer;
impl GetStatusOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStatusOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetStatusOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ArtifactList" => {
                        obj.artifact_list = Some(try!(ArtifactListDeserializer::deserialize(
                            "ArtifactList",
                            stack
                        )));
                    }
                    "Carrier" => {
                        obj.carrier =
                            Some(try!(CarrierDeserializer::deserialize("Carrier", stack)));
                    }
                    "CreationDate" => {
                        obj.creation_date = Some(try!(CreationDateDeserializer::deserialize(
                            "CreationDate",
                            stack
                        )));
                    }
                    "CurrentManifest" => {
                        obj.current_manifest = Some(try!(
                            CurrentManifestDeserializer::deserialize("CurrentManifest", stack)
                        ));
                    }
                    "ErrorCount" => {
                        obj.error_count = Some(try!(ErrorCountDeserializer::deserialize(
                            "ErrorCount",
                            stack
                        )));
                    }
                    "JobId" => {
                        obj.job_id = Some(try!(JobIdDeserializer::deserialize("JobId", stack)));
                    }
                    "JobType" => {
                        obj.job_type =
                            Some(try!(JobTypeDeserializer::deserialize("JobType", stack)));
                    }
                    "LocationCode" => {
                        obj.location_code = Some(try!(LocationCodeDeserializer::deserialize(
                            "LocationCode",
                            stack
                        )));
                    }
                    "LocationMessage" => {
                        obj.location_message = Some(try!(
                            LocationMessageDeserializer::deserialize("LocationMessage", stack)
                        ));
                    }
                    "LogBucket" => {
                        obj.log_bucket =
                            Some(try!(LogBucketDeserializer::deserialize("LogBucket", stack)));
                    }
                    "LogKey" => {
                        obj.log_key = Some(try!(LogKeyDeserializer::deserialize("LogKey", stack)));
                    }
                    "ProgressCode" => {
                        obj.progress_code = Some(try!(ProgressCodeDeserializer::deserialize(
                            "ProgressCode",
                            stack
                        )));
                    }
                    "ProgressMessage" => {
                        obj.progress_message = Some(try!(
                            ProgressMessageDeserializer::deserialize("ProgressMessage", stack)
                        ));
                    }
                    "Signature" => {
                        obj.signature =
                            Some(try!(SignatureDeserializer::deserialize("Signature", stack)));
                    }
                    "SignatureFileContents" => {
                        obj.signature_file_contents = Some(try!(
                            SignatureDeserializer::deserialize("SignatureFileContents", stack)
                        ));
                    }
                    "TrackingNumber" => {
                        obj.tracking_number = Some(try!(TrackingNumberDeserializer::deserialize(
                            "TrackingNumber",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
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
struct IsCanceledDeserializer;
impl IsCanceledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IsTruncatedDeserializer;
impl IsTruncatedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Representation of a job returned by the ListJobs operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Job {
    pub creation_date: Option<String>,
    pub is_canceled: Option<bool>,
    pub job_id: Option<String>,
    pub job_type: Option<String>,
}

struct JobDeserializer;
impl JobDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Job, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Job::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "CreationDate" => {
                        obj.creation_date = Some(try!(CreationDateDeserializer::deserialize(
                            "CreationDate",
                            stack
                        )));
                    }
                    "IsCanceled" => {
                        obj.is_canceled = Some(try!(IsCanceledDeserializer::deserialize(
                            "IsCanceled",
                            stack
                        )));
                    }
                    "JobId" => {
                        obj.job_id = Some(try!(JobIdDeserializer::deserialize("JobId", stack)));
                    }
                    "JobType" => {
                        obj.job_type =
                            Some(try!(JobTypeDeserializer::deserialize("JobType", stack)));
                    }
                    _ => skip_tree(stack),
                },
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
struct JobIdDeserializer;
impl JobIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// Serialize `JobIdList` contents to a `SignedRequest`.
struct JobIdListSerializer;
impl JobIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct JobTypeDeserializer;
impl JobTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct JobsListDeserializer;
impl JobsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Job>, XmlParseError> {
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
                    if name == "member" {
                        obj.push(try!(JobDeserializer::deserialize("member", stack)));
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
/// <p>Input structure for the ListJobs operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListJobsInput {
    pub api_version: Option<String>,
    pub marker: Option<String>,
    pub max_jobs: Option<i64>,
}

/// Serialize `ListJobsInput` contents to a `SignedRequest`.
struct ListJobsInputSerializer;
impl ListJobsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListJobsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(
                &format!("{}{}", prefix, "Marker"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_jobs {
            params.put(
                &format!("{}{}", prefix, "MaxJobs"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Output structure for the ListJobs operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListJobsOutput {
    pub is_truncated: Option<bool>,
    pub jobs: Option<Vec<Job>>,
}

struct ListJobsOutputDeserializer;
impl ListJobsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListJobsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListJobsOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "IsTruncated" => {
                        obj.is_truncated = Some(try!(IsTruncatedDeserializer::deserialize(
                            "IsTruncated",
                            stack
                        )));
                    }
                    "Jobs" => {
                        obj.jobs = Some(try!(JobsListDeserializer::deserialize("Jobs", stack)));
                    }
                    _ => skip_tree(stack),
                },
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
struct LocationCodeDeserializer;
impl LocationCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LocationMessageDeserializer;
impl LocationMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LogBucketDeserializer;
impl LogBucketDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct LogKeyDeserializer;
impl LogKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ProgressCodeDeserializer;
impl ProgressCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ProgressMessageDeserializer;
impl ProgressMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SignatureDeserializer;
impl SignatureDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SignatureFileContentsDeserializer;
impl SignatureFileContentsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct SuccessDeserializer;
impl SuccessDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bool, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct TrackingNumberDeserializer;
impl TrackingNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct URLDeserializer;
impl URLDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Input structure for the UpateJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateJobInput {
    pub api_version: Option<String>,
    pub job_id: String,
    pub job_type: String,
    pub manifest: String,
    pub validate_only: bool,
}

/// Serialize `UpdateJobInput` contents to a `SignedRequest`.
struct UpdateJobInputSerializer;
impl UpdateJobInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateJobInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.api_version {
            params.put(
                &format!("{}{}", prefix, "APIVersion"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "JobId"),
            &obj.job_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "JobType"),
            &obj.job_type.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Manifest"),
            &obj.manifest.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "ValidateOnly"),
            &obj.validate_only.to_string().replace("+", "%2B"),
        );
    }
}

/// <p>Output structure for the UpateJob operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateJobOutput {
    pub artifact_list: Option<Vec<Artifact>>,
    pub success: Option<bool>,
    pub warning_message: Option<String>,
}

struct UpdateJobOutputDeserializer;
impl UpdateJobOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateJobOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateJobOutput::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "ArtifactList" => {
                        obj.artifact_list = Some(try!(ArtifactListDeserializer::deserialize(
                            "ArtifactList",
                            stack
                        )));
                    }
                    "Success" => {
                        obj.success =
                            Some(try!(SuccessDeserializer::deserialize("Success", stack)));
                    }
                    "WarningMessage" => {
                        obj.warning_message = Some(try!(WarningMessageDeserializer::deserialize(
                            "WarningMessage",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
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
struct WarningMessageDeserializer;
impl WarningMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>AWS Import/Export cannot cancel the job</p>
    UnableToCancelJobId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelJobError {
    pub fn from_body(body: &str) -> CancelJobError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CanceledJobIdException" => {
                    CancelJobError::CanceledJobId(String::from(parsed_error.message))
                }
                "ExpiredJobIdException" => {
                    CancelJobError::ExpiredJobId(String::from(parsed_error.message))
                }
                "InvalidAccessKeyIdException" => {
                    CancelJobError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidJobIdException" => {
                    CancelJobError::InvalidJobId(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    CancelJobError::InvalidVersion(String::from(parsed_error.message))
                }
                "UnableToCancelJobIdException" => {
                    CancelJobError::UnableToCancelJobId(String::from(parsed_error.message))
                }
                _ => CancelJobError::Unknown(String::from(body)),
            },
            Err(_) => CancelJobError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CancelJobError {
    fn from(err: XmlParseError) -> CancelJobError {
        let XmlParseError(message) = err;
        CancelJobError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelJobError {
    fn from(err: io::Error) -> CancelJobError {
        CancelJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::CanceledJobId(ref cause) => cause,
            CancelJobError::ExpiredJobId(ref cause) => cause,
            CancelJobError::InvalidAccessKeyId(ref cause) => cause,
            CancelJobError::InvalidJobId(ref cause) => cause,
            CancelJobError::InvalidVersion(ref cause) => cause,
            CancelJobError::UnableToCancelJobId(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>The account specified does not have the appropriate bucket permissions.</p>
    BucketPermission(String),
    /// <p>Each account can create only a certain number of jobs per day. If you need to create more than this, please contact awsimportexport@amazon.com to explain your particular use case.</p>
    CreateJobQuotaExceeded(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>One or more customs parameters was invalid. Please correct and resubmit.</p>
    InvalidCustoms(String),
    /// <p>File system specified in export manifest is invalid.</p>
    InvalidFileSystem(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more manifest fields was invalid. Please correct and resubmit.</p>
    InvalidManifestField(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>Your manifest is not well-formed.</p>
    MalformedManifest(String),
    /// <p>One or more required customs parameters was missing from the manifest.</p>
    MissingCustoms(String),
    /// <p>One or more required fields were missing from the manifest file. Please correct and resubmit.</p>
    MissingManifestField(String),
    /// <p>One or more required parameters was missing from the request.</p>
    MissingParameter(String),
    /// <p>Your manifest file contained buckets from multiple regions. A job is restricted to buckets from one region. Please correct and resubmit.</p>
    MultipleRegions(String),
    /// <p>The specified bucket does not exist. Create the specified bucket or change the manifest&#39;s bucket, exportBucket, or logBucket field to a bucket that the account, as specified by the manifest&#39;s Access Key ID, has write permissions to.</p>
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

impl CreateJobError {
    pub fn from_body(body: &str) -> CreateJobError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BucketPermissionException" => {
                    CreateJobError::BucketPermission(String::from(parsed_error.message))
                }
                "CreateJobQuotaExceededException" => {
                    CreateJobError::CreateJobQuotaExceeded(String::from(parsed_error.message))
                }
                "InvalidAccessKeyIdException" => {
                    CreateJobError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidAddressException" => {
                    CreateJobError::InvalidAddress(String::from(parsed_error.message))
                }
                "InvalidCustomsException" => {
                    CreateJobError::InvalidCustoms(String::from(parsed_error.message))
                }
                "InvalidFileSystemException" => {
                    CreateJobError::InvalidFileSystem(String::from(parsed_error.message))
                }
                "InvalidJobIdException" => {
                    CreateJobError::InvalidJobId(String::from(parsed_error.message))
                }
                "InvalidManifestFieldException" => {
                    CreateJobError::InvalidManifestField(String::from(parsed_error.message))
                }
                "InvalidParameterException" => {
                    CreateJobError::InvalidParameter(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    CreateJobError::InvalidVersion(String::from(parsed_error.message))
                }
                "MalformedManifestException" => {
                    CreateJobError::MalformedManifest(String::from(parsed_error.message))
                }
                "MissingCustomsException" => {
                    CreateJobError::MissingCustoms(String::from(parsed_error.message))
                }
                "MissingManifestFieldException" => {
                    CreateJobError::MissingManifestField(String::from(parsed_error.message))
                }
                "MissingParameterException" => {
                    CreateJobError::MissingParameter(String::from(parsed_error.message))
                }
                "MultipleRegionsException" => {
                    CreateJobError::MultipleRegions(String::from(parsed_error.message))
                }
                "NoSuchBucketException" => {
                    CreateJobError::NoSuchBucket(String::from(parsed_error.message))
                }
                _ => CreateJobError::Unknown(String::from(body)),
            },
            Err(_) => CreateJobError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for CreateJobError {
    fn from(err: XmlParseError) -> CreateJobError {
        let XmlParseError(message) = err;
        CreateJobError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateJobError {
    fn from(err: CredentialsError) -> CreateJobError {
        CreateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobError {
    fn from(err: HttpDispatchError) -> CreateJobError {
        CreateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobError {
    fn from(err: io::Error) -> CreateJobError {
        CreateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::BucketPermission(ref cause) => cause,
            CreateJobError::CreateJobQuotaExceeded(ref cause) => cause,
            CreateJobError::InvalidAccessKeyId(ref cause) => cause,
            CreateJobError::InvalidAddress(ref cause) => cause,
            CreateJobError::InvalidCustoms(ref cause) => cause,
            CreateJobError::InvalidFileSystem(ref cause) => cause,
            CreateJobError::InvalidJobId(ref cause) => cause,
            CreateJobError::InvalidManifestField(ref cause) => cause,
            CreateJobError::InvalidParameter(ref cause) => cause,
            CreateJobError::InvalidVersion(ref cause) => cause,
            CreateJobError::MalformedManifest(ref cause) => cause,
            CreateJobError::MissingCustoms(ref cause) => cause,
            CreateJobError::MissingManifestField(ref cause) => cause,
            CreateJobError::MissingParameter(ref cause) => cause,
            CreateJobError::MultipleRegions(ref cause) => cause,
            CreateJobError::NoSuchBucket(ref cause) => cause,
            CreateJobError::Validation(ref cause) => cause,
            CreateJobError::Credentials(ref err) => err.description(),
            CreateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetShippingLabel
#[derive(Debug, PartialEq)]
pub enum GetShippingLabelError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetShippingLabelError {
    pub fn from_body(body: &str) -> GetShippingLabelError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CanceledJobIdException" => {
                    GetShippingLabelError::CanceledJobId(String::from(parsed_error.message))
                }
                "ExpiredJobIdException" => {
                    GetShippingLabelError::ExpiredJobId(String::from(parsed_error.message))
                }
                "InvalidAccessKeyIdException" => {
                    GetShippingLabelError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidAddressException" => {
                    GetShippingLabelError::InvalidAddress(String::from(parsed_error.message))
                }
                "InvalidJobIdException" => {
                    GetShippingLabelError::InvalidJobId(String::from(parsed_error.message))
                }
                "InvalidParameterException" => {
                    GetShippingLabelError::InvalidParameter(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    GetShippingLabelError::InvalidVersion(String::from(parsed_error.message))
                }
                _ => GetShippingLabelError::Unknown(String::from(body)),
            },
            Err(_) => GetShippingLabelError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetShippingLabelError {
    fn from(err: XmlParseError) -> GetShippingLabelError {
        let XmlParseError(message) = err;
        GetShippingLabelError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetShippingLabelError {
    fn from(err: CredentialsError) -> GetShippingLabelError {
        GetShippingLabelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetShippingLabelError {
    fn from(err: HttpDispatchError) -> GetShippingLabelError {
        GetShippingLabelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetShippingLabelError {
    fn from(err: io::Error) -> GetShippingLabelError {
        GetShippingLabelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetShippingLabelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetShippingLabelError {
    fn description(&self) -> &str {
        match *self {
            GetShippingLabelError::CanceledJobId(ref cause) => cause,
            GetShippingLabelError::ExpiredJobId(ref cause) => cause,
            GetShippingLabelError::InvalidAccessKeyId(ref cause) => cause,
            GetShippingLabelError::InvalidAddress(ref cause) => cause,
            GetShippingLabelError::InvalidJobId(ref cause) => cause,
            GetShippingLabelError::InvalidParameter(ref cause) => cause,
            GetShippingLabelError::InvalidVersion(ref cause) => cause,
            GetShippingLabelError::Validation(ref cause) => cause,
            GetShippingLabelError::Credentials(ref err) => err.description(),
            GetShippingLabelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetShippingLabelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStatus
#[derive(Debug, PartialEq)]
pub enum GetStatusError {
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetStatusError {
    pub fn from_body(body: &str) -> GetStatusError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CanceledJobIdException" => {
                    GetStatusError::CanceledJobId(String::from(parsed_error.message))
                }
                "ExpiredJobIdException" => {
                    GetStatusError::ExpiredJobId(String::from(parsed_error.message))
                }
                "InvalidAccessKeyIdException" => {
                    GetStatusError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidJobIdException" => {
                    GetStatusError::InvalidJobId(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    GetStatusError::InvalidVersion(String::from(parsed_error.message))
                }
                _ => GetStatusError::Unknown(String::from(body)),
            },
            Err(_) => GetStatusError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for GetStatusError {
    fn from(err: XmlParseError) -> GetStatusError {
        let XmlParseError(message) = err;
        GetStatusError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetStatusError {
    fn from(err: CredentialsError) -> GetStatusError {
        GetStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStatusError {
    fn from(err: HttpDispatchError) -> GetStatusError {
        GetStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStatusError {
    fn from(err: io::Error) -> GetStatusError {
        GetStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStatusError {
    fn description(&self) -> &str {
        match *self {
            GetStatusError::CanceledJobId(ref cause) => cause,
            GetStatusError::ExpiredJobId(ref cause) => cause,
            GetStatusError::InvalidAccessKeyId(ref cause) => cause,
            GetStatusError::InvalidJobId(ref cause) => cause,
            GetStatusError::InvalidVersion(ref cause) => cause,
            GetStatusError::Validation(ref cause) => cause,
            GetStatusError::Credentials(ref err) => err.description(),
            GetStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidAccessKeyIdException" => {
                    ListJobsError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidParameterException" => {
                    ListJobsError::InvalidParameter(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    ListJobsError::InvalidVersion(String::from(parsed_error.message))
                }
                _ => ListJobsError::Unknown(String::from(body)),
            },
            Err(_) => ListJobsError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for ListJobsError {
    fn from(err: XmlParseError) -> ListJobsError {
        let XmlParseError(message) = err;
        ListJobsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::InvalidAccessKeyId(ref cause) => cause,
            ListJobsError::InvalidParameter(ref cause) => cause,
            ListJobsError::InvalidVersion(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>The account specified does not have the appropriate bucket permissions.</p>
    BucketPermission(String),
    /// <p>The specified job ID has been canceled and is no longer valid.</p>
    CanceledJobId(String),
    /// <p>Indicates that the specified job has expired out of the system.</p>
    ExpiredJobId(String),
    /// <p>The AWS Access Key ID specified in the request did not match the manifest&#39;s accessKeyId value. The manifest and the request authentication must use the same AWS Access Key ID.</p>
    InvalidAccessKeyId(String),
    /// <p>The address specified in the manifest is invalid.</p>
    InvalidAddress(String),
    /// <p>One or more customs parameters was invalid. Please correct and resubmit.</p>
    InvalidCustoms(String),
    /// <p>File system specified in export manifest is invalid.</p>
    InvalidFileSystem(String),
    /// <p>The JOBID was missing, not found, or not associated with the AWS account.</p>
    InvalidJobId(String),
    /// <p>One or more manifest fields was invalid. Please correct and resubmit.</p>
    InvalidManifestField(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The client tool version is invalid.</p>
    InvalidVersion(String),
    /// <p>Your manifest is not well-formed.</p>
    MalformedManifest(String),
    /// <p>One or more required customs parameters was missing from the manifest.</p>
    MissingCustoms(String),
    /// <p>One or more required fields were missing from the manifest file. Please correct and resubmit.</p>
    MissingManifestField(String),
    /// <p>One or more required parameters was missing from the request.</p>
    MissingParameter(String),
    /// <p>Your manifest file contained buckets from multiple regions. A job is restricted to buckets from one region. Please correct and resubmit.</p>
    MultipleRegions(String),
    /// <p>The specified bucket does not exist. Create the specified bucket or change the manifest&#39;s bucket, exportBucket, or logBucket field to a bucket that the account, as specified by the manifest&#39;s Access Key ID, has write permissions to.</p>
    NoSuchBucket(String),
    /// <p>AWS Import/Export cannot update the job</p>
    UnableToUpdateJobId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateJobError {
    pub fn from_body(body: &str) -> UpdateJobError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "BucketPermissionException" => {
                    UpdateJobError::BucketPermission(String::from(parsed_error.message))
                }
                "CanceledJobIdException" => {
                    UpdateJobError::CanceledJobId(String::from(parsed_error.message))
                }
                "ExpiredJobIdException" => {
                    UpdateJobError::ExpiredJobId(String::from(parsed_error.message))
                }
                "InvalidAccessKeyIdException" => {
                    UpdateJobError::InvalidAccessKeyId(String::from(parsed_error.message))
                }
                "InvalidAddressException" => {
                    UpdateJobError::InvalidAddress(String::from(parsed_error.message))
                }
                "InvalidCustomsException" => {
                    UpdateJobError::InvalidCustoms(String::from(parsed_error.message))
                }
                "InvalidFileSystemException" => {
                    UpdateJobError::InvalidFileSystem(String::from(parsed_error.message))
                }
                "InvalidJobIdException" => {
                    UpdateJobError::InvalidJobId(String::from(parsed_error.message))
                }
                "InvalidManifestFieldException" => {
                    UpdateJobError::InvalidManifestField(String::from(parsed_error.message))
                }
                "InvalidParameterException" => {
                    UpdateJobError::InvalidParameter(String::from(parsed_error.message))
                }
                "InvalidVersionException" => {
                    UpdateJobError::InvalidVersion(String::from(parsed_error.message))
                }
                "MalformedManifestException" => {
                    UpdateJobError::MalformedManifest(String::from(parsed_error.message))
                }
                "MissingCustomsException" => {
                    UpdateJobError::MissingCustoms(String::from(parsed_error.message))
                }
                "MissingManifestFieldException" => {
                    UpdateJobError::MissingManifestField(String::from(parsed_error.message))
                }
                "MissingParameterException" => {
                    UpdateJobError::MissingParameter(String::from(parsed_error.message))
                }
                "MultipleRegionsException" => {
                    UpdateJobError::MultipleRegions(String::from(parsed_error.message))
                }
                "NoSuchBucketException" => {
                    UpdateJobError::NoSuchBucket(String::from(parsed_error.message))
                }
                "UnableToUpdateJobIdException" => {
                    UpdateJobError::UnableToUpdateJobId(String::from(parsed_error.message))
                }
                _ => UpdateJobError::Unknown(String::from(body)),
            },
            Err(_) => UpdateJobError::Unknown(body.to_string()),
        }
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}

impl From<XmlParseError> for UpdateJobError {
    fn from(err: XmlParseError) -> UpdateJobError {
        let XmlParseError(message) = err;
        UpdateJobError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateJobError {
    fn from(err: CredentialsError) -> UpdateJobError {
        UpdateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobError {
    fn from(err: HttpDispatchError) -> UpdateJobError {
        UpdateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateJobError {
    fn from(err: io::Error) -> UpdateJobError {
        UpdateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobError::BucketPermission(ref cause) => cause,
            UpdateJobError::CanceledJobId(ref cause) => cause,
            UpdateJobError::ExpiredJobId(ref cause) => cause,
            UpdateJobError::InvalidAccessKeyId(ref cause) => cause,
            UpdateJobError::InvalidAddress(ref cause) => cause,
            UpdateJobError::InvalidCustoms(ref cause) => cause,
            UpdateJobError::InvalidFileSystem(ref cause) => cause,
            UpdateJobError::InvalidJobId(ref cause) => cause,
            UpdateJobError::InvalidManifestField(ref cause) => cause,
            UpdateJobError::InvalidParameter(ref cause) => cause,
            UpdateJobError::InvalidVersion(ref cause) => cause,
            UpdateJobError::MalformedManifest(ref cause) => cause,
            UpdateJobError::MissingCustoms(ref cause) => cause,
            UpdateJobError::MissingManifestField(ref cause) => cause,
            UpdateJobError::MissingParameter(ref cause) => cause,
            UpdateJobError::MultipleRegions(ref cause) => cause,
            UpdateJobError::NoSuchBucket(ref cause) => cause,
            UpdateJobError::UnableToUpdateJobId(ref cause) => cause,
            UpdateJobError::Validation(ref cause) => cause,
            UpdateJobError::Credentials(ref err) => err.description(),
            UpdateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Import/Export API. AWS Import/Export clients implement this trait.
pub trait ImportExport {
    /// <p>This operation cancels a specified job. Only the job owner can cancel it. The operation fails if the job has already started or is complete.</p>
    fn cancel_job(&self, input: CancelJobInput) -> RusotoFuture<CancelJobOutput, CancelJobError>;

    /// <p>This operation initiates the process of scheduling an upload or download of your data. You include in the request a manifest that describes the data transfer specifics. The response to the request includes a job ID, which you can use in other operations, a signature that you use to identify your storage device, and the address where you should ship your storage device.</p>
    fn create_job(&self, input: CreateJobInput) -> RusotoFuture<CreateJobOutput, CreateJobError>;

    /// <p>This operation generates a pre-paid UPS shipping label that you will use to ship your device to AWS for processing.</p>
    fn get_shipping_label(
        &self,
        input: GetShippingLabelInput,
    ) -> RusotoFuture<GetShippingLabelOutput, GetShippingLabelError>;

    /// <p>This operation returns information about a job, including where the job is in the processing pipeline, the status of the results, and the signature value associated with the job. You can only return information about jobs you own.</p>
    fn get_status(&self, input: GetStatusInput) -> RusotoFuture<GetStatusOutput, GetStatusError>;

    /// <p>This operation returns the jobs associated with the requester. AWS Import/Export lists the jobs in reverse chronological order based on the date of creation. For example if Job Test1 was created 2009Dec30 and Test2 was created 2010Feb05, the ListJobs operation would return Test2 followed by Test1.</p>
    fn list_jobs(&self, input: ListJobsInput) -> RusotoFuture<ListJobsOutput, ListJobsError>;

    /// <p>You use this operation to change the parameters specified in the original manifest file by supplying a new manifest file. The manifest file attached to this request replaces the original manifest file. You can only use the operation after a CreateJob request but before the data transfer starts and you can only use it on jobs you own.</p>
    fn update_job(&self, input: UpdateJobInput) -> RusotoFuture<UpdateJobOutput, UpdateJobError>;
}
/// A client for the AWS Import/Export API.
pub struct ImportExportClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ImportExportClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ImportExportClient {
        ImportExportClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ImportExportClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ImportExportClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ImportExport for ImportExportClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>This operation cancels a specified job. Only the job owner can cancel it. The operation fails if the job has already started or is complete.</p>
    fn cancel_job(&self, input: CancelJobInput) -> RusotoFuture<CancelJobOutput, CancelJobError> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=CancelJob",
        );
        let mut params = Params::new();

        params.put("Action", "CancelJob");
        params.put("Version", "2010-06-01");
        CancelJobInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CancelJobOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CancelJobOutputDeserializer::deserialize(
                        "CancelJobResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation initiates the process of scheduling an upload or download of your data. You include in the request a manifest that describes the data transfer specifics. The response to the request includes a job ID, which you can use in other operations, a signature that you use to identify your storage device, and the address where you should ship your storage device.</p>
    fn create_job(&self, input: CreateJobInput) -> RusotoFuture<CreateJobOutput, CreateJobError> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=CreateJob",
        );
        let mut params = Params::new();

        params.put("Action", "CreateJob");
        params.put("Version", "2010-06-01");
        CreateJobInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateJobOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateJobOutputDeserializer::deserialize(
                        "CreateJobResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation generates a pre-paid UPS shipping label that you will use to ship your device to AWS for processing.</p>
    fn get_shipping_label(
        &self,
        input: GetShippingLabelInput,
    ) -> RusotoFuture<GetShippingLabelOutput, GetShippingLabelError> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=GetShippingLabel",
        );
        let mut params = Params::new();

        params.put("Action", "GetShippingLabel");
        params.put("Version", "2010-06-01");
        GetShippingLabelInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetShippingLabelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetShippingLabelOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetShippingLabelOutputDeserializer::deserialize(
                        "GetShippingLabelResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns information about a job, including where the job is in the processing pipeline, the status of the results, and the signature value associated with the job. You can only return information about jobs you own.</p>
    fn get_status(&self, input: GetStatusInput) -> RusotoFuture<GetStatusOutput, GetStatusError> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=GetStatus",
        );
        let mut params = Params::new();

        params.put("Action", "GetStatus");
        params.put("Version", "2010-06-01");
        GetStatusInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetStatusOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetStatusOutputDeserializer::deserialize(
                        "GetStatusResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>This operation returns the jobs associated with the requester. AWS Import/Export lists the jobs in reverse chronological order based on the date of creation. For example if Job Test1 was created 2009Dec30 and Test2 was created 2010Feb05, the ListJobs operation would return Test2 followed by Test1.</p>
    fn list_jobs(&self, input: ListJobsInput) -> RusotoFuture<ListJobsOutput, ListJobsError> {
        let mut request =
            SignedRequest::new("POST", "importexport", &self.region, "/?Operation=ListJobs");
        let mut params = Params::new();

        params.put("Action", "ListJobs");
        params.put("Version", "2010-06-01");
        ListJobsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListJobsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListJobsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListJobsOutputDeserializer::deserialize(
                        "ListJobsResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>You use this operation to change the parameters specified in the original manifest file by supplying a new manifest file. The manifest file attached to this request replaces the original manifest file. You can only use the operation after a CreateJob request but before the data transfer starts and you can only use it on jobs you own.</p>
    fn update_job(&self, input: UpdateJobInput) -> RusotoFuture<UpdateJobOutput, UpdateJobError> {
        let mut request = SignedRequest::new(
            "POST",
            "importexport",
            &self.region,
            "/?Operation=UpdateJob",
        );
        let mut params = Params::new();

        params.put("Action", "UpdateJob");
        params.put("Version", "2010-06-01");
        UpdateJobInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateJobOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateJobOutputDeserializer::deserialize(
                        "UpdateJobResult",
                        &mut stack
                    ));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_importexport_get_status() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "importexport-get-status.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client = ImportExportClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetStatusInput::default();
        let result = client.get_status(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_importexport_list_jobs() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "importexport-list-jobs.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = ImportExportClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListJobsInput::default();
        let result = client.list_jobs(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
