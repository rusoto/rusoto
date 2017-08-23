
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
use xml::EventReader;
use xml::reader::ParserConfig;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use xml::reader::XmlEvent;
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::xmlutil::{characters, end_element, start_element, skip_tree, peek_at_name};
use rusoto_core::xmlerror::*;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
#[doc="<p>The AccountLimit data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct AccountLimit {
    #[doc="<p>The name of the account limit. Currently, the only account limit is <code>StackLimit</code>.</p>"]
    pub name: Option<String>,
    #[doc="<p>The value that is associated with the account limit name.</p>"]
    pub value: Option<i64>,
}

struct AccountLimitDeserializer;
impl AccountLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AccountLimit, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccountLimit::default();

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
                            obj.name = Some(try!(LimitNameDeserializer::deserialize("Name",
                                                                                    stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(LimitValueDeserializer::deserialize("Value",
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
struct AccountLimitListDeserializer;
impl AccountLimitListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<AccountLimit>, XmlParseError> {

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
                        obj.push(try!(AccountLimitDeserializer::deserialize("member", stack)));
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
struct AllowedValueDeserializer;
impl AllowedValueDeserializer {
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
struct AllowedValuesDeserializer;
impl AllowedValuesDeserializer {
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
                    if name == "member" {
                        obj.push(try!(AllowedValueDeserializer::deserialize("member", stack)));
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
#[doc="<p>The input for the <a>CancelUpdateStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CancelUpdateStackInput {
    #[doc="<p>A unique identifier for this <code>CancelUpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to cancel an update on a stack with the same name. You might retry <code>CancelUpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack.</p>"]
    pub stack_name: String,
}


/// Serialize `CancelUpdateStackInput` contents to a `SignedRequest`.
struct CancelUpdateStackInputSerializer;
impl CancelUpdateStackInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CancelUpdateStackInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

struct CapabilitiesDeserializer;
impl CapabilitiesDeserializer {
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
                    if name == "member" {
                        obj.push(try!(CapabilityDeserializer::deserialize("member", stack)));
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

/// Serialize `Capabilities` contents to a `SignedRequest`.
struct CapabilitiesSerializer;
impl CapabilitiesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct CapabilitiesReasonDeserializer;
impl CapabilitiesReasonDeserializer {
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
struct CapabilityDeserializer;
impl CapabilityDeserializer {
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
struct CausingEntityDeserializer;
impl CausingEntityDeserializer {
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
#[doc="<p>The <code>Change</code> structure describes the changes AWS CloudFormation will perform if you execute the change set.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Change {
    #[doc="<p>A <code>ResourceChange</code> structure that describes the resource and action that AWS CloudFormation will perform.</p>"]
    pub resource_change: Option<ResourceChange>,
    #[doc="<p>The type of entity that AWS CloudFormation changes. Currently, the only entity type is <code>Resource</code>.</p>"]
    pub type_: Option<String>,
}

struct ChangeDeserializer;
impl ChangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Change, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Change::default();

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
                        "ResourceChange" => {
                            obj.resource_change =
                                Some(try!(ResourceChangeDeserializer::deserialize("ResourceChange",
                                                                                  stack)));
                        }
                        "Type" => {
                            obj.type_ = Some(try!(ChangeTypeDeserializer::deserialize("Type",
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
struct ChangeActionDeserializer;
impl ChangeActionDeserializer {
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
struct ChangeSetIdDeserializer;
impl ChangeSetIdDeserializer {
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
struct ChangeSetNameDeserializer;
impl ChangeSetNameDeserializer {
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
struct ChangeSetStatusDeserializer;
impl ChangeSetStatusDeserializer {
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
struct ChangeSetStatusReasonDeserializer;
impl ChangeSetStatusReasonDeserializer {
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
struct ChangeSetSummariesDeserializer;
impl ChangeSetSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<ChangeSetSummary>, XmlParseError> {

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
                        obj.push(try!(ChangeSetSummaryDeserializer::deserialize("member", stack)));
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
#[doc="<p>The <code>ChangeSetSummary</code> structure describes a change set, its status, and the stack with which it's associated.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ChangeSetSummary {
    #[doc="<p>The ID of the change set.</p>"]
    pub change_set_id: Option<String>,
    #[doc="<p>The name of the change set.</p>"]
    pub change_set_name: Option<String>,
    #[doc="<p>The start time when the change set was created, in UTC.</p>"]
    pub creation_time: Option<String>,
    #[doc="<p>Descriptive information about the change set.</p>"]
    pub description: Option<String>,
    #[doc="<p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change set. If you can’t execute the change set, the status indicates why. For example, a change set might be in an <code>UNAVAILABLE</code> state because AWS CloudFormation is still creating it or in an <code>OBSOLETE</code> state because the stack was already updated.</p>"]
    pub execution_status: Option<String>,
    #[doc="<p>The ID of the stack with which the change set is associated.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name of the stack with which the change set is associated.</p>"]
    pub stack_name: Option<String>,
    #[doc="<p>The state of the change set, such as <code>CREATE_IN_PROGRESS</code>, <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p>"]
    pub status: Option<String>,
    #[doc="<p>A description of the change set's status. For example, if your change set is in the <code>FAILED</code> state, AWS CloudFormation shows the error message.</p>"]
    pub status_reason: Option<String>,
}

struct ChangeSetSummaryDeserializer;
impl ChangeSetSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ChangeSetSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ChangeSetSummary::default();

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
                        "ChangeSetId" => {
                            obj.change_set_id =
                                Some(try!(ChangeSetIdDeserializer::deserialize("ChangeSetId",
                                                                               stack)));
                        }
                        "ChangeSetName" => {
                            obj.change_set_name =
                                Some(try!(ChangeSetNameDeserializer::deserialize("ChangeSetName",
                                                                                 stack)));
                        }
                        "CreationTime" => {
                            obj.creation_time =
                                Some(try!(CreationTimeDeserializer::deserialize("CreationTime",
                                                                                stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "ExecutionStatus" => {
                            obj.execution_status =
                                Some(try!(ExecutionStatusDeserializer::deserialize("ExecutionStatus",
                                                                                   stack)));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = Some(try!(StackNameDeserializer::deserialize("StackName",
                                                                                          stack)));
                        }
                        "Status" => {
                            obj.status =
                                Some(try!(ChangeSetStatusDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        "StatusReason" => {
                            obj.status_reason =
                                Some(try!(ChangeSetStatusReasonDeserializer::deserialize("StatusReason",
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
struct ChangeSourceDeserializer;
impl ChangeSourceDeserializer {
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
struct ChangeTypeDeserializer;
impl ChangeTypeDeserializer {
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
struct ChangesDeserializer;
impl ChangesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Change>, XmlParseError> {

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
                        obj.push(try!(ChangeDeserializer::deserialize("member", stack)));
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
struct ClientRequestTokenDeserializer;
impl ClientRequestTokenDeserializer {
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
#[doc="<p>The input for the <a>ContinueUpdateRollback</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ContinueUpdateRollbackInput {
    #[doc="<p>A unique identifier for this <code>ContinueUpdateRollback</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to continue the rollback to a stack with the same name. You might retry <code>ContinueUpdateRollback</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>A list of the logical IDs of the resources that AWS CloudFormation skips during the continue update rollback operation. You can specify only resources that are in the <code>UPDATE_FAILED</code> state because a rollback failed. You can't specify resources that are in the <code>UPDATE_FAILED</code> state for other reasons, for example, because an update was canceled. To check why a resource update failed, use the <a>DescribeStackResources</a> action, and view the resource status reason. </p> <important> <p>Specify this property to skip rolling back resources that AWS CloudFormation can't successfully roll back. We recommend that you <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed\"> troubleshoot</a> resources before skipping them. AWS CloudFormation sets the status of the specified resources to <code>UPDATE_COMPLETE</code> and continues to roll back the stack. After the rollback is complete, the state of the skipped resources will be inconsistent with the state of the resources in the stack template. Before performing another stack update, you must update the stack or resources to be consistent with each other. If you don't, subsequent stack updates might fail, and the stack will become unrecoverable. </p> </important> <p>Specify the minimum number of resources required to successfully roll back your stack. For example, a failed resource update might cause dependent resources to fail. In this case, it might not be necessary to skip the dependent resources. </p> <p>To specify resources in a nested stack, use the following format: <code>NestedStackName.ResourceLogicalID</code>. If the <code>ResourceLogicalID</code> is a stack resource (<code>Type: AWS::CloudFormation::Stack</code>), it must be in one of the following states: <code>DELETE_IN_PROGRESS</code>, <code>DELETE_COMPLETE</code>, or <code>DELETE_FAILED</code>. </p>"]
    pub resources_to_skip: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to roll back the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>The name or the unique ID of the stack that you want to continue rolling back.</p> <note> <p>Don't specify the name of a nested stack (a stack that was created by using the <code>AWS::CloudFormation::Stack</code> resource). Instead, use this operation on the parent stack (the stack that contains the <code>AWS::CloudFormation::Stack</code> resource).</p> </note>"]
    pub stack_name: String,
}


/// Serialize `ContinueUpdateRollbackInput` contents to a `SignedRequest`.
struct ContinueUpdateRollbackInputSerializer;
impl ContinueUpdateRollbackInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ContinueUpdateRollbackInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.resources_to_skip {
            ResourcesToSkipSerializer::serialize(params,
                                                 &format!("{}{}", prefix, "ResourcesToSkip"),
                                                 field_value);
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

#[doc="<p>The output for a <a>ContinueUpdateRollback</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ContinueUpdateRollbackOutput;

struct ContinueUpdateRollbackOutputDeserializer;
impl ContinueUpdateRollbackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ContinueUpdateRollbackOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ContinueUpdateRollbackOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>The input for the <a>CreateChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateChangeSetInput {
    #[doc="<p>A list of values that you must specify before AWS CloudFormation can update certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html\"> AWS::IAM::AccessKey</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html\"> AWS::IAM::Group</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html\"> AWS::IAM::InstanceProfile</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html\"> AWS::IAM::Policy</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html\"> AWS::IAM::Role</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html\"> AWS::IAM::User</a>, and <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html\"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities\">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>The name of the change set. The name must be unique among all change sets that are associated with the specified stack.</p> <p>A change set name can contain only alphanumeric, case sensitive characters and hyphens. It must start with an alphabetic character and cannot exceed 128 characters.</p>"]
    pub change_set_name: String,
    #[doc="<p>The type of change set operation. To create a change set for a new stack, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code>.</p> <p>If you create a change set for a new stack, AWS Cloudformation creates a stack with a unique stack ID, but no template or resources. The stack will be in the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-describing-stacks.html#d0e11995\"> <code>REVIEW_IN_PROGRESS</code> </a> state until you execute the change set.</p> <p>By default, AWS CloudFormation specifies <code>UPDATE</code>. You can't use the <code>UPDATE</code> type to create a change set for a new stack or the <code>CREATE</code> type to create a change set for an existing stack.</p>"]
    pub change_set_type: Option<String>,
    #[doc="<p>A unique identifier for this <code>CreateChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create another change set with the same name. You might retry <code>CreateChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_token: Option<String>,
    #[doc="<p>A description to help you identify this change set.</p>"]
    pub description: Option<String>,
    #[doc="<p>The Amazon Resource Names (ARNs) of Amazon Simple Notification Service (Amazon SNS) topics that AWS CloudFormation associates with the stack. To remove all associated notification topics, specify an empty list.</p>"]
    pub notification_ar_ns: Option<Vec<String>>,
    #[doc="<p>A list of <code>Parameter</code> structures that specify input parameters for the change set. For more information, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html\">Parameter</a> data type.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>The template resource types that you have permissions to work with if you execute this change set, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource type that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for condition keys in IAM policies for AWS CloudFormation. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html\">Controlling Access with AWS Identity and Access Management</a> in the AWS CloudFormation User Guide.</p>"]
    pub resource_types: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes when executing the change set. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates the change set by comparing this stack's information with the information that you submit, such as a modified template or different parameter input values.</p>"]
    pub stack_name: String,
    #[doc="<p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to resources in the stack. You can specify a maximum of 10 tags.</p>"]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>A structure that contains the body of the revised template, with a minimum length of 1 byte and a maximum length of 51,200 bytes. AWS CloudFormation generates the change set by comparing this template with the template of the stack that you specified.</p> <p>Conditional: You must specify only <code>TemplateBody</code> or <code>TemplateURL</code>.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>The location of the file that contains the revised template. The URL must point to a template (max size: 460,800 bytes) that is located in an S3 bucket. AWS CloudFormation generates the change set by comparing this template with the stack that you specified.</p> <p>Conditional: You must specify only <code>TemplateBody</code> or <code>TemplateURL</code>.</p>"]
    pub template_url: Option<String>,
    #[doc="<p>Whether to reuse the template that is associated with the stack to create the change set.</p>"]
    pub use_previous_template: Option<bool>,
}


/// Serialize `CreateChangeSetInput` contents to a `SignedRequest`.
struct CreateChangeSetInputSerializer;
impl CreateChangeSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateChangeSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.capabilities {
            CapabilitiesSerializer::serialize(params,
                                              &format!("{}{}", prefix, "Capabilities"),
                                              field_value);
        }
        params.put(&format!("{}{}", prefix, "ChangeSetName"),
                   &obj.change_set_name);
        if let Some(ref field_value) = obj.change_set_type {
            params.put(&format!("{}{}", prefix, "ChangeSetType"), &field_value);
        }
        if let Some(ref field_value) = obj.client_token {
            params.put(&format!("{}{}", prefix, "ClientToken"), &field_value);
        }
        if let Some(ref field_value) = obj.description {
            params.put(&format!("{}{}", prefix, "Description"), &field_value);
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "NotificationARNs"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Parameters"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(params,
                                               &format!("{}{}", prefix, "ResourceTypes"),
                                               field_value);
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
        if let Some(ref field_value) = obj.use_previous_template {
            params.put(&format!("{}{}", prefix, "UsePreviousTemplate"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>The output for the <a>CreateChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateChangeSetOutput {
    #[doc="<p>The Amazon Resource Name (ARN) of the change set.</p>"]
    pub id: Option<String>,
    #[doc="<p>The unique ID of the stack.</p>"]
    pub stack_id: Option<String>,
}

struct CreateChangeSetOutputDeserializer;
impl CreateChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateChangeSetOutput::default();

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
                        "Id" => {
                            obj.id = Some(try!(ChangeSetIdDeserializer::deserialize("Id", stack)));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
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
#[doc="<p>The input for <a>CreateStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateStackInput {
    #[doc="<p>A list of values that you must specify before AWS CloudFormation can create certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html\"> AWS::IAM::AccessKey</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html\"> AWS::IAM::Group</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html\"> AWS::IAM::InstanceProfile</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html\"> AWS::IAM::Policy</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html\"> AWS::IAM::Role</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html\"> AWS::IAM::User</a>, and <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html\"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities\">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>A unique identifier for this <code>CreateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create a stack with the same name. You might retry <code>CreateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>Set to <code>true</code> to disable rollback of the stack if stack creation failed. You can specify either <code>DisableRollback</code> or <code>OnFailure</code>, but not both.</p> <p>Default: <code>false</code> </p>"]
    pub disable_rollback: Option<bool>,
    #[doc="<p>The Simple Notification Service (SNS) topic ARNs to publish stack related events. You can find your SNS topic ARNs using the SNS console or your Command Line Interface (CLI).</p>"]
    pub notification_ar_ns: Option<Vec<String>>,
    #[doc="<p>Determines what action will be taken if stack creation fails. This must be one of: DO_NOTHING, ROLLBACK, or DELETE. You can specify either <code>OnFailure</code> or <code>DisableRollback</code>, but not both.</p> <p>Default: <code>ROLLBACK</code> </p>"]
    pub on_failure: Option<String>,
    #[doc="<p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html\">Parameter</a> data type.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>The template resource types that you have permissions to work with for this create stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>. Use the following syntax to describe template resource types: <code>AWS::*</code> (for all AWS resource), <code>Custom::*</code> (for all custom resources), <code>Custom::<i>logical_ID</i> </code> (for a specific custom resource), <code>AWS::<i>service_name</i>::*</code> (for all resources of a particular AWS service), and <code>AWS::<i>service_name</i>::<i>resource_logical_ID</i> </code> (for a specific AWS resource).</p> <p>If the list of resource types doesn't include a resource that you're creating, the stack creation fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html\">Controlling Access with AWS Identity and Access Management</a>.</p>"]
    pub resource_types: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to create the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>The name that is associated with the stack. The name must be unique in the region in which you are creating the stack.</p> <note> <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.</p> </note>"]
    pub stack_name: String,
    #[doc="<p>Structure containing the stack policy body. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html\"> Prevent Updates to Stack Resources</a> in the <i>AWS CloudFormation User Guide</i>. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>"]
    pub stack_policy_body: Option<String>,
    #[doc="<p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>"]
    pub stack_policy_url: Option<String>,
    #[doc="<p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to the resources created in the stack. A maximum number of 10 tags can be specified.</p>"]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>"]
    pub template_url: Option<String>,
    #[doc="<p>The amount of time that can pass before the stack status becomes CREATE_FAILED; if <code>DisableRollback</code> is not set or is set to <code>false</code>, the stack will be rolled back.</p>"]
    pub timeout_in_minutes: Option<i64>,
}


/// Serialize `CreateStackInput` contents to a `SignedRequest`.
struct CreateStackInputSerializer;
impl CreateStackInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.capabilities {
            CapabilitiesSerializer::serialize(params,
                                              &format!("{}{}", prefix, "Capabilities"),
                                              field_value);
        }
        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.disable_rollback {
            params.put(&format!("{}{}", prefix, "DisableRollback"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "NotificationARNs"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.on_failure {
            params.put(&format!("{}{}", prefix, "OnFailure"), &field_value);
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Parameters"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(params,
                                               &format!("{}{}", prefix, "ResourceTypes"),
                                               field_value);
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(&format!("{}{}", prefix, "StackPolicyBody"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(&format!("{}{}", prefix, "StackPolicyURL"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
        if let Some(ref field_value) = obj.timeout_in_minutes {
            params.put(&format!("{}{}", prefix, "TimeoutInMinutes"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>The output for a <a>CreateStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateStackOutput {
    #[doc="<p>Unique identifier of the stack.</p>"]
    pub stack_id: Option<String>,
}

struct CreateStackOutputDeserializer;
impl CreateStackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateStackOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStackOutput::default();

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
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
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
struct CreationTimeDeserializer;
impl CreationTimeDeserializer {
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
#[doc="<p>The input for the <a>DeleteChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteChangeSetInput {
    #[doc="<p>The name or Amazon Resource Name (ARN) of the change set that you want to delete.</p>"]
    pub change_set_name: String,
    #[doc="<p>If you specified the name of a change set to delete, specify the stack name or ID (ARN) that is associated with it.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `DeleteChangeSetInput` contents to a `SignedRequest`.
struct DeleteChangeSetInputSerializer;
impl DeleteChangeSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteChangeSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ChangeSetName"),
                   &obj.change_set_name);
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for the <a>DeleteChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteChangeSetOutput;

struct DeleteChangeSetOutputDeserializer;
impl DeleteChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteChangeSetOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>The input for <a>DeleteStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteStackInput {
    #[doc="<p>A unique identifier for this <code>DeleteStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to delete a stack with the same name. You might retry <code>DeleteStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>For stacks in the <code>DELETE_FAILED</code> state, a list of resource logical IDs that are associated with the resources you want to retain. During deletion, AWS CloudFormation deletes the stack but does not delete the retained resources.</p> <p>Retaining resources is useful when you cannot delete a resource, such as a non-empty S3 bucket, but you want to delete the stack.</p>"]
    pub retain_resources: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to delete the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack.</p>"]
    pub stack_name: String,
}


/// Serialize `DeleteStackInput` contents to a `SignedRequest`.
struct DeleteStackInputSerializer;
impl DeleteStackInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.retain_resources {
            RetainResourcesSerializer::serialize(params,
                                                 &format!("{}{}", prefix, "RetainResources"),
                                                 field_value);
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

struct DeletionTimeDeserializer;
impl DeletionTimeDeserializer {
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
#[doc="<p>The input for the <a>DescribeAccountLimits</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeAccountLimitsInput {
    #[doc="<p>A string that identifies the next page of limits that you want to retrieve.</p>"]
    pub next_token: Option<String>,
}


/// Serialize `DescribeAccountLimitsInput` contents to a `SignedRequest`.
struct DescribeAccountLimitsInputSerializer;
impl DescribeAccountLimitsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAccountLimitsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }

    }
}

#[doc="<p>The output for the <a>DescribeAccountLimits</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeAccountLimitsOutput {
    #[doc="<p>An account limit structure that contain a list of AWS CloudFormation account limits and their values.</p>"]
    pub account_limits: Option<Vec<AccountLimit>>,
    #[doc="<p>If the output exceeds 1 MB in size, a string that identifies the next page of limits. If no additional page exists, this value is null.</p>"]
    pub next_token: Option<String>,
}

struct DescribeAccountLimitsOutputDeserializer;
impl DescribeAccountLimitsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeAccountLimitsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeAccountLimitsOutput::default();

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
                        "AccountLimits" => {
                            obj.account_limits =
                                Some(try!(AccountLimitListDeserializer::deserialize("AccountLimits",
                                                                                    stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
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
#[doc="<p>The input for the <a>DescribeChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeChangeSetInput {
    #[doc="<p>The name or Amazon Resource Name (ARN) of the change set that you want to describe.</p>"]
    pub change_set_name: String,
    #[doc="<p>A string (provided by the <a>DescribeChangeSet</a> response output) that identifies the next page of information that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>If you specified the name of a change set, specify the stack name or ID (ARN) of the change set you want to describe.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `DescribeChangeSetInput` contents to a `SignedRequest`.
struct DescribeChangeSetInputSerializer;
impl DescribeChangeSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeChangeSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ChangeSetName"),
                   &obj.change_set_name);
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for the <a>DescribeChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeChangeSetOutput {
    #[doc="<p>If you execute the change set, the list of capabilities that were explicitly acknowledged when the change set was created.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>The ARN of the change set.</p>"]
    pub change_set_id: Option<String>,
    #[doc="<p>The name of the change set.</p>"]
    pub change_set_name: Option<String>,
    #[doc="<p>A list of <code>Change</code> structures that describes the resources AWS CloudFormation changes if you execute the change set.</p>"]
    pub changes: Option<Vec<Change>>,
    #[doc="<p>The start time when the change set was created, in UTC.</p>"]
    pub creation_time: Option<String>,
    #[doc="<p>Information about the change set.</p>"]
    pub description: Option<String>,
    #[doc="<p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change set. If you can’t execute the change set, the status indicates why. For example, a change set might be in an <code>UNAVAILABLE</code> state because AWS CloudFormation is still creating it or in an <code>OBSOLETE</code> state because the stack was already updated.</p>"]
    pub execution_status: Option<String>,
    #[doc="<p>If the output exceeds 1 MB, a string that identifies the next page of changes. If there is no additional page, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>The ARNs of the Amazon Simple Notification Service (Amazon SNS) topics that will be associated with the stack if you execute the change set.</p>"]
    pub notification_ar_ns: Option<Vec<String>>,
    #[doc="<p>A list of <code>Parameter</code> structures that describes the input parameters and their values used to create the change set. For more information, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html\">Parameter</a> data type.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>The ARN of the stack that is associated with the change set.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name of the stack that is associated with the change set.</p>"]
    pub stack_name: Option<String>,
    #[doc="<p>The current status of the change set, such as <code>CREATE_IN_PROGRESS</code>, <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p>"]
    pub status: Option<String>,
    #[doc="<p>A description of the change set's status. For example, if your attempt to create a change set failed, AWS CloudFormation shows the error message.</p>"]
    pub status_reason: Option<String>,
    #[doc="<p>If you execute the change set, the tags that will be associated with the stack.</p>"]
    pub tags: Option<Vec<Tag>>,
}

struct DescribeChangeSetOutputDeserializer;
impl DescribeChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeChangeSetOutput::default();

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
                        "Capabilities" => {
                            obj.capabilities =
                                Some(try!(CapabilitiesDeserializer::deserialize("Capabilities",
                                                                                stack)));
                        }
                        "ChangeSetId" => {
                            obj.change_set_id =
                                Some(try!(ChangeSetIdDeserializer::deserialize("ChangeSetId",
                                                                               stack)));
                        }
                        "ChangeSetName" => {
                            obj.change_set_name =
                                Some(try!(ChangeSetNameDeserializer::deserialize("ChangeSetName",
                                                                                 stack)));
                        }
                        "Changes" => {
                            obj.changes = Some(try!(ChangesDeserializer::deserialize("Changes",
                                                                                     stack)));
                        }
                        "CreationTime" => {
                            obj.creation_time =
                                Some(try!(CreationTimeDeserializer::deserialize("CreationTime",
                                                                                stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "ExecutionStatus" => {
                            obj.execution_status =
                                Some(try!(ExecutionStatusDeserializer::deserialize("ExecutionStatus",
                                                                                   stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "NotificationARNs" => {
                            obj.notification_ar_ns =
                                Some(try!(NotificationARNsDeserializer::deserialize("NotificationARNs",
                                                                                    stack)));
                        }
                        "Parameters" => {
                            obj.parameters = Some(try!(ParametersDeserializer::deserialize("Parameters",
                                                                                           stack)));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = Some(try!(StackNameDeserializer::deserialize("StackName",
                                                                                          stack)));
                        }
                        "Status" => {
                            obj.status =
                                Some(try!(ChangeSetStatusDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        "StatusReason" => {
                            obj.status_reason =
                                Some(try!(ChangeSetStatusReasonDeserializer::deserialize("StatusReason",
                                                                                         stack)));
                        }
                        "Tags" => {
                            obj.tags = Some(try!(TagsDeserializer::deserialize("Tags", stack)));
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
#[doc="<p>The input for <a>DescribeStackEvents</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackEventsInput {
    #[doc="<p>A string that identifies the next page of events that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `DescribeStackEventsInput` contents to a `SignedRequest`.
struct DescribeStackEventsInputSerializer;
impl DescribeStackEventsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackEventsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for a <a>DescribeStackEvents</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackEventsOutput {
    #[doc="<p>If the output exceeds 1 MB in size, a string that identifies the next page of events. If no additional page exists, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>StackEvents</code> structures.</p>"]
    pub stack_events: Option<Vec<StackEvent>>,
}

struct DescribeStackEventsOutputDeserializer;
impl DescribeStackEventsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeStackEventsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackEventsOutput::default();

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
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "StackEvents" => {
                            obj.stack_events =
                                Some(try!(StackEventsDeserializer::deserialize("StackEvents",
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
#[doc="<p>The input for <a>DescribeStackResource</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackResourceInput {
    #[doc="<p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>"]
    pub logical_resource_id: String,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>"]
    pub stack_name: String,
}


/// Serialize `DescribeStackResourceInput` contents to a `SignedRequest`.
struct DescribeStackResourceInputSerializer;
impl DescribeStackResourceInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackResourceInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "LogicalResourceId"),
                   &obj.logical_resource_id);
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

#[doc="<p>The output for a <a>DescribeStackResource</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackResourceOutput {
    #[doc="<p>A <code>StackResourceDetail</code> structure containing the description of the specified resource in the specified stack.</p>"]
    pub stack_resource_detail: Option<StackResourceDetail>,
}

struct DescribeStackResourceOutputDeserializer;
impl DescribeStackResourceOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeStackResourceOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackResourceOutput::default();

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
                        "StackResourceDetail" => {
                            obj.stack_resource_detail =
                                Some(try!(StackResourceDetailDeserializer::deserialize("StackResourceDetail",
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
#[doc="<p>The input for <a>DescribeStackResources</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackResourcesInput {
    #[doc="<p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>"]
    pub logical_resource_id: Option<String>,
    #[doc="<p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p> <p>For example, for an Amazon Elastic Compute Cloud (EC2) instance, <code>PhysicalResourceId</code> corresponds to the <code>InstanceId</code>. You can pass the EC2 <code>InstanceId</code> to <code>DescribeStackResources</code> to find which stack the instance belongs to and what other resources are part of the stack.</p> <p>Required: Conditional. If you do not specify <code>PhysicalResourceId</code>, you must specify <code>StackName</code>.</p> <p>Default: There is no default value.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p> <p>Required: Conditional. If you do not specify <code>StackName</code>, you must specify <code>PhysicalResourceId</code>.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `DescribeStackResourcesInput` contents to a `SignedRequest`.
struct DescribeStackResourcesInputSerializer;
impl DescribeStackResourcesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackResourcesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.logical_resource_id {
            params.put(&format!("{}{}", prefix, "LogicalResourceId"), &field_value);
        }
        if let Some(ref field_value) = obj.physical_resource_id {
            params.put(&format!("{}{}", prefix, "PhysicalResourceId"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for a <a>DescribeStackResources</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStackResourcesOutput {
    #[doc="<p>A list of <code>StackResource</code> structures.</p>"]
    pub stack_resources: Option<Vec<StackResource>>,
}

struct DescribeStackResourcesOutputDeserializer;
impl DescribeStackResourcesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeStackResourcesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackResourcesOutput::default();

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
                        "StackResources" => {
                            obj.stack_resources =
                                Some(try!(StackResourcesDeserializer::deserialize("StackResources",
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
#[doc="<p>The input for <a>DescribeStacks</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStacksInput {
    #[doc="<p>A string that identifies the next page of stacks that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `DescribeStacksInput` contents to a `SignedRequest`.
struct DescribeStacksInputSerializer;
impl DescribeStacksInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStacksInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for a <a>DescribeStacks</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeStacksOutput {
    #[doc="<p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>A list of stack structures.</p>"]
    pub stacks: Option<Vec<Stack>>,
}

struct DescribeStacksOutputDeserializer;
impl DescribeStacksOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeStacksOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStacksOutput::default();

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
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "Stacks" => {
                            obj.stacks = Some(try!(StacksDeserializer::deserialize("Stacks",
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
struct DescriptionDeserializer;
impl DescriptionDeserializer {
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
struct DisableRollbackDeserializer;
impl DisableRollbackDeserializer {
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
#[doc="<p>The input for an <a>EstimateTemplateCost</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct EstimateTemplateCostInput {
    #[doc="<p>A list of <code>Parameter</code> structures that specify input parameters.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must pass <code>TemplateBody</code> or <code>TemplateURL</code>. If both are passed, only <code>TemplateBody</code> is used.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>"]
    pub template_url: Option<String>,
}


/// Serialize `EstimateTemplateCostInput` contents to a `SignedRequest`.
struct EstimateTemplateCostInputSerializer;
impl EstimateTemplateCostInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EstimateTemplateCostInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Parameters"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }

    }
}

#[doc="<p>The output for a <a>EstimateTemplateCost</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct EstimateTemplateCostOutput {
    #[doc="<p>An AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>"]
    pub url: Option<String>,
}

struct EstimateTemplateCostOutputDeserializer;
impl EstimateTemplateCostOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EstimateTemplateCostOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EstimateTemplateCostOutput::default();

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
                        "Url" => {
                            obj.url = Some(try!(UrlDeserializer::deserialize("Url", stack)));
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
struct EvaluationTypeDeserializer;
impl EvaluationTypeDeserializer {
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
struct EventIdDeserializer;
impl EventIdDeserializer {
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
#[doc="<p>The input for the <a>ExecuteChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ExecuteChangeSetInput {
    #[doc="<p>The name or ARN of the change set that you want use to update the specified stack.</p>"]
    pub change_set_name: String,
    #[doc="<p>A unique identifier for this <code>ExecuteChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to execute a change set to update a stack with the same name. You might retry <code>ExecuteChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>If you specified the name of a change set, specify the stack name or ID (ARN) that is associated with the change set you want to execute.</p>"]
    pub stack_name: Option<String>,
}


/// Serialize `ExecuteChangeSetInput` contents to a `SignedRequest`.
struct ExecuteChangeSetInputSerializer;
impl ExecuteChangeSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExecuteChangeSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ChangeSetName"),
                   &obj.change_set_name);
        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }

    }
}

#[doc="<p>The output for the <a>ExecuteChangeSet</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ExecuteChangeSetOutput;

struct ExecuteChangeSetOutputDeserializer;
impl ExecuteChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ExecuteChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ExecuteChangeSetOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct ExecutionStatusDeserializer;
impl ExecutionStatusDeserializer {
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
#[doc="<p>The <code>Export</code> structure describes the exported output values for a stack.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Export {
    #[doc="<p>The stack that contains the exported output name and value.</p>"]
    pub exporting_stack_id: Option<String>,
    #[doc="<p>The name of exported output value. Use this name and the <code>Fn::ImportValue</code> function to import the associated value into other stacks. The name is defined in the <code>Export</code> field in the associated stack's <code>Outputs</code> section.</p>"]
    pub name: Option<String>,
    #[doc="<p>The value of the exported output, such as a resource physical ID. This value is defined in the <code>Export</code> field in the associated stack's <code>Outputs</code> section.</p>"]
    pub value: Option<String>,
}

struct ExportDeserializer;
impl ExportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Export, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Export::default();

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
                        "ExportingStackId" => {
                            obj.exporting_stack_id =
                                Some(try!(StackIdDeserializer::deserialize("ExportingStackId",
                                                                           stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(ExportNameDeserializer::deserialize("Name",
                                                                                     stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(ExportValueDeserializer::deserialize("Value",
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
struct ExportNameDeserializer;
impl ExportNameDeserializer {
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
struct ExportValueDeserializer;
impl ExportValueDeserializer {
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
struct ExportsDeserializer;
impl ExportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Export>, XmlParseError> {

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
                        obj.push(try!(ExportDeserializer::deserialize("member", stack)));
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
#[doc="<p>The input for the <a>GetStackPolicy</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetStackPolicyInput {
    #[doc="<p>The name or unique stack ID that is associated with the stack whose policy you want to get.</p>"]
    pub stack_name: String,
}


/// Serialize `GetStackPolicyInput` contents to a `SignedRequest`.
struct GetStackPolicyInputSerializer;
impl GetStackPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetStackPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

#[doc="<p>The output for the <a>GetStackPolicy</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetStackPolicyOutput {
    #[doc="<p>Structure containing the stack policy body. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html\"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide.)</p>"]
    pub stack_policy_body: Option<String>,
}

struct GetStackPolicyOutputDeserializer;
impl GetStackPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetStackPolicyOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetStackPolicyOutput::default();

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
                        "StackPolicyBody" => {
                            obj.stack_policy_body =
                                Some(try!(StackPolicyBodyDeserializer::deserialize("StackPolicyBody",
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
#[doc="<p>The input for a <a>GetTemplate</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetTemplateInput {
    #[doc="<p>The name or Amazon Resource Name (ARN) of a change set for which AWS CloudFormation returns the associated template. If you specify a name, you must also specify the <code>StackName</code>.</p>"]
    pub change_set_name: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>"]
    pub stack_name: Option<String>,
    #[doc="<p>For templates that include transforms, the stage of the template that AWS CloudFormation returns. To get the user-submitted template, specify <code>Original</code>. To get the template after AWS CloudFormation has processed all transforms, specify <code>Processed</code>. </p> <p>If the template doesn't include transforms, <code>Original</code> and <code>Processed</code> return the same template. By default, AWS CloudFormation specifies <code>Original</code>. </p>"]
    pub template_stage: Option<String>,
}


/// Serialize `GetTemplateInput` contents to a `SignedRequest`.
struct GetTemplateInputSerializer;
impl GetTemplateInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTemplateInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.change_set_name {
            params.put(&format!("{}{}", prefix, "ChangeSetName"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
        if let Some(ref field_value) = obj.template_stage {
            params.put(&format!("{}{}", prefix, "TemplateStage"), &field_value);
        }

    }
}

#[doc="<p>The output for <a>GetTemplate</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetTemplateOutput {
    #[doc="<p>The stage of the template that you can retrieve. For stacks, the <code>Original</code> and <code>Processed</code> templates are always available. For change sets, the <code>Original</code> template is always available. After AWS CloudFormation finishes creating the change set, the <code>Processed</code> template becomes available.</p>"]
    pub stages_available: Option<Vec<String>>,
    #[doc="<p>Structure containing the template body. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>AWS CloudFormation returns the same template that was used when the stack was created.</p>"]
    pub template_body: Option<String>,
}

struct GetTemplateOutputDeserializer;
impl GetTemplateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetTemplateOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTemplateOutput::default();

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
                        "StagesAvailable" => {
                            obj.stages_available =
                                Some(try!(StageListDeserializer::deserialize("StagesAvailable",
                                                                             stack)));
                        }
                        "TemplateBody" => {
                            obj.template_body =
                                Some(try!(TemplateBodyDeserializer::deserialize("TemplateBody",
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
#[doc="<p>The input for the <a>GetTemplateSummary</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetTemplateSummaryInput {
    #[doc="<p>The name or the stack ID that is associated with the stack, which are not always interchangeable. For running stacks, you can specify either the stack's name or its unique stack ID. For deleted stack, you must specify the unique stack ID.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>"]
    pub stack_name: Option<String>,
    #[doc="<p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information about templates, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information about templates, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>"]
    pub template_url: Option<String>,
}


/// Serialize `GetTemplateSummaryInput` contents to a `SignedRequest`.
struct GetTemplateSummaryInputSerializer;
impl GetTemplateSummaryInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTemplateSummaryInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }

    }
}

#[doc="<p>The output for the <a>GetTemplateSummary</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct GetTemplateSummaryOutput {
    #[doc="<p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities\">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>The list of resources that generated the values in the <code>Capabilities</code> response element.</p>"]
    pub capabilities_reason: Option<String>,
    #[doc="<p>A list of the transforms that are declared in the template.</p>"]
    pub declared_transforms: Option<Vec<String>>,
    #[doc="<p>The value that is defined in the <code>Description</code> property of the template.</p>"]
    pub description: Option<String>,
    #[doc="<p>The value that is defined for the <code>Metadata</code> property of the template.</p>"]
    pub metadata: Option<String>,
    #[doc="<p>A list of parameter declarations that describe various properties for each parameter.</p>"]
    pub parameters: Option<Vec<ParameterDeclaration>>,
    #[doc="<p>A list of all the template resource types that are defined in the template, such as <code>AWS::EC2::Instance</code>, <code>AWS::Dynamo::Table</code>, and <code>Custom::MyCustomInstance</code>.</p>"]
    pub resource_types: Option<Vec<String>>,
    #[doc="<p>The AWS template format version, which identifies the capabilities of the template.</p>"]
    pub version: Option<String>,
}

struct GetTemplateSummaryOutputDeserializer;
impl GetTemplateSummaryOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<GetTemplateSummaryOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = GetTemplateSummaryOutput::default();

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
                        "Capabilities" => {
                            obj.capabilities =
                                Some(try!(CapabilitiesDeserializer::deserialize("Capabilities",
                                                                                stack)));
                        }
                        "CapabilitiesReason" => {
                            obj.capabilities_reason =
                                Some(try!(CapabilitiesReasonDeserializer::deserialize("CapabilitiesReason",
                                                                                      stack)));
                        }
                        "DeclaredTransforms" => {
                            obj.declared_transforms =
                                Some(try!(TransformsListDeserializer::deserialize("DeclaredTransforms",
                                                                                  stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "Metadata" => {
                            obj.metadata = Some(try!(MetadataDeserializer::deserialize("Metadata",
                                                                                       stack)));
                        }
                        "Parameters" => {
                            obj.parameters =
                                Some(try!(ParameterDeclarationsDeserializer::deserialize("Parameters",
                                                                                         stack)));
                        }
                        "ResourceTypes" => {
                            obj.resource_types =
                                Some(try!(ResourceTypesDeserializer::deserialize("ResourceTypes",
                                                                                 stack)));
                        }
                        "Version" => {
                            obj.version = Some(try!(VersionDeserializer::deserialize("Version",
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
struct ImportsDeserializer;
impl ImportsDeserializer {
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
                    if name == "member" {
                        obj.push(try!(StackNameDeserializer::deserialize("member", stack)));
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
struct LastUpdatedTimeDeserializer;
impl LastUpdatedTimeDeserializer {
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
struct LimitNameDeserializer;
impl LimitNameDeserializer {
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
struct LimitValueDeserializer;
impl LimitValueDeserializer {
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
#[doc="<p>The input for the <a>ListChangeSets</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListChangeSetsInput {
    #[doc="<p>A string (provided by the <a>ListChangeSets</a> response output) that identifies the next page of change sets that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>The name or the Amazon Resource Name (ARN) of the stack for which you want to list change sets.</p>"]
    pub stack_name: String,
}


/// Serialize `ListChangeSetsInput` contents to a `SignedRequest`.
struct ListChangeSetsInputSerializer;
impl ListChangeSetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListChangeSetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

#[doc="<p>The output for the <a>ListChangeSets</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListChangeSetsOutput {
    #[doc="<p>If the output exceeds 1 MB, a string that identifies the next page of change sets. If there is no additional page, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>ChangeSetSummary</code> structures that provides the ID and status of each change set for the specified stack.</p>"]
    pub summaries: Option<Vec<ChangeSetSummary>>,
}

struct ListChangeSetsOutputDeserializer;
impl ListChangeSetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListChangeSetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListChangeSetsOutput::default();

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
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "Summaries" => {
                            obj.summaries =
                                Some(try!(ChangeSetSummariesDeserializer::deserialize("Summaries",
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
#[derive(Default,Debug,Clone)]
pub struct ListExportsInput {
    #[doc="<p>A string (provided by the <a>ListExports</a> response output) that identifies the next page of exported output values that you asked to retrieve.</p>"]
    pub next_token: Option<String>,
}


/// Serialize `ListExportsInput` contents to a `SignedRequest`.
struct ListExportsInputSerializer;
impl ListExportsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListExportsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ListExportsOutput {
    #[doc="<p>The output for the <a>ListExports</a> action.</p>"]
    pub exports: Option<Vec<Export>>,
    #[doc="<p>If the output exceeds 100 exported output values, a string that identifies the next page of exports. If there is no additional page, this value is null.</p>"]
    pub next_token: Option<String>,
}

struct ListExportsOutputDeserializer;
impl ListExportsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListExportsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListExportsOutput::default();

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
                        "Exports" => {
                            obj.exports = Some(try!(ExportsDeserializer::deserialize("Exports",
                                                                                     stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
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
#[derive(Default,Debug,Clone)]
pub struct ListImportsInput {
    #[doc="<p>The name of the exported output value. AWS CloudFormation returns the stack names that are importing this value. </p>"]
    pub export_name: String,
    #[doc="<p>A string (provided by the <a>ListImports</a> response output) that identifies the next page of stacks that are importing the specified exported output value. </p>"]
    pub next_token: Option<String>,
}


/// Serialize `ListImportsInput` contents to a `SignedRequest`.
struct ListImportsInputSerializer;
impl ListImportsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListImportsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ExportName"), &obj.export_name);
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ListImportsOutput {
    #[doc="<p>A list of stack names that are importing the specified exported output value. </p>"]
    pub imports: Option<Vec<String>>,
    #[doc="<p>A string that identifies the next page of exports. If there is no additional page, this value is null.</p>"]
    pub next_token: Option<String>,
}

struct ListImportsOutputDeserializer;
impl ListImportsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListImportsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListImportsOutput::default();

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
                        "Imports" => {
                            obj.imports = Some(try!(ImportsDeserializer::deserialize("Imports",
                                                                                     stack)));
                        }
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
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
#[doc="<p>The input for the <a>ListStackResource</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListStackResourcesInput {
    #[doc="<p>A string that identifies the next page of stack resources that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>"]
    pub stack_name: String,
}


/// Serialize `ListStackResourcesInput` contents to a `SignedRequest`.
struct ListStackResourcesInputSerializer;
impl ListStackResourcesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackResourcesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);

    }
}

#[doc="<p>The output for a <a>ListStackResources</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListStackResourcesOutput {
    #[doc="<p>If the output exceeds 1 MB, a string that identifies the next page of stack resources. If no additional page exists, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>StackResourceSummary</code> structures.</p>"]
    pub stack_resource_summaries: Option<Vec<StackResourceSummary>>,
}

struct ListStackResourcesOutputDeserializer;
impl ListStackResourcesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListStackResourcesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStackResourcesOutput::default();

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
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "StackResourceSummaries" => {
                            obj.stack_resource_summaries =
                                Some(try!(StackResourceSummariesDeserializer::deserialize("StackResourceSummaries",
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
#[doc="<p>The input for <a>ListStacks</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListStacksInput {
    #[doc="<p>A string that identifies the next page of stacks that you want to retrieve.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>Stack status to use as a filter. Specify one or more stack status codes to list only stacks with the specified status codes. For a complete list of stack status codes, see the <code>StackStatus</code> parameter of the <a>Stack</a> data type.</p>"]
    pub stack_status_filter: Option<Vec<String>>,
}


/// Serialize `ListStacksInput` contents to a `SignedRequest`.
struct ListStacksInputSerializer;
impl ListStacksInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStacksInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_status_filter {
            StackStatusFilterSerializer::serialize(params,
                                                   &format!("{}{}", prefix, "StackStatusFilter"),
                                                   field_value);
        }

    }
}

#[doc="<p>The output for <a>ListStacks</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListStacksOutput {
    #[doc="<p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>"]
    pub next_token: Option<String>,
    #[doc="<p>A list of <code>StackSummary</code> structures containing information about the specified stacks.</p>"]
    pub stack_summaries: Option<Vec<StackSummary>>,
}

struct ListStacksOutputDeserializer;
impl ListStacksOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ListStacksOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStacksOutput::default();

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
                        "NextToken" => {
                            obj.next_token = Some(try!(NextTokenDeserializer::deserialize("NextToken",
                                                                                          stack)));
                        }
                        "StackSummaries" => {
                            obj.stack_summaries =
                                Some(try!(StackSummariesDeserializer::deserialize("StackSummaries",
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
struct LogicalResourceIdDeserializer;
impl LogicalResourceIdDeserializer {
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
struct MetadataDeserializer;
impl MetadataDeserializer {
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
struct NoEchoDeserializer;
impl NoEchoDeserializer {
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
struct NotificationARNDeserializer;
impl NotificationARNDeserializer {
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
struct NotificationARNsDeserializer;
impl NotificationARNsDeserializer {
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
                    if name == "member" {
                        obj.push(try!(NotificationARNDeserializer::deserialize("member", stack)));
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

/// Serialize `NotificationARNs` contents to a `SignedRequest`.
struct NotificationARNsSerializer;
impl NotificationARNsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[doc="<p>The Output data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Output {
    #[doc="<p>User defined description associated with the output.</p>"]
    pub description: Option<String>,
    #[doc="<p>The key associated with the output.</p>"]
    pub output_key: Option<String>,
    #[doc="<p>The value associated with the output.</p>"]
    pub output_value: Option<String>,
}

struct OutputDeserializer;
impl OutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Output, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Output::default();

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
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "OutputKey" => {
                            obj.output_key = Some(try!(OutputKeyDeserializer::deserialize("OutputKey",
                                                                                          stack)));
                        }
                        "OutputValue" => {
                            obj.output_value =
                                Some(try!(OutputValueDeserializer::deserialize("OutputValue",
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
struct OutputKeyDeserializer;
impl OutputKeyDeserializer {
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
struct OutputValueDeserializer;
impl OutputValueDeserializer {
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
struct OutputsDeserializer;
impl OutputsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Output>, XmlParseError> {

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
                        obj.push(try!(OutputDeserializer::deserialize("member", stack)));
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
#[doc="<p>The Parameter data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Parameter {
    #[doc="<p>The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation uses the default value that is specified in your template.</p>"]
    pub parameter_key: Option<String>,
    #[doc="<p>The value associated with the parameter.</p>"]
    pub parameter_value: Option<String>,
    #[doc="<p>During a stack update, use the existing parameter value that the stack is using for a given parameter key. If you specify <code>true</code>, do not specify a parameter value.</p>"]
    pub use_previous_value: Option<bool>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Parameter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Parameter::default();

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
                        "ParameterKey" => {
                            obj.parameter_key =
                                Some(try!(ParameterKeyDeserializer::deserialize("ParameterKey",
                                                                                stack)));
                        }
                        "ParameterValue" => {
                            obj.parameter_value =
                                Some(try!(ParameterValueDeserializer::deserialize("ParameterValue",
                                                                                  stack)));
                        }
                        "UsePreviousValue" => {
                            obj.use_previous_value =
                                Some(try!(UsePreviousValueDeserializer::deserialize("UsePreviousValue",
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

/// Serialize `Parameter` contents to a `SignedRequest`.
struct ParameterSerializer;
impl ParameterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Parameter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.parameter_key {
            params.put(&format!("{}{}", prefix, "ParameterKey"), &field_value);
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(&format!("{}{}", prefix, "ParameterValue"), &field_value);
        }
        if let Some(ref field_value) = obj.use_previous_value {
            params.put(&format!("{}{}", prefix, "UsePreviousValue"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>A set of criteria that AWS CloudFormation uses to validate parameter values. Although other constraints might be defined in the stack template, AWS CloudFormation returns only the <code>AllowedValues</code> property.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ParameterConstraints {
    #[doc="<p>A list of values that are permitted for a parameter.</p>"]
    pub allowed_values: Option<Vec<String>>,
}

struct ParameterConstraintsDeserializer;
impl ParameterConstraintsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ParameterConstraints, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ParameterConstraints::default();

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
                        "AllowedValues" => {
                            obj.allowed_values =
                                Some(try!(AllowedValuesDeserializer::deserialize("AllowedValues",
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
#[doc="<p>The ParameterDeclaration data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ParameterDeclaration {
    #[doc="<p>The default value of the parameter.</p>"]
    pub default_value: Option<String>,
    #[doc="<p>The description that is associate with the parameter.</p>"]
    pub description: Option<String>,
    #[doc="<p>Flag that indicates whether the parameter value is shown as plain text in logs and in the AWS Management Console.</p>"]
    pub no_echo: Option<bool>,
    #[doc="<p>The criteria that AWS CloudFormation uses to validate parameter values.</p>"]
    pub parameter_constraints: Option<ParameterConstraints>,
    #[doc="<p>The name that is associated with the parameter.</p>"]
    pub parameter_key: Option<String>,
    #[doc="<p>The type of parameter.</p>"]
    pub parameter_type: Option<String>,
}

struct ParameterDeclarationDeserializer;
impl ParameterDeclarationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ParameterDeclaration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ParameterDeclaration::default();

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
                        "DefaultValue" => {
                            obj.default_value =
                                Some(try!(ParameterValueDeserializer::deserialize("DefaultValue",
                                                                                  stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "NoEcho" => {
                            obj.no_echo = Some(try!(NoEchoDeserializer::deserialize("NoEcho",
                                                                                    stack)));
                        }
                        "ParameterConstraints" => {
                            obj.parameter_constraints =
                                Some(try!(ParameterConstraintsDeserializer::deserialize("ParameterConstraints",
                                                                                        stack)));
                        }
                        "ParameterKey" => {
                            obj.parameter_key =
                                Some(try!(ParameterKeyDeserializer::deserialize("ParameterKey",
                                                                                stack)));
                        }
                        "ParameterType" => {
                            obj.parameter_type =
                                Some(try!(ParameterTypeDeserializer::deserialize("ParameterType",
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
struct ParameterDeclarationsDeserializer;
impl ParameterDeclarationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<ParameterDeclaration>, XmlParseError> {

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
                        obj.push(try!(ParameterDeclarationDeserializer::deserialize("member",
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
struct ParameterKeyDeserializer;
impl ParameterKeyDeserializer {
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
struct ParameterTypeDeserializer;
impl ParameterTypeDeserializer {
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
struct ParameterValueDeserializer;
impl ParameterValueDeserializer {
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
struct ParametersDeserializer;
impl ParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Parameter>, XmlParseError> {

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
                        obj.push(try!(ParameterDeserializer::deserialize("member", stack)));
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

/// Serialize `Parameters` contents to a `SignedRequest`.
struct ParametersSerializer;
impl ParametersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Parameter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ParameterSerializer::serialize(params, &key, obj);
        }
    }
}

struct PhysicalResourceIdDeserializer;
impl PhysicalResourceIdDeserializer {
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
struct PropertyNameDeserializer;
impl PropertyNameDeserializer {
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
struct ReplacementDeserializer;
impl ReplacementDeserializer {
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
struct RequiresRecreationDeserializer;
impl RequiresRecreationDeserializer {
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
struct ResourceAttributeDeserializer;
impl ResourceAttributeDeserializer {
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
#[doc="<p>The <code>ResourceChange</code> structure describes the resource and the action that AWS CloudFormation will perform on it if you execute this change set.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ResourceChange {
    #[doc="<p>The action that AWS CloudFormation takes on the resource, such as <code>Add</code> (adds a new resource), <code>Modify</code> (changes a resource), or <code>Remove</code> (deletes a resource).</p>"]
    pub action: Option<String>,
    #[doc="<p>For the <code>Modify</code> action, a list of <code>ResourceChangeDetail</code> structures that describes the changes that AWS CloudFormation will make to the resource. </p>"]
    pub details: Option<Vec<ResourceChangeDetail>>,
    #[doc="<p>The resource's logical ID, which is defined in the stack's template.</p>"]
    pub logical_resource_id: Option<String>,
    #[doc="<p>The resource's physical ID (resource name). Resources that you are adding don't have physical IDs because they haven't been created.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>For the <code>Modify</code> action, indicates whether AWS CloudFormation will replace the resource by creating a new one and deleting the old one. This value depends on the value of the <code>RequiresRecreation</code> property in the <code>ResourceTargetDefinition</code> structure. For example, if the <code>RequiresRecreation</code> field is <code>Always</code> and the <code>Evaluation</code> field is <code>Static</code>, <code>Replacement</code> is <code>True</code>. If the <code>RequiresRecreation</code> field is <code>Always</code> and the <code>Evaluation</code> field is <code>Dynamic</code>, <code>Replacement</code> is <code>Conditionally</code>.</p> <p>If you have multiple changes with different <code>RequiresRecreation</code> values, the <code>Replacement</code> value depends on the change with the most impact. A <code>RequiresRecreation</code> value of <code>Always</code> has the most impact, followed by <code>Conditionally</code>, and then <code>Never</code>.</p>"]
    pub replacement: Option<String>,
    #[doc="<p>The type of AWS CloudFormation resource, such as <code>AWS::S3::Bucket</code>.</p>"]
    pub resource_type: Option<String>,
    #[doc="<p>For the <code>Modify</code> action, indicates which resource attribute is triggering this update, such as a change in the resource attribute's <code>Metadata</code>, <code>Properties</code>, or <code>Tags</code>.</p>"]
    pub scope: Option<Vec<String>>,
}

struct ResourceChangeDeserializer;
impl ResourceChangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ResourceChange, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceChange::default();

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
                        "Action" => {
                            obj.action = Some(try!(ChangeActionDeserializer::deserialize("Action",
                                                                                         stack)));
                        }
                        "Details" => {
                            obj.details =
                                Some(try!(ResourceChangeDetailsDeserializer::deserialize("Details",
                                                                                         stack)));
                        }
                        "LogicalResourceId" => {
                            obj.logical_resource_id =
                                Some(try!(LogicalResourceIdDeserializer::deserialize("LogicalResourceId",
                                                                                     stack)));
                        }
                        "PhysicalResourceId" => {
                            obj.physical_resource_id =
                                Some(try!(PhysicalResourceIdDeserializer::deserialize("PhysicalResourceId",
                                                                                      stack)));
                        }
                        "Replacement" => {
                            obj.replacement =
                                Some(try!(ReplacementDeserializer::deserialize("Replacement",
                                                                               stack)));
                        }
                        "ResourceType" => {
                            obj.resource_type =
                                Some(try!(ResourceTypeDeserializer::deserialize("ResourceType",
                                                                                stack)));
                        }
                        "Scope" => {
                            obj.scope = Some(try!(ScopeDeserializer::deserialize("Scope", stack)));
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
#[doc="<p>For a resource with <code>Modify</code> as the action, the <code>ResourceChange</code> structure describes the changes AWS CloudFormation will make to that resource.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ResourceChangeDetail {
    #[doc="<p>The identity of the entity that triggered this change. This entity is a member of the group that is specified by the <code>ChangeSource</code> field. For example, if you modified the value of the <code>KeyPairName</code> parameter, the <code>CausingEntity</code> is the name of the parameter (<code>KeyPairName</code>).</p> <p>If the <code>ChangeSource</code> value is <code>DirectModification</code>, no value is given for <code>CausingEntity</code>.</p>"]
    pub causing_entity: Option<String>,
    #[doc="<p>The group to which the <code>CausingEntity</code> value belongs. There are five entity groups:</p> <ul> <li> <p> <code>ResourceReference</code> entities are <code>Ref</code> intrinsic functions that refer to resources in the template, such as <code>{ \"Ref\" : \"MyEC2InstanceResource\" }</code>.</p> </li> <li> <p> <code>ParameterReference</code> entities are <code>Ref</code> intrinsic functions that get template parameter values, such as <code>{ \"Ref\" : \"MyPasswordParameter\" }</code>.</p> </li> <li> <p> <code>ResourceAttribute</code> entities are <code>Fn::GetAtt</code> intrinsic functions that get resource attribute values, such as <code>{ \"Fn::GetAtt\" : [ \"MyEC2InstanceResource\", \"PublicDnsName\" ] }</code>.</p> </li> <li> <p> <code>DirectModification</code> entities are changes that are made directly to the template.</p> </li> <li> <p> <code>Automatic</code> entities are <code>AWS::CloudFormation::Stack</code> resource types, which are also known as nested stacks. If you made no changes to the <code>AWS::CloudFormation::Stack</code> resource, AWS CloudFormation sets the <code>ChangeSource</code> to <code>Automatic</code> because the nested stack's template might have changed. Changes to a nested stack's template aren't visible to AWS CloudFormation until you run an update on the parent stack.</p> </li> </ul>"]
    pub change_source: Option<String>,
    #[doc="<p>Indicates whether AWS CloudFormation can determine the target value, and whether the target value will change before you execute a change set.</p> <p>For <code>Static</code> evaluations, AWS CloudFormation can determine that the target value will change, and its value. For example, if you directly modify the <code>InstanceType</code> property of an EC2 instance, AWS CloudFormation knows that this property value will change, and its value, so this is a <code>Static</code> evaluation.</p> <p>For <code>Dynamic</code> evaluations, cannot determine the target value because it depends on the result of an intrinsic function, such as a <code>Ref</code> or <code>Fn::GetAtt</code> intrinsic function, when the stack is updated. For example, if your template includes a reference to a resource that is conditionally recreated, the value of the reference (the physical ID of the resource) might change, depending on if the resource is recreated. If the resource is recreated, it will have a new physical ID, so all references to that resource will also be updated.</p>"]
    pub evaluation: Option<String>,
    #[doc="<p>A <code>ResourceTargetDefinition</code> structure that describes the field that AWS CloudFormation will change and whether the resource will be recreated.</p>"]
    pub target: Option<ResourceTargetDefinition>,
}

struct ResourceChangeDetailDeserializer;
impl ResourceChangeDetailDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ResourceChangeDetail, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceChangeDetail::default();

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
                        "CausingEntity" => {
                            obj.causing_entity =
                                Some(try!(CausingEntityDeserializer::deserialize("CausingEntity",
                                                                                 stack)));
                        }
                        "ChangeSource" => {
                            obj.change_source =
                                Some(try!(ChangeSourceDeserializer::deserialize("ChangeSource",
                                                                                stack)));
                        }
                        "Evaluation" => {
                            obj.evaluation =
                                Some(try!(EvaluationTypeDeserializer::deserialize("Evaluation",
                                                                                  stack)));
                        }
                        "Target" => {
                            obj.target = Some(try!(ResourceTargetDefinitionDeserializer::deserialize("Target", stack)));
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
struct ResourceChangeDetailsDeserializer;
impl ResourceChangeDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<ResourceChangeDetail>, XmlParseError> {

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
                        obj.push(try!(ResourceChangeDetailDeserializer::deserialize("member",
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
struct ResourcePropertiesDeserializer;
impl ResourcePropertiesDeserializer {
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
struct ResourceStatusDeserializer;
impl ResourceStatusDeserializer {
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
struct ResourceStatusReasonDeserializer;
impl ResourceStatusReasonDeserializer {
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
#[doc="<p>The field that AWS CloudFormation will change, such as the name of a resource's property, and whether the resource will be recreated.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ResourceTargetDefinition {
    #[doc="<p>Indicates which resource attribute is triggering this update, such as a change in the resource attribute's <code>Metadata</code>, <code>Properties</code>, or <code>Tags</code>.</p>"]
    pub attribute: Option<String>,
    #[doc="<p>If the <code>Attribute</code> value is <code>Properties</code>, the name of the property. For all other attributes, the value is null.</p>"]
    pub name: Option<String>,
    #[doc="<p>If the <code>Attribute</code> value is <code>Properties</code>, indicates whether a change to this property causes the resource to be recreated. The value can be <code>Never</code>, <code>Always</code>, or <code>Conditionally</code>. To determine the conditions for a <code>Conditionally</code> recreation, see the update behavior for that <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html\">property</a> in the AWS CloudFormation User Guide.</p>"]
    pub requires_recreation: Option<String>,
}

struct ResourceTargetDefinitionDeserializer;
impl ResourceTargetDefinitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ResourceTargetDefinition, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ResourceTargetDefinition::default();

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
                        "Attribute" => {
                            obj.attribute =
                                Some(try!(ResourceAttributeDeserializer::deserialize("Attribute",
                                                                                     stack)));
                        }
                        "Name" => {
                            obj.name = Some(try!(PropertyNameDeserializer::deserialize("Name",
                                                                                       stack)));
                        }
                        "RequiresRecreation" => {
                            obj.requires_recreation =
                                Some(try!(RequiresRecreationDeserializer::deserialize("RequiresRecreation",
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
struct ResourceTypeDeserializer;
impl ResourceTypeDeserializer {
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
struct ResourceTypesDeserializer;
impl ResourceTypesDeserializer {
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
                    if name == "member" {
                        obj.push(try!(ResourceTypeDeserializer::deserialize("member", stack)));
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

/// Serialize `ResourceTypes` contents to a `SignedRequest`.
struct ResourceTypesSerializer;
impl ResourceTypesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}


/// Serialize `ResourcesToSkip` contents to a `SignedRequest`.
struct ResourcesToSkipSerializer;
impl ResourcesToSkipSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}


/// Serialize `RetainResources` contents to a `SignedRequest`.
struct RetainResourcesSerializer;
impl RetainResourcesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct RoleARNDeserializer;
impl RoleARNDeserializer {
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
struct ScopeDeserializer;
impl ScopeDeserializer {
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
                    if name == "member" {
                        obj.push(try!(ResourceAttributeDeserializer::deserialize("member", stack)));
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
#[doc="<p>The input for the <a>SetStackPolicy</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct SetStackPolicyInput {
    #[doc="<p>The name or unique stack ID that you want to associate a policy with.</p>"]
    pub stack_name: String,
    #[doc="<p>Structure containing the stack policy body. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html\"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>"]
    pub stack_policy_body: Option<String>,
    #[doc="<p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>"]
    pub stack_policy_url: Option<String>,
}


/// Serialize `SetStackPolicyInput` contents to a `SignedRequest`.
struct SetStackPolicyInputSerializer;
impl SetStackPolicyInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetStackPolicyInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(&format!("{}{}", prefix, "StackPolicyBody"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(&format!("{}{}", prefix, "StackPolicyURL"), &field_value);
        }

    }
}

#[doc="<p>The input for the <a>SignalResource</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct SignalResourceInput {
    #[doc="<p>The logical ID of the resource that you want to signal. The logical ID is the name of the resource that given in the template.</p>"]
    pub logical_resource_id: String,
    #[doc="<p>The stack name or unique stack ID that includes the resource that you want to signal.</p>"]
    pub stack_name: String,
    #[doc="<p>The status of the signal, which is either success or failure. A failure signal causes AWS CloudFormation to immediately fail the stack creation or update.</p>"]
    pub status: String,
    #[doc="<p>A unique ID of the signal. When you signal Amazon EC2 instances or Auto Scaling groups, specify the instance ID that you are signaling as the unique ID. If you send multiple signals to a single resource (such as signaling a wait condition), each signal requires a different unique ID.</p>"]
    pub unique_id: String,
}


/// Serialize `SignalResourceInput` contents to a `SignedRequest`.
struct SignalResourceInputSerializer;
impl SignalResourceInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SignalResourceInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "LogicalResourceId"),
                   &obj.logical_resource_id);
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        params.put(&format!("{}{}", prefix, "Status"), &obj.status);
        params.put(&format!("{}{}", prefix, "UniqueId"), &obj.unique_id);

    }
}

#[doc="<p>The Stack data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Stack {
    #[doc="<p>The capabilities allowed in the stack.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>The unique ID of the change set.</p>"]
    pub change_set_id: Option<String>,
    #[doc="<p>The time at which the stack was created.</p>"]
    pub creation_time: String,
    #[doc="<p>A user-defined description associated with the stack.</p>"]
    pub description: Option<String>,
    #[doc="<p>Boolean to enable or disable rollback on stack creation failures:</p> <ul> <li> <p> <code>true</code>: disable rollback</p> </li> <li> <p> <code>false</code>: enable rollback</p> </li> </ul>"]
    pub disable_rollback: Option<bool>,
    #[doc="<p>The time the stack was last updated. This field will only be returned if the stack has been updated at least once.</p>"]
    pub last_updated_time: Option<String>,
    #[doc="<p>SNS topic ARNs to which stack related events are published.</p>"]
    pub notification_ar_ns: Option<Vec<String>>,
    #[doc="<p>A list of output structures.</p>"]
    pub outputs: Option<Vec<Output>>,
    #[doc="<p>A list of <code>Parameter</code> structures.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that is associated with the stack. During a stack operation, AWS CloudFormation uses this role's credentials to make calls on your behalf.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>Unique identifier of the stack.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name associated with the stack.</p>"]
    pub stack_name: String,
    #[doc="<p>Current status of the stack.</p>"]
    pub stack_status: String,
    #[doc="<p>Success/failure message associated with the stack status.</p>"]
    pub stack_status_reason: Option<String>,
    #[doc="<p>A list of <code>Tag</code>s that specify information about the stack.</p>"]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>The amount of time within which stack creation should complete.</p>"]
    pub timeout_in_minutes: Option<i64>,
}

struct StackDeserializer;
impl StackDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Stack, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Stack::default();

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
                        "Capabilities" => {
                            obj.capabilities =
                                Some(try!(CapabilitiesDeserializer::deserialize("Capabilities",
                                                                                stack)));
                        }
                        "ChangeSetId" => {
                            obj.change_set_id =
                                Some(try!(ChangeSetIdDeserializer::deserialize("ChangeSetId",
                                                                               stack)));
                        }
                        "CreationTime" => {
                            obj.creation_time = try!(CreationTimeDeserializer::deserialize("CreationTime",
                                                                                           stack));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "DisableRollback" => {
                            obj.disable_rollback =
                                Some(try!(DisableRollbackDeserializer::deserialize("DisableRollback",
                                                                                   stack)));
                        }
                        "LastUpdatedTime" => {
                            obj.last_updated_time =
                                Some(try!(LastUpdatedTimeDeserializer::deserialize("LastUpdatedTime",
                                                                                   stack)));
                        }
                        "NotificationARNs" => {
                            obj.notification_ar_ns =
                                Some(try!(NotificationARNsDeserializer::deserialize("NotificationARNs",
                                                                                    stack)));
                        }
                        "Outputs" => {
                            obj.outputs = Some(try!(OutputsDeserializer::deserialize("Outputs",
                                                                                     stack)));
                        }
                        "Parameters" => {
                            obj.parameters = Some(try!(ParametersDeserializer::deserialize("Parameters",
                                                                                           stack)));
                        }
                        "RoleARN" => {
                            obj.role_arn = Some(try!(RoleARNDeserializer::deserialize("RoleARN",
                                                                                      stack)));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = try!(StackNameDeserializer::deserialize("StackName",
                                                                                     stack));
                        }
                        "StackStatus" => {
                            obj.stack_status = try!(StackStatusDeserializer::deserialize("StackStatus",
                                                                                         stack));
                        }
                        "StackStatusReason" => {
                            obj.stack_status_reason =
                                Some(try!(StackStatusReasonDeserializer::deserialize("StackStatusReason",
                                                                                     stack)));
                        }
                        "Tags" => {
                            obj.tags = Some(try!(TagsDeserializer::deserialize("Tags", stack)));
                        }
                        "TimeoutInMinutes" => {
                            obj.timeout_in_minutes =
                                Some(try!(TimeoutMinutesDeserializer::deserialize("TimeoutInMinutes",
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
#[doc="<p>The StackEvent data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct StackEvent {
    #[doc="<p>The token passed to the operation that generated this event.</p> <p>For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>The unique ID of this event.</p>"]
    pub event_id: String,
    #[doc="<p>The logical name of the resource specified in the template.</p>"]
    pub logical_resource_id: Option<String>,
    #[doc="<p>The name or unique identifier associated with the physical instance of the resource.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>BLOB of the properties used to create the resource.</p>"]
    pub resource_properties: Option<String>,
    #[doc="<p>Current status of the resource.</p>"]
    pub resource_status: Option<String>,
    #[doc="<p>Success/failure message associated with the resource.</p>"]
    pub resource_status_reason: Option<String>,
    #[doc="<p>Type of resource. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html\"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>"]
    pub resource_type: Option<String>,
    #[doc="<p>The unique ID name of the instance of the stack.</p>"]
    pub stack_id: String,
    #[doc="<p>The name associated with a stack.</p>"]
    pub stack_name: String,
    #[doc="<p>Time the status was updated.</p>"]
    pub timestamp: String,
}

struct StackEventDeserializer;
impl StackEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StackEvent, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackEvent::default();

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
                        "ClientRequestToken" => {
                            obj.client_request_token =
                                Some(try!(ClientRequestTokenDeserializer::deserialize("ClientRequestToken",
                                                                                      stack)));
                        }
                        "EventId" => {
                            obj.event_id = try!(EventIdDeserializer::deserialize("EventId", stack));
                        }
                        "LogicalResourceId" => {
                            obj.logical_resource_id =
                                Some(try!(LogicalResourceIdDeserializer::deserialize("LogicalResourceId",
                                                                                     stack)));
                        }
                        "PhysicalResourceId" => {
                            obj.physical_resource_id =
                                Some(try!(PhysicalResourceIdDeserializer::deserialize("PhysicalResourceId",
                                                                                      stack)));
                        }
                        "ResourceProperties" => {
                            obj.resource_properties =
                                Some(try!(ResourcePropertiesDeserializer::deserialize("ResourceProperties",
                                                                                      stack)));
                        }
                        "ResourceStatus" => {
                            obj.resource_status =
                                Some(try!(ResourceStatusDeserializer::deserialize("ResourceStatus",
                                                                                  stack)));
                        }
                        "ResourceStatusReason" => {
                            obj.resource_status_reason =
                                Some(try!(ResourceStatusReasonDeserializer::deserialize("ResourceStatusReason",
                                                                                        stack)));
                        }
                        "ResourceType" => {
                            obj.resource_type =
                                Some(try!(ResourceTypeDeserializer::deserialize("ResourceType",
                                                                                stack)));
                        }
                        "StackId" => {
                            obj.stack_id = try!(StackIdDeserializer::deserialize("StackId", stack));
                        }
                        "StackName" => {
                            obj.stack_name = try!(StackNameDeserializer::deserialize("StackName",
                                                                                     stack));
                        }
                        "Timestamp" => {
                            obj.timestamp = try!(TimestampDeserializer::deserialize("Timestamp",
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
struct StackEventsDeserializer;
impl StackEventsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<StackEvent>, XmlParseError> {

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
                        obj.push(try!(StackEventDeserializer::deserialize("member", stack)));
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
struct StackIdDeserializer;
impl StackIdDeserializer {
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
struct StackNameDeserializer;
impl StackNameDeserializer {
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
struct StackPolicyBodyDeserializer;
impl StackPolicyBodyDeserializer {
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
#[doc="<p>The StackResource data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct StackResource {
    #[doc="<p>User defined description associated with the resource.</p>"]
    pub description: Option<String>,
    #[doc="<p>The logical name of the resource specified in the template.</p>"]
    pub logical_resource_id: String,
    #[doc="<p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>Current status of the resource.</p>"]
    pub resource_status: String,
    #[doc="<p>Success/failure message associated with the resource.</p>"]
    pub resource_status_reason: Option<String>,
    #[doc="<p>Type of resource. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html\"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>"]
    pub resource_type: String,
    #[doc="<p>Unique identifier of the stack.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name associated with the stack.</p>"]
    pub stack_name: Option<String>,
    #[doc="<p>Time the status was updated.</p>"]
    pub timestamp: String,
}

struct StackResourceDeserializer;
impl StackResourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StackResource, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackResource::default();

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
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "LogicalResourceId" => {
                            obj.logical_resource_id =
                                try!(LogicalResourceIdDeserializer::deserialize("LogicalResourceId",
                                                                                stack));
                        }
                        "PhysicalResourceId" => {
                            obj.physical_resource_id =
                                Some(try!(PhysicalResourceIdDeserializer::deserialize("PhysicalResourceId",
                                                                                      stack)));
                        }
                        "ResourceStatus" => {
                            obj.resource_status =
                                try!(ResourceStatusDeserializer::deserialize("ResourceStatus",
                                                                             stack));
                        }
                        "ResourceStatusReason" => {
                            obj.resource_status_reason =
                                Some(try!(ResourceStatusReasonDeserializer::deserialize("ResourceStatusReason",
                                                                                        stack)));
                        }
                        "ResourceType" => {
                            obj.resource_type = try!(ResourceTypeDeserializer::deserialize("ResourceType",
                                                                                           stack));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = Some(try!(StackNameDeserializer::deserialize("StackName",
                                                                                          stack)));
                        }
                        "Timestamp" => {
                            obj.timestamp = try!(TimestampDeserializer::deserialize("Timestamp",
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
#[doc="<p>Contains detailed information about the specified stack resource.</p>"]
#[derive(Default,Debug,Clone)]
pub struct StackResourceDetail {
    #[doc="<p>User defined description associated with the resource.</p>"]
    pub description: Option<String>,
    #[doc="<p>Time the status was updated.</p>"]
    pub last_updated_timestamp: String,
    #[doc="<p>The logical name of the resource specified in the template.</p>"]
    pub logical_resource_id: String,
    #[doc="<p>The content of the <code>Metadata</code> attribute declared for the resource. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-attribute-metadata.html\">Metadata Attribute</a> in the AWS CloudFormation User Guide.</p>"]
    pub metadata: Option<String>,
    #[doc="<p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>Current status of the resource.</p>"]
    pub resource_status: String,
    #[doc="<p>Success/failure message associated with the resource.</p>"]
    pub resource_status_reason: Option<String>,
    #[doc="<p>Type of resource. ((For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html\"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>"]
    pub resource_type: String,
    #[doc="<p>Unique identifier of the stack.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name associated with the stack.</p>"]
    pub stack_name: Option<String>,
}

struct StackResourceDetailDeserializer;
impl StackResourceDetailDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StackResourceDetail, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackResourceDetail::default();

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
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "LastUpdatedTimestamp" => {
                            obj.last_updated_timestamp =
                                try!(TimestampDeserializer::deserialize("LastUpdatedTimestamp",
                                                                        stack));
                        }
                        "LogicalResourceId" => {
                            obj.logical_resource_id =
                                try!(LogicalResourceIdDeserializer::deserialize("LogicalResourceId",
                                                                                stack));
                        }
                        "Metadata" => {
                            obj.metadata = Some(try!(MetadataDeserializer::deserialize("Metadata",
                                                                                       stack)));
                        }
                        "PhysicalResourceId" => {
                            obj.physical_resource_id =
                                Some(try!(PhysicalResourceIdDeserializer::deserialize("PhysicalResourceId",
                                                                                      stack)));
                        }
                        "ResourceStatus" => {
                            obj.resource_status =
                                try!(ResourceStatusDeserializer::deserialize("ResourceStatus",
                                                                             stack));
                        }
                        "ResourceStatusReason" => {
                            obj.resource_status_reason =
                                Some(try!(ResourceStatusReasonDeserializer::deserialize("ResourceStatusReason",
                                                                                        stack)));
                        }
                        "ResourceType" => {
                            obj.resource_type = try!(ResourceTypeDeserializer::deserialize("ResourceType",
                                                                                           stack));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = Some(try!(StackNameDeserializer::deserialize("StackName",
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
struct StackResourceSummariesDeserializer;
impl StackResourceSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<StackResourceSummary>, XmlParseError> {

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
                        obj.push(try!(StackResourceSummaryDeserializer::deserialize("member",
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
#[doc="<p>Contains high-level information about the specified stack resource.</p>"]
#[derive(Default,Debug,Clone)]
pub struct StackResourceSummary {
    #[doc="<p>Time the status was updated.</p>"]
    pub last_updated_timestamp: String,
    #[doc="<p>The logical name of the resource specified in the template.</p>"]
    pub logical_resource_id: String,
    #[doc="<p>The name or unique identifier that corresponds to a physical instance ID of the resource.</p>"]
    pub physical_resource_id: Option<String>,
    #[doc="<p>Current status of the resource.</p>"]
    pub resource_status: String,
    #[doc="<p>Success/failure message associated with the resource.</p>"]
    pub resource_status_reason: Option<String>,
    #[doc="<p>Type of resource. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html\"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>"]
    pub resource_type: String,
}

struct StackResourceSummaryDeserializer;
impl StackResourceSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StackResourceSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackResourceSummary::default();

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
                        "LastUpdatedTimestamp" => {
                            obj.last_updated_timestamp =
                                try!(TimestampDeserializer::deserialize("LastUpdatedTimestamp",
                                                                        stack));
                        }
                        "LogicalResourceId" => {
                            obj.logical_resource_id =
                                try!(LogicalResourceIdDeserializer::deserialize("LogicalResourceId",
                                                                                stack));
                        }
                        "PhysicalResourceId" => {
                            obj.physical_resource_id =
                                Some(try!(PhysicalResourceIdDeserializer::deserialize("PhysicalResourceId",
                                                                                      stack)));
                        }
                        "ResourceStatus" => {
                            obj.resource_status =
                                try!(ResourceStatusDeserializer::deserialize("ResourceStatus",
                                                                             stack));
                        }
                        "ResourceStatusReason" => {
                            obj.resource_status_reason =
                                Some(try!(ResourceStatusReasonDeserializer::deserialize("ResourceStatusReason",
                                                                                        stack)));
                        }
                        "ResourceType" => {
                            obj.resource_type = try!(ResourceTypeDeserializer::deserialize("ResourceType",
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
struct StackResourcesDeserializer;
impl StackResourcesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<StackResource>, XmlParseError> {

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
                        obj.push(try!(StackResourceDeserializer::deserialize("member", stack)));
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
struct StackStatusDeserializer;
impl StackStatusDeserializer {
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

/// Serialize `StackStatusFilter` contents to a `SignedRequest`.
struct StackStatusFilterSerializer;
impl StackStatusFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct StackStatusReasonDeserializer;
impl StackStatusReasonDeserializer {
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
struct StackSummariesDeserializer;
impl StackSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<StackSummary>, XmlParseError> {

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
                        obj.push(try!(StackSummaryDeserializer::deserialize("member", stack)));
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
#[doc="<p>The StackSummary Data Type</p>"]
#[derive(Default,Debug,Clone)]
pub struct StackSummary {
    #[doc="<p>The time the stack was created.</p>"]
    pub creation_time: String,
    #[doc="<p>The time the stack was deleted.</p>"]
    pub deletion_time: Option<String>,
    #[doc="<p>The time the stack was last updated. This field will only be returned if the stack has been updated at least once.</p>"]
    pub last_updated_time: Option<String>,
    #[doc="<p>Unique stack identifier.</p>"]
    pub stack_id: Option<String>,
    #[doc="<p>The name associated with the stack.</p>"]
    pub stack_name: String,
    #[doc="<p>The current status of the stack.</p>"]
    pub stack_status: String,
    #[doc="<p>Success/Failure message associated with the stack status.</p>"]
    pub stack_status_reason: Option<String>,
    #[doc="<p>The template description of the template used to create the stack.</p>"]
    pub template_description: Option<String>,
}

struct StackSummaryDeserializer;
impl StackSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<StackSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSummary::default();

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
                        "CreationTime" => {
                            obj.creation_time = try!(CreationTimeDeserializer::deserialize("CreationTime",
                                                                                           stack));
                        }
                        "DeletionTime" => {
                            obj.deletion_time =
                                Some(try!(DeletionTimeDeserializer::deserialize("DeletionTime",
                                                                                stack)));
                        }
                        "LastUpdatedTime" => {
                            obj.last_updated_time =
                                Some(try!(LastUpdatedTimeDeserializer::deserialize("LastUpdatedTime",
                                                                                   stack)));
                        }
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
                                                                                      stack)));
                        }
                        "StackName" => {
                            obj.stack_name = try!(StackNameDeserializer::deserialize("StackName",
                                                                                     stack));
                        }
                        "StackStatus" => {
                            obj.stack_status = try!(StackStatusDeserializer::deserialize("StackStatus",
                                                                                         stack));
                        }
                        "StackStatusReason" => {
                            obj.stack_status_reason =
                                Some(try!(StackStatusReasonDeserializer::deserialize("StackStatusReason",
                                                                                     stack)));
                        }
                        "TemplateDescription" => {
                            obj.template_description =
                                Some(try!(TemplateDescriptionDeserializer::deserialize("TemplateDescription",
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
struct StacksDeserializer;
impl StacksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<Stack>, XmlParseError> {

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
                        obj.push(try!(StackDeserializer::deserialize("member", stack)));
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
struct StageListDeserializer;
impl StageListDeserializer {
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
                    if name == "member" {
                        obj.push(try!(TemplateStageDeserializer::deserialize("member", stack)));
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
#[doc="<p>The Tag type enables you to specify a key-value pair that can be used to store information about an AWS CloudFormation stack.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Tag {
    #[doc="<p> <i>Required</i>. A string used to identify this tag. You can specify a maximum of 128 characters for a tag key. Tags owned by Amazon Web Services (AWS) have the reserved prefix: <code>aws:</code>.</p>"]
    pub key: Option<String>,
    #[doc="<p> <i>Required</i>. A string containing the value for this tag. You can specify a maximum of 256 characters for a tag value.</p>"]
    pub value: Option<String>,
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
                            obj.key = Some(try!(TagKeyDeserializer::deserialize("Key", stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(TagValueDeserializer::deserialize("Value",
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

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.key {
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }

    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
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
struct TagValueDeserializer;
impl TagValueDeserializer {
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
struct TagsDeserializer;
impl TagsDeserializer {
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

struct TemplateBodyDeserializer;
impl TemplateBodyDeserializer {
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
struct TemplateDescriptionDeserializer;
impl TemplateDescriptionDeserializer {
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
#[doc="<p>The TemplateParameter data type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct TemplateParameter {
    #[doc="<p>The default value associated with the parameter.</p>"]
    pub default_value: Option<String>,
    #[doc="<p>User defined description associated with the parameter.</p>"]
    pub description: Option<String>,
    #[doc="<p>Flag indicating whether the parameter should be displayed as plain text in logs and UIs.</p>"]
    pub no_echo: Option<bool>,
    #[doc="<p>The name associated with the parameter.</p>"]
    pub parameter_key: Option<String>,
}

struct TemplateParameterDeserializer;
impl TemplateParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TemplateParameter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TemplateParameter::default();

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
                        "DefaultValue" => {
                            obj.default_value =
                                Some(try!(ParameterValueDeserializer::deserialize("DefaultValue",
                                                                                  stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "NoEcho" => {
                            obj.no_echo = Some(try!(NoEchoDeserializer::deserialize("NoEcho",
                                                                                    stack)));
                        }
                        "ParameterKey" => {
                            obj.parameter_key =
                                Some(try!(ParameterKeyDeserializer::deserialize("ParameterKey",
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
struct TemplateParametersDeserializer;
impl TemplateParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Vec<TemplateParameter>, XmlParseError> {

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
                        obj.push(try!(TemplateParameterDeserializer::deserialize("member", stack)));
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
struct TemplateStageDeserializer;
impl TemplateStageDeserializer {
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
struct TimeoutMinutesDeserializer;
impl TimeoutMinutesDeserializer {
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
struct TimestampDeserializer;
impl TimestampDeserializer {
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
struct TransformNameDeserializer;
impl TransformNameDeserializer {
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
struct TransformsListDeserializer;
impl TransformsListDeserializer {
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
                    if name == "member" {
                        obj.push(try!(TransformNameDeserializer::deserialize("member", stack)));
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
#[doc="<p>The input for an <a>UpdateStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct UpdateStackInput {
    #[doc="<p>A list of values that you must specify before AWS CloudFormation can update certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html\"> AWS::IAM::AccessKey</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html\"> AWS::IAM::Group</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html\"> AWS::IAM::InstanceProfile</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html\"> AWS::IAM::Policy</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html\"> AWS::IAM::Role</a>, <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html\"> AWS::IAM::User</a>, and <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html\"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities\">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>A unique identifier for this <code>UpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to update a stack with the same name. You might retry <code>UpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>"]
    pub client_request_token: Option<String>,
    #[doc="<p>Amazon Simple Notification Service topic Amazon Resource Names (ARNs) that AWS CloudFormation associates with the stack. Specify an empty list to remove all notification topics.</p>"]
    pub notification_ar_ns: Option<Vec<String>>,
    #[doc="<p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html\">Parameter</a> data type.</p>"]
    pub parameters: Option<Vec<Parameter>>,
    #[doc="<p>The template resource types that you have permissions to work with for this update stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html\">Controlling Access with AWS Identity and Access Management</a>.</p>"]
    pub resource_types: Option<Vec<String>>,
    #[doc="<p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to update the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>"]
    pub role_arn: Option<String>,
    #[doc="<p>The name or unique stack ID of the stack to update.</p>"]
    pub stack_name: String,
    #[doc="<p>Structure containing a new stack policy body. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p> <p>You might update the stack policy, for example, in order to protect a new resource that you created during a stack update. If you do not specify a stack policy, the current policy that is associated with the stack is unchanged.</p>"]
    pub stack_policy_body: Option<String>,
    #[doc="<p>Structure containing the temporary overriding stack policy body. You can specify either the <code>StackPolicyDuringUpdateBody</code> or the <code>StackPolicyDuringUpdateURL</code> parameter, but not both.</p> <p>If you want to update protected resources, specify a temporary overriding stack policy during this update. If you do not specify a stack policy, the current policy that is associated with the stack will be used.</p>"]
    pub stack_policy_during_update_body: Option<String>,
    #[doc="<p>Location of a file containing the temporary overriding stack policy. The URL must point to a policy (max size: 16KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyDuringUpdateBody</code> or the <code>StackPolicyDuringUpdateURL</code> parameter, but not both.</p> <p>If you want to update protected resources, specify a temporary overriding stack policy during this update. If you do not specify a stack policy, the current policy that is associated with the stack will be used.</p>"]
    pub stack_policy_during_update_url: Option<String>,
    #[doc="<p>Location of a file containing the updated stack policy. The URL must point to a policy (max size: 16KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p> <p>You might update the stack policy, for example, in order to protect a new resource that you created during a stack update. If you do not specify a stack policy, the current policy that is associated with the stack is unchanged.</p>"]
    pub stack_policy_url: Option<String>,
    #[doc="<p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to supported resources in the stack. You can specify a maximum number of 10 tags.</p> <p>If you don't specify this parameter, AWS CloudFormation doesn't modify the stack's tags. If you specify an empty value, AWS CloudFormation removes all associated tags.</p>"]
    pub tags: Option<Vec<Tag>>,
    #[doc="<p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>"]
    pub template_url: Option<String>,
    #[doc="<p>Reuse the existing template that is associated with the stack that you are updating.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>"]
    pub use_previous_template: Option<bool>,
}


/// Serialize `UpdateStackInput` contents to a `SignedRequest`.
struct UpdateStackInputSerializer;
impl UpdateStackInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.capabilities {
            CapabilitiesSerializer::serialize(params,
                                              &format!("{}{}", prefix, "Capabilities"),
                                              field_value);
        }
        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "NotificationARNs"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(params,
                                            &format!("{}{}", prefix, "Parameters"),
                                            field_value);
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(params,
                                               &format!("{}{}", prefix, "ResourceTypes"),
                                               field_value);
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(&format!("{}{}", prefix, "StackPolicyBody"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_body {
            params.put(&format!("{}{}", prefix, "StackPolicyDuringUpdateBody"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_url {
            params.put(&format!("{}{}", prefix, "StackPolicyDuringUpdateURL"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(&format!("{}{}", prefix, "StackPolicyURL"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
        if let Some(ref field_value) = obj.use_previous_template {
            params.put(&format!("{}{}", prefix, "UsePreviousTemplate"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>The output for an <a>UpdateStack</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct UpdateStackOutput {
    #[doc="<p>Unique identifier of the stack.</p>"]
    pub stack_id: Option<String>,
}

struct UpdateStackOutputDeserializer;
impl UpdateStackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<UpdateStackOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateStackOutput::default();

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
                        "StackId" => {
                            obj.stack_id = Some(try!(StackIdDeserializer::deserialize("StackId",
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
struct UrlDeserializer;
impl UrlDeserializer {
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
struct UsePreviousValueDeserializer;
impl UsePreviousValueDeserializer {
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
#[doc="<p>The input for <a>ValidateTemplate</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ValidateTemplateInput {
    #[doc="<p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>"]
    pub template_body: Option<String>,
    #[doc="<p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html\">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>"]
    pub template_url: Option<String>,
}


/// Serialize `ValidateTemplateInput` contents to a `SignedRequest`.
struct ValidateTemplateInputSerializer;
impl ValidateTemplateInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ValidateTemplateInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }

    }
}

#[doc="<p>The output for <a>ValidateTemplate</a> action.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ValidateTemplateOutput {
    #[doc="<p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities\">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>"]
    pub capabilities: Option<Vec<String>>,
    #[doc="<p>The list of resources that generated the values in the <code>Capabilities</code> response element.</p>"]
    pub capabilities_reason: Option<String>,
    #[doc="<p>A list of the transforms that are declared in the template.</p>"]
    pub declared_transforms: Option<Vec<String>>,
    #[doc="<p>The description found within the template.</p>"]
    pub description: Option<String>,
    #[doc="<p>A list of <code>TemplateParameter</code> structures.</p>"]
    pub parameters: Option<Vec<TemplateParameter>>,
}

struct ValidateTemplateOutputDeserializer;
impl ValidateTemplateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ValidateTemplateOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ValidateTemplateOutput::default();

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
                        "Capabilities" => {
                            obj.capabilities =
                                Some(try!(CapabilitiesDeserializer::deserialize("Capabilities",
                                                                                stack)));
                        }
                        "CapabilitiesReason" => {
                            obj.capabilities_reason =
                                Some(try!(CapabilitiesReasonDeserializer::deserialize("CapabilitiesReason",
                                                                                      stack)));
                        }
                        "DeclaredTransforms" => {
                            obj.declared_transforms =
                                Some(try!(TransformsListDeserializer::deserialize("DeclaredTransforms",
                                                                                  stack)));
                        }
                        "Description" => {
                            obj.description =
                                Some(try!(DescriptionDeserializer::deserialize("Description",
                                                                               stack)));
                        }
                        "Parameters" => {
                            obj.parameters =
                                Some(try!(TemplateParametersDeserializer::deserialize("Parameters",
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
struct VersionDeserializer;
impl VersionDeserializer {
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
/// Errors returned by CancelUpdateStack
#[derive(Debug, PartialEq)]
pub enum CancelUpdateStackError {
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelUpdateStackError {
    pub fn from_body(body: &str) -> CancelUpdateStackError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => CancelUpdateStackError::TokenAlreadyExists(String::from(parsed_error.message)),
                    _ => CancelUpdateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelUpdateStackError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CancelUpdateStackError {
    fn from(err: XmlParseError) -> CancelUpdateStackError {
        let XmlParseError(message) = err;
        CancelUpdateStackError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CancelUpdateStackError {
    fn from(err: CredentialsError) -> CancelUpdateStackError {
        CancelUpdateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelUpdateStackError {
    fn from(err: HttpDispatchError) -> CancelUpdateStackError {
        CancelUpdateStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelUpdateStackError {
    fn from(err: io::Error) -> CancelUpdateStackError {
        CancelUpdateStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelUpdateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelUpdateStackError {
    fn description(&self) -> &str {
        match *self {
            CancelUpdateStackError::TokenAlreadyExists(ref cause) => cause,
            CancelUpdateStackError::Validation(ref cause) => cause,
            CancelUpdateStackError::Credentials(ref err) => err.description(),
            CancelUpdateStackError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelUpdateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ContinueUpdateRollback
#[derive(Debug, PartialEq)]
pub enum ContinueUpdateRollbackError {
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ContinueUpdateRollbackError {
    pub fn from_body(body: &str) -> ContinueUpdateRollbackError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => ContinueUpdateRollbackError::TokenAlreadyExists(String::from(parsed_error.message)),
                    _ => ContinueUpdateRollbackError::Unknown(String::from(body)),
                }
            }
            Err(_) => ContinueUpdateRollbackError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ContinueUpdateRollbackError {
    fn from(err: XmlParseError) -> ContinueUpdateRollbackError {
        let XmlParseError(message) = err;
        ContinueUpdateRollbackError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ContinueUpdateRollbackError {
    fn from(err: CredentialsError) -> ContinueUpdateRollbackError {
        ContinueUpdateRollbackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ContinueUpdateRollbackError {
    fn from(err: HttpDispatchError) -> ContinueUpdateRollbackError {
        ContinueUpdateRollbackError::HttpDispatch(err)
    }
}
impl From<io::Error> for ContinueUpdateRollbackError {
    fn from(err: io::Error) -> ContinueUpdateRollbackError {
        ContinueUpdateRollbackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ContinueUpdateRollbackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ContinueUpdateRollbackError {
    fn description(&self) -> &str {
        match *self {
            ContinueUpdateRollbackError::TokenAlreadyExists(ref cause) => cause,
            ContinueUpdateRollbackError::Validation(ref cause) => cause,
            ContinueUpdateRollbackError::Credentials(ref err) => err.description(),
            ContinueUpdateRollbackError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ContinueUpdateRollbackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateChangeSet
#[derive(Debug, PartialEq)]
pub enum CreateChangeSetError {
    ///<p>Resource with the name requested already exists.</p>
    AlreadyExists(String),
    ///<p>The template contains resources with capabilities that were not specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    ///<p>Quota for the resource has already been reached.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateChangeSetError {
    pub fn from_body(body: &str) -> CreateChangeSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "AlreadyExistsException" => {
                        CreateChangeSetError::AlreadyExists(String::from(parsed_error.message))
                    }
                    "InsufficientCapabilitiesException" => CreateChangeSetError::InsufficientCapabilities(String::from(parsed_error.message)),
                    "LimitExceededException" => {
                        CreateChangeSetError::LimitExceeded(String::from(parsed_error.message))
                    }
                    _ => CreateChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateChangeSetError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateChangeSetError {
    fn from(err: XmlParseError) -> CreateChangeSetError {
        let XmlParseError(message) = err;
        CreateChangeSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateChangeSetError {
    fn from(err: CredentialsError) -> CreateChangeSetError {
        CreateChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateChangeSetError {
    fn from(err: HttpDispatchError) -> CreateChangeSetError {
        CreateChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateChangeSetError {
    fn from(err: io::Error) -> CreateChangeSetError {
        CreateChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateChangeSetError {
    fn description(&self) -> &str {
        match *self {
            CreateChangeSetError::AlreadyExists(ref cause) => cause,
            CreateChangeSetError::InsufficientCapabilities(ref cause) => cause,
            CreateChangeSetError::LimitExceeded(ref cause) => cause,
            CreateChangeSetError::Validation(ref cause) => cause,
            CreateChangeSetError::Credentials(ref err) => err.description(),
            CreateChangeSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStack
#[derive(Debug, PartialEq)]
pub enum CreateStackError {
    ///<p>Resource with the name requested already exists.</p>
    AlreadyExists(String),
    ///<p>The template contains resources with capabilities that were not specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    ///<p>Quota for the resource has already been reached.</p>
    LimitExceeded(String),
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateStackError {
    pub fn from_body(body: &str) -> CreateStackError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "AlreadyExistsException" => {
                        CreateStackError::AlreadyExists(String::from(parsed_error.message))
                    }
                    "InsufficientCapabilitiesException" => CreateStackError::InsufficientCapabilities(String::from(parsed_error.message)),
                    "LimitExceededException" => {
                        CreateStackError::LimitExceeded(String::from(parsed_error.message))
                    }
                    "TokenAlreadyExistsException" => {
                        CreateStackError::TokenAlreadyExists(String::from(parsed_error.message))
                    }
                    _ => CreateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStackError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateStackError {
    fn from(err: XmlParseError) -> CreateStackError {
        let XmlParseError(message) = err;
        CreateStackError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStackError {
    fn from(err: CredentialsError) -> CreateStackError {
        CreateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStackError {
    fn from(err: HttpDispatchError) -> CreateStackError {
        CreateStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStackError {
    fn from(err: io::Error) -> CreateStackError {
        CreateStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackError {
    fn description(&self) -> &str {
        match *self {
            CreateStackError::AlreadyExists(ref cause) => cause,
            CreateStackError::InsufficientCapabilities(ref cause) => cause,
            CreateStackError::LimitExceeded(ref cause) => cause,
            CreateStackError::TokenAlreadyExists(ref cause) => cause,
            CreateStackError::Validation(ref cause) => cause,
            CreateStackError::Credentials(ref err) => err.description(),
            CreateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteChangeSet
#[derive(Debug, PartialEq)]
pub enum DeleteChangeSetError {
    ///<p>The specified change set cannot be used to update the stack. For example, the change set status might be <code>CREATE_IN_PROGRESS</code> or the stack status might be <code>UPDATE_IN_PROGRESS</code>.</p>
    InvalidChangeSetStatus(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteChangeSetError {
    pub fn from_body(body: &str) -> DeleteChangeSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidChangeSetStatusException" => DeleteChangeSetError::InvalidChangeSetStatus(String::from(parsed_error.message)),
                    _ => DeleteChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteChangeSetError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteChangeSetError {
    fn from(err: XmlParseError) -> DeleteChangeSetError {
        let XmlParseError(message) = err;
        DeleteChangeSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteChangeSetError {
    fn from(err: CredentialsError) -> DeleteChangeSetError {
        DeleteChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteChangeSetError {
    fn from(err: HttpDispatchError) -> DeleteChangeSetError {
        DeleteChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteChangeSetError {
    fn from(err: io::Error) -> DeleteChangeSetError {
        DeleteChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteChangeSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteChangeSetError::InvalidChangeSetStatus(ref cause) => cause,
            DeleteChangeSetError::Validation(ref cause) => cause,
            DeleteChangeSetError::Credentials(ref err) => err.description(),
            DeleteChangeSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteStackError {
    pub fn from_body(body: &str) -> DeleteStackError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => {
                        DeleteStackError::TokenAlreadyExists(String::from(parsed_error.message))
                    }
                    _ => DeleteStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStackError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteStackError {
    fn from(err: XmlParseError) -> DeleteStackError {
        let XmlParseError(message) = err;
        DeleteStackError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteStackError {
    fn from(err: CredentialsError) -> DeleteStackError {
        DeleteStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStackError {
    fn from(err: HttpDispatchError) -> DeleteStackError {
        DeleteStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStackError {
    fn from(err: io::Error) -> DeleteStackError {
        DeleteStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStackError {
    fn description(&self) -> &str {
        match *self {
            DeleteStackError::TokenAlreadyExists(ref cause) => cause,
            DeleteStackError::Validation(ref cause) => cause,
            DeleteStackError::Credentials(ref err) => err.description(),
            DeleteStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAccountLimitsError {
    pub fn from_body(body: &str) -> DescribeAccountLimitsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeAccountLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAccountLimitsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeAccountLimitsError {
    fn from(err: XmlParseError) -> DescribeAccountLimitsError {
        let XmlParseError(message) = err;
        DescribeAccountLimitsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeAccountLimitsError {
    fn from(err: CredentialsError) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountLimitsError {
    fn from(err: HttpDispatchError) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAccountLimitsError {
    fn from(err: io::Error) -> DescribeAccountLimitsError {
        DescribeAccountLimitsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAccountLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountLimitsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountLimitsError::Validation(ref cause) => cause,
            DescribeAccountLimitsError::Credentials(ref err) => err.description(),
            DescribeAccountLimitsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeChangeSet
#[derive(Debug, PartialEq)]
pub enum DescribeChangeSetError {
    ///<p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeChangeSetError {
    pub fn from_body(body: &str) -> DescribeChangeSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ChangeSetNotFoundException" => DescribeChangeSetError::ChangeSetNotFound(String::from(parsed_error.message)),
                    _ => DescribeChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeChangeSetError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeChangeSetError {
    fn from(err: XmlParseError) -> DescribeChangeSetError {
        let XmlParseError(message) = err;
        DescribeChangeSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeChangeSetError {
    fn from(err: CredentialsError) -> DescribeChangeSetError {
        DescribeChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeChangeSetError {
    fn from(err: HttpDispatchError) -> DescribeChangeSetError {
        DescribeChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeChangeSetError {
    fn from(err: io::Error) -> DescribeChangeSetError {
        DescribeChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeChangeSetError {
    fn description(&self) -> &str {
        match *self {
            DescribeChangeSetError::ChangeSetNotFound(ref cause) => cause,
            DescribeChangeSetError::Validation(ref cause) => cause,
            DescribeChangeSetError::Credentials(ref err) => err.description(),
            DescribeChangeSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackEvents
#[derive(Debug, PartialEq)]
pub enum DescribeStackEventsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStackEventsError {
    pub fn from_body(body: &str) -> DescribeStackEventsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeStackEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStackEventsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeStackEventsError {
    fn from(err: XmlParseError) -> DescribeStackEventsError {
        let XmlParseError(message) = err;
        DescribeStackEventsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackEventsError {
    fn from(err: CredentialsError) -> DescribeStackEventsError {
        DescribeStackEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackEventsError {
    fn from(err: HttpDispatchError) -> DescribeStackEventsError {
        DescribeStackEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackEventsError {
    fn from(err: io::Error) -> DescribeStackEventsError {
        DescribeStackEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackEventsError::Validation(ref cause) => cause,
            DescribeStackEventsError::Credentials(ref err) => err.description(),
            DescribeStackEventsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackResource
#[derive(Debug, PartialEq)]
pub enum DescribeStackResourceError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStackResourceError {
    pub fn from_body(body: &str) -> DescribeStackResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeStackResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStackResourceError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeStackResourceError {
    fn from(err: XmlParseError) -> DescribeStackResourceError {
        let XmlParseError(message) = err;
        DescribeStackResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackResourceError {
    fn from(err: CredentialsError) -> DescribeStackResourceError {
        DescribeStackResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackResourceError {
    fn from(err: HttpDispatchError) -> DescribeStackResourceError {
        DescribeStackResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackResourceError {
    fn from(err: io::Error) -> DescribeStackResourceError {
        DescribeStackResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackResourceError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackResourceError::Validation(ref cause) => cause,
            DescribeStackResourceError::Credentials(ref err) => err.description(),
            DescribeStackResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackResources
#[derive(Debug, PartialEq)]
pub enum DescribeStackResourcesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStackResourcesError {
    pub fn from_body(body: &str) -> DescribeStackResourcesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeStackResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStackResourcesError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeStackResourcesError {
    fn from(err: XmlParseError) -> DescribeStackResourcesError {
        let XmlParseError(message) = err;
        DescribeStackResourcesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackResourcesError {
    fn from(err: CredentialsError) -> DescribeStackResourcesError {
        DescribeStackResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackResourcesError {
    fn from(err: HttpDispatchError) -> DescribeStackResourcesError {
        DescribeStackResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackResourcesError {
    fn from(err: io::Error) -> DescribeStackResourcesError {
        DescribeStackResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackResourcesError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackResourcesError::Validation(ref cause) => cause,
            DescribeStackResourcesError::Credentials(ref err) => err.description(),
            DescribeStackResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeStacksError {
    pub fn from_body(body: &str) -> DescribeStacksError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeStacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStacksError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeStacksError {
    fn from(err: XmlParseError) -> DescribeStacksError {
        let XmlParseError(message) = err;
        DescribeStacksError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStacksError {
    fn from(err: CredentialsError) -> DescribeStacksError {
        DescribeStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStacksError {
    fn from(err: HttpDispatchError) -> DescribeStacksError {
        DescribeStacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStacksError {
    fn from(err: io::Error) -> DescribeStacksError {
        DescribeStacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStacksError {
    fn description(&self) -> &str {
        match *self {
            DescribeStacksError::Validation(ref cause) => cause,
            DescribeStacksError::Credentials(ref err) => err.description(),
            DescribeStacksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EstimateTemplateCost
#[derive(Debug, PartialEq)]
pub enum EstimateTemplateCostError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EstimateTemplateCostError {
    pub fn from_body(body: &str) -> EstimateTemplateCostError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => EstimateTemplateCostError::Unknown(String::from(body)),
                }
            }
            Err(_) => EstimateTemplateCostError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for EstimateTemplateCostError {
    fn from(err: XmlParseError) -> EstimateTemplateCostError {
        let XmlParseError(message) = err;
        EstimateTemplateCostError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for EstimateTemplateCostError {
    fn from(err: CredentialsError) -> EstimateTemplateCostError {
        EstimateTemplateCostError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EstimateTemplateCostError {
    fn from(err: HttpDispatchError) -> EstimateTemplateCostError {
        EstimateTemplateCostError::HttpDispatch(err)
    }
}
impl From<io::Error> for EstimateTemplateCostError {
    fn from(err: io::Error) -> EstimateTemplateCostError {
        EstimateTemplateCostError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EstimateTemplateCostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EstimateTemplateCostError {
    fn description(&self) -> &str {
        match *self {
            EstimateTemplateCostError::Validation(ref cause) => cause,
            EstimateTemplateCostError::Credentials(ref err) => err.description(),
            EstimateTemplateCostError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EstimateTemplateCostError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecuteChangeSet
#[derive(Debug, PartialEq)]
pub enum ExecuteChangeSetError {
    ///<p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
    ///<p>The template contains resources with capabilities that were not specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    ///<p>The specified change set cannot be used to update the stack. For example, the change set status might be <code>CREATE_IN_PROGRESS</code> or the stack status might be <code>UPDATE_IN_PROGRESS</code>.</p>
    InvalidChangeSetStatus(String),
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ExecuteChangeSetError {
    pub fn from_body(body: &str) -> ExecuteChangeSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ChangeSetNotFoundException" => {
                        ExecuteChangeSetError::ChangeSetNotFound(String::from(parsed_error.message))
                    }
                    "InsufficientCapabilitiesException" => ExecuteChangeSetError::InsufficientCapabilities(String::from(parsed_error.message)),
                    "InvalidChangeSetStatusException" => ExecuteChangeSetError::InvalidChangeSetStatus(String::from(parsed_error.message)),
                    "TokenAlreadyExistsException" => ExecuteChangeSetError::TokenAlreadyExists(String::from(parsed_error.message)),
                    _ => ExecuteChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ExecuteChangeSetError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ExecuteChangeSetError {
    fn from(err: XmlParseError) -> ExecuteChangeSetError {
        let XmlParseError(message) = err;
        ExecuteChangeSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ExecuteChangeSetError {
    fn from(err: CredentialsError) -> ExecuteChangeSetError {
        ExecuteChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExecuteChangeSetError {
    fn from(err: HttpDispatchError) -> ExecuteChangeSetError {
        ExecuteChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExecuteChangeSetError {
    fn from(err: io::Error) -> ExecuteChangeSetError {
        ExecuteChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExecuteChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteChangeSetError {
    fn description(&self) -> &str {
        match *self {
            ExecuteChangeSetError::ChangeSetNotFound(ref cause) => cause,
            ExecuteChangeSetError::InsufficientCapabilities(ref cause) => cause,
            ExecuteChangeSetError::InvalidChangeSetStatus(ref cause) => cause,
            ExecuteChangeSetError::TokenAlreadyExists(ref cause) => cause,
            ExecuteChangeSetError::Validation(ref cause) => cause,
            ExecuteChangeSetError::Credentials(ref err) => err.description(),
            ExecuteChangeSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ExecuteChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStackPolicy
#[derive(Debug, PartialEq)]
pub enum GetStackPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetStackPolicyError {
    pub fn from_body(body: &str) -> GetStackPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetStackPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetStackPolicyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetStackPolicyError {
    fn from(err: XmlParseError) -> GetStackPolicyError {
        let XmlParseError(message) = err;
        GetStackPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetStackPolicyError {
    fn from(err: CredentialsError) -> GetStackPolicyError {
        GetStackPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetStackPolicyError {
    fn from(err: HttpDispatchError) -> GetStackPolicyError {
        GetStackPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetStackPolicyError {
    fn from(err: io::Error) -> GetStackPolicyError {
        GetStackPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetStackPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStackPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetStackPolicyError::Validation(ref cause) => cause,
            GetStackPolicyError::Credentials(ref err) => err.description(),
            GetStackPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetStackPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTemplate
#[derive(Debug, PartialEq)]
pub enum GetTemplateError {
    ///<p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTemplateError {
    pub fn from_body(body: &str) -> GetTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "ChangeSetNotFoundException" => {
                        GetTemplateError::ChangeSetNotFound(String::from(parsed_error.message))
                    }
                    _ => GetTemplateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTemplateError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetTemplateError {
    fn from(err: XmlParseError) -> GetTemplateError {
        let XmlParseError(message) = err;
        GetTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTemplateError {
    fn from(err: CredentialsError) -> GetTemplateError {
        GetTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTemplateError {
    fn from(err: HttpDispatchError) -> GetTemplateError {
        GetTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTemplateError {
    fn from(err: io::Error) -> GetTemplateError {
        GetTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetTemplateError::ChangeSetNotFound(ref cause) => cause,
            GetTemplateError::Validation(ref cause) => cause,
            GetTemplateError::Credentials(ref err) => err.description(),
            GetTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTemplateSummary
#[derive(Debug, PartialEq)]
pub enum GetTemplateSummaryError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTemplateSummaryError {
    pub fn from_body(body: &str) -> GetTemplateSummaryError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => GetTemplateSummaryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTemplateSummaryError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for GetTemplateSummaryError {
    fn from(err: XmlParseError) -> GetTemplateSummaryError {
        let XmlParseError(message) = err;
        GetTemplateSummaryError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for GetTemplateSummaryError {
    fn from(err: CredentialsError) -> GetTemplateSummaryError {
        GetTemplateSummaryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTemplateSummaryError {
    fn from(err: HttpDispatchError) -> GetTemplateSummaryError {
        GetTemplateSummaryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTemplateSummaryError {
    fn from(err: io::Error) -> GetTemplateSummaryError {
        GetTemplateSummaryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTemplateSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTemplateSummaryError {
    fn description(&self) -> &str {
        match *self {
            GetTemplateSummaryError::Validation(ref cause) => cause,
            GetTemplateSummaryError::Credentials(ref err) => err.description(),
            GetTemplateSummaryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTemplateSummaryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListChangeSets
#[derive(Debug, PartialEq)]
pub enum ListChangeSetsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListChangeSetsError {
    pub fn from_body(body: &str) -> ListChangeSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListChangeSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListChangeSetsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListChangeSetsError {
    fn from(err: XmlParseError) -> ListChangeSetsError {
        let XmlParseError(message) = err;
        ListChangeSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListChangeSetsError {
    fn from(err: CredentialsError) -> ListChangeSetsError {
        ListChangeSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListChangeSetsError {
    fn from(err: HttpDispatchError) -> ListChangeSetsError {
        ListChangeSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListChangeSetsError {
    fn from(err: io::Error) -> ListChangeSetsError {
        ListChangeSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListChangeSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChangeSetsError {
    fn description(&self) -> &str {
        match *self {
            ListChangeSetsError::Validation(ref cause) => cause,
            ListChangeSetsError::Credentials(ref err) => err.description(),
            ListChangeSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListChangeSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListExports
#[derive(Debug, PartialEq)]
pub enum ListExportsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListExportsError {
    pub fn from_body(body: &str) -> ListExportsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListExportsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListExportsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListExportsError {
    fn from(err: XmlParseError) -> ListExportsError {
        let XmlParseError(message) = err;
        ListExportsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListExportsError {
    fn from(err: CredentialsError) -> ListExportsError {
        ListExportsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListExportsError {
    fn from(err: HttpDispatchError) -> ListExportsError {
        ListExportsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListExportsError {
    fn from(err: io::Error) -> ListExportsError {
        ListExportsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListExportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListExportsError {
    fn description(&self) -> &str {
        match *self {
            ListExportsError::Validation(ref cause) => cause,
            ListExportsError::Credentials(ref err) => err.description(),
            ListExportsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListExportsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListImports
#[derive(Debug, PartialEq)]
pub enum ListImportsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListImportsError {
    pub fn from_body(body: &str) -> ListImportsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListImportsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListImportsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListImportsError {
    fn from(err: XmlParseError) -> ListImportsError {
        let XmlParseError(message) = err;
        ListImportsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListImportsError {
    fn from(err: CredentialsError) -> ListImportsError {
        ListImportsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListImportsError {
    fn from(err: HttpDispatchError) -> ListImportsError {
        ListImportsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListImportsError {
    fn from(err: io::Error) -> ListImportsError {
        ListImportsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListImportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListImportsError {
    fn description(&self) -> &str {
        match *self {
            ListImportsError::Validation(ref cause) => cause,
            ListImportsError::Credentials(ref err) => err.description(),
            ListImportsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListImportsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStackResources
#[derive(Debug, PartialEq)]
pub enum ListStackResourcesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListStackResourcesError {
    pub fn from_body(body: &str) -> ListStackResourcesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListStackResourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStackResourcesError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListStackResourcesError {
    fn from(err: XmlParseError) -> ListStackResourcesError {
        let XmlParseError(message) = err;
        ListStackResourcesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStackResourcesError {
    fn from(err: CredentialsError) -> ListStackResourcesError {
        ListStackResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStackResourcesError {
    fn from(err: HttpDispatchError) -> ListStackResourcesError {
        ListStackResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStackResourcesError {
    fn from(err: io::Error) -> ListStackResourcesError {
        ListStackResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStackResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListStackResourcesError::Validation(ref cause) => cause,
            ListStackResourcesError::Credentials(ref err) => err.description(),
            ListStackResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStackResourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStacks
#[derive(Debug, PartialEq)]
pub enum ListStacksError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListStacksError {
    pub fn from_body(body: &str) -> ListStacksError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ListStacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStacksError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListStacksError {
    fn from(err: XmlParseError) -> ListStacksError {
        let XmlParseError(message) = err;
        ListStacksError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStacksError {
    fn from(err: CredentialsError) -> ListStacksError {
        ListStacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStacksError {
    fn from(err: HttpDispatchError) -> ListStacksError {
        ListStacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStacksError {
    fn from(err: io::Error) -> ListStacksError {
        ListStacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStacksError {
    fn description(&self) -> &str {
        match *self {
            ListStacksError::Validation(ref cause) => cause,
            ListStacksError::Credentials(ref err) => err.description(),
            ListStacksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetStackPolicy
#[derive(Debug, PartialEq)]
pub enum SetStackPolicyError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetStackPolicyError {
    pub fn from_body(body: &str) -> SetStackPolicyError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => SetStackPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetStackPolicyError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for SetStackPolicyError {
    fn from(err: XmlParseError) -> SetStackPolicyError {
        let XmlParseError(message) = err;
        SetStackPolicyError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SetStackPolicyError {
    fn from(err: CredentialsError) -> SetStackPolicyError {
        SetStackPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetStackPolicyError {
    fn from(err: HttpDispatchError) -> SetStackPolicyError {
        SetStackPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetStackPolicyError {
    fn from(err: io::Error) -> SetStackPolicyError {
        SetStackPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetStackPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetStackPolicyError {
    fn description(&self) -> &str {
        match *self {
            SetStackPolicyError::Validation(ref cause) => cause,
            SetStackPolicyError::Credentials(ref err) => err.description(),
            SetStackPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetStackPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SignalResource
#[derive(Debug, PartialEq)]
pub enum SignalResourceError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SignalResourceError {
    pub fn from_body(body: &str) -> SignalResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => SignalResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => SignalResourceError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for SignalResourceError {
    fn from(err: XmlParseError) -> SignalResourceError {
        let XmlParseError(message) = err;
        SignalResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for SignalResourceError {
    fn from(err: CredentialsError) -> SignalResourceError {
        SignalResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SignalResourceError {
    fn from(err: HttpDispatchError) -> SignalResourceError {
        SignalResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for SignalResourceError {
    fn from(err: io::Error) -> SignalResourceError {
        SignalResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SignalResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SignalResourceError {
    fn description(&self) -> &str {
        match *self {
            SignalResourceError::Validation(ref cause) => cause,
            SignalResourceError::Credentials(ref err) => err.description(),
            SignalResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SignalResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
    ///<p>The template contains resources with capabilities that were not specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    ///<p>A client request token already exists.</p>
    TokenAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateStackError {
    pub fn from_body(body: &str) -> UpdateStackError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InsufficientCapabilitiesException" => UpdateStackError::InsufficientCapabilities(String::from(parsed_error.message)),
                    "TokenAlreadyExistsException" => {
                        UpdateStackError::TokenAlreadyExists(String::from(parsed_error.message))
                    }
                    _ => UpdateStackError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateStackError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for UpdateStackError {
    fn from(err: XmlParseError) -> UpdateStackError {
        let XmlParseError(message) = err;
        UpdateStackError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateStackError {
    fn from(err: CredentialsError) -> UpdateStackError {
        UpdateStackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStackError {
    fn from(err: HttpDispatchError) -> UpdateStackError {
        UpdateStackError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStackError {
    fn from(err: io::Error) -> UpdateStackError {
        UpdateStackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackError::InsufficientCapabilities(ref cause) => cause,
            UpdateStackError::TokenAlreadyExists(ref cause) => cause,
            UpdateStackError::Validation(ref cause) => cause,
            UpdateStackError::Credentials(ref err) => err.description(),
            UpdateStackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ValidateTemplate
#[derive(Debug, PartialEq)]
pub enum ValidateTemplateError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ValidateTemplateError {
    pub fn from_body(body: &str) -> ValidateTemplateError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => ValidateTemplateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ValidateTemplateError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ValidateTemplateError {
    fn from(err: XmlParseError) -> ValidateTemplateError {
        let XmlParseError(message) = err;
        ValidateTemplateError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ValidateTemplateError {
    fn from(err: CredentialsError) -> ValidateTemplateError {
        ValidateTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ValidateTemplateError {
    fn from(err: HttpDispatchError) -> ValidateTemplateError {
        ValidateTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for ValidateTemplateError {
    fn from(err: io::Error) -> ValidateTemplateError {
        ValidateTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ValidateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ValidateTemplateError {
    fn description(&self) -> &str {
        match *self {
            ValidateTemplateError::Validation(ref cause) => cause,
            ValidateTemplateError::Credentials(ref err) => err.description(),
            ValidateTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ValidateTemplateError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS CloudFormation API. AWS CloudFormation clients implement this trait.
pub trait CloudFormation {
    #[doc="<p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE_IN_PROGRESS state.</p> </note>"]
    fn cancel_update_stack(&self,
                           input: &CancelUpdateStackInput)
                           -> Result<(), CancelUpdateStackError>;


    #[doc="<p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed\"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>"]
    fn continue_update_rollback
        (&self,
         input: &ContinueUpdateRollbackInput)
         -> Result<ContinueUpdateRollbackOutput, ContinueUpdateRollbackError>;


    #[doc="<p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>"]
    fn create_change_set(&self,
                         input: &CreateChangeSetInput)
                         -> Result<CreateChangeSetOutput, CreateChangeSetError>;


    #[doc="<p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>"]
    fn create_stack(&self,
                    input: &CreateStackInput)
                    -> Result<CreateStackOutput, CreateStackError>;


    #[doc="<p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>"]
    fn delete_change_set(&self,
                         input: &DeleteChangeSetInput)
                         -> Result<DeleteChangeSetOutput, DeleteChangeSetError>;


    #[doc="<p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>"]
    fn delete_stack(&self, input: &DeleteStackInput) -> Result<(), DeleteStackError>;


    #[doc="<p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account.</p>"]
    fn describe_account_limits
        (&self,
         input: &DescribeAccountLimitsInput)
         -> Result<DescribeAccountLimitsOutput, DescribeAccountLimitsError>;


    #[doc="<p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html\">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>"]
    fn describe_change_set(&self,
                           input: &DescribeChangeSetInput)
                           -> Result<DescribeChangeSetOutput, DescribeChangeSetError>;


    #[doc="<p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack's event history, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html\">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note>"]
    fn describe_stack_events(&self,
                             input: &DescribeStackEventsInput)
                             -> Result<DescribeStackEventsOutput, DescribeStackEventsError>;


    #[doc="<p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>"]
    fn describe_stack_resource
        (&self,
         input: &DescribeStackResourceInput)
         -> Result<DescribeStackResourceOutput, DescribeStackResourceError>;


    #[doc="<p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/\">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note>"]
    fn describe_stack_resources
        (&self,
         input: &DescribeStackResourcesInput)
         -> Result<DescribeStackResourcesOutput, DescribeStackResourcesError>;


    #[doc="<p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksInput)
                       -> Result<DescribeStacksOutput, DescribeStacksError>;


    #[doc="<p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>"]
    fn estimate_template_cost(&self,
                              input: &EstimateTemplateCostInput)
                              -> Result<EstimateTemplateCostOutput, EstimateTemplateCostError>;


    #[doc="<p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>"]
    fn execute_change_set(&self,
                          input: &ExecuteChangeSetInput)
                          -> Result<ExecuteChangeSetOutput, ExecuteChangeSetError>;


    #[doc="<p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>"]
    fn get_stack_policy(&self,
                        input: &GetStackPolicyInput)
                        -> Result<GetStackPolicyOutput, GetStackPolicyError>;


    #[doc="<p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note>"]
    fn get_template(&self,
                    input: &GetTemplateInput)
                    -> Result<GetTemplateOutput, GetTemplateError>;


    #[doc="<p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>"]
    fn get_template_summary(&self,
                            input: &GetTemplateSummaryInput)
                            -> Result<GetTemplateSummaryOutput, GetTemplateSummaryError>;


    #[doc="<p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>"]
    fn list_change_sets(&self,
                        input: &ListChangeSetsInput)
                        -> Result<ListChangeSetsOutput, ListChangeSetsError>;


    #[doc="<p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html\"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html\"> AWS CloudFormation Export Stack Output Values</a>.</p>"]
    fn list_exports(&self,
                    input: &ListExportsInput)
                    -> Result<ListExportsOutput, ListExportsError>;


    #[doc="<p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html\"> <code>Fn::ImportValue</code> </a> function. </p>"]
    fn list_imports(&self,
                    input: &ListImportsInput)
                    -> Result<ListImportsOutput, ListImportsError>;


    #[doc="<p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>"]
    fn list_stack_resources(&self,
                            input: &ListStackResourcesInput)
                            -> Result<ListStackResourcesOutput, ListStackResourcesError>;


    #[doc="<p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>"]
    fn list_stacks(&self, input: &ListStacksInput) -> Result<ListStacksOutput, ListStacksError>;


    #[doc="<p>Sets a stack policy for a specified stack.</p>"]
    fn set_stack_policy(&self, input: &SetStackPolicyInput) -> Result<(), SetStackPolicyError>;


    #[doc="<p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>"]
    fn signal_resource(&self, input: &SignalResourceInput) -> Result<(), SignalResourceError>;


    #[doc="<p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html\">Updating a Stack</a>.</p>"]
    fn update_stack(&self,
                    input: &UpdateStackInput)
                    -> Result<UpdateStackOutput, UpdateStackError>;


    #[doc="<p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>"]
    fn validate_template(&self,
                         input: &ValidateTemplateInput)
                         -> Result<ValidateTemplateOutput, ValidateTemplateError>;
}
/// A client for the AWS CloudFormation API.
pub struct CloudFormationClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CloudFormationClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudFormationClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CloudFormation for CloudFormationClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE_IN_PROGRESS state.</p> </note>"]
    fn cancel_update_stack(&self,
                           input: &CancelUpdateStackInput)
                           -> Result<(), CancelUpdateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CancelUpdateStack");
        params.put("Version", "2010-05-15");
        CancelUpdateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CancelUpdateStackError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed\"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>"]
    fn continue_update_rollback
        (&self,
         input: &ContinueUpdateRollbackInput)
         -> Result<ContinueUpdateRollbackOutput, ContinueUpdateRollbackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ContinueUpdateRollback");
        params.put("Version", "2010-05-15");
        ContinueUpdateRollbackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ContinueUpdateRollbackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(ContinueUpdateRollbackOutputDeserializer::deserialize("ContinueUpdateRollbackResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ContinueUpdateRollbackError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>"]
    fn create_change_set(&self,
                         input: &CreateChangeSetInput)
                         -> Result<CreateChangeSetOutput, CreateChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateChangeSet");
        params.put("Version", "2010-05-15");
        CreateChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CreateChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateChangeSetOutputDeserializer::deserialize("CreateChangeSetResult",
                                                                                 &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateChangeSetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>"]
    fn create_stack(&self,
                    input: &CreateStackInput)
                    -> Result<CreateStackOutput, CreateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStack");
        params.put("Version", "2010-05-15");
        CreateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = CreateStackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateStackOutputDeserializer::deserialize("CreateStackResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateStackError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>"]
    fn delete_change_set(&self,
                         input: &DeleteChangeSetInput)
                         -> Result<DeleteChangeSetOutput, DeleteChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteChangeSet");
        params.put("Version", "2010-05-15");
        DeleteChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DeleteChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteChangeSetOutputDeserializer::deserialize("DeleteChangeSetResult",
                                                                                 &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteChangeSetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>"]
    fn delete_stack(&self, input: &DeleteStackInput) -> Result<(), DeleteStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStack");
        params.put("Version", "2010-05-15");
        DeleteStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteStackError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account.</p>"]
    fn describe_account_limits
        (&self,
         input: &DescribeAccountLimitsInput)
         -> Result<DescribeAccountLimitsOutput, DescribeAccountLimitsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2010-05-15");
        DescribeAccountLimitsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeAccountLimitsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccountLimitsOutputDeserializer::deserialize("DescribeAccountLimitsResult",
                                                                                       &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeAccountLimitsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html\">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>"]
    fn describe_change_set(&self,
                           input: &DescribeChangeSetInput)
                           -> Result<DescribeChangeSetOutput, DescribeChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeChangeSet");
        params.put("Version", "2010-05-15");
        DescribeChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeChangeSetOutputDeserializer::deserialize("DescribeChangeSetResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeChangeSetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack's event history, go to <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html\">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note>"]
    fn describe_stack_events(&self,
                             input: &DescribeStackEventsInput)
                             -> Result<DescribeStackEventsOutput, DescribeStackEventsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackEvents");
        params.put("Version", "2010-05-15");
        DescribeStackEventsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeStackEventsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackEventsOutputDeserializer::deserialize("DescribeStackEventsResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStackEventsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>"]
    fn describe_stack_resource
        (&self,
         input: &DescribeStackResourceInput)
         -> Result<DescribeStackResourceOutput, DescribeStackResourceError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResource");
        params.put("Version", "2010-05-15");
        DescribeStackResourceInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeStackResourceOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackResourceOutputDeserializer::deserialize("DescribeStackResourceResult",
                                                                                       &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStackResourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/\">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note>"]
    fn describe_stack_resources
        (&self,
         input: &DescribeStackResourcesInput)
         -> Result<DescribeStackResourcesOutput, DescribeStackResourcesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResources");
        params.put("Version", "2010-05-15");
        DescribeStackResourcesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeStackResourcesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(DescribeStackResourcesOutputDeserializer::deserialize("DescribeStackResourcesResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStackResourcesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note>"]
    fn describe_stacks(&self,
                       input: &DescribeStacksInput)
                       -> Result<DescribeStacksOutput, DescribeStacksError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStacks");
        params.put("Version", "2010-05-15");
        DescribeStacksInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = DescribeStacksOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStacksOutputDeserializer::deserialize("DescribeStacksResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStacksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>"]
    fn estimate_template_cost(&self,
                              input: &EstimateTemplateCostInput)
                              -> Result<EstimateTemplateCostOutput, EstimateTemplateCostError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EstimateTemplateCost");
        params.put("Version", "2010-05-15");
        EstimateTemplateCostInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = EstimateTemplateCostOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EstimateTemplateCostOutputDeserializer::deserialize("EstimateTemplateCostResult",
                                                                                      &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(EstimateTemplateCostError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>"]
    fn execute_change_set(&self,
                          input: &ExecuteChangeSetInput)
                          -> Result<ExecuteChangeSetOutput, ExecuteChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExecuteChangeSet");
        params.put("Version", "2010-05-15");
        ExecuteChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ExecuteChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ExecuteChangeSetOutputDeserializer::deserialize("ExecuteChangeSetResult",
                                                                                  &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ExecuteChangeSetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>"]
    fn get_stack_policy(&self,
                        input: &GetStackPolicyInput)
                        -> Result<GetStackPolicyOutput, GetStackPolicyError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetStackPolicy");
        params.put("Version", "2010-05-15");
        GetStackPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetStackPolicyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetStackPolicyOutputDeserializer::deserialize("GetStackPolicyResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetStackPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note>"]
    fn get_template(&self,
                    input: &GetTemplateInput)
                    -> Result<GetTemplateOutput, GetTemplateError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplate");
        params.put("Version", "2010-05-15");
        GetTemplateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetTemplateOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTemplateOutputDeserializer::deserialize("GetTemplateResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTemplateError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>"]
    fn get_template_summary(&self,
                            input: &GetTemplateSummaryInput)
                            -> Result<GetTemplateSummaryOutput, GetTemplateSummaryError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplateSummary");
        params.put("Version", "2010-05-15");
        GetTemplateSummaryInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = GetTemplateSummaryOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTemplateSummaryOutputDeserializer::deserialize("GetTemplateSummaryResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTemplateSummaryError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>"]
    fn list_change_sets(&self,
                        input: &ListChangeSetsInput)
                        -> Result<ListChangeSetsOutput, ListChangeSetsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListChangeSets");
        params.put("Version", "2010-05-15");
        ListChangeSetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListChangeSetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListChangeSetsOutputDeserializer::deserialize("ListChangeSetsResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListChangeSetsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html\"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html\"> AWS CloudFormation Export Stack Output Values</a>.</p>"]
    fn list_exports(&self,
                    input: &ListExportsInput)
                    -> Result<ListExportsOutput, ListExportsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListExports");
        params.put("Version", "2010-05-15");
        ListExportsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListExportsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListExportsOutputDeserializer::deserialize("ListExportsResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListExportsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html\"> <code>Fn::ImportValue</code> </a> function. </p>"]
    fn list_imports(&self,
                    input: &ListImportsInput)
                    -> Result<ListImportsOutput, ListImportsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListImports");
        params.put("Version", "2010-05-15");
        ListImportsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListImportsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListImportsOutputDeserializer::deserialize("ListImportsResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListImportsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>"]
    fn list_stack_resources(&self,
                            input: &ListStackResourcesInput)
                            -> Result<ListStackResourcesOutput, ListStackResourcesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackResources");
        params.put("Version", "2010-05-15");
        ListStackResourcesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListStackResourcesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackResourcesOutputDeserializer::deserialize("ListStackResourcesResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListStackResourcesError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>"]
    fn list_stacks(&self, input: &ListStacksInput) -> Result<ListStacksOutput, ListStacksError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStacks");
        params.put("Version", "2010-05-15");
        ListStacksInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ListStacksOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStacksOutputDeserializer::deserialize("ListStacksResult",
                                                                            &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListStacksError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets a stack policy for a specified stack.</p>"]
    fn set_stack_policy(&self, input: &SetStackPolicyInput) -> Result<(), SetStackPolicyError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetStackPolicy");
        params.put("Version", "2010-05-15");
        SetStackPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetStackPolicyError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>"]
    fn signal_resource(&self, input: &SignalResourceInput) -> Result<(), SignalResourceError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SignalResource");
        params.put("Version", "2010-05-15");
        SignalResourceInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SignalResourceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href=\"http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html\">Updating a Stack</a>.</p>"]
    fn update_stack(&self,
                    input: &UpdateStackInput)
                    -> Result<UpdateStackOutput, UpdateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStack");
        params.put("Version", "2010-05-15");
        UpdateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = UpdateStackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateStackOutputDeserializer::deserialize("UpdateStackResult",
                                                                             &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateStackError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>"]
    fn validate_template(&self,
                         input: &ValidateTemplateInput)
                         -> Result<ValidateTemplateOutput, ValidateTemplateError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ValidateTemplate");
        params.put("Version", "2010-05-15");
        ValidateTemplateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let mut response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body.is_empty() {
                    result = ValidateTemplateOutput::default();
                } else {
                    let reader = EventReader::new_with_config(body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ValidateTemplateOutputDeserializer::deserialize("ValidateTemplateResult",
                                                                                  &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ValidateTemplateError::from_body(String::from_utf8_lossy(&body).as_ref()))
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
    fn test_parse_error_cloudformation_cancel_update_stack() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/error",
                                                              "cloudformation-cancel-update-stack.xml");
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CancelUpdateStackInput::default();
        let result = client.cancel_update_stack(&request);
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudformation_describe_stacks() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudformation-describe-stacks.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeStacksInput::default();
        let result = client.describe_stacks(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_cloudformation_get_template() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudformation-get-template.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetTemplateInput::default();
        let result = client.get_template(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }


    #[test]
    fn test_parse_valid_cloudformation_list_stacks() {
        let mock_response = MockResponseReader::read_response("test_resources/generated/valid",
                                                              "cloudformation-list-stacks.xml");
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListStacksInput::default();
        let result = client.list_stacks(&request);
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
