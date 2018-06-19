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
struct ARNDeserializer;
impl ARNDeserializer {
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
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AbortEnvironmentUpdateMessage {
    /// <p>This specifies the ID of the environment with the in-progress update that you want to cancel.</p>
    pub environment_id: Option<String>,
    /// <p>This specifies the name of the environment with the in-progress update that you want to cancel.</p>
    pub environment_name: Option<String>,
}

/// Serialize `AbortEnvironmentUpdateMessage` contents to a `SignedRequest`.
struct AbortEnvironmentUpdateMessageSerializer;
impl AbortEnvironmentUpdateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AbortEnvironmentUpdateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct AbortableOperationInProgressDeserializer;
impl AbortableOperationInProgressDeserializer {
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
struct ActionHistoryStatusDeserializer;
impl ActionHistoryStatusDeserializer {
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
struct ActionStatusDeserializer;
impl ActionStatusDeserializer {
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
struct ActionTypeDeserializer;
impl ActionTypeDeserializer {
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
/// <p>Describes the properties of an application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationDescription {
    /// <p>The name of the application.</p>
    pub application_name: Option<String>,
    /// <p>The names of the configuration templates associated with this application.</p>
    pub configuration_templates: Option<Vec<String>>,
    /// <p>The date when the application was created.</p>
    pub date_created: Option<String>,
    /// <p>The date when the application was last modified.</p>
    pub date_updated: Option<String>,
    /// <p>User-defined description of the application.</p>
    pub description: Option<String>,
    /// <p>The lifecycle settings for the application.</p>
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
    /// <p>The names of the versions for this application.</p>
    pub versions: Option<Vec<String>>,
}

struct ApplicationDescriptionDeserializer;
impl ApplicationDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationDescription::default();

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
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "ConfigurationTemplates" => {
                        obj.configuration_templates = Some(try!(
                            ConfigurationTemplateNamesListDeserializer::deserialize(
                                "ConfigurationTemplates",
                                stack
                            )
                        ));
                    }
                    "DateCreated" => {
                        obj.date_created = Some(try!(CreationDateDeserializer::deserialize(
                            "DateCreated",
                            stack
                        )));
                    }
                    "DateUpdated" => {
                        obj.date_updated = Some(try!(UpdateDateDeserializer::deserialize(
                            "DateUpdated",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "ResourceLifecycleConfig" => {
                        obj.resource_lifecycle_config = Some(try!(
                            ApplicationResourceLifecycleConfigDeserializer::deserialize(
                                "ResourceLifecycleConfig",
                                stack
                            )
                        ));
                    }
                    "Versions" => {
                        obj.versions = Some(try!(VersionLabelsListDeserializer::deserialize(
                            "Versions", stack
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
struct ApplicationDescriptionListDeserializer;
impl ApplicationDescriptionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ApplicationDescription>, XmlParseError> {
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
                        obj.push(try!(ApplicationDescriptionDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Result message containing a single description of an application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationDescriptionMessage {
    /// <p> The <a>ApplicationDescription</a> of the application. </p>
    pub application: Option<ApplicationDescription>,
}

struct ApplicationDescriptionMessageDeserializer;
impl ApplicationDescriptionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationDescriptionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationDescriptionMessage::default();

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
                    "Application" => {
                        obj.application = Some(try!(
                            ApplicationDescriptionDeserializer::deserialize("Application", stack)
                        ));
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
/// <p>Result message containing a list of application descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationDescriptionsMessage {
    /// <p>This parameter contains a list of <a>ApplicationDescription</a>.</p>
    pub applications: Option<Vec<ApplicationDescription>>,
}

struct ApplicationDescriptionsMessageDeserializer;
impl ApplicationDescriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationDescriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationDescriptionsMessage::default();

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
                    "Applications" => {
                        obj.applications =
                            Some(try!(ApplicationDescriptionListDeserializer::deserialize(
                                "Applications",
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
/// <p>Application request metrics for an AWS Elastic Beanstalk environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationMetrics {
    /// <p>The amount of time that the metrics cover (usually 10 seconds). For example, you might have 5 requests (<code>request_count</code>) within the most recent time slice of 10 seconds (<code>duration</code>).</p>
    pub duration: Option<i64>,
    /// <p>Represents the average latency for the slowest X percent of requests over the last 10 seconds. Latencies are in seconds with one millisecond resolution.</p>
    pub latency: Option<Latency>,
    /// <p>Average number of requests handled by the web server per second over the last 10 seconds.</p>
    pub request_count: Option<i64>,
    /// <p>Represents the percentage of requests over the last 10 seconds that resulted in each type of status code response.</p>
    pub status_codes: Option<StatusCodes>,
}

struct ApplicationMetricsDeserializer;
impl ApplicationMetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationMetrics, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationMetrics::default();

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
                    "Duration" => {
                        obj.duration = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Duration", stack
                        )));
                    }
                    "Latency" => {
                        obj.latency =
                            Some(try!(LatencyDeserializer::deserialize("Latency", stack)));
                    }
                    "RequestCount" => {
                        obj.request_count = Some(try!(RequestCountDeserializer::deserialize(
                            "RequestCount",
                            stack
                        )));
                    }
                    "StatusCodes" => {
                        obj.status_codes = Some(try!(StatusCodesDeserializer::deserialize(
                            "StatusCodes",
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
struct ApplicationNameDeserializer;
impl ApplicationNameDeserializer {
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

/// Serialize `ApplicationNamesList` contents to a `SignedRequest`.
struct ApplicationNamesListSerializer;
impl ApplicationNamesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>The resource lifecycle configuration for an application. Defines lifecycle settings for resources that belong to the application, and the service role that Elastic Beanstalk assumes in order to apply lifecycle settings. The version lifecycle configuration defines lifecycle settings for application versions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationResourceLifecycleConfig {
    /// <p>The ARN of an IAM service role that Elastic Beanstalk has permission to assume.</p>
    pub service_role: Option<String>,
    /// <p>The application version lifecycle configuration.</p>
    pub version_lifecycle_config: Option<ApplicationVersionLifecycleConfig>,
}

struct ApplicationResourceLifecycleConfigDeserializer;
impl ApplicationResourceLifecycleConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationResourceLifecycleConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationResourceLifecycleConfig::default();

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
                    "ServiceRole" => {
                        obj.service_role =
                            Some(try!(StringDeserializer::deserialize("ServiceRole", stack)));
                    }
                    "VersionLifecycleConfig" => {
                        obj.version_lifecycle_config = Some(try!(
                            ApplicationVersionLifecycleConfigDeserializer::deserialize(
                                "VersionLifecycleConfig",
                                stack
                            )
                        ));
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

/// Serialize `ApplicationResourceLifecycleConfig` contents to a `SignedRequest`.
struct ApplicationResourceLifecycleConfigSerializer;
impl ApplicationResourceLifecycleConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplicationResourceLifecycleConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.service_role {
            params.put(
                &format!("{}{}", prefix, "ServiceRole"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version_lifecycle_config {
            ApplicationVersionLifecycleConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VersionLifecycleConfig"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationResourceLifecycleDescriptionMessage {
    /// <p>The name of the application.</p>
    pub application_name: Option<String>,
    /// <p>The lifecycle configuration.</p>
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
}

struct ApplicationResourceLifecycleDescriptionMessageDeserializer;
impl ApplicationResourceLifecycleDescriptionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationResourceLifecycleDescriptionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationResourceLifecycleDescriptionMessage::default();

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
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "ResourceLifecycleConfig" => {
                        obj.resource_lifecycle_config = Some(try!(
                            ApplicationResourceLifecycleConfigDeserializer::deserialize(
                                "ResourceLifecycleConfig",
                                stack
                            )
                        ));
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
/// <p>Describes the properties of an application version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionDescription {
    /// <p>The name of the application to which the application version belongs.</p>
    pub application_name: Option<String>,
    /// <p>Reference to the artifact from the AWS CodeBuild build.</p>
    pub build_arn: Option<String>,
    /// <p>The creation date of the application version.</p>
    pub date_created: Option<String>,
    /// <p>The last modified date of the application version.</p>
    pub date_updated: Option<String>,
    /// <p>The description of the application version.</p>
    pub description: Option<String>,
    /// <p>If the version's source code was retrieved from AWS CodeCommit, the location of the source code for the application version.</p>
    pub source_build_information: Option<SourceBuildInformation>,
    /// <p>The storage location of the application version's source bundle in Amazon S3.</p>
    pub source_bundle: Option<S3Location>,
    /// <p>The processing status of the application version.</p>
    pub status: Option<String>,
    /// <p>A unique identifier for the application version.</p>
    pub version_label: Option<String>,
}

struct ApplicationVersionDescriptionDeserializer;
impl ApplicationVersionDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationVersionDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationVersionDescription::default();

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
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "BuildArn" => {
                        obj.build_arn =
                            Some(try!(StringDeserializer::deserialize("BuildArn", stack)));
                    }
                    "DateCreated" => {
                        obj.date_created = Some(try!(CreationDateDeserializer::deserialize(
                            "DateCreated",
                            stack
                        )));
                    }
                    "DateUpdated" => {
                        obj.date_updated = Some(try!(UpdateDateDeserializer::deserialize(
                            "DateUpdated",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "SourceBuildInformation" => {
                        obj.source_build_information =
                            Some(try!(SourceBuildInformationDeserializer::deserialize(
                                "SourceBuildInformation",
                                stack
                            )));
                    }
                    "SourceBundle" => {
                        obj.source_bundle = Some(try!(S3LocationDeserializer::deserialize(
                            "SourceBundle",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(ApplicationVersionStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "VersionLabel" => {
                        obj.version_label = Some(try!(VersionLabelDeserializer::deserialize(
                            "VersionLabel",
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
struct ApplicationVersionDescriptionListDeserializer;
impl ApplicationVersionDescriptionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ApplicationVersionDescription>, XmlParseError> {
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
                        obj.push(try!(
                            ApplicationVersionDescriptionDeserializer::deserialize("member", stack)
                        ));
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
/// <p>Result message wrapping a single description of an application version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionDescriptionMessage {
    /// <p> The <a>ApplicationVersionDescription</a> of the application version. </p>
    pub application_version: Option<ApplicationVersionDescription>,
}

struct ApplicationVersionDescriptionMessageDeserializer;
impl ApplicationVersionDescriptionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationVersionDescriptionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationVersionDescriptionMessage::default();

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
                    "ApplicationVersion" => {
                        obj.application_version = Some(try!(
                            ApplicationVersionDescriptionDeserializer::deserialize(
                                "ApplicationVersion",
                                stack
                            )
                        ));
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
/// <p>Result message wrapping a list of application version descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionDescriptionsMessage {
    /// <p>List of <code>ApplicationVersionDescription</code> objects sorted in order of creation.</p>
    pub application_versions: Option<Vec<ApplicationVersionDescription>>,
    /// <p>In a paginated request, the token that you can pass in a subsequent request to get the next response page.</p>
    pub next_token: Option<String>,
}

struct ApplicationVersionDescriptionsMessageDeserializer;
impl ApplicationVersionDescriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationVersionDescriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationVersionDescriptionsMessage::default();

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
                    "ApplicationVersions" => {
                        obj.application_versions = Some(try!(
                            ApplicationVersionDescriptionListDeserializer::deserialize(
                                "ApplicationVersions",
                                stack
                            )
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(TokenDeserializer::deserialize("NextToken", stack)));
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
/// <p>The application version lifecycle settings for an application. Defines the rules that Elastic Beanstalk applies to an application's versions in order to avoid hitting the per-region limit for application versions.</p> <p>When Elastic Beanstalk deletes an application version from its database, you can no longer deploy that version to an environment. The source bundle remains in S3 unless you configure the rule to delete it.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplicationVersionLifecycleConfig {
    /// <p>Specify a max age rule to restrict the length of time that application versions are retained for an application.</p>
    pub max_age_rule: Option<MaxAgeRule>,
    /// <p>Specify a max count rule to restrict the number of application versions that are retained for an application.</p>
    pub max_count_rule: Option<MaxCountRule>,
}

struct ApplicationVersionLifecycleConfigDeserializer;
impl ApplicationVersionLifecycleConfigDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplicationVersionLifecycleConfig, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplicationVersionLifecycleConfig::default();

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
                    "MaxAgeRule" => {
                        obj.max_age_rule = Some(try!(MaxAgeRuleDeserializer::deserialize(
                            "MaxAgeRule",
                            stack
                        )));
                    }
                    "MaxCountRule" => {
                        obj.max_count_rule = Some(try!(MaxCountRuleDeserializer::deserialize(
                            "MaxCountRule",
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

/// Serialize `ApplicationVersionLifecycleConfig` contents to a `SignedRequest`.
struct ApplicationVersionLifecycleConfigSerializer;
impl ApplicationVersionLifecycleConfigSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplicationVersionLifecycleConfig) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_age_rule {
            MaxAgeRuleSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MaxAgeRule"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_count_rule {
            MaxCountRuleSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MaxCountRule"),
                field_value,
            );
        }
    }
}

struct ApplicationVersionStatusDeserializer;
impl ApplicationVersionStatusDeserializer {
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
/// <p>Request to execute a scheduled managed action immediately.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplyEnvironmentManagedActionRequest {
    /// <p>The action ID of the scheduled managed action to execute.</p>
    pub action_id: String,
    /// <p>The environment ID of the target environment.</p>
    pub environment_id: Option<String>,
    /// <p>The name of the target environment.</p>
    pub environment_name: Option<String>,
}

/// Serialize `ApplyEnvironmentManagedActionRequest` contents to a `SignedRequest`.
struct ApplyEnvironmentManagedActionRequestSerializer;
impl ApplyEnvironmentManagedActionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ApplyEnvironmentManagedActionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ActionId"),
            &obj.action_id.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The result message containing information about the managed action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApplyEnvironmentManagedActionResult {
    /// <p>A description of the managed action.</p>
    pub action_description: Option<String>,
    /// <p>The action ID of the managed action.</p>
    pub action_id: Option<String>,
    /// <p>The type of managed action.</p>
    pub action_type: Option<String>,
    /// <p>The status of the managed action.</p>
    pub status: Option<String>,
}

struct ApplyEnvironmentManagedActionResultDeserializer;
impl ApplyEnvironmentManagedActionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ApplyEnvironmentManagedActionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ApplyEnvironmentManagedActionResult::default();

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
                    "ActionDescription" => {
                        obj.action_description = Some(try!(StringDeserializer::deserialize(
                            "ActionDescription",
                            stack
                        )));
                    }
                    "ActionId" => {
                        obj.action_id =
                            Some(try!(StringDeserializer::deserialize("ActionId", stack)));
                    }
                    "ActionType" => {
                        obj.action_type = Some(try!(ActionTypeDeserializer::deserialize(
                            "ActionType",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
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
/// <p>Describes an Auto Scaling launch configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AutoScalingGroup {
    /// <p>The name of the <code>AutoScalingGroup</code> . </p>
    pub name: Option<String>,
}

struct AutoScalingGroupDeserializer;
impl AutoScalingGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AutoScalingGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AutoScalingGroup::default();

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
                    "Name" => {
                        obj.name = Some(try!(ResourceIdDeserializer::deserialize("Name", stack)));
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
struct AutoScalingGroupListDeserializer;
impl AutoScalingGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AutoScalingGroup>, XmlParseError> {
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
                        obj.push(try!(AutoScalingGroupDeserializer::deserialize(
                            "member", stack
                        )));
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
struct AvailableSolutionStackDetailsListDeserializer;
impl AvailableSolutionStackDetailsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SolutionStackDescription>, XmlParseError> {
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
                        obj.push(try!(SolutionStackDescriptionDeserializer::deserialize(
                            "member", stack
                        )));
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
struct AvailableSolutionStackNamesListDeserializer;
impl AvailableSolutionStackNamesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(SolutionStackNameDeserializer::deserialize(
                            "member", stack
                        )));
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
struct BoxedBooleanDeserializer;
impl BoxedBooleanDeserializer {
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
struct BoxedIntDeserializer;
impl BoxedIntDeserializer {
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
/// <p>Settings for an AWS CodeBuild build.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BuildConfiguration {
    /// <p>The name of the artifact of the CodeBuild build. If provided, Elastic Beanstalk stores the build artifact in the S3 location <i>S3-bucket</i>/resources/<i>application-name</i>/codebuild/codebuild-<i>version-label</i>-<i>artifact-name</i>.zip. If not provided, Elastic Beanstalk stores the build artifact in the S3 location <i>S3-bucket</i>/resources/<i>application-name</i>/codebuild/codebuild-<i>version-label</i>.zip. </p>
    pub artifact_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>
    pub code_build_service_role: String,
    /// <p><p>Information about the compute resources the build project will use.</p> <ul> <li> <p> <code>BUILD<em>GENERAL1</em>SMALL: Use up to 3 GB memory and 2 vCPUs for builds</code> </p> </li> <li> <p> <code>BUILD<em>GENERAL1</em>MEDIUM: Use up to 7 GB memory and 4 vCPUs for builds</code> </p> </li> <li> <p> <code>BUILD<em>GENERAL1</em>LARGE: Use up to 15 GB memory and 8 vCPUs for builds</code> </p> </li> </ul></p>
    pub compute_type: Option<String>,
    /// <p>The ID of the Docker image to use for this build project.</p>
    pub image: String,
    /// <p>How long in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait until timing out any related build that does not get marked as completed. The default is 60 minutes.</p>
    pub timeout_in_minutes: Option<i64>,
}

/// Serialize `BuildConfiguration` contents to a `SignedRequest`.
struct BuildConfigurationSerializer;
impl BuildConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BuildConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.artifact_name {
            params.put(
                &format!("{}{}", prefix, "ArtifactName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "CodeBuildServiceRole"),
            &obj.code_build_service_role.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.compute_type {
            params.put(
                &format!("{}{}", prefix, "ComputeType"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Image"),
            &obj.image.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.timeout_in_minutes {
            params.put(
                &format!("{}{}", prefix, "TimeoutInMinutes"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>The builder used to build the custom platform.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Builder {
    /// <p>The ARN of the builder.</p>
    pub arn: Option<String>,
}

struct BuilderDeserializer;
impl BuilderDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Builder, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Builder::default();

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
                    "ARN" => {
                        obj.arn = Some(try!(ARNDeserializer::deserialize("ARN", stack)));
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
/// <p>CPU utilization metrics for an instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CPUUtilization {
    /// <p>Percentage of time that the CPU has spent in the <code>I/O Wait</code> state over the last 10 seconds.</p>
    pub io_wait: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>IRQ</code> state over the last 10 seconds.</p>
    pub irq: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>Idle</code> state over the last 10 seconds.</p>
    pub idle: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>Nice</code> state over the last 10 seconds.</p>
    pub nice: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>SoftIRQ</code> state over the last 10 seconds.</p>
    pub soft_irq: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>System</code> state over the last 10 seconds.</p>
    pub system: Option<f64>,
    /// <p>Percentage of time that the CPU has spent in the <code>User</code> state over the last 10 seconds.</p>
    pub user: Option<f64>,
}

struct CPUUtilizationDeserializer;
impl CPUUtilizationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CPUUtilization, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CPUUtilization::default();

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
                    "IOWait" => {
                        obj.io_wait = Some(try!(NullableDoubleDeserializer::deserialize(
                            "IOWait", stack
                        )));
                    }
                    "IRQ" => {
                        obj.irq = Some(try!(NullableDoubleDeserializer::deserialize("IRQ", stack)));
                    }
                    "Idle" => {
                        obj.idle =
                            Some(try!(NullableDoubleDeserializer::deserialize("Idle", stack)));
                    }
                    "Nice" => {
                        obj.nice =
                            Some(try!(NullableDoubleDeserializer::deserialize("Nice", stack)));
                    }
                    "SoftIRQ" => {
                        obj.soft_irq = Some(try!(NullableDoubleDeserializer::deserialize(
                            "SoftIRQ", stack
                        )));
                    }
                    "System" => {
                        obj.system = Some(try!(NullableDoubleDeserializer::deserialize(
                            "System", stack
                        )));
                    }
                    "User" => {
                        obj.user =
                            Some(try!(NullableDoubleDeserializer::deserialize("User", stack)));
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
struct CauseDeserializer;
impl CauseDeserializer {
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
struct CausesDeserializer;
impl CausesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(CauseDeserializer::deserialize("member", stack)));
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
/// <p>Results message indicating whether a CNAME is available.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CheckDNSAvailabilityMessage {
    /// <p>The prefix used when this CNAME is reserved.</p>
    pub cname_prefix: String,
}

/// Serialize `CheckDNSAvailabilityMessage` contents to a `SignedRequest`.
struct CheckDNSAvailabilityMessageSerializer;
impl CheckDNSAvailabilityMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CheckDNSAvailabilityMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CNAMEPrefix"),
            &obj.cname_prefix.replace("+", "%2B"),
        );
    }
}

/// <p>Indicates if the specified CNAME is available.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CheckDNSAvailabilityResultMessage {
    /// <p><p>Indicates if the specified CNAME is available:</p> <ul> <li> <p> <code>true</code> : The CNAME is available.</p> </li> <li> <p> <code>false</code> : The CNAME is not available.</p> </li> </ul></p>
    pub available: Option<bool>,
    /// <p>The fully qualified CNAME to reserve when <a>CreateEnvironment</a> is called with the provided prefix.</p>
    pub fully_qualified_cname: Option<String>,
}

struct CheckDNSAvailabilityResultMessageDeserializer;
impl CheckDNSAvailabilityResultMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CheckDNSAvailabilityResultMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CheckDNSAvailabilityResultMessage::default();

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
                    "Available" => {
                        obj.available = Some(try!(CnameAvailabilityDeserializer::deserialize(
                            "Available",
                            stack
                        )));
                    }
                    "FullyQualifiedCNAME" => {
                        obj.fully_qualified_cname = Some(try!(DNSCnameDeserializer::deserialize(
                            "FullyQualifiedCNAME",
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
struct CnameAvailabilityDeserializer;
impl CnameAvailabilityDeserializer {
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
/// <p>Request to create or update a group of environments.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ComposeEnvironmentsMessage {
    /// <p>The name of the application to which the specified source bundles belong.</p>
    pub application_name: Option<String>,
    /// <p>The name of the group to which the target environments belong. Specify a group name only if the environment name defined in each target environment's manifest ends with a + (plus) character. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest (env.yaml)</a> for details.</p>
    pub group_name: Option<String>,
    /// <p>A list of version labels, specifying one or more application source bundles that belong to the target application. Each source bundle must include an environment manifest that specifies the name of the environment and the name of the solution stack to use, and optionally can specify environment links to create.</p>
    pub version_labels: Option<Vec<String>>,
}

/// Serialize `ComposeEnvironmentsMessage` contents to a `SignedRequest`.
struct ComposeEnvironmentsMessageSerializer;
impl ComposeEnvironmentsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ComposeEnvironmentsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.group_name {
            params.put(
                &format!("{}{}", prefix, "GroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version_labels {
            VersionLabelsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VersionLabels"),
                field_value,
            );
        }
    }
}

struct ConfigurationDeploymentStatusDeserializer;
impl ConfigurationDeploymentStatusDeserializer {
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
struct ConfigurationOptionDefaultValueDeserializer;
impl ConfigurationOptionDefaultValueDeserializer {
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
/// <p>Describes the possible values for a configuration option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationOptionDescription {
    /// <p><p>An indication of which action is required if the value for this configuration option changes:</p> <ul> <li> <p> <code>NoInterruption</code> : There is no interruption to the environment or application availability.</p> </li> <li> <p> <code>RestartEnvironment</code> : The environment is entirely restarted, all AWS resources are deleted and recreated, and the environment is unavailable during the process.</p> </li> <li> <p> <code>RestartApplicationServer</code> : The environment is available the entire time. However, a short application outage occurs when the application servers on the running Amazon EC2 instances are restarted.</p> </li> </ul></p>
    pub change_severity: Option<String>,
    /// <p>The default value for this configuration option.</p>
    pub default_value: Option<String>,
    /// <p>If specified, the configuration option must be a string value no longer than this value.</p>
    pub max_length: Option<i64>,
    /// <p>If specified, the configuration option must be a numeric value less than this value.</p>
    pub max_value: Option<i64>,
    /// <p>If specified, the configuration option must be a numeric value greater than this value.</p>
    pub min_value: Option<i64>,
    /// <p>The name of the configuration option.</p>
    pub name: Option<String>,
    /// <p>A unique namespace identifying the option's associated AWS resource.</p>
    pub namespace: Option<String>,
    /// <p>If specified, the configuration option must be a string value that satisfies this regular expression.</p>
    pub regex: Option<OptionRestrictionRegex>,
    /// <p>An indication of whether the user defined this configuration option:</p> <ul> <li> <p> <code>true</code> : This configuration option was defined by the user. It is a valid choice for specifying if this as an <code>Option to Remove</code> when updating configuration settings. </p> </li> <li> <p> <code>false</code> : This configuration was not defined by the user.</p> </li> </ul> <p> Constraint: You can remove only <code>UserDefined</code> options from a configuration. </p> <p> Valid Values: <code>true</code> | <code>false</code> </p>
    pub user_defined: Option<bool>,
    /// <p>If specified, values for the configuration option are selected from this list.</p>
    pub value_options: Option<Vec<String>>,
    /// <p><p>An indication of which type of values this option has and whether it is allowable to select one or more than one of the possible values:</p> <ul> <li> <p> <code>Scalar</code> : Values for this option are a single selection from the possible values, or an unformatted string, or numeric value governed by the <code>MIN/MAX/Regex</code> constraints.</p> </li> <li> <p> <code>List</code> : Values for this option are multiple selections from the possible values.</p> </li> <li> <p> <code>Boolean</code> : Values for this option are either <code>true</code> or <code>false</code> .</p> </li> <li> <p> <code>Json</code> : Values for this option are a JSON representation of a <code>ConfigDocument</code>.</p> </li> </ul></p>
    pub value_type: Option<String>,
}

struct ConfigurationOptionDescriptionDeserializer;
impl ConfigurationOptionDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationOptionDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationOptionDescription::default();

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
                    "ChangeSeverity" => {
                        obj.change_severity =
                            Some(try!(ConfigurationOptionSeverityDeserializer::deserialize(
                                "ChangeSeverity",
                                stack
                            )));
                    }
                    "DefaultValue" => {
                        obj.default_value = Some(try!(
                            ConfigurationOptionDefaultValueDeserializer::deserialize(
                                "DefaultValue",
                                stack
                            )
                        ));
                    }
                    "MaxLength" => {
                        obj.max_length = Some(try!(
                            OptionRestrictionMaxLengthDeserializer::deserialize("MaxLength", stack)
                        ));
                    }
                    "MaxValue" => {
                        obj.max_value = Some(try!(
                            OptionRestrictionMaxValueDeserializer::deserialize("MaxValue", stack)
                        ));
                    }
                    "MinValue" => {
                        obj.min_value = Some(try!(
                            OptionRestrictionMinValueDeserializer::deserialize("MinValue", stack)
                        ));
                    }
                    "Name" => {
                        obj.name = Some(try!(ConfigurationOptionNameDeserializer::deserialize(
                            "Name", stack
                        )));
                    }
                    "Namespace" => {
                        obj.namespace = Some(try!(OptionNamespaceDeserializer::deserialize(
                            "Namespace",
                            stack
                        )));
                    }
                    "Regex" => {
                        obj.regex = Some(try!(OptionRestrictionRegexDeserializer::deserialize(
                            "Regex", stack
                        )));
                    }
                    "UserDefined" => {
                        obj.user_defined = Some(try!(UserDefinedOptionDeserializer::deserialize(
                            "UserDefined",
                            stack
                        )));
                    }
                    "ValueOptions" => {
                        obj.value_options = Some(try!(
                            ConfigurationOptionPossibleValuesDeserializer::deserialize(
                                "ValueOptions",
                                stack
                            )
                        ));
                    }
                    "ValueType" => {
                        obj.value_type =
                            Some(try!(ConfigurationOptionValueTypeDeserializer::deserialize(
                                "ValueType",
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
struct ConfigurationOptionDescriptionsListDeserializer;
impl ConfigurationOptionDescriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ConfigurationOptionDescription>, XmlParseError> {
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
                        obj.push(try!(
                            ConfigurationOptionDescriptionDeserializer::deserialize(
                                "member", stack
                            )
                        ));
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
struct ConfigurationOptionNameDeserializer;
impl ConfigurationOptionNameDeserializer {
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
struct ConfigurationOptionPossibleValueDeserializer;
impl ConfigurationOptionPossibleValueDeserializer {
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
struct ConfigurationOptionPossibleValuesDeserializer;
impl ConfigurationOptionPossibleValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(
                            ConfigurationOptionPossibleValueDeserializer::deserialize(
                                "member", stack
                            )
                        ));
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
/// <p> A specification identifying an individual configuration option along with its current value. For a list of possible option values, go to <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options.html">Option Values</a> in the <i>AWS Elastic Beanstalk Developer Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationOptionSetting {
    /// <p>A unique namespace identifying the option's associated AWS resource.</p>
    pub namespace: Option<String>,
    /// <p>The name of the configuration option.</p>
    pub option_name: Option<String>,
    /// <p>A unique resource name for a time-based scaling configuration option.</p>
    pub resource_name: Option<String>,
    /// <p>The current value for the configuration option.</p>
    pub value: Option<String>,
}

struct ConfigurationOptionSettingDeserializer;
impl ConfigurationOptionSettingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationOptionSetting, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationOptionSetting::default();

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
                    "Namespace" => {
                        obj.namespace = Some(try!(OptionNamespaceDeserializer::deserialize(
                            "Namespace",
                            stack
                        )));
                    }
                    "OptionName" => {
                        obj.option_name = Some(try!(
                            ConfigurationOptionNameDeserializer::deserialize("OptionName", stack)
                        ));
                    }
                    "ResourceName" => {
                        obj.resource_name = Some(try!(ResourceNameDeserializer::deserialize(
                            "ResourceName",
                            stack
                        )));
                    }
                    "Value" => {
                        obj.value = Some(try!(ConfigurationOptionValueDeserializer::deserialize(
                            "Value", stack
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

/// Serialize `ConfigurationOptionSetting` contents to a `SignedRequest`.
struct ConfigurationOptionSettingSerializer;
impl ConfigurationOptionSettingSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConfigurationOptionSetting) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.namespace {
            params.put(
                &format!("{}{}", prefix, "Namespace"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_name {
            params.put(
                &format!("{}{}", prefix, "OptionName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_name {
            params.put(
                &format!("{}{}", prefix, "ResourceName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct ConfigurationOptionSettingsListDeserializer;
impl ConfigurationOptionSettingsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ConfigurationOptionSetting>, XmlParseError> {
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
                        obj.push(try!(ConfigurationOptionSettingDeserializer::deserialize(
                            "member", stack
                        )));
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

/// Serialize `ConfigurationOptionSettingsList` contents to a `SignedRequest`.
struct ConfigurationOptionSettingsListSerializer;
impl ConfigurationOptionSettingsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ConfigurationOptionSetting>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ConfigurationOptionSettingSerializer::serialize(params, &key, obj);
        }
    }
}

struct ConfigurationOptionSeverityDeserializer;
impl ConfigurationOptionSeverityDeserializer {
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
struct ConfigurationOptionValueDeserializer;
impl ConfigurationOptionValueDeserializer {
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
struct ConfigurationOptionValueTypeDeserializer;
impl ConfigurationOptionValueTypeDeserializer {
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
/// <p>Describes the settings for a specified configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationOptionsDescription {
    /// <p> A list of <a>ConfigurationOptionDescription</a>. </p>
    pub options: Option<Vec<ConfigurationOptionDescription>>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The name of the solution stack these configuration options belong to.</p>
    pub solution_stack_name: Option<String>,
}

struct ConfigurationOptionsDescriptionDeserializer;
impl ConfigurationOptionsDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationOptionsDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationOptionsDescription::default();

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
                    "Options" => {
                        obj.options = Some(try!(
                            ConfigurationOptionDescriptionsListDeserializer::deserialize(
                                "Options", stack
                            )
                        ));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "SolutionStackName" => {
                        obj.solution_stack_name = Some(try!(
                            SolutionStackNameDeserializer::deserialize("SolutionStackName", stack)
                        ));
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
/// <p>Describes the settings for a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationSettingsDescription {
    /// <p>The name of the application associated with this configuration set.</p>
    pub application_name: Option<String>,
    /// <p>The date (in UTC time) when this configuration set was created.</p>
    pub date_created: Option<String>,
    /// <p>The date (in UTC time) when this configuration set was last modified.</p>
    pub date_updated: Option<String>,
    /// <p><p> If this configuration set is associated with an environment, the <code>DeploymentStatus</code> parameter indicates the deployment status of this configuration set: </p> <ul> <li> <p> <code>null</code>: This configuration is not associated with a running environment.</p> </li> <li> <p> <code>pending</code>: This is a draft configuration that is not deployed to the associated environment but is in the process of deploying.</p> </li> <li> <p> <code>deployed</code>: This is the configuration that is currently deployed to the associated running environment.</p> </li> <li> <p> <code>failed</code>: This is a draft configuration that failed to successfully deploy.</p> </li> </ul></p>
    pub deployment_status: Option<String>,
    /// <p>Describes this configuration set.</p>
    pub description: Option<String>,
    /// <p> If not <code>null</code>, the name of the environment for this configuration set. </p>
    pub environment_name: Option<String>,
    /// <p>A list of the configuration options and their values in this configuration set.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The name of the solution stack this configuration set uses.</p>
    pub solution_stack_name: Option<String>,
    /// <p> If not <code>null</code>, the name of the configuration template for this configuration set. </p>
    pub template_name: Option<String>,
}

struct ConfigurationSettingsDescriptionDeserializer;
impl ConfigurationSettingsDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationSettingsDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationSettingsDescription::default();

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
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "DateCreated" => {
                        obj.date_created = Some(try!(CreationDateDeserializer::deserialize(
                            "DateCreated",
                            stack
                        )));
                    }
                    "DateUpdated" => {
                        obj.date_updated = Some(try!(UpdateDateDeserializer::deserialize(
                            "DateUpdated",
                            stack
                        )));
                    }
                    "DeploymentStatus" => {
                        obj.deployment_status = Some(try!(
                            ConfigurationDeploymentStatusDeserializer::deserialize(
                                "DeploymentStatus",
                                stack
                            )
                        ));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(
                            EnvironmentNameDeserializer::deserialize("EnvironmentName", stack)
                        ));
                    }
                    "OptionSettings" => {
                        obj.option_settings = Some(try!(
                            ConfigurationOptionSettingsListDeserializer::deserialize(
                                "OptionSettings",
                                stack
                            )
                        ));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "SolutionStackName" => {
                        obj.solution_stack_name = Some(try!(
                            SolutionStackNameDeserializer::deserialize("SolutionStackName", stack)
                        ));
                    }
                    "TemplateName" => {
                        obj.template_name =
                            Some(try!(ConfigurationTemplateNameDeserializer::deserialize(
                                "TemplateName",
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
struct ConfigurationSettingsDescriptionListDeserializer;
impl ConfigurationSettingsDescriptionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ConfigurationSettingsDescription>, XmlParseError> {
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
                        obj.push(try!(
                            ConfigurationSettingsDescriptionDeserializer::deserialize(
                                "member", stack
                            )
                        ));
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
/// <p>The results from a request to change the configuration settings of an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationSettingsDescriptions {
    /// <p> A list of <a>ConfigurationSettingsDescription</a>. </p>
    pub configuration_settings: Option<Vec<ConfigurationSettingsDescription>>,
}

struct ConfigurationSettingsDescriptionsDeserializer;
impl ConfigurationSettingsDescriptionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationSettingsDescriptions, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationSettingsDescriptions::default();

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
                    "ConfigurationSettings" => {
                        obj.configuration_settings = Some(try!(
                            ConfigurationSettingsDescriptionListDeserializer::deserialize(
                                "ConfigurationSettings",
                                stack
                            )
                        ));
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
/// <p>Provides a list of validation messages.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigurationSettingsValidationMessages {
    /// <p> A list of <a>ValidationMessage</a>. </p>
    pub messages: Option<Vec<ValidationMessage>>,
}

struct ConfigurationSettingsValidationMessagesDeserializer;
impl ConfigurationSettingsValidationMessagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ConfigurationSettingsValidationMessages, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ConfigurationSettingsValidationMessages::default();

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
                    "Messages" => {
                        obj.messages = Some(try!(ValidationMessagesListDeserializer::deserialize(
                            "Messages", stack
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
struct ConfigurationTemplateNameDeserializer;
impl ConfigurationTemplateNameDeserializer {
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
struct ConfigurationTemplateNamesListDeserializer;
impl ConfigurationTemplateNamesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(ConfigurationTemplateNameDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Request to create an application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationMessage {
    /// <p>The name of the application.</p> <p>Constraint: This name must be unique within your account. If the specified name already exists, the action returns an <code>InvalidParameterValue</code> error.</p>
    pub application_name: String,
    /// <p>Describes the application.</p>
    pub description: Option<String>,
    /// <p>Specify an application resource lifecycle configuration to prevent your application from accumulating too many versions.</p>
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
}

/// Serialize `CreateApplicationMessage` contents to a `SignedRequest`.
struct CreateApplicationMessageSerializer;
impl CreateApplicationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateApplicationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_lifecycle_config {
            ApplicationResourceLifecycleConfigSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourceLifecycleConfig"),
                field_value,
            );
        }
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateApplicationVersionMessage {
    /// <p> The name of the application. If no application is found with this name, and <code>AutoCreateApplication</code> is <code>false</code>, returns an <code>InvalidParameterValue</code> error. </p>
    pub application_name: String,
    /// <p>Set to <code>true</code> to create an application with the specified name if it doesn't already exist.</p>
    pub auto_create_application: Option<bool>,
    /// <p>Settings for an AWS CodeBuild build.</p>
    pub build_configuration: Option<BuildConfiguration>,
    /// <p>Describes this version.</p>
    pub description: Option<String>,
    /// <p><p>Preprocesses and validates the environment manifest (<code>env.yaml</code>) and configuration files (<code>*.config</code> files in the <code>.ebextensions</code> folder) in the source bundle. Validating configuration files can identify issues prior to deploying the application version to an environment.</p> <note> <p>The <code>Process</code> option validates Elastic Beanstalk configuration files. It doesn&#39;t validate your application&#39;s configuration files, like proxy server or Docker configuration.</p> </note></p>
    pub process: Option<bool>,
    /// <p>Specify a commit in an AWS CodeCommit Git repository to use as the source code for the application version.</p>
    pub source_build_information: Option<SourceBuildInformation>,
    /// <p>The Amazon S3 bucket and key that identify the location of the source bundle for this version.</p> <note> <p>The Amazon S3 bucket must be in the same region as the environment.</p> </note> <p>Specify a source bundle in S3 or a commit in an AWS CodeCommit repository (with <code>SourceBuildInformation</code>), but not both. If neither <code>SourceBundle</code> nor <code>SourceBuildInformation</code> are provided, Elastic Beanstalk uses a sample application.</p>
    pub source_bundle: Option<S3Location>,
    /// <p>A label identifying this version.</p> <p>Constraint: Must be unique per application. If an application version already exists with this label for the specified application, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p>
    pub version_label: String,
}

/// Serialize `CreateApplicationVersionMessage` contents to a `SignedRequest`.
struct CreateApplicationVersionMessageSerializer;
impl CreateApplicationVersionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateApplicationVersionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.auto_create_application {
            params.put(
                &format!("{}{}", prefix, "AutoCreateApplication"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.build_configuration {
            BuildConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "BuildConfiguration"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.process {
            params.put(
                &format!("{}{}", prefix, "Process"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_build_information {
            SourceBuildInformationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceBuildInformation"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.source_bundle {
            S3LocationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceBundle"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "VersionLabel"),
            &obj.version_label.replace("+", "%2B"),
        );
    }
}

/// <p>Request to create a configuration template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateConfigurationTemplateMessage {
    /// <p>The name of the application to associate with this configuration template. If no application is found with this name, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p>
    pub application_name: String,
    /// <p>Describes this configuration.</p>
    pub description: Option<String>,
    /// <p>The ID of the environment used with this configuration template.</p>
    pub environment_id: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk sets the specified configuration option to the requested value. The new value overrides the value obtained from the solution stack or the source configuration template.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>The ARN of the custom platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The name of the solution stack used by this configuration. The solution stack specifies the operating system, architecture, and application server for a configuration template. It determines the set of configuration options as well as the possible and default values.</p> <p> Use <a>ListAvailableSolutionStacks</a> to obtain a list of available solution stacks. </p> <p> A solution stack name or a source configuration parameter must be specified, otherwise AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p> <p>If a solution stack name is not specified and the source configuration parameter is specified, AWS Elastic Beanstalk uses the same solution stack as the source configuration template.</p>
    pub solution_stack_name: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk uses the configuration values from the specified configuration template to create a new configuration.</p> <p> Values specified in the <code>OptionSettings</code> parameter of this call overrides any values obtained from the <code>SourceConfiguration</code>. </p> <p> If no configuration template is found, returns an <code>InvalidParameterValue</code> error. </p> <p> Constraint: If both the solution stack name parameter and the source configuration parameters are specified, the solution stack of the source configuration template must match the specified solution stack name or else AWS Elastic Beanstalk returns an <code>InvalidParameterCombination</code> error. </p>
    pub source_configuration: Option<SourceConfiguration>,
    /// <p>The name of the configuration template.</p> <p>Constraint: This name must be unique per application.</p> <p>Default: If a configuration template already exists with this name, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p>
    pub template_name: String,
}

/// Serialize `CreateConfigurationTemplateMessage` contents to a `SignedRequest`.
struct CreateConfigurationTemplateMessageSerializer;
impl CreateConfigurationTemplateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateConfigurationTemplateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_settings {
            ConfigurationOptionSettingsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionSettings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.solution_stack_name {
            params.put(
                &format!("{}{}", prefix, "SolutionStackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_configuration {
            SourceConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SourceConfiguration"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateEnvironmentMessage {
    /// <p>The name of the application that contains the version to be deployed.</p> <p> If no application is found with this name, <code>CreateEnvironment</code> returns an <code>InvalidParameterValue</code> error. </p>
    pub application_name: String,
    /// <p>If specified, the environment attempts to use this value as the prefix for the CNAME. If not specified, the CNAME is generated automatically by appending a random alphanumeric string to the environment name.</p>
    pub cname_prefix: Option<String>,
    /// <p>Describes this environment.</p>
    pub description: Option<String>,
    /// <p>A unique name for the deployment environment. Used in the application URL.</p> <p>Constraint: Must be from 4 to 40 characters in length. The name can contain only letters, numbers, and hyphens. It cannot start or end with a hyphen. This name must be unique within a region in your account. If the specified name already exists in the region, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p> <p>Default: If the CNAME parameter is not specified, the environment name becomes part of the CNAME, and therefore part of the visible URL for your application.</p>
    pub environment_name: Option<String>,
    /// <p>The name of the group to which the target environment belongs. Specify a group name only if the environment's name is specified in an environment manifest and not with the environment name parameter. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest (env.yaml)</a> for details.</p>
    pub group_name: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk sets the specified configuration options to the requested value in the configuration set for the new environment. These override the values obtained from the solution stack or the configuration template.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>A list of custom user-defined configuration options to remove from the configuration set for this new environment.</p>
    pub options_to_remove: Option<Vec<OptionSpecification>>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>This is an alternative to specifying a template name. If specified, AWS Elastic Beanstalk sets the configuration values to the default values associated with the specified solution stack.</p>
    pub solution_stack_name: Option<String>,
    /// <p>This specifies the tags applied to resources in the environment.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p> The name of the configuration template to use in deployment. If no configuration template is found with this name, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p>
    pub template_name: Option<String>,
    /// <p>This specifies the tier to use for creating this environment.</p>
    pub tier: Option<EnvironmentTier>,
    /// <p>The name of the application version to deploy.</p> <p> If the specified application has no associated application versions, AWS Elastic Beanstalk <code>UpdateEnvironment</code> returns an <code>InvalidParameterValue</code> error. </p> <p>Default: If not specified, AWS Elastic Beanstalk attempts to launch the sample application in the container.</p>
    pub version_label: Option<String>,
}

/// Serialize `CreateEnvironmentMessage` contents to a `SignedRequest`.
struct CreateEnvironmentMessageSerializer;
impl CreateEnvironmentMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateEnvironmentMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.cname_prefix {
            params.put(
                &format!("{}{}", prefix, "CNAMEPrefix"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.group_name {
            params.put(
                &format!("{}{}", prefix, "GroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_settings {
            ConfigurationOptionSettingsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionSettings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.options_to_remove {
            OptionsSpecifierListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionsToRemove"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.solution_stack_name {
            params.put(
                &format!("{}{}", prefix, "SolutionStackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tier {
            EnvironmentTierSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tier"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.version_label {
            params.put(
                &format!("{}{}", prefix, "VersionLabel"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Request to create a new platform version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreatePlatformVersionRequest {
    /// <p>The name of the builder environment.</p>
    pub environment_name: Option<String>,
    /// <p>The configuration option settings to apply to the builder environment.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>The location of the platform definition archive in Amazon S3.</p>
    pub platform_definition_bundle: S3Location,
    /// <p>The name of your custom platform.</p>
    pub platform_name: String,
    /// <p>The number, such as 1.0.2, for the new platform version.</p>
    pub platform_version: String,
}

/// Serialize `CreatePlatformVersionRequest` contents to a `SignedRequest`.
struct CreatePlatformVersionRequestSerializer;
impl CreatePlatformVersionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreatePlatformVersionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_settings {
            ConfigurationOptionSettingsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionSettings"),
                field_value,
            );
        }
        S3LocationSerializer::serialize(
            params,
            &format!("{}{}", prefix, "PlatformDefinitionBundle"),
            &obj.platform_definition_bundle,
        );
        params.put(
            &format!("{}{}", prefix, "PlatformName"),
            &obj.platform_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "PlatformVersion"),
            &obj.platform_version.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreatePlatformVersionResult {
    /// <p>The builder used to create the custom platform.</p>
    pub builder: Option<Builder>,
    /// <p>Detailed information about the new version of the custom platform.</p>
    pub platform_summary: Option<PlatformSummary>,
}

struct CreatePlatformVersionResultDeserializer;
impl CreatePlatformVersionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreatePlatformVersionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreatePlatformVersionResult::default();

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
                    "Builder" => {
                        obj.builder =
                            Some(try!(BuilderDeserializer::deserialize("Builder", stack)));
                    }
                    "PlatformSummary" => {
                        obj.platform_summary = Some(try!(
                            PlatformSummaryDeserializer::deserialize("PlatformSummary", stack)
                        ));
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
/// <p>Results of a <a>CreateStorageLocationResult</a> call.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStorageLocationResultMessage {
    /// <p>The name of the Amazon S3 bucket created.</p>
    pub s3_bucket: Option<String>,
}

struct CreateStorageLocationResultMessageDeserializer;
impl CreateStorageLocationResultMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStorageLocationResultMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStorageLocationResultMessage::default();

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
                    "S3Bucket" => {
                        obj.s3_bucket =
                            Some(try!(S3BucketDeserializer::deserialize("S3Bucket", stack)));
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
/// <p>A custom AMI available to platforms.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CustomAmi {
    /// <p>THe ID of the image used to create the custom AMI.</p>
    pub image_id: Option<String>,
    /// <p>The type of virtualization used to create the custom AMI.</p>
    pub virtualization_type: Option<String>,
}

struct CustomAmiDeserializer;
impl CustomAmiDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CustomAmi, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CustomAmi::default();

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
                    "ImageId" => {
                        obj.image_id =
                            Some(try!(ImageIdDeserializer::deserialize("ImageId", stack)));
                    }
                    "VirtualizationType" => {
                        obj.virtualization_type =
                            Some(try!(VirtualizationTypeDeserializer::deserialize(
                                "VirtualizationType",
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
struct CustomAmiListDeserializer;
impl CustomAmiListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CustomAmi>, XmlParseError> {
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
                        obj.push(try!(CustomAmiDeserializer::deserialize("member", stack)));
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
struct DNSCnameDeserializer;
impl DNSCnameDeserializer {
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
/// <p>Request to delete an application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteApplicationMessage {
    /// <p>The name of the application to delete.</p>
    pub application_name: String,
    /// <p>When set to true, running environments will be terminated before deleting the application.</p>
    pub terminate_env_by_force: Option<bool>,
}

/// Serialize `DeleteApplicationMessage` contents to a `SignedRequest`.
struct DeleteApplicationMessageSerializer;
impl DeleteApplicationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteApplicationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.terminate_env_by_force {
            params.put(
                &format!("{}{}", prefix, "TerminateEnvByForce"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>Request to delete an application version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteApplicationVersionMessage {
    /// <p>The name of the application to which the version belongs.</p>
    pub application_name: String,
    /// <p>Set to <code>true</code> to delete the source bundle from your storage bucket. Otherwise, the application version is deleted only from Elastic Beanstalk and the source bundle remains in Amazon S3.</p>
    pub delete_source_bundle: Option<bool>,
    /// <p>The label of the version to delete.</p>
    pub version_label: String,
}

/// Serialize `DeleteApplicationVersionMessage` contents to a `SignedRequest`.
struct DeleteApplicationVersionMessageSerializer;
impl DeleteApplicationVersionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteApplicationVersionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.delete_source_bundle {
            params.put(
                &format!("{}{}", prefix, "DeleteSourceBundle"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "VersionLabel"),
            &obj.version_label.replace("+", "%2B"),
        );
    }
}

/// <p>Request to delete a configuration template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteConfigurationTemplateMessage {
    /// <p>The name of the application to delete the configuration template from.</p>
    pub application_name: String,
    /// <p>The name of the configuration template to delete.</p>
    pub template_name: String,
}

/// Serialize `DeleteConfigurationTemplateMessage` contents to a `SignedRequest`.
struct DeleteConfigurationTemplateMessageSerializer;
impl DeleteConfigurationTemplateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteConfigurationTemplateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

/// <p>Request to delete a draft environment configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteEnvironmentConfigurationMessage {
    /// <p>The name of the application the environment is associated with.</p>
    pub application_name: String,
    /// <p>The name of the environment to delete the draft configuration from.</p>
    pub environment_name: String,
}

/// Serialize `DeleteEnvironmentConfigurationMessage` contents to a `SignedRequest`.
struct DeleteEnvironmentConfigurationMessageSerializer;
impl DeleteEnvironmentConfigurationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteEnvironmentConfigurationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "EnvironmentName"),
            &obj.environment_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePlatformVersionRequest {
    /// <p>The ARN of the version of the custom platform.</p>
    pub platform_arn: Option<String>,
}

/// Serialize `DeletePlatformVersionRequest` contents to a `SignedRequest`.
struct DeletePlatformVersionRequestSerializer;
impl DeletePlatformVersionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeletePlatformVersionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeletePlatformVersionResult {
    /// <p>Detailed information about the version of the custom platform.</p>
    pub platform_summary: Option<PlatformSummary>,
}

struct DeletePlatformVersionResultDeserializer;
impl DeletePlatformVersionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeletePlatformVersionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeletePlatformVersionResult::default();

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
                    "PlatformSummary" => {
                        obj.platform_summary = Some(try!(
                            PlatformSummaryDeserializer::deserialize("PlatformSummary", stack)
                        ));
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
/// <p>Information about an application version deployment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Deployment {
    /// <p>The ID of the deployment. This number increases by one each time that you deploy source code or change instance configuration settings.</p>
    pub deployment_id: Option<i64>,
    /// <p>For in-progress deployments, the time that the deployment started.</p> <p>For completed deployments, the time that the deployment ended.</p>
    pub deployment_time: Option<String>,
    /// <p><p>The status of the deployment:</p> <ul> <li> <p> <code>In Progress</code> : The deployment is in progress.</p> </li> <li> <p> <code>Deployed</code> : The deployment succeeded.</p> </li> <li> <p> <code>Failed</code> : The deployment failed.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The version label of the application version in the deployment.</p>
    pub version_label: Option<String>,
}

struct DeploymentDeserializer;
impl DeploymentDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Deployment, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Deployment::default();

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
                    "DeploymentId" => {
                        obj.deployment_id = Some(try!(NullableLongDeserializer::deserialize(
                            "DeploymentId",
                            stack
                        )));
                    }
                    "DeploymentTime" => {
                        obj.deployment_time = Some(try!(
                            DeploymentTimestampDeserializer::deserialize("DeploymentTime", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(StringDeserializer::deserialize("Status", stack)));
                    }
                    "VersionLabel" => {
                        obj.version_label =
                            Some(try!(StringDeserializer::deserialize("VersionLabel", stack)));
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
struct DeploymentTimestampDeserializer;
impl DeploymentTimestampDeserializer {
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
pub struct DescribeAccountAttributesResult {
    /// <p>The Elastic Beanstalk resource quotas associated with the calling AWS account.</p>
    pub resource_quotas: Option<ResourceQuotas>,
}

struct DescribeAccountAttributesResultDeserializer;
impl DescribeAccountAttributesResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountAttributesResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAccountAttributesResult::default();

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
                    "ResourceQuotas" => {
                        obj.resource_quotas = Some(try!(ResourceQuotasDeserializer::deserialize(
                            "ResourceQuotas",
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
/// <p>Request to describe application versions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeApplicationVersionsMessage {
    /// <p>Specify an application name to show only application versions for that application.</p>
    pub application_name: Option<String>,
    /// <p>For a paginated request. Specify a maximum number of application versions to include in each response.</p> <p>If no <code>MaxRecords</code> is specified, all available application versions are retrieved in a single response.</p>
    pub max_records: Option<i64>,
    /// <p>For a paginated request. Specify a token from a previous response page to retrieve the next response page. All other parameter values must be identical to the ones specified in the initial request.</p> <p>If no <code>NextToken</code> is specified, the first page is retrieved.</p>
    pub next_token: Option<String>,
    /// <p>Specify a version label to show a specific application version.</p>
    pub version_labels: Option<Vec<String>>,
}

/// Serialize `DescribeApplicationVersionsMessage` contents to a `SignedRequest`.
struct DescribeApplicationVersionsMessageSerializer;
impl DescribeApplicationVersionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeApplicationVersionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version_labels {
            VersionLabelsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "VersionLabels"),
                field_value,
            );
        }
    }
}

/// <p>Request to describe one or more applications.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeApplicationsMessage {
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to only include those with the specified names.</p>
    pub application_names: Option<Vec<String>>,
}

/// Serialize `DescribeApplicationsMessage` contents to a `SignedRequest`.
struct DescribeApplicationsMessageSerializer;
impl DescribeApplicationsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeApplicationsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_names {
            ApplicationNamesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ApplicationNames"),
                field_value,
            );
        }
    }
}

/// <p>Result message containing a list of application version descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeConfigurationOptionsMessage {
    /// <p>The name of the application associated with the configuration template or environment. Only needed if you want to describe the configuration options associated with either the configuration template or environment.</p>
    pub application_name: Option<String>,
    /// <p>The name of the environment whose configuration options you want to describe.</p>
    pub environment_name: Option<String>,
    /// <p>If specified, restricts the descriptions to only the specified options.</p>
    pub options: Option<Vec<OptionSpecification>>,
    /// <p>The ARN of the custom platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The name of the solution stack whose configuration options you want to describe.</p>
    pub solution_stack_name: Option<String>,
    /// <p>The name of the configuration template whose configuration options you want to describe.</p>
    pub template_name: Option<String>,
}

/// Serialize `DescribeConfigurationOptionsMessage` contents to a `SignedRequest`.
struct DescribeConfigurationOptionsMessageSerializer;
impl DescribeConfigurationOptionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeConfigurationOptionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.options {
            OptionsSpecifierListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Options"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.solution_stack_name {
            params.put(
                &format!("{}{}", prefix, "SolutionStackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Result message containing all of the configuration settings for a specified solution stack or configuration template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeConfigurationSettingsMessage {
    /// <p>The application for the environment or configuration template.</p>
    pub application_name: String,
    /// <p>The name of the environment to describe.</p> <p> Condition: You must specify either this or a TemplateName, but not both. If you specify both, AWS Elastic Beanstalk returns an <code>InvalidParameterCombination</code> error. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
    /// <p>The name of the configuration template to describe.</p> <p> Conditional: You must specify either this parameter or an EnvironmentName, but not both. If you specify both, AWS Elastic Beanstalk returns an <code>InvalidParameterCombination</code> error. If you do not specify either, AWS Elastic Beanstalk returns a <code>MissingRequiredParameter</code> error. </p>
    pub template_name: Option<String>,
}

/// Serialize `DescribeConfigurationSettingsMessage` contents to a `SignedRequest`.
struct DescribeConfigurationSettingsMessageSerializer;
impl DescribeConfigurationSettingsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeConfigurationSettingsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>See the example below to learn how to create a request body.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentHealthRequest {
    /// <p>Specify the response elements to return. To retrieve all attributes, set to <code>All</code>. If no attribute names are specified, returns the name of the environment.</p>
    pub attribute_names: Option<Vec<String>>,
    /// <p>Specify the environment by ID.</p> <p>You must specify either this or an EnvironmentName, or both.</p>
    pub environment_id: Option<String>,
    /// <p>Specify the environment by name.</p> <p>You must specify either this or an EnvironmentName, or both.</p>
    pub environment_name: Option<String>,
}

/// Serialize `DescribeEnvironmentHealthRequest` contents to a `SignedRequest`.
struct DescribeEnvironmentHealthRequestSerializer;
impl DescribeEnvironmentHealthRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEnvironmentHealthRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_names {
            EnvironmentHealthAttributesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Health details for an AWS Elastic Beanstalk environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentHealthResult {
    /// <p>Application request metrics for the environment.</p>
    pub application_metrics: Option<ApplicationMetrics>,
    /// <p>Descriptions of the data that contributed to the environment's current health status.</p>
    pub causes: Option<Vec<String>>,
    /// <p>The <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health color</a> of the environment.</p>
    pub color: Option<String>,
    /// <p>The environment's name.</p>
    pub environment_name: Option<String>,
    /// <p>The <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">health status</a> of the environment. For example, <code>Ok</code>.</p>
    pub health_status: Option<String>,
    /// <p>Summary health information for the instances in the environment.</p>
    pub instances_health: Option<InstanceHealthSummary>,
    /// <p>The date and time that the health information was retrieved.</p>
    pub refreshed_at: Option<String>,
    /// <p>The environment's operational status. <code>Ready</code>, <code>Launching</code>, <code>Updating</code>, <code>Terminating</code>, or <code>Terminated</code>.</p>
    pub status: Option<String>,
}

struct DescribeEnvironmentHealthResultDeserializer;
impl DescribeEnvironmentHealthResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEnvironmentHealthResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEnvironmentHealthResult::default();

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
                    "ApplicationMetrics" => {
                        obj.application_metrics =
                            Some(try!(ApplicationMetricsDeserializer::deserialize(
                                "ApplicationMetrics",
                                stack
                            )));
                    }
                    "Causes" => {
                        obj.causes = Some(try!(CausesDeserializer::deserialize("Causes", stack)));
                    }
                    "Color" => {
                        obj.color = Some(try!(StringDeserializer::deserialize("Color", stack)));
                    }
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(
                            EnvironmentNameDeserializer::deserialize("EnvironmentName", stack)
                        ));
                    }
                    "HealthStatus" => {
                        obj.health_status =
                            Some(try!(StringDeserializer::deserialize("HealthStatus", stack)));
                    }
                    "InstancesHealth" => {
                        obj.instances_health =
                            Some(try!(InstanceHealthSummaryDeserializer::deserialize(
                                "InstancesHealth",
                                stack
                            )));
                    }
                    "RefreshedAt" => {
                        obj.refreshed_at = Some(try!(RefreshedAtDeserializer::deserialize(
                            "RefreshedAt",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(EnvironmentHealthDeserializer::deserialize(
                            "Status", stack
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
/// <p>Request to list completed and failed managed actions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentManagedActionHistoryRequest {
    /// <p>The environment ID of the target environment.</p>
    pub environment_id: Option<String>,
    /// <p>The name of the target environment.</p>
    pub environment_name: Option<String>,
    /// <p>The maximum number of items to return for a single request.</p>
    pub max_items: Option<i64>,
    /// <p>The pagination token returned by a previous request.</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeEnvironmentManagedActionHistoryRequest` contents to a `SignedRequest`.
struct DescribeEnvironmentManagedActionHistoryRequestSerializer;
impl DescribeEnvironmentManagedActionHistoryRequestSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeEnvironmentManagedActionHistoryRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_items {
            params.put(
                &format!("{}{}", prefix, "MaxItems"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>A result message containing a list of completed and failed managed actions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentManagedActionHistoryResult {
    /// <p>A list of completed and failed managed actions.</p>
    pub managed_action_history_items: Option<Vec<ManagedActionHistoryItem>>,
    /// <p>A pagination token that you pass to <a>DescribeEnvironmentManagedActionHistory</a> to get the next page of results.</p>
    pub next_token: Option<String>,
}

struct DescribeEnvironmentManagedActionHistoryResultDeserializer;
impl DescribeEnvironmentManagedActionHistoryResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEnvironmentManagedActionHistoryResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEnvironmentManagedActionHistoryResult::default();

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
                    "ManagedActionHistoryItems" => {
                        obj.managed_action_history_items =
                            Some(try!(ManagedActionHistoryItemsDeserializer::deserialize(
                                "ManagedActionHistoryItems",
                                stack
                            )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(StringDeserializer::deserialize("NextToken", stack)));
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
/// <p>Request to list an environment's upcoming and in-progress managed actions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentManagedActionsRequest {
    /// <p>The environment ID of the target environment.</p>
    pub environment_id: Option<String>,
    /// <p>The name of the target environment.</p>
    pub environment_name: Option<String>,
    /// <p>To show only actions with a particular status, specify a status.</p>
    pub status: Option<String>,
}

/// Serialize `DescribeEnvironmentManagedActionsRequest` contents to a `SignedRequest`.
struct DescribeEnvironmentManagedActionsRequestSerializer;
impl DescribeEnvironmentManagedActionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEnvironmentManagedActionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.status {
            params.put(
                &format!("{}{}", prefix, "Status"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The result message containing a list of managed actions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentManagedActionsResult {
    /// <p>A list of upcoming and in-progress managed actions.</p>
    pub managed_actions: Option<Vec<ManagedAction>>,
}

struct DescribeEnvironmentManagedActionsResultDeserializer;
impl DescribeEnvironmentManagedActionsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeEnvironmentManagedActionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEnvironmentManagedActionsResult::default();

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
                    "ManagedActions" => {
                        obj.managed_actions = Some(try!(ManagedActionsDeserializer::deserialize(
                            "ManagedActions",
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
/// <p>Request to describe the resources in an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentResourcesMessage {
    /// <p>The ID of the environment to retrieve AWS resource usage data.</p> <p> Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment to retrieve AWS resource usage data.</p> <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
}

/// Serialize `DescribeEnvironmentResourcesMessage` contents to a `SignedRequest`.
struct DescribeEnvironmentResourcesMessageSerializer;
impl DescribeEnvironmentResourcesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEnvironmentResourcesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Request to describe one or more environments.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEnvironmentsMessage {
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to include only those that are associated with this application.</p>
    pub application_name: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to include only those that have the specified IDs.</p>
    pub environment_ids: Option<Vec<String>>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to include only those that have the specified names.</p>
    pub environment_names: Option<Vec<String>>,
    /// <p>Indicates whether to include deleted environments:</p> <p> <code>true</code>: Environments that have been deleted after <code>IncludedDeletedBackTo</code> are displayed.</p> <p> <code>false</code>: Do not include deleted environments.</p>
    pub include_deleted: Option<bool>,
    /// <p> If specified when <code>IncludeDeleted</code> is set to <code>true</code>, then environments deleted after this date are displayed. </p>
    pub included_deleted_back_to: Option<String>,
    /// <p>For a paginated request. Specify a maximum number of environments to include in each response.</p> <p>If no <code>MaxRecords</code> is specified, all available environments are retrieved in a single response.</p>
    pub max_records: Option<i64>,
    /// <p>For a paginated request. Specify a token from a previous response page to retrieve the next response page. All other parameter values must be identical to the ones specified in the initial request.</p> <p>If no <code>NextToken</code> is specified, the first page is retrieved.</p>
    pub next_token: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to include only those that are associated with this application version.</p>
    pub version_label: Option<String>,
}

/// Serialize `DescribeEnvironmentsMessage` contents to a `SignedRequest`.
struct DescribeEnvironmentsMessageSerializer;
impl DescribeEnvironmentsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEnvironmentsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_ids {
            EnvironmentIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnvironmentIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.environment_names {
            EnvironmentNamesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "EnvironmentNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.include_deleted {
            params.put(
                &format!("{}{}", prefix, "IncludeDeleted"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.included_deleted_back_to {
            params.put(
                &format!("{}{}", prefix, "IncludedDeletedBackTo"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version_label {
            params.put(
                &format!("{}{}", prefix, "VersionLabel"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Request to retrieve a list of events for an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeEventsMessage {
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to include only those associated with this application.</p>
    pub application_name: Option<String>,
    /// <p> If specified, AWS Elastic Beanstalk restricts the returned descriptions to those that occur up to, but not including, the <code>EndTime</code>. </p>
    pub end_time: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to those associated with this environment.</p>
    pub environment_id: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to those associated with this environment.</p>
    pub environment_name: Option<String>,
    /// <p>Specifies the maximum number of events that can be returned, beginning with the most recent event.</p>
    pub max_records: Option<i64>,
    /// <p>Pagination token. If specified, the events return the next batch of results.</p>
    pub next_token: Option<String>,
    /// <p>The ARN of the version of the custom platform.</p>
    pub platform_arn: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the described events to include only those associated with this request ID.</p>
    pub request_id: Option<String>,
    /// <p>If specified, limits the events returned from this call to include only those with the specified severity or higher.</p>
    pub severity: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to those that occur on or after this time.</p>
    pub start_time: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to those that are associated with this environment configuration.</p>
    pub template_name: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk restricts the returned descriptions to those associated with this application version.</p>
    pub version_label: Option<String>,
}

/// Serialize `DescribeEventsMessage` contents to a `SignedRequest`.
struct DescribeEventsMessageSerializer;
impl DescribeEventsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEventsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(
                &format!("{}{}", prefix, "EndTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.request_id {
            params.put(
                &format!("{}{}", prefix, "RequestId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.severity {
            params.put(
                &format!("{}{}", prefix, "Severity"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(
                &format!("{}{}", prefix, "StartTime"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version_label {
            params.put(
                &format!("{}{}", prefix, "VersionLabel"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Parameters for a call to <code>DescribeInstancesHealth</code>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeInstancesHealthRequest {
    /// <p>Specifies the response elements you wish to receive. To retrieve all attributes, set to <code>All</code>. If no attribute names are specified, returns a list of instances.</p>
    pub attribute_names: Option<Vec<String>>,
    /// <p>Specify the AWS Elastic Beanstalk environment by ID.</p>
    pub environment_id: Option<String>,
    /// <p>Specify the AWS Elastic Beanstalk environment by name.</p>
    pub environment_name: Option<String>,
    /// <p>Specify the pagination token returned by a previous call.</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeInstancesHealthRequest` contents to a `SignedRequest`.
struct DescribeInstancesHealthRequestSerializer;
impl DescribeInstancesHealthRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeInstancesHealthRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.attribute_names {
            InstancesHealthAttributesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AttributeNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Detailed health information about the Amazon EC2 instances in an AWS Elastic Beanstalk environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeInstancesHealthResult {
    /// <p>Detailed health information about each instance.</p>
    pub instance_health_list: Option<Vec<SingleInstanceHealth>>,
    /// <p>Pagination token for the next page of results, if available.</p>
    pub next_token: Option<String>,
    /// <p>The date and time that the health information was retrieved.</p>
    pub refreshed_at: Option<String>,
}

struct DescribeInstancesHealthResultDeserializer;
impl DescribeInstancesHealthResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeInstancesHealthResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeInstancesHealthResult::default();

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
                    "InstanceHealthList" => {
                        obj.instance_health_list =
                            Some(try!(InstanceHealthListDeserializer::deserialize(
                                "InstanceHealthList",
                                stack
                            )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "RefreshedAt" => {
                        obj.refreshed_at = Some(try!(RefreshedAtDeserializer::deserialize(
                            "RefreshedAt",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribePlatformVersionRequest {
    /// <p>The ARN of the version of the platform.</p>
    pub platform_arn: Option<String>,
}

/// Serialize `DescribePlatformVersionRequest` contents to a `SignedRequest`.
struct DescribePlatformVersionRequestSerializer;
impl DescribePlatformVersionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribePlatformVersionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribePlatformVersionResult {
    /// <p>Detailed information about the version of the platform.</p>
    pub platform_description: Option<PlatformDescription>,
}

struct DescribePlatformVersionResultDeserializer;
impl DescribePlatformVersionResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribePlatformVersionResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribePlatformVersionResult::default();

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
                    "PlatformDescription" => {
                        obj.platform_description =
                            Some(try!(PlatformDescriptionDeserializer::deserialize(
                                "PlatformDescription",
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
struct Ec2InstanceIdDeserializer;
impl Ec2InstanceIdDeserializer {
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
struct EndpointURLDeserializer;
impl EndpointURLDeserializer {
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
struct EnvironmentArnDeserializer;
impl EnvironmentArnDeserializer {
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
/// <p>Describes the properties of an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentDescription {
    /// <p>Indicates if there is an in-progress environment configuration update or application version deployment that you can cancel.</p> <p> <code>true:</code> There is an update in progress. </p> <p> <code>false:</code> There are no updates currently in progress. </p>
    pub abortable_operation_in_progress: Option<bool>,
    /// <p>The name of the application associated with this environment.</p>
    pub application_name: Option<String>,
    /// <p>The URL to the CNAME for this environment.</p>
    pub cname: Option<String>,
    /// <p>The creation date for this environment.</p>
    pub date_created: Option<String>,
    /// <p>The last modified date for this environment.</p>
    pub date_updated: Option<String>,
    /// <p>Describes this environment.</p>
    pub description: Option<String>,
    /// <p>For load-balanced, autoscaling environments, the URL to the LoadBalancer. For single-instance environments, the IP address of the instance.</p>
    pub endpoint_url: Option<String>,
    /// <p>The environment's Amazon Resource Name (ARN), which can be used in other API reuqests that require an ARN.</p>
    pub environment_arn: Option<String>,
    /// <p>The ID of this environment.</p>
    pub environment_id: Option<String>,
    /// <p>A list of links to other environments in the same group.</p>
    pub environment_links: Option<Vec<EnvironmentLink>>,
    /// <p>The name of this environment.</p>
    pub environment_name: Option<String>,
    /// <p>Describes the health status of the environment. AWS Elastic Beanstalk indicates the failure levels for a running environment:</p> <ul> <li> <p> <code>Red</code>: Indicates the environment is not responsive. Occurs when three or more consecutive failures occur for an environment.</p> </li> <li> <p> <code>Yellow</code>: Indicates that something is wrong. Occurs when two consecutive failures occur for an environment.</p> </li> <li> <p> <code>Green</code>: Indicates the environment is healthy and fully functional.</p> </li> <li> <p> <code>Grey</code>: Default health for a new environment. The environment is not fully launched and health checks have not started or health checks are suspended during an <code>UpdateEnvironment</code> or <code>RestartEnvironement</code> request.</p> </li> </ul> <p> Default: <code>Grey</code> </p>
    pub health: Option<String>,
    /// <p>Returns the health status of the application running in your environment. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">Health Colors and Statuses</a>.</p>
    pub health_status: Option<String>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The description of the AWS resources used by this environment.</p>
    pub resources: Option<EnvironmentResourcesDescription>,
    /// <p> The name of the <code>SolutionStack</code> deployed with this environment. </p>
    pub solution_stack_name: Option<String>,
    /// <p><p>The current operational status of the environment:</p> <ul> <li> <p> <code>Launching</code>: Environment is in the process of initial deployment.</p> </li> <li> <p> <code>Updating</code>: Environment is in the process of updating its configuration settings or application version.</p> </li> <li> <p> <code>Ready</code>: Environment is available to have an action performed on it, such as update or terminate.</p> </li> <li> <p> <code>Terminating</code>: Environment is in the shut-down process.</p> </li> <li> <p> <code>Terminated</code>: Environment is not running.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The name of the configuration template used to originally launch this environment.</p>
    pub template_name: Option<String>,
    /// <p>Describes the current tier of this environment.</p>
    pub tier: Option<EnvironmentTier>,
    /// <p>The application version deployed in this environment.</p>
    pub version_label: Option<String>,
}

struct EnvironmentDescriptionDeserializer;
impl EnvironmentDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentDescription::default();

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
                    "AbortableOperationInProgress" => {
                        obj.abortable_operation_in_progress =
                            Some(try!(AbortableOperationInProgressDeserializer::deserialize(
                                "AbortableOperationInProgress",
                                stack
                            )));
                    }
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "CNAME" => {
                        obj.cname = Some(try!(DNSCnameDeserializer::deserialize("CNAME", stack)));
                    }
                    "DateCreated" => {
                        obj.date_created = Some(try!(CreationDateDeserializer::deserialize(
                            "DateCreated",
                            stack
                        )));
                    }
                    "DateUpdated" => {
                        obj.date_updated = Some(try!(UpdateDateDeserializer::deserialize(
                            "DateUpdated",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "EndpointURL" => {
                        obj.endpoint_url = Some(try!(EndpointURLDeserializer::deserialize(
                            "EndpointURL",
                            stack
                        )));
                    }
                    "EnvironmentArn" => {
                        obj.environment_arn = Some(try!(EnvironmentArnDeserializer::deserialize(
                            "EnvironmentArn",
                            stack
                        )));
                    }
                    "EnvironmentId" => {
                        obj.environment_id = Some(try!(EnvironmentIdDeserializer::deserialize(
                            "EnvironmentId",
                            stack
                        )));
                    }
                    "EnvironmentLinks" => {
                        obj.environment_links = Some(try!(
                            EnvironmentLinksDeserializer::deserialize("EnvironmentLinks", stack)
                        ));
                    }
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(
                            EnvironmentNameDeserializer::deserialize("EnvironmentName", stack)
                        ));
                    }
                    "Health" => {
                        obj.health = Some(try!(EnvironmentHealthDeserializer::deserialize(
                            "Health", stack
                        )));
                    }
                    "HealthStatus" => {
                        obj.health_status = Some(try!(
                            EnvironmentHealthStatusDeserializer::deserialize("HealthStatus", stack)
                        ));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "Resources" => {
                        obj.resources = Some(try!(
                            EnvironmentResourcesDescriptionDeserializer::deserialize(
                                "Resources",
                                stack
                            )
                        ));
                    }
                    "SolutionStackName" => {
                        obj.solution_stack_name = Some(try!(
                            SolutionStackNameDeserializer::deserialize("SolutionStackName", stack)
                        ));
                    }
                    "Status" => {
                        obj.status = Some(try!(EnvironmentStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "TemplateName" => {
                        obj.template_name =
                            Some(try!(ConfigurationTemplateNameDeserializer::deserialize(
                                "TemplateName",
                                stack
                            )));
                    }
                    "Tier" => {
                        obj.tier = Some(try!(EnvironmentTierDeserializer::deserialize(
                            "Tier", stack
                        )));
                    }
                    "VersionLabel" => {
                        obj.version_label = Some(try!(VersionLabelDeserializer::deserialize(
                            "VersionLabel",
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
struct EnvironmentDescriptionsListDeserializer;
impl EnvironmentDescriptionsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EnvironmentDescription>, XmlParseError> {
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
                        obj.push(try!(EnvironmentDescriptionDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Result message containing a list of environment descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentDescriptionsMessage {
    /// <p> Returns an <a>EnvironmentDescription</a> list. </p>
    pub environments: Option<Vec<EnvironmentDescription>>,
    /// <p>In a paginated request, the token that you can pass in a subsequent request to get the next response page.</p>
    pub next_token: Option<String>,
}

struct EnvironmentDescriptionsMessageDeserializer;
impl EnvironmentDescriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentDescriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentDescriptionsMessage::default();

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
                    "Environments" => {
                        obj.environments =
                            Some(try!(EnvironmentDescriptionsListDeserializer::deserialize(
                                "Environments",
                                stack
                            )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(TokenDeserializer::deserialize("NextToken", stack)));
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
struct EnvironmentHealthDeserializer;
impl EnvironmentHealthDeserializer {
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

/// Serialize `EnvironmentHealthAttributes` contents to a `SignedRequest`.
struct EnvironmentHealthAttributesSerializer;
impl EnvironmentHealthAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct EnvironmentHealthStatusDeserializer;
impl EnvironmentHealthStatusDeserializer {
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
struct EnvironmentIdDeserializer;
impl EnvironmentIdDeserializer {
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

/// Serialize `EnvironmentIdList` contents to a `SignedRequest`.
struct EnvironmentIdListSerializer;
impl EnvironmentIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>The information retrieved from the Amazon EC2 instances.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentInfoDescription {
    /// <p>The Amazon EC2 Instance ID for this information.</p>
    pub ec_2_instance_id: Option<String>,
    /// <p>The type of information retrieved.</p>
    pub info_type: Option<String>,
    /// <p>The retrieved information.</p>
    pub message: Option<String>,
    /// <p>The time stamp when this information was retrieved.</p>
    pub sample_timestamp: Option<String>,
}

struct EnvironmentInfoDescriptionDeserializer;
impl EnvironmentInfoDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentInfoDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentInfoDescription::default();

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
                    "Ec2InstanceId" => {
                        obj.ec_2_instance_id = Some(try!(Ec2InstanceIdDeserializer::deserialize(
                            "Ec2InstanceId",
                            stack
                        )));
                    }
                    "InfoType" => {
                        obj.info_type = Some(try!(EnvironmentInfoTypeDeserializer::deserialize(
                            "InfoType", stack
                        )));
                    }
                    "Message" => {
                        obj.message =
                            Some(try!(MessageDeserializer::deserialize("Message", stack)));
                    }
                    "SampleTimestamp" => {
                        obj.sample_timestamp = Some(try!(
                            SampleTimestampDeserializer::deserialize("SampleTimestamp", stack)
                        ));
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
struct EnvironmentInfoDescriptionListDeserializer;
impl EnvironmentInfoDescriptionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EnvironmentInfoDescription>, XmlParseError> {
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
                        obj.push(try!(EnvironmentInfoDescriptionDeserializer::deserialize(
                            "member", stack
                        )));
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
struct EnvironmentInfoTypeDeserializer;
impl EnvironmentInfoTypeDeserializer {
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
/// <p>A link to another environment, defined in the environment's manifest. Links provide connection information in system properties that can be used to connect to another environment in the same group. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest (env.yaml)</a> for details.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentLink {
    /// <p>The name of the linked environment (the dependency).</p>
    pub environment_name: Option<String>,
    /// <p>The name of the link.</p>
    pub link_name: Option<String>,
}

struct EnvironmentLinkDeserializer;
impl EnvironmentLinkDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentLink, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentLink::default();

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
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(StringDeserializer::deserialize(
                            "EnvironmentName",
                            stack
                        )));
                    }
                    "LinkName" => {
                        obj.link_name =
                            Some(try!(StringDeserializer::deserialize("LinkName", stack)));
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
struct EnvironmentLinksDeserializer;
impl EnvironmentLinksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EnvironmentLink>, XmlParseError> {
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
                        obj.push(try!(EnvironmentLinkDeserializer::deserialize(
                            "member", stack
                        )));
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
struct EnvironmentNameDeserializer;
impl EnvironmentNameDeserializer {
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

/// Serialize `EnvironmentNamesList` contents to a `SignedRequest`.
struct EnvironmentNamesListSerializer;
impl EnvironmentNamesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes the AWS resources in use by this environment. This data is live.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentResourceDescription {
    /// <p> The <code>AutoScalingGroups</code> used by this environment. </p>
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    /// <p>The name of the environment.</p>
    pub environment_name: Option<String>,
    /// <p>The Amazon EC2 instances used by this environment.</p>
    pub instances: Option<Vec<Instance>>,
    /// <p>The Auto Scaling launch configurations in use by this environment.</p>
    pub launch_configurations: Option<Vec<LaunchConfiguration>>,
    /// <p>The LoadBalancers in use by this environment.</p>
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The queues used by this environment.</p>
    pub queues: Option<Vec<Queue>>,
    /// <p>The <code>AutoScaling</code> triggers in use by this environment. </p>
    pub triggers: Option<Vec<Trigger>>,
}

struct EnvironmentResourceDescriptionDeserializer;
impl EnvironmentResourceDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentResourceDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentResourceDescription::default();

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
                    "AutoScalingGroups" => {
                        obj.auto_scaling_groups =
                            Some(try!(AutoScalingGroupListDeserializer::deserialize(
                                "AutoScalingGroups",
                                stack
                            )));
                    }
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(
                            EnvironmentNameDeserializer::deserialize("EnvironmentName", stack)
                        ));
                    }
                    "Instances" => {
                        obj.instances = Some(try!(InstanceListDeserializer::deserialize(
                            "Instances",
                            stack
                        )));
                    }
                    "LaunchConfigurations" => {
                        obj.launch_configurations =
                            Some(try!(LaunchConfigurationListDeserializer::deserialize(
                                "LaunchConfigurations",
                                stack
                            )));
                    }
                    "LoadBalancers" => {
                        obj.load_balancers = Some(try!(LoadBalancerListDeserializer::deserialize(
                            "LoadBalancers",
                            stack
                        )));
                    }
                    "Queues" => {
                        obj.queues =
                            Some(try!(QueueListDeserializer::deserialize("Queues", stack)));
                    }
                    "Triggers" => {
                        obj.triggers = Some(try!(TriggerListDeserializer::deserialize(
                            "Triggers", stack
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
/// <p>Result message containing a list of environment resource descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentResourceDescriptionsMessage {
    /// <p> A list of <a>EnvironmentResourceDescription</a>. </p>
    pub environment_resources: Option<EnvironmentResourceDescription>,
}

struct EnvironmentResourceDescriptionsMessageDeserializer;
impl EnvironmentResourceDescriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentResourceDescriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentResourceDescriptionsMessage::default();

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
                    "EnvironmentResources" => {
                        obj.environment_resources = Some(try!(
                            EnvironmentResourceDescriptionDeserializer::deserialize(
                                "EnvironmentResources",
                                stack
                            )
                        ));
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
/// <p>Describes the AWS resources in use by this environment. This data is not live data.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentResourcesDescription {
    /// <p>Describes the LoadBalancer.</p>
    pub load_balancer: Option<LoadBalancerDescription>,
}

struct EnvironmentResourcesDescriptionDeserializer;
impl EnvironmentResourcesDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentResourcesDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentResourcesDescription::default();

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
                    "LoadBalancer" => {
                        obj.load_balancer = Some(try!(
                            LoadBalancerDescriptionDeserializer::deserialize("LoadBalancer", stack)
                        ));
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
struct EnvironmentStatusDeserializer;
impl EnvironmentStatusDeserializer {
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
/// <p>Describes the properties of an environment tier</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnvironmentTier {
    /// <p>The name of this environment tier.</p>
    pub name: Option<String>,
    /// <p>The type of this environment tier.</p>
    pub type_: Option<String>,
    /// <p>The version of this environment tier.</p>
    pub version: Option<String>,
}

struct EnvironmentTierDeserializer;
impl EnvironmentTierDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnvironmentTier, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EnvironmentTier::default();

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
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    "Type" => {
                        obj.type_ = Some(try!(StringDeserializer::deserialize("Type", stack)));
                    }
                    "Version" => {
                        obj.version = Some(try!(StringDeserializer::deserialize("Version", stack)));
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

/// Serialize `EnvironmentTier` contents to a `SignedRequest`.
struct EnvironmentTierSerializer;
impl EnvironmentTierSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EnvironmentTier) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.name {
            params.put(
                &format!("{}{}", prefix, "Name"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.type_ {
            params.put(
                &format!("{}{}", prefix, "Type"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.version {
            params.put(
                &format!("{}{}", prefix, "Version"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct EventDateDeserializer;
impl EventDateDeserializer {
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
/// <p>Describes an event.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventDescription {
    /// <p>The application associated with the event.</p>
    pub application_name: Option<String>,
    /// <p>The name of the environment associated with this event.</p>
    pub environment_name: Option<String>,
    /// <p>The date when the event occurred.</p>
    pub event_date: Option<String>,
    /// <p>The event message.</p>
    pub message: Option<String>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The web service request ID for the activity of this event.</p>
    pub request_id: Option<String>,
    /// <p>The severity level of this event.</p>
    pub severity: Option<String>,
    /// <p>The name of the configuration associated with this event.</p>
    pub template_name: Option<String>,
    /// <p>The release label for the application version associated with this event.</p>
    pub version_label: Option<String>,
}

struct EventDescriptionDeserializer;
impl EventDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventDescription::default();

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
                    "ApplicationName" => {
                        obj.application_name = Some(try!(
                            ApplicationNameDeserializer::deserialize("ApplicationName", stack)
                        ));
                    }
                    "EnvironmentName" => {
                        obj.environment_name = Some(try!(
                            EnvironmentNameDeserializer::deserialize("EnvironmentName", stack)
                        ));
                    }
                    "EventDate" => {
                        obj.event_date =
                            Some(try!(EventDateDeserializer::deserialize("EventDate", stack)));
                    }
                    "Message" => {
                        obj.message = Some(try!(EventMessageDeserializer::deserialize(
                            "Message", stack
                        )));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "RequestId" => {
                        obj.request_id =
                            Some(try!(RequestIdDeserializer::deserialize("RequestId", stack)));
                    }
                    "Severity" => {
                        obj.severity = Some(try!(EventSeverityDeserializer::deserialize(
                            "Severity", stack
                        )));
                    }
                    "TemplateName" => {
                        obj.template_name =
                            Some(try!(ConfigurationTemplateNameDeserializer::deserialize(
                                "TemplateName",
                                stack
                            )));
                    }
                    "VersionLabel" => {
                        obj.version_label = Some(try!(VersionLabelDeserializer::deserialize(
                            "VersionLabel",
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
struct EventDescriptionListDeserializer;
impl EventDescriptionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EventDescription>, XmlParseError> {
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
                        obj.push(try!(EventDescriptionDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Result message wrapping a list of event descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventDescriptionsMessage {
    /// <p> A list of <a>EventDescription</a>. </p>
    pub events: Option<Vec<EventDescription>>,
    /// <p> If returned, this indicates that there are more results to obtain. Use this token in the next <a>DescribeEvents</a> call to get the next batch of events. </p>
    pub next_token: Option<String>,
}

struct EventDescriptionsMessageDeserializer;
impl EventDescriptionsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EventDescriptionsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EventDescriptionsMessage::default();

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
                    "Events" => {
                        obj.events = Some(try!(EventDescriptionListDeserializer::deserialize(
                            "Events", stack
                        )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(TokenDeserializer::deserialize("NextToken", stack)));
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
struct EventMessageDeserializer;
impl EventMessageDeserializer {
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
struct EventSeverityDeserializer;
impl EventSeverityDeserializer {
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
struct FailureTypeDeserializer;
impl FailureTypeDeserializer {
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
struct FileTypeExtensionDeserializer;
impl FileTypeExtensionDeserializer {
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
struct ImageIdDeserializer;
impl ImageIdDeserializer {
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
/// <p>The description of an Amazon EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Instance {
    /// <p>The ID of the Amazon EC2 instance.</p>
    pub id: Option<String>,
}

struct InstanceDeserializer;
impl InstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Instance, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Instance::default();

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
                    "Id" => {
                        obj.id = Some(try!(ResourceIdDeserializer::deserialize("Id", stack)));
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
struct InstanceHealthListDeserializer;
impl InstanceHealthListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SingleInstanceHealth>, XmlParseError> {
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
                        obj.push(try!(SingleInstanceHealthDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Represents summary information about the health of an instance. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">Health Colors and Statuses</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InstanceHealthSummary {
    /// <p> <b>Red.</b> The health agent is reporting a high number of request failures or other issues for an instance or environment.</p>
    pub degraded: Option<i64>,
    /// <p> <b>Green.</b> An operation is in progress on an instance.</p>
    pub info: Option<i64>,
    /// <p> <b>Grey.</b> AWS Elastic Beanstalk and the health agent are reporting no data on an instance.</p>
    pub no_data: Option<i64>,
    /// <p> <b>Green.</b> An instance is passing health checks and the health agent is not reporting any problems.</p>
    pub ok: Option<i64>,
    /// <p> <b>Grey.</b> An operation is in progress on an instance within the command timeout.</p>
    pub pending: Option<i64>,
    /// <p> <b>Red.</b> The health agent is reporting a very high number of request failures or other issues for an instance or environment.</p>
    pub severe: Option<i64>,
    /// <p> <b>Grey.</b> AWS Elastic Beanstalk and the health agent are reporting an insufficient amount of data on an instance.</p>
    pub unknown: Option<i64>,
    /// <p> <b>Yellow.</b> The health agent is reporting a moderate number of request failures or other issues for an instance or environment.</p>
    pub warning: Option<i64>,
}

struct InstanceHealthSummaryDeserializer;
impl InstanceHealthSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<InstanceHealthSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = InstanceHealthSummary::default();

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
                    "Degraded" => {
                        obj.degraded = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Degraded", stack
                        )));
                    }
                    "Info" => {
                        obj.info = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Info", stack
                        )));
                    }
                    "NoData" => {
                        obj.no_data = Some(try!(NullableIntegerDeserializer::deserialize(
                            "NoData", stack
                        )));
                    }
                    "Ok" => {
                        obj.ok = Some(try!(NullableIntegerDeserializer::deserialize("Ok", stack)));
                    }
                    "Pending" => {
                        obj.pending = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Pending", stack
                        )));
                    }
                    "Severe" => {
                        obj.severe = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Severe", stack
                        )));
                    }
                    "Unknown" => {
                        obj.unknown = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Unknown", stack
                        )));
                    }
                    "Warning" => {
                        obj.warning = Some(try!(NullableIntegerDeserializer::deserialize(
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
struct InstanceIdDeserializer;
impl InstanceIdDeserializer {
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
struct InstanceListDeserializer;
impl InstanceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Instance>, XmlParseError> {
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
                        obj.push(try!(InstanceDeserializer::deserialize("member", stack)));
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

/// Serialize `InstancesHealthAttributes` contents to a `SignedRequest`.
struct InstancesHealthAttributesSerializer;
impl InstancesHealthAttributesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct IntegerDeserializer;
impl IntegerDeserializer {
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
/// <p>Represents the average latency for the slowest X percent of requests over the last 10 seconds.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Latency {
    /// <p>The average latency for the slowest 90 percent of requests over the last 10 seconds.</p>
    pub p10: Option<f64>,
    /// <p>The average latency for the slowest 50 percent of requests over the last 10 seconds.</p>
    pub p50: Option<f64>,
    /// <p>The average latency for the slowest 25 percent of requests over the last 10 seconds.</p>
    pub p75: Option<f64>,
    /// <p>The average latency for the slowest 15 percent of requests over the last 10 seconds.</p>
    pub p85: Option<f64>,
    /// <p>The average latency for the slowest 10 percent of requests over the last 10 seconds.</p>
    pub p90: Option<f64>,
    /// <p>The average latency for the slowest 5 percent of requests over the last 10 seconds.</p>
    pub p95: Option<f64>,
    /// <p>The average latency for the slowest 1 percent of requests over the last 10 seconds.</p>
    pub p99: Option<f64>,
    /// <p>The average latency for the slowest 0.1 percent of requests over the last 10 seconds.</p>
    pub p999: Option<f64>,
}

struct LatencyDeserializer;
impl LatencyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Latency, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Latency::default();

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
                    "P10" => {
                        obj.p10 = Some(try!(NullableDoubleDeserializer::deserialize("P10", stack)));
                    }
                    "P50" => {
                        obj.p50 = Some(try!(NullableDoubleDeserializer::deserialize("P50", stack)));
                    }
                    "P75" => {
                        obj.p75 = Some(try!(NullableDoubleDeserializer::deserialize("P75", stack)));
                    }
                    "P85" => {
                        obj.p85 = Some(try!(NullableDoubleDeserializer::deserialize("P85", stack)));
                    }
                    "P90" => {
                        obj.p90 = Some(try!(NullableDoubleDeserializer::deserialize("P90", stack)));
                    }
                    "P95" => {
                        obj.p95 = Some(try!(NullableDoubleDeserializer::deserialize("P95", stack)));
                    }
                    "P99" => {
                        obj.p99 = Some(try!(NullableDoubleDeserializer::deserialize("P99", stack)));
                    }
                    "P999" => {
                        obj.p999 =
                            Some(try!(NullableDoubleDeserializer::deserialize("P999", stack)));
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
/// <p>Describes an Auto Scaling launch configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LaunchConfiguration {
    /// <p>The name of the launch configuration.</p>
    pub name: Option<String>,
}

struct LaunchConfigurationDeserializer;
impl LaunchConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LaunchConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LaunchConfiguration::default();

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
                    "Name" => {
                        obj.name = Some(try!(ResourceIdDeserializer::deserialize("Name", stack)));
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
struct LaunchConfigurationListDeserializer;
impl LaunchConfigurationListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LaunchConfiguration>, XmlParseError> {
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
                        obj.push(try!(LaunchConfigurationDeserializer::deserialize(
                            "member", stack
                        )));
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
struct LaunchedAtDeserializer;
impl LaunchedAtDeserializer {
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
/// <p>A list of available AWS Elastic Beanstalk solution stacks.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListAvailableSolutionStacksResultMessage {
    /// <p> A list of available solution stacks and their <a>SolutionStackDescription</a>. </p>
    pub solution_stack_details: Option<Vec<SolutionStackDescription>>,
    /// <p>A list of available solution stacks.</p>
    pub solution_stacks: Option<Vec<String>>,
}

struct ListAvailableSolutionStacksResultMessageDeserializer;
impl ListAvailableSolutionStacksResultMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListAvailableSolutionStacksResultMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListAvailableSolutionStacksResultMessage::default();

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
                    "SolutionStackDetails" => {
                        obj.solution_stack_details = Some(try!(
                            AvailableSolutionStackDetailsListDeserializer::deserialize(
                                "SolutionStackDetails",
                                stack
                            )
                        ));
                    }
                    "SolutionStacks" => {
                        obj.solution_stacks = Some(try!(
                            AvailableSolutionStackNamesListDeserializer::deserialize(
                                "SolutionStacks",
                                stack
                            )
                        ));
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPlatformVersionsRequest {
    /// <p>List only the platforms where the platform member value relates to one of the supplied values.</p>
    pub filters: Option<Vec<PlatformFilter>>,
    /// <p>The maximum number of platform values returned in one call.</p>
    pub max_records: Option<i64>,
    /// <p>The starting index into the remaining list of platforms. Use the <code>NextToken</code> value from a previous <code>ListPlatformVersion</code> call.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListPlatformVersionsRequest` contents to a `SignedRequest`.
struct ListPlatformVersionsRequestSerializer;
impl ListPlatformVersionsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListPlatformVersionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            PlatformFiltersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(
                &format!("{}{}", prefix, "MaxRecords"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListPlatformVersionsResult {
    /// <p>The starting index into the remaining list of platforms. if this value is not <code>null</code>, you can use it in a subsequent <code>ListPlatformVersion</code> call. </p>
    pub next_token: Option<String>,
    /// <p>Detailed information about the platforms.</p>
    pub platform_summary_list: Option<Vec<PlatformSummary>>,
}

struct ListPlatformVersionsResultDeserializer;
impl ListPlatformVersionsResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListPlatformVersionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListPlatformVersionsResult::default();

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
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(TokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "PlatformSummaryList" => {
                        obj.platform_summary_list =
                            Some(try!(PlatformSummaryListDeserializer::deserialize(
                                "PlatformSummaryList",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the resouce for which a tag list is requested.</p> <p>Must be the ARN of an Elastic Beanstalk environment.</p>
    pub resource_arn: String,
}

/// Serialize `ListTagsForResourceMessage` contents to a `SignedRequest`.
struct ListTagsForResourceMessageSerializer;
impl ListTagsForResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListTagsForResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ResourceArn"),
            &obj.resource_arn.replace("+", "%2B"),
        );
    }
}

/// <p>Describes the properties of a Listener for the LoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Listener {
    /// <p>The port that is used by the Listener.</p>
    pub port: Option<i64>,
    /// <p>The protocol that is used by the Listener.</p>
    pub protocol: Option<String>,
}

struct ListenerDeserializer;
impl ListenerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Listener, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Listener::default();

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
                    "Port" => {
                        obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                    }
                    "Protocol" => {
                        obj.protocol =
                            Some(try!(StringDeserializer::deserialize("Protocol", stack)));
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
struct LoadAverageDeserializer;
impl LoadAverageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<f64>, XmlParseError> {
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
                        obj.push(try!(LoadAverageValueDeserializer::deserialize(
                            "member", stack
                        )));
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
struct LoadAverageValueDeserializer;
impl LoadAverageValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>Describes a LoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancer {
    /// <p>The name of the LoadBalancer.</p>
    pub name: Option<String>,
}

struct LoadBalancerDeserializer;
impl LoadBalancerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancer, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancer::default();

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
                    "Name" => {
                        obj.name = Some(try!(ResourceIdDeserializer::deserialize("Name", stack)));
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
/// <p>Describes the details of a LoadBalancer.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LoadBalancerDescription {
    /// <p>The domain name of the LoadBalancer.</p>
    pub domain: Option<String>,
    /// <p>A list of Listeners used by the LoadBalancer.</p>
    pub listeners: Option<Vec<Listener>>,
    /// <p>The name of the LoadBalancer.</p>
    pub load_balancer_name: Option<String>,
}

struct LoadBalancerDescriptionDeserializer;
impl LoadBalancerDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<LoadBalancerDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = LoadBalancerDescription::default();

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
                    "Domain" => {
                        obj.domain = Some(try!(StringDeserializer::deserialize("Domain", stack)));
                    }
                    "Listeners" => {
                        obj.listeners = Some(try!(
                            LoadBalancerListenersDescriptionDeserializer::deserialize(
                                "Listeners",
                                stack
                            )
                        ));
                    }
                    "LoadBalancerName" => {
                        obj.load_balancer_name = Some(try!(StringDeserializer::deserialize(
                            "LoadBalancerName",
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
struct LoadBalancerListDeserializer;
impl LoadBalancerListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<LoadBalancer>, XmlParseError> {
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
                        obj.push(try!(LoadBalancerDeserializer::deserialize("member", stack)));
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
struct LoadBalancerListenersDescriptionDeserializer;
impl LoadBalancerListenersDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Listener>, XmlParseError> {
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
                        obj.push(try!(ListenerDeserializer::deserialize("member", stack)));
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
struct MaintainerDeserializer;
impl MaintainerDeserializer {
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
/// <p>The record of an upcoming or in-progress managed action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ManagedAction {
    /// <p>A description of the managed action.</p>
    pub action_description: Option<String>,
    /// <p>A unique identifier for the managed action.</p>
    pub action_id: Option<String>,
    /// <p>The type of managed action.</p>
    pub action_type: Option<String>,
    /// <p>The status of the managed action. If the action is <code>Scheduled</code>, you can apply it immediately with <a>ApplyEnvironmentManagedAction</a>.</p>
    pub status: Option<String>,
    /// <p>The start time of the maintenance window in which the managed action will execute.</p>
    pub window_start_time: Option<String>,
}

struct ManagedActionDeserializer;
impl ManagedActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ManagedAction, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ManagedAction::default();

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
                    "ActionDescription" => {
                        obj.action_description = Some(try!(StringDeserializer::deserialize(
                            "ActionDescription",
                            stack
                        )));
                    }
                    "ActionId" => {
                        obj.action_id =
                            Some(try!(StringDeserializer::deserialize("ActionId", stack)));
                    }
                    "ActionType" => {
                        obj.action_type = Some(try!(ActionTypeDeserializer::deserialize(
                            "ActionType",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status =
                            Some(try!(ActionStatusDeserializer::deserialize("Status", stack)));
                    }
                    "WindowStartTime" => {
                        obj.window_start_time = Some(try!(TimestampDeserializer::deserialize(
                            "WindowStartTime",
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
/// <p>The record of a completed or failed managed action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ManagedActionHistoryItem {
    /// <p>A description of the managed action.</p>
    pub action_description: Option<String>,
    /// <p>A unique identifier for the managed action.</p>
    pub action_id: Option<String>,
    /// <p>The type of the managed action.</p>
    pub action_type: Option<String>,
    /// <p>The date and time that the action started executing.</p>
    pub executed_time: Option<String>,
    /// <p>If the action failed, a description of the failure.</p>
    pub failure_description: Option<String>,
    /// <p>If the action failed, the type of failure.</p>
    pub failure_type: Option<String>,
    /// <p>The date and time that the action finished executing.</p>
    pub finished_time: Option<String>,
    /// <p>The status of the action.</p>
    pub status: Option<String>,
}

struct ManagedActionHistoryItemDeserializer;
impl ManagedActionHistoryItemDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ManagedActionHistoryItem, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ManagedActionHistoryItem::default();

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
                    "ActionDescription" => {
                        obj.action_description = Some(try!(StringDeserializer::deserialize(
                            "ActionDescription",
                            stack
                        )));
                    }
                    "ActionId" => {
                        obj.action_id =
                            Some(try!(StringDeserializer::deserialize("ActionId", stack)));
                    }
                    "ActionType" => {
                        obj.action_type = Some(try!(ActionTypeDeserializer::deserialize(
                            "ActionType",
                            stack
                        )));
                    }
                    "ExecutedTime" => {
                        obj.executed_time = Some(try!(TimestampDeserializer::deserialize(
                            "ExecutedTime",
                            stack
                        )));
                    }
                    "FailureDescription" => {
                        obj.failure_description = Some(try!(StringDeserializer::deserialize(
                            "FailureDescription",
                            stack
                        )));
                    }
                    "FailureType" => {
                        obj.failure_type = Some(try!(FailureTypeDeserializer::deserialize(
                            "FailureType",
                            stack
                        )));
                    }
                    "FinishedTime" => {
                        obj.finished_time = Some(try!(TimestampDeserializer::deserialize(
                            "FinishedTime",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(ActionHistoryStatusDeserializer::deserialize(
                            "Status", stack
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
struct ManagedActionHistoryItemsDeserializer;
impl ManagedActionHistoryItemsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ManagedActionHistoryItem>, XmlParseError> {
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
                        obj.push(try!(ManagedActionHistoryItemDeserializer::deserialize(
                            "member", stack
                        )));
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
struct ManagedActionsDeserializer;
impl ManagedActionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ManagedAction>, XmlParseError> {
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
                        obj.push(try!(ManagedActionDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>A lifecycle rule that deletes application versions after the specified number of days.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MaxAgeRule {
    /// <p>Set to <code>true</code> to delete a version's source bundle from Amazon S3 when Elastic Beanstalk deletes the application version.</p>
    pub delete_source_from_s3: Option<bool>,
    /// <p>Specify <code>true</code> to apply the rule, or <code>false</code> to disable it.</p>
    pub enabled: bool,
    /// <p>Specify the number of days to retain an application versions.</p>
    pub max_age_in_days: Option<i64>,
}

struct MaxAgeRuleDeserializer;
impl MaxAgeRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MaxAgeRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MaxAgeRule::default();

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
                    "DeleteSourceFromS3" => {
                        obj.delete_source_from_s3 = Some(try!(
                            BoxedBooleanDeserializer::deserialize("DeleteSourceFromS3", stack)
                        ));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BoxedBooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "MaxAgeInDays" => {
                        obj.max_age_in_days = Some(try!(BoxedIntDeserializer::deserialize(
                            "MaxAgeInDays",
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

/// Serialize `MaxAgeRule` contents to a `SignedRequest`.
struct MaxAgeRuleSerializer;
impl MaxAgeRuleSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MaxAgeRule) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.delete_source_from_s3 {
            params.put(
                &format!("{}{}", prefix, "DeleteSourceFromS3"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.max_age_in_days {
            params.put(
                &format!("{}{}", prefix, "MaxAgeInDays"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>A lifecycle rule that deletes the oldest application version when the maximum count is exceeded.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MaxCountRule {
    /// <p>Set to <code>true</code> to delete a version's source bundle from Amazon S3 when Elastic Beanstalk deletes the application version.</p>
    pub delete_source_from_s3: Option<bool>,
    /// <p>Specify <code>true</code> to apply the rule, or <code>false</code> to disable it.</p>
    pub enabled: bool,
    /// <p>Specify the maximum number of application versions to retain.</p>
    pub max_count: Option<i64>,
}

struct MaxCountRuleDeserializer;
impl MaxCountRuleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MaxCountRule, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = MaxCountRule::default();

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
                    "DeleteSourceFromS3" => {
                        obj.delete_source_from_s3 = Some(try!(
                            BoxedBooleanDeserializer::deserialize("DeleteSourceFromS3", stack)
                        ));
                    }
                    "Enabled" => {
                        obj.enabled = try!(BoxedBooleanDeserializer::deserialize("Enabled", stack));
                    }
                    "MaxCount" => {
                        obj.max_count =
                            Some(try!(BoxedIntDeserializer::deserialize("MaxCount", stack)));
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

/// Serialize `MaxCountRule` contents to a `SignedRequest`.
struct MaxCountRuleSerializer;
impl MaxCountRuleSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &MaxCountRule) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.delete_source_from_s3 {
            params.put(
                &format!("{}{}", prefix, "DeleteSourceFromS3"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "Enabled"),
            &obj.enabled.to_string().replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.max_count {
            params.put(
                &format!("{}{}", prefix, "MaxCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

struct MessageDeserializer;
impl MessageDeserializer {
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
struct NextTokenDeserializer;
impl NextTokenDeserializer {
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
struct NullableDoubleDeserializer;
impl NullableDoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<f64, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct NullableIntegerDeserializer;
impl NullableIntegerDeserializer {
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
struct NullableLongDeserializer;
impl NullableLongDeserializer {
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
struct OperatingSystemNameDeserializer;
impl OperatingSystemNameDeserializer {
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
struct OperatingSystemVersionDeserializer;
impl OperatingSystemVersionDeserializer {
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
struct OptionNamespaceDeserializer;
impl OptionNamespaceDeserializer {
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
struct OptionRestrictionMaxLengthDeserializer;
impl OptionRestrictionMaxLengthDeserializer {
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
struct OptionRestrictionMaxValueDeserializer;
impl OptionRestrictionMaxValueDeserializer {
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
struct OptionRestrictionMinValueDeserializer;
impl OptionRestrictionMinValueDeserializer {
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
/// <p>A regular expression representing a restriction on a string configuration option value.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptionRestrictionRegex {
    /// <p>A unique name representing this regular expression.</p>
    pub label: Option<String>,
    /// <p>The regular expression pattern that a string configuration option value with this restriction must match.</p>
    pub pattern: Option<String>,
}

struct OptionRestrictionRegexDeserializer;
impl OptionRestrictionRegexDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<OptionRestrictionRegex, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = OptionRestrictionRegex::default();

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
                    "Label" => {
                        obj.label = Some(try!(RegexLabelDeserializer::deserialize("Label", stack)));
                    }
                    "Pattern" => {
                        obj.pattern = Some(try!(RegexPatternDeserializer::deserialize(
                            "Pattern", stack
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
/// <p>A specification identifying an individual configuration option.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OptionSpecification {
    /// <p>A unique namespace identifying the option's associated AWS resource.</p>
    pub namespace: Option<String>,
    /// <p>The name of the configuration option.</p>
    pub option_name: Option<String>,
    /// <p>A unique resource name for a time-based scaling configuration option.</p>
    pub resource_name: Option<String>,
}

/// Serialize `OptionSpecification` contents to a `SignedRequest`.
struct OptionSpecificationSerializer;
impl OptionSpecificationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &OptionSpecification) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.namespace {
            params.put(
                &format!("{}{}", prefix, "Namespace"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_name {
            params.put(
                &format!("{}{}", prefix, "OptionName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resource_name {
            params.put(
                &format!("{}{}", prefix, "ResourceName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// Serialize `OptionsSpecifierList` contents to a `SignedRequest`.
struct OptionsSpecifierListSerializer;
impl OptionsSpecifierListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<OptionSpecification>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            OptionSpecificationSerializer::serialize(params, &key, obj);
        }
    }
}

struct PlatformArnDeserializer;
impl PlatformArnDeserializer {
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
struct PlatformCategoryDeserializer;
impl PlatformCategoryDeserializer {
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
/// <p>Detailed information about a platform.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformDescription {
    /// <p>The custom AMIs supported by the platform.</p>
    pub custom_ami_list: Option<Vec<CustomAmi>>,
    /// <p>The date when the platform was created.</p>
    pub date_created: Option<String>,
    /// <p>The date when the platform was last updated.</p>
    pub date_updated: Option<String>,
    /// <p>The description of the platform.</p>
    pub description: Option<String>,
    /// <p>The frameworks supported by the platform.</p>
    pub frameworks: Option<Vec<PlatformFramework>>,
    /// <p>Information about the maintainer of the platform.</p>
    pub maintainer: Option<String>,
    /// <p>The operating system used by the platform.</p>
    pub operating_system_name: Option<String>,
    /// <p>The version of the operating system used by the platform.</p>
    pub operating_system_version: Option<String>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The category of the platform.</p>
    pub platform_category: Option<String>,
    /// <p>The name of the platform.</p>
    pub platform_name: Option<String>,
    /// <p>The AWS account ID of the person who created the platform.</p>
    pub platform_owner: Option<String>,
    /// <p>The status of the platform.</p>
    pub platform_status: Option<String>,
    /// <p>The version of the platform.</p>
    pub platform_version: Option<String>,
    /// <p>The programming languages supported by the platform.</p>
    pub programming_languages: Option<Vec<PlatformProgrammingLanguage>>,
    /// <p>The name of the solution stack used by the platform.</p>
    pub solution_stack_name: Option<String>,
    /// <p>The additions supported by the platform.</p>
    pub supported_addon_list: Option<Vec<String>>,
    /// <p>The tiers supported by the platform.</p>
    pub supported_tier_list: Option<Vec<String>>,
}

struct PlatformDescriptionDeserializer;
impl PlatformDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PlatformDescription::default();

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
                    "CustomAmiList" => {
                        obj.custom_ami_list = Some(try!(CustomAmiListDeserializer::deserialize(
                            "CustomAmiList",
                            stack
                        )));
                    }
                    "DateCreated" => {
                        obj.date_created = Some(try!(CreationDateDeserializer::deserialize(
                            "DateCreated",
                            stack
                        )));
                    }
                    "DateUpdated" => {
                        obj.date_updated = Some(try!(UpdateDateDeserializer::deserialize(
                            "DateUpdated",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "Frameworks" => {
                        obj.frameworks = Some(try!(PlatformFrameworksDeserializer::deserialize(
                            "Frameworks",
                            stack
                        )));
                    }
                    "Maintainer" => {
                        obj.maintainer = Some(try!(MaintainerDeserializer::deserialize(
                            "Maintainer",
                            stack
                        )));
                    }
                    "OperatingSystemName" => {
                        obj.operating_system_name =
                            Some(try!(OperatingSystemNameDeserializer::deserialize(
                                "OperatingSystemName",
                                stack
                            )));
                    }
                    "OperatingSystemVersion" => {
                        obj.operating_system_version =
                            Some(try!(OperatingSystemVersionDeserializer::deserialize(
                                "OperatingSystemVersion",
                                stack
                            )));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "PlatformCategory" => {
                        obj.platform_category = Some(try!(
                            PlatformCategoryDeserializer::deserialize("PlatformCategory", stack)
                        ));
                    }
                    "PlatformName" => {
                        obj.platform_name = Some(try!(PlatformNameDeserializer::deserialize(
                            "PlatformName",
                            stack
                        )));
                    }
                    "PlatformOwner" => {
                        obj.platform_owner = Some(try!(PlatformOwnerDeserializer::deserialize(
                            "PlatformOwner",
                            stack
                        )));
                    }
                    "PlatformStatus" => {
                        obj.platform_status = Some(try!(PlatformStatusDeserializer::deserialize(
                            "PlatformStatus",
                            stack
                        )));
                    }
                    "PlatformVersion" => {
                        obj.platform_version = Some(try!(
                            PlatformVersionDeserializer::deserialize("PlatformVersion", stack)
                        ));
                    }
                    "ProgrammingLanguages" => {
                        obj.programming_languages =
                            Some(try!(PlatformProgrammingLanguagesDeserializer::deserialize(
                                "ProgrammingLanguages",
                                stack
                            )));
                    }
                    "SolutionStackName" => {
                        obj.solution_stack_name = Some(try!(
                            SolutionStackNameDeserializer::deserialize("SolutionStackName", stack)
                        ));
                    }
                    "SupportedAddonList" => {
                        obj.supported_addon_list =
                            Some(try!(SupportedAddonListDeserializer::deserialize(
                                "SupportedAddonList",
                                stack
                            )));
                    }
                    "SupportedTierList" => {
                        obj.supported_tier_list = Some(try!(
                            SupportedTierListDeserializer::deserialize("SupportedTierList", stack)
                        ));
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
/// <p>Specify criteria to restrict the results when listing custom platforms.</p> <p>The filter is evaluated as the expression:</p> <p> <code>Type</code> <code>Operator</code> <code>Values[i]</code> </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformFilter {
    /// <p>The operator to apply to the <code>Type</code> with each of the <code>Values</code>.</p> <p> Valid Values: <code>=</code> (equal to) | <code>!=</code> (not equal to) | <code>&lt;</code> (less than) | <code>&lt;=</code> (less than or equal to) | <code>&gt;</code> (greater than) | <code>&gt;=</code> (greater than or equal to) | <code>contains</code> | <code>begins_with</code> | <code>ends_with</code> </p>
    pub operator: Option<String>,
    /// <p>The custom platform attribute to which the filter values are applied.</p> <p>Valid Values: <code>PlatformName</code> | <code>PlatformVersion</code> | <code>PlatformStatus</code> | <code>PlatformOwner</code> </p>
    pub type_: Option<String>,
    /// <p>The list of values applied to the custom platform attribute.</p>
    pub values: Option<Vec<String>>,
}

/// Serialize `PlatformFilter` contents to a `SignedRequest`.
struct PlatformFilterSerializer;
impl PlatformFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PlatformFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.operator {
            params.put(
                &format!("{}{}", prefix, "Operator"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.type_ {
            params.put(
                &format!("{}{}", prefix, "Type"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.values {
            PlatformFilterValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Values"),
                field_value,
            );
        }
    }
}

/// Serialize `PlatformFilterValueList` contents to a `SignedRequest`.
struct PlatformFilterValueListSerializer;
impl PlatformFilterValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `PlatformFilters` contents to a `SignedRequest`.
struct PlatformFiltersSerializer;
impl PlatformFiltersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<PlatformFilter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            PlatformFilterSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>A framework supported by the custom platform.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformFramework {
    /// <p>The name of the framework.</p>
    pub name: Option<String>,
    /// <p>The version of the framework.</p>
    pub version: Option<String>,
}

struct PlatformFrameworkDeserializer;
impl PlatformFrameworkDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformFramework, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PlatformFramework::default();

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
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    "Version" => {
                        obj.version = Some(try!(StringDeserializer::deserialize("Version", stack)));
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
struct PlatformFrameworksDeserializer;
impl PlatformFrameworksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PlatformFramework>, XmlParseError> {
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
                        obj.push(try!(PlatformFrameworkDeserializer::deserialize(
                            "member", stack
                        )));
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
struct PlatformNameDeserializer;
impl PlatformNameDeserializer {
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
struct PlatformOwnerDeserializer;
impl PlatformOwnerDeserializer {
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
/// <p>A programming language supported by the platform.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformProgrammingLanguage {
    /// <p>The name of the programming language.</p>
    pub name: Option<String>,
    /// <p>The version of the programming language.</p>
    pub version: Option<String>,
}

struct PlatformProgrammingLanguageDeserializer;
impl PlatformProgrammingLanguageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformProgrammingLanguage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PlatformProgrammingLanguage::default();

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
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    "Version" => {
                        obj.version = Some(try!(StringDeserializer::deserialize("Version", stack)));
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
struct PlatformProgrammingLanguagesDeserializer;
impl PlatformProgrammingLanguagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PlatformProgrammingLanguage>, XmlParseError> {
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
                        obj.push(try!(PlatformProgrammingLanguageDeserializer::deserialize(
                            "member", stack
                        )));
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
struct PlatformStatusDeserializer;
impl PlatformStatusDeserializer {
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
/// <p>Detailed information about a platform.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PlatformSummary {
    /// <p>The operating system used by the platform.</p>
    pub operating_system_name: Option<String>,
    /// <p>The version of the operating system used by the platform.</p>
    pub operating_system_version: Option<String>,
    /// <p>The ARN of the platform.</p>
    pub platform_arn: Option<String>,
    /// <p>The category of platform.</p>
    pub platform_category: Option<String>,
    /// <p>The AWS account ID of the person who created the platform.</p>
    pub platform_owner: Option<String>,
    /// <p>The status of the platform. You can create an environment from the platform once it is ready.</p>
    pub platform_status: Option<String>,
    /// <p>The additions associated with the platform.</p>
    pub supported_addon_list: Option<Vec<String>>,
    /// <p>The tiers in which the platform runs.</p>
    pub supported_tier_list: Option<Vec<String>>,
}

struct PlatformSummaryDeserializer;
impl PlatformSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PlatformSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PlatformSummary::default();

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
                    "OperatingSystemName" => {
                        obj.operating_system_name =
                            Some(try!(OperatingSystemNameDeserializer::deserialize(
                                "OperatingSystemName",
                                stack
                            )));
                    }
                    "OperatingSystemVersion" => {
                        obj.operating_system_version =
                            Some(try!(OperatingSystemVersionDeserializer::deserialize(
                                "OperatingSystemVersion",
                                stack
                            )));
                    }
                    "PlatformArn" => {
                        obj.platform_arn = Some(try!(PlatformArnDeserializer::deserialize(
                            "PlatformArn",
                            stack
                        )));
                    }
                    "PlatformCategory" => {
                        obj.platform_category = Some(try!(
                            PlatformCategoryDeserializer::deserialize("PlatformCategory", stack)
                        ));
                    }
                    "PlatformOwner" => {
                        obj.platform_owner = Some(try!(PlatformOwnerDeserializer::deserialize(
                            "PlatformOwner",
                            stack
                        )));
                    }
                    "PlatformStatus" => {
                        obj.platform_status = Some(try!(PlatformStatusDeserializer::deserialize(
                            "PlatformStatus",
                            stack
                        )));
                    }
                    "SupportedAddonList" => {
                        obj.supported_addon_list =
                            Some(try!(SupportedAddonListDeserializer::deserialize(
                                "SupportedAddonList",
                                stack
                            )));
                    }
                    "SupportedTierList" => {
                        obj.supported_tier_list = Some(try!(
                            SupportedTierListDeserializer::deserialize("SupportedTierList", stack)
                        ));
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
struct PlatformSummaryListDeserializer;
impl PlatformSummaryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PlatformSummary>, XmlParseError> {
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
                        obj.push(try!(PlatformSummaryDeserializer::deserialize(
                            "member", stack
                        )));
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
struct PlatformVersionDeserializer;
impl PlatformVersionDeserializer {
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
/// <p>Describes a queue.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Queue {
    /// <p>The name of the queue.</p>
    pub name: Option<String>,
    /// <p>The URL of the queue.</p>
    pub url: Option<String>,
}

struct QueueDeserializer;
impl QueueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Queue, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Queue::default();

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
                    "Name" => {
                        obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                    }
                    "URL" => {
                        obj.url = Some(try!(StringDeserializer::deserialize("URL", stack)));
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
struct QueueListDeserializer;
impl QueueListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Queue>, XmlParseError> {
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
                        obj.push(try!(QueueDeserializer::deserialize("member", stack)));
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
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RebuildEnvironmentMessage {
    /// <p>The ID of the environment to rebuild.</p> <p> Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment to rebuild.</p> <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
}

/// Serialize `RebuildEnvironmentMessage` contents to a `SignedRequest`.
struct RebuildEnvironmentMessageSerializer;
impl RebuildEnvironmentMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebuildEnvironmentMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct RefreshedAtDeserializer;
impl RefreshedAtDeserializer {
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
struct RegexLabelDeserializer;
impl RegexLabelDeserializer {
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
struct RegexPatternDeserializer;
impl RegexPatternDeserializer {
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
struct RequestCountDeserializer;
impl RequestCountDeserializer {
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
/// <p>Request to retrieve logs from an environment and store them in your Elastic Beanstalk storage bucket.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestEnvironmentInfoMessage {
    /// <p>The ID of the environment of the requested data.</p> <p>If no such environment is found, <code>RequestEnvironmentInfo</code> returns an <code>InvalidParameterValue</code> error. </p> <p>Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment of the requested data.</p> <p>If no such environment is found, <code>RequestEnvironmentInfo</code> returns an <code>InvalidParameterValue</code> error. </p> <p>Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
    /// <p>The type of information to request.</p>
    pub info_type: String,
}

/// Serialize `RequestEnvironmentInfoMessage` contents to a `SignedRequest`.
struct RequestEnvironmentInfoMessageSerializer;
impl RequestEnvironmentInfoMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RequestEnvironmentInfoMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "InfoType"),
            &obj.info_type.replace("+", "%2B"),
        );
    }
}

struct RequestIdDeserializer;
impl RequestIdDeserializer {
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
struct ResourceArnDeserializer;
impl ResourceArnDeserializer {
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
struct ResourceIdDeserializer;
impl ResourceIdDeserializer {
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
struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
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
/// <p>The AWS Elastic Beanstalk quota information for a single resource type in an AWS account. It reflects the resource's limits for this account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceQuota {
    /// <p>The maximum number of instances of this Elastic Beanstalk resource type that an AWS account can use.</p>
    pub maximum: Option<i64>,
}

struct ResourceQuotaDeserializer;
impl ResourceQuotaDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceQuota, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceQuota::default();

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
                    "Maximum" => {
                        obj.maximum =
                            Some(try!(BoxedIntDeserializer::deserialize("Maximum", stack)));
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
/// <p>A set of per-resource AWS Elastic Beanstalk quotas associated with an AWS account. They reflect Elastic Beanstalk resource limits for this account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceQuotas {
    /// <p>The quota for applications in the AWS account.</p>
    pub application_quota: Option<ResourceQuota>,
    /// <p>The quota for application versions in the AWS account.</p>
    pub application_version_quota: Option<ResourceQuota>,
    /// <p>The quota for configuration templates in the AWS account.</p>
    pub configuration_template_quota: Option<ResourceQuota>,
    /// <p>The quota for custom platforms in the AWS account.</p>
    pub custom_platform_quota: Option<ResourceQuota>,
    /// <p>The quota for environments in the AWS account.</p>
    pub environment_quota: Option<ResourceQuota>,
}

struct ResourceQuotasDeserializer;
impl ResourceQuotasDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceQuotas, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceQuotas::default();

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
                    "ApplicationQuota" => {
                        obj.application_quota = Some(try!(ResourceQuotaDeserializer::deserialize(
                            "ApplicationQuota",
                            stack
                        )));
                    }
                    "ApplicationVersionQuota" => {
                        obj.application_version_quota =
                            Some(try!(ResourceQuotaDeserializer::deserialize(
                                "ApplicationVersionQuota",
                                stack
                            )));
                    }
                    "ConfigurationTemplateQuota" => {
                        obj.configuration_template_quota =
                            Some(try!(ResourceQuotaDeserializer::deserialize(
                                "ConfigurationTemplateQuota",
                                stack
                            )));
                    }
                    "CustomPlatformQuota" => {
                        obj.custom_platform_quota = Some(try!(
                            ResourceQuotaDeserializer::deserialize("CustomPlatformQuota", stack)
                        ));
                    }
                    "EnvironmentQuota" => {
                        obj.environment_quota = Some(try!(ResourceQuotaDeserializer::deserialize(
                            "EnvironmentQuota",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceTagsDescriptionMessage {
    /// <p>The Amazon Resource Name (ARN) of the resouce for which a tag list was requested.</p>
    pub resource_arn: Option<String>,
    /// <p>A list of tag key-value pairs.</p>
    pub resource_tags: Option<Vec<Tag>>,
}

struct ResourceTagsDescriptionMessageDeserializer;
impl ResourceTagsDescriptionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceTagsDescriptionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceTagsDescriptionMessage::default();

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
                    "ResourceArn" => {
                        obj.resource_arn = Some(try!(ResourceArnDeserializer::deserialize(
                            "ResourceArn",
                            stack
                        )));
                    }
                    "ResourceTags" => {
                        obj.resource_tags = Some(try!(TagListDeserializer::deserialize(
                            "ResourceTags",
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
/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RestartAppServerMessage {
    /// <p>The ID of the environment to restart the server for.</p> <p> Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment to restart the server for.</p> <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
}

/// Serialize `RestartAppServerMessage` contents to a `SignedRequest`.
struct RestartAppServerMessageSerializer;
impl RestartAppServerMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RestartAppServerMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>Request to download logs retrieved with <a>RequestEnvironmentInfo</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RetrieveEnvironmentInfoMessage {
    /// <p>The ID of the data's environment.</p> <p>If no such environment is found, returns an <code>InvalidParameterValue</code> error.</p> <p>Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error.</p>
    pub environment_id: Option<String>,
    /// <p>The name of the data's environment.</p> <p> If no such environment is found, returns an <code>InvalidParameterValue</code> error. </p> <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
    /// <p>The type of information to retrieve.</p>
    pub info_type: String,
}

/// Serialize `RetrieveEnvironmentInfoMessage` contents to a `SignedRequest`.
struct RetrieveEnvironmentInfoMessageSerializer;
impl RetrieveEnvironmentInfoMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RetrieveEnvironmentInfoMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "InfoType"),
            &obj.info_type.replace("+", "%2B"),
        );
    }
}

/// <p>Result message containing a description of the requested environment info.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RetrieveEnvironmentInfoResultMessage {
    /// <p> The <a>EnvironmentInfoDescription</a> of the environment. </p>
    pub environment_info: Option<Vec<EnvironmentInfoDescription>>,
}

struct RetrieveEnvironmentInfoResultMessageDeserializer;
impl RetrieveEnvironmentInfoResultMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RetrieveEnvironmentInfoResultMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RetrieveEnvironmentInfoResultMessage::default();

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
                    "EnvironmentInfo" => {
                        obj.environment_info = Some(try!(
                            EnvironmentInfoDescriptionListDeserializer::deserialize(
                                "EnvironmentInfo",
                                stack
                            )
                        ));
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
struct S3BucketDeserializer;
impl S3BucketDeserializer {
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
struct S3KeyDeserializer;
impl S3KeyDeserializer {
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
/// <p>The bucket and key of an item stored in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Location {
    /// <p>The Amazon S3 bucket where the data is located.</p>
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key where the data is located.</p>
    pub s3_key: Option<String>,
}

struct S3LocationDeserializer;
impl S3LocationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<S3Location, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = S3Location::default();

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
                    "S3Bucket" => {
                        obj.s3_bucket =
                            Some(try!(S3BucketDeserializer::deserialize("S3Bucket", stack)));
                    }
                    "S3Key" => {
                        obj.s3_key = Some(try!(S3KeyDeserializer::deserialize("S3Key", stack)));
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

/// Serialize `S3Location` contents to a `SignedRequest`.
struct S3LocationSerializer;
impl S3LocationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &S3Location) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.s3_bucket {
            params.put(
                &format!("{}{}", prefix, "S3Bucket"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.s3_key {
            params.put(
                &format!("{}{}", prefix, "S3Key"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct SampleTimestampDeserializer;
impl SampleTimestampDeserializer {
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
/// <p>Detailed health information about an Amazon EC2 instance in your Elastic Beanstalk environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SingleInstanceHealth {
    /// <p>Request metrics from your application.</p>
    pub application_metrics: Option<ApplicationMetrics>,
    /// <p>The availability zone in which the instance runs.</p>
    pub availability_zone: Option<String>,
    /// <p>Represents the causes, which provide more information about the current health status.</p>
    pub causes: Option<Vec<String>>,
    /// <p>Represents the color indicator that gives you information about the health of the EC2 instance. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">Health Colors and Statuses</a>.</p>
    pub color: Option<String>,
    /// <p>Information about the most recent deployment to an instance.</p>
    pub deployment: Option<Deployment>,
    /// <p>Returns the health status of the specified instance. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-status.html">Health Colors and Statuses</a>.</p>
    pub health_status: Option<String>,
    /// <p>The ID of the Amazon EC2 instance.</p>
    pub instance_id: Option<String>,
    /// <p>The instance's type.</p>
    pub instance_type: Option<String>,
    /// <p>The time at which the EC2 instance was launched.</p>
    pub launched_at: Option<String>,
    /// <p>Operating system metrics from the instance.</p>
    pub system: Option<SystemStatus>,
}

struct SingleInstanceHealthDeserializer;
impl SingleInstanceHealthDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SingleInstanceHealth, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SingleInstanceHealth::default();

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
                    "ApplicationMetrics" => {
                        obj.application_metrics =
                            Some(try!(ApplicationMetricsDeserializer::deserialize(
                                "ApplicationMetrics",
                                stack
                            )));
                    }
                    "AvailabilityZone" => {
                        obj.availability_zone = Some(try!(StringDeserializer::deserialize(
                            "AvailabilityZone",
                            stack
                        )));
                    }
                    "Causes" => {
                        obj.causes = Some(try!(CausesDeserializer::deserialize("Causes", stack)));
                    }
                    "Color" => {
                        obj.color = Some(try!(StringDeserializer::deserialize("Color", stack)));
                    }
                    "Deployment" => {
                        obj.deployment = Some(try!(DeploymentDeserializer::deserialize(
                            "Deployment",
                            stack
                        )));
                    }
                    "HealthStatus" => {
                        obj.health_status =
                            Some(try!(StringDeserializer::deserialize("HealthStatus", stack)));
                    }
                    "InstanceId" => {
                        obj.instance_id = Some(try!(InstanceIdDeserializer::deserialize(
                            "InstanceId",
                            stack
                        )));
                    }
                    "InstanceType" => {
                        obj.instance_type =
                            Some(try!(StringDeserializer::deserialize("InstanceType", stack)));
                    }
                    "LaunchedAt" => {
                        obj.launched_at = Some(try!(LaunchedAtDeserializer::deserialize(
                            "LaunchedAt",
                            stack
                        )));
                    }
                    "System" => {
                        obj.system =
                            Some(try!(SystemStatusDeserializer::deserialize("System", stack)));
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
/// <p>Describes the solution stack.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SolutionStackDescription {
    /// <p>The permitted file types allowed for a solution stack.</p>
    pub permitted_file_types: Option<Vec<String>>,
    /// <p>The name of the solution stack.</p>
    pub solution_stack_name: Option<String>,
}

struct SolutionStackDescriptionDeserializer;
impl SolutionStackDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SolutionStackDescription, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SolutionStackDescription::default();

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
                    "PermittedFileTypes" => {
                        obj.permitted_file_types =
                            Some(try!(SolutionStackFileTypeListDeserializer::deserialize(
                                "PermittedFileTypes",
                                stack
                            )));
                    }
                    "SolutionStackName" => {
                        obj.solution_stack_name = Some(try!(
                            SolutionStackNameDeserializer::deserialize("SolutionStackName", stack)
                        ));
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
struct SolutionStackFileTypeListDeserializer;
impl SolutionStackFileTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(FileTypeExtensionDeserializer::deserialize(
                            "member", stack
                        )));
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
struct SolutionStackNameDeserializer;
impl SolutionStackNameDeserializer {
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
/// <p>Location of the source code for an application version.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceBuildInformation {
    /// <p><p>The location of the source code, as a formatted string, depending on the value of <code>SourceRepository</code> </p> <ul> <li> <p>For <code>CodeCommit</code>, the format is the repository name and commit ID, separated by a forward slash. For example, <code>my-git-repo/265cfa0cf6af46153527f55d6503ec030551f57a</code>.</p> </li> <li> <p>For <code>S3</code>, the format is the S3 bucket name and object key, separated by a forward slash. For example, <code>my-s3-bucket/Folders/my-source-file</code>.</p> </li> </ul></p>
    pub source_location: String,
    /// <p><p>Location where the repository is stored.</p> <ul> <li> <p> <code>CodeCommit</code> </p> </li> <li> <p> <code>S3</code> </p> </li> </ul></p>
    pub source_repository: String,
    /// <p><p>The type of repository.</p> <ul> <li> <p> <code>Git</code> </p> </li> <li> <p> <code>Zip</code> </p> </li> </ul></p>
    pub source_type: String,
}

struct SourceBuildInformationDeserializer;
impl SourceBuildInformationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SourceBuildInformation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SourceBuildInformation::default();

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
                    "SourceLocation" => {
                        obj.source_location = try!(SourceLocationDeserializer::deserialize(
                            "SourceLocation",
                            stack
                        ));
                    }
                    "SourceRepository" => {
                        obj.source_repository = try!(SourceRepositoryDeserializer::deserialize(
                            "SourceRepository",
                            stack
                        ));
                    }
                    "SourceType" => {
                        obj.source_type =
                            try!(SourceTypeDeserializer::deserialize("SourceType", stack));
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

/// Serialize `SourceBuildInformation` contents to a `SignedRequest`.
struct SourceBuildInformationSerializer;
impl SourceBuildInformationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SourceBuildInformation) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "SourceLocation"),
            &obj.source_location.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SourceRepository"),
            &obj.source_repository.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "SourceType"),
            &obj.source_type.replace("+", "%2B"),
        );
    }
}

/// <p>A specification for an environment configuration</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SourceConfiguration {
    /// <p>The name of the application associated with the configuration.</p>
    pub application_name: Option<String>,
    /// <p>The name of the configuration template.</p>
    pub template_name: Option<String>,
}

/// Serialize `SourceConfiguration` contents to a `SignedRequest`.
struct SourceConfigurationSerializer;
impl SourceConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SourceConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct SourceLocationDeserializer;
impl SourceLocationDeserializer {
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
struct SourceRepositoryDeserializer;
impl SourceRepositoryDeserializer {
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
struct SourceTypeDeserializer;
impl SourceTypeDeserializer {
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
/// <p>Represents the percentage of requests over the last 10 seconds that resulted in each type of status code response. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html">Status Code Definitions</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatusCodes {
    /// <p>The percentage of requests over the last 10 seconds that resulted in a 2xx (200, 201, etc.) status code.</p>
    pub status_2xx: Option<i64>,
    /// <p>The percentage of requests over the last 10 seconds that resulted in a 3xx (300, 301, etc.) status code.</p>
    pub status_3xx: Option<i64>,
    /// <p>The percentage of requests over the last 10 seconds that resulted in a 4xx (400, 401, etc.) status code.</p>
    pub status_4xx: Option<i64>,
    /// <p>The percentage of requests over the last 10 seconds that resulted in a 5xx (500, 501, etc.) status code.</p>
    pub status_5xx: Option<i64>,
}

struct StatusCodesDeserializer;
impl StatusCodesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StatusCodes, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StatusCodes::default();

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
                    "Status2xx" => {
                        obj.status_2xx = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Status2xx",
                            stack
                        )));
                    }
                    "Status3xx" => {
                        obj.status_3xx = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Status3xx",
                            stack
                        )));
                    }
                    "Status4xx" => {
                        obj.status_4xx = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Status4xx",
                            stack
                        )));
                    }
                    "Status5xx" => {
                        obj.status_5xx = Some(try!(NullableIntegerDeserializer::deserialize(
                            "Status5xx",
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
struct StringDeserializer;
impl StringDeserializer {
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
struct SupportedAddonDeserializer;
impl SupportedAddonDeserializer {
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
struct SupportedAddonListDeserializer;
impl SupportedAddonListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(SupportedAddonDeserializer::deserialize(
                            "member", stack
                        )));
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
struct SupportedTierDeserializer;
impl SupportedTierDeserializer {
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
struct SupportedTierListDeserializer;
impl SupportedTierListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(SupportedTierDeserializer::deserialize(
                            "member", stack
                        )));
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
/// <p>Swaps the CNAMEs of two environments.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SwapEnvironmentCNAMEsMessage {
    /// <p>The ID of the destination environment.</p> <p> Condition: You must specify at least the <code>DestinationEnvironmentID</code> or the <code>DestinationEnvironmentName</code>. You may also specify both. You must specify the <code>SourceEnvironmentId</code> with the <code>DestinationEnvironmentId</code>. </p>
    pub destination_environment_id: Option<String>,
    /// <p>The name of the destination environment.</p> <p> Condition: You must specify at least the <code>DestinationEnvironmentID</code> or the <code>DestinationEnvironmentName</code>. You may also specify both. You must specify the <code>SourceEnvironmentName</code> with the <code>DestinationEnvironmentName</code>. </p>
    pub destination_environment_name: Option<String>,
    /// <p>The ID of the source environment.</p> <p> Condition: You must specify at least the <code>SourceEnvironmentID</code> or the <code>SourceEnvironmentName</code>. You may also specify both. If you specify the <code>SourceEnvironmentId</code>, you must specify the <code>DestinationEnvironmentId</code>. </p>
    pub source_environment_id: Option<String>,
    /// <p>The name of the source environment.</p> <p> Condition: You must specify at least the <code>SourceEnvironmentID</code> or the <code>SourceEnvironmentName</code>. You may also specify both. If you specify the <code>SourceEnvironmentName</code>, you must specify the <code>DestinationEnvironmentName</code>. </p>
    pub source_environment_name: Option<String>,
}

/// Serialize `SwapEnvironmentCNAMEsMessage` contents to a `SignedRequest`.
struct SwapEnvironmentCNAMEsMessageSerializer;
impl SwapEnvironmentCNAMEsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SwapEnvironmentCNAMEsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.destination_environment_id {
            params.put(
                &format!("{}{}", prefix, "DestinationEnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.destination_environment_name {
            params.put(
                &format!("{}{}", prefix, "DestinationEnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_environment_id {
            params.put(
                &format!("{}{}", prefix, "SourceEnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.source_environment_name {
            params.put(
                &format!("{}{}", prefix, "SourceEnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>CPU utilization and load average metrics for an Amazon EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SystemStatus {
    /// <p>CPU utilization metrics for the instance.</p>
    pub cpu_utilization: Option<CPUUtilization>,
    /// <p>Load average in the last 1-minute, 5-minute, and 15-minute periods. For more information, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced-metrics.html#health-enhanced-metrics-os">Operating System Metrics</a>.</p>
    pub load_average: Option<Vec<f64>>,
}

struct SystemStatusDeserializer;
impl SystemStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SystemStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SystemStatus::default();

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
                    "CPUUtilization" => {
                        obj.cpu_utilization = Some(try!(CPUUtilizationDeserializer::deserialize(
                            "CPUUtilization",
                            stack
                        )));
                    }
                    "LoadAverage" => {
                        obj.load_average = Some(try!(LoadAverageDeserializer::deserialize(
                            "LoadAverage",
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
/// <p>Describes a tag applied to a resource in an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: Option<String>,
    /// <p>The value of the tag.</p>
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Tag, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Key" => {
                        obj.key = Some(try!(TagKeyDeserializer::deserialize("Key", stack)));
                    }
                    "Value" => {
                        obj.value = Some(try!(TagValueDeserializer::deserialize("Value", stack)));
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

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(
                &format!("{}{}", prefix, "Key"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.value {
            params.put(
                &format!("{}{}", prefix, "Value"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
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

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
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
                        obj.push(try!(TagDeserializer::deserialize("member", stack)));
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

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

struct TagValueDeserializer;
impl TagValueDeserializer {
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

/// Serialize `Tags` contents to a `SignedRequest`.
struct TagsSerializer;
impl TagsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Request to terminate an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TerminateEnvironmentMessage {
    /// <p>The ID of the environment to terminate.</p> <p> Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment to terminate.</p> <p> Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
    /// <p>Terminates the target environment even if another environment in the same group is dependent on it.</p>
    pub force_terminate: Option<bool>,
    /// <p>Indicates whether the associated AWS resources should shut down when the environment is terminated:</p> <ul> <li> <p> <code>true</code>: The specified environment as well as the associated AWS resources, such as Auto Scaling group and LoadBalancer, are terminated.</p> </li> <li> <p> <code>false</code>: AWS Elastic Beanstalk resource management is removed from the environment, but the AWS resources continue to operate.</p> </li> </ul> <p> For more information, see the <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/ug/"> AWS Elastic Beanstalk User Guide. </a> </p> <p> Default: <code>true</code> </p> <p> Valid Values: <code>true</code> | <code>false</code> </p>
    pub terminate_resources: Option<bool>,
}

/// Serialize `TerminateEnvironmentMessage` contents to a `SignedRequest`.
struct TerminateEnvironmentMessageSerializer;
impl TerminateEnvironmentMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TerminateEnvironmentMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.force_terminate {
            params.put(
                &format!("{}{}", prefix, "ForceTerminate"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.terminate_resources {
            params.put(
                &format!("{}{}", prefix, "TerminateResources"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

struct TimestampDeserializer;
impl TimestampDeserializer {
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
struct TokenDeserializer;
impl TokenDeserializer {
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
/// <p>Describes a trigger.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Trigger {
    /// <p>The name of the trigger.</p>
    pub name: Option<String>,
}

struct TriggerDeserializer;
impl TriggerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Trigger, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Trigger::default();

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
                    "Name" => {
                        obj.name = Some(try!(ResourceIdDeserializer::deserialize("Name", stack)));
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
struct TriggerListDeserializer;
impl TriggerListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Trigger>, XmlParseError> {
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
                        obj.push(try!(TriggerDeserializer::deserialize("member", stack)));
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
/// <p>Request to update an application.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateApplicationMessage {
    /// <p>The name of the application to update. If no such application is found, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error. </p>
    pub application_name: String,
    /// <p>A new description for the application.</p> <p>Default: If not specified, AWS Elastic Beanstalk does not update the description.</p>
    pub description: Option<String>,
}

/// Serialize `UpdateApplicationMessage` contents to a `SignedRequest`.
struct UpdateApplicationMessageSerializer;
impl UpdateApplicationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateApplicationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateApplicationResourceLifecycleMessage {
    /// <p>The name of the application.</p>
    pub application_name: String,
    /// <p>The lifecycle configuration.</p>
    pub resource_lifecycle_config: ApplicationResourceLifecycleConfig,
}

/// Serialize `UpdateApplicationResourceLifecycleMessage` contents to a `SignedRequest`.
struct UpdateApplicationResourceLifecycleMessageSerializer;
impl UpdateApplicationResourceLifecycleMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateApplicationResourceLifecycleMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        ApplicationResourceLifecycleConfigSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ResourceLifecycleConfig"),
            &obj.resource_lifecycle_config,
        );
    }
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateApplicationVersionMessage {
    /// <p>The name of the application associated with this version.</p> <p> If no application is found with this name, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error.</p>
    pub application_name: String,
    /// <p>A new description for this version.</p>
    pub description: Option<String>,
    /// <p>The name of the version to update.</p> <p>If no application version is found with this label, <code>UpdateApplication</code> returns an <code>InvalidParameterValue</code> error. </p>
    pub version_label: String,
}

/// Serialize `UpdateApplicationVersionMessage` contents to a `SignedRequest`.
struct UpdateApplicationVersionMessageSerializer;
impl UpdateApplicationVersionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateApplicationVersionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "VersionLabel"),
            &obj.version_label.replace("+", "%2B"),
        );
    }
}

/// <p>The result message containing the options for the specified solution stack.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateConfigurationTemplateMessage {
    /// <p>The name of the application associated with the configuration template to update.</p> <p> If no application is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error. </p>
    pub application_name: String,
    /// <p>A new description for the configuration.</p>
    pub description: Option<String>,
    /// <p>A list of configuration option settings to update with the new specified option value.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>A list of configuration options to remove from the configuration set.</p> <p> Constraint: You can remove only <code>UserDefined</code> configuration options. </p>
    pub options_to_remove: Option<Vec<OptionSpecification>>,
    /// <p>The name of the configuration template to update.</p> <p> If no configuration template is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error. </p>
    pub template_name: String,
}

/// Serialize `UpdateConfigurationTemplateMessage` contents to a `SignedRequest`.
struct UpdateConfigurationTemplateMessageSerializer;
impl UpdateConfigurationTemplateMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateConfigurationTemplateMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_settings {
            ConfigurationOptionSettingsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionSettings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.options_to_remove {
            OptionsSpecifierListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionsToRemove"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "TemplateName"),
            &obj.template_name.replace("+", "%2B"),
        );
    }
}

struct UpdateDateDeserializer;
impl UpdateDateDeserializer {
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
/// <p>Request to update an environment.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateEnvironmentMessage {
    /// <p>The name of the application with which the environment is associated.</p>
    pub application_name: Option<String>,
    /// <p>If this parameter is specified, AWS Elastic Beanstalk updates the description of this environment.</p>
    pub description: Option<String>,
    /// <p>The ID of the environment to update.</p> <p>If no environment with this ID exists, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error.</p> <p>Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_id: Option<String>,
    /// <p>The name of the environment to update. If no environment with this name exists, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p> <p>Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    pub environment_name: Option<String>,
    /// <p>The name of the group to which the target environment belongs. Specify a group name only if the environment's name is specified in an environment manifest and not with the environment name or environment ID parameters. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest (env.yaml)</a> for details.</p>
    pub group_name: Option<String>,
    /// <p>If specified, AWS Elastic Beanstalk updates the configuration set associated with the running environment and sets the specified configuration options to the requested value.</p>
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,
    /// <p>A list of custom user-defined configuration options to remove from the configuration set for this environment.</p>
    pub options_to_remove: Option<Vec<OptionSpecification>>,
    /// <p>The ARN of the platform, if used.</p>
    pub platform_arn: Option<String>,
    /// <p>This specifies the platform version that the environment will run after the environment is updated.</p>
    pub solution_stack_name: Option<String>,
    /// <p>If this parameter is specified, AWS Elastic Beanstalk deploys this configuration template to the environment. If no such configuration template is found, AWS Elastic Beanstalk returns an <code>InvalidParameterValue</code> error. </p>
    pub template_name: Option<String>,
    /// <p>This specifies the tier to use to update the environment.</p> <p>Condition: At this time, if you change the tier version, name, or type, AWS Elastic Beanstalk returns <code>InvalidParameterValue</code> error. </p>
    pub tier: Option<EnvironmentTier>,
    /// <p>If this parameter is specified, AWS Elastic Beanstalk deploys the named application version to the environment. If no such application version is found, returns an <code>InvalidParameterValue</code> error. </p>
    pub version_label: Option<String>,
}

/// Serialize `UpdateEnvironmentMessage` contents to a `SignedRequest`.
struct UpdateEnvironmentMessageSerializer;
impl UpdateEnvironmentMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateEnvironmentMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.application_name {
            params.put(
                &format!("{}{}", prefix, "ApplicationName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_id {
            params.put(
                &format!("{}{}", prefix, "EnvironmentId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.group_name {
            params.put(
                &format!("{}{}", prefix, "GroupName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.option_settings {
            ConfigurationOptionSettingsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionSettings"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.options_to_remove {
            OptionsSpecifierListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OptionsToRemove"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.platform_arn {
            params.put(
                &format!("{}{}", prefix, "PlatformArn"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.solution_stack_name {
            params.put(
                &format!("{}{}", prefix, "SolutionStackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tier {
            EnvironmentTierSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Tier"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.version_label {
            params.put(
                &format!("{}{}", prefix, "VersionLabel"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTagsForResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the resouce to be updated.</p> <p>Must be the ARN of an Elastic Beanstalk environment.</p>
    pub resource_arn: String,
    /// <p>A list of tags to add or update.</p> <p>If a key of an existing tag is added, the tag's value is updated.</p>
    pub tags_to_add: Option<Vec<Tag>>,
    /// <p>A list of tag keys to remove.</p> <p>If a tag key doesn't exist, it is silently ignored.</p>
    pub tags_to_remove: Option<Vec<String>>,
}

/// Serialize `UpdateTagsForResourceMessage` contents to a `SignedRequest`.
struct UpdateTagsForResourceMessageSerializer;
impl UpdateTagsForResourceMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateTagsForResourceMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ResourceArn"),
            &obj.resource_arn.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags_to_add {
            TagListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagsToAdd"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.tags_to_remove {
            TagKeyListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "TagsToRemove"),
                field_value,
            );
        }
    }
}

struct UserDefinedOptionDeserializer;
impl UserDefinedOptionDeserializer {
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
/// <p>A list of validation messages for a specified configuration template.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidateConfigurationSettingsMessage {
    /// <p>The name of the application that the configuration template or environment belongs to.</p>
    pub application_name: String,
    /// <p>The name of the environment to validate the settings against.</p> <p>Condition: You cannot specify both this and a configuration template name.</p>
    pub environment_name: Option<String>,
    /// <p>A list of the options and desired values to evaluate.</p>
    pub option_settings: Vec<ConfigurationOptionSetting>,
    /// <p>The name of the configuration template to validate the settings against.</p> <p>Condition: You cannot specify both this and an environment name.</p>
    pub template_name: Option<String>,
}

/// Serialize `ValidateConfigurationSettingsMessage` contents to a `SignedRequest`.
struct ValidateConfigurationSettingsMessageSerializer;
impl ValidateConfigurationSettingsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ValidateConfigurationSettingsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplicationName"),
            &obj.application_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.environment_name {
            params.put(
                &format!("{}{}", prefix, "EnvironmentName"),
                &field_value.replace("+", "%2B"),
            );
        }
        ConfigurationOptionSettingsListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "OptionSettings"),
            &obj.option_settings,
        );
        if let Some(ref field_value) = obj.template_name {
            params.put(
                &format!("{}{}", prefix, "TemplateName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>An error or warning for a desired configuration option value.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidationMessage {
    /// <p>A message describing the error or warning.</p>
    pub message: Option<String>,
    /// <p>The namespace to which the option belongs.</p>
    pub namespace: Option<String>,
    /// <p>The name of the option.</p>
    pub option_name: Option<String>,
    /// <p><p>An indication of the severity of this message:</p> <ul> <li> <p> <code>error</code>: This message indicates that this is not a valid setting for an option.</p> </li> <li> <p> <code>warning</code>: This message is providing information you should take into account.</p> </li> </ul></p>
    pub severity: Option<String>,
}

struct ValidationMessageDeserializer;
impl ValidationMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidationMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ValidationMessage::default();

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
                    "Message" => {
                        obj.message = Some(try!(ValidationMessageStringDeserializer::deserialize(
                            "Message", stack
                        )));
                    }
                    "Namespace" => {
                        obj.namespace = Some(try!(OptionNamespaceDeserializer::deserialize(
                            "Namespace",
                            stack
                        )));
                    }
                    "OptionName" => {
                        obj.option_name = Some(try!(
                            ConfigurationOptionNameDeserializer::deserialize("OptionName", stack)
                        ));
                    }
                    "Severity" => {
                        obj.severity = Some(try!(ValidationSeverityDeserializer::deserialize(
                            "Severity", stack
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
struct ValidationMessageStringDeserializer;
impl ValidationMessageStringDeserializer {
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
struct ValidationMessagesListDeserializer;
impl ValidationMessagesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ValidationMessage>, XmlParseError> {
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
                        obj.push(try!(ValidationMessageDeserializer::deserialize(
                            "member", stack
                        )));
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
struct ValidationSeverityDeserializer;
impl ValidationSeverityDeserializer {
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
struct VersionLabelDeserializer;
impl VersionLabelDeserializer {
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

/// Serialize `VersionLabels` contents to a `SignedRequest`.
struct VersionLabelsSerializer;
impl VersionLabelsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct VersionLabelsListDeserializer;
impl VersionLabelsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
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
                        obj.push(try!(VersionLabelDeserializer::deserialize("member", stack)));
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

/// Serialize `VersionLabelsList` contents to a `SignedRequest`.
struct VersionLabelsListSerializer;
impl VersionLabelsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct VirtualizationTypeDeserializer;
impl VirtualizationTypeDeserializer {
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
/// Errors returned by AbortEnvironmentUpdate
#[derive(Debug, PartialEq)]
pub enum AbortEnvironmentUpdateError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AbortEnvironmentUpdateError {
    pub fn from_body(body: &str) -> AbortEnvironmentUpdateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    AbortEnvironmentUpdateError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => AbortEnvironmentUpdateError::Unknown(String::from(body)),
            },
            Err(_) => AbortEnvironmentUpdateError::Unknown(body.to_string()),
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

impl From<XmlParseError> for AbortEnvironmentUpdateError {
    fn from(err: XmlParseError) -> AbortEnvironmentUpdateError {
        let XmlParseError(message) = err;
        AbortEnvironmentUpdateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AbortEnvironmentUpdateError {
    fn from(err: CredentialsError) -> AbortEnvironmentUpdateError {
        AbortEnvironmentUpdateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AbortEnvironmentUpdateError {
    fn from(err: HttpDispatchError) -> AbortEnvironmentUpdateError {
        AbortEnvironmentUpdateError::HttpDispatch(err)
    }
}
impl From<io::Error> for AbortEnvironmentUpdateError {
    fn from(err: io::Error) -> AbortEnvironmentUpdateError {
        AbortEnvironmentUpdateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AbortEnvironmentUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AbortEnvironmentUpdateError {
    fn description(&self) -> &str {
        match *self {
            AbortEnvironmentUpdateError::InsufficientPrivileges(ref cause) => cause,
            AbortEnvironmentUpdateError::Validation(ref cause) => cause,
            AbortEnvironmentUpdateError::Credentials(ref err) => err.description(),
            AbortEnvironmentUpdateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AbortEnvironmentUpdateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ApplyEnvironmentManagedAction
#[derive(Debug, PartialEq)]
pub enum ApplyEnvironmentManagedActionError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>Cannot modify the managed action in its current state.</p>
    ManagedActionInvalidState(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ApplyEnvironmentManagedActionError {
    pub fn from_body(body: &str) -> ApplyEnvironmentManagedActionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    ApplyEnvironmentManagedActionError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "ManagedActionInvalidStateException" => {
                    ApplyEnvironmentManagedActionError::ManagedActionInvalidState(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ApplyEnvironmentManagedActionError::Unknown(String::from(body)),
            },
            Err(_) => ApplyEnvironmentManagedActionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ApplyEnvironmentManagedActionError {
    fn from(err: XmlParseError) -> ApplyEnvironmentManagedActionError {
        let XmlParseError(message) = err;
        ApplyEnvironmentManagedActionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ApplyEnvironmentManagedActionError {
    fn from(err: CredentialsError) -> ApplyEnvironmentManagedActionError {
        ApplyEnvironmentManagedActionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ApplyEnvironmentManagedActionError {
    fn from(err: HttpDispatchError) -> ApplyEnvironmentManagedActionError {
        ApplyEnvironmentManagedActionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ApplyEnvironmentManagedActionError {
    fn from(err: io::Error) -> ApplyEnvironmentManagedActionError {
        ApplyEnvironmentManagedActionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ApplyEnvironmentManagedActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplyEnvironmentManagedActionError {
    fn description(&self) -> &str {
        match *self {
            ApplyEnvironmentManagedActionError::ElasticBeanstalkService(ref cause) => cause,
            ApplyEnvironmentManagedActionError::ManagedActionInvalidState(ref cause) => cause,
            ApplyEnvironmentManagedActionError::Validation(ref cause) => cause,
            ApplyEnvironmentManagedActionError::Credentials(ref err) => err.description(),
            ApplyEnvironmentManagedActionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ApplyEnvironmentManagedActionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CheckDNSAvailability
#[derive(Debug, PartialEq)]
pub enum CheckDNSAvailabilityError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CheckDNSAvailabilityError {
    pub fn from_body(body: &str) -> CheckDNSAvailabilityError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => CheckDNSAvailabilityError::Unknown(String::from(body)),
            },
            Err(_) => CheckDNSAvailabilityError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CheckDNSAvailabilityError {
    fn from(err: XmlParseError) -> CheckDNSAvailabilityError {
        let XmlParseError(message) = err;
        CheckDNSAvailabilityError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CheckDNSAvailabilityError {
    fn from(err: CredentialsError) -> CheckDNSAvailabilityError {
        CheckDNSAvailabilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CheckDNSAvailabilityError {
    fn from(err: HttpDispatchError) -> CheckDNSAvailabilityError {
        CheckDNSAvailabilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for CheckDNSAvailabilityError {
    fn from(err: io::Error) -> CheckDNSAvailabilityError {
        CheckDNSAvailabilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CheckDNSAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CheckDNSAvailabilityError {
    fn description(&self) -> &str {
        match *self {
            CheckDNSAvailabilityError::Validation(ref cause) => cause,
            CheckDNSAvailabilityError::Credentials(ref err) => err.description(),
            CheckDNSAvailabilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CheckDNSAvailabilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ComposeEnvironments
#[derive(Debug, PartialEq)]
pub enum ComposeEnvironmentsError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of environments.</p>
    TooManyEnvironments(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ComposeEnvironmentsError {
    pub fn from_body(body: &str) -> ComposeEnvironmentsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    ComposeEnvironmentsError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyEnvironmentsException" => ComposeEnvironmentsError::TooManyEnvironments(
                    String::from(parsed_error.message),
                ),
                _ => ComposeEnvironmentsError::Unknown(String::from(body)),
            },
            Err(_) => ComposeEnvironmentsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ComposeEnvironmentsError {
    fn from(err: XmlParseError) -> ComposeEnvironmentsError {
        let XmlParseError(message) = err;
        ComposeEnvironmentsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ComposeEnvironmentsError {
    fn from(err: CredentialsError) -> ComposeEnvironmentsError {
        ComposeEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ComposeEnvironmentsError {
    fn from(err: HttpDispatchError) -> ComposeEnvironmentsError {
        ComposeEnvironmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ComposeEnvironmentsError {
    fn from(err: io::Error) -> ComposeEnvironmentsError {
        ComposeEnvironmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ComposeEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ComposeEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            ComposeEnvironmentsError::InsufficientPrivileges(ref cause) => cause,
            ComposeEnvironmentsError::TooManyEnvironments(ref cause) => cause,
            ComposeEnvironmentsError::Validation(ref cause) => cause,
            ComposeEnvironmentsError::Credentials(ref err) => err.description(),
            ComposeEnvironmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ComposeEnvironmentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The specified account has reached its limit of applications.</p>
    TooManyApplications(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationError {
    pub fn from_body(body: &str) -> CreateApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TooManyApplicationsException" => {
                    CreateApplicationError::TooManyApplications(String::from(parsed_error.message))
                }
                _ => CreateApplicationError::Unknown(String::from(body)),
            },
            Err(_) => CreateApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateApplicationError {
    fn from(err: XmlParseError) -> CreateApplicationError {
        let XmlParseError(message) = err;
        CreateApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::TooManyApplications(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateApplicationVersionError {
    /// <p>AWS CodeBuild is not available in the specified region.</p>
    CodeBuildNotInServiceRegion(String),
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p><p>The specified S3 bucket does not belong to the S3 region in which the service is running. The following regions are supported:</p> <ul> <li> <p>IAD/us-east-1</p> </li> <li> <p>PDX/us-west-2</p> </li> <li> <p>DUB/eu-west-1</p> </li> </ul></p>
    S3LocationNotInServiceRegion(String),
    /// <p>The specified account has reached its limit of application versions.</p>
    TooManyApplicationVersions(String),
    /// <p>The specified account has reached its limit of applications.</p>
    TooManyApplications(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationVersionError {
    pub fn from_body(body: &str) -> CreateApplicationVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CodeBuildNotInServiceRegionException" => {
                    CreateApplicationVersionError::CodeBuildNotInServiceRegion(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientPrivilegesException" => {
                    CreateApplicationVersionError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "S3LocationNotInServiceRegionException" => {
                    CreateApplicationVersionError::S3LocationNotInServiceRegion(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyApplicationVersionsException" => {
                    CreateApplicationVersionError::TooManyApplicationVersions(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyApplicationsException" => {
                    CreateApplicationVersionError::TooManyApplications(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateApplicationVersionError::Unknown(String::from(body)),
            },
            Err(_) => CreateApplicationVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateApplicationVersionError {
    fn from(err: XmlParseError) -> CreateApplicationVersionError {
        let XmlParseError(message) = err;
        CreateApplicationVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateApplicationVersionError {
    fn from(err: CredentialsError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationVersionError {
    fn from(err: HttpDispatchError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationVersionError {
    fn from(err: io::Error) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationVersionError::CodeBuildNotInServiceRegion(ref cause) => cause,
            CreateApplicationVersionError::InsufficientPrivileges(ref cause) => cause,
            CreateApplicationVersionError::S3LocationNotInServiceRegion(ref cause) => cause,
            CreateApplicationVersionError::TooManyApplicationVersions(ref cause) => cause,
            CreateApplicationVersionError::TooManyApplications(ref cause) => cause,
            CreateApplicationVersionError::Validation(ref cause) => cause,
            CreateApplicationVersionError::Credentials(ref err) => err.description(),
            CreateApplicationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConfigurationTemplate
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationTemplateError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// <p>The specified account has reached its limit of configuration templates.</p>
    TooManyConfigurationTemplates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateConfigurationTemplateError {
    pub fn from_body(body: &str) -> CreateConfigurationTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    CreateConfigurationTemplateError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyBucketsException" => CreateConfigurationTemplateError::TooManyBuckets(
                    String::from(parsed_error.message),
                ),
                "TooManyConfigurationTemplatesException" => {
                    CreateConfigurationTemplateError::TooManyConfigurationTemplates(String::from(
                        parsed_error.message,
                    ))
                }
                _ => CreateConfigurationTemplateError::Unknown(String::from(body)),
            },
            Err(_) => CreateConfigurationTemplateError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateConfigurationTemplateError {
    fn from(err: XmlParseError) -> CreateConfigurationTemplateError {
        let XmlParseError(message) = err;
        CreateConfigurationTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateConfigurationTemplateError {
    fn from(err: CredentialsError) -> CreateConfigurationTemplateError {
        CreateConfigurationTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConfigurationTemplateError {
    fn from(err: HttpDispatchError) -> CreateConfigurationTemplateError {
        CreateConfigurationTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConfigurationTemplateError {
    fn from(err: io::Error) -> CreateConfigurationTemplateError {
        CreateConfigurationTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConfigurationTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationTemplateError::InsufficientPrivileges(ref cause) => cause,
            CreateConfigurationTemplateError::TooManyBuckets(ref cause) => cause,
            CreateConfigurationTemplateError::TooManyConfigurationTemplates(ref cause) => cause,
            CreateConfigurationTemplateError::Validation(ref cause) => cause,
            CreateConfigurationTemplateError::Credentials(ref err) => err.description(),
            CreateConfigurationTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateConfigurationTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateEnvironmentError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of environments.</p>
    TooManyEnvironments(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEnvironmentError {
    pub fn from_body(body: &str) -> CreateEnvironmentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    CreateEnvironmentError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyEnvironmentsException" => {
                    CreateEnvironmentError::TooManyEnvironments(String::from(parsed_error.message))
                }
                _ => CreateEnvironmentError::Unknown(String::from(body)),
            },
            Err(_) => CreateEnvironmentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateEnvironmentError {
    fn from(err: XmlParseError) -> CreateEnvironmentError {
        let XmlParseError(message) = err;
        CreateEnvironmentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateEnvironmentError {
    fn from(err: CredentialsError) -> CreateEnvironmentError {
        CreateEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEnvironmentError {
    fn from(err: HttpDispatchError) -> CreateEnvironmentError {
        CreateEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEnvironmentError {
    fn from(err: io::Error) -> CreateEnvironmentError {
        CreateEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            CreateEnvironmentError::InsufficientPrivileges(ref cause) => cause,
            CreateEnvironmentError::TooManyEnvironments(ref cause) => cause,
            CreateEnvironmentError::Validation(ref cause) => cause,
            CreateEnvironmentError::Credentials(ref err) => err.description(),
            CreateEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePlatformVersion
#[derive(Debug, PartialEq)]
pub enum CreatePlatformVersionError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>You have exceeded the maximum number of allowed platforms associated with the account.</p>
    TooManyPlatforms(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePlatformVersionError {
    pub fn from_body(body: &str) -> CreatePlatformVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    CreatePlatformVersionError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientPrivilegesException" => {
                    CreatePlatformVersionError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyPlatformsException" => {
                    CreatePlatformVersionError::TooManyPlatforms(String::from(parsed_error.message))
                }
                _ => CreatePlatformVersionError::Unknown(String::from(body)),
            },
            Err(_) => CreatePlatformVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreatePlatformVersionError {
    fn from(err: XmlParseError) -> CreatePlatformVersionError {
        let XmlParseError(message) = err;
        CreatePlatformVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreatePlatformVersionError {
    fn from(err: CredentialsError) -> CreatePlatformVersionError {
        CreatePlatformVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePlatformVersionError {
    fn from(err: HttpDispatchError) -> CreatePlatformVersionError {
        CreatePlatformVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePlatformVersionError {
    fn from(err: io::Error) -> CreatePlatformVersionError {
        CreatePlatformVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePlatformVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePlatformVersionError {
    fn description(&self) -> &str {
        match *self {
            CreatePlatformVersionError::ElasticBeanstalkService(ref cause) => cause,
            CreatePlatformVersionError::InsufficientPrivileges(ref cause) => cause,
            CreatePlatformVersionError::TooManyPlatforms(ref cause) => cause,
            CreatePlatformVersionError::Validation(ref cause) => cause,
            CreatePlatformVersionError::Credentials(ref err) => err.description(),
            CreatePlatformVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePlatformVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStorageLocation
#[derive(Debug, PartialEq)]
pub enum CreateStorageLocationError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account does not have a subscription to Amazon S3.</p>
    S3SubscriptionRequired(String),
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStorageLocationError {
    pub fn from_body(body: &str) -> CreateStorageLocationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    CreateStorageLocationError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "S3SubscriptionRequiredException" => {
                    CreateStorageLocationError::S3SubscriptionRequired(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyBucketsException" => {
                    CreateStorageLocationError::TooManyBuckets(String::from(parsed_error.message))
                }
                _ => CreateStorageLocationError::Unknown(String::from(body)),
            },
            Err(_) => CreateStorageLocationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateStorageLocationError {
    fn from(err: XmlParseError) -> CreateStorageLocationError {
        let XmlParseError(message) = err;
        CreateStorageLocationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStorageLocationError {
    fn from(err: CredentialsError) -> CreateStorageLocationError {
        CreateStorageLocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStorageLocationError {
    fn from(err: HttpDispatchError) -> CreateStorageLocationError {
        CreateStorageLocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStorageLocationError {
    fn from(err: io::Error) -> CreateStorageLocationError {
        CreateStorageLocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStorageLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStorageLocationError {
    fn description(&self) -> &str {
        match *self {
            CreateStorageLocationError::InsufficientPrivileges(ref cause) => cause,
            CreateStorageLocationError::S3SubscriptionRequired(ref cause) => cause,
            CreateStorageLocationError::TooManyBuckets(ref cause) => cause,
            CreateStorageLocationError::Validation(ref cause) => cause,
            CreateStorageLocationError::Credentials(ref err) => err.description(),
            CreateStorageLocationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStorageLocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>Unable to perform the specified operation because another operation that effects an element in this activity is already in progress.</p>
    OperationInProgress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteApplicationError {
    pub fn from_body(body: &str) -> DeleteApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationInProgressFailure" => {
                    DeleteApplicationError::OperationInProgress(String::from(parsed_error.message))
                }
                _ => DeleteApplicationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteApplicationError {
    fn from(err: XmlParseError) -> DeleteApplicationError {
        let XmlParseError(message) = err;
        DeleteApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationError {
    fn from(err: CredentialsError) -> DeleteApplicationError {
        DeleteApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationError {
    fn from(err: io::Error) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::OperationInProgress(ref cause) => cause,
            DeleteApplicationError::Validation(ref cause) => cause,
            DeleteApplicationError::Credentials(ref err) => err.description(),
            DeleteApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationVersion
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationVersionError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>Unable to perform the specified operation because another operation that effects an element in this activity is already in progress.</p>
    OperationInProgress(String),
    /// <p><p>The specified S3 bucket does not belong to the S3 region in which the service is running. The following regions are supported:</p> <ul> <li> <p>IAD/us-east-1</p> </li> <li> <p>PDX/us-west-2</p> </li> <li> <p>DUB/eu-west-1</p> </li> </ul></p>
    S3LocationNotInServiceRegion(String),
    /// <p>Unable to delete the Amazon S3 source bundle associated with the application version. The application version was deleted successfully.</p>
    SourceBundleDeletion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteApplicationVersionError {
    pub fn from_body(body: &str) -> DeleteApplicationVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    DeleteApplicationVersionError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressFailure" => DeleteApplicationVersionError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "S3LocationNotInServiceRegionException" => {
                    DeleteApplicationVersionError::S3LocationNotInServiceRegion(String::from(
                        parsed_error.message,
                    ))
                }
                "SourceBundleDeletionFailure" => {
                    DeleteApplicationVersionError::SourceBundleDeletion(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteApplicationVersionError::Unknown(String::from(body)),
            },
            Err(_) => DeleteApplicationVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteApplicationVersionError {
    fn from(err: XmlParseError) -> DeleteApplicationVersionError {
        let XmlParseError(message) = err;
        DeleteApplicationVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationVersionError {
    fn from(err: CredentialsError) -> DeleteApplicationVersionError {
        DeleteApplicationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationVersionError {
    fn from(err: HttpDispatchError) -> DeleteApplicationVersionError {
        DeleteApplicationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationVersionError {
    fn from(err: io::Error) -> DeleteApplicationVersionError {
        DeleteApplicationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationVersionError::InsufficientPrivileges(ref cause) => cause,
            DeleteApplicationVersionError::OperationInProgress(ref cause) => cause,
            DeleteApplicationVersionError::S3LocationNotInServiceRegion(ref cause) => cause,
            DeleteApplicationVersionError::SourceBundleDeletion(ref cause) => cause,
            DeleteApplicationVersionError::Validation(ref cause) => cause,
            DeleteApplicationVersionError::Credentials(ref err) => err.description(),
            DeleteApplicationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationTemplateError {
    /// <p>Unable to perform the specified operation because another operation that effects an element in this activity is already in progress.</p>
    OperationInProgress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteConfigurationTemplateError {
    pub fn from_body(body: &str) -> DeleteConfigurationTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationInProgressFailure" => {
                    DeleteConfigurationTemplateError::OperationInProgress(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeleteConfigurationTemplateError::Unknown(String::from(body)),
            },
            Err(_) => DeleteConfigurationTemplateError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteConfigurationTemplateError {
    fn from(err: XmlParseError) -> DeleteConfigurationTemplateError {
        let XmlParseError(message) = err;
        DeleteConfigurationTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteConfigurationTemplateError {
    fn from(err: CredentialsError) -> DeleteConfigurationTemplateError {
        DeleteConfigurationTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConfigurationTemplateError {
    fn from(err: HttpDispatchError) -> DeleteConfigurationTemplateError {
        DeleteConfigurationTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConfigurationTemplateError {
    fn from(err: io::Error) -> DeleteConfigurationTemplateError {
        DeleteConfigurationTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConfigurationTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationTemplateError::OperationInProgress(ref cause) => cause,
            DeleteConfigurationTemplateError::Validation(ref cause) => cause,
            DeleteConfigurationTemplateError::Credentials(ref err) => err.description(),
            DeleteConfigurationTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteConfigurationTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEnvironmentConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteEnvironmentConfigurationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEnvironmentConfigurationError {
    pub fn from_body(body: &str) -> DeleteEnvironmentConfigurationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DeleteEnvironmentConfigurationError::Unknown(String::from(body)),
            },
            Err(_) => DeleteEnvironmentConfigurationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteEnvironmentConfigurationError {
    fn from(err: XmlParseError) -> DeleteEnvironmentConfigurationError {
        let XmlParseError(message) = err;
        DeleteEnvironmentConfigurationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteEnvironmentConfigurationError {
    fn from(err: CredentialsError) -> DeleteEnvironmentConfigurationError {
        DeleteEnvironmentConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEnvironmentConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteEnvironmentConfigurationError {
        DeleteEnvironmentConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEnvironmentConfigurationError {
    fn from(err: io::Error) -> DeleteEnvironmentConfigurationError {
        DeleteEnvironmentConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEnvironmentConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEnvironmentConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteEnvironmentConfigurationError::Validation(ref cause) => cause,
            DeleteEnvironmentConfigurationError::Credentials(ref err) => err.description(),
            DeleteEnvironmentConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEnvironmentConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePlatformVersion
#[derive(Debug, PartialEq)]
pub enum DeletePlatformVersionError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>Unable to perform the specified operation because another operation that effects an element in this activity is already in progress.</p>
    OperationInProgress(String),
    /// <p>You cannot delete the platform version because there are still environments running on it.</p>
    PlatformVersionStillReferenced(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePlatformVersionError {
    pub fn from_body(body: &str) -> DeletePlatformVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DeletePlatformVersionError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientPrivilegesException" => {
                    DeletePlatformVersionError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressFailure" => DeletePlatformVersionError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "PlatformVersionStillReferencedException" => {
                    DeletePlatformVersionError::PlatformVersionStillReferenced(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DeletePlatformVersionError::Unknown(String::from(body)),
            },
            Err(_) => DeletePlatformVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeletePlatformVersionError {
    fn from(err: XmlParseError) -> DeletePlatformVersionError {
        let XmlParseError(message) = err;
        DeletePlatformVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeletePlatformVersionError {
    fn from(err: CredentialsError) -> DeletePlatformVersionError {
        DeletePlatformVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePlatformVersionError {
    fn from(err: HttpDispatchError) -> DeletePlatformVersionError {
        DeletePlatformVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePlatformVersionError {
    fn from(err: io::Error) -> DeletePlatformVersionError {
        DeletePlatformVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePlatformVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePlatformVersionError {
    fn description(&self) -> &str {
        match *self {
            DeletePlatformVersionError::ElasticBeanstalkService(ref cause) => cause,
            DeletePlatformVersionError::InsufficientPrivileges(ref cause) => cause,
            DeletePlatformVersionError::OperationInProgress(ref cause) => cause,
            DeletePlatformVersionError::PlatformVersionStillReferenced(ref cause) => cause,
            DeletePlatformVersionError::Validation(ref cause) => cause,
            DeletePlatformVersionError::Credentials(ref err) => err.description(),
            DeletePlatformVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePlatformVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAccountAttributesError {
    pub fn from_body(body: &str) -> DescribeAccountAttributesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    DescribeAccountAttributesError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeAccountAttributesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAccountAttributesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeAccountAttributesError {
    fn from(err: XmlParseError) -> DescribeAccountAttributesError {
        let XmlParseError(message) = err;
        DescribeAccountAttributesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAccountAttributesError {
    fn from(err: CredentialsError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountAttributesError {
    fn from(err: HttpDispatchError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAccountAttributesError {
    fn from(err: io::Error) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountAttributesError::InsufficientPrivileges(ref cause) => cause,
            DescribeAccountAttributesError::Validation(ref cause) => cause,
            DescribeAccountAttributesError::Credentials(ref err) => err.description(),
            DescribeAccountAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplicationVersions
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeApplicationVersionsError {
    pub fn from_body(body: &str) -> DescribeApplicationVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeApplicationVersionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeApplicationVersionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeApplicationVersionsError {
    fn from(err: XmlParseError) -> DescribeApplicationVersionsError {
        let XmlParseError(message) = err;
        DescribeApplicationVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeApplicationVersionsError {
    fn from(err: CredentialsError) -> DescribeApplicationVersionsError {
        DescribeApplicationVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeApplicationVersionsError {
    fn from(err: HttpDispatchError) -> DescribeApplicationVersionsError {
        DescribeApplicationVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeApplicationVersionsError {
    fn from(err: io::Error) -> DescribeApplicationVersionsError {
        DescribeApplicationVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeApplicationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeApplicationVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeApplicationVersionsError::Validation(ref cause) => cause,
            DescribeApplicationVersionsError::Credentials(ref err) => err.description(),
            DescribeApplicationVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeApplicationVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplications
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeApplicationsError {
    pub fn from_body(body: &str) -> DescribeApplicationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeApplicationsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeApplicationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeApplicationsError {
    fn from(err: XmlParseError) -> DescribeApplicationsError {
        let XmlParseError(message) = err;
        DescribeApplicationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeApplicationsError {
    fn from(err: CredentialsError) -> DescribeApplicationsError {
        DescribeApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeApplicationsError {
    fn from(err: HttpDispatchError) -> DescribeApplicationsError {
        DescribeApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeApplicationsError {
    fn from(err: io::Error) -> DescribeApplicationsError {
        DescribeApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeApplicationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeApplicationsError::Validation(ref cause) => cause,
            DescribeApplicationsError::Credentials(ref err) => err.description(),
            DescribeApplicationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationOptions
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationOptionsError {
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationOptionsError {
    pub fn from_body(body: &str) -> DescribeConfigurationOptionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TooManyBucketsException" => DescribeConfigurationOptionsError::TooManyBuckets(
                    String::from(parsed_error.message),
                ),
                _ => DescribeConfigurationOptionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeConfigurationOptionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeConfigurationOptionsError {
    fn from(err: XmlParseError) -> DescribeConfigurationOptionsError {
        let XmlParseError(message) = err;
        DescribeConfigurationOptionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationOptionsError {
    fn from(err: CredentialsError) -> DescribeConfigurationOptionsError {
        DescribeConfigurationOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationOptionsError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationOptionsError {
        DescribeConfigurationOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationOptionsError {
    fn from(err: io::Error) -> DescribeConfigurationOptionsError {
        DescribeConfigurationOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConfigurationOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationOptionsError::TooManyBuckets(ref cause) => cause,
            DescribeConfigurationOptionsError::Validation(ref cause) => cause,
            DescribeConfigurationOptionsError::Credentials(ref err) => err.description(),
            DescribeConfigurationOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationSettings
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationSettingsError {
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationSettingsError {
    pub fn from_body(body: &str) -> DescribeConfigurationSettingsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TooManyBucketsException" => DescribeConfigurationSettingsError::TooManyBuckets(
                    String::from(parsed_error.message),
                ),
                _ => DescribeConfigurationSettingsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeConfigurationSettingsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeConfigurationSettingsError {
    fn from(err: XmlParseError) -> DescribeConfigurationSettingsError {
        let XmlParseError(message) = err;
        DescribeConfigurationSettingsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationSettingsError {
    fn from(err: CredentialsError) -> DescribeConfigurationSettingsError {
        DescribeConfigurationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationSettingsError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationSettingsError {
        DescribeConfigurationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationSettingsError {
    fn from(err: io::Error) -> DescribeConfigurationSettingsError {
        DescribeConfigurationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConfigurationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationSettingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationSettingsError::TooManyBuckets(ref cause) => cause,
            DescribeConfigurationSettingsError::Validation(ref cause) => cause,
            DescribeConfigurationSettingsError::Credentials(ref err) => err.description(),
            DescribeConfigurationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEnvironmentHealth
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentHealthError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>One or more input parameters is not valid. Please correct the input parameters and try the operation again.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentHealthError {
    pub fn from_body(body: &str) -> DescribeEnvironmentHealthError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DescribeEnvironmentHealthError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidRequestException" => DescribeEnvironmentHealthError::InvalidRequest(
                    String::from(parsed_error.message),
                ),
                _ => DescribeEnvironmentHealthError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEnvironmentHealthError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEnvironmentHealthError {
    fn from(err: XmlParseError) -> DescribeEnvironmentHealthError {
        let XmlParseError(message) = err;
        DescribeEnvironmentHealthError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentHealthError {
    fn from(err: CredentialsError) -> DescribeEnvironmentHealthError {
        DescribeEnvironmentHealthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentHealthError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentHealthError {
        DescribeEnvironmentHealthError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentHealthError {
    fn from(err: io::Error) -> DescribeEnvironmentHealthError {
        DescribeEnvironmentHealthError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentHealthError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentHealthError::ElasticBeanstalkService(ref cause) => cause,
            DescribeEnvironmentHealthError::InvalidRequest(ref cause) => cause,
            DescribeEnvironmentHealthError::Validation(ref cause) => cause,
            DescribeEnvironmentHealthError::Credentials(ref err) => err.description(),
            DescribeEnvironmentHealthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentHealthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEnvironmentManagedActionHistory
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentManagedActionHistoryError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentManagedActionHistoryError {
    pub fn from_body(body: &str) -> DescribeEnvironmentManagedActionHistoryError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DescribeEnvironmentManagedActionHistoryError::ElasticBeanstalkService(
                        String::from(parsed_error.message),
                    )
                }
                _ => DescribeEnvironmentManagedActionHistoryError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEnvironmentManagedActionHistoryError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEnvironmentManagedActionHistoryError {
    fn from(err: XmlParseError) -> DescribeEnvironmentManagedActionHistoryError {
        let XmlParseError(message) = err;
        DescribeEnvironmentManagedActionHistoryError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentManagedActionHistoryError {
    fn from(err: CredentialsError) -> DescribeEnvironmentManagedActionHistoryError {
        DescribeEnvironmentManagedActionHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentManagedActionHistoryError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentManagedActionHistoryError {
        DescribeEnvironmentManagedActionHistoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentManagedActionHistoryError {
    fn from(err: io::Error) -> DescribeEnvironmentManagedActionHistoryError {
        DescribeEnvironmentManagedActionHistoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentManagedActionHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentManagedActionHistoryError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentManagedActionHistoryError::ElasticBeanstalkService(ref cause) => {
                cause
            }
            DescribeEnvironmentManagedActionHistoryError::Validation(ref cause) => cause,
            DescribeEnvironmentManagedActionHistoryError::Credentials(ref err) => err.description(),
            DescribeEnvironmentManagedActionHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentManagedActionHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEnvironmentManagedActions
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentManagedActionsError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentManagedActionsError {
    pub fn from_body(body: &str) -> DescribeEnvironmentManagedActionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DescribeEnvironmentManagedActionsError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeEnvironmentManagedActionsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEnvironmentManagedActionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEnvironmentManagedActionsError {
    fn from(err: XmlParseError) -> DescribeEnvironmentManagedActionsError {
        let XmlParseError(message) = err;
        DescribeEnvironmentManagedActionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentManagedActionsError {
    fn from(err: CredentialsError) -> DescribeEnvironmentManagedActionsError {
        DescribeEnvironmentManagedActionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentManagedActionsError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentManagedActionsError {
        DescribeEnvironmentManagedActionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentManagedActionsError {
    fn from(err: io::Error) -> DescribeEnvironmentManagedActionsError {
        DescribeEnvironmentManagedActionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentManagedActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentManagedActionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentManagedActionsError::ElasticBeanstalkService(ref cause) => cause,
            DescribeEnvironmentManagedActionsError::Validation(ref cause) => cause,
            DescribeEnvironmentManagedActionsError::Credentials(ref err) => err.description(),
            DescribeEnvironmentManagedActionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentManagedActionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEnvironmentResources
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentResourcesError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentResourcesError {
    pub fn from_body(body: &str) -> DescribeEnvironmentResourcesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    DescribeEnvironmentResourcesError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribeEnvironmentResourcesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEnvironmentResourcesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEnvironmentResourcesError {
    fn from(err: XmlParseError) -> DescribeEnvironmentResourcesError {
        let XmlParseError(message) = err;
        DescribeEnvironmentResourcesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentResourcesError {
    fn from(err: CredentialsError) -> DescribeEnvironmentResourcesError {
        DescribeEnvironmentResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentResourcesError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentResourcesError {
        DescribeEnvironmentResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentResourcesError {
    fn from(err: io::Error) -> DescribeEnvironmentResourcesError {
        DescribeEnvironmentResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentResourcesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentResourcesError::InsufficientPrivileges(ref cause) => cause,
            DescribeEnvironmentResourcesError::Validation(ref cause) => cause,
            DescribeEnvironmentResourcesError::Credentials(ref err) => err.description(),
            DescribeEnvironmentResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeEnvironmentsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEnvironmentsError {
    pub fn from_body(body: &str) -> DescribeEnvironmentsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeEnvironmentsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEnvironmentsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEnvironmentsError {
    fn from(err: XmlParseError) -> DescribeEnvironmentsError {
        let XmlParseError(message) = err;
        DescribeEnvironmentsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEnvironmentsError {
    fn from(err: CredentialsError) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEnvironmentsError {
    fn from(err: HttpDispatchError) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEnvironmentsError {
    fn from(err: io::Error) -> DescribeEnvironmentsError {
        DescribeEnvironmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEnvironmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEnvironmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEnvironmentsError::Validation(ref cause) => cause,
            DescribeEnvironmentsError::Credentials(ref err) => err.description(),
            DescribeEnvironmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEnvironmentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventsError {
    pub fn from_body(body: &str) -> DescribeEventsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeEventsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeEventsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeEventsError {
    fn from(err: XmlParseError) -> DescribeEventsError {
        let XmlParseError(message) = err;
        DescribeEventsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventsError {
    fn from(err: io::Error) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancesHealth
#[derive(Debug, PartialEq)]
pub enum DescribeInstancesHealthError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>One or more input parameters is not valid. Please correct the input parameters and try the operation again.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstancesHealthError {
    pub fn from_body(body: &str) -> DescribeInstancesHealthError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DescribeInstancesHealthError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidRequestException" => {
                    DescribeInstancesHealthError::InvalidRequest(String::from(parsed_error.message))
                }
                _ => DescribeInstancesHealthError::Unknown(String::from(body)),
            },
            Err(_) => DescribeInstancesHealthError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeInstancesHealthError {
    fn from(err: XmlParseError) -> DescribeInstancesHealthError {
        let XmlParseError(message) = err;
        DescribeInstancesHealthError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeInstancesHealthError {
    fn from(err: CredentialsError) -> DescribeInstancesHealthError {
        DescribeInstancesHealthError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancesHealthError {
    fn from(err: HttpDispatchError) -> DescribeInstancesHealthError {
        DescribeInstancesHealthError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstancesHealthError {
    fn from(err: io::Error) -> DescribeInstancesHealthError {
        DescribeInstancesHealthError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstancesHealthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancesHealthError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancesHealthError::ElasticBeanstalkService(ref cause) => cause,
            DescribeInstancesHealthError::InvalidRequest(ref cause) => cause,
            DescribeInstancesHealthError::Validation(ref cause) => cause,
            DescribeInstancesHealthError::Credentials(ref err) => err.description(),
            DescribeInstancesHealthError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancesHealthError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePlatformVersion
#[derive(Debug, PartialEq)]
pub enum DescribePlatformVersionError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePlatformVersionError {
    pub fn from_body(body: &str) -> DescribePlatformVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    DescribePlatformVersionError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientPrivilegesException" => {
                    DescribePlatformVersionError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => DescribePlatformVersionError::Unknown(String::from(body)),
            },
            Err(_) => DescribePlatformVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribePlatformVersionError {
    fn from(err: XmlParseError) -> DescribePlatformVersionError {
        let XmlParseError(message) = err;
        DescribePlatformVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribePlatformVersionError {
    fn from(err: CredentialsError) -> DescribePlatformVersionError {
        DescribePlatformVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePlatformVersionError {
    fn from(err: HttpDispatchError) -> DescribePlatformVersionError {
        DescribePlatformVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePlatformVersionError {
    fn from(err: io::Error) -> DescribePlatformVersionError {
        DescribePlatformVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePlatformVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePlatformVersionError {
    fn description(&self) -> &str {
        match *self {
            DescribePlatformVersionError::ElasticBeanstalkService(ref cause) => cause,
            DescribePlatformVersionError::InsufficientPrivileges(ref cause) => cause,
            DescribePlatformVersionError::Validation(ref cause) => cause,
            DescribePlatformVersionError::Credentials(ref err) => err.description(),
            DescribePlatformVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePlatformVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAvailableSolutionStacks
#[derive(Debug, PartialEq)]
pub enum ListAvailableSolutionStacksError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAvailableSolutionStacksError {
    pub fn from_body(body: &str) -> ListAvailableSolutionStacksError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListAvailableSolutionStacksError::Unknown(String::from(body)),
            },
            Err(_) => ListAvailableSolutionStacksError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListAvailableSolutionStacksError {
    fn from(err: XmlParseError) -> ListAvailableSolutionStacksError {
        let XmlParseError(message) = err;
        ListAvailableSolutionStacksError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListAvailableSolutionStacksError {
    fn from(err: CredentialsError) -> ListAvailableSolutionStacksError {
        ListAvailableSolutionStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAvailableSolutionStacksError {
    fn from(err: HttpDispatchError) -> ListAvailableSolutionStacksError {
        ListAvailableSolutionStacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAvailableSolutionStacksError {
    fn from(err: io::Error) -> ListAvailableSolutionStacksError {
        ListAvailableSolutionStacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAvailableSolutionStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAvailableSolutionStacksError {
    fn description(&self) -> &str {
        match *self {
            ListAvailableSolutionStacksError::Validation(ref cause) => cause,
            ListAvailableSolutionStacksError::Credentials(ref err) => err.description(),
            ListAvailableSolutionStacksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAvailableSolutionStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPlatformVersions
#[derive(Debug, PartialEq)]
pub enum ListPlatformVersionsError {
    /// <p>A generic service exception has occurred.</p>
    ElasticBeanstalkService(String),
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPlatformVersionsError {
    pub fn from_body(body: &str) -> ListPlatformVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ElasticBeanstalkServiceException" => {
                    ListPlatformVersionsError::ElasticBeanstalkService(String::from(
                        parsed_error.message,
                    ))
                }
                "InsufficientPrivilegesException" => {
                    ListPlatformVersionsError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ListPlatformVersionsError::Unknown(String::from(body)),
            },
            Err(_) => ListPlatformVersionsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListPlatformVersionsError {
    fn from(err: XmlParseError) -> ListPlatformVersionsError {
        let XmlParseError(message) = err;
        ListPlatformVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListPlatformVersionsError {
    fn from(err: CredentialsError) -> ListPlatformVersionsError {
        ListPlatformVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPlatformVersionsError {
    fn from(err: HttpDispatchError) -> ListPlatformVersionsError {
        ListPlatformVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPlatformVersionsError {
    fn from(err: io::Error) -> ListPlatformVersionsError {
        ListPlatformVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPlatformVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPlatformVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListPlatformVersionsError::ElasticBeanstalkService(ref cause) => cause,
            ListPlatformVersionsError::InsufficientPrivileges(ref cause) => cause,
            ListPlatformVersionsError::Validation(ref cause) => cause,
            ListPlatformVersionsError::Credentials(ref err) => err.description(),
            ListPlatformVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPlatformVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>A resource doesn't exist for the specified Amazon Resource Name (ARN).</p>
    ResourceNotFound(String),
    /// <p>The type of the specified Amazon Resource Name (ARN) isn't supported for this operation.</p>
    ResourceTypeNotSupported(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    ListTagsForResourceError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "ResourceNotFoundException" => {
                    ListTagsForResourceError::ResourceNotFound(String::from(parsed_error.message))
                }
                "ResourceTypeNotSupportedException" => {
                    ListTagsForResourceError::ResourceTypeNotSupported(String::from(
                        parsed_error.message,
                    ))
                }
                _ => ListTagsForResourceError::Unknown(String::from(body)),
            },
            Err(_) => ListTagsForResourceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListTagsForResourceError {
    fn from(err: XmlParseError) -> ListTagsForResourceError {
        let XmlParseError(message) = err;
        ListTagsForResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InsufficientPrivileges(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsForResourceError::ResourceTypeNotSupported(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebuildEnvironment
#[derive(Debug, PartialEq)]
pub enum RebuildEnvironmentError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RebuildEnvironmentError {
    pub fn from_body(body: &str) -> RebuildEnvironmentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    RebuildEnvironmentError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => RebuildEnvironmentError::Unknown(String::from(body)),
            },
            Err(_) => RebuildEnvironmentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RebuildEnvironmentError {
    fn from(err: XmlParseError) -> RebuildEnvironmentError {
        let XmlParseError(message) = err;
        RebuildEnvironmentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RebuildEnvironmentError {
    fn from(err: CredentialsError) -> RebuildEnvironmentError {
        RebuildEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebuildEnvironmentError {
    fn from(err: HttpDispatchError) -> RebuildEnvironmentError {
        RebuildEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for RebuildEnvironmentError {
    fn from(err: io::Error) -> RebuildEnvironmentError {
        RebuildEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RebuildEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebuildEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            RebuildEnvironmentError::InsufficientPrivileges(ref cause) => cause,
            RebuildEnvironmentError::Validation(ref cause) => cause,
            RebuildEnvironmentError::Credentials(ref err) => err.description(),
            RebuildEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RebuildEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RequestEnvironmentInfo
#[derive(Debug, PartialEq)]
pub enum RequestEnvironmentInfoError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RequestEnvironmentInfoError {
    pub fn from_body(body: &str) -> RequestEnvironmentInfoError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => RequestEnvironmentInfoError::Unknown(String::from(body)),
            },
            Err(_) => RequestEnvironmentInfoError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RequestEnvironmentInfoError {
    fn from(err: XmlParseError) -> RequestEnvironmentInfoError {
        let XmlParseError(message) = err;
        RequestEnvironmentInfoError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RequestEnvironmentInfoError {
    fn from(err: CredentialsError) -> RequestEnvironmentInfoError {
        RequestEnvironmentInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RequestEnvironmentInfoError {
    fn from(err: HttpDispatchError) -> RequestEnvironmentInfoError {
        RequestEnvironmentInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for RequestEnvironmentInfoError {
    fn from(err: io::Error) -> RequestEnvironmentInfoError {
        RequestEnvironmentInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RequestEnvironmentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RequestEnvironmentInfoError {
    fn description(&self) -> &str {
        match *self {
            RequestEnvironmentInfoError::Validation(ref cause) => cause,
            RequestEnvironmentInfoError::Credentials(ref err) => err.description(),
            RequestEnvironmentInfoError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RequestEnvironmentInfoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestartAppServer
#[derive(Debug, PartialEq)]
pub enum RestartAppServerError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestartAppServerError {
    pub fn from_body(body: &str) -> RestartAppServerError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => RestartAppServerError::Unknown(String::from(body)),
            },
            Err(_) => RestartAppServerError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RestartAppServerError {
    fn from(err: XmlParseError) -> RestartAppServerError {
        let XmlParseError(message) = err;
        RestartAppServerError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RestartAppServerError {
    fn from(err: CredentialsError) -> RestartAppServerError {
        RestartAppServerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestartAppServerError {
    fn from(err: HttpDispatchError) -> RestartAppServerError {
        RestartAppServerError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestartAppServerError {
    fn from(err: io::Error) -> RestartAppServerError {
        RestartAppServerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestartAppServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestartAppServerError {
    fn description(&self) -> &str {
        match *self {
            RestartAppServerError::Validation(ref cause) => cause,
            RestartAppServerError::Credentials(ref err) => err.description(),
            RestartAppServerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RestartAppServerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RetrieveEnvironmentInfo
#[derive(Debug, PartialEq)]
pub enum RetrieveEnvironmentInfoError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RetrieveEnvironmentInfoError {
    pub fn from_body(body: &str) -> RetrieveEnvironmentInfoError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => RetrieveEnvironmentInfoError::Unknown(String::from(body)),
            },
            Err(_) => RetrieveEnvironmentInfoError::Unknown(body.to_string()),
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

impl From<XmlParseError> for RetrieveEnvironmentInfoError {
    fn from(err: XmlParseError) -> RetrieveEnvironmentInfoError {
        let XmlParseError(message) = err;
        RetrieveEnvironmentInfoError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RetrieveEnvironmentInfoError {
    fn from(err: CredentialsError) -> RetrieveEnvironmentInfoError {
        RetrieveEnvironmentInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetrieveEnvironmentInfoError {
    fn from(err: HttpDispatchError) -> RetrieveEnvironmentInfoError {
        RetrieveEnvironmentInfoError::HttpDispatch(err)
    }
}
impl From<io::Error> for RetrieveEnvironmentInfoError {
    fn from(err: io::Error) -> RetrieveEnvironmentInfoError {
        RetrieveEnvironmentInfoError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RetrieveEnvironmentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetrieveEnvironmentInfoError {
    fn description(&self) -> &str {
        match *self {
            RetrieveEnvironmentInfoError::Validation(ref cause) => cause,
            RetrieveEnvironmentInfoError::Credentials(ref err) => err.description(),
            RetrieveEnvironmentInfoError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RetrieveEnvironmentInfoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SwapEnvironmentCNAMEs
#[derive(Debug, PartialEq)]
pub enum SwapEnvironmentCNAMEsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SwapEnvironmentCNAMEsError {
    pub fn from_body(body: &str) -> SwapEnvironmentCNAMEsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SwapEnvironmentCNAMEsError::Unknown(String::from(body)),
            },
            Err(_) => SwapEnvironmentCNAMEsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for SwapEnvironmentCNAMEsError {
    fn from(err: XmlParseError) -> SwapEnvironmentCNAMEsError {
        let XmlParseError(message) = err;
        SwapEnvironmentCNAMEsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SwapEnvironmentCNAMEsError {
    fn from(err: CredentialsError) -> SwapEnvironmentCNAMEsError {
        SwapEnvironmentCNAMEsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SwapEnvironmentCNAMEsError {
    fn from(err: HttpDispatchError) -> SwapEnvironmentCNAMEsError {
        SwapEnvironmentCNAMEsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SwapEnvironmentCNAMEsError {
    fn from(err: io::Error) -> SwapEnvironmentCNAMEsError {
        SwapEnvironmentCNAMEsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SwapEnvironmentCNAMEsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SwapEnvironmentCNAMEsError {
    fn description(&self) -> &str {
        match *self {
            SwapEnvironmentCNAMEsError::Validation(ref cause) => cause,
            SwapEnvironmentCNAMEsError::Credentials(ref err) => err.description(),
            SwapEnvironmentCNAMEsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SwapEnvironmentCNAMEsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateEnvironment
#[derive(Debug, PartialEq)]
pub enum TerminateEnvironmentError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TerminateEnvironmentError {
    pub fn from_body(body: &str) -> TerminateEnvironmentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    TerminateEnvironmentError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => TerminateEnvironmentError::Unknown(String::from(body)),
            },
            Err(_) => TerminateEnvironmentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for TerminateEnvironmentError {
    fn from(err: XmlParseError) -> TerminateEnvironmentError {
        let XmlParseError(message) = err;
        TerminateEnvironmentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for TerminateEnvironmentError {
    fn from(err: CredentialsError) -> TerminateEnvironmentError {
        TerminateEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateEnvironmentError {
    fn from(err: HttpDispatchError) -> TerminateEnvironmentError {
        TerminateEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateEnvironmentError {
    fn from(err: io::Error) -> TerminateEnvironmentError {
        TerminateEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            TerminateEnvironmentError::InsufficientPrivileges(ref cause) => cause,
            TerminateEnvironmentError::Validation(ref cause) => cause,
            TerminateEnvironmentError::Credentials(ref err) => err.description(),
            TerminateEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TerminateEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateApplicationError {
    pub fn from_body(body: &str) -> UpdateApplicationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UpdateApplicationError::Unknown(String::from(body)),
            },
            Err(_) => UpdateApplicationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateApplicationError {
    fn from(err: XmlParseError) -> UpdateApplicationError {
        let XmlParseError(message) = err;
        UpdateApplicationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplicationResourceLifecycle
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationResourceLifecycleError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateApplicationResourceLifecycleError {
    pub fn from_body(body: &str) -> UpdateApplicationResourceLifecycleError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    UpdateApplicationResourceLifecycleError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                _ => UpdateApplicationResourceLifecycleError::Unknown(String::from(body)),
            },
            Err(_) => UpdateApplicationResourceLifecycleError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateApplicationResourceLifecycleError {
    fn from(err: XmlParseError) -> UpdateApplicationResourceLifecycleError {
        let XmlParseError(message) = err;
        UpdateApplicationResourceLifecycleError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationResourceLifecycleError {
    fn from(err: CredentialsError) -> UpdateApplicationResourceLifecycleError {
        UpdateApplicationResourceLifecycleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationResourceLifecycleError {
    fn from(err: HttpDispatchError) -> UpdateApplicationResourceLifecycleError {
        UpdateApplicationResourceLifecycleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationResourceLifecycleError {
    fn from(err: io::Error) -> UpdateApplicationResourceLifecycleError {
        UpdateApplicationResourceLifecycleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationResourceLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationResourceLifecycleError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationResourceLifecycleError::InsufficientPrivileges(ref cause) => cause,
            UpdateApplicationResourceLifecycleError::Validation(ref cause) => cause,
            UpdateApplicationResourceLifecycleError::Credentials(ref err) => err.description(),
            UpdateApplicationResourceLifecycleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationResourceLifecycleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplicationVersion
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationVersionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateApplicationVersionError {
    pub fn from_body(body: &str) -> UpdateApplicationVersionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UpdateApplicationVersionError::Unknown(String::from(body)),
            },
            Err(_) => UpdateApplicationVersionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateApplicationVersionError {
    fn from(err: XmlParseError) -> UpdateApplicationVersionError {
        let XmlParseError(message) = err;
        UpdateApplicationVersionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationVersionError {
    fn from(err: CredentialsError) -> UpdateApplicationVersionError {
        UpdateApplicationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationVersionError {
    fn from(err: HttpDispatchError) -> UpdateApplicationVersionError {
        UpdateApplicationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationVersionError {
    fn from(err: io::Error) -> UpdateApplicationVersionError {
        UpdateApplicationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationVersionError::Validation(ref cause) => cause,
            UpdateApplicationVersionError::Credentials(ref err) => err.description(),
            UpdateApplicationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfigurationTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationTemplateError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateConfigurationTemplateError {
    pub fn from_body(body: &str) -> UpdateConfigurationTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    UpdateConfigurationTemplateError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyBucketsException" => UpdateConfigurationTemplateError::TooManyBuckets(
                    String::from(parsed_error.message),
                ),
                _ => UpdateConfigurationTemplateError::Unknown(String::from(body)),
            },
            Err(_) => UpdateConfigurationTemplateError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateConfigurationTemplateError {
    fn from(err: XmlParseError) -> UpdateConfigurationTemplateError {
        let XmlParseError(message) = err;
        UpdateConfigurationTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateConfigurationTemplateError {
    fn from(err: CredentialsError) -> UpdateConfigurationTemplateError {
        UpdateConfigurationTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateConfigurationTemplateError {
    fn from(err: HttpDispatchError) -> UpdateConfigurationTemplateError {
        UpdateConfigurationTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateConfigurationTemplateError {
    fn from(err: io::Error) -> UpdateConfigurationTemplateError {
        UpdateConfigurationTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateConfigurationTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationTemplateError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationTemplateError::InsufficientPrivileges(ref cause) => cause,
            UpdateConfigurationTemplateError::TooManyBuckets(ref cause) => cause,
            UpdateConfigurationTemplateError::Validation(ref cause) => cause,
            UpdateConfigurationTemplateError::Credentials(ref err) => err.description(),
            UpdateConfigurationTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateConfigurationTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateEnvironmentError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEnvironmentError {
    pub fn from_body(body: &str) -> UpdateEnvironmentError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    UpdateEnvironmentError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyBucketsException" => {
                    UpdateEnvironmentError::TooManyBuckets(String::from(parsed_error.message))
                }
                _ => UpdateEnvironmentError::Unknown(String::from(body)),
            },
            Err(_) => UpdateEnvironmentError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateEnvironmentError {
    fn from(err: XmlParseError) -> UpdateEnvironmentError {
        let XmlParseError(message) = err;
        UpdateEnvironmentError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateEnvironmentError {
    fn from(err: CredentialsError) -> UpdateEnvironmentError {
        UpdateEnvironmentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEnvironmentError {
    fn from(err: HttpDispatchError) -> UpdateEnvironmentError {
        UpdateEnvironmentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEnvironmentError {
    fn from(err: io::Error) -> UpdateEnvironmentError {
        UpdateEnvironmentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEnvironmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateEnvironmentError::InsufficientPrivileges(ref cause) => cause,
            UpdateEnvironmentError::TooManyBuckets(ref cause) => cause,
            UpdateEnvironmentError::Validation(ref cause) => cause,
            UpdateEnvironmentError::Credentials(ref err) => err.description(),
            UpdateEnvironmentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEnvironmentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTagsForResource
#[derive(Debug, PartialEq)]
pub enum UpdateTagsForResourceError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>Unable to perform the specified operation because another operation that effects an element in this activity is already in progress.</p>
    OperationInProgress(String),
    /// <p>A resource doesn't exist for the specified Amazon Resource Name (ARN).</p>
    ResourceNotFound(String),
    /// <p>The type of the specified Amazon Resource Name (ARN) isn't supported for this operation.</p>
    ResourceTypeNotSupported(String),
    /// <p>The number of tags in the resource would exceed the number of tags that each resource can have.</p> <p>To calculate this, the operation considers both the number of tags the resource already has and the tags this operation would add if it succeeded.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTagsForResourceError {
    pub fn from_body(body: &str) -> UpdateTagsForResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    UpdateTagsForResourceError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressFailure" => UpdateTagsForResourceError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "ResourceNotFoundException" => {
                    UpdateTagsForResourceError::ResourceNotFound(String::from(parsed_error.message))
                }
                "ResourceTypeNotSupportedException" => {
                    UpdateTagsForResourceError::ResourceTypeNotSupported(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyTagsException" => {
                    UpdateTagsForResourceError::TooManyTags(String::from(parsed_error.message))
                }
                _ => UpdateTagsForResourceError::Unknown(String::from(body)),
            },
            Err(_) => UpdateTagsForResourceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateTagsForResourceError {
    fn from(err: XmlParseError) -> UpdateTagsForResourceError {
        let XmlParseError(message) = err;
        UpdateTagsForResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateTagsForResourceError {
    fn from(err: CredentialsError) -> UpdateTagsForResourceError {
        UpdateTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTagsForResourceError {
    fn from(err: HttpDispatchError) -> UpdateTagsForResourceError {
        UpdateTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTagsForResourceError {
    fn from(err: io::Error) -> UpdateTagsForResourceError {
        UpdateTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateTagsForResourceError::InsufficientPrivileges(ref cause) => cause,
            UpdateTagsForResourceError::OperationInProgress(ref cause) => cause,
            UpdateTagsForResourceError::ResourceNotFound(ref cause) => cause,
            UpdateTagsForResourceError::ResourceTypeNotSupported(ref cause) => cause,
            UpdateTagsForResourceError::TooManyTags(ref cause) => cause,
            UpdateTagsForResourceError::Validation(ref cause) => cause,
            UpdateTagsForResourceError::Credentials(ref err) => err.description(),
            UpdateTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ValidateConfigurationSettings
#[derive(Debug, PartialEq)]
pub enum ValidateConfigurationSettingsError {
    /// <p>The specified account does not have sufficient privileges for one or more AWS services.</p>
    InsufficientPrivileges(String),
    /// <p>The specified account has reached its limit of Amazon S3 buckets.</p>
    TooManyBuckets(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ValidateConfigurationSettingsError {
    pub fn from_body(body: &str) -> ValidateConfigurationSettingsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientPrivilegesException" => {
                    ValidateConfigurationSettingsError::InsufficientPrivileges(String::from(
                        parsed_error.message,
                    ))
                }
                "TooManyBucketsException" => ValidateConfigurationSettingsError::TooManyBuckets(
                    String::from(parsed_error.message),
                ),
                _ => ValidateConfigurationSettingsError::Unknown(String::from(body)),
            },
            Err(_) => ValidateConfigurationSettingsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ValidateConfigurationSettingsError {
    fn from(err: XmlParseError) -> ValidateConfigurationSettingsError {
        let XmlParseError(message) = err;
        ValidateConfigurationSettingsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ValidateConfigurationSettingsError {
    fn from(err: CredentialsError) -> ValidateConfigurationSettingsError {
        ValidateConfigurationSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ValidateConfigurationSettingsError {
    fn from(err: HttpDispatchError) -> ValidateConfigurationSettingsError {
        ValidateConfigurationSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ValidateConfigurationSettingsError {
    fn from(err: io::Error) -> ValidateConfigurationSettingsError {
        ValidateConfigurationSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ValidateConfigurationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ValidateConfigurationSettingsError {
    fn description(&self) -> &str {
        match *self {
            ValidateConfigurationSettingsError::InsufficientPrivileges(ref cause) => cause,
            ValidateConfigurationSettingsError::TooManyBuckets(ref cause) => cause,
            ValidateConfigurationSettingsError::Validation(ref cause) => cause,
            ValidateConfigurationSettingsError::Credentials(ref err) => err.description(),
            ValidateConfigurationSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ValidateConfigurationSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Elastic Beanstalk API. Elastic Beanstalk clients implement this trait.
pub trait ElasticBeanstalk {
    /// <p>Cancels in-progress environment configuration update or application version deployment.</p>
    fn abort_environment_update(
        &self,
        input: AbortEnvironmentUpdateMessage,
    ) -> RusotoFuture<(), AbortEnvironmentUpdateError>;

    /// <p>Applies a scheduled managed action immediately. A managed action can be applied only if its status is <code>Scheduled</code>. Get the status and action ID of a managed action with <a>DescribeEnvironmentManagedActions</a>.</p>
    fn apply_environment_managed_action(
        &self,
        input: ApplyEnvironmentManagedActionRequest,
    ) -> RusotoFuture<ApplyEnvironmentManagedActionResult, ApplyEnvironmentManagedActionError>;

    /// <p>Checks if the specified CNAME is available.</p>
    fn check_dns_availability(
        &self,
        input: CheckDNSAvailabilityMessage,
    ) -> RusotoFuture<CheckDNSAvailabilityResultMessage, CheckDNSAvailabilityError>;

    /// <p>Create or update a group of environments that each run a separate component of a single application. Takes a list of version labels that specify application source bundles for each of the environments to create or update. The name of each environment and other required information must be included in the source bundles in an environment manifest named <code>env.yaml</code>. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-mgmt-compose.html">Compose Environments</a> for details.</p>
    fn compose_environments(
        &self,
        input: ComposeEnvironmentsMessage,
    ) -> RusotoFuture<EnvironmentDescriptionsMessage, ComposeEnvironmentsError>;

    /// <p> Creates an application that has one configuration template named <code>default</code> and no application versions. </p>
    fn create_application(
        &self,
        input: CreateApplicationMessage,
    ) -> RusotoFuture<ApplicationDescriptionMessage, CreateApplicationError>;

    /// <p><p>Creates an application version for the specified application. You can create an application version from a source bundle in Amazon S3, a commit in AWS CodeCommit, or the output of an AWS CodeBuild build as follows:</p> <p>Specify a commit in an AWS CodeCommit repository with <code>SourceBuildInformation</code>.</p> <p>Specify a build in an AWS CodeBuild with <code>SourceBuildInformation</code> and <code>BuildConfiguration</code>.</p> <p>Specify a source bundle in S3 with <code>SourceBundle</code> </p> <p>Omit both <code>SourceBuildInformation</code> and <code>SourceBundle</code> to use the default sample application.</p> <note> <p>Once you create an application version with a specified Amazon S3 bucket and key location, you cannot change that Amazon S3 location. If you change the Amazon S3 location, you receive an exception when you attempt to launch an environment from the application version.</p> </note></p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionMessage, CreateApplicationVersionError>;

    /// <p><p>Creates a configuration template. Templates are associated with a specific application and are used to deploy different versions of the application with the same configuration settings.</p> <p>Related Topics</p> <ul> <li> <p> <a>DescribeConfigurationOptions</a> </p> </li> <li> <p> <a>DescribeConfigurationSettings</a> </p> </li> <li> <p> <a>ListAvailableSolutionStacks</a> </p> </li> </ul></p>
    fn create_configuration_template(
        &self,
        input: CreateConfigurationTemplateMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescription, CreateConfigurationTemplateError>;

    /// <p>Launches an environment for the specified application using the specified configuration.</p>
    fn create_environment(
        &self,
        input: CreateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, CreateEnvironmentError>;

    /// <p>Create a new version of your custom platform.</p>
    fn create_platform_version(
        &self,
        input: CreatePlatformVersionRequest,
    ) -> RusotoFuture<CreatePlatformVersionResult, CreatePlatformVersionError>;

    /// <p>Creates a bucket in Amazon S3 to store application versions, logs, and other files used by Elastic Beanstalk environments. The Elastic Beanstalk console and EB CLI call this API the first time you create an environment in a region. If the storage location already exists, <code>CreateStorageLocation</code> still returns the bucket name but does not create a new bucket.</p>
    fn create_storage_location(
        &self,
    ) -> RusotoFuture<CreateStorageLocationResultMessage, CreateStorageLocationError>;

    /// <p><p>Deletes the specified application along with all associated versions and configurations. The application versions will not be deleted from your Amazon S3 bucket.</p> <note> <p>You cannot delete an application that has a running environment.</p> </note></p>
    fn delete_application(
        &self,
        input: DeleteApplicationMessage,
    ) -> RusotoFuture<(), DeleteApplicationError>;

    /// <p><p>Deletes the specified version from the specified application.</p> <note> <p>You cannot delete an application version that is associated with a running environment.</p> </note></p>
    fn delete_application_version(
        &self,
        input: DeleteApplicationVersionMessage,
    ) -> RusotoFuture<(), DeleteApplicationVersionError>;

    /// <p><p>Deletes the specified configuration template.</p> <note> <p>When you launch an environment using a configuration template, the environment gets a copy of the template. You can delete or modify the environment&#39;s copy of the template without affecting the running environment.</p> </note></p>
    fn delete_configuration_template(
        &self,
        input: DeleteConfigurationTemplateMessage,
    ) -> RusotoFuture<(), DeleteConfigurationTemplateError>;

    /// <p>Deletes the draft configuration associated with the running environment.</p> <p>Updating a running environment with any configuration changes creates a draft configuration set. You can get the draft configuration using <a>DescribeConfigurationSettings</a> while the update is in progress or if the update fails. The <code>DeploymentStatus</code> for the draft configuration indicates whether the deployment is in process or has failed. The draft configuration remains in existence until it is deleted with this action.</p>
    fn delete_environment_configuration(
        &self,
        input: DeleteEnvironmentConfigurationMessage,
    ) -> RusotoFuture<(), DeleteEnvironmentConfigurationError>;

    /// <p>Deletes the specified version of a custom platform.</p>
    fn delete_platform_version(
        &self,
        input: DeletePlatformVersionRequest,
    ) -> RusotoFuture<DeletePlatformVersionResult, DeletePlatformVersionError>;

    /// <p>Returns attributes related to AWS Elastic Beanstalk that are associated with the calling AWS account.</p> <p>The result currently has one set of attributes—resource quotas.</p>
    fn describe_account_attributes(
        &self,
    ) -> RusotoFuture<DescribeAccountAttributesResult, DescribeAccountAttributesError>;

    /// <p>Retrieve a list of application versions.</p>
    fn describe_application_versions(
        &self,
        input: DescribeApplicationVersionsMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionsMessage, DescribeApplicationVersionsError>;

    /// <p>Returns the descriptions of existing applications.</p>
    fn describe_applications(
        &self,
        input: DescribeApplicationsMessage,
    ) -> RusotoFuture<ApplicationDescriptionsMessage, DescribeApplicationsError>;

    /// <p>Describes the configuration options that are used in a particular configuration template or environment, or that a specified solution stack defines. The description includes the values the options, their default values, and an indication of the required action on a running environment if an option value is changed.</p>
    fn describe_configuration_options(
        &self,
        input: DescribeConfigurationOptionsMessage,
    ) -> RusotoFuture<ConfigurationOptionsDescription, DescribeConfigurationOptionsError>;

    /// <p><p>Returns a description of the settings for the specified configuration set, that is, either a configuration template or the configuration set associated with a running environment.</p> <p>When describing the settings for the configuration set associated with a running environment, it is possible to receive two sets of setting descriptions. One is the deployed configuration set, and the other is a draft configuration of an environment that is either in the process of deployment or that failed to deploy.</p> <p>Related Topics</p> <ul> <li> <p> <a>DeleteEnvironmentConfiguration</a> </p> </li> </ul></p>
    fn describe_configuration_settings(
        &self,
        input: DescribeConfigurationSettingsMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescriptions, DescribeConfigurationSettingsError>;

    /// <p>Returns information about the overall health of the specified environment. The <b>DescribeEnvironmentHealth</b> operation is only available with AWS Elastic Beanstalk Enhanced Health.</p>
    fn describe_environment_health(
        &self,
        input: DescribeEnvironmentHealthRequest,
    ) -> RusotoFuture<DescribeEnvironmentHealthResult, DescribeEnvironmentHealthError>;

    /// <p>Lists an environment's completed and failed managed actions.</p>
    fn describe_environment_managed_action_history(
        &self,
        input: DescribeEnvironmentManagedActionHistoryRequest,
    ) -> RusotoFuture<
        DescribeEnvironmentManagedActionHistoryResult,
        DescribeEnvironmentManagedActionHistoryError,
    >;

    /// <p>Lists an environment's upcoming and in-progress managed actions.</p>
    fn describe_environment_managed_actions(
        &self,
        input: DescribeEnvironmentManagedActionsRequest,
    ) -> RusotoFuture<DescribeEnvironmentManagedActionsResult, DescribeEnvironmentManagedActionsError>;

    /// <p>Returns AWS resources for this environment.</p>
    fn describe_environment_resources(
        &self,
        input: DescribeEnvironmentResourcesMessage,
    ) -> RusotoFuture<EnvironmentResourceDescriptionsMessage, DescribeEnvironmentResourcesError>;

    /// <p>Returns descriptions for existing environments.</p>
    fn describe_environments(
        &self,
        input: DescribeEnvironmentsMessage,
    ) -> RusotoFuture<EnvironmentDescriptionsMessage, DescribeEnvironmentsError>;

    /// <p><p>Returns list of event descriptions matching criteria up to the last 6 weeks.</p> <note> <p>This action returns the most recent 1,000 events from the specified <code>NextToken</code>.</p> </note></p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventDescriptionsMessage, DescribeEventsError>;

    /// <p>Retrives detailed information about the health of instances in your AWS Elastic Beanstalk. This operation requires <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced.html">enhanced health reporting</a>.</p>
    fn describe_instances_health(
        &self,
        input: DescribeInstancesHealthRequest,
    ) -> RusotoFuture<DescribeInstancesHealthResult, DescribeInstancesHealthError>;

    /// <p>Describes the version of the platform.</p>
    fn describe_platform_version(
        &self,
        input: DescribePlatformVersionRequest,
    ) -> RusotoFuture<DescribePlatformVersionResult, DescribePlatformVersionError>;

    /// <p>Returns a list of the available solution stack names, with the public version first and then in reverse chronological order.</p>
    fn list_available_solution_stacks(
        &self,
    ) -> RusotoFuture<ListAvailableSolutionStacksResultMessage, ListAvailableSolutionStacksError>;

    /// <p>Lists the available platforms.</p>
    fn list_platform_versions(
        &self,
        input: ListPlatformVersionsRequest,
    ) -> RusotoFuture<ListPlatformVersionsResult, ListPlatformVersionsError>;

    /// <p>Returns the tags applied to an AWS Elastic Beanstalk resource. The response contains a list of tag key-value pairs.</p> <p>Currently, Elastic Beanstalk only supports tagging of Elastic Beanstalk environments. For details about environment tagging, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/using-features.tagging.html">Tagging Resources in Your Elastic Beanstalk Environment</a>.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<ResourceTagsDescriptionMessage, ListTagsForResourceError>;

    /// <p>Deletes and recreates all of the AWS resources (for example: the Auto Scaling group, load balancer, etc.) for a specified environment and forces a restart.</p>
    fn rebuild_environment(
        &self,
        input: RebuildEnvironmentMessage,
    ) -> RusotoFuture<(), RebuildEnvironmentError>;

    /// <p><p>Initiates a request to compile the specified type of information of the deployed environment.</p> <p> Setting the <code>InfoType</code> to <code>tail</code> compiles the last lines from the application server log files of every Amazon EC2 instance in your environment. </p> <p> Setting the <code>InfoType</code> to <code>bundle</code> compresses the application server log files for every Amazon EC2 instance into a <code>.zip</code> file. Legacy and .NET containers do not support bundle logs. </p> <p> Use <a>RetrieveEnvironmentInfo</a> to obtain the set of logs. </p> <p>Related Topics</p> <ul> <li> <p> <a>RetrieveEnvironmentInfo</a> </p> </li> </ul></p>
    fn request_environment_info(
        &self,
        input: RequestEnvironmentInfoMessage,
    ) -> RusotoFuture<(), RequestEnvironmentInfoError>;

    /// <p>Causes the environment to restart the application container server running on each Amazon EC2 instance.</p>
    fn restart_app_server(
        &self,
        input: RestartAppServerMessage,
    ) -> RusotoFuture<(), RestartAppServerError>;

    /// <p><p>Retrieves the compiled information from a <a>RequestEnvironmentInfo</a> request.</p> <p>Related Topics</p> <ul> <li> <p> <a>RequestEnvironmentInfo</a> </p> </li> </ul></p>
    fn retrieve_environment_info(
        &self,
        input: RetrieveEnvironmentInfoMessage,
    ) -> RusotoFuture<RetrieveEnvironmentInfoResultMessage, RetrieveEnvironmentInfoError>;

    /// <p>Swaps the CNAMEs of two environments.</p>
    fn swap_environment_cnam_es(
        &self,
        input: SwapEnvironmentCNAMEsMessage,
    ) -> RusotoFuture<(), SwapEnvironmentCNAMEsError>;

    /// <p>Terminates the specified environment.</p>
    fn terminate_environment(
        &self,
        input: TerminateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, TerminateEnvironmentError>;

    /// <p><p>Updates the specified application to have the specified properties.</p> <note> <p>If a property (for example, <code>description</code>) is not provided, the value remains unchanged. To clear these properties, specify an empty string.</p> </note></p>
    fn update_application(
        &self,
        input: UpdateApplicationMessage,
    ) -> RusotoFuture<ApplicationDescriptionMessage, UpdateApplicationError>;

    /// <p>Modifies lifecycle settings for an application.</p>
    fn update_application_resource_lifecycle(
        &self,
        input: UpdateApplicationResourceLifecycleMessage,
    ) -> RusotoFuture<
        ApplicationResourceLifecycleDescriptionMessage,
        UpdateApplicationResourceLifecycleError,
    >;

    /// <p><p>Updates the specified application version to have the specified properties.</p> <note> <p>If a property (for example, <code>description</code>) is not provided, the value remains unchanged. To clear properties, specify an empty string.</p> </note></p>
    fn update_application_version(
        &self,
        input: UpdateApplicationVersionMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionMessage, UpdateApplicationVersionError>;

    /// <p><p>Updates the specified configuration template to have the specified properties or configuration option values.</p> <note> <p>If a property (for example, <code>ApplicationName</code>) is not provided, its value remains unchanged. To clear such properties, specify an empty string.</p> </note> <p>Related Topics</p> <ul> <li> <p> <a>DescribeConfigurationOptions</a> </p> </li> </ul></p>
    fn update_configuration_template(
        &self,
        input: UpdateConfigurationTemplateMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescription, UpdateConfigurationTemplateError>;

    /// <p>Updates the environment description, deploys a new application version, updates the configuration settings to an entirely new configuration template, or updates select configuration option values in the running environment.</p> <p> Attempting to update both the release and configuration is not allowed and AWS Elastic Beanstalk returns an <code>InvalidParameterCombination</code> error. </p> <p> When updating the configuration settings to a new template or individual settings, a draft configuration is created and <a>DescribeConfigurationSettings</a> for this environment returns two setting descriptions with different <code>DeploymentStatus</code> values. </p>
    fn update_environment(
        &self,
        input: UpdateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, UpdateEnvironmentError>;

    /// <p>Update the list of tags applied to an AWS Elastic Beanstalk resource. Two lists can be passed: <code>TagsToAdd</code> for tags to add or update, and <code>TagsToRemove</code>.</p> <p>Currently, Elastic Beanstalk only supports tagging of Elastic Beanstalk environments. For details about environment tagging, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/using-features.tagging.html">Tagging Resources in Your Elastic Beanstalk Environment</a>.</p> <p>If you create a custom IAM user policy to control permission to this operation, specify one of the following two virtual actions (or both) instead of the API operation name:</p> <dl> <dt>elasticbeanstalk:AddTags</dt> <dd> <p>Controls permission to call <code>UpdateTagsForResource</code> and pass a list of tags to add in the <code>TagsToAdd</code> parameter.</p> </dd> <dt>elasticbeanstalk:RemoveTags</dt> <dd> <p>Controls permission to call <code>UpdateTagsForResource</code> and pass a list of tag keys to remove in the <code>TagsToRemove</code> parameter.</p> </dd> </dl> <p>For details about creating a custom user policy, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/AWSHowTo.iam.managed-policies.html#AWSHowTo.iam.policies">Creating a Custom User Policy</a>.</p>
    fn update_tags_for_resource(
        &self,
        input: UpdateTagsForResourceMessage,
    ) -> RusotoFuture<(), UpdateTagsForResourceError>;

    /// <p>Takes a set of configuration settings and either a configuration template or environment, and determines whether those values are valid.</p> <p>This action returns a list of messages indicating any errors or warnings associated with the selection of option values.</p>
    fn validate_configuration_settings(
        &self,
        input: ValidateConfigurationSettingsMessage,
    ) -> RusotoFuture<ConfigurationSettingsValidationMessages, ValidateConfigurationSettingsError>;
}
/// A client for the Elastic Beanstalk API.
pub struct ElasticBeanstalkClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ElasticBeanstalkClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ElasticBeanstalkClient {
        ElasticBeanstalkClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ElasticBeanstalkClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ElasticBeanstalkClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ElasticBeanstalk for ElasticBeanstalkClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Cancels in-progress environment configuration update or application version deployment.</p>
    fn abort_environment_update(
        &self,
        input: AbortEnvironmentUpdateMessage,
    ) -> RusotoFuture<(), AbortEnvironmentUpdateError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AbortEnvironmentUpdate");
        params.put("Version", "2010-12-01");
        AbortEnvironmentUpdateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AbortEnvironmentUpdateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Applies a scheduled managed action immediately. A managed action can be applied only if its status is <code>Scheduled</code>. Get the status and action ID of a managed action with <a>DescribeEnvironmentManagedActions</a>.</p>
    fn apply_environment_managed_action(
        &self,
        input: ApplyEnvironmentManagedActionRequest,
    ) -> RusotoFuture<ApplyEnvironmentManagedActionResult, ApplyEnvironmentManagedActionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ApplyEnvironmentManagedAction");
        params.put("Version", "2010-12-01");
        ApplyEnvironmentManagedActionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ApplyEnvironmentManagedActionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplyEnvironmentManagedActionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplyEnvironmentManagedActionResultDeserializer::deserialize(
                            "ApplyEnvironmentManagedActionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Checks if the specified CNAME is available.</p>
    fn check_dns_availability(
        &self,
        input: CheckDNSAvailabilityMessage,
    ) -> RusotoFuture<CheckDNSAvailabilityResultMessage, CheckDNSAvailabilityError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CheckDNSAvailability");
        params.put("Version", "2010-12-01");
        CheckDNSAvailabilityMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CheckDNSAvailabilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CheckDNSAvailabilityResultMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CheckDNSAvailabilityResultMessageDeserializer::deserialize(
                        "CheckDNSAvailabilityResult",
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

    /// <p>Create or update a group of environments that each run a separate component of a single application. Takes a list of version labels that specify application source bundles for each of the environments to create or update. The name of each environment and other required information must be included in the source bundles in an environment manifest named <code>env.yaml</code>. See <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-mgmt-compose.html">Compose Environments</a> for details.</p>
    fn compose_environments(
        &self,
        input: ComposeEnvironmentsMessage,
    ) -> RusotoFuture<EnvironmentDescriptionsMessage, ComposeEnvironmentsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ComposeEnvironments");
        params.put("Version", "2010-12-01");
        ComposeEnvironmentsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ComposeEnvironmentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnvironmentDescriptionsMessageDeserializer::deserialize(
                        "ComposeEnvironmentsResult",
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

    /// <p> Creates an application that has one configuration template named <code>default</code> and no application versions. </p>
    fn create_application(
        &self,
        input: CreateApplicationMessage,
    ) -> RusotoFuture<ApplicationDescriptionMessage, CreateApplicationError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateApplication");
        params.put("Version", "2010-12-01");
        CreateApplicationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ApplicationDescriptionMessageDeserializer::deserialize(
                        "CreateApplicationResult",
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

    /// <p><p>Creates an application version for the specified application. You can create an application version from a source bundle in Amazon S3, a commit in AWS CodeCommit, or the output of an AWS CodeBuild build as follows:</p> <p>Specify a commit in an AWS CodeCommit repository with <code>SourceBuildInformation</code>.</p> <p>Specify a build in an AWS CodeBuild with <code>SourceBuildInformation</code> and <code>BuildConfiguration</code>.</p> <p>Specify a source bundle in S3 with <code>SourceBundle</code> </p> <p>Omit both <code>SourceBuildInformation</code> and <code>SourceBundle</code> to use the default sample application.</p> <note> <p>Once you create an application version with a specified Amazon S3 bucket and key location, you cannot change that Amazon S3 location. If you change the Amazon S3 location, you receive an exception when you attempt to launch an environment from the application version.</p> </note></p>
    fn create_application_version(
        &self,
        input: CreateApplicationVersionMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionMessage, CreateApplicationVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateApplicationVersion");
        params.put("Version", "2010-12-01");
        CreateApplicationVersionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationVersionDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplicationVersionDescriptionMessageDeserializer::deserialize(
                            "CreateApplicationVersionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a configuration template. Templates are associated with a specific application and are used to deploy different versions of the application with the same configuration settings.</p> <p>Related Topics</p> <ul> <li> <p> <a>DescribeConfigurationOptions</a> </p> </li> <li> <p> <a>DescribeConfigurationSettings</a> </p> </li> <li> <p> <a>ListAvailableSolutionStacks</a> </p> </li> </ul></p>
    fn create_configuration_template(
        &self,
        input: CreateConfigurationTemplateMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescription, CreateConfigurationTemplateError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateConfigurationTemplate");
        params.put("Version", "2010-12-01");
        CreateConfigurationTemplateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigurationSettingsDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfigurationSettingsDescriptionDeserializer::deserialize(
                        "CreateConfigurationTemplateResult",
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

    /// <p>Launches an environment for the specified application using the specified configuration.</p>
    fn create_environment(
        &self,
        input: CreateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, CreateEnvironmentError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateEnvironment");
        params.put("Version", "2010-12-01");
        CreateEnvironmentMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnvironmentDescriptionDeserializer::deserialize(
                        "CreateEnvironmentResult",
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

    /// <p>Create a new version of your custom platform.</p>
    fn create_platform_version(
        &self,
        input: CreatePlatformVersionRequest,
    ) -> RusotoFuture<CreatePlatformVersionResult, CreatePlatformVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreatePlatformVersion");
        params.put("Version", "2010-12-01");
        CreatePlatformVersionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePlatformVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreatePlatformVersionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreatePlatformVersionResultDeserializer::deserialize(
                        "CreatePlatformVersionResult",
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

    /// <p>Creates a bucket in Amazon S3 to store application versions, logs, and other files used by Elastic Beanstalk environments. The Elastic Beanstalk console and EB CLI call this API the first time you create an environment in a region. If the storage location already exists, <code>CreateStorageLocation</code> still returns the bucket name but does not create a new bucket.</p>
    fn create_storage_location(
        &self,
    ) -> RusotoFuture<CreateStorageLocationResultMessage, CreateStorageLocationError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStorageLocation");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStorageLocationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStorageLocationResultMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateStorageLocationResultMessageDeserializer::deserialize(
                        "CreateStorageLocationResult",
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

    /// <p><p>Deletes the specified application along with all associated versions and configurations. The application versions will not be deleted from your Amazon S3 bucket.</p> <note> <p>You cannot delete an application that has a running environment.</p> </note></p>
    fn delete_application(
        &self,
        input: DeleteApplicationMessage,
    ) -> RusotoFuture<(), DeleteApplicationError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteApplication");
        params.put("Version", "2010-12-01");
        DeleteApplicationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes the specified version from the specified application.</p> <note> <p>You cannot delete an application version that is associated with a running environment.</p> </note></p>
    fn delete_application_version(
        &self,
        input: DeleteApplicationVersionMessage,
    ) -> RusotoFuture<(), DeleteApplicationVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteApplicationVersion");
        params.put("Version", "2010-12-01");
        DeleteApplicationVersionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes the specified configuration template.</p> <note> <p>When you launch an environment using a configuration template, the environment gets a copy of the template. You can delete or modify the environment&#39;s copy of the template without affecting the running environment.</p> </note></p>
    fn delete_configuration_template(
        &self,
        input: DeleteConfigurationTemplateMessage,
    ) -> RusotoFuture<(), DeleteConfigurationTemplateError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteConfigurationTemplate");
        params.put("Version", "2010-12-01");
        DeleteConfigurationTemplateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the draft configuration associated with the running environment.</p> <p>Updating a running environment with any configuration changes creates a draft configuration set. You can get the draft configuration using <a>DescribeConfigurationSettings</a> while the update is in progress or if the update fails. The <code>DeploymentStatus</code> for the draft configuration indicates whether the deployment is in process or has failed. The draft configuration remains in existence until it is deleted with this action.</p>
    fn delete_environment_configuration(
        &self,
        input: DeleteEnvironmentConfigurationMessage,
    ) -> RusotoFuture<(), DeleteEnvironmentConfigurationError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteEnvironmentConfiguration");
        params.put("Version", "2010-12-01");
        DeleteEnvironmentConfigurationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEnvironmentConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified version of a custom platform.</p>
    fn delete_platform_version(
        &self,
        input: DeletePlatformVersionRequest,
    ) -> RusotoFuture<DeletePlatformVersionResult, DeletePlatformVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeletePlatformVersion");
        params.put("Version", "2010-12-01");
        DeletePlatformVersionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePlatformVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeletePlatformVersionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeletePlatformVersionResultDeserializer::deserialize(
                        "DeletePlatformVersionResult",
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

    /// <p>Returns attributes related to AWS Elastic Beanstalk that are associated with the calling AWS account.</p> <p>The result currently has one set of attributes—resource quotas.</p>
    fn describe_account_attributes(
        &self,
    ) -> RusotoFuture<DescribeAccountAttributesResult, DescribeAccountAttributesError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountAttributes");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountAttributesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAccountAttributesResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccountAttributesResultDeserializer::deserialize(
                        "DescribeAccountAttributesResult",
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

    /// <p>Retrieve a list of application versions.</p>
    fn describe_application_versions(
        &self,
        input: DescribeApplicationVersionsMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionsMessage, DescribeApplicationVersionsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeApplicationVersions");
        params.put("Version", "2010-12-01");
        DescribeApplicationVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeApplicationVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationVersionDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplicationVersionDescriptionsMessageDeserializer::deserialize(
                            "DescribeApplicationVersionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the descriptions of existing applications.</p>
    fn describe_applications(
        &self,
        input: DescribeApplicationsMessage,
    ) -> RusotoFuture<ApplicationDescriptionsMessage, DescribeApplicationsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeApplications");
        params.put("Version", "2010-12-01");
        DescribeApplicationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ApplicationDescriptionsMessageDeserializer::deserialize(
                        "DescribeApplicationsResult",
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

    /// <p>Describes the configuration options that are used in a particular configuration template or environment, or that a specified solution stack defines. The description includes the values the options, their default values, and an indication of the required action on a running environment if an option value is changed.</p>
    fn describe_configuration_options(
        &self,
        input: DescribeConfigurationOptionsMessage,
    ) -> RusotoFuture<ConfigurationOptionsDescription, DescribeConfigurationOptionsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeConfigurationOptions");
        params.put("Version", "2010-12-01");
        DescribeConfigurationOptionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationOptionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigurationOptionsDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfigurationOptionsDescriptionDeserializer::deserialize(
                        "DescribeConfigurationOptionsResult",
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

    /// <p><p>Returns a description of the settings for the specified configuration set, that is, either a configuration template or the configuration set associated with a running environment.</p> <p>When describing the settings for the configuration set associated with a running environment, it is possible to receive two sets of setting descriptions. One is the deployed configuration set, and the other is a draft configuration of an environment that is either in the process of deployment or that failed to deploy.</p> <p>Related Topics</p> <ul> <li> <p> <a>DeleteEnvironmentConfiguration</a> </p> </li> </ul></p>
    fn describe_configuration_settings(
        &self,
        input: DescribeConfigurationSettingsMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescriptions, DescribeConfigurationSettingsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeConfigurationSettings");
        params.put("Version", "2010-12-01");
        DescribeConfigurationSettingsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationSettingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigurationSettingsDescriptions::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfigurationSettingsDescriptionsDeserializer::deserialize(
                        "DescribeConfigurationSettingsResult",
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

    /// <p>Returns information about the overall health of the specified environment. The <b>DescribeEnvironmentHealth</b> operation is only available with AWS Elastic Beanstalk Enhanced Health.</p>
    fn describe_environment_health(
        &self,
        input: DescribeEnvironmentHealthRequest,
    ) -> RusotoFuture<DescribeEnvironmentHealthResult, DescribeEnvironmentHealthError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEnvironmentHealth");
        params.put("Version", "2010-12-01");
        DescribeEnvironmentHealthRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentHealthError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEnvironmentHealthResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeEnvironmentHealthResultDeserializer::deserialize(
                        "DescribeEnvironmentHealthResult",
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

    /// <p>Lists an environment's completed and failed managed actions.</p>
    fn describe_environment_managed_action_history(
        &self,
        input: DescribeEnvironmentManagedActionHistoryRequest,
    ) -> RusotoFuture<
        DescribeEnvironmentManagedActionHistoryResult,
        DescribeEnvironmentManagedActionHistoryError,
    > {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEnvironmentManagedActionHistory");
        params.put("Version", "2010-12-01");
        DescribeEnvironmentManagedActionHistoryRequestSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentManagedActionHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEnvironmentManagedActionHistoryResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeEnvironmentManagedActionHistoryResultDeserializer::deserialize(
                            "DescribeEnvironmentManagedActionHistoryResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists an environment's upcoming and in-progress managed actions.</p>
    fn describe_environment_managed_actions(
        &self,
        input: DescribeEnvironmentManagedActionsRequest,
    ) -> RusotoFuture<DescribeEnvironmentManagedActionsResult, DescribeEnvironmentManagedActionsError>
    {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEnvironmentManagedActions");
        params.put("Version", "2010-12-01");
        DescribeEnvironmentManagedActionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentManagedActionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeEnvironmentManagedActionsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        DescribeEnvironmentManagedActionsResultDeserializer::deserialize(
                            "DescribeEnvironmentManagedActionsResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns AWS resources for this environment.</p>
    fn describe_environment_resources(
        &self,
        input: DescribeEnvironmentResourcesMessage,
    ) -> RusotoFuture<EnvironmentResourceDescriptionsMessage, DescribeEnvironmentResourcesError>
    {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEnvironmentResources");
        params.put("Version", "2010-12-01");
        DescribeEnvironmentResourcesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentResourceDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        EnvironmentResourceDescriptionsMessageDeserializer::deserialize(
                            "DescribeEnvironmentResourcesResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns descriptions for existing environments.</p>
    fn describe_environments(
        &self,
        input: DescribeEnvironmentsMessage,
    ) -> RusotoFuture<EnvironmentDescriptionsMessage, DescribeEnvironmentsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEnvironments");
        params.put("Version", "2010-12-01");
        DescribeEnvironmentsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEnvironmentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnvironmentDescriptionsMessageDeserializer::deserialize(
                        "DescribeEnvironmentsResult",
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

    /// <p><p>Returns list of event descriptions matching criteria up to the last 6 weeks.</p> <note> <p>This action returns the most recent 1,000 events from the specified <code>NextToken</code>.</p> </note></p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<EventDescriptionsMessage, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEvents");
        params.put("Version", "2010-12-01");
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EventDescriptionsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventDescriptionsMessageDeserializer::deserialize(
                        "DescribeEventsResult",
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

    /// <p>Retrives detailed information about the health of instances in your AWS Elastic Beanstalk. This operation requires <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/health-enhanced.html">enhanced health reporting</a>.</p>
    fn describe_instances_health(
        &self,
        input: DescribeInstancesHealthRequest,
    ) -> RusotoFuture<DescribeInstancesHealthResult, DescribeInstancesHealthError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeInstancesHealth");
        params.put("Version", "2010-12-01");
        DescribeInstancesHealthRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancesHealthError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeInstancesHealthResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeInstancesHealthResultDeserializer::deserialize(
                        "DescribeInstancesHealthResult",
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

    /// <p>Describes the version of the platform.</p>
    fn describe_platform_version(
        &self,
        input: DescribePlatformVersionRequest,
    ) -> RusotoFuture<DescribePlatformVersionResult, DescribePlatformVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribePlatformVersion");
        params.put("Version", "2010-12-01");
        DescribePlatformVersionRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePlatformVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribePlatformVersionResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribePlatformVersionResultDeserializer::deserialize(
                        "DescribePlatformVersionResult",
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

    /// <p>Returns a list of the available solution stack names, with the public version first and then in reverse chronological order.</p>
    fn list_available_solution_stacks(
        &self,
    ) -> RusotoFuture<ListAvailableSolutionStacksResultMessage, ListAvailableSolutionStacksError>
    {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListAvailableSolutionStacks");
        params.put("Version", "2010-12-01");

        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAvailableSolutionStacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListAvailableSolutionStacksResultMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ListAvailableSolutionStacksResultMessageDeserializer::deserialize(
                            "ListAvailableSolutionStacksResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the available platforms.</p>
    fn list_platform_versions(
        &self,
        input: ListPlatformVersionsRequest,
    ) -> RusotoFuture<ListPlatformVersionsResult, ListPlatformVersionsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListPlatformVersions");
        params.put("Version", "2010-12-01");
        ListPlatformVersionsRequestSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPlatformVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListPlatformVersionsResult::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListPlatformVersionsResultDeserializer::deserialize(
                        "ListPlatformVersionsResult",
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

    /// <p>Returns the tags applied to an AWS Elastic Beanstalk resource. The response contains a list of tag key-value pairs.</p> <p>Currently, Elastic Beanstalk only supports tagging of Elastic Beanstalk environments. For details about environment tagging, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/using-features.tagging.html">Tagging Resources in Your Elastic Beanstalk Environment</a>.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<ResourceTagsDescriptionMessage, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2010-12-01");
        ListTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ResourceTagsDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ResourceTagsDescriptionMessageDeserializer::deserialize(
                        "ListTagsForResourceResult",
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

    /// <p>Deletes and recreates all of the AWS resources (for example: the Auto Scaling group, load balancer, etc.) for a specified environment and forces a restart.</p>
    fn rebuild_environment(
        &self,
        input: RebuildEnvironmentMessage,
    ) -> RusotoFuture<(), RebuildEnvironmentError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RebuildEnvironment");
        params.put("Version", "2010-12-01");
        RebuildEnvironmentMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RebuildEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Initiates a request to compile the specified type of information of the deployed environment.</p> <p> Setting the <code>InfoType</code> to <code>tail</code> compiles the last lines from the application server log files of every Amazon EC2 instance in your environment. </p> <p> Setting the <code>InfoType</code> to <code>bundle</code> compresses the application server log files for every Amazon EC2 instance into a <code>.zip</code> file. Legacy and .NET containers do not support bundle logs. </p> <p> Use <a>RetrieveEnvironmentInfo</a> to obtain the set of logs. </p> <p>Related Topics</p> <ul> <li> <p> <a>RetrieveEnvironmentInfo</a> </p> </li> </ul></p>
    fn request_environment_info(
        &self,
        input: RequestEnvironmentInfoMessage,
    ) -> RusotoFuture<(), RequestEnvironmentInfoError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RequestEnvironmentInfo");
        params.put("Version", "2010-12-01");
        RequestEnvironmentInfoMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RequestEnvironmentInfoError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Causes the environment to restart the application container server running on each Amazon EC2 instance.</p>
    fn restart_app_server(
        &self,
        input: RestartAppServerMessage,
    ) -> RusotoFuture<(), RestartAppServerError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RestartAppServer");
        params.put("Version", "2010-12-01");
        RestartAppServerMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestartAppServerError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves the compiled information from a <a>RequestEnvironmentInfo</a> request.</p> <p>Related Topics</p> <ul> <li> <p> <a>RequestEnvironmentInfo</a> </p> </li> </ul></p>
    fn retrieve_environment_info(
        &self,
        input: RetrieveEnvironmentInfoMessage,
    ) -> RusotoFuture<RetrieveEnvironmentInfoResultMessage, RetrieveEnvironmentInfoError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RetrieveEnvironmentInfo");
        params.put("Version", "2010-12-01");
        RetrieveEnvironmentInfoMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RetrieveEnvironmentInfoError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = RetrieveEnvironmentInfoResultMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        RetrieveEnvironmentInfoResultMessageDeserializer::deserialize(
                            "RetrieveEnvironmentInfoResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p>Swaps the CNAMEs of two environments.</p>
    fn swap_environment_cnam_es(
        &self,
        input: SwapEnvironmentCNAMEsMessage,
    ) -> RusotoFuture<(), SwapEnvironmentCNAMEsError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SwapEnvironmentCNAMEs");
        params.put("Version", "2010-12-01");
        SwapEnvironmentCNAMEsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SwapEnvironmentCNAMEsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Terminates the specified environment.</p>
    fn terminate_environment(
        &self,
        input: TerminateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, TerminateEnvironmentError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "TerminateEnvironment");
        params.put("Version", "2010-12-01");
        TerminateEnvironmentMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TerminateEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnvironmentDescriptionDeserializer::deserialize(
                        "TerminateEnvironmentResult",
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

    /// <p><p>Updates the specified application to have the specified properties.</p> <note> <p>If a property (for example, <code>description</code>) is not provided, the value remains unchanged. To clear these properties, specify an empty string.</p> </note></p>
    fn update_application(
        &self,
        input: UpdateApplicationMessage,
    ) -> RusotoFuture<ApplicationDescriptionMessage, UpdateApplicationError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateApplication");
        params.put("Version", "2010-12-01");
        UpdateApplicationMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ApplicationDescriptionMessageDeserializer::deserialize(
                        "UpdateApplicationResult",
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

    /// <p>Modifies lifecycle settings for an application.</p>
    fn update_application_resource_lifecycle(
        &self,
        input: UpdateApplicationResourceLifecycleMessage,
    ) -> RusotoFuture<
        ApplicationResourceLifecycleDescriptionMessage,
        UpdateApplicationResourceLifecycleError,
    > {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateApplicationResourceLifecycle");
        params.put("Version", "2010-12-01");
        UpdateApplicationResourceLifecycleMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationResourceLifecycleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationResourceLifecycleDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplicationResourceLifecycleDescriptionMessageDeserializer::deserialize(
                            "UpdateApplicationResourceLifecycleResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the specified application version to have the specified properties.</p> <note> <p>If a property (for example, <code>description</code>) is not provided, the value remains unchanged. To clear properties, specify an empty string.</p> </note></p>
    fn update_application_version(
        &self,
        input: UpdateApplicationVersionMessage,
    ) -> RusotoFuture<ApplicationVersionDescriptionMessage, UpdateApplicationVersionError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateApplicationVersion");
        params.put("Version", "2010-12-01");
        UpdateApplicationVersionMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ApplicationVersionDescriptionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ApplicationVersionDescriptionMessageDeserializer::deserialize(
                            "UpdateApplicationVersionResult",
                            &mut stack
                        )
                    );
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }

                Ok(result)
            }))
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Updates the specified configuration template to have the specified properties or configuration option values.</p> <note> <p>If a property (for example, <code>ApplicationName</code>) is not provided, its value remains unchanged. To clear such properties, specify an empty string.</p> </note> <p>Related Topics</p> <ul> <li> <p> <a>DescribeConfigurationOptions</a> </p> </li> </ul></p>
    fn update_configuration_template(
        &self,
        input: UpdateConfigurationTemplateMessage,
    ) -> RusotoFuture<ConfigurationSettingsDescription, UpdateConfigurationTemplateError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateConfigurationTemplate");
        params.put("Version", "2010-12-01");
        UpdateConfigurationTemplateMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigurationSettingsDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ConfigurationSettingsDescriptionDeserializer::deserialize(
                        "UpdateConfigurationTemplateResult",
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

    /// <p>Updates the environment description, deploys a new application version, updates the configuration settings to an entirely new configuration template, or updates select configuration option values in the running environment.</p> <p> Attempting to update both the release and configuration is not allowed and AWS Elastic Beanstalk returns an <code>InvalidParameterCombination</code> error. </p> <p> When updating the configuration settings to a new template or individual settings, a draft configuration is created and <a>DescribeConfigurationSettings</a> for this environment returns two setting descriptions with different <code>DeploymentStatus</code> values. </p>
    fn update_environment(
        &self,
        input: UpdateEnvironmentMessage,
    ) -> RusotoFuture<EnvironmentDescription, UpdateEnvironmentError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateEnvironment");
        params.put("Version", "2010-12-01");
        UpdateEnvironmentMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEnvironmentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnvironmentDescription::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EnvironmentDescriptionDeserializer::deserialize(
                        "UpdateEnvironmentResult",
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

    /// <p>Update the list of tags applied to an AWS Elastic Beanstalk resource. Two lists can be passed: <code>TagsToAdd</code> for tags to add or update, and <code>TagsToRemove</code>.</p> <p>Currently, Elastic Beanstalk only supports tagging of Elastic Beanstalk environments. For details about environment tagging, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/using-features.tagging.html">Tagging Resources in Your Elastic Beanstalk Environment</a>.</p> <p>If you create a custom IAM user policy to control permission to this operation, specify one of the following two virtual actions (or both) instead of the API operation name:</p> <dl> <dt>elasticbeanstalk:AddTags</dt> <dd> <p>Controls permission to call <code>UpdateTagsForResource</code> and pass a list of tags to add in the <code>TagsToAdd</code> parameter.</p> </dd> <dt>elasticbeanstalk:RemoveTags</dt> <dd> <p>Controls permission to call <code>UpdateTagsForResource</code> and pass a list of tag keys to remove in the <code>TagsToRemove</code> parameter.</p> </dd> </dl> <p>For details about creating a custom user policy, see <a href="http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/AWSHowTo.iam.managed-policies.html#AWSHowTo.iam.policies">Creating a Custom User Policy</a>.</p>
    fn update_tags_for_resource(
        &self,
        input: UpdateTagsForResourceMessage,
    ) -> RusotoFuture<(), UpdateTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateTagsForResource");
        params.put("Version", "2010-12-01");
        UpdateTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Takes a set of configuration settings and either a configuration template or environment, and determines whether those values are valid.</p> <p>This action returns a list of messages indicating any errors or warnings associated with the selection of option values.</p>
    fn validate_configuration_settings(
        &self,
        input: ValidateConfigurationSettingsMessage,
    ) -> RusotoFuture<ConfigurationSettingsValidationMessages, ValidateConfigurationSettingsError>
    {
        let mut request = SignedRequest::new("POST", "elasticbeanstalk", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ValidateConfigurationSettings");
        params.put("Version", "2010-12-01");
        ValidateConfigurationSettingsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ValidateConfigurationSettingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ConfigurationSettingsValidationMessages::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(
                        ConfigurationSettingsValidationMessagesDeserializer::deserialize(
                            "ValidateConfigurationSettingsResult",
                            &mut stack
                        )
                    );
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
    fn test_parse_valid_elasticbeanstalk_check_dns_availability() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-check-dns-availability.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CheckDNSAvailabilityMessage::default();
        let result = client.check_dns_availability(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_create_application_version() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-create-application-version.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateApplicationVersionMessage::default();
        let result = client.create_application_version(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_create_application() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-create-application.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateApplicationMessage::default();
        let result = client.create_application(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_create_configuration_template() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-create-configuration-template.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateConfigurationTemplateMessage::default();
        let result = client.create_configuration_template(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_create_environment() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-create-environment.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CreateEnvironmentMessage::default();
        let result = client.create_environment(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_create_storage_location() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-create-storage-location.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.create_storage_location().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_delete_application() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-delete-application.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DeleteApplicationMessage::default();
        let result = client.delete_application(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_describe_application_versions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-describe-application-versions.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeApplicationVersionsMessage::default();
        let result = client.describe_application_versions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_describe_applications() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-describe-applications.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeApplicationsMessage::default();
        let result = client.describe_applications(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_describe_configuration_options() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-describe-configuration-options.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeConfigurationOptionsMessage::default();
        let result = client.describe_configuration_options(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_describe_environments() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-describe-environments.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeEnvironmentsMessage::default();
        let result = client.describe_environments(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_describe_events() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-describe-events.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeEventsMessage::default();
        let result = client.describe_events(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_list_available_solution_stacks() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-list-available-solution-stacks.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);

        let result = client.list_available_solution_stacks().sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_retrieve_environment_info() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-retrieve-environment-info.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = RetrieveEnvironmentInfoMessage::default();
        let result = client.retrieve_environment_info(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_terminate_environment() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-terminate-environment.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = TerminateEnvironmentMessage::default();
        let result = client.terminate_environment(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_update_application_version() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-update-application-version.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = UpdateApplicationVersionMessage::default();
        let result = client.update_application_version(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_elasticbeanstalk_update_application() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "elasticbeanstalk-update-application.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = UpdateApplicationMessage::default();
        let result = client.update_application(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
