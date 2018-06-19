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
struct AccountDeserializer;
impl AccountDeserializer {
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
/// <p>Structure that contains the results of the account gate function which AWS CloudFormation invokes, if present, before proceeding with a stack set operation in an account and region.</p> <p>For each account and region, AWS CloudFormation lets you specify a Lamdba function that encapsulates any requirements that must be met before CloudFormation can proceed with a stack set operation in that account and region. CloudFormation invokes the function each time a stack set operation is requested for that account and region; if the function returns <code>FAILED</code>, CloudFormation cancels the operation in that account and region, and sets the stack set operation result status for that account and region to <code>FAILED</code>. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-account-gating.html">Configuring a target account gate</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountGateResult {
    /// <p><p>The status of the account gate function.</p> <ul> <li> <p> <code>SUCCEEDED</code>: The account gate function has determined that the account and region passes any requirements for a stack set operation to occur. AWS CloudFormation proceeds with the stack operation in that account and region. </p> </li> <li> <p> <code>FAILED</code>: The account gate function has determined that the account and region does not meet the requirements for a stack set operation to occur. AWS CloudFormation cancels the stack set operation in that account and region, and sets the stack set operation result status for that account and region to <code>FAILED</code>. </p> </li> <li> <p> <code>SKIPPED</code>: AWS CloudFormation has skipped calling the account gate function for this account and region, for one of the following reasons:</p> <ul> <li> <p>An account gate function has not been specified for the account and region. AWS CloudFormation proceeds with the stack set operation in this account and region.</p> </li> <li> <p>The <code>AWSCloudFormationStackSetExecutionRole</code> of the stack set adminstration account lacks permissions to invoke the function. AWS CloudFormation proceeds with the stack set operation in this account and region.</p> </li> <li> <p>Either no action is necessary, or no action is possible, on the stack. AWS CloudFormation skips the stack set operation in this account and region.</p> </li> </ul> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The reason for the account gate status assigned to this account and region for the stack set operation.</p>
    pub status_reason: Option<String>,
}

struct AccountGateResultDeserializer;
impl AccountGateResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountGateResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AccountGateResult::default();

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
                    "Status" => {
                        obj.status = Some(try!(AccountGateStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "StatusReason" => {
                        obj.status_reason = Some(try!(
                            AccountGateStatusReasonDeserializer::deserialize("StatusReason", stack)
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
struct AccountGateStatusDeserializer;
impl AccountGateStatusDeserializer {
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
struct AccountGateStatusReasonDeserializer;
impl AccountGateStatusReasonDeserializer {
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
/// <p>The AccountLimit data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountLimit {
    /// <p>The name of the account limit. Currently, the only account limit is <code>StackLimit</code>.</p>
    pub name: Option<String>,
    /// <p>The value that is associated with the account limit name.</p>
    pub value: Option<i64>,
}

struct AccountLimitDeserializer;
impl AccountLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountLimit, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Name" => {
                        obj.name = Some(try!(LimitNameDeserializer::deserialize("Name", stack)));
                    }
                    "Value" => {
                        obj.value = Some(try!(LimitValueDeserializer::deserialize("Value", stack)));
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
struct AccountLimitListDeserializer;
impl AccountLimitListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AccountLimit>, XmlParseError> {
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

/// Serialize `AccountList` contents to a `SignedRequest`.
struct AccountListSerializer;
impl AccountListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct AllowedValueDeserializer;
impl AllowedValueDeserializer {
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
struct AllowedValuesDeserializer;
impl AllowedValuesDeserializer {
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
struct ArnDeserializer;
impl ArnDeserializer {
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
/// <p>The input for the <a>CancelUpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CancelUpdateStackInput {
    /// <p>A unique identifier for this <code>CancelUpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to cancel an update on a stack with the same name. You might retry <code>CancelUpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack.</p>
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
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

struct CapabilitiesDeserializer;
impl CapabilitiesDeserializer {
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
struct CapabilityDeserializer;
impl CapabilityDeserializer {
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
struct CausingEntityDeserializer;
impl CausingEntityDeserializer {
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
/// <p>The <code>Change</code> structure describes the changes AWS CloudFormation will perform if you execute the change set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Change {
    /// <p>A <code>ResourceChange</code> structure that describes the resource and action that AWS CloudFormation will perform.</p>
    pub resource_change: Option<ResourceChange>,
    /// <p>The type of entity that AWS CloudFormation changes. Currently, the only entity type is <code>Resource</code>.</p>
    pub type_: Option<String>,
}

struct ChangeDeserializer;
impl ChangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Change, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "ResourceChange" => {
                        obj.resource_change = Some(try!(ResourceChangeDeserializer::deserialize(
                            "ResourceChange",
                            stack
                        )));
                    }
                    "Type" => {
                        obj.type_ = Some(try!(ChangeTypeDeserializer::deserialize("Type", stack)));
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
struct ChangeActionDeserializer;
impl ChangeActionDeserializer {
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
struct ChangeSetIdDeserializer;
impl ChangeSetIdDeserializer {
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
struct ChangeSetNameDeserializer;
impl ChangeSetNameDeserializer {
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
struct ChangeSetStatusDeserializer;
impl ChangeSetStatusDeserializer {
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
struct ChangeSetStatusReasonDeserializer;
impl ChangeSetStatusReasonDeserializer {
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
struct ChangeSetSummariesDeserializer;
impl ChangeSetSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ChangeSetSummary>, XmlParseError> {
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
                        obj.push(try!(ChangeSetSummaryDeserializer::deserialize(
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
/// <p>The <code>ChangeSetSummary</code> structure describes a change set, its status, and the stack with which it's associated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChangeSetSummary {
    /// <p>The ID of the change set.</p>
    pub change_set_id: Option<String>,
    /// <p>The name of the change set.</p>
    pub change_set_name: Option<String>,
    /// <p>The start time when the change set was created, in UTC.</p>
    pub creation_time: Option<String>,
    /// <p>Descriptive information about the change set.</p>
    pub description: Option<String>,
    /// <p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change set. If you can’t execute the change set, the status indicates why. For example, a change set might be in an <code>UNAVAILABLE</code> state because AWS CloudFormation is still creating it or in an <code>OBSOLETE</code> state because the stack was already updated.</p>
    pub execution_status: Option<String>,
    /// <p>The ID of the stack with which the change set is associated.</p>
    pub stack_id: Option<String>,
    /// <p>The name of the stack with which the change set is associated.</p>
    pub stack_name: Option<String>,
    /// <p>The state of the change set, such as <code>CREATE_IN_PROGRESS</code>, <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p>
    pub status: Option<String>,
    /// <p>A description of the change set's status. For example, if your change set is in the <code>FAILED</code> state, AWS CloudFormation shows the error message.</p>
    pub status_reason: Option<String>,
}

struct ChangeSetSummaryDeserializer;
impl ChangeSetSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeSetSummary, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "ChangeSetId" => {
                        obj.change_set_id = Some(try!(ChangeSetIdDeserializer::deserialize(
                            "ChangeSetId",
                            stack
                        )));
                    }
                    "ChangeSetName" => {
                        obj.change_set_name = Some(try!(ChangeSetNameDeserializer::deserialize(
                            "ChangeSetName",
                            stack
                        )));
                    }
                    "CreationTime" => {
                        obj.creation_time = Some(try!(CreationTimeDeserializer::deserialize(
                            "CreationTime",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "ExecutionStatus" => {
                        obj.execution_status = Some(try!(
                            ExecutionStatusDeserializer::deserialize("ExecutionStatus", stack)
                        ));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            Some(try!(StackNameDeserializer::deserialize("StackName", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(ChangeSetStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "StatusReason" => {
                        obj.status_reason = Some(try!(
                            ChangeSetStatusReasonDeserializer::deserialize("StatusReason", stack)
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
struct ChangeSourceDeserializer;
impl ChangeSourceDeserializer {
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
struct ChangeTypeDeserializer;
impl ChangeTypeDeserializer {
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
struct ChangesDeserializer;
impl ChangesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Change>, XmlParseError> {
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
/// <p>The input for the <a>ContinueUpdateRollback</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinueUpdateRollbackInput {
    /// <p>A unique identifier for this <code>ContinueUpdateRollback</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to continue the rollback to a stack with the same name. You might retry <code>ContinueUpdateRollback</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p><p>A list of the logical IDs of the resources that AWS CloudFormation skips during the continue update rollback operation. You can specify only resources that are in the <code>UPDATE<em>FAILED</code> state because a rollback failed. You can&#39;t specify resources that are in the <code>UPDATE</em>FAILED</code> state for other reasons, for example, because an update was cancelled. To check why a resource update failed, use the <a>DescribeStackResources</a> action, and view the resource status reason. </p> <important> <p>Specify this property to skip rolling back resources that AWS CloudFormation can&#39;t successfully roll back. We recommend that you <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> troubleshoot</a> resources before skipping them. AWS CloudFormation sets the status of the specified resources to <code>UPDATE<em>COMPLETE</code> and continues to roll back the stack. After the rollback is complete, the state of the skipped resources will be inconsistent with the state of the resources in the stack template. Before performing another stack update, you must update the stack or resources to be consistent with each other. If you don&#39;t, subsequent stack updates might fail, and the stack will become unrecoverable. </p> </important> <p>Specify the minimum number of resources required to successfully roll back your stack. For example, a failed resource update might cause dependent resources to fail. In this case, it might not be necessary to skip the dependent resources. </p> <p>To skip resources that are part of nested stacks, use the following format: <code>NestedStackName.ResourceLogicalID</code>. If you want to specify the logical ID of a stack resource (<code>Type: AWS::CloudFormation::Stack</code>) in the <code>ResourcesToSkip</code> list, then its corresponding embedded stack must be in one of the following states: <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>COMPLETE</code>, or <code>DELETE_FAILED</code>. </p> <note> <p>Don&#39;t confuse a child stack&#39;s name with its corresponding logical ID defined in the parent stack. For an example of a continue update rollback operation with nested stacks, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-continueupdaterollback.html#nested-stacks">Using ResourcesToSkip to recover a nested stacks hierarchy</a>. </p> </note></p>
    pub resources_to_skip: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to roll back the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p><p>The name or the unique ID of the stack that you want to continue rolling back.</p> <note> <p>Don&#39;t specify the name of a nested stack (a stack that was created by using the <code>AWS::CloudFormation::Stack</code> resource). Instead, use this operation on the parent stack (the stack that contains the <code>AWS::CloudFormation::Stack</code> resource).</p> </note></p>
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
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resources_to_skip {
            ResourcesToSkipSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourcesToSkip"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

/// <p>The output for a <a>ContinueUpdateRollback</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinueUpdateRollbackOutput {}

struct ContinueUpdateRollbackOutputDeserializer;
impl ContinueUpdateRollbackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContinueUpdateRollbackOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ContinueUpdateRollbackOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The input for the <a>CreateChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateChangeSetInput {
    /// <p>A list of values that you must specify before AWS CloudFormation can update certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a>, and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The name of the change set. The name must be unique among all change sets that are associated with the specified stack.</p> <p>A change set name can contain only alphanumeric, case sensitive characters and hyphens. It must start with an alphabetic character and cannot exceed 128 characters.</p>
    pub change_set_name: String,
    /// <p>The type of change set operation. To create a change set for a new stack, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code>.</p> <p>If you create a change set for a new stack, AWS Cloudformation creates a stack with a unique stack ID, but no template or resources. The stack will be in the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-describing-stacks.html#d0e11995"> <code>REVIEW_IN_PROGRESS</code> </a> state until you execute the change set.</p> <p>By default, AWS CloudFormation specifies <code>UPDATE</code>. You can't use the <code>UPDATE</code> type to create a change set for a new stack or the <code>CREATE</code> type to create a change set for an existing stack.</p>
    pub change_set_type: Option<String>,
    /// <p>A unique identifier for this <code>CreateChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create another change set with the same name. You might retry <code>CreateChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_token: Option<String>,
    /// <p>A description to help you identify this change set.</p>
    pub description: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of Amazon Simple Notification Service (Amazon SNS) topics that AWS CloudFormation associates with the stack. To remove all associated notification topics, specify an empty list.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the change set. For more information, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with if you execute this change set, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource type that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for condition keys in IAM policies for AWS CloudFormation. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a> in the AWS CloudFormation User Guide.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes when executing the change set. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates the change set by comparing this stack's information with the information that you submit, such as a modified template or different parameter input values.</p>
    pub stack_name: String,
    /// <p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to resources in the stack. You can specify a maximum of 50 tags.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>A structure that contains the body of the revised template, with a minimum length of 1 byte and a maximum length of 51,200 bytes. AWS CloudFormation generates the change set by comparing this template with the template of the stack that you specified.</p> <p>Conditional: You must specify only <code>TemplateBody</code> or <code>TemplateURL</code>.</p>
    pub template_body: Option<String>,
    /// <p>The location of the file that contains the revised template. The URL must point to a template (max size: 460,800 bytes) that is located in an S3 bucket. AWS CloudFormation generates the change set by comparing this template with the stack that you specified.</p> <p>Conditional: You must specify only <code>TemplateBody</code> or <code>TemplateURL</code>.</p>
    pub template_url: Option<String>,
    /// <p>Whether to reuse the template that is associated with the stack to create the change set.</p>
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
            CapabilitiesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Capabilities"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.change_set_type {
            params.put(
                &format!("{}{}", prefix, "ChangeSetType"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.client_token {
            params.put(
                &format!("{}{}", prefix, "ClientToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NotificationARNs"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourceTypes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.use_previous_template {
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>CreateChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateChangeSetOutput {
    /// <p>The Amazon Resource Name (ARN) of the change set.</p>
    pub id: Option<String>,
    /// <p>The unique ID of the stack.</p>
    pub stack_id: Option<String>,
}

struct CreateChangeSetOutputDeserializer;
impl CreateChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateChangeSetOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Id" => {
                        obj.id = Some(try!(ChangeSetIdDeserializer::deserialize("Id", stack)));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
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
/// <p>The input for <a>CreateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackInput {
    /// <p>A list of values that you must specify before AWS CloudFormation can create certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a>, and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A unique identifier for this <code>CreateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create a stack with the same name. You might retry <code>CreateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>Set to <code>true</code> to disable rollback of the stack if stack creation failed. You can specify either <code>DisableRollback</code> or <code>OnFailure</code>, but not both.</p> <p>Default: <code>false</code> </p>
    pub disable_rollback: Option<bool>,
    /// <p>Whether to enable termination protection on the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>. Termination protection is disabled on stacks by default. </p> <p> For <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack.</p>
    pub enable_termination_protection: Option<bool>,
    /// <p>The Simple Notification Service (SNS) topic ARNs to publish stack related events. You can find your SNS topic ARNs using the SNS console or your Command Line Interface (CLI).</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>Determines what action will be taken if stack creation fails. This must be one of: DO_NOTHING, ROLLBACK, or DELETE. You can specify either <code>OnFailure</code> or <code>DisableRollback</code>, but not both.</p> <p>Default: <code>ROLLBACK</code> </p>
    pub on_failure: Option<String>,
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with for this create stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>. Use the following syntax to describe template resource types: <code>AWS::*</code> (for all AWS resource), <code>Custom::*</code> (for all custom resources), <code>Custom::<i>logical_ID</i> </code> (for a specific custom resource), <code>AWS::<i>service_name</i>::*</code> (for all resources of a particular AWS service), and <code>AWS::<i>service_name</i>::<i>resource_logical_ID</i> </code> (for a specific AWS resource).</p> <p>If the list of resource types doesn't include a resource that you're creating, the stack creation fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a>.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to create the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p><p>The name that is associated with the stack. The name must be unique in the region in which you are creating the stack.</p> <note> <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.</p> </note></p>
    pub stack_name: String,
    /// <p>Structure containing the stack policy body. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the <i>AWS CloudFormation User Guide</i>. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_body: Option<String>,
    /// <p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_url: Option<String>,
    /// <p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to the resources created in the stack. A maximum number of 50 tags can be specified.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    pub template_url: Option<String>,
    /// <p>The amount of time that can pass before the stack status becomes CREATE_FAILED; if <code>DisableRollback</code> is not set or is set to <code>false</code>, the stack will be rolled back.</p>
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
            CapabilitiesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Capabilities"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.client_request_token {
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.disable_rollback {
            params.put(
                &format!("{}{}", prefix, "DisableRollback"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.enable_termination_protection {
            params.put(
                &format!("{}{}", prefix, "EnableTerminationProtection"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NotificationARNs"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.on_failure {
            params.put(
                &format!("{}{}", prefix, "OnFailure"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourceTypes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(
                &format!("{}{}", prefix, "StackPolicyBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(
                &format!("{}{}", prefix, "StackPolicyURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.timeout_in_minutes {
            params.put(
                &format!("{}{}", prefix, "TimeoutInMinutes"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackInstancesInput {
    /// <p>The names of one or more AWS accounts that you want to create stack instances in the specified region(s) for.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>A list of stack set parameters whose values you want to override in the selected stack instances.</p> <p>Any overridden parameter values will be applied to all stack instances in the specified accounts and regions. When specifying parameters and their values, be aware of how AWS CloudFormation sets parameter values during stack instance operations:</p> <ul> <li> <p>To override the current value for a parameter, include the parameter and specify its value.</p> </li> <li> <p>To leave a parameter set to its present value, you can do one of the following:</p> <ul> <li> <p>Do not include the parameter in the list.</p> </li> <li> <p>Include the parameter and specify <code>UsePreviousValue</code> as <code>true</code>. (You cannot specify both a value and set <code>UsePreviousValue</code> to <code>true</code>.)</p> </li> </ul> </li> <li> <p>To set all overridden parameter back to the values specified in the stack set, specify a parameter list but do not include any parameters.</p> </li> <li> <p>To leave all parameters set to their present values, do not specify this property at all.</p> </li> </ul> <p>During stack set updates, any parameter values overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only override the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template.</p>
    pub parameter_overrides: Option<Vec<Parameter>>,
    /// <p>The names of one or more regions where you want to create stack instances using the specified AWS account(s). </p>
    pub regions: Vec<String>,
    /// <p>The name or unique ID of the stack set that you want to create stack instances from.</p>
    pub stack_set_name: String,
}

/// Serialize `CreateStackInstancesInput` contents to a `SignedRequest`.
struct CreateStackInstancesInputSerializer;
impl CreateStackInstancesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackInstancesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AccountListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Accounts"),
            &obj.accounts,
        );
        if let Some(ref field_value) = obj.operation_id {
            params.put(
                &format!("{}{}", prefix, "OperationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.operation_preferences {
            StackSetOperationPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OperationPreferences"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.parameter_overrides {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ParameterOverrides"),
                field_value,
            );
        }
        RegionListSerializer::serialize(params, &format!("{}{}", prefix, "Regions"), &obj.regions);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackInstancesOutput {
    /// <p>The unique identifier for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct CreateStackInstancesOutputDeserializer;
impl CreateStackInstancesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackInstancesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStackInstancesOutput::default();

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
                    "OperationId" => {
                        obj.operation_id = Some(try!(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
/// <p>The output for a <a>CreateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackOutput {
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
}

struct CreateStackOutputDeserializer;
impl CreateStackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
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
pub struct CreateStackSetInput {
    /// <p>The Amazon Resource Number (ARN) of the IAM role to use to create this stack set. </p> <p>Specify an IAM role only if you are using customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Define Permissions for Multiple Administrators</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub administration_role_arn: Option<String>,
    /// <p>A list of values that you must specify before AWS CloudFormation can create certain stack sets. Some stack set templates might include resources that can affect permissions in your AWS account—for example, by creating new AWS Identity and Access Management (IAM) users. For those stack sets, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are CAPABILITY_IAM and CAPABILITY_NAMED_IAM. The following resources require you to specify this parameter: </p> <ul> <li> <p>AWS::IAM::AccessKey</p> </li> <li> <p>AWS::IAM::Group</p> </li> <li> <p>AWS::IAM::InstanceProfile</p> </li> <li> <p>AWS::IAM::Policy</p> </li> <li> <p>AWS::IAM::Role</p> </li> <li> <p>AWS::IAM::User</p> </li> <li> <p>AWS::IAM::UserToGroupAddition</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions that are associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify CAPABILITY_NAMED_IAM. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates.</a> </p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A unique identifier for this <code>CreateStackSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create another stack set with the same name. You might retry <code>CreateStackSet</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p>
    pub client_request_token: Option<String>,
    /// <p>A description of the stack set. You can use the description to identify the stack set's purpose or other important information.</p>
    pub description: Option<String>,
    /// <p>The input parameters for the stack set template. </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p><p>The name to associate with the stack set. The name must be unique in the region where you create your stack set.</p> <note> <p>A stack name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and can&#39;t be longer than 128 characters.</p> </note></p>
    pub stack_set_name: String,
    /// <p>The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the stacks. A maximum number of 50 tags can be specified.</p> <p>If you specify tags as part of a <code>CreateStackSet</code> action, AWS CloudFormation checks to see if you have the required IAM permission to tag resources. If you don't, the entire <code>CreateStackSet</code> action fails with an <code>access denied</code> error, and the stack set is not created.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the TemplateBody or the TemplateURL parameter, but not both.</p>
    pub template_body: Option<String>,
    /// <p>The location of the file that contains the template body. The URL must point to a template (maximum size: 460,800 bytes) that's located in an Amazon S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the TemplateBody or the TemplateURL parameter, but not both.</p>
    pub template_url: Option<String>,
}

/// Serialize `CreateStackSetInput` contents to a `SignedRequest`.
struct CreateStackSetInputSerializer;
impl CreateStackSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.administration_role_arn {
            params.put(
                &format!("{}{}", prefix, "AdministrationRoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.capabilities {
            CapabilitiesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Capabilities"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.client_request_token {
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackSetOutput {
    /// <p>The ID of the stack set that you're creating.</p>
    pub stack_set_id: Option<String>,
}

struct CreateStackSetOutputDeserializer;
impl CreateStackSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateStackSetOutput::default();

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
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
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
struct CreationTimeDeserializer;
impl CreationTimeDeserializer {
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
/// <p>The input for the <a>DeleteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteChangeSetInput {
    /// <p>The name or Amazon Resource Name (ARN) of the change set that you want to delete.</p>
    pub change_set_name: String,
    /// <p>If you specified the name of a change set to delete, specify the stack name or ID (ARN) that is associated with it.</p>
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

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>DeleteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteChangeSetOutput {}

struct DeleteChangeSetOutputDeserializer;
impl DeleteChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteChangeSetOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The input for <a>DeleteStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackInput {
    /// <p>A unique identifier for this <code>DeleteStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to delete a stack with the same name. You might retry <code>DeleteStack</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>For stacks in the <code>DELETE_FAILED</code> state, a list of resource logical IDs that are associated with the resources you want to retain. During deletion, AWS CloudFormation deletes the stack but does not delete the retained resources.</p> <p>Retaining resources is useful when you cannot delete a resource, such as a non-empty S3 bucket, but you want to delete the stack.</p>
    pub retain_resources: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to delete the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack.</p>
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
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.retain_resources {
            RetainResourcesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RetainResources"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackInstancesInput {
    /// <p>The names of the AWS accounts that you want to delete stack instances for.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You can retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>The regions where you want to delete stack set instances. </p>
    pub regions: Vec<String>,
    /// <p>Removes the stack instances from the specified stack set, but doesn't delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
    pub retain_stacks: bool,
    /// <p>The name or unique ID of the stack set that you want to delete stack instances for.</p>
    pub stack_set_name: String,
}

/// Serialize `DeleteStackInstancesInput` contents to a `SignedRequest`.
struct DeleteStackInstancesInputSerializer;
impl DeleteStackInstancesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackInstancesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AccountListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Accounts"),
            &obj.accounts,
        );
        if let Some(ref field_value) = obj.operation_id {
            params.put(
                &format!("{}{}", prefix, "OperationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.operation_preferences {
            StackSetOperationPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OperationPreferences"),
                field_value,
            );
        }
        RegionListSerializer::serialize(params, &format!("{}{}", prefix, "Regions"), &obj.regions);
        params.put(
            &format!("{}{}", prefix, "RetainStacks"),
            &obj.retain_stacks.to_string().replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackInstancesOutput {
    /// <p>The unique identifier for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct DeleteStackInstancesOutputDeserializer;
impl DeleteStackInstancesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteStackInstancesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteStackInstancesOutput::default();

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
                    "OperationId" => {
                        obj.operation_id = Some(try!(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
pub struct DeleteStackSetInput {
    /// <p>The name or unique ID of the stack set that you're deleting. You can obtain this value by running <a>ListStackSets</a>.</p>
    pub stack_set_name: String,
}

/// Serialize `DeleteStackSetInput` contents to a `SignedRequest`.
struct DeleteStackSetInputSerializer;
impl DeleteStackSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackSetOutput {}

struct DeleteStackSetOutputDeserializer;
impl DeleteStackSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteStackSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = DeleteStackSetOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct DeletionTimeDeserializer;
impl DeletionTimeDeserializer {
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
/// <p>The input for the <a>DescribeAccountLimits</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsInput {
    /// <p>A string that identifies the next page of limits that you want to retrieve.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>DescribeAccountLimits</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsOutput {
    /// <p>An account limit structure that contain a list of AWS CloudFormation account limits and their values.</p>
    pub account_limits: Option<Vec<AccountLimit>>,
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of limits. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
}

struct DescribeAccountLimitsOutputDeserializer;
impl DescribeAccountLimitsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "AccountLimits" => {
                        obj.account_limits = Some(try!(AccountLimitListDeserializer::deserialize(
                            "AccountLimits",
                            stack
                        )));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
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
/// <p>The input for the <a>DescribeChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeChangeSetInput {
    /// <p>The name or Amazon Resource Name (ARN) of the change set that you want to describe.</p>
    pub change_set_name: String,
    /// <p>A string (provided by the <a>DescribeChangeSet</a> response output) that identifies the next page of information that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>If you specified the name of a change set, specify the stack name or ID (ARN) of the change set you want to describe.</p>
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

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>DescribeChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeChangeSetOutput {
    /// <p>If you execute the change set, the list of capabilities that were explicitly acknowledged when the change set was created.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The ARN of the change set.</p>
    pub change_set_id: Option<String>,
    /// <p>The name of the change set.</p>
    pub change_set_name: Option<String>,
    /// <p>A list of <code>Change</code> structures that describes the resources AWS CloudFormation changes if you execute the change set.</p>
    pub changes: Option<Vec<Change>>,
    /// <p>The start time when the change set was created, in UTC.</p>
    pub creation_time: Option<String>,
    /// <p>Information about the change set.</p>
    pub description: Option<String>,
    /// <p>If the change set execution status is <code>AVAILABLE</code>, you can execute the change set. If you can’t execute the change set, the status indicates why. For example, a change set might be in an <code>UNAVAILABLE</code> state because AWS CloudFormation is still creating it or in an <code>OBSOLETE</code> state because the stack was already updated.</p>
    pub execution_status: Option<String>,
    /// <p>If the output exceeds 1 MB, a string that identifies the next page of changes. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>The ARNs of the Amazon Simple Notification Service (Amazon SNS) topics that will be associated with the stack if you execute the change set.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of <code>Parameter</code> structures that describes the input parameters and their values used to create the change set. For more information, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The ARN of the stack that is associated with the change set.</p>
    pub stack_id: Option<String>,
    /// <p>The name of the stack that is associated with the change set.</p>
    pub stack_name: Option<String>,
    /// <p>The current status of the change set, such as <code>CREATE_IN_PROGRESS</code>, <code>CREATE_COMPLETE</code>, or <code>FAILED</code>.</p>
    pub status: Option<String>,
    /// <p>A description of the change set's status. For example, if your attempt to create a change set failed, AWS CloudFormation shows the error message.</p>
    pub status_reason: Option<String>,
    /// <p>If you execute the change set, the tags that will be associated with the stack.</p>
    pub tags: Option<Vec<Tag>>,
}

struct DescribeChangeSetOutputDeserializer;
impl DescribeChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeChangeSetOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Capabilities" => {
                        obj.capabilities = Some(try!(CapabilitiesDeserializer::deserialize(
                            "Capabilities",
                            stack
                        )));
                    }
                    "ChangeSetId" => {
                        obj.change_set_id = Some(try!(ChangeSetIdDeserializer::deserialize(
                            "ChangeSetId",
                            stack
                        )));
                    }
                    "ChangeSetName" => {
                        obj.change_set_name = Some(try!(ChangeSetNameDeserializer::deserialize(
                            "ChangeSetName",
                            stack
                        )));
                    }
                    "Changes" => {
                        obj.changes =
                            Some(try!(ChangesDeserializer::deserialize("Changes", stack)));
                    }
                    "CreationTime" => {
                        obj.creation_time = Some(try!(CreationTimeDeserializer::deserialize(
                            "CreationTime",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "ExecutionStatus" => {
                        obj.execution_status = Some(try!(
                            ExecutionStatusDeserializer::deserialize("ExecutionStatus", stack)
                        ));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "NotificationARNs" => {
                        obj.notification_ar_ns = Some(try!(
                            NotificationARNsDeserializer::deserialize("NotificationARNs", stack)
                        ));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    "RollbackConfiguration" => {
                        obj.rollback_configuration =
                            Some(try!(RollbackConfigurationDeserializer::deserialize(
                                "RollbackConfiguration",
                                stack
                            )));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            Some(try!(StackNameDeserializer::deserialize("StackName", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(ChangeSetStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "StatusReason" => {
                        obj.status_reason = Some(try!(
                            ChangeSetStatusReasonDeserializer::deserialize("StatusReason", stack)
                        ));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagsDeserializer::deserialize("Tags", stack)));
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
/// <p>The input for <a>DescribeStackEvents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackEventsInput {
    /// <p>A string that identifies the next page of events that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for a <a>DescribeStackEvents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackEventsOutput {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of events. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackEvents</code> structures.</p>
    pub stack_events: Option<Vec<StackEvent>>,
}

struct DescribeStackEventsOutputDeserializer;
impl DescribeStackEventsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackEventsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "StackEvents" => {
                        obj.stack_events = Some(try!(StackEventsDeserializer::deserialize(
                            "StackEvents",
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
pub struct DescribeStackInstanceInput {
    /// <p>The ID of an AWS account that's associated with this stack instance.</p>
    pub stack_instance_account: String,
    /// <p>The name of a region that's associated with this stack instance.</p>
    pub stack_instance_region: String,
    /// <p>The name or the unique stack ID of the stack set that you want to get stack instance information for.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackInstanceInput` contents to a `SignedRequest`.
struct DescribeStackInstanceInputSerializer;
impl DescribeStackInstanceInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackInstanceInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackInstanceAccount"),
            &obj.stack_instance_account.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackInstanceRegion"),
            &obj.stack_instance_region.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackInstanceOutput {
    /// <p>The stack instance that matches the specified request parameters.</p>
    pub stack_instance: Option<StackInstance>,
}

struct DescribeStackInstanceOutputDeserializer;
impl DescribeStackInstanceOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackInstanceOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackInstanceOutput::default();

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
                    "StackInstance" => {
                        obj.stack_instance = Some(try!(StackInstanceDeserializer::deserialize(
                            "StackInstance",
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
/// <p>The input for <a>DescribeStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourceInput {
    /// <p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>
    pub logical_resource_id: String,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
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

        params.put(
            &format!("{}{}", prefix, "LogicalResourceId"),
            &obj.logical_resource_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

/// <p>The output for a <a>DescribeStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourceOutput {
    /// <p>A <code>StackResourceDetail</code> structure containing the description of the specified resource in the specified stack.</p>
    pub stack_resource_detail: Option<StackResourceDetail>,
}

struct DescribeStackResourceOutputDeserializer;
impl DescribeStackResourceOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackResourceOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StackResourceDetail" => {
                        obj.stack_resource_detail =
                            Some(try!(StackResourceDetailDeserializer::deserialize(
                                "StackResourceDetail",
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
/// <p>The input for <a>DescribeStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourcesInput {
    /// <p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>
    pub logical_resource_id: Option<String>,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p> <p>For example, for an Amazon Elastic Compute Cloud (EC2) instance, <code>PhysicalResourceId</code> corresponds to the <code>InstanceId</code>. You can pass the EC2 <code>InstanceId</code> to <code>DescribeStackResources</code> to find which stack the instance belongs to and what other resources are part of the stack.</p> <p>Required: Conditional. If you do not specify <code>PhysicalResourceId</code>, you must specify <code>StackName</code>.</p> <p>Default: There is no default value.</p>
    pub physical_resource_id: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p> <p>Required: Conditional. If you do not specify <code>StackName</code>, you must specify <code>PhysicalResourceId</code>.</p>
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
            params.put(
                &format!("{}{}", prefix, "LogicalResourceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.physical_resource_id {
            params.put(
                &format!("{}{}", prefix, "PhysicalResourceId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for a <a>DescribeStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourcesOutput {
    /// <p>A list of <code>StackResource</code> structures.</p>
    pub stack_resources: Option<Vec<StackResource>>,
}

struct DescribeStackResourcesOutputDeserializer;
impl DescribeStackResourcesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackResourcesOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StackResources" => {
                        obj.stack_resources = Some(try!(StackResourcesDeserializer::deserialize(
                            "StackResources",
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
pub struct DescribeStackSetInput {
    /// <p>The name or unique ID of the stack set whose description you want.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackSetInput` contents to a `SignedRequest`.
struct DescribeStackSetInputSerializer;
impl DescribeStackSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackSetOperationInput {
    /// <p>The unique ID of the stack set operation. </p>
    pub operation_id: String,
    /// <p>The name or the unique stack ID of the stack set for the stack operation.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackSetOperationInput` contents to a `SignedRequest`.
struct DescribeStackSetOperationInputSerializer;
impl DescribeStackSetOperationInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackSetOperationInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "OperationId"),
            &obj.operation_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackSetOperationOutput {
    /// <p>The specified stack set operation.</p>
    pub stack_set_operation: Option<StackSetOperation>,
}

struct DescribeStackSetOperationOutputDeserializer;
impl DescribeStackSetOperationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackSetOperationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackSetOperationOutput::default();

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
                    "StackSetOperation" => {
                        obj.stack_set_operation = Some(try!(
                            StackSetOperationDeserializer::deserialize("StackSetOperation", stack)
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
pub struct DescribeStackSetOutput {
    /// <p>The specified stack set.</p>
    pub stack_set: Option<StackSet>,
}

struct DescribeStackSetOutputDeserializer;
impl DescribeStackSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeStackSetOutput::default();

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
                    "StackSet" => {
                        obj.stack_set =
                            Some(try!(StackSetDeserializer::deserialize("StackSet", stack)));
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
/// <p>The input for <a>DescribeStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStacksInput {
    /// <p>A string that identifies the next page of stacks that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for a <a>DescribeStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStacksOutput {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of stack structures.</p>
    pub stacks: Option<Vec<Stack>>,
}

struct DescribeStacksOutputDeserializer;
impl DescribeStacksOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStacksOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Stacks" => {
                        obj.stacks = Some(try!(StacksDeserializer::deserialize("Stacks", stack)));
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
struct DisableRollbackDeserializer;
impl DisableRollbackDeserializer {
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
struct EnableTerminationProtectionDeserializer;
impl EnableTerminationProtectionDeserializer {
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
/// <p>The input for an <a>EstimateTemplateCost</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EstimateTemplateCostInput {
    /// <p>A list of <code>Parameter</code> structures that specify input parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must pass <code>TemplateBody</code> or <code>TemplateURL</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
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
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for a <a>EstimateTemplateCost</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EstimateTemplateCostOutput {
    /// <p>An AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    pub url: Option<String>,
}

struct EstimateTemplateCostOutputDeserializer;
impl EstimateTemplateCostOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EstimateTemplateCostOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Url" => {
                        obj.url = Some(try!(UrlDeserializer::deserialize("Url", stack)));
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
struct EvaluationTypeDeserializer;
impl EvaluationTypeDeserializer {
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
struct EventIdDeserializer;
impl EventIdDeserializer {
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
/// <p>The input for the <a>ExecuteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExecuteChangeSetInput {
    /// <p>The name or ARN of the change set that you want use to update the specified stack.</p>
    pub change_set_name: String,
    /// <p>A unique identifier for this <code>ExecuteChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to execute a change set to update a stack with the same name. You might retry <code>ExecuteChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p>If you specified the name of a change set, specify the stack name or ID (ARN) that is associated with the change set you want to execute.</p>
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

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.client_request_token {
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>ExecuteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExecuteChangeSetOutput {}

struct ExecuteChangeSetOutputDeserializer;
impl ExecuteChangeSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExecuteChangeSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = ExecuteChangeSetOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct ExecutionStatusDeserializer;
impl ExecutionStatusDeserializer {
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
/// <p>The <code>Export</code> structure describes the exported output values for a stack.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Export {
    /// <p>The stack that contains the exported output name and value.</p>
    pub exporting_stack_id: Option<String>,
    /// <p>The name of exported output value. Use this name and the <code>Fn::ImportValue</code> function to import the associated value into other stacks. The name is defined in the <code>Export</code> field in the associated stack's <code>Outputs</code> section.</p>
    pub name: Option<String>,
    /// <p>The value of the exported output, such as a resource physical ID. This value is defined in the <code>Export</code> field in the associated stack's <code>Outputs</code> section.</p>
    pub value: Option<String>,
}

struct ExportDeserializer;
impl ExportDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Export, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "ExportingStackId" => {
                        obj.exporting_stack_id = Some(try!(StackIdDeserializer::deserialize(
                            "ExportingStackId",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = Some(try!(ExportNameDeserializer::deserialize("Name", stack)));
                    }
                    "Value" => {
                        obj.value =
                            Some(try!(ExportValueDeserializer::deserialize("Value", stack)));
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
struct ExportNameDeserializer;
impl ExportNameDeserializer {
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
struct ExportValueDeserializer;
impl ExportValueDeserializer {
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
struct ExportsDeserializer;
impl ExportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Export>, XmlParseError> {
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
struct FailureToleranceCountDeserializer;
impl FailureToleranceCountDeserializer {
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
struct FailureTolerancePercentageDeserializer;
impl FailureTolerancePercentageDeserializer {
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
/// <p>The input for the <a>GetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStackPolicyInput {
    /// <p>The name or unique stack ID that is associated with the stack whose policy you want to get.</p>
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

        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

/// <p>The output for the <a>GetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStackPolicyOutput {
    /// <p>Structure containing the stack policy body. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide.)</p>
    pub stack_policy_body: Option<String>,
}

struct GetStackPolicyOutputDeserializer;
impl GetStackPolicyOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStackPolicyOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StackPolicyBody" => {
                        obj.stack_policy_body = Some(try!(
                            StackPolicyBodyDeserializer::deserialize("StackPolicyBody", stack)
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
/// <p>The input for a <a>GetTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateInput {
    /// <p>The name or Amazon Resource Name (ARN) of a change set for which AWS CloudFormation returns the associated template. If you specify a name, you must also specify the <code>StackName</code>.</p>
    pub change_set_name: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: Option<String>,
    /// <p>For templates that include transforms, the stage of the template that AWS CloudFormation returns. To get the user-submitted template, specify <code>Original</code>. To get the template after AWS CloudFormation has processed all transforms, specify <code>Processed</code>. </p> <p>If the template doesn't include transforms, <code>Original</code> and <code>Processed</code> return the same template. By default, AWS CloudFormation specifies <code>Original</code>. </p>
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
            params.put(
                &format!("{}{}", prefix, "ChangeSetName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_stage {
            params.put(
                &format!("{}{}", prefix, "TemplateStage"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for <a>GetTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateOutput {
    /// <p>The stage of the template that you can retrieve. For stacks, the <code>Original</code> and <code>Processed</code> templates are always available. For change sets, the <code>Original</code> template is always available. After AWS CloudFormation finishes creating the change set, the <code>Processed</code> template becomes available.</p>
    pub stages_available: Option<Vec<String>>,
    /// <p>Structure containing the template body. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>AWS CloudFormation returns the same template that was used when the stack was created.</p>
    pub template_body: Option<String>,
}

struct GetTemplateOutputDeserializer;
impl GetTemplateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StagesAvailable" => {
                        obj.stages_available = Some(try!(StageListDeserializer::deserialize(
                            "StagesAvailable",
                            stack
                        )));
                    }
                    "TemplateBody" => {
                        obj.template_body = Some(try!(TemplateBodyDeserializer::deserialize(
                            "TemplateBody",
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
/// <p>The input for the <a>GetTemplateSummary</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateSummaryInput {
    /// <p>The name or the stack ID that is associated with the stack, which are not always interchangeable. For running stacks, you can specify either the stack's name or its unique stack ID. For deleted stack, you must specify the unique stack ID.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub stack_name: Option<String>,
    /// <p>The name or unique ID of the stack set from which the stack was created.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub stack_set_name: Option<String>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information about templates, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information about templates, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
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
            params.put(
                &format!("{}{}", prefix, "StackName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_set_name {
            params.put(
                &format!("{}{}", prefix, "StackSetName"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for the <a>GetTemplateSummary</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateSummaryOutput {
    /// <p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The list of resources that generated the values in the <code>Capabilities</code> response element.</p>
    pub capabilities_reason: Option<String>,
    /// <p>A list of the transforms that are declared in the template.</p>
    pub declared_transforms: Option<Vec<String>>,
    /// <p>The value that is defined in the <code>Description</code> property of the template.</p>
    pub description: Option<String>,
    /// <p>The value that is defined for the <code>Metadata</code> property of the template.</p>
    pub metadata: Option<String>,
    /// <p>A list of parameter declarations that describe various properties for each parameter.</p>
    pub parameters: Option<Vec<ParameterDeclaration>>,
    /// <p>A list of all the template resource types that are defined in the template, such as <code>AWS::EC2::Instance</code>, <code>AWS::Dynamo::Table</code>, and <code>Custom::MyCustomInstance</code>.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>The AWS template format version, which identifies the capabilities of the template.</p>
    pub version: Option<String>,
}

struct GetTemplateSummaryOutputDeserializer;
impl GetTemplateSummaryOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateSummaryOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Capabilities" => {
                        obj.capabilities = Some(try!(CapabilitiesDeserializer::deserialize(
                            "Capabilities",
                            stack
                        )));
                    }
                    "CapabilitiesReason" => {
                        obj.capabilities_reason =
                            Some(try!(CapabilitiesReasonDeserializer::deserialize(
                                "CapabilitiesReason",
                                stack
                            )));
                    }
                    "DeclaredTransforms" => {
                        obj.declared_transforms = Some(try!(
                            TransformsListDeserializer::deserialize("DeclaredTransforms", stack)
                        ));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "Metadata" => {
                        obj.metadata =
                            Some(try!(MetadataDeserializer::deserialize("Metadata", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(
                            ParameterDeclarationsDeserializer::deserialize("Parameters", stack)
                        ));
                    }
                    "ResourceTypes" => {
                        obj.resource_types = Some(try!(ResourceTypesDeserializer::deserialize(
                            "ResourceTypes",
                            stack
                        )));
                    }
                    "Version" => {
                        obj.version =
                            Some(try!(VersionDeserializer::deserialize("Version", stack)));
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
struct ImportsDeserializer;
impl ImportsDeserializer {
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
struct LimitNameDeserializer;
impl LimitNameDeserializer {
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
struct LimitValueDeserializer;
impl LimitValueDeserializer {
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
/// <p>The input for the <a>ListChangeSets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListChangeSetsInput {
    /// <p>A string (provided by the <a>ListChangeSets</a> response output) that identifies the next page of change sets that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the Amazon Resource Name (ARN) of the stack for which you want to list change sets.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

/// <p>The output for the <a>ListChangeSets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListChangeSetsOutput {
    /// <p>If the output exceeds 1 MB, a string that identifies the next page of change sets. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>ChangeSetSummary</code> structures that provides the ID and status of each change set for the specified stack.</p>
    pub summaries: Option<Vec<ChangeSetSummary>>,
}

struct ListChangeSetsOutputDeserializer;
impl ListChangeSetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListChangeSetsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Summaries" => {
                        obj.summaries = Some(try!(ChangeSetSummariesDeserializer::deserialize(
                            "Summaries",
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
pub struct ListExportsInput {
    /// <p>A string (provided by the <a>ListExports</a> response output) that identifies the next page of exported output values that you asked to retrieve.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListExportsOutput {
    /// <p>The output for the <a>ListExports</a> action.</p>
    pub exports: Option<Vec<Export>>,
    /// <p>If the output exceeds 100 exported output values, a string that identifies the next page of exports. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
}

struct ListExportsOutputDeserializer;
impl ListExportsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListExportsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Exports" => {
                        obj.exports =
                            Some(try!(ExportsDeserializer::deserialize("Exports", stack)));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
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
pub struct ListImportsInput {
    /// <p>The name of the exported output value. AWS CloudFormation returns the stack names that are importing this value. </p>
    pub export_name: String,
    /// <p>A string (provided by the <a>ListImports</a> response output) that identifies the next page of stacks that are importing the specified exported output value. </p>
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

        params.put(
            &format!("{}{}", prefix, "ExportName"),
            &obj.export_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListImportsOutput {
    /// <p>A list of stack names that are importing the specified exported output value. </p>
    pub imports: Option<Vec<String>>,
    /// <p>A string that identifies the next page of exports. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
}

struct ListImportsOutputDeserializer;
impl ListImportsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListImportsOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Imports" => {
                        obj.imports =
                            Some(try!(ImportsDeserializer::deserialize("Imports", stack)));
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
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
pub struct ListStackInstancesInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous request didn't return all of the remaining results, the response's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The name of the AWS account that you want to list stack instances for.</p>
    pub stack_instance_account: Option<String>,
    /// <p>The name of the region where you want to list stack instances. </p>
    pub stack_instance_region: Option<String>,
    /// <p>The name or unique ID of the stack set that you want to list stack instances for.</p>
    pub stack_set_name: String,
}

/// Serialize `ListStackInstancesInput` contents to a `SignedRequest`.
struct ListStackInstancesInputSerializer;
impl ListStackInstancesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackInstancesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(
                &format!("{}{}", prefix, "MaxResults"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_instance_account {
            params.put(
                &format!("{}{}", prefix, "StackInstanceAccount"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_instance_region {
            params.put(
                &format!("{}{}", prefix, "StackInstanceRegion"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackInstancesOutput {
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackInstanceSummary</code> structures that contain information about the specified stack instances.</p>
    pub summaries: Option<Vec<StackInstanceSummary>>,
}

struct ListStackInstancesOutputDeserializer;
impl ListStackInstancesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackInstancesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStackInstancesOutput::default();

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
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Summaries" => {
                        obj.summaries = Some(try!(
                            StackInstanceSummariesDeserializer::deserialize("Summaries", stack)
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
/// <p>The input for the <a>ListStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackResourcesInput {
    /// <p>A string that identifies the next page of stack resources that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

/// <p>The output for a <a>ListStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackResourcesOutput {
    /// <p>If the output exceeds 1 MB, a string that identifies the next page of stack resources. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackResourceSummary</code> structures.</p>
    pub stack_resource_summaries: Option<Vec<StackResourceSummary>>,
}

struct ListStackResourcesOutputDeserializer;
impl ListStackResourcesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackResourcesOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "StackResourceSummaries" => {
                        obj.stack_resource_summaries =
                            Some(try!(StackResourceSummariesDeserializer::deserialize(
                                "StackResourceSummaries",
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
pub struct ListStackSetOperationResultsInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSetOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The ID of the stack set operation.</p>
    pub operation_id: String,
    /// <p>The name or unique ID of the stack set that you want to get operation results for.</p>
    pub stack_set_name: String,
}

/// Serialize `ListStackSetOperationResultsInput` contents to a `SignedRequest`.
struct ListStackSetOperationResultsInputSerializer;
impl ListStackSetOperationResultsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetOperationResultsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(
                &format!("{}{}", prefix, "MaxResults"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "OperationId"),
            &obj.operation_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetOperationResultsOutput {
    /// <p>If the request doesn't return all results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetOperationResultSummary</code> structures that contain information about the specified operation results, for accounts and regions that are included in the operation.</p>
    pub summaries: Option<Vec<StackSetOperationResultSummary>>,
}

struct ListStackSetOperationResultsOutputDeserializer;
impl ListStackSetOperationResultsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetOperationResultsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStackSetOperationResultsOutput::default();

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
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Summaries" => {
                        obj.summaries = Some(try!(
                            StackSetOperationResultSummariesDeserializer::deserialize(
                                "Summaries",
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
pub struct ListStackSetOperationsInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSetOperations</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The name or unique ID of the stack set that you want to get operation summaries for.</p>
    pub stack_set_name: String,
}

/// Serialize `ListStackSetOperationsInput` contents to a `SignedRequest`.
struct ListStackSetOperationsInputSerializer;
impl ListStackSetOperationsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetOperationsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(
                &format!("{}{}", prefix, "MaxResults"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetOperationsOutput {
    /// <p>If the request doesn't return all results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetOperationSummary</code> structures that contain summary information about operations for the specified stack set.</p>
    pub summaries: Option<Vec<StackSetOperationSummary>>,
}

struct ListStackSetOperationsOutputDeserializer;
impl ListStackSetOperationsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetOperationsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStackSetOperationsOutput::default();

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
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Summaries" => {
                        obj.summaries = Some(try!(
                            StackSetOperationSummariesDeserializer::deserialize("Summaries", stack)
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
pub struct ListStackSetsInput {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSets</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The status of the stack sets that you want to get summary information about.</p>
    pub status: Option<String>,
}

/// Serialize `ListStackSetsInput` contents to a `SignedRequest`.
struct ListStackSetsInputSerializer;
impl ListStackSetsInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetsInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(
                &format!("{}{}", prefix, "MaxResults"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(
                &format!("{}{}", prefix, "NextToken"),
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetsOutput {
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetSummary</code> structures that contain information about the user's stack sets.</p>
    pub summaries: Option<Vec<StackSetSummary>>,
}

struct ListStackSetsOutputDeserializer;
impl ListStackSetsOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ListStackSetsOutput::default();

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
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "Summaries" => {
                        obj.summaries = Some(try!(StackSetSummariesDeserializer::deserialize(
                            "Summaries",
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
/// <p>The input for <a>ListStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStacksInput {
    /// <p>A string that identifies the next page of stacks that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>Stack status to use as a filter. Specify one or more stack status codes to list only stacks with the specified status codes. For a complete list of stack status codes, see the <code>StackStatus</code> parameter of the <a>Stack</a> data type.</p>
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
            params.put(
                &format!("{}{}", prefix, "NextToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_status_filter {
            StackStatusFilterSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StackStatusFilter"),
                field_value,
            );
        }
    }
}

/// <p>The output for <a>ListStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStacksOutput {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSummary</code> structures containing information about the specified stacks.</p>
    pub stack_summaries: Option<Vec<StackSummary>>,
}

struct ListStacksOutputDeserializer;
impl ListStacksOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStacksOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "NextToken" => {
                        obj.next_token =
                            Some(try!(NextTokenDeserializer::deserialize("NextToken", stack)));
                    }
                    "StackSummaries" => {
                        obj.stack_summaries = Some(try!(StackSummariesDeserializer::deserialize(
                            "StackSummaries",
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
struct LogicalResourceIdDeserializer;
impl LogicalResourceIdDeserializer {
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
struct MaxConcurrentCountDeserializer;
impl MaxConcurrentCountDeserializer {
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
struct MaxConcurrentPercentageDeserializer;
impl MaxConcurrentPercentageDeserializer {
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
struct MetadataDeserializer;
impl MetadataDeserializer {
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
struct MonitoringTimeInMinutesDeserializer;
impl MonitoringTimeInMinutesDeserializer {
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
struct NoEchoDeserializer;
impl NoEchoDeserializer {
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
struct NotificationARNDeserializer;
impl NotificationARNDeserializer {
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
struct NotificationARNsDeserializer;
impl NotificationARNsDeserializer {
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
                        obj.push(try!(NotificationARNDeserializer::deserialize(
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

/// <p>The Output data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Output {
    /// <p>User defined description associated with the output.</p>
    pub description: Option<String>,
    /// <p>The name of the export associated with the output.</p>
    pub export_name: Option<String>,
    /// <p>The key associated with the output.</p>
    pub output_key: Option<String>,
    /// <p>The value associated with the output.</p>
    pub output_value: Option<String>,
}

struct OutputDeserializer;
impl OutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Output, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "ExportName" => {
                        obj.export_name = Some(try!(ExportNameDeserializer::deserialize(
                            "ExportName",
                            stack
                        )));
                    }
                    "OutputKey" => {
                        obj.output_key =
                            Some(try!(OutputKeyDeserializer::deserialize("OutputKey", stack)));
                    }
                    "OutputValue" => {
                        obj.output_value = Some(try!(OutputValueDeserializer::deserialize(
                            "OutputValue",
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
struct OutputKeyDeserializer;
impl OutputKeyDeserializer {
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
struct OutputValueDeserializer;
impl OutputValueDeserializer {
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
struct OutputsDeserializer;
impl OutputsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Output>, XmlParseError> {
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
/// <p>The Parameter data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parameter {
    /// <p>The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation uses the default value that is specified in your template.</p>
    pub parameter_key: Option<String>,
    /// <p>The input value associated with the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>Read-only. The value that corresponds to a Systems Manager parameter key. This field is returned only for <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/parameters-section-structure.html#aws-ssm-parameter-types"> <code>SSM</code> parameter types</a> in the template.</p>
    pub resolved_value: Option<String>,
    /// <p>During a stack update, use the existing parameter value that the stack is using for a given parameter key. If you specify <code>true</code>, do not specify a parameter value.</p>
    pub use_previous_value: Option<bool>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Parameter, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "ParameterKey" => {
                        obj.parameter_key = Some(try!(ParameterKeyDeserializer::deserialize(
                            "ParameterKey",
                            stack
                        )));
                    }
                    "ParameterValue" => {
                        obj.parameter_value = Some(try!(ParameterValueDeserializer::deserialize(
                            "ParameterValue",
                            stack
                        )));
                    }
                    "ResolvedValue" => {
                        obj.resolved_value = Some(try!(ParameterValueDeserializer::deserialize(
                            "ResolvedValue",
                            stack
                        )));
                    }
                    "UsePreviousValue" => {
                        obj.use_previous_value = Some(try!(
                            UsePreviousValueDeserializer::deserialize("UsePreviousValue", stack)
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

/// Serialize `Parameter` contents to a `SignedRequest`.
struct ParameterSerializer;
impl ParameterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Parameter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.parameter_key {
            params.put(
                &format!("{}{}", prefix, "ParameterKey"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(
                &format!("{}{}", prefix, "ParameterValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.resolved_value {
            params.put(
                &format!("{}{}", prefix, "ResolvedValue"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.use_previous_value {
            params.put(
                &format!("{}{}", prefix, "UsePreviousValue"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

/// <p>A set of criteria that AWS CloudFormation uses to validate parameter values. Although other constraints might be defined in the stack template, AWS CloudFormation returns only the <code>AllowedValues</code> property.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParameterConstraints {
    /// <p>A list of values that are permitted for a parameter.</p>
    pub allowed_values: Option<Vec<String>>,
}

struct ParameterConstraintsDeserializer;
impl ParameterConstraintsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParameterConstraints, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "AllowedValues" => {
                        obj.allowed_values = Some(try!(AllowedValuesDeserializer::deserialize(
                            "AllowedValues",
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
/// <p>The ParameterDeclaration data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    /// <p>The default value of the parameter.</p>
    pub default_value: Option<String>,
    /// <p>The description that is associate with the parameter.</p>
    pub description: Option<String>,
    /// <p>Flag that indicates whether the parameter value is shown as plain text in logs and in the AWS Management Console.</p>
    pub no_echo: Option<bool>,
    /// <p>The criteria that AWS CloudFormation uses to validate parameter values.</p>
    pub parameter_constraints: Option<ParameterConstraints>,
    /// <p>The name that is associated with the parameter.</p>
    pub parameter_key: Option<String>,
    /// <p>The type of parameter.</p>
    pub parameter_type: Option<String>,
}

struct ParameterDeclarationDeserializer;
impl ParameterDeclarationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParameterDeclaration, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(ParameterValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "NoEcho" => {
                        obj.no_echo = Some(try!(NoEchoDeserializer::deserialize("NoEcho", stack)));
                    }
                    "ParameterConstraints" => {
                        obj.parameter_constraints =
                            Some(try!(ParameterConstraintsDeserializer::deserialize(
                                "ParameterConstraints",
                                stack
                            )));
                    }
                    "ParameterKey" => {
                        obj.parameter_key = Some(try!(ParameterKeyDeserializer::deserialize(
                            "ParameterKey",
                            stack
                        )));
                    }
                    "ParameterType" => {
                        obj.parameter_type = Some(try!(ParameterTypeDeserializer::deserialize(
                            "ParameterType",
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
struct ParameterDeclarationsDeserializer;
impl ParameterDeclarationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ParameterDeclaration>, XmlParseError> {
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
                        obj.push(try!(ParameterDeclarationDeserializer::deserialize(
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
struct ParameterKeyDeserializer;
impl ParameterKeyDeserializer {
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
struct ParameterTypeDeserializer;
impl ParameterTypeDeserializer {
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
struct ParameterValueDeserializer;
impl ParameterValueDeserializer {
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
struct ParametersDeserializer;
impl ParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Parameter>, XmlParseError> {
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
struct PropertyNameDeserializer;
impl PropertyNameDeserializer {
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
struct ReasonDeserializer;
impl ReasonDeserializer {
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
struct RegionDeserializer;
impl RegionDeserializer {
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
struct RegionListDeserializer;
impl RegionListDeserializer {
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
                        obj.push(try!(RegionDeserializer::deserialize("member", stack)));
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

/// Serialize `RegionList` contents to a `SignedRequest`.
struct RegionListSerializer;
impl RegionListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ReplacementDeserializer;
impl ReplacementDeserializer {
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
struct RequiresRecreationDeserializer;
impl RequiresRecreationDeserializer {
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
struct ResourceAttributeDeserializer;
impl ResourceAttributeDeserializer {
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
/// <p>The <code>ResourceChange</code> structure describes the resource and the action that AWS CloudFormation will perform on it if you execute this change set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceChange {
    /// <p>The action that AWS CloudFormation takes on the resource, such as <code>Add</code> (adds a new resource), <code>Modify</code> (changes a resource), or <code>Remove</code> (deletes a resource).</p>
    pub action: Option<String>,
    /// <p>For the <code>Modify</code> action, a list of <code>ResourceChangeDetail</code> structures that describes the changes that AWS CloudFormation will make to the resource. </p>
    pub details: Option<Vec<ResourceChangeDetail>>,
    /// <p>The resource's logical ID, which is defined in the stack's template.</p>
    pub logical_resource_id: Option<String>,
    /// <p>The resource's physical ID (resource name). Resources that you are adding don't have physical IDs because they haven't been created.</p>
    pub physical_resource_id: Option<String>,
    /// <p>For the <code>Modify</code> action, indicates whether AWS CloudFormation will replace the resource by creating a new one and deleting the old one. This value depends on the value of the <code>RequiresRecreation</code> property in the <code>ResourceTargetDefinition</code> structure. For example, if the <code>RequiresRecreation</code> field is <code>Always</code> and the <code>Evaluation</code> field is <code>Static</code>, <code>Replacement</code> is <code>True</code>. If the <code>RequiresRecreation</code> field is <code>Always</code> and the <code>Evaluation</code> field is <code>Dynamic</code>, <code>Replacement</code> is <code>Conditionally</code>.</p> <p>If you have multiple changes with different <code>RequiresRecreation</code> values, the <code>Replacement</code> value depends on the change with the most impact. A <code>RequiresRecreation</code> value of <code>Always</code> has the most impact, followed by <code>Conditionally</code>, and then <code>Never</code>.</p>
    pub replacement: Option<String>,
    /// <p>The type of AWS CloudFormation resource, such as <code>AWS::S3::Bucket</code>.</p>
    pub resource_type: Option<String>,
    /// <p>For the <code>Modify</code> action, indicates which resource attribute is triggering this update, such as a change in the resource attribute's <code>Metadata</code>, <code>Properties</code>, or <code>Tags</code>.</p>
    pub scope: Option<Vec<String>>,
}

struct ResourceChangeDeserializer;
impl ResourceChangeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceChange, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Action" => {
                        obj.action =
                            Some(try!(ChangeActionDeserializer::deserialize("Action", stack)));
                    }
                    "Details" => {
                        obj.details = Some(try!(ResourceChangeDetailsDeserializer::deserialize(
                            "Details", stack
                        )));
                    }
                    "LogicalResourceId" => {
                        obj.logical_resource_id = Some(try!(
                            LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)
                        ));
                    }
                    "PhysicalResourceId" => {
                        obj.physical_resource_id =
                            Some(try!(PhysicalResourceIdDeserializer::deserialize(
                                "PhysicalResourceId",
                                stack
                            )));
                    }
                    "Replacement" => {
                        obj.replacement = Some(try!(ReplacementDeserializer::deserialize(
                            "Replacement",
                            stack
                        )));
                    }
                    "ResourceType" => {
                        obj.resource_type = Some(try!(ResourceTypeDeserializer::deserialize(
                            "ResourceType",
                            stack
                        )));
                    }
                    "Scope" => {
                        obj.scope = Some(try!(ScopeDeserializer::deserialize("Scope", stack)));
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
/// <p>For a resource with <code>Modify</code> as the action, the <code>ResourceChange</code> structure describes the changes AWS CloudFormation will make to that resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceChangeDetail {
    /// <p>The identity of the entity that triggered this change. This entity is a member of the group that is specified by the <code>ChangeSource</code> field. For example, if you modified the value of the <code>KeyPairName</code> parameter, the <code>CausingEntity</code> is the name of the parameter (<code>KeyPairName</code>).</p> <p>If the <code>ChangeSource</code> value is <code>DirectModification</code>, no value is given for <code>CausingEntity</code>.</p>
    pub causing_entity: Option<String>,
    /// <p><p>The group to which the <code>CausingEntity</code> value belongs. There are five entity groups:</p> <ul> <li> <p> <code>ResourceReference</code> entities are <code>Ref</code> intrinsic functions that refer to resources in the template, such as <code>{ &quot;Ref&quot; : &quot;MyEC2InstanceResource&quot; }</code>.</p> </li> <li> <p> <code>ParameterReference</code> entities are <code>Ref</code> intrinsic functions that get template parameter values, such as <code>{ &quot;Ref&quot; : &quot;MyPasswordParameter&quot; }</code>.</p> </li> <li> <p> <code>ResourceAttribute</code> entities are <code>Fn::GetAtt</code> intrinsic functions that get resource attribute values, such as <code>{ &quot;Fn::GetAtt&quot; : [ &quot;MyEC2InstanceResource&quot;, &quot;PublicDnsName&quot; ] }</code>.</p> </li> <li> <p> <code>DirectModification</code> entities are changes that are made directly to the template.</p> </li> <li> <p> <code>Automatic</code> entities are <code>AWS::CloudFormation::Stack</code> resource types, which are also known as nested stacks. If you made no changes to the <code>AWS::CloudFormation::Stack</code> resource, AWS CloudFormation sets the <code>ChangeSource</code> to <code>Automatic</code> because the nested stack&#39;s template might have changed. Changes to a nested stack&#39;s template aren&#39;t visible to AWS CloudFormation until you run an update on the parent stack.</p> </li> </ul></p>
    pub change_source: Option<String>,
    /// <p>Indicates whether AWS CloudFormation can determine the target value, and whether the target value will change before you execute a change set.</p> <p>For <code>Static</code> evaluations, AWS CloudFormation can determine that the target value will change, and its value. For example, if you directly modify the <code>InstanceType</code> property of an EC2 instance, AWS CloudFormation knows that this property value will change, and its value, so this is a <code>Static</code> evaluation.</p> <p>For <code>Dynamic</code> evaluations, cannot determine the target value because it depends on the result of an intrinsic function, such as a <code>Ref</code> or <code>Fn::GetAtt</code> intrinsic function, when the stack is updated. For example, if your template includes a reference to a resource that is conditionally recreated, the value of the reference (the physical ID of the resource) might change, depending on if the resource is recreated. If the resource is recreated, it will have a new physical ID, so all references to that resource will also be updated.</p>
    pub evaluation: Option<String>,
    /// <p>A <code>ResourceTargetDefinition</code> structure that describes the field that AWS CloudFormation will change and whether the resource will be recreated.</p>
    pub target: Option<ResourceTargetDefinition>,
}

struct ResourceChangeDetailDeserializer;
impl ResourceChangeDetailDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceChangeDetail, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "CausingEntity" => {
                        obj.causing_entity = Some(try!(CausingEntityDeserializer::deserialize(
                            "CausingEntity",
                            stack
                        )));
                    }
                    "ChangeSource" => {
                        obj.change_source = Some(try!(ChangeSourceDeserializer::deserialize(
                            "ChangeSource",
                            stack
                        )));
                    }
                    "Evaluation" => {
                        obj.evaluation = Some(try!(EvaluationTypeDeserializer::deserialize(
                            "Evaluation",
                            stack
                        )));
                    }
                    "Target" => {
                        obj.target = Some(try!(ResourceTargetDefinitionDeserializer::deserialize(
                            "Target", stack
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
struct ResourceChangeDetailsDeserializer;
impl ResourceChangeDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceChangeDetail>, XmlParseError> {
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
                        obj.push(try!(ResourceChangeDetailDeserializer::deserialize(
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
struct ResourcePropertiesDeserializer;
impl ResourcePropertiesDeserializer {
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
struct ResourceStatusDeserializer;
impl ResourceStatusDeserializer {
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
struct ResourceStatusReasonDeserializer;
impl ResourceStatusReasonDeserializer {
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
/// <p>The field that AWS CloudFormation will change, such as the name of a resource's property, and whether the resource will be recreated.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceTargetDefinition {
    /// <p>Indicates which resource attribute is triggering this update, such as a change in the resource attribute's <code>Metadata</code>, <code>Properties</code>, or <code>Tags</code>.</p>
    pub attribute: Option<String>,
    /// <p>If the <code>Attribute</code> value is <code>Properties</code>, the name of the property. For all other attributes, the value is null.</p>
    pub name: Option<String>,
    /// <p>If the <code>Attribute</code> value is <code>Properties</code>, indicates whether a change to this property causes the resource to be recreated. The value can be <code>Never</code>, <code>Always</code>, or <code>Conditionally</code>. To determine the conditions for a <code>Conditionally</code> recreation, see the update behavior for that <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">property</a> in the AWS CloudFormation User Guide.</p>
    pub requires_recreation: Option<String>,
}

struct ResourceTargetDefinitionDeserializer;
impl ResourceTargetDefinitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceTargetDefinition, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Attribute" => {
                        obj.attribute = Some(try!(ResourceAttributeDeserializer::deserialize(
                            "Attribute",
                            stack
                        )));
                    }
                    "Name" => {
                        obj.name = Some(try!(PropertyNameDeserializer::deserialize("Name", stack)));
                    }
                    "RequiresRecreation" => {
                        obj.requires_recreation =
                            Some(try!(RequiresRecreationDeserializer::deserialize(
                                "RequiresRecreation",
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
struct ResourceTypeDeserializer;
impl ResourceTypeDeserializer {
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
struct ResourceTypesDeserializer;
impl ResourceTypesDeserializer {
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

struct RetainStacksNullableDeserializer;
impl RetainStacksNullableDeserializer {
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
struct RoleARNDeserializer;
impl RoleARNDeserializer {
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
/// <p>Structure containing the rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p> <p>Rollback triggers enable you to have AWS CloudFormation monitor the state of your application during stack creation and updating, and to roll back that operation if the application breaches the threshold of any of the alarms you've specified. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-rollback-triggers.html">Monitor and Roll Back Stack Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RollbackConfiguration {
    /// <p>The amount of time, in minutes, during which CloudFormation should monitor all the rollback triggers after the stack creation or update operation deploys all necessary resources.</p> <p>The default is 0 minutes.</p> <p>If you specify a monitoring period but do not specify any rollback triggers, CloudFormation still waits the specified period of time before cleaning up old resources after update operations. You can use this monitoring period to perform any manual stack validation desired, and manually cancel the stack creation or update (using <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CancelUpdateStack.html">CancelUpdateStack</a>, for example) as necessary.</p> <p>If you specify 0 for this parameter, CloudFormation still monitors the specified rollback triggers during stack creation and update operations. Then, for update operations, it begins disposing of old resources immediately once the operation completes.</p>
    pub monitoring_time_in_minutes: Option<i64>,
    /// <p>The triggers to monitor during stack creation or update actions. </p> <p>By default, AWS CloudFormation saves the rollback triggers specified for a stack and applies them to any subsequent update operations for the stack, unless you specify otherwise. If you do specify rollback triggers for this parameter, those triggers replace any list of triggers previously specified for the stack. This means:</p> <ul> <li> <p>To use the rollback triggers previously specified for this stack, if any, don't specify this parameter.</p> </li> <li> <p>To specify new or updated rollback triggers, you must specify <i>all</i> the triggers that you want used for this stack, even triggers you've specifed before (for example, when creating the stack or during a previous stack update). Any triggers that you don't include in the updated list of triggers are no longer applied to the stack.</p> </li> <li> <p>To remove all currently specified triggers, specify an empty list for this parameter.</p> </li> </ul> <p>If a specified trigger is missing, the entire stack operation fails and is rolled back. </p>
    pub rollback_triggers: Option<Vec<RollbackTrigger>>,
}

struct RollbackConfigurationDeserializer;
impl RollbackConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RollbackConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RollbackConfiguration::default();

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
                    "MonitoringTimeInMinutes" => {
                        obj.monitoring_time_in_minutes =
                            Some(try!(MonitoringTimeInMinutesDeserializer::deserialize(
                                "MonitoringTimeInMinutes",
                                stack
                            )));
                    }
                    "RollbackTriggers" => {
                        obj.rollback_triggers = Some(try!(
                            RollbackTriggersDeserializer::deserialize("RollbackTriggers", stack)
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

/// Serialize `RollbackConfiguration` contents to a `SignedRequest`.
struct RollbackConfigurationSerializer;
impl RollbackConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RollbackConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.monitoring_time_in_minutes {
            params.put(
                &format!("{}{}", prefix, "MonitoringTimeInMinutes"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.rollback_triggers {
            RollbackTriggersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackTriggers"),
                field_value,
            );
        }
    }
}

/// <p>A rollback trigger AWS CloudFormation monitors during creation and updating of stacks. If any of the alarms you specify goes to ALARM state during the stack operation or within the specified monitoring period afterwards, CloudFormation rolls back the entire stack operation. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RollbackTrigger {
    /// <p>The Amazon Resource Name (ARN) of the rollback trigger.</p> <p>If a specified trigger is missing, the entire stack operation fails and is rolled back. </p>
    pub arn: String,
    /// <p>The resource type of the rollback trigger. Currently, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> is the only supported resource type.</p>
    pub type_: String,
}

struct RollbackTriggerDeserializer;
impl RollbackTriggerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RollbackTrigger, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RollbackTrigger::default();

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
                    "Arn" => {
                        obj.arn = try!(ArnDeserializer::deserialize("Arn", stack));
                    }
                    "Type" => {
                        obj.type_ = try!(TypeDeserializer::deserialize("Type", stack));
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

/// Serialize `RollbackTrigger` contents to a `SignedRequest`.
struct RollbackTriggerSerializer;
impl RollbackTriggerSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RollbackTrigger) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "Arn"),
            &obj.arn.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Type"),
            &obj.type_.replace("+", "%2B"),
        );
    }
}

struct RollbackTriggersDeserializer;
impl RollbackTriggersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RollbackTrigger>, XmlParseError> {
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
                        obj.push(try!(RollbackTriggerDeserializer::deserialize(
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

/// Serialize `RollbackTriggers` contents to a `SignedRequest`.
struct RollbackTriggersSerializer;
impl RollbackTriggersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RollbackTrigger>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RollbackTriggerSerializer::serialize(params, &key, obj);
        }
    }
}

struct ScopeDeserializer;
impl ScopeDeserializer {
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
                        obj.push(try!(ResourceAttributeDeserializer::deserialize(
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
/// <p>The input for the <a>SetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetStackPolicyInput {
    /// <p>The name or unique stack ID that you want to associate a policy with.</p>
    pub stack_name: String,
    /// <p>Structure containing the stack policy body. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_body: Option<String>,
    /// <p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
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

        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(
                &format!("{}{}", prefix, "StackPolicyBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(
                &format!("{}{}", prefix, "StackPolicyURL"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The input for the <a>SignalResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SignalResourceInput {
    /// <p>The logical ID of the resource that you want to signal. The logical ID is the name of the resource that given in the template.</p>
    pub logical_resource_id: String,
    /// <p>The stack name or unique stack ID that includes the resource that you want to signal.</p>
    pub stack_name: String,
    /// <p>The status of the signal, which is either success or failure. A failure signal causes AWS CloudFormation to immediately fail the stack creation or update.</p>
    pub status: String,
    /// <p>A unique ID of the signal. When you signal Amazon EC2 instances or Auto Scaling groups, specify the instance ID that you are signaling as the unique ID. If you send multiple signals to a single resource (such as signaling a wait condition), each signal requires a different unique ID.</p>
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

        params.put(
            &format!("{}{}", prefix, "LogicalResourceId"),
            &obj.logical_resource_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Status"),
            &obj.status.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "UniqueId"),
            &obj.unique_id.replace("+", "%2B"),
        );
    }
}

/// <p>The Stack data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stack {
    /// <p>The capabilities allowed in the stack.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The unique ID of the change set.</p>
    pub change_set_id: Option<String>,
    /// <p>The time at which the stack was created.</p>
    pub creation_time: String,
    /// <p>The time the stack was deleted.</p>
    pub deletion_time: Option<String>,
    /// <p>A user-defined description associated with the stack.</p>
    pub description: Option<String>,
    /// <p><p>Boolean to enable or disable rollback on stack creation failures:</p> <ul> <li> <p> <code>true</code>: disable rollback</p> </li> <li> <p> <code>false</code>: enable rollback</p> </li> </ul></p>
    pub disable_rollback: Option<bool>,
    /// <p>Whether termination protection is enabled for the stack.</p> <p> For <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub enable_termination_protection: Option<bool>,
    /// <p>The time the stack was last updated. This field will only be returned if the stack has been updated at least once.</p>
    pub last_updated_time: Option<String>,
    /// <p>SNS topic ARNs to which stack related events are published.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of output structures.</p>
    pub outputs: Option<Vec<Output>>,
    /// <p>A list of <code>Parameter</code> structures.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>For nested stacks--stacks created as resources for another stack--the stack ID of the direct parent of this stack. For the first level of nested stacks, the root stack is also the parent stack.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">Working with Nested Stacks</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub parent_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that is associated with the stack. During a stack operation, AWS CloudFormation uses this role's credentials to make calls on your behalf.</p>
    pub role_arn: Option<String>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>For nested stacks--stacks created as resources for another stack--the stack ID of the the top-level stack to which the nested stack ultimately belongs.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">Working with Nested Stacks</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub root_id: Option<String>,
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
    /// <p>The name associated with the stack.</p>
    pub stack_name: String,
    /// <p>Current status of the stack.</p>
    pub stack_status: String,
    /// <p>Success/failure message associated with the stack status.</p>
    pub stack_status_reason: Option<String>,
    /// <p>A list of <code>Tag</code>s that specify information about the stack.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The amount of time within which stack creation should complete.</p>
    pub timeout_in_minutes: Option<i64>,
}

struct StackDeserializer;
impl StackDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Stack, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Capabilities" => {
                        obj.capabilities = Some(try!(CapabilitiesDeserializer::deserialize(
                            "Capabilities",
                            stack
                        )));
                    }
                    "ChangeSetId" => {
                        obj.change_set_id = Some(try!(ChangeSetIdDeserializer::deserialize(
                            "ChangeSetId",
                            stack
                        )));
                    }
                    "CreationTime" => {
                        obj.creation_time =
                            try!(CreationTimeDeserializer::deserialize("CreationTime", stack));
                    }
                    "DeletionTime" => {
                        obj.deletion_time = Some(try!(DeletionTimeDeserializer::deserialize(
                            "DeletionTime",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "DisableRollback" => {
                        obj.disable_rollback = Some(try!(
                            DisableRollbackDeserializer::deserialize("DisableRollback", stack)
                        ));
                    }
                    "EnableTerminationProtection" => {
                        obj.enable_termination_protection =
                            Some(try!(EnableTerminationProtectionDeserializer::deserialize(
                                "EnableTerminationProtection",
                                stack
                            )));
                    }
                    "LastUpdatedTime" => {
                        obj.last_updated_time = Some(try!(
                            LastUpdatedTimeDeserializer::deserialize("LastUpdatedTime", stack)
                        ));
                    }
                    "NotificationARNs" => {
                        obj.notification_ar_ns = Some(try!(
                            NotificationARNsDeserializer::deserialize("NotificationARNs", stack)
                        ));
                    }
                    "Outputs" => {
                        obj.outputs =
                            Some(try!(OutputsDeserializer::deserialize("Outputs", stack)));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    "ParentId" => {
                        obj.parent_id =
                            Some(try!(StackIdDeserializer::deserialize("ParentId", stack)));
                    }
                    "RoleARN" => {
                        obj.role_arn =
                            Some(try!(RoleARNDeserializer::deserialize("RoleARN", stack)));
                    }
                    "RollbackConfiguration" => {
                        obj.rollback_configuration =
                            Some(try!(RollbackConfigurationDeserializer::deserialize(
                                "RollbackConfiguration",
                                stack
                            )));
                    }
                    "RootId" => {
                        obj.root_id = Some(try!(StackIdDeserializer::deserialize("RootId", stack)));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            try!(StackNameDeserializer::deserialize("StackName", stack));
                    }
                    "StackStatus" => {
                        obj.stack_status =
                            try!(StackStatusDeserializer::deserialize("StackStatus", stack));
                    }
                    "StackStatusReason" => {
                        obj.stack_status_reason = Some(try!(
                            StackStatusReasonDeserializer::deserialize("StackStatusReason", stack)
                        ));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagsDeserializer::deserialize("Tags", stack)));
                    }
                    "TimeoutInMinutes" => {
                        obj.timeout_in_minutes = Some(try!(
                            TimeoutMinutesDeserializer::deserialize("TimeoutInMinutes", stack)
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
/// <p>The StackEvent data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackEvent {
    /// <p>The token passed to the operation that generated this event.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>The unique ID of this event.</p>
    pub event_id: String,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: Option<String>,
    /// <p>The name or unique identifier associated with the physical instance of the resource.</p>
    pub physical_resource_id: Option<String>,
    /// <p>BLOB of the properties used to create the resource.</p>
    pub resource_properties: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: Option<String>,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: Option<String>,
    /// <p>The unique ID name of the instance of the stack.</p>
    pub stack_id: String,
    /// <p>The name associated with a stack.</p>
    pub stack_name: String,
    /// <p>Time the status was updated.</p>
    pub timestamp: String,
}

struct StackEventDeserializer;
impl StackEventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackEvent, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "ClientRequestToken" => {
                        obj.client_request_token =
                            Some(try!(ClientRequestTokenDeserializer::deserialize(
                                "ClientRequestToken",
                                stack
                            )));
                    }
                    "EventId" => {
                        obj.event_id = try!(EventIdDeserializer::deserialize("EventId", stack));
                    }
                    "LogicalResourceId" => {
                        obj.logical_resource_id = Some(try!(
                            LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)
                        ));
                    }
                    "PhysicalResourceId" => {
                        obj.physical_resource_id =
                            Some(try!(PhysicalResourceIdDeserializer::deserialize(
                                "PhysicalResourceId",
                                stack
                            )));
                    }
                    "ResourceProperties" => {
                        obj.resource_properties =
                            Some(try!(ResourcePropertiesDeserializer::deserialize(
                                "ResourceProperties",
                                stack
                            )));
                    }
                    "ResourceStatus" => {
                        obj.resource_status = Some(try!(ResourceStatusDeserializer::deserialize(
                            "ResourceStatus",
                            stack
                        )));
                    }
                    "ResourceStatusReason" => {
                        obj.resource_status_reason =
                            Some(try!(ResourceStatusReasonDeserializer::deserialize(
                                "ResourceStatusReason",
                                stack
                            )));
                    }
                    "ResourceType" => {
                        obj.resource_type = Some(try!(ResourceTypeDeserializer::deserialize(
                            "ResourceType",
                            stack
                        )));
                    }
                    "StackId" => {
                        obj.stack_id = try!(StackIdDeserializer::deserialize("StackId", stack));
                    }
                    "StackName" => {
                        obj.stack_name =
                            try!(StackNameDeserializer::deserialize("StackName", stack));
                    }
                    "Timestamp" => {
                        obj.timestamp =
                            try!(TimestampDeserializer::deserialize("Timestamp", stack));
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
struct StackEventsDeserializer;
impl StackEventsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackEvent>, XmlParseError> {
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
/// <p>An AWS CloudFormation stack, in a specific account and region, that's part of a stack set operation. A stack instance is a reference to an attempted or actual stack in a given account within a given region. A stack instance can exist without a stack—for example, if the stack couldn't be created for some reason. A stack instance is associated with only one stack set. Each stack instance contains the ID of its associated stack set, as well as the ID of the actual stack and the stack status.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackInstance {
    /// <p>The name of the AWS account that the stack instance is associated with.</p>
    pub account: Option<String>,
    /// <p>A list of parameters from the stack set template whose values have been overridden in this stack instance.</p>
    pub parameter_overrides: Option<Vec<Parameter>>,
    /// <p>The name of the AWS region that the stack instance is associated with.</p>
    pub region: Option<String>,
    /// <p>The ID of the stack instance.</p>
    pub stack_id: Option<String>,
    /// <p>The name or unique ID of the stack set that the stack instance is associated with.</p>
    pub stack_set_id: Option<String>,
    /// <p><p>The status of the stack instance, in terms of its synchronization with its associated stack set.</p> <ul> <li> <p> <code>INOPERABLE</code>: A <code>DeleteStackInstances</code> operation has failed and left the stack in an unstable state. Stacks in this state are excluded from further <code>UpdateStackSet</code> operations. You might need to perform a <code>DeleteStackInstances</code> operation, with <code>RetainStacks</code> set to <code>true</code>, to delete the stack instance, and then delete the stack manually.</p> </li> <li> <p> <code>OUTDATED</code>: The stack isn&#39;t currently up to date with the stack set because:</p> <ul> <li> <p>The associated stack failed during a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation. </p> </li> <li> <p>The stack was part of a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation that failed or was stopped before the stack was created or updated. </p> </li> </ul> </li> <li> <p> <code>CURRENT</code>: The stack is currently up to date with the stack set.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The explanation for the specific status code that is assigned to this stack instance.</p>
    pub status_reason: Option<String>,
}

struct StackInstanceDeserializer;
impl StackInstanceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackInstance, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackInstance::default();

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
                    "Account" => {
                        obj.account =
                            Some(try!(AccountDeserializer::deserialize("Account", stack)));
                    }
                    "ParameterOverrides" => {
                        obj.parameter_overrides = Some(try!(ParametersDeserializer::deserialize(
                            "ParameterOverrides",
                            stack
                        )));
                    }
                    "Region" => {
                        obj.region = Some(try!(RegionDeserializer::deserialize("Region", stack)));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StackInstanceStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "StatusReason" => {
                        obj.status_reason =
                            Some(try!(ReasonDeserializer::deserialize("StatusReason", stack)));
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
struct StackInstanceStatusDeserializer;
impl StackInstanceStatusDeserializer {
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
struct StackInstanceSummariesDeserializer;
impl StackInstanceSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackInstanceSummary>, XmlParseError> {
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
                        obj.push(try!(StackInstanceSummaryDeserializer::deserialize(
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
/// <p>The structure that contains summary information about a stack instance.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackInstanceSummary {
    /// <p>The name of the AWS account that the stack instance is associated with.</p>
    pub account: Option<String>,
    /// <p>The name of the AWS region that the stack instance is associated with.</p>
    pub region: Option<String>,
    /// <p>The ID of the stack instance.</p>
    pub stack_id: Option<String>,
    /// <p>The name or unique ID of the stack set that the stack instance is associated with.</p>
    pub stack_set_id: Option<String>,
    /// <p><p>The status of the stack instance, in terms of its synchronization with its associated stack set.</p> <ul> <li> <p> <code>INOPERABLE</code>: A <code>DeleteStackInstances</code> operation has failed and left the stack in an unstable state. Stacks in this state are excluded from further <code>UpdateStackSet</code> operations. You might need to perform a <code>DeleteStackInstances</code> operation, with <code>RetainStacks</code> set to <code>true</code>, to delete the stack instance, and then delete the stack manually.</p> </li> <li> <p> <code>OUTDATED</code>: The stack isn&#39;t currently up to date with the stack set because:</p> <ul> <li> <p>The associated stack failed during a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation. </p> </li> <li> <p>The stack was part of a <code>CreateStackSet</code> or <code>UpdateStackSet</code> operation that failed or was stopped before the stack was created or updated. </p> </li> </ul> </li> <li> <p> <code>CURRENT</code>: The stack is currently up to date with the stack set.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The explanation for the specific status code assigned to this stack instance.</p>
    pub status_reason: Option<String>,
}

struct StackInstanceSummaryDeserializer;
impl StackInstanceSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackInstanceSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackInstanceSummary::default();

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
                    "Account" => {
                        obj.account =
                            Some(try!(AccountDeserializer::deserialize("Account", stack)));
                    }
                    "Region" => {
                        obj.region = Some(try!(RegionDeserializer::deserialize("Region", stack)));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StackInstanceStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "StatusReason" => {
                        obj.status_reason =
                            Some(try!(ReasonDeserializer::deserialize("StatusReason", stack)));
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
struct StackNameDeserializer;
impl StackNameDeserializer {
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
struct StackPolicyBodyDeserializer;
impl StackPolicyBodyDeserializer {
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
/// <p>The StackResource data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResource {
    /// <p>User defined description associated with the resource.</p>
    pub description: Option<String>,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>
    pub physical_resource_id: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: String,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: String,
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
    /// <p>The name associated with the stack.</p>
    pub stack_name: Option<String>,
    /// <p>Time the status was updated.</p>
    pub timestamp: String,
}

struct StackResourceDeserializer;
impl StackResourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResource, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "LogicalResourceId" => {
                        obj.logical_resource_id = try!(LogicalResourceIdDeserializer::deserialize(
                            "LogicalResourceId",
                            stack
                        ));
                    }
                    "PhysicalResourceId" => {
                        obj.physical_resource_id =
                            Some(try!(PhysicalResourceIdDeserializer::deserialize(
                                "PhysicalResourceId",
                                stack
                            )));
                    }
                    "ResourceStatus" => {
                        obj.resource_status = try!(ResourceStatusDeserializer::deserialize(
                            "ResourceStatus",
                            stack
                        ));
                    }
                    "ResourceStatusReason" => {
                        obj.resource_status_reason =
                            Some(try!(ResourceStatusReasonDeserializer::deserialize(
                                "ResourceStatusReason",
                                stack
                            )));
                    }
                    "ResourceType" => {
                        obj.resource_type =
                            try!(ResourceTypeDeserializer::deserialize("ResourceType", stack));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            Some(try!(StackNameDeserializer::deserialize("StackName", stack)));
                    }
                    "Timestamp" => {
                        obj.timestamp =
                            try!(TimestampDeserializer::deserialize("Timestamp", stack));
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
/// <p>Contains detailed information about the specified stack resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceDetail {
    /// <p>User defined description associated with the resource.</p>
    pub description: Option<String>,
    /// <p>Time the status was updated.</p>
    pub last_updated_timestamp: String,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The content of the <code>Metadata</code> attribute declared for the resource. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-attribute-metadata.html">Metadata Attribute</a> in the AWS CloudFormation User Guide.</p>
    pub metadata: Option<String>,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>
    pub physical_resource_id: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: String,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. ((For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: String,
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
    /// <p>The name associated with the stack.</p>
    pub stack_name: Option<String>,
}

struct StackResourceDetailDeserializer;
impl StackResourceDetailDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceDetail, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "LastUpdatedTimestamp" => {
                        obj.last_updated_timestamp = try!(TimestampDeserializer::deserialize(
                            "LastUpdatedTimestamp",
                            stack
                        ));
                    }
                    "LogicalResourceId" => {
                        obj.logical_resource_id = try!(LogicalResourceIdDeserializer::deserialize(
                            "LogicalResourceId",
                            stack
                        ));
                    }
                    "Metadata" => {
                        obj.metadata =
                            Some(try!(MetadataDeserializer::deserialize("Metadata", stack)));
                    }
                    "PhysicalResourceId" => {
                        obj.physical_resource_id =
                            Some(try!(PhysicalResourceIdDeserializer::deserialize(
                                "PhysicalResourceId",
                                stack
                            )));
                    }
                    "ResourceStatus" => {
                        obj.resource_status = try!(ResourceStatusDeserializer::deserialize(
                            "ResourceStatus",
                            stack
                        ));
                    }
                    "ResourceStatusReason" => {
                        obj.resource_status_reason =
                            Some(try!(ResourceStatusReasonDeserializer::deserialize(
                                "ResourceStatusReason",
                                stack
                            )));
                    }
                    "ResourceType" => {
                        obj.resource_type =
                            try!(ResourceTypeDeserializer::deserialize("ResourceType", stack));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            Some(try!(StackNameDeserializer::deserialize("StackName", stack)));
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
struct StackResourceSummariesDeserializer;
impl StackResourceSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackResourceSummary>, XmlParseError> {
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
                        obj.push(try!(StackResourceSummaryDeserializer::deserialize(
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
/// <p>Contains high-level information about the specified stack resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceSummary {
    /// <p>Time the status was updated.</p>
    pub last_updated_timestamp: String,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of the resource.</p>
    pub physical_resource_id: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: String,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: String,
}

struct StackResourceSummaryDeserializer;
impl StackResourceSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceSummary, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "LastUpdatedTimestamp" => {
                        obj.last_updated_timestamp = try!(TimestampDeserializer::deserialize(
                            "LastUpdatedTimestamp",
                            stack
                        ));
                    }
                    "LogicalResourceId" => {
                        obj.logical_resource_id = try!(LogicalResourceIdDeserializer::deserialize(
                            "LogicalResourceId",
                            stack
                        ));
                    }
                    "PhysicalResourceId" => {
                        obj.physical_resource_id =
                            Some(try!(PhysicalResourceIdDeserializer::deserialize(
                                "PhysicalResourceId",
                                stack
                            )));
                    }
                    "ResourceStatus" => {
                        obj.resource_status = try!(ResourceStatusDeserializer::deserialize(
                            "ResourceStatus",
                            stack
                        ));
                    }
                    "ResourceStatusReason" => {
                        obj.resource_status_reason =
                            Some(try!(ResourceStatusReasonDeserializer::deserialize(
                                "ResourceStatusReason",
                                stack
                            )));
                    }
                    "ResourceType" => {
                        obj.resource_type =
                            try!(ResourceTypeDeserializer::deserialize("ResourceType", stack));
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
struct StackResourcesDeserializer;
impl StackResourcesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackResource>, XmlParseError> {
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
                        obj.push(try!(StackResourceDeserializer::deserialize(
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
/// <p>A structure that contains information about a stack set. A stack set enables you to provision stacks into AWS accounts and across regions by using a single CloudFormation template. In the stack set, you specify the template to use, as well as any parameters and capabilities that the template requires. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSet {
    /// <p>The Amazon Resource Number (ARN) of the IAM role used to create or update the stack set.</p> <p>Use customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Define Permissions for Multiple Administrators</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub administration_role_arn: Option<String>,
    /// <p>The capabilities that are allowed in the stack set. Some stack set templates might include resources that can affect permissions in your AWS account—for example, by creating new AWS Identity and Access Management (IAM) users. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates.</a> </p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A description of the stack set that you specify when the stack set is created or updated.</p>
    pub description: Option<String>,
    /// <p>A list of input parameters for a stack set.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The Amazon Resource Number (ARN) of the stack set.</p>
    pub stack_set_arn: Option<String>,
    /// <p>The ID of the stack set.</p>
    pub stack_set_id: Option<String>,
    /// <p>The name that's associated with the stack set.</p>
    pub stack_set_name: Option<String>,
    /// <p>The status of the stack set.</p>
    pub status: Option<String>,
    /// <p>A list of tags that specify information about the stack set. A maximum number of 50 tags can be specified.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The structure that contains the body of the template that was used to create or update the stack set.</p>
    pub template_body: Option<String>,
}

struct StackSetDeserializer;
impl StackSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSet, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSet::default();

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
                    "AdministrationRoleARN" => {
                        obj.administration_role_arn = Some(try!(RoleARNDeserializer::deserialize(
                            "AdministrationRoleARN",
                            stack
                        )));
                    }
                    "Capabilities" => {
                        obj.capabilities = Some(try!(CapabilitiesDeserializer::deserialize(
                            "Capabilities",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(ParametersDeserializer::deserialize(
                            "Parameters",
                            stack
                        )));
                    }
                    "StackSetARN" => {
                        obj.stack_set_arn = Some(try!(StackSetARNDeserializer::deserialize(
                            "StackSetARN",
                            stack
                        )));
                    }
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
                            stack
                        )));
                    }
                    "StackSetName" => {
                        obj.stack_set_name = Some(try!(StackSetNameDeserializer::deserialize(
                            "StackSetName",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StackSetStatusDeserializer::deserialize(
                            "Status", stack
                        )));
                    }
                    "Tags" => {
                        obj.tags = Some(try!(TagsDeserializer::deserialize("Tags", stack)));
                    }
                    "TemplateBody" => {
                        obj.template_body = Some(try!(TemplateBodyDeserializer::deserialize(
                            "TemplateBody",
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
struct StackSetARNDeserializer;
impl StackSetARNDeserializer {
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
struct StackSetIdDeserializer;
impl StackSetIdDeserializer {
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
struct StackSetNameDeserializer;
impl StackSetNameDeserializer {
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
/// <p>The structure that contains information about a stack set operation. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSetOperation {
    /// <p>The type of stack set operation: <code>CREATE</code>, <code>UPDATE</code>, or <code>DELETE</code>. Create and delete operations affect only the specified stack set instances that are associated with the specified stack set. Update operations affect both the stack set itself, as well as <i>all</i> associated stack set instances.</p>
    pub action: Option<String>,
    /// <p>The Amazon Resource Number (ARN) of the IAM role used to perform this stack set operation. </p> <p>Use customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Define Permissions for Multiple Administrators</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub administration_role_arn: Option<String>,
    /// <p>The time at which the operation was initiated. Note that the creation times for the stack set operation might differ from the creation time of the individual stacks themselves. This is because AWS CloudFormation needs to perform preparatory work for the operation, such as dispatching the work to the requested regions, before actually creating the first stacks.</p>
    pub creation_timestamp: Option<String>,
    /// <p>The time at which the stack set operation ended, across all accounts and regions specified. Note that this doesn't necessarily mean that the stack set operation was successful, or even attempted, in each account or region.</p>
    pub end_timestamp: Option<String>,
    /// <p>The unique ID of a stack set operation.</p>
    pub operation_id: Option<String>,
    /// <p>The preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>For stack set operations of action type <code>DELETE</code>, specifies whether to remove the stack instances from the specified stack set, but doesn't delete the stacks. You can't reassociate a retained stack, or add an existing, saved stack to a new stack set.</p>
    pub retain_stacks: Option<bool>,
    /// <p>The ID of the stack set.</p>
    pub stack_set_id: Option<String>,
    /// <p><p>The status of the operation. </p> <ul> <li> <p> <code>FAILED</code>: The operation exceeded the specified failure tolerance. The failure tolerance value that you&#39;ve set for an operation is applied for each region during stack create and update operations. If the number of failed stacks within a region exceeds the failure tolerance, the status of the operation in the region is set to <code>FAILED</code>. This in turn sets the status of the operation as a whole to <code>FAILED</code>, and AWS CloudFormation cancels the operation in any remaining regions.</p> </li> <li> <p> <code>RUNNING</code>: The operation is currently being performed.</p> </li> <li> <p> <code>STOPPED</code>: The user has cancelled the operation.</p> </li> <li> <p> <code>STOPPING</code>: The operation is in the process of stopping, at user request. </p> </li> <li> <p> <code>SUCCEEDED</code>: The operation completed creating or updating all the specified stacks without exceeding the failure tolerance for the operation.</p> </li> </ul></p>
    pub status: Option<String>,
}

struct StackSetOperationDeserializer;
impl StackSetOperationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperation, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSetOperation::default();

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
                    "Action" => {
                        obj.action = Some(try!(StackSetOperationActionDeserializer::deserialize(
                            "Action", stack
                        )));
                    }
                    "AdministrationRoleARN" => {
                        obj.administration_role_arn = Some(try!(RoleARNDeserializer::deserialize(
                            "AdministrationRoleARN",
                            stack
                        )));
                    }
                    "CreationTimestamp" => {
                        obj.creation_timestamp = Some(try!(TimestampDeserializer::deserialize(
                            "CreationTimestamp",
                            stack
                        )));
                    }
                    "EndTimestamp" => {
                        obj.end_timestamp = Some(try!(TimestampDeserializer::deserialize(
                            "EndTimestamp",
                            stack
                        )));
                    }
                    "OperationId" => {
                        obj.operation_id = Some(try!(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
                            stack
                        )));
                    }
                    "OperationPreferences" => {
                        obj.operation_preferences =
                            Some(try!(StackSetOperationPreferencesDeserializer::deserialize(
                                "OperationPreferences",
                                stack
                            )));
                    }
                    "RetainStacks" => {
                        obj.retain_stacks = Some(try!(
                            RetainStacksNullableDeserializer::deserialize("RetainStacks", stack)
                        ));
                    }
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StackSetOperationStatusDeserializer::deserialize(
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
struct StackSetOperationActionDeserializer;
impl StackSetOperationActionDeserializer {
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
/// <p>The user-specified preferences for how AWS CloudFormation performs a stack set operation. </p> <p>For more information on maximum concurrent accounts and failure tolerance, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSetOperationPreferences {
    /// <p>The number of accounts, per region, for which this operation can fail before AWS CloudFormation stops the operation in that region. If the operation is stopped in a region, AWS CloudFormation doesn't attempt the operation in any subsequent regions.</p> <p>Conditional: You must specify either <code>FailureToleranceCount</code> or <code>FailureTolerancePercentage</code> (but not both).</p>
    pub failure_tolerance_count: Option<i64>,
    /// <p>The percentage of accounts, per region, for which this stack operation can fail before AWS CloudFormation stops the operation in that region. If the operation is stopped in a region, AWS CloudFormation doesn't attempt the operation in any subsequent regions.</p> <p>When calculating the number of accounts based on the specified percentage, AWS CloudFormation rounds <i>down</i> to the next whole number.</p> <p>Conditional: You must specify either <code>FailureToleranceCount</code> or <code>FailureTolerancePercentage</code>, but not both.</p>
    pub failure_tolerance_percentage: Option<i64>,
    /// <p>The maximum number of accounts in which to perform this operation at one time. This is dependent on the value of <code>FailureToleranceCount</code>—<code>MaxConcurrentCount</code> is at most one more than the <code>FailureToleranceCount</code> .</p> <p>Note that this setting lets you specify the <i>maximum</i> for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Conditional: You must specify either <code>MaxConcurrentCount</code> or <code>MaxConcurrentPercentage</code>, but not both.</p>
    pub max_concurrent_count: Option<i64>,
    /// <p>The maximum percentage of accounts in which to perform this operation at one time.</p> <p>When calculating the number of accounts based on the specified percentage, AWS CloudFormation rounds down to the next whole number. This is true except in cases where rounding down would result is zero. In this case, CloudFormation sets the number as one instead.</p> <p>Note that this setting lets you specify the <i>maximum</i> for operations. For large deployments, under certain circumstances the actual number of accounts acted upon concurrently may be lower due to service throttling.</p> <p>Conditional: You must specify either <code>MaxConcurrentCount</code> or <code>MaxConcurrentPercentage</code>, but not both.</p>
    pub max_concurrent_percentage: Option<i64>,
    /// <p>The order of the regions in where you want to perform the stack operation.</p>
    pub region_order: Option<Vec<String>>,
}

struct StackSetOperationPreferencesDeserializer;
impl StackSetOperationPreferencesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationPreferences, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSetOperationPreferences::default();

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
                    "FailureToleranceCount" => {
                        obj.failure_tolerance_count =
                            Some(try!(FailureToleranceCountDeserializer::deserialize(
                                "FailureToleranceCount",
                                stack
                            )));
                    }
                    "FailureTolerancePercentage" => {
                        obj.failure_tolerance_percentage =
                            Some(try!(FailureTolerancePercentageDeserializer::deserialize(
                                "FailureTolerancePercentage",
                                stack
                            )));
                    }
                    "MaxConcurrentCount" => {
                        obj.max_concurrent_count =
                            Some(try!(MaxConcurrentCountDeserializer::deserialize(
                                "MaxConcurrentCount",
                                stack
                            )));
                    }
                    "MaxConcurrentPercentage" => {
                        obj.max_concurrent_percentage =
                            Some(try!(MaxConcurrentPercentageDeserializer::deserialize(
                                "MaxConcurrentPercentage",
                                stack
                            )));
                    }
                    "RegionOrder" => {
                        obj.region_order = Some(try!(RegionListDeserializer::deserialize(
                            "RegionOrder",
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

/// Serialize `StackSetOperationPreferences` contents to a `SignedRequest`.
struct StackSetOperationPreferencesSerializer;
impl StackSetOperationPreferencesSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StackSetOperationPreferences) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.failure_tolerance_count {
            params.put(
                &format!("{}{}", prefix, "FailureToleranceCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.failure_tolerance_percentage {
            params.put(
                &format!("{}{}", prefix, "FailureTolerancePercentage"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_concurrent_count {
            params.put(
                &format!("{}{}", prefix, "MaxConcurrentCount"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.max_concurrent_percentage {
            params.put(
                &format!("{}{}", prefix, "MaxConcurrentPercentage"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.region_order {
            RegionListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RegionOrder"),
                field_value,
            );
        }
    }
}

struct StackSetOperationResultStatusDeserializer;
impl StackSetOperationResultStatusDeserializer {
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
struct StackSetOperationResultSummariesDeserializer;
impl StackSetOperationResultSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetOperationResultSummary>, XmlParseError> {
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
                            StackSetOperationResultSummaryDeserializer::deserialize(
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
/// <p>The structure that contains information about a specified operation's results for a given account in a given region.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSetOperationResultSummary {
    /// <p>The name of the AWS account for this operation result.</p>
    pub account: Option<String>,
    /// <p>The results of the account gate function AWS CloudFormation invokes, if present, before proceeding with stack set operations in an account</p>
    pub account_gate_result: Option<AccountGateResult>,
    /// <p>The name of the AWS region for this operation result.</p>
    pub region: Option<String>,
    /// <p><p>The result status of the stack set operation for the given account in the given region.</p> <ul> <li> <p> <code>CANCELLED</code>: The operation in the specified account and region has been cancelled. This is either because a user has stopped the stack set operation, or because the failure tolerance of the stack set operation has been exceeded.</p> </li> <li> <p> <code>FAILED</code>: The operation in the specified account and region failed. </p> <p>If the stack set operation fails in enough accounts within a region, the failure tolerance for the stack set operation as a whole might be exceeded. </p> </li> <li> <p> <code>RUNNING</code>: The operation in the specified account and region is currently in progress.</p> </li> <li> <p> <code>PENDING</code>: The operation in the specified account and region has yet to start. </p> </li> <li> <p> <code>SUCCEEDED</code>: The operation in the specified account and region completed successfully.</p> </li> </ul></p>
    pub status: Option<String>,
    /// <p>The reason for the assigned result status.</p>
    pub status_reason: Option<String>,
}

struct StackSetOperationResultSummaryDeserializer;
impl StackSetOperationResultSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationResultSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSetOperationResultSummary::default();

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
                    "Account" => {
                        obj.account =
                            Some(try!(AccountDeserializer::deserialize("Account", stack)));
                    }
                    "AccountGateResult" => {
                        obj.account_gate_result = Some(try!(
                            AccountGateResultDeserializer::deserialize("AccountGateResult", stack)
                        ));
                    }
                    "Region" => {
                        obj.region = Some(try!(RegionDeserializer::deserialize("Region", stack)));
                    }
                    "Status" => {
                        obj.status = Some(try!(
                            StackSetOperationResultStatusDeserializer::deserialize("Status", stack)
                        ));
                    }
                    "StatusReason" => {
                        obj.status_reason =
                            Some(try!(ReasonDeserializer::deserialize("StatusReason", stack)));
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
struct StackSetOperationStatusDeserializer;
impl StackSetOperationStatusDeserializer {
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
struct StackSetOperationSummariesDeserializer;
impl StackSetOperationSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetOperationSummary>, XmlParseError> {
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
                        obj.push(try!(StackSetOperationSummaryDeserializer::deserialize(
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
/// <p>The structures that contain summary information about the specified operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSetOperationSummary {
    /// <p>The type of operation: <code>CREATE</code>, <code>UPDATE</code>, or <code>DELETE</code>. Create and delete operations affect only the specified stack instances that are associated with the specified stack set. Update operations affect both the stack set itself as well as <i>all</i> associated stack set instances.</p>
    pub action: Option<String>,
    /// <p>The time at which the operation was initiated. Note that the creation times for the stack set operation might differ from the creation time of the individual stacks themselves. This is because AWS CloudFormation needs to perform preparatory work for the operation, such as dispatching the work to the requested regions, before actually creating the first stacks.</p>
    pub creation_timestamp: Option<String>,
    /// <p>The time at which the stack set operation ended, across all accounts and regions specified. Note that this doesn't necessarily mean that the stack set operation was successful, or even attempted, in each account or region.</p>
    pub end_timestamp: Option<String>,
    /// <p>The unique ID of the stack set operation.</p>
    pub operation_id: Option<String>,
    /// <p><p>The overall status of the operation.</p> <ul> <li> <p> <code>FAILED</code>: The operation exceeded the specified failure tolerance. The failure tolerance value that you&#39;ve set for an operation is applied for each region during stack create and update operations. If the number of failed stacks within a region exceeds the failure tolerance, the status of the operation in the region is set to <code>FAILED</code>. This in turn sets the status of the operation as a whole to <code>FAILED</code>, and AWS CloudFormation cancels the operation in any remaining regions.</p> </li> <li> <p> <code>RUNNING</code>: The operation is currently being performed.</p> </li> <li> <p> <code>STOPPED</code>: The user has cancelled the operation.</p> </li> <li> <p> <code>STOPPING</code>: The operation is in the process of stopping, at user request. </p> </li> <li> <p> <code>SUCCEEDED</code>: The operation completed creating or updating all the specified stacks without exceeding the failure tolerance for the operation.</p> </li> </ul></p>
    pub status: Option<String>,
}

struct StackSetOperationSummaryDeserializer;
impl StackSetOperationSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSetOperationSummary::default();

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
                            obj.action = Some(try!(
                                StackSetOperationActionDeserializer::deserialize("Action", stack)
                            ));
                        }
                        "CreationTimestamp" => {
                            obj.creation_timestamp = Some(try!(
                                TimestampDeserializer::deserialize("CreationTimestamp", stack)
                            ));
                        }
                        "EndTimestamp" => {
                            obj.end_timestamp = Some(try!(TimestampDeserializer::deserialize(
                                "EndTimestamp",
                                stack
                            )));
                        }
                        "OperationId" => {
                            obj.operation_id = Some(try!(
                                ClientRequestTokenDeserializer::deserialize("OperationId", stack)
                            ));
                        }
                        "Status" => {
                            obj.status = Some(try!(
                                StackSetOperationStatusDeserializer::deserialize("Status", stack)
                            ));
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
struct StackSetStatusDeserializer;
impl StackSetStatusDeserializer {
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
struct StackSetSummariesDeserializer;
impl StackSetSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetSummary>, XmlParseError> {
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
                        obj.push(try!(StackSetSummaryDeserializer::deserialize(
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
/// <p>The structures that contain summary information about the specified stack set.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSetSummary {
    /// <p>A description of the stack set that you specify when the stack set is created or updated.</p>
    pub description: Option<String>,
    /// <p>The ID of the stack set.</p>
    pub stack_set_id: Option<String>,
    /// <p>The name of the stack set.</p>
    pub stack_set_name: Option<String>,
    /// <p>The status of the stack set.</p>
    pub status: Option<String>,
}

struct StackSetSummaryDeserializer;
impl StackSetSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetSummary, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = StackSetSummary::default();

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
                    "StackSetId" => {
                        obj.stack_set_id = Some(try!(StackSetIdDeserializer::deserialize(
                            "StackSetId",
                            stack
                        )));
                    }
                    "StackSetName" => {
                        obj.stack_set_name = Some(try!(StackSetNameDeserializer::deserialize(
                            "StackSetName",
                            stack
                        )));
                    }
                    "Status" => {
                        obj.status = Some(try!(StackSetStatusDeserializer::deserialize(
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
struct StackStatusDeserializer;
impl StackStatusDeserializer {
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
struct StackSummariesDeserializer;
impl StackSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSummary>, XmlParseError> {
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
/// <p>The StackSummary Data Type</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSummary {
    /// <p>The time the stack was created.</p>
    pub creation_time: String,
    /// <p>The time the stack was deleted.</p>
    pub deletion_time: Option<String>,
    /// <p>The time the stack was last updated. This field will only be returned if the stack has been updated at least once.</p>
    pub last_updated_time: Option<String>,
    /// <p>For nested stacks--stacks created as resources for another stack--the stack ID of the direct parent of this stack. For the first level of nested stacks, the root stack is also the parent stack.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">Working with Nested Stacks</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub parent_id: Option<String>,
    /// <p>For nested stacks--stacks created as resources for another stack--the stack ID of the the top-level stack to which the nested stack ultimately belongs.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">Working with Nested Stacks</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub root_id: Option<String>,
    /// <p>Unique stack identifier.</p>
    pub stack_id: Option<String>,
    /// <p>The name associated with the stack.</p>
    pub stack_name: String,
    /// <p>The current status of the stack.</p>
    pub stack_status: String,
    /// <p>Success/Failure message associated with the stack status.</p>
    pub stack_status_reason: Option<String>,
    /// <p>The template description of the template used to create the stack.</p>
    pub template_description: Option<String>,
}

struct StackSummaryDeserializer;
impl StackSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSummary, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "CreationTime" => {
                        obj.creation_time =
                            try!(CreationTimeDeserializer::deserialize("CreationTime", stack));
                    }
                    "DeletionTime" => {
                        obj.deletion_time = Some(try!(DeletionTimeDeserializer::deserialize(
                            "DeletionTime",
                            stack
                        )));
                    }
                    "LastUpdatedTime" => {
                        obj.last_updated_time = Some(try!(
                            LastUpdatedTimeDeserializer::deserialize("LastUpdatedTime", stack)
                        ));
                    }
                    "ParentId" => {
                        obj.parent_id =
                            Some(try!(StackIdDeserializer::deserialize("ParentId", stack)));
                    }
                    "RootId" => {
                        obj.root_id = Some(try!(StackIdDeserializer::deserialize("RootId", stack)));
                    }
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
                    }
                    "StackName" => {
                        obj.stack_name =
                            try!(StackNameDeserializer::deserialize("StackName", stack));
                    }
                    "StackStatus" => {
                        obj.stack_status =
                            try!(StackStatusDeserializer::deserialize("StackStatus", stack));
                    }
                    "StackStatusReason" => {
                        obj.stack_status_reason = Some(try!(
                            StackStatusReasonDeserializer::deserialize("StackStatusReason", stack)
                        ));
                    }
                    "TemplateDescription" => {
                        obj.template_description =
                            Some(try!(TemplateDescriptionDeserializer::deserialize(
                                "TemplateDescription",
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
struct StacksDeserializer;
impl StacksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Stack>, XmlParseError> {
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
                        obj.push(try!(TemplateStageDeserializer::deserialize(
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StopStackSetOperationInput {
    /// <p>The ID of the stack operation. </p>
    pub operation_id: String,
    /// <p>The name or unique ID of the stack set that you want to stop the operation for.</p>
    pub stack_set_name: String,
}

/// Serialize `StopStackSetOperationInput` contents to a `SignedRequest`.
struct StopStackSetOperationInputSerializer;
impl StopStackSetOperationInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StopStackSetOperationInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "OperationId"),
            &obj.operation_id.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StopStackSetOperationOutput {}

struct StopStackSetOperationOutputDeserializer;
impl StopStackSetOperationOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StopStackSetOperationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let obj = StopStackSetOperationOutput::default();

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
/// <p>The Tag type enables you to specify a key-value pair that can be used to store information about an AWS CloudFormation stack.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p> <i>Required</i>. A string used to identify this tag. You can specify a maximum of 128 characters for a tag key. Tags owned by Amazon Web Services (AWS) have the reserved prefix: <code>aws:</code>.</p>
    pub key: String,
    /// <p> <i>Required</i>. A string containing the value for this tag. You can specify a maximum of 256 characters for a tag value.</p>
    pub value: String,
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
                        obj.key = try!(TagKeyDeserializer::deserialize("Key", stack));
                    }
                    "Value" => {
                        obj.value = try!(TagValueDeserializer::deserialize("Value", stack));
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

        params.put(
            &format!("{}{}", prefix, "Key"),
            &obj.key.replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "Value"),
            &obj.value.replace("+", "%2B"),
        );
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
struct TagsDeserializer;
impl TagsDeserializer {
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
struct TemplateDescriptionDeserializer;
impl TemplateDescriptionDeserializer {
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
/// <p>The TemplateParameter data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemplateParameter {
    /// <p>The default value associated with the parameter.</p>
    pub default_value: Option<String>,
    /// <p>User defined description associated with the parameter.</p>
    pub description: Option<String>,
    /// <p>Flag indicating whether the parameter should be displayed as plain text in logs and UIs.</p>
    pub no_echo: Option<bool>,
    /// <p>The name associated with the parameter.</p>
    pub parameter_key: Option<String>,
}

struct TemplateParameterDeserializer;
impl TemplateParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TemplateParameter, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "DefaultValue" => {
                        obj.default_value = Some(try!(ParameterValueDeserializer::deserialize(
                            "DefaultValue",
                            stack
                        )));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "NoEcho" => {
                        obj.no_echo = Some(try!(NoEchoDeserializer::deserialize("NoEcho", stack)));
                    }
                    "ParameterKey" => {
                        obj.parameter_key = Some(try!(ParameterKeyDeserializer::deserialize(
                            "ParameterKey",
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
struct TemplateParametersDeserializer;
impl TemplateParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TemplateParameter>, XmlParseError> {
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
                        obj.push(try!(TemplateParameterDeserializer::deserialize(
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
struct TemplateStageDeserializer;
impl TemplateStageDeserializer {
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
struct TimeoutMinutesDeserializer;
impl TimeoutMinutesDeserializer {
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
struct TransformNameDeserializer;
impl TransformNameDeserializer {
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
struct TransformsListDeserializer;
impl TransformsListDeserializer {
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
                        obj.push(try!(TransformNameDeserializer::deserialize(
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
struct TypeDeserializer;
impl TypeDeserializer {
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
/// <p>The input for an <a>UpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackInput {
    /// <p>A list of values that you must specify before AWS CloudFormation can update certain stacks. Some stack templates might include resources that can affect permissions in your AWS account, for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code>. The following resources require you to specify this parameter: <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a>, <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a>, and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a>. If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify <code>CAPABILITY_NAMED_IAM</code>. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A unique identifier for this <code>UpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to update a stack with the same name. You might retry <code>UpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>Amazon Simple Notification Service topic Amazon Resource Names (ARNs) that AWS CloudFormation associates with the stack. Specify an empty list to remove all notification topics.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with for this update stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a>.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to update the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p>The name or unique stack ID of the stack to update.</p>
    pub stack_name: String,
    /// <p>Structure containing a new stack policy body. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p> <p>You might update the stack policy, for example, in order to protect a new resource that you created during a stack update. If you do not specify a stack policy, the current policy that is associated with the stack is unchanged.</p>
    pub stack_policy_body: Option<String>,
    /// <p>Structure containing the temporary overriding stack policy body. You can specify either the <code>StackPolicyDuringUpdateBody</code> or the <code>StackPolicyDuringUpdateURL</code> parameter, but not both.</p> <p>If you want to update protected resources, specify a temporary overriding stack policy during this update. If you do not specify a stack policy, the current policy that is associated with the stack will be used.</p>
    pub stack_policy_during_update_body: Option<String>,
    /// <p>Location of a file containing the temporary overriding stack policy. The URL must point to a policy (max size: 16KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyDuringUpdateBody</code> or the <code>StackPolicyDuringUpdateURL</code> parameter, but not both.</p> <p>If you want to update protected resources, specify a temporary overriding stack policy during this update. If you do not specify a stack policy, the current policy that is associated with the stack will be used.</p>
    pub stack_policy_during_update_url: Option<String>,
    /// <p>Location of a file containing the updated stack policy. The URL must point to a policy (max size: 16KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p> <p>You might update the stack policy, for example, in order to protect a new resource that you created during a stack update. If you do not specify a stack policy, the current policy that is associated with the stack is unchanged.</p>
    pub stack_policy_url: Option<String>,
    /// <p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to supported resources in the stack. You can specify a maximum number of 50 tags.</p> <p>If you don't specify this parameter, AWS CloudFormation doesn't modify the stack's tags. If you specify an empty value, AWS CloudFormation removes all associated tags.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
    pub template_url: Option<String>,
    /// <p>Reuse the existing template that is associated with the stack that you are updating.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
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
            CapabilitiesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Capabilities"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.client_request_token {
            params.put(
                &format!("{}{}", prefix, "ClientRequestToken"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.notification_ar_ns {
            NotificationARNsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NotificationARNs"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.resource_types {
            ResourceTypesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourceTypes"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(
                &format!("{}{}", prefix, "RoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(
                &format!("{}{}", prefix, "StackPolicyBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_body {
            params.put(
                &format!("{}{}", prefix, "StackPolicyDuringUpdateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_url {
            params.put(
                &format!("{}{}", prefix, "StackPolicyDuringUpdateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.stack_policy_url {
            params.put(
                &format!("{}{}", prefix, "StackPolicyURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.use_previous_template {
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackInstancesInput {
    /// <p>The names of one or more AWS accounts for which you want to update parameter values for stack instances. The overridden parameter values will be applied to all stack instances in the specified accounts and regions.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p> A list of input parameters whose values you want to update for the specified stack instances. </p> <p>Any overridden parameter values will be applied to all stack instances in the specified accounts and regions. When specifying parameters and their values, be aware of how AWS CloudFormation sets parameter values during stack instance update operations:</p> <ul> <li> <p>To override the current value for a parameter, include the parameter and specify its value.</p> </li> <li> <p>To leave a parameter set to its present value, you can do one of the following:</p> <ul> <li> <p>Do not include the parameter in the list.</p> </li> <li> <p>Include the parameter and specify <code>UsePreviousValue</code> as <code>true</code>. (You cannot specify both a value and set <code>UsePreviousValue</code> to <code>true</code>.)</p> </li> </ul> </li> <li> <p>To set all overridden parameter back to the values specified in the stack set, specify a parameter list but do not include any parameters.</p> </li> <li> <p>To leave all parameters set to their present values, do not specify this property at all.</p> </li> </ul> <p>During stack set updates, any parameter values overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only override the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <code>UpdateStackSet</code> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    pub parameter_overrides: Option<Vec<Parameter>>,
    /// <p>The names of one or more regions in which you want to update parameter values for stack instances. The overridden parameter values will be applied to all stack instances in the specified accounts and regions.</p>
    pub regions: Vec<String>,
    /// <p>The name or unique ID of the stack set associated with the stack instances.</p>
    pub stack_set_name: String,
}

/// Serialize `UpdateStackInstancesInput` contents to a `SignedRequest`.
struct UpdateStackInstancesInputSerializer;
impl UpdateStackInstancesInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackInstancesInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AccountListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "Accounts"),
            &obj.accounts,
        );
        if let Some(ref field_value) = obj.operation_id {
            params.put(
                &format!("{}{}", prefix, "OperationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.operation_preferences {
            StackSetOperationPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OperationPreferences"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.parameter_overrides {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ParameterOverrides"),
                field_value,
            );
        }
        RegionListSerializer::serialize(params, &format!("{}{}", prefix, "Regions"), &obj.regions);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackInstancesOutput {
    /// <p>The unique identifier for this stack set operation. </p>
    pub operation_id: Option<String>,
}

struct UpdateStackInstancesOutputDeserializer;
impl UpdateStackInstancesOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackInstancesOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateStackInstancesOutput::default();

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
                    "OperationId" => {
                        obj.operation_id = Some(try!(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
/// <p>The output for an <a>UpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackOutput {
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
}

struct UpdateStackOutputDeserializer;
impl UpdateStackOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
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
pub struct UpdateStackSetInput {
    /// <p>The Amazon Resource Number (ARN) of the IAM role to use to update this stack set.</p> <p>Specify an IAM role only if you are using customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Define Permissions for Multiple Administrators</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p> If you specify a customized administrator role, AWS CloudFormation uses that role to update the stack. If you do not specify a customized administrator role, AWS CloudFormation performs the update using the role previously associated with the stack set, so long as you have permissions to perform operations on the stack set.</p>
    pub administration_role_arn: Option<String>,
    /// <p>A list of values that you must specify before AWS CloudFormation can create certain stack sets. Some stack set templates might include resources that can affect permissions in your AWS account—for example, by creating new AWS Identity and Access Management (IAM) users. For those stack sets, you must explicitly acknowledge their capabilities by specifying this parameter.</p> <p>The only valid values are CAPABILITY_IAM and CAPABILITY_NAMED_IAM. The following resources require you to specify this parameter: </p> <ul> <li> <p>AWS::IAM::AccessKey</p> </li> <li> <p>AWS::IAM::Group</p> </li> <li> <p>AWS::IAM::InstanceProfile</p> </li> <li> <p>AWS::IAM::Policy</p> </li> <li> <p>AWS::IAM::Role</p> </li> <li> <p>AWS::IAM::User</p> </li> <li> <p>AWS::IAM::UserToGroupAddition</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions that are associated with them and edit their permissions if necessary.</p> <p>If you have IAM resources, you can specify either capability. If you have IAM resources with custom names, you must specify CAPABILITY_NAMED_IAM. If you don't specify this parameter, this action returns an <code>InsufficientCapabilities</code> error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates.</a> </p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A brief description of updates that you are making.</p>
    pub description: Option<String>,
    /// <p>The unique ID for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, AWS CloudFormation generates one automatically.</p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>A list of input parameters for the stack set template. </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The name or unique ID of the stack set that you want to update.</p>
    pub stack_set_name: String,
    /// <p>The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the stacks. You can specify a maximum number of 50 tags.</p> <p>If you specify tags for this parameter, those tags replace any list of tags that are currently associated with this stack set. This means:</p> <ul> <li> <p>If you don't specify this parameter, AWS CloudFormation doesn't modify the stack's tags. </p> </li> <li> <p>If you specify <i>any</i> tags using this parameter, you must specify <i>all</i> the tags that you want associated with this stack set, even tags you've specifed before (for example, when creating the stack set or during a previous update of the stack set.). Any tags that you don't include in the updated list of tags are removed from the stack set, and therefore from the stacks and resources as well. </p> </li> <li> <p>If you specify an empty value, AWS CloudFormation removes all currently associated tags.</p> </li> </ul> <p>If you specify new tags as part of an <code>UpdateStackSet</code> action, AWS CloudFormation checks to see if you have the required IAM permission to tag resources. If you omit tags that are currently associated with the stack set from the list of tags you specify, AWS CloudFormation assumes that you want to remove those tags from the stack set, and checks to see if you have permission to untag resources. If you don't have the necessary permission(s), the entire <code>UpdateStackSet</code> action fails with an <code>access denied</code> error, and the stack set is not updated.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true.</p>
    pub template_body: Option<String>,
    /// <p>The location of the file that contains the template body. The URL must point to a template (maximum size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true. </p>
    pub template_url: Option<String>,
    /// <p>Use the existing template that's associated with the stack set that you're updating.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true. </p>
    pub use_previous_template: Option<bool>,
}

/// Serialize `UpdateStackSetInput` contents to a `SignedRequest`.
struct UpdateStackSetInputSerializer;
impl UpdateStackSetInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackSetInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.administration_role_arn {
            params.put(
                &format!("{}{}", prefix, "AdministrationRoleARN"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.capabilities {
            CapabilitiesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Capabilities"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.description {
            params.put(
                &format!("{}{}", prefix, "Description"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.operation_id {
            params.put(
                &format!("{}{}", prefix, "OperationId"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.operation_preferences {
            StackSetOperationPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OperationPreferences"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.parameters {
            ParametersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Parameters"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name.replace("+", "%2B"),
        );
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.use_previous_template {
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value.to_string().replace("+", "%2B"),
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackSetOutput {
    /// <p>The unique ID for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct UpdateStackSetOutputDeserializer;
impl UpdateStackSetOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackSetOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateStackSetOutput::default();

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
                    "OperationId" => {
                        obj.operation_id = Some(try!(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
pub struct UpdateTerminationProtectionInput {
    /// <p>Whether to enable termination protection on the specified stack.</p>
    pub enable_termination_protection: bool,
    /// <p>The name or unique ID of the stack for which you want to set termination protection.</p>
    pub stack_name: String,
}

/// Serialize `UpdateTerminationProtectionInput` contents to a `SignedRequest`.
struct UpdateTerminationProtectionInputSerializer;
impl UpdateTerminationProtectionInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateTerminationProtectionInput) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "EnableTerminationProtection"),
            &obj.enable_termination_protection
                .to_string()
                .replace("+", "%2B"),
        );
        params.put(
            &format!("{}{}", prefix, "StackName"),
            &obj.stack_name.replace("+", "%2B"),
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTerminationProtectionOutput {
    /// <p>The unique ID of the stack.</p>
    pub stack_id: Option<String>,
}

struct UpdateTerminationProtectionOutputDeserializer;
impl UpdateTerminationProtectionOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTerminationProtectionOutput, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = UpdateTerminationProtectionOutput::default();

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
                    "StackId" => {
                        obj.stack_id =
                            Some(try!(StackIdDeserializer::deserialize("StackId", stack)));
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
struct UrlDeserializer;
impl UrlDeserializer {
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
struct UsePreviousValueDeserializer;
impl UsePreviousValueDeserializer {
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
/// <p>The input for <a>ValidateTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidateTemplateInput {
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
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
            params.put(
                &format!("{}{}", prefix, "TemplateBody"),
                &field_value.replace("+", "%2B"),
            );
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(
                &format!("{}{}", prefix, "TemplateURL"),
                &field_value.replace("+", "%2B"),
            );
        }
    }
}

/// <p>The output for <a>ValidateTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidateTemplateOutput {
    /// <p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The list of resources that generated the values in the <code>Capabilities</code> response element.</p>
    pub capabilities_reason: Option<String>,
    /// <p>A list of the transforms that are declared in the template.</p>
    pub declared_transforms: Option<Vec<String>>,
    /// <p>The description found within the template.</p>
    pub description: Option<String>,
    /// <p>A list of <code>TemplateParameter</code> structures.</p>
    pub parameters: Option<Vec<TemplateParameter>>,
}

struct ValidateTemplateOutputDeserializer;
impl ValidateTemplateOutputDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidateTemplateOutput, XmlParseError> {
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
                DeserializerNext::Element(name) => match &name[..] {
                    "Capabilities" => {
                        obj.capabilities = Some(try!(CapabilitiesDeserializer::deserialize(
                            "Capabilities",
                            stack
                        )));
                    }
                    "CapabilitiesReason" => {
                        obj.capabilities_reason =
                            Some(try!(CapabilitiesReasonDeserializer::deserialize(
                                "CapabilitiesReason",
                                stack
                            )));
                    }
                    "DeclaredTransforms" => {
                        obj.declared_transforms = Some(try!(
                            TransformsListDeserializer::deserialize("DeclaredTransforms", stack)
                        ));
                    }
                    "Description" => {
                        obj.description = Some(try!(DescriptionDeserializer::deserialize(
                            "Description",
                            stack
                        )));
                    }
                    "Parameters" => {
                        obj.parameters = Some(try!(TemplateParametersDeserializer::deserialize(
                            "Parameters",
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
struct VersionDeserializer;
impl VersionDeserializer {
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
/// Errors returned by CancelUpdateStack
#[derive(Debug, PartialEq)]
pub enum CancelUpdateStackError {
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TokenAlreadyExistsException" => {
                    CancelUpdateStackError::TokenAlreadyExists(String::from(parsed_error.message))
                }
                _ => CancelUpdateStackError::Unknown(String::from(body)),
            },
            Err(_) => CancelUpdateStackError::Unknown(body.to_string()),
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
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TokenAlreadyExistsException" => ContinueUpdateRollbackError::TokenAlreadyExists(
                    String::from(parsed_error.message),
                ),
                _ => ContinueUpdateRollbackError::Unknown(String::from(body)),
            },
            Err(_) => ContinueUpdateRollbackError::Unknown(body.to_string()),
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
    /// <p>The resource with the name requested already exists.</p>
    AlreadyExists(String),
    /// <p>The template contains resources with capabilities that weren't specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on stack set limitations, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-limitations.html">Limitations of StackSets</a>.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExistsException" => {
                    CreateChangeSetError::AlreadyExists(String::from(parsed_error.message))
                }
                "InsufficientCapabilitiesException" => {
                    CreateChangeSetError::InsufficientCapabilities(String::from(
                        parsed_error.message,
                    ))
                }
                "LimitExceededException" => {
                    CreateChangeSetError::LimitExceeded(String::from(parsed_error.message))
                }
                _ => CreateChangeSetError::Unknown(String::from(body)),
            },
            Err(_) => CreateChangeSetError::Unknown(body.to_string()),
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
    /// <p>The resource with the name requested already exists.</p>
    AlreadyExists(String),
    /// <p>The template contains resources with capabilities that weren't specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on stack set limitations, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-limitations.html">Limitations of StackSets</a>.</p>
    LimitExceeded(String),
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "AlreadyExistsException" => {
                    CreateStackError::AlreadyExists(String::from(parsed_error.message))
                }
                "InsufficientCapabilitiesException" => {
                    CreateStackError::InsufficientCapabilities(String::from(parsed_error.message))
                }
                "LimitExceededException" => {
                    CreateStackError::LimitExceeded(String::from(parsed_error.message))
                }
                "TokenAlreadyExistsException" => {
                    CreateStackError::TokenAlreadyExists(String::from(parsed_error.message))
                }
                _ => CreateStackError::Unknown(String::from(body)),
            },
            Err(_) => CreateStackError::Unknown(body.to_string()),
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
/// Errors returned by CreateStackInstances
#[derive(Debug, PartialEq)]
pub enum CreateStackInstancesError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on stack set limitations, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-limitations.html">Limitations of StackSets</a>.</p>
    LimitExceeded(String),
    /// <p>The specified operation ID already exists.</p>
    OperationIdAlreadyExists(String),
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStackInstancesError {
    pub fn from_body(body: &str) -> CreateStackInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidOperationException" => {
                    CreateStackInstancesError::InvalidOperation(String::from(parsed_error.message))
                }
                "LimitExceededException" => {
                    CreateStackInstancesError::LimitExceeded(String::from(parsed_error.message))
                }
                "OperationIdAlreadyExistsException" => {
                    CreateStackInstancesError::OperationIdAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressException" => CreateStackInstancesError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "StackSetNotFoundException" => {
                    CreateStackInstancesError::StackSetNotFound(String::from(parsed_error.message))
                }
                "StaleRequestException" => {
                    CreateStackInstancesError::StaleRequest(String::from(parsed_error.message))
                }
                _ => CreateStackInstancesError::Unknown(String::from(body)),
            },
            Err(_) => CreateStackInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateStackInstancesError {
    fn from(err: XmlParseError) -> CreateStackInstancesError {
        let XmlParseError(message) = err;
        CreateStackInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStackInstancesError {
    fn from(err: CredentialsError) -> CreateStackInstancesError {
        CreateStackInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStackInstancesError {
    fn from(err: HttpDispatchError) -> CreateStackInstancesError {
        CreateStackInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStackInstancesError {
    fn from(err: io::Error) -> CreateStackInstancesError {
        CreateStackInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStackInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackInstancesError {
    fn description(&self) -> &str {
        match *self {
            CreateStackInstancesError::InvalidOperation(ref cause) => cause,
            CreateStackInstancesError::LimitExceeded(ref cause) => cause,
            CreateStackInstancesError::OperationIdAlreadyExists(ref cause) => cause,
            CreateStackInstancesError::OperationInProgress(ref cause) => cause,
            CreateStackInstancesError::StackSetNotFound(ref cause) => cause,
            CreateStackInstancesError::StaleRequest(ref cause) => cause,
            CreateStackInstancesError::Validation(ref cause) => cause,
            CreateStackInstancesError::Credentials(ref err) => err.description(),
            CreateStackInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateStackInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStackSet
#[derive(Debug, PartialEq)]
pub enum CreateStackSetError {
    /// <p>The specified resource exists, but has been changed.</p>
    CreatedButModified(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on stack set limitations, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-limitations.html">Limitations of StackSets</a>.</p>
    LimitExceeded(String),
    /// <p>The specified name is already in use.</p>
    NameAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStackSetError {
    pub fn from_body(body: &str) -> CreateStackSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "CreatedButModifiedException" => {
                    CreateStackSetError::CreatedButModified(String::from(parsed_error.message))
                }
                "LimitExceededException" => {
                    CreateStackSetError::LimitExceeded(String::from(parsed_error.message))
                }
                "NameAlreadyExistsException" => {
                    CreateStackSetError::NameAlreadyExists(String::from(parsed_error.message))
                }
                _ => CreateStackSetError::Unknown(String::from(body)),
            },
            Err(_) => CreateStackSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for CreateStackSetError {
    fn from(err: XmlParseError) -> CreateStackSetError {
        let XmlParseError(message) = err;
        CreateStackSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateStackSetError {
    fn from(err: CredentialsError) -> CreateStackSetError {
        CreateStackSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStackSetError {
    fn from(err: HttpDispatchError) -> CreateStackSetError {
        CreateStackSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStackSetError {
    fn from(err: io::Error) -> CreateStackSetError {
        CreateStackSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStackSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStackSetError {
    fn description(&self) -> &str {
        match *self {
            CreateStackSetError::CreatedButModified(ref cause) => cause,
            CreateStackSetError::LimitExceeded(ref cause) => cause,
            CreateStackSetError::NameAlreadyExists(ref cause) => cause,
            CreateStackSetError::Validation(ref cause) => cause,
            CreateStackSetError::Credentials(ref err) => err.description(),
            CreateStackSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStackSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteChangeSet
#[derive(Debug, PartialEq)]
pub enum DeleteChangeSetError {
    /// <p>The specified change set can't be used to update the stack. For example, the change set status might be <code>CREATE_IN_PROGRESS</code>, or the stack status might be <code>UPDATE_IN_PROGRESS</code>.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidChangeSetStatus" => {
                    DeleteChangeSetError::InvalidChangeSetStatus(String::from(parsed_error.message))
                }
                _ => DeleteChangeSetError::Unknown(String::from(body)),
            },
            Err(_) => DeleteChangeSetError::Unknown(body.to_string()),
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
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "TokenAlreadyExistsException" => {
                    DeleteStackError::TokenAlreadyExists(String::from(parsed_error.message))
                }
                _ => DeleteStackError::Unknown(String::from(body)),
            },
            Err(_) => DeleteStackError::Unknown(body.to_string()),
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
/// Errors returned by DeleteStackInstances
#[derive(Debug, PartialEq)]
pub enum DeleteStackInstancesError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The specified operation ID already exists.</p>
    OperationIdAlreadyExists(String),
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteStackInstancesError {
    pub fn from_body(body: &str) -> DeleteStackInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidOperationException" => {
                    DeleteStackInstancesError::InvalidOperation(String::from(parsed_error.message))
                }
                "OperationIdAlreadyExistsException" => {
                    DeleteStackInstancesError::OperationIdAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressException" => DeleteStackInstancesError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "StackSetNotFoundException" => {
                    DeleteStackInstancesError::StackSetNotFound(String::from(parsed_error.message))
                }
                "StaleRequestException" => {
                    DeleteStackInstancesError::StaleRequest(String::from(parsed_error.message))
                }
                _ => DeleteStackInstancesError::Unknown(String::from(body)),
            },
            Err(_) => DeleteStackInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteStackInstancesError {
    fn from(err: XmlParseError) -> DeleteStackInstancesError {
        let XmlParseError(message) = err;
        DeleteStackInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteStackInstancesError {
    fn from(err: CredentialsError) -> DeleteStackInstancesError {
        DeleteStackInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStackInstancesError {
    fn from(err: HttpDispatchError) -> DeleteStackInstancesError {
        DeleteStackInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStackInstancesError {
    fn from(err: io::Error) -> DeleteStackInstancesError {
        DeleteStackInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStackInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStackInstancesError {
    fn description(&self) -> &str {
        match *self {
            DeleteStackInstancesError::InvalidOperation(ref cause) => cause,
            DeleteStackInstancesError::OperationIdAlreadyExists(ref cause) => cause,
            DeleteStackInstancesError::OperationInProgress(ref cause) => cause,
            DeleteStackInstancesError::StackSetNotFound(ref cause) => cause,
            DeleteStackInstancesError::StaleRequest(ref cause) => cause,
            DeleteStackInstancesError::Validation(ref cause) => cause,
            DeleteStackInstancesError::Credentials(ref err) => err.description(),
            DeleteStackInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteStackInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStackSet
#[derive(Debug, PartialEq)]
pub enum DeleteStackSetError {
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>You can't yet delete this stack set, because it still contains one or more stack instances. Delete all stack instances from the stack set before deleting the stack set.</p>
    StackSetNotEmpty(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteStackSetError {
    pub fn from_body(body: &str) -> DeleteStackSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationInProgressException" => {
                    DeleteStackSetError::OperationInProgress(String::from(parsed_error.message))
                }
                "StackSetNotEmptyException" => {
                    DeleteStackSetError::StackSetNotEmpty(String::from(parsed_error.message))
                }
                _ => DeleteStackSetError::Unknown(String::from(body)),
            },
            Err(_) => DeleteStackSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DeleteStackSetError {
    fn from(err: XmlParseError) -> DeleteStackSetError {
        let XmlParseError(message) = err;
        DeleteStackSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteStackSetError {
    fn from(err: CredentialsError) -> DeleteStackSetError {
        DeleteStackSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStackSetError {
    fn from(err: HttpDispatchError) -> DeleteStackSetError {
        DeleteStackSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStackSetError {
    fn from(err: io::Error) -> DeleteStackSetError {
        DeleteStackSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStackSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStackSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteStackSetError::OperationInProgress(ref cause) => cause,
            DeleteStackSetError::StackSetNotEmpty(ref cause) => cause,
            DeleteStackSetError::Validation(ref cause) => cause,
            DeleteStackSetError::Credentials(ref err) => err.description(),
            DeleteStackSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStackSetError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeAccountLimitsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeAccountLimitsError::Unknown(body.to_string()),
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
    /// <p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ChangeSetNotFound" => {
                    DescribeChangeSetError::ChangeSetNotFound(String::from(parsed_error.message))
                }
                _ => DescribeChangeSetError::Unknown(String::from(body)),
            },
            Err(_) => DescribeChangeSetError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeStackEventsError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackEventsError::Unknown(body.to_string()),
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
/// Errors returned by DescribeStackInstance
#[derive(Debug, PartialEq)]
pub enum DescribeStackInstanceError {
    /// <p>The specified stack instance doesn't exist.</p>
    StackInstanceNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStackInstanceError {
    pub fn from_body(body: &str) -> DescribeStackInstanceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "StackInstanceNotFoundException" => {
                    DescribeStackInstanceError::StackInstanceNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "StackSetNotFoundException" => {
                    DescribeStackInstanceError::StackSetNotFound(String::from(parsed_error.message))
                }
                _ => DescribeStackInstanceError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackInstanceError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeStackInstanceError {
    fn from(err: XmlParseError) -> DescribeStackInstanceError {
        let XmlParseError(message) = err;
        DescribeStackInstanceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackInstanceError {
    fn from(err: CredentialsError) -> DescribeStackInstanceError {
        DescribeStackInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackInstanceError {
    fn from(err: HttpDispatchError) -> DescribeStackInstanceError {
        DescribeStackInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackInstanceError {
    fn from(err: io::Error) -> DescribeStackInstanceError {
        DescribeStackInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackInstanceError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackInstanceError::StackInstanceNotFound(ref cause) => cause,
            DescribeStackInstanceError::StackSetNotFound(ref cause) => cause,
            DescribeStackInstanceError::Validation(ref cause) => cause,
            DescribeStackInstanceError::Credentials(ref err) => err.description(),
            DescribeStackInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackInstanceError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeStackResourceError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackResourceError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeStackResourcesError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackResourcesError::Unknown(body.to_string()),
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
/// Errors returned by DescribeStackSet
#[derive(Debug, PartialEq)]
pub enum DescribeStackSetError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStackSetError {
    pub fn from_body(body: &str) -> DescribeStackSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "StackSetNotFoundException" => {
                    DescribeStackSetError::StackSetNotFound(String::from(parsed_error.message))
                }
                _ => DescribeStackSetError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeStackSetError {
    fn from(err: XmlParseError) -> DescribeStackSetError {
        let XmlParseError(message) = err;
        DescribeStackSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackSetError {
    fn from(err: CredentialsError) -> DescribeStackSetError {
        DescribeStackSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackSetError {
    fn from(err: HttpDispatchError) -> DescribeStackSetError {
        DescribeStackSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackSetError {
    fn from(err: io::Error) -> DescribeStackSetError {
        DescribeStackSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackSetError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackSetError::StackSetNotFound(ref cause) => cause,
            DescribeStackSetError::Validation(ref cause) => cause,
            DescribeStackSetError::Credentials(ref err) => err.description(),
            DescribeStackSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStackSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStackSetOperation
#[derive(Debug, PartialEq)]
pub enum DescribeStackSetOperationError {
    /// <p>The specified ID refers to an operation that doesn't exist.</p>
    OperationNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStackSetOperationError {
    pub fn from_body(body: &str) -> DescribeStackSetOperationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationNotFoundException" => DescribeStackSetOperationError::OperationNotFound(
                    String::from(parsed_error.message),
                ),
                "StackSetNotFoundException" => DescribeStackSetOperationError::StackSetNotFound(
                    String::from(parsed_error.message),
                ),
                _ => DescribeStackSetOperationError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStackSetOperationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for DescribeStackSetOperationError {
    fn from(err: XmlParseError) -> DescribeStackSetOperationError {
        let XmlParseError(message) = err;
        DescribeStackSetOperationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeStackSetOperationError {
    fn from(err: CredentialsError) -> DescribeStackSetOperationError {
        DescribeStackSetOperationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStackSetOperationError {
    fn from(err: HttpDispatchError) -> DescribeStackSetOperationError {
        DescribeStackSetOperationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStackSetOperationError {
    fn from(err: io::Error) -> DescribeStackSetOperationError {
        DescribeStackSetOperationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStackSetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackSetOperationError {
    fn description(&self) -> &str {
        match *self {
            DescribeStackSetOperationError::OperationNotFound(ref cause) => cause,
            DescribeStackSetOperationError::StackSetNotFound(ref cause) => cause,
            DescribeStackSetOperationError::Validation(ref cause) => cause,
            DescribeStackSetOperationError::Credentials(ref err) => err.description(),
            DescribeStackSetOperationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeStackSetOperationError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => DescribeStacksError::Unknown(String::from(body)),
            },
            Err(_) => DescribeStacksError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => EstimateTemplateCostError::Unknown(String::from(body)),
            },
            Err(_) => EstimateTemplateCostError::Unknown(body.to_string()),
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
    /// <p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
    /// <p>The template contains resources with capabilities that weren't specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    /// <p>The specified change set can't be used to update the stack. For example, the change set status might be <code>CREATE_IN_PROGRESS</code>, or the stack status might be <code>UPDATE_IN_PROGRESS</code>.</p>
    InvalidChangeSetStatus(String),
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ChangeSetNotFound" => {
                    ExecuteChangeSetError::ChangeSetNotFound(String::from(parsed_error.message))
                }
                "InsufficientCapabilitiesException" => {
                    ExecuteChangeSetError::InsufficientCapabilities(String::from(
                        parsed_error.message,
                    ))
                }
                "InvalidChangeSetStatus" => ExecuteChangeSetError::InvalidChangeSetStatus(
                    String::from(parsed_error.message),
                ),
                "TokenAlreadyExistsException" => {
                    ExecuteChangeSetError::TokenAlreadyExists(String::from(parsed_error.message))
                }
                _ => ExecuteChangeSetError::Unknown(String::from(body)),
            },
            Err(_) => ExecuteChangeSetError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => GetStackPolicyError::Unknown(String::from(body)),
            },
            Err(_) => GetStackPolicyError::Unknown(body.to_string()),
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
    /// <p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "ChangeSetNotFound" => {
                    GetTemplateError::ChangeSetNotFound(String::from(parsed_error.message))
                }
                _ => GetTemplateError::Unknown(String::from(body)),
            },
            Err(_) => GetTemplateError::Unknown(body.to_string()),
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
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "StackSetNotFoundException" => {
                    GetTemplateSummaryError::StackSetNotFound(String::from(parsed_error.message))
                }
                _ => GetTemplateSummaryError::Unknown(String::from(body)),
            },
            Err(_) => GetTemplateSummaryError::Unknown(body.to_string()),
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
            GetTemplateSummaryError::StackSetNotFound(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListChangeSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListChangeSetsError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListExportsError::Unknown(String::from(body)),
            },
            Err(_) => ListExportsError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListImportsError::Unknown(String::from(body)),
            },
            Err(_) => ListImportsError::Unknown(body.to_string()),
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
/// Errors returned by ListStackInstances
#[derive(Debug, PartialEq)]
pub enum ListStackInstancesError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStackInstancesError {
    pub fn from_body(body: &str) -> ListStackInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "StackSetNotFoundException" => {
                    ListStackInstancesError::StackSetNotFound(String::from(parsed_error.message))
                }
                _ => ListStackInstancesError::Unknown(String::from(body)),
            },
            Err(_) => ListStackInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListStackInstancesError {
    fn from(err: XmlParseError) -> ListStackInstancesError {
        let XmlParseError(message) = err;
        ListStackInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStackInstancesError {
    fn from(err: CredentialsError) -> ListStackInstancesError {
        ListStackInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStackInstancesError {
    fn from(err: HttpDispatchError) -> ListStackInstancesError {
        ListStackInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStackInstancesError {
    fn from(err: io::Error) -> ListStackInstancesError {
        ListStackInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStackInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListStackInstancesError::StackSetNotFound(ref cause) => cause,
            ListStackInstancesError::Validation(ref cause) => cause,
            ListStackInstancesError::Credentials(ref err) => err.description(),
            ListStackInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStackInstancesError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListStackResourcesError::Unknown(String::from(body)),
            },
            Err(_) => ListStackResourcesError::Unknown(body.to_string()),
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
/// Errors returned by ListStackSetOperationResults
#[derive(Debug, PartialEq)]
pub enum ListStackSetOperationResultsError {
    /// <p>The specified ID refers to an operation that doesn't exist.</p>
    OperationNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStackSetOperationResultsError {
    pub fn from_body(body: &str) -> ListStackSetOperationResultsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "OperationNotFoundException" => {
                    ListStackSetOperationResultsError::OperationNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "StackSetNotFoundException" => ListStackSetOperationResultsError::StackSetNotFound(
                    String::from(parsed_error.message),
                ),
                _ => ListStackSetOperationResultsError::Unknown(String::from(body)),
            },
            Err(_) => ListStackSetOperationResultsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListStackSetOperationResultsError {
    fn from(err: XmlParseError) -> ListStackSetOperationResultsError {
        let XmlParseError(message) = err;
        ListStackSetOperationResultsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStackSetOperationResultsError {
    fn from(err: CredentialsError) -> ListStackSetOperationResultsError {
        ListStackSetOperationResultsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStackSetOperationResultsError {
    fn from(err: HttpDispatchError) -> ListStackSetOperationResultsError {
        ListStackSetOperationResultsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStackSetOperationResultsError {
    fn from(err: io::Error) -> ListStackSetOperationResultsError {
        ListStackSetOperationResultsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStackSetOperationResultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackSetOperationResultsError {
    fn description(&self) -> &str {
        match *self {
            ListStackSetOperationResultsError::OperationNotFound(ref cause) => cause,
            ListStackSetOperationResultsError::StackSetNotFound(ref cause) => cause,
            ListStackSetOperationResultsError::Validation(ref cause) => cause,
            ListStackSetOperationResultsError::Credentials(ref err) => err.description(),
            ListStackSetOperationResultsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStackSetOperationResultsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStackSetOperations
#[derive(Debug, PartialEq)]
pub enum ListStackSetOperationsError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStackSetOperationsError {
    pub fn from_body(body: &str) -> ListStackSetOperationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "StackSetNotFoundException" => ListStackSetOperationsError::StackSetNotFound(
                    String::from(parsed_error.message),
                ),
                _ => ListStackSetOperationsError::Unknown(String::from(body)),
            },
            Err(_) => ListStackSetOperationsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListStackSetOperationsError {
    fn from(err: XmlParseError) -> ListStackSetOperationsError {
        let XmlParseError(message) = err;
        ListStackSetOperationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStackSetOperationsError {
    fn from(err: CredentialsError) -> ListStackSetOperationsError {
        ListStackSetOperationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStackSetOperationsError {
    fn from(err: HttpDispatchError) -> ListStackSetOperationsError {
        ListStackSetOperationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStackSetOperationsError {
    fn from(err: io::Error) -> ListStackSetOperationsError {
        ListStackSetOperationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStackSetOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackSetOperationsError {
    fn description(&self) -> &str {
        match *self {
            ListStackSetOperationsError::StackSetNotFound(ref cause) => cause,
            ListStackSetOperationsError::Validation(ref cause) => cause,
            ListStackSetOperationsError::Credentials(ref err) => err.description(),
            ListStackSetOperationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListStackSetOperationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStackSets
#[derive(Debug, PartialEq)]
pub enum ListStackSetsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStackSetsError {
    pub fn from_body(body: &str) -> ListStackSetsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListStackSetsError::Unknown(String::from(body)),
            },
            Err(_) => ListStackSetsError::Unknown(body.to_string()),
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

impl From<XmlParseError> for ListStackSetsError {
    fn from(err: XmlParseError) -> ListStackSetsError {
        let XmlParseError(message) = err;
        ListStackSetsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListStackSetsError {
    fn from(err: CredentialsError) -> ListStackSetsError {
        ListStackSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStackSetsError {
    fn from(err: HttpDispatchError) -> ListStackSetsError {
        ListStackSetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStackSetsError {
    fn from(err: io::Error) -> ListStackSetsError {
        ListStackSetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStackSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackSetsError {
    fn description(&self) -> &str {
        match *self {
            ListStackSetsError::Validation(ref cause) => cause,
            ListStackSetsError::Credentials(ref err) => err.description(),
            ListStackSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStackSetsError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ListStacksError::Unknown(String::from(body)),
            },
            Err(_) => ListStacksError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SetStackPolicyError::Unknown(String::from(body)),
            },
            Err(_) => SetStackPolicyError::Unknown(body.to_string()),
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => SignalResourceError::Unknown(String::from(body)),
            },
            Err(_) => SignalResourceError::Unknown(body.to_string()),
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
/// Errors returned by StopStackSetOperation
#[derive(Debug, PartialEq)]
pub enum StopStackSetOperationError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The specified ID refers to an operation that doesn't exist.</p>
    OperationNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopStackSetOperationError {
    pub fn from_body(body: &str) -> StopStackSetOperationError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidOperationException" => {
                    StopStackSetOperationError::InvalidOperation(String::from(parsed_error.message))
                }
                "OperationNotFoundException" => StopStackSetOperationError::OperationNotFound(
                    String::from(parsed_error.message),
                ),
                "StackSetNotFoundException" => {
                    StopStackSetOperationError::StackSetNotFound(String::from(parsed_error.message))
                }
                _ => StopStackSetOperationError::Unknown(String::from(body)),
            },
            Err(_) => StopStackSetOperationError::Unknown(body.to_string()),
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

impl From<XmlParseError> for StopStackSetOperationError {
    fn from(err: XmlParseError) -> StopStackSetOperationError {
        let XmlParseError(message) = err;
        StopStackSetOperationError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for StopStackSetOperationError {
    fn from(err: CredentialsError) -> StopStackSetOperationError {
        StopStackSetOperationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopStackSetOperationError {
    fn from(err: HttpDispatchError) -> StopStackSetOperationError {
        StopStackSetOperationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopStackSetOperationError {
    fn from(err: io::Error) -> StopStackSetOperationError {
        StopStackSetOperationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopStackSetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopStackSetOperationError {
    fn description(&self) -> &str {
        match *self {
            StopStackSetOperationError::InvalidOperation(ref cause) => cause,
            StopStackSetOperationError::OperationNotFound(ref cause) => cause,
            StopStackSetOperationError::StackSetNotFound(ref cause) => cause,
            StopStackSetOperationError::Validation(ref cause) => cause,
            StopStackSetOperationError::Credentials(ref err) => err.description(),
            StopStackSetOperationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopStackSetOperationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStack
#[derive(Debug, PartialEq)]
pub enum UpdateStackError {
    /// <p>The template contains resources with capabilities that weren't specified in the Capabilities parameter.</p>
    InsufficientCapabilities(String),
    /// <p>A client request token already exists.</p>
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InsufficientCapabilitiesException" => {
                    UpdateStackError::InsufficientCapabilities(String::from(parsed_error.message))
                }
                "TokenAlreadyExistsException" => {
                    UpdateStackError::TokenAlreadyExists(String::from(parsed_error.message))
                }
                _ => UpdateStackError::Unknown(String::from(body)),
            },
            Err(_) => UpdateStackError::Unknown(body.to_string()),
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
/// Errors returned by UpdateStackInstances
#[derive(Debug, PartialEq)]
pub enum UpdateStackInstancesError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The specified operation ID already exists.</p>
    OperationIdAlreadyExists(String),
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>The specified stack instance doesn't exist.</p>
    StackInstanceNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateStackInstancesError {
    pub fn from_body(body: &str) -> UpdateStackInstancesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidOperationException" => {
                    UpdateStackInstancesError::InvalidOperation(String::from(parsed_error.message))
                }
                "OperationIdAlreadyExistsException" => {
                    UpdateStackInstancesError::OperationIdAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressException" => UpdateStackInstancesError::OperationInProgress(
                    String::from(parsed_error.message),
                ),
                "StackInstanceNotFoundException" => {
                    UpdateStackInstancesError::StackInstanceNotFound(String::from(
                        parsed_error.message,
                    ))
                }
                "StackSetNotFoundException" => {
                    UpdateStackInstancesError::StackSetNotFound(String::from(parsed_error.message))
                }
                "StaleRequestException" => {
                    UpdateStackInstancesError::StaleRequest(String::from(parsed_error.message))
                }
                _ => UpdateStackInstancesError::Unknown(String::from(body)),
            },
            Err(_) => UpdateStackInstancesError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateStackInstancesError {
    fn from(err: XmlParseError) -> UpdateStackInstancesError {
        let XmlParseError(message) = err;
        UpdateStackInstancesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateStackInstancesError {
    fn from(err: CredentialsError) -> UpdateStackInstancesError {
        UpdateStackInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStackInstancesError {
    fn from(err: HttpDispatchError) -> UpdateStackInstancesError {
        UpdateStackInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStackInstancesError {
    fn from(err: io::Error) -> UpdateStackInstancesError {
        UpdateStackInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStackInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackInstancesError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackInstancesError::InvalidOperation(ref cause) => cause,
            UpdateStackInstancesError::OperationIdAlreadyExists(ref cause) => cause,
            UpdateStackInstancesError::OperationInProgress(ref cause) => cause,
            UpdateStackInstancesError::StackInstanceNotFound(ref cause) => cause,
            UpdateStackInstancesError::StackSetNotFound(ref cause) => cause,
            UpdateStackInstancesError::StaleRequest(ref cause) => cause,
            UpdateStackInstancesError::Validation(ref cause) => cause,
            UpdateStackInstancesError::Credentials(ref err) => err.description(),
            UpdateStackInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateStackInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStackSet
#[derive(Debug, PartialEq)]
pub enum UpdateStackSetError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The specified operation ID already exists.</p>
    OperationIdAlreadyExists(String),
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateStackSetError {
    pub fn from_body(body: &str) -> UpdateStackSetError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                "InvalidOperationException" => {
                    UpdateStackSetError::InvalidOperation(String::from(parsed_error.message))
                }
                "OperationIdAlreadyExistsException" => {
                    UpdateStackSetError::OperationIdAlreadyExists(String::from(
                        parsed_error.message,
                    ))
                }
                "OperationInProgressException" => {
                    UpdateStackSetError::OperationInProgress(String::from(parsed_error.message))
                }
                "StackSetNotFoundException" => {
                    UpdateStackSetError::StackSetNotFound(String::from(parsed_error.message))
                }
                "StaleRequestException" => {
                    UpdateStackSetError::StaleRequest(String::from(parsed_error.message))
                }
                _ => UpdateStackSetError::Unknown(String::from(body)),
            },
            Err(_) => UpdateStackSetError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateStackSetError {
    fn from(err: XmlParseError) -> UpdateStackSetError {
        let XmlParseError(message) = err;
        UpdateStackSetError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateStackSetError {
    fn from(err: CredentialsError) -> UpdateStackSetError {
        UpdateStackSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStackSetError {
    fn from(err: HttpDispatchError) -> UpdateStackSetError {
        UpdateStackSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStackSetError {
    fn from(err: io::Error) -> UpdateStackSetError {
        UpdateStackSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStackSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStackSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateStackSetError::InvalidOperation(ref cause) => cause,
            UpdateStackSetError::OperationIdAlreadyExists(ref cause) => cause,
            UpdateStackSetError::OperationInProgress(ref cause) => cause,
            UpdateStackSetError::StackSetNotFound(ref cause) => cause,
            UpdateStackSetError::StaleRequest(ref cause) => cause,
            UpdateStackSetError::Validation(ref cause) => cause,
            UpdateStackSetError::Credentials(ref err) => err.description(),
            UpdateStackSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStackSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTerminationProtection
#[derive(Debug, PartialEq)]
pub enum UpdateTerminationProtectionError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTerminationProtectionError {
    pub fn from_body(body: &str) -> UpdateTerminationProtectionError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => UpdateTerminationProtectionError::Unknown(String::from(body)),
            },
            Err(_) => UpdateTerminationProtectionError::Unknown(body.to_string()),
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

impl From<XmlParseError> for UpdateTerminationProtectionError {
    fn from(err: XmlParseError) -> UpdateTerminationProtectionError {
        let XmlParseError(message) = err;
        UpdateTerminationProtectionError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for UpdateTerminationProtectionError {
    fn from(err: CredentialsError) -> UpdateTerminationProtectionError {
        UpdateTerminationProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTerminationProtectionError {
    fn from(err: HttpDispatchError) -> UpdateTerminationProtectionError {
        UpdateTerminationProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTerminationProtectionError {
    fn from(err: io::Error) -> UpdateTerminationProtectionError {
        UpdateTerminationProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTerminationProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTerminationProtectionError {
    fn description(&self) -> &str {
        match *self {
            UpdateTerminationProtectionError::Validation(ref cause) => cause,
            UpdateTerminationProtectionError::Credentials(ref err) => err.description(),
            UpdateTerminationProtectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateTerminationProtectionError::Unknown(ref cause) => cause,
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
        find_start_element(&mut stack);
        match Self::deserialize(&mut stack) {
            Ok(parsed_error) => match &parsed_error.code[..] {
                _ => ValidateTemplateError::Unknown(String::from(body)),
            },
            Err(_) => ValidateTemplateError::Unknown(body.to_string()),
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
    /// <p><p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE<em>IN</em>PROGRESS state.</p> </note></p>
    fn cancel_update_stack(
        &self,
        input: CancelUpdateStackInput,
    ) -> RusotoFuture<(), CancelUpdateStackError>;

    /// <p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>
    fn continue_update_rollback(
        &self,
        input: ContinueUpdateRollbackInput,
    ) -> RusotoFuture<ContinueUpdateRollbackOutput, ContinueUpdateRollbackError>;

    /// <p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>
    fn create_change_set(
        &self,
        input: CreateChangeSetInput,
    ) -> RusotoFuture<CreateChangeSetOutput, CreateChangeSetError>;

    /// <p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>
    fn create_stack(
        &self,
        input: CreateStackInput,
    ) -> RusotoFuture<CreateStackOutput, CreateStackError>;

    /// <p>Creates stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. <code>Accounts</code> and <code>Regions</code> are required parameters—you must specify at least one account and one region. </p>
    fn create_stack_instances(
        &self,
        input: CreateStackInstancesInput,
    ) -> RusotoFuture<CreateStackInstancesOutput, CreateStackInstancesError>;

    /// <p>Creates a stack set.</p>
    fn create_stack_set(
        &self,
        input: CreateStackSetInput,
    ) -> RusotoFuture<CreateStackSetOutput, CreateStackSetError>;

    /// <p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>
    fn delete_change_set(
        &self,
        input: DeleteChangeSetInput,
    ) -> RusotoFuture<DeleteChangeSetOutput, DeleteChangeSetError>;

    /// <p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>
    fn delete_stack(&self, input: DeleteStackInput) -> RusotoFuture<(), DeleteStackError>;

    /// <p>Deletes stack instances for the specified accounts, in the specified regions. </p>
    fn delete_stack_instances(
        &self,
        input: DeleteStackInstancesInput,
    ) -> RusotoFuture<DeleteStackInstancesOutput, DeleteStackInstancesError>;

    /// <p>Deletes a stack set. Before you can delete a stack set, all of its member stack instances must be deleted. For more information about how to do this, see <a>DeleteStackInstances</a>. </p>
    fn delete_stack_set(
        &self,
        input: DeleteStackSetInput,
    ) -> RusotoFuture<DeleteStackSetOutput, DeleteStackSetError>;

    /// <p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError>;

    /// <p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>
    fn describe_change_set(
        &self,
        input: DescribeChangeSetInput,
    ) -> RusotoFuture<DescribeChangeSetOutput, DescribeChangeSetError>;

    /// <p><p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack&#39;s event history, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note></p>
    fn describe_stack_events(
        &self,
        input: DescribeStackEventsInput,
    ) -> RusotoFuture<DescribeStackEventsOutput, DescribeStackEventsError>;

    /// <p>Returns the stack instance that's associated with the specified stack set, AWS account, and region.</p> <p>For a list of stack instances that are associated with a specific stack set, use <a>ListStackInstances</a>.</p>
    fn describe_stack_instance(
        &self,
        input: DescribeStackInstanceInput,
    ) -> RusotoFuture<DescribeStackInstanceOutput, DescribeStackInstanceError>;

    /// <p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>
    fn describe_stack_resource(
        &self,
        input: DescribeStackResourceInput,
    ) -> RusotoFuture<DescribeStackResourceOutput, DescribeStackResourceError>;

    /// <p><p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note></p>
    fn describe_stack_resources(
        &self,
        input: DescribeStackResourcesInput,
    ) -> RusotoFuture<DescribeStackResourcesOutput, DescribeStackResourcesError>;

    /// <p>Returns the description of the specified stack set. </p>
    fn describe_stack_set(
        &self,
        input: DescribeStackSetInput,
    ) -> RusotoFuture<DescribeStackSetOutput, DescribeStackSetError>;

    /// <p>Returns the description of the specified stack set operation. </p>
    fn describe_stack_set_operation(
        &self,
        input: DescribeStackSetOperationInput,
    ) -> RusotoFuture<DescribeStackSetOperationOutput, DescribeStackSetOperationError>;

    /// <p><p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note></p>
    fn describe_stacks(
        &self,
        input: DescribeStacksInput,
    ) -> RusotoFuture<DescribeStacksOutput, DescribeStacksError>;

    /// <p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    fn estimate_template_cost(
        &self,
        input: EstimateTemplateCostInput,
    ) -> RusotoFuture<EstimateTemplateCostOutput, EstimateTemplateCostError>;

    /// <p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>
    fn execute_change_set(
        &self,
        input: ExecuteChangeSetInput,
    ) -> RusotoFuture<ExecuteChangeSetOutput, ExecuteChangeSetError>;

    /// <p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>
    fn get_stack_policy(
        &self,
        input: GetStackPolicyInput,
    ) -> RusotoFuture<GetStackPolicyOutput, GetStackPolicyError>;

    /// <p><p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note></p>
    fn get_template(
        &self,
        input: GetTemplateInput,
    ) -> RusotoFuture<GetTemplateOutput, GetTemplateError>;

    /// <p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack or stack set.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a stack set, or a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>
    fn get_template_summary(
        &self,
        input: GetTemplateSummaryInput,
    ) -> RusotoFuture<GetTemplateSummaryOutput, GetTemplateSummaryError>;

    /// <p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>
    fn list_change_sets(
        &self,
        input: ListChangeSetsInput,
    ) -> RusotoFuture<ListChangeSetsOutput, ListChangeSetsError>;

    /// <p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html"> AWS CloudFormation Export Stack Output Values</a>.</p>
    fn list_exports(
        &self,
        input: ListExportsInput,
    ) -> RusotoFuture<ListExportsOutput, ListExportsError>;

    /// <p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p>
    fn list_imports(
        &self,
        input: ListImportsInput,
    ) -> RusotoFuture<ListImportsOutput, ListImportsError>;

    /// <p>Returns summary information about stack instances that are associated with the specified stack set. You can filter for stack instances that are associated with a specific AWS account name or region.</p>
    fn list_stack_instances(
        &self,
        input: ListStackInstancesInput,
    ) -> RusotoFuture<ListStackInstancesOutput, ListStackInstancesError>;

    /// <p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>
    fn list_stack_resources(
        &self,
        input: ListStackResourcesInput,
    ) -> RusotoFuture<ListStackResourcesOutput, ListStackResourcesError>;

    /// <p>Returns summary information about the results of a stack set operation. </p>
    fn list_stack_set_operation_results(
        &self,
        input: ListStackSetOperationResultsInput,
    ) -> RusotoFuture<ListStackSetOperationResultsOutput, ListStackSetOperationResultsError>;

    /// <p>Returns summary information about operations performed on a stack set. </p>
    fn list_stack_set_operations(
        &self,
        input: ListStackSetOperationsInput,
    ) -> RusotoFuture<ListStackSetOperationsOutput, ListStackSetOperationsError>;

    /// <p>Returns summary information about stack sets that are associated with the user.</p>
    fn list_stack_sets(
        &self,
        input: ListStackSetsInput,
    ) -> RusotoFuture<ListStackSetsOutput, ListStackSetsError>;

    /// <p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>
    fn list_stacks(
        &self,
        input: ListStacksInput,
    ) -> RusotoFuture<ListStacksOutput, ListStacksError>;

    /// <p>Sets a stack policy for a specified stack.</p>
    fn set_stack_policy(&self, input: SetStackPolicyInput)
        -> RusotoFuture<(), SetStackPolicyError>;

    /// <p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>
    fn signal_resource(&self, input: SignalResourceInput) -> RusotoFuture<(), SignalResourceError>;

    /// <p>Stops an in-progress operation on a stack set and its associated stack instances. </p>
    fn stop_stack_set_operation(
        &self,
        input: StopStackSetOperationInput,
    ) -> RusotoFuture<StopStackSetOperationOutput, StopStackSetOperationError>;

    /// <p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html">Updating a Stack</a>.</p>
    fn update_stack(
        &self,
        input: UpdateStackInput,
    ) -> RusotoFuture<UpdateStackOutput, UpdateStackError>;

    /// <p>Updates the parameter values for stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. </p> <p>You can only update stack instances in regions and accounts where they already exist; to create additional stack instances, use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CreateStackInstances.html">CreateStackInstances</a>. </p> <p>During stack set updates, any parameters overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only update the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    fn update_stack_instances(
        &self,
        input: UpdateStackInstancesInput,
    ) -> RusotoFuture<UpdateStackInstancesOutput, UpdateStackInstancesError>;

    /// <p>Updates the stack set and <i>all</i> associated stack instances.</p> <p>Even if the stack set operation created by updating the stack set fails (completely or partially, below or above a specified failure tolerance), the stack set is updated with your changes. Subsequent <a>CreateStackInstances</a> calls on the specified stack set use the updated stack set.</p>
    fn update_stack_set(
        &self,
        input: UpdateStackSetInput,
    ) -> RusotoFuture<UpdateStackSetOutput, UpdateStackSetError>;

    /// <p>Updates termination protection for the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p> For <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack.</p>
    fn update_termination_protection(
        &self,
        input: UpdateTerminationProtectionInput,
    ) -> RusotoFuture<UpdateTerminationProtectionOutput, UpdateTerminationProtectionError>;

    /// <p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>
    fn validate_template(
        &self,
        input: ValidateTemplateInput,
    ) -> RusotoFuture<ValidateTemplateOutput, ValidateTemplateError>;
}
/// A client for the AWS CloudFormation API.
pub struct CloudFormationClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudFormationClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudFormationClient {
        CloudFormationClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudFormationClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudFormationClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudFormation for CloudFormationClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE<em>IN</em>PROGRESS state.</p> </note></p>
    fn cancel_update_stack(
        &self,
        input: CancelUpdateStackInput,
    ) -> RusotoFuture<(), CancelUpdateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CancelUpdateStack");
        params.put("Version", "2010-05-15");
        CancelUpdateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelUpdateStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>
    fn continue_update_rollback(
        &self,
        input: ContinueUpdateRollbackInput,
    ) -> RusotoFuture<ContinueUpdateRollbackOutput, ContinueUpdateRollbackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ContinueUpdateRollback");
        params.put("Version", "2010-05-15");
        ContinueUpdateRollbackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ContinueUpdateRollbackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ContinueUpdateRollbackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ContinueUpdateRollbackOutputDeserializer::deserialize(
                        "ContinueUpdateRollbackResult",
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

    /// <p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>
    fn create_change_set(
        &self,
        input: CreateChangeSetInput,
    ) -> RusotoFuture<CreateChangeSetOutput, CreateChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateChangeSet");
        params.put("Version", "2010-05-15");
        CreateChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateChangeSetOutputDeserializer::deserialize(
                        "CreateChangeSetResult",
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

    /// <p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>
    fn create_stack(
        &self,
        input: CreateStackInput,
    ) -> RusotoFuture<CreateStackOutput, CreateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStack");
        params.put("Version", "2010-05-15");
        CreateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateStackOutputDeserializer::deserialize(
                        "CreateStackResult",
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

    /// <p>Creates stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. <code>Accounts</code> and <code>Regions</code> are required parameters—you must specify at least one account and one region. </p>
    fn create_stack_instances(
        &self,
        input: CreateStackInstancesInput,
    ) -> RusotoFuture<CreateStackInstancesOutput, CreateStackInstancesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStackInstances");
        params.put("Version", "2010-05-15");
        CreateStackInstancesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStackInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackInstancesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateStackInstancesOutputDeserializer::deserialize(
                        "CreateStackInstancesResult",
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

    /// <p>Creates a stack set.</p>
    fn create_stack_set(
        &self,
        input: CreateStackSetInput,
    ) -> RusotoFuture<CreateStackSetOutput, CreateStackSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStackSet");
        params.put("Version", "2010-05-15");
        CreateStackSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateStackSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateStackSetOutputDeserializer::deserialize(
                        "CreateStackSetResult",
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

    /// <p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>
    fn delete_change_set(
        &self,
        input: DeleteChangeSetInput,
    ) -> RusotoFuture<DeleteChangeSetOutput, DeleteChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteChangeSet");
        params.put("Version", "2010-05-15");
        DeleteChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteChangeSetOutputDeserializer::deserialize(
                        "DeleteChangeSetResult",
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

    /// <p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>
    fn delete_stack(&self, input: DeleteStackInput) -> RusotoFuture<(), DeleteStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStack");
        params.put("Version", "2010-05-15");
        DeleteStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes stack instances for the specified accounts, in the specified regions. </p>
    fn delete_stack_instances(
        &self,
        input: DeleteStackInstancesInput,
    ) -> RusotoFuture<DeleteStackInstancesOutput, DeleteStackInstancesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStackInstances");
        params.put("Version", "2010-05-15");
        DeleteStackInstancesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStackInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteStackInstancesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteStackInstancesOutputDeserializer::deserialize(
                        "DeleteStackInstancesResult",
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

    /// <p>Deletes a stack set. Before you can delete a stack set, all of its member stack instances must be deleted. For more information about how to do this, see <a>DeleteStackInstances</a>. </p>
    fn delete_stack_set(
        &self,
        input: DeleteStackSetInput,
    ) -> RusotoFuture<DeleteStackSetOutput, DeleteStackSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStackSet");
        params.put("Version", "2010-05-15");
        DeleteStackSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteStackSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteStackSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteStackSetOutputDeserializer::deserialize(
                        "DeleteStackSetResult",
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

    /// <p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsInput,
    ) -> RusotoFuture<DescribeAccountLimitsOutput, DescribeAccountLimitsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2010-05-15");
        DescribeAccountLimitsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountLimitsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAccountLimitsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeAccountLimitsOutputDeserializer::deserialize(
                        "DescribeAccountLimitsResult",
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

    /// <p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>
    fn describe_change_set(
        &self,
        input: DescribeChangeSetInput,
    ) -> RusotoFuture<DescribeChangeSetOutput, DescribeChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeChangeSet");
        params.put("Version", "2010-05-15");
        DescribeChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeChangeSetOutputDeserializer::deserialize(
                        "DescribeChangeSetResult",
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

    /// <p><p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack&#39;s event history, go to <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note></p>
    fn describe_stack_events(
        &self,
        input: DescribeStackEventsInput,
    ) -> RusotoFuture<DescribeStackEventsOutput, DescribeStackEventsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackEvents");
        params.put("Version", "2010-05-15");
        DescribeStackEventsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackEventsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackEventsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackEventsOutputDeserializer::deserialize(
                        "DescribeStackEventsResult",
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

    /// <p>Returns the stack instance that's associated with the specified stack set, AWS account, and region.</p> <p>For a list of stack instances that are associated with a specific stack set, use <a>ListStackInstances</a>.</p>
    fn describe_stack_instance(
        &self,
        input: DescribeStackInstanceInput,
    ) -> RusotoFuture<DescribeStackInstanceOutput, DescribeStackInstanceError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackInstance");
        params.put("Version", "2010-05-15");
        DescribeStackInstanceInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackInstanceOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackInstanceOutputDeserializer::deserialize(
                        "DescribeStackInstanceResult",
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

    /// <p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>
    fn describe_stack_resource(
        &self,
        input: DescribeStackResourceInput,
    ) -> RusotoFuture<DescribeStackResourceOutput, DescribeStackResourceError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResource");
        params.put("Version", "2010-05-15");
        DescribeStackResourceInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackResourceOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackResourceOutputDeserializer::deserialize(
                        "DescribeStackResourceResult",
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

    /// <p><p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note></p>
    fn describe_stack_resources(
        &self,
        input: DescribeStackResourcesInput,
    ) -> RusotoFuture<DescribeStackResourcesOutput, DescribeStackResourcesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResources");
        params.put("Version", "2010-05-15");
        DescribeStackResourcesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackResourcesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackResourcesOutputDeserializer::deserialize(
                        "DescribeStackResourcesResult",
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

    /// <p>Returns the description of the specified stack set. </p>
    fn describe_stack_set(
        &self,
        input: DescribeStackSetInput,
    ) -> RusotoFuture<DescribeStackSetOutput, DescribeStackSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackSet");
        params.put("Version", "2010-05-15");
        DescribeStackSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackSetOutputDeserializer::deserialize(
                        "DescribeStackSetResult",
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

    /// <p>Returns the description of the specified stack set operation. </p>
    fn describe_stack_set_operation(
        &self,
        input: DescribeStackSetOperationInput,
    ) -> RusotoFuture<DescribeStackSetOperationOutput, DescribeStackSetOperationError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackSetOperation");
        params.put("Version", "2010-05-15");
        DescribeStackSetOperationInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackSetOperationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackSetOperationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStackSetOperationOutputDeserializer::deserialize(
                        "DescribeStackSetOperationResult",
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

    /// <p><p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note></p>
    fn describe_stacks(
        &self,
        input: DescribeStacksInput,
    ) -> RusotoFuture<DescribeStacksOutput, DescribeStacksError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStacks");
        params.put("Version", "2010-05-15");
        DescribeStacksInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStacksOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeStacksOutputDeserializer::deserialize(
                        "DescribeStacksResult",
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

    /// <p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    fn estimate_template_cost(
        &self,
        input: EstimateTemplateCostInput,
    ) -> RusotoFuture<EstimateTemplateCostOutput, EstimateTemplateCostError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "EstimateTemplateCost");
        params.put("Version", "2010-05-15");
        EstimateTemplateCostInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EstimateTemplateCostError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EstimateTemplateCostOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EstimateTemplateCostOutputDeserializer::deserialize(
                        "EstimateTemplateCostResult",
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

    /// <p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>
    fn execute_change_set(
        &self,
        input: ExecuteChangeSetInput,
    ) -> RusotoFuture<ExecuteChangeSetOutput, ExecuteChangeSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ExecuteChangeSet");
        params.put("Version", "2010-05-15");
        ExecuteChangeSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExecuteChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ExecuteChangeSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ExecuteChangeSetOutputDeserializer::deserialize(
                        "ExecuteChangeSetResult",
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

    /// <p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>
    fn get_stack_policy(
        &self,
        input: GetStackPolicyInput,
    ) -> RusotoFuture<GetStackPolicyOutput, GetStackPolicyError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetStackPolicy");
        params.put("Version", "2010-05-15");
        GetStackPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetStackPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetStackPolicyOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetStackPolicyOutputDeserializer::deserialize(
                        "GetStackPolicyResult",
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

    /// <p><p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note></p>
    fn get_template(
        &self,
        input: GetTemplateInput,
    ) -> RusotoFuture<GetTemplateOutput, GetTemplateError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplate");
        params.put("Version", "2010-05-15");
        GetTemplateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTemplateOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTemplateOutputDeserializer::deserialize(
                        "GetTemplateResult",
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

    /// <p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack or stack set.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a stack set, or a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>
    fn get_template_summary(
        &self,
        input: GetTemplateSummaryInput,
    ) -> RusotoFuture<GetTemplateSummaryOutput, GetTemplateSummaryError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplateSummary");
        params.put("Version", "2010-05-15");
        GetTemplateSummaryInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetTemplateSummaryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTemplateSummaryOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(GetTemplateSummaryOutputDeserializer::deserialize(
                        "GetTemplateSummaryResult",
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

    /// <p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>
    fn list_change_sets(
        &self,
        input: ListChangeSetsInput,
    ) -> RusotoFuture<ListChangeSetsOutput, ListChangeSetsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListChangeSets");
        params.put("Version", "2010-05-15");
        ListChangeSetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListChangeSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListChangeSetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListChangeSetsOutputDeserializer::deserialize(
                        "ListChangeSetsResult",
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

    /// <p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html"> AWS CloudFormation Export Stack Output Values</a>.</p>
    fn list_exports(
        &self,
        input: ListExportsInput,
    ) -> RusotoFuture<ListExportsOutput, ListExportsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListExports");
        params.put("Version", "2010-05-15");
        ListExportsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListExportsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListExportsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListExportsOutputDeserializer::deserialize(
                        "ListExportsResult",
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

    /// <p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p>
    fn list_imports(
        &self,
        input: ListImportsInput,
    ) -> RusotoFuture<ListImportsOutput, ListImportsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListImports");
        params.put("Version", "2010-05-15");
        ListImportsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListImportsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListImportsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListImportsOutputDeserializer::deserialize(
                        "ListImportsResult",
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

    /// <p>Returns summary information about stack instances that are associated with the specified stack set. You can filter for stack instances that are associated with a specific AWS account name or region.</p>
    fn list_stack_instances(
        &self,
        input: ListStackInstancesInput,
    ) -> RusotoFuture<ListStackInstancesOutput, ListStackInstancesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackInstances");
        params.put("Version", "2010-05-15");
        ListStackInstancesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStackInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackInstancesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackInstancesOutputDeserializer::deserialize(
                        "ListStackInstancesResult",
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

    /// <p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>
    fn list_stack_resources(
        &self,
        input: ListStackResourcesInput,
    ) -> RusotoFuture<ListStackResourcesOutput, ListStackResourcesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackResources");
        params.put("Version", "2010-05-15");
        ListStackResourcesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStackResourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackResourcesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackResourcesOutputDeserializer::deserialize(
                        "ListStackResourcesResult",
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

    /// <p>Returns summary information about the results of a stack set operation. </p>
    fn list_stack_set_operation_results(
        &self,
        input: ListStackSetOperationResultsInput,
    ) -> RusotoFuture<ListStackSetOperationResultsOutput, ListStackSetOperationResultsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSetOperationResults");
        params.put("Version", "2010-05-15");
        ListStackSetOperationResultsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStackSetOperationResultsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetOperationResultsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackSetOperationResultsOutputDeserializer::deserialize(
                        "ListStackSetOperationResultsResult",
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

    /// <p>Returns summary information about operations performed on a stack set. </p>
    fn list_stack_set_operations(
        &self,
        input: ListStackSetOperationsInput,
    ) -> RusotoFuture<ListStackSetOperationsOutput, ListStackSetOperationsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSetOperations");
        params.put("Version", "2010-05-15");
        ListStackSetOperationsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStackSetOperationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetOperationsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackSetOperationsOutputDeserializer::deserialize(
                        "ListStackSetOperationsResult",
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

    /// <p>Returns summary information about stack sets that are associated with the user.</p>
    fn list_stack_sets(
        &self,
        input: ListStackSetsInput,
    ) -> RusotoFuture<ListStackSetsOutput, ListStackSetsError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSets");
        params.put("Version", "2010-05-15");
        ListStackSetsInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStackSetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetsOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStackSetsOutputDeserializer::deserialize(
                        "ListStackSetsResult",
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

    /// <p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>
    fn list_stacks(
        &self,
        input: ListStacksInput,
    ) -> RusotoFuture<ListStacksOutput, ListStacksError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStacks");
        params.put("Version", "2010-05-15");
        ListStacksInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListStacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStacksOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ListStacksOutputDeserializer::deserialize(
                        "ListStacksResult",
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

    /// <p>Sets a stack policy for a specified stack.</p>
    fn set_stack_policy(
        &self,
        input: SetStackPolicyInput,
    ) -> RusotoFuture<(), SetStackPolicyError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SetStackPolicy");
        params.put("Version", "2010-05-15");
        SetStackPolicyInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetStackPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>
    fn signal_resource(&self, input: SignalResourceInput) -> RusotoFuture<(), SignalResourceError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "SignalResource");
        params.put("Version", "2010-05-15");
        SignalResourceInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SignalResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(future::ok(::std::mem::drop(response)))
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops an in-progress operation on a stack set and its associated stack instances. </p>
    fn stop_stack_set_operation(
        &self,
        input: StopStackSetOperationInput,
    ) -> RusotoFuture<StopStackSetOperationOutput, StopStackSetOperationError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "StopStackSetOperation");
        params.put("Version", "2010-05-15");
        StopStackSetOperationInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopStackSetOperationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = StopStackSetOperationOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(StopStackSetOperationOutputDeserializer::deserialize(
                        "StopStackSetOperationResult",
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

    /// <p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html">Updating a Stack</a>.</p>
    fn update_stack(
        &self,
        input: UpdateStackInput,
    ) -> RusotoFuture<UpdateStackOutput, UpdateStackError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStack");
        params.put("Version", "2010-05-15");
        UpdateStackInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateStackOutputDeserializer::deserialize(
                        "UpdateStackResult",
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

    /// <p>Updates the parameter values for stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. </p> <p>You can only update stack instances in regions and accounts where they already exist; to create additional stack instances, use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CreateStackInstances.html">CreateStackInstances</a>. </p> <p>During stack set updates, any parameters overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only update the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    fn update_stack_instances(
        &self,
        input: UpdateStackInstancesInput,
    ) -> RusotoFuture<UpdateStackInstancesOutput, UpdateStackInstancesError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStackInstances");
        params.put("Version", "2010-05-15");
        UpdateStackInstancesInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStackInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackInstancesOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateStackInstancesOutputDeserializer::deserialize(
                        "UpdateStackInstancesResult",
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

    /// <p>Updates the stack set and <i>all</i> associated stack instances.</p> <p>Even if the stack set operation created by updating the stack set fails (completely or partially, below or above a specified failure tolerance), the stack set is updated with your changes. Subsequent <a>CreateStackInstances</a> calls on the specified stack set use the updated stack set.</p>
    fn update_stack_set(
        &self,
        input: UpdateStackSetInput,
    ) -> RusotoFuture<UpdateStackSetOutput, UpdateStackSetError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStackSet");
        params.put("Version", "2010-05-15");
        UpdateStackSetInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateStackSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackSetOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateStackSetOutputDeserializer::deserialize(
                        "UpdateStackSetResult",
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

    /// <p>Updates termination protection for the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p> For <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack.</p>
    fn update_termination_protection(
        &self,
        input: UpdateTerminationProtectionInput,
    ) -> RusotoFuture<UpdateTerminationProtectionOutput, UpdateTerminationProtectionError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateTerminationProtection");
        params.put("Version", "2010-05-15");
        UpdateTerminationProtectionInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTerminationProtectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateTerminationProtectionOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(UpdateTerminationProtectionOutputDeserializer::deserialize(
                        "UpdateTerminationProtectionResult",
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

    /// <p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>
    fn validate_template(
        &self,
        input: ValidateTemplateInput,
    ) -> RusotoFuture<ValidateTemplateOutput, ValidateTemplateError> {
        let mut request = SignedRequest::new("POST", "cloudformation", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ValidateTemplate");
        params.put("Version", "2010-05-15");
        ValidateTemplateInputSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status != StatusCode::OK {
                return future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ValidateTemplateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }));
            }

            future::Either::A(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ValidateTemplateOutput::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_slice(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ValidateTemplateOutputDeserializer::deserialize(
                        "ValidateTemplateResult",
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
    fn test_parse_error_cloudformation_cancel_update_stack() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "cloudformation-cancel-update-stack.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CancelUpdateStackInput::default();
        let result = client.cancel_update_stack(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudformation_describe_stacks() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudformation-describe-stacks.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeStacksInput::default();
        let result = client.describe_stacks(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudformation_get_template() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudformation-get-template.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetTemplateInput::default();
        let result = client.get_template(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudformation_list_stacks() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudformation-list-stacks.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListStacksInput::default();
        let result = client.list_stacks(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
