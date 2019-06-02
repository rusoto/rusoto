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
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

struct AccountDeserializer;
impl AccountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Structure that contains the results of the account gate function which AWS CloudFormation invokes, if present, before proceeding with a stack set operation in an account and region.</p> <p>For each account and region, AWS CloudFormation lets you specify a Lamdba function that encapsulates any requirements that must be met before CloudFormation can proceed with a stack set operation in that account and region. CloudFormation invokes the function each time a stack set operation is requested for that account and region; if the function returns <code>FAILED</code>, CloudFormation cancels the operation in that account and region, and sets the stack set operation result status for that account and region to <code>FAILED</code>. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-account-gating.html">Configuring a target account gate</a>.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountGateResult, XmlParseError> {
        deserialize_elements::<_, AccountGateResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Status" => {
                    obj.status = Some(AccountGateStatusDeserializer::deserialize("Status", stack)?);
                }
                "StatusReason" => {
                    obj.status_reason = Some(AccountGateStatusReasonDeserializer::deserialize(
                        "StatusReason",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AccountGateStatusDeserializer;
impl AccountGateStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AccountGateStatusReasonDeserializer;
impl AccountGateStatusReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The AccountLimit data type. For more information about account limits, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">AWS CloudFormation Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccountLimit {
    /// <p>The name of the account limit.</p>
    pub name: Option<String>,
    /// <p>The value that is associated with the account limit name.</p>
    pub value: Option<i64>,
}

struct AccountLimitDeserializer;
impl AccountLimitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AccountLimit, XmlParseError> {
        deserialize_elements::<_, AccountLimit, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = Some(LimitNameDeserializer::deserialize("Name", stack)?);
                }
                "Value" => {
                    obj.value = Some(LimitValueDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AccountLimitListDeserializer;
impl AccountLimitListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AccountLimit>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AccountLimitDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AllowedValuesDeserializer;
impl AllowedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AllowedValueDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ArnDeserializer;
impl ArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct BoxedIntegerDeserializer;
impl BoxedIntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>CancelUpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CancelUpdateStackRequest {
    /// <p>A unique identifier for this <code>CancelUpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to cancel an update on a stack with the same name. You might retry <code>CancelUpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack.</p>
    pub stack_name: String,
}

/// Serialize `CancelUpdateStackRequest` contents to a `SignedRequest`.
struct CancelUpdateStackRequestSerializer;
impl CancelUpdateStackRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CancelUpdateStackRequest) {
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CancelUpdateStackResponse {}

struct CancelUpdateStackResponseDeserializer;
impl CancelUpdateStackResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CancelUpdateStackResponse, XmlParseError> {
        Ok(CancelUpdateStackResponse::default())
    }
}
struct CapabilitiesDeserializer;
impl CapabilitiesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(CapabilityDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CapabilityDeserializer;
impl CapabilityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct CausingEntityDeserializer;
impl CausingEntityDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Change, XmlParseError> {
        deserialize_elements::<_, Change, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ResourceChange" => {
                    obj.resource_change = Some(ResourceChangeDeserializer::deserialize(
                        "ResourceChange",
                        stack,
                    )?);
                }
                "Type" => {
                    obj.type_ = Some(ChangeTypeDeserializer::deserialize("Type", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ChangeActionDeserializer;
impl ChangeActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeSetIdDeserializer;
impl ChangeSetIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeSetNameDeserializer;
impl ChangeSetNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeSetStatusDeserializer;
impl ChangeSetStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeSetStatusReasonDeserializer;
impl ChangeSetStatusReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeSetSummariesDeserializer;
impl ChangeSetSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ChangeSetSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ChangeSetSummaryDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ChangeSetSummary, XmlParseError> {
        deserialize_elements::<_, ChangeSetSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ChangeSetId" => {
                    obj.change_set_id =
                        Some(ChangeSetIdDeserializer::deserialize("ChangeSetId", stack)?);
                }
                "ChangeSetName" => {
                    obj.change_set_name = Some(ChangeSetNameDeserializer::deserialize(
                        "ChangeSetName",
                        stack,
                    )?);
                }
                "CreationTime" => {
                    obj.creation_time = Some(CreationTimeDeserializer::deserialize(
                        "CreationTime",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "ExecutionStatus" => {
                    obj.execution_status = Some(ExecutionStatusDeserializer::deserialize(
                        "ExecutionStatus",
                        stack,
                    )?);
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackName" => {
                    obj.stack_name = Some(StackNameDeserializer::deserialize("StackName", stack)?);
                }
                "Status" => {
                    obj.status = Some(ChangeSetStatusDeserializer::deserialize("Status", stack)?);
                }
                "StatusReason" => {
                    obj.status_reason = Some(ChangeSetStatusReasonDeserializer::deserialize(
                        "StatusReason",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ChangeSourceDeserializer;
impl ChangeSourceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangeTypeDeserializer;
impl ChangeTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ChangesDeserializer;
impl ChangesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Change>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ChangeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ClientRequestTokenDeserializer;
impl ClientRequestTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>ContinueUpdateRollback</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinueUpdateRollbackRequest {
    /// <p>A unique identifier for this <code>ContinueUpdateRollback</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to continue the rollback to a stack with the same name. You might retry <code>ContinueUpdateRollback</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p><p>A list of the logical IDs of the resources that AWS CloudFormation skips during the continue update rollback operation. You can specify only resources that are in the <code>UPDATE<em>FAILED</code> state because a rollback failed. You can&#39;t specify resources that are in the <code>UPDATE</em>FAILED</code> state for other reasons, for example, because an update was cancelled. To check why a resource update failed, use the <a>DescribeStackResources</a> action, and view the resource status reason. </p> <important> <p>Specify this property to skip rolling back resources that AWS CloudFormation can&#39;t successfully roll back. We recommend that you <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> troubleshoot</a> resources before skipping them. AWS CloudFormation sets the status of the specified resources to <code>UPDATE<em>COMPLETE</code> and continues to roll back the stack. After the rollback is complete, the state of the skipped resources will be inconsistent with the state of the resources in the stack template. Before performing another stack update, you must update the stack or resources to be consistent with each other. If you don&#39;t, subsequent stack updates might fail, and the stack will become unrecoverable. </p> </important> <p>Specify the minimum number of resources required to successfully roll back your stack. For example, a failed resource update might cause dependent resources to fail. In this case, it might not be necessary to skip the dependent resources. </p> <p>To skip resources that are part of nested stacks, use the following format: <code>NestedStackName.ResourceLogicalID</code>. If you want to specify the logical ID of a stack resource (<code>Type: AWS::CloudFormation::Stack</code>) in the <code>ResourcesToSkip</code> list, then its corresponding embedded stack must be in one of the following states: <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>COMPLETE</code>, or <code>DELETE_FAILED</code>. </p> <note> <p>Don&#39;t confuse a child stack&#39;s name with its corresponding logical ID defined in the parent stack. For an example of a continue update rollback operation with nested stacks, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-continueupdaterollback.html#nested-stacks">Using ResourcesToSkip to recover a nested stacks hierarchy</a>. </p> </note></p>
    pub resources_to_skip: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to roll back the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p><p>The name or the unique ID of the stack that you want to continue rolling back.</p> <note> <p>Don&#39;t specify the name of a nested stack (a stack that was created by using the <code>AWS::CloudFormation::Stack</code> resource). Instead, use this operation on the parent stack (the stack that contains the <code>AWS::CloudFormation::Stack</code> resource).</p> </note></p>
    pub stack_name: String,
}

/// Serialize `ContinueUpdateRollbackRequest` contents to a `SignedRequest`.
struct ContinueUpdateRollbackRequestSerializer;
impl ContinueUpdateRollbackRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ContinueUpdateRollbackRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.resources_to_skip {
            ResourcesToSkipSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ResourcesToSkip"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

/// <p>The output for a <a>ContinueUpdateRollback</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContinueUpdateRollbackResponse {}

struct ContinueUpdateRollbackResponseDeserializer;
impl ContinueUpdateRollbackResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ContinueUpdateRollbackResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = ContinueUpdateRollbackResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>CreateChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateChangeSetRequest {
    /// <p><p>In some cases, you must explicity acknowledge that your stack template contains certain capabilities in order for AWS CloudFormation to create the stack.</p> <ul> <li> <p> <code>CAPABILITY<em>IAM</code> and <code>CAPABILITY</em>NAMED<em>IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your AWS account; for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY</em>IAM</code> or <code>CAPABILITY<em>NAMED</em>IAM</code> capability.</p> <ul> <li> <p>If you have IAM resources, you can specify either capability. </p> </li> <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY<em>NAMED</em>IAM</code>. </p> </li> <li> <p>If you don&#39;t specify either of these capabilities, AWS CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p> </li> <li> <p> <code>CAPABILITY<em>AUTO</em>EXPAND</code> </p> <p>Some template contain macros. Macros perform custom processing on templates; this can include simple actions like find-and-replace operations, all the way to extensive transformations of entire templates. Because of this, users typically create a change set from the processed template, so that they can review the changes resulting from the macros before actually creating the stack. If your stack template contains one or more macros, and you choose to create a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by AWS CloudFormation.</p> <note> <p>This capacity does not apply to creating change sets, and specifying it when creating change sets has no effect.</p> <p>Also, change sets do not currently support nested stacks. If you want to create a stack from a stack template that contains macros <i>and</i> nested stacks, you must create or update the stack directly from the template using the <a>CreateStack</a> or <a>UpdateStack</a> action, and specifying this capability.</p> </note> <p>For more information on macros, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using AWS CloudFormation Macros to Perform Custom Processing on Templates</a>.</p> </li> </ul></p>
    pub capabilities: Option<Vec<String>>,
    /// <p>The name of the change set. The name must be unique among all change sets that are associated with the specified stack.</p> <p>A change set name can contain only alphanumeric, case sensitive characters and hyphens. It must start with an alphabetic character and cannot exceed 128 characters.</p>
    pub change_set_name: String,
    /// <p>The type of change set operation. To create a change set for a new stack, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code>.</p> <p>If you create a change set for a new stack, AWS Cloudformation creates a stack with a unique stack ID, but no template or resources. The stack will be in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-describing-stacks.html#d0e11995"> <code>REVIEW_IN_PROGRESS</code> </a> state until you execute the change set.</p> <p>By default, AWS CloudFormation specifies <code>UPDATE</code>. You can't use the <code>UPDATE</code> type to create a change set for a new stack or the <code>CREATE</code> type to create a change set for an existing stack.</p>
    pub change_set_type: Option<String>,
    /// <p>A unique identifier for this <code>CreateChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create another change set with the same name. You might retry <code>CreateChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_token: Option<String>,
    /// <p>A description to help you identify this change set.</p>
    pub description: Option<String>,
    /// <p>The Amazon Resource Names (ARNs) of Amazon Simple Notification Service (Amazon SNS) topics that AWS CloudFormation associates with the stack. To remove all associated notification topics, specify an empty list.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the change set. For more information, see the <a>Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with if you execute this change set, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource type that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for condition keys in IAM policies for AWS CloudFormation. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a> in the AWS CloudFormation User Guide.</p>
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

/// Serialize `CreateChangeSetRequest` contents to a `SignedRequest`.
struct CreateChangeSetRequestSerializer;
impl CreateChangeSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateChangeSetRequest) {
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
            &obj.change_set_name,
        );
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
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
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
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value,
            );
        }
    }
}

/// <p>The output for the <a>CreateChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateChangeSetResponse {
    /// <p>The Amazon Resource Name (ARN) of the change set.</p>
    pub id: Option<String>,
    /// <p>The unique ID of the stack.</p>
    pub stack_id: Option<String>,
}

struct CreateChangeSetResponseDeserializer;
impl CreateChangeSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateChangeSetResponse, XmlParseError> {
        deserialize_elements::<_, CreateChangeSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Id" => {
                        obj.id = Some(ChangeSetIdDeserializer::deserialize("Id", stack)?);
                    }
                    "StackId" => {
                        obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackInstancesRequest {
    /// <p>The names of one or more AWS accounts that you want to create stack instances in the specified region(s) for.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>A list of stack set parameters whose values you want to override in the selected stack instances.</p> <p>Any overridden parameter values will be applied to all stack instances in the specified accounts and regions. When specifying parameters and their values, be aware of how AWS CloudFormation sets parameter values during stack instance operations:</p> <ul> <li> <p>To override the current value for a parameter, include the parameter and specify its value.</p> </li> <li> <p>To leave a parameter set to its present value, you can do one of the following:</p> <ul> <li> <p>Do not include the parameter in the list.</p> </li> <li> <p>Include the parameter and specify <code>UsePreviousValue</code> as <code>true</code>. (You cannot specify both a value and set <code>UsePreviousValue</code> to <code>true</code>.)</p> </li> </ul> </li> <li> <p>To set all overridden parameter back to the values specified in the stack set, specify a parameter list but do not include any parameters.</p> </li> <li> <p>To leave all parameters set to their present values, do not specify this property at all.</p> </li> </ul> <p>During stack set updates, any parameter values overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only override the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template.</p>
    pub parameter_overrides: Option<Vec<Parameter>>,
    /// <p>The names of one or more regions where you want to create stack instances using the specified AWS account(s). </p>
    pub regions: Vec<String>,
    /// <p>The name or unique ID of the stack set that you want to create stack instances from.</p>
    pub stack_set_name: String,
}

/// Serialize `CreateStackInstancesRequest` contents to a `SignedRequest`.
struct CreateStackInstancesRequestSerializer;
impl CreateStackInstancesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackInstancesRequest) {
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
            params.put(&format!("{}{}", prefix, "OperationId"), &field_value);
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
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackInstancesResponse {
    /// <p>The unique identifier for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct CreateStackInstancesResponseDeserializer;
impl CreateStackInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackInstancesResponse, XmlParseError> {
        deserialize_elements::<_, CreateStackInstancesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OperationId" => {
                        obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
/// <p>The input for <a>CreateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackRequest {
    /// <p><p>In some cases, you must explicity acknowledge that your stack template contains certain capabilities in order for AWS CloudFormation to create the stack.</p> <ul> <li> <p> <code>CAPABILITY<em>IAM</code> and <code>CAPABILITY</em>NAMED<em>IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your AWS account; for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY</em>IAM</code> or <code>CAPABILITY<em>NAMED</em>IAM</code> capability.</p> <ul> <li> <p>If you have IAM resources, you can specify either capability. </p> </li> <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY<em>NAMED</em>IAM</code>. </p> </li> <li> <p>If you don&#39;t specify either of these capabilities, AWS CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p> </li> <li> <p> <code>CAPABILITY<em>AUTO</em>EXPAND</code> </p> <p>Some template contain macros. Macros perform custom processing on templates; this can include simple actions like find-and-replace operations, all the way to extensive transformations of entire templates. Because of this, users typically create a change set from the processed template, so that they can review the changes resulting from the macros before actually creating the stack. If your stack template contains one or more macros, and you choose to create a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by AWS CloudFormation.</p> <p>Change sets do not currently support nested stacks. If you want to create a stack from a stack template that contains macros <i>and</i> nested stacks, you must create the stack directly from the template using this capability.</p> <important> <p>You should only create stacks directly from a stack template that contains macros if you know what processing the macro performs.</p> <p>Each macro relies on an underlying Lambda service function for processing stack templates. Be aware that the Lambda function owner can update the function operation without AWS CloudFormation being notified.</p> </important> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using AWS CloudFormation Macros to Perform Custom Processing on Templates</a>.</p> </li> </ul></p>
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
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with for this create stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>. Use the following syntax to describe template resource types: <code>AWS::*</code> (for all AWS resource), <code>Custom::*</code> (for all custom resources), <code>Custom::<i>logical_ID</i> </code> (for a specific custom resource), <code>AWS::<i>service_name</i>::*</code> (for all resources of a particular AWS service), and <code>AWS::<i>service_name</i>::<i>resource_logical_ID</i> </code> (for a specific AWS resource).</p> <p>If the list of resource types doesn't include a resource that you're creating, the stack creation fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a>.</p>
    pub resource_types: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to create the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf. AWS CloudFormation always uses this role for all future operations on the stack. As long as users have permission to operate on the stack, AWS CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    pub rollback_configuration: Option<RollbackConfiguration>,
    /// <p><p>The name that is associated with the stack. The name must be unique in the region in which you are creating the stack.</p> <note> <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.</p> </note></p>
    pub stack_name: String,
    /// <p>Structure containing the stack policy body. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the <i>AWS CloudFormation User Guide</i>. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_body: Option<String>,
    /// <p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_url: Option<String>,
    /// <p>Key-value pairs to associate with this stack. AWS CloudFormation also propagates these tags to the resources created in the stack. A maximum number of 50 tags can be specified.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    pub template_url: Option<String>,
    /// <p>The amount of time that can pass before the stack status becomes CREATE_FAILED; if <code>DisableRollback</code> is not set or is set to <code>false</code>, the stack will be rolled back.</p>
    pub timeout_in_minutes: Option<i64>,
}

/// Serialize `CreateStackRequest` contents to a `SignedRequest`.
struct CreateStackRequestSerializer;
impl CreateStackRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackRequest) {
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
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.disable_rollback {
            params.put(&format!("{}{}", prefix, "DisableRollback"), &field_value);
        }
        if let Some(ref field_value) = obj.enable_termination_protection {
            params.put(
                &format!("{}{}", prefix, "EnableTerminationProtection"),
                &field_value,
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
            params.put(&format!("{}{}", prefix, "OnFailure"), &field_value);
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
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
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
            params.put(&format!("{}{}", prefix, "TimeoutInMinutes"), &field_value);
        }
    }
}

/// <p>The output for a <a>CreateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackResponse {
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
}

struct CreateStackResponseDeserializer;
impl CreateStackResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackResponse, XmlParseError> {
        deserialize_elements::<_, CreateStackResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackSetRequest {
    /// <p>The Amazon Resource Number (ARN) of the IAM role to use to create this stack set. </p> <p>Specify an IAM role only if you are using customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Prerequisites: Granting Permissions for Stack Set Operations</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub administration_role_arn: Option<String>,
    /// <p><p>In some cases, you must explicity acknowledge that your stack set template contains certain capabilities in order for AWS CloudFormation to create the stack set and related stack instances.</p> <ul> <li> <p> <code>CAPABILITY<em>IAM</code> and <code>CAPABILITY</em>NAMED<em>IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your AWS account; for example, by creating new AWS Identity and Access Management (IAM) users. For those stack sets, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY</em>IAM</code> or <code>CAPABILITY<em>NAMED</em>IAM</code> capability.</p> <ul> <li> <p>If you have IAM resources, you can specify either capability. </p> </li> <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY<em>NAMED</em>IAM</code>. </p> </li> <li> <p>If you don&#39;t specify either of these capabilities, AWS CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p> </li> <li> <p> <code>CAPABILITY<em>AUTO</em>EXPAND</code> </p> <p>Some templates contain macros. If your stack template contains one or more macros, and you choose to create a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using AWS CloudFormation Macros to Perform Custom Processing on Templates</a>.</p> <note> <p>Stack sets do not currently support macros in stack templates. (This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by AWS CloudFormation.) Even if you specify this capability, if you include a macro in your template the stack set operation will fail.</p> </note> </li> </ul></p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A unique identifier for this <code>CreateStackSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to create another stack set with the same name. You might retry <code>CreateStackSet</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p>
    pub client_request_token: Option<String>,
    /// <p>A description of the stack set. You can use the description to identify the stack set's purpose or other important information.</p>
    pub description: Option<String>,
    /// <p>The name of the IAM execution role to use to create the stack set. If you do not specify an execution role, AWS CloudFormation uses the <code>AWSCloudFormationStackSetExecutionRole</code> role for the stack set operation.</p> <p>Specify an IAM role only if you are using customized execution roles to control which stack resources users and groups can include in their stack sets. </p>
    pub execution_role_name: Option<String>,
    /// <p>The input parameters for the stack set template. </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p><p>The name to associate with the stack set. The name must be unique in the region where you create your stack set.</p> <note> <p>A stack name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and can&#39;t be longer than 128 characters.</p> </note></p>
    pub stack_set_name: String,
    /// <p>The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the stacks. A maximum number of 50 tags can be specified.</p> <p>If you specify tags as part of a <code>CreateStackSet</code> action, AWS CloudFormation checks to see if you have the required IAM permission to tag resources. If you don't, the entire <code>CreateStackSet</code> action fails with an <code>access denied</code> error, and the stack set is not created.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the TemplateBody or the TemplateURL parameter, but not both.</p>
    pub template_body: Option<String>,
    /// <p>The location of the file that contains the template body. The URL must point to a template (maximum size: 460,800 bytes) that's located in an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify either the TemplateBody or the TemplateURL parameter, but not both.</p>
    pub template_url: Option<String>,
}

/// Serialize `CreateStackSetRequest` contents to a `SignedRequest`.
struct CreateStackSetRequestSerializer;
impl CreateStackSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateStackSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.administration_role_arn {
            params.put(
                &format!("{}{}", prefix, "AdministrationRoleARN"),
                &field_value,
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
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.description {
            params.put(&format!("{}{}", prefix, "Description"), &field_value);
        }
        if let Some(ref field_value) = obj.execution_role_name {
            params.put(&format!("{}{}", prefix, "ExecutionRoleName"), &field_value);
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
            &obj.stack_set_name,
        );
        if let Some(ref field_value) = obj.tags {
            TagsSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateStackSetResponse {
    /// <p>The ID of the stack set that you're creating.</p>
    pub stack_set_id: Option<String>,
}

struct CreateStackSetResponseDeserializer;
impl CreateStackSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateStackSetResponse, XmlParseError> {
        deserialize_elements::<_, CreateStackSetResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct CreationTimeDeserializer;
impl CreationTimeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>DeleteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteChangeSetRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the change set that you want to delete.</p>
    pub change_set_name: String,
    /// <p>If you specified the name of a change set to delete, specify the stack name or ID (ARN) that is associated with it.</p>
    pub stack_name: Option<String>,
}

/// Serialize `DeleteChangeSetRequest` contents to a `SignedRequest`.
struct DeleteChangeSetRequestSerializer;
impl DeleteChangeSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteChangeSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name,
        );
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
    }
}

/// <p>The output for the <a>DeleteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteChangeSetResponse {}

struct DeleteChangeSetResponseDeserializer;
impl DeleteChangeSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteChangeSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteChangeSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackInstancesRequest {
    /// <p>The names of the AWS accounts that you want to delete stack instances for.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You can retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>The regions where you want to delete stack set instances. </p>
    pub regions: Vec<String>,
    /// <p>Removes the stack instances from the specified stack set, but doesn't delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
    pub retain_stacks: bool,
    /// <p>The name or unique ID of the stack set that you want to delete stack instances for.</p>
    pub stack_set_name: String,
}

/// Serialize `DeleteStackInstancesRequest` contents to a `SignedRequest`.
struct DeleteStackInstancesRequestSerializer;
impl DeleteStackInstancesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackInstancesRequest) {
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
            params.put(&format!("{}{}", prefix, "OperationId"), &field_value);
        }
        if let Some(ref field_value) = obj.operation_preferences {
            StackSetOperationPreferencesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OperationPreferences"),
                field_value,
            );
        }
        RegionListSerializer::serialize(params, &format!("{}{}", prefix, "Regions"), &obj.regions);
        params.put(&format!("{}{}", prefix, "RetainStacks"), &obj.retain_stacks);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackInstancesResponse {
    /// <p>The unique identifier for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct DeleteStackInstancesResponseDeserializer;
impl DeleteStackInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteStackInstancesResponse, XmlParseError> {
        deserialize_elements::<_, DeleteStackInstancesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OperationId" => {
                        obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
/// <p>The input for <a>DeleteStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackRequest {
    /// <p>A unique identifier for this <code>DeleteStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to delete a stack with the same name. You might retry <code>DeleteStack</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>For stacks in the <code>DELETE_FAILED</code> state, a list of resource logical IDs that are associated with the resources you want to retain. During deletion, AWS CloudFormation deletes the stack but does not delete the retained resources.</p> <p>Retaining resources is useful when you cannot delete a resource, such as a non-empty S3 bucket, but you want to delete the stack.</p>
    pub retain_resources: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity and Access Management (IAM) role that AWS CloudFormation assumes to delete the stack. AWS CloudFormation uses the role's credentials to make calls on your behalf.</p> <p>If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.</p>
    pub role_arn: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack.</p>
    pub stack_name: String,
}

/// Serialize `DeleteStackRequest` contents to a `SignedRequest`.
struct DeleteStackRequestSerializer;
impl DeleteStackRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.retain_resources {
            RetainResourcesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RetainResources"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.role_arn {
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackResponse {}

struct DeleteStackResponseDeserializer;
impl DeleteStackResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteStackResponse, XmlParseError> {
        Ok(DeleteStackResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackSetRequest {
    /// <p>The name or unique ID of the stack set that you're deleting. You can obtain this value by running <a>ListStackSets</a>.</p>
    pub stack_set_name: String,
}

/// Serialize `DeleteStackSetRequest` contents to a `SignedRequest`.
struct DeleteStackSetRequestSerializer;
impl DeleteStackSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteStackSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStackSetResponse {}

struct DeleteStackSetResponseDeserializer;
impl DeleteStackSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteStackSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteStackSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DeletionTimeDeserializer;
impl DeletionTimeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>DescribeAccountLimits</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsRequest {
    /// <p>A string that identifies the next page of limits that you want to retrieve.</p>
    pub next_token: Option<String>,
}

/// Serialize `DescribeAccountLimitsRequest` contents to a `SignedRequest`.
struct DescribeAccountLimitsRequestSerializer;
impl DescribeAccountLimitsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAccountLimitsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

/// <p>The output for the <a>DescribeAccountLimits</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAccountLimitsResponse {
    /// <p>An account limit structure that contain a list of AWS CloudFormation account limits and their values.</p>
    pub account_limits: Option<Vec<AccountLimit>>,
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of limits. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
}

struct DescribeAccountLimitsResponseDeserializer;
impl DescribeAccountLimitsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAccountLimitsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAccountLimitsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AccountLimits" => {
                        obj.account_limits.get_or_insert(vec![]).extend(
                            AccountLimitListDeserializer::deserialize("AccountLimits", stack)?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The input for the <a>DescribeChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeChangeSetRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the change set that you want to describe.</p>
    pub change_set_name: String,
    /// <p>A string (provided by the <a>DescribeChangeSet</a> response output) that identifies the next page of information that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>If you specified the name of a change set, specify the stack name or ID (ARN) of the change set you want to describe.</p>
    pub stack_name: Option<String>,
}

/// Serialize `DescribeChangeSetRequest` contents to a `SignedRequest`.
struct DescribeChangeSetRequestSerializer;
impl DescribeChangeSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeChangeSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name,
        );
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
    }
}

/// <p>The output for the <a>DescribeChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeChangeSetResponse {
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
    /// <p>A list of <code>Parameter</code> structures that describes the input parameters and their values used to create the change set. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
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

struct DescribeChangeSetResponseDeserializer;
impl DescribeChangeSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeChangeSetResponse, XmlParseError> {
        deserialize_elements::<_, DescribeChangeSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Capabilities" => {
                        obj.capabilities.get_or_insert(vec![]).extend(
                            CapabilitiesDeserializer::deserialize("Capabilities", stack)?,
                        );
                    }
                    "ChangeSetId" => {
                        obj.change_set_id =
                            Some(ChangeSetIdDeserializer::deserialize("ChangeSetId", stack)?);
                    }
                    "ChangeSetName" => {
                        obj.change_set_name = Some(ChangeSetNameDeserializer::deserialize(
                            "ChangeSetName",
                            stack,
                        )?);
                    }
                    "Changes" => {
                        obj.changes
                            .get_or_insert(vec![])
                            .extend(ChangesDeserializer::deserialize("Changes", stack)?);
                    }
                    "CreationTime" => {
                        obj.creation_time = Some(CreationTimeDeserializer::deserialize(
                            "CreationTime",
                            stack,
                        )?);
                    }
                    "Description" => {
                        obj.description =
                            Some(DescriptionDeserializer::deserialize("Description", stack)?);
                    }
                    "ExecutionStatus" => {
                        obj.execution_status = Some(ExecutionStatusDeserializer::deserialize(
                            "ExecutionStatus",
                            stack,
                        )?);
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "NotificationARNs" => {
                        obj.notification_ar_ns.get_or_insert(vec![]).extend(
                            NotificationARNsDeserializer::deserialize("NotificationARNs", stack)?,
                        );
                    }
                    "Parameters" => {
                        obj.parameters
                            .get_or_insert(vec![])
                            .extend(ParametersDeserializer::deserialize("Parameters", stack)?);
                    }
                    "RollbackConfiguration" => {
                        obj.rollback_configuration =
                            Some(RollbackConfigurationDeserializer::deserialize(
                                "RollbackConfiguration",
                                stack,
                            )?);
                    }
                    "StackId" => {
                        obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                    }
                    "StackName" => {
                        obj.stack_name =
                            Some(StackNameDeserializer::deserialize("StackName", stack)?);
                    }
                    "Status" => {
                        obj.status =
                            Some(ChangeSetStatusDeserializer::deserialize("Status", stack)?);
                    }
                    "StatusReason" => {
                        obj.status_reason = Some(ChangeSetStatusReasonDeserializer::deserialize(
                            "StatusReason",
                            stack,
                        )?);
                    }
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagsDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackDriftDetectionStatusRequest {
    /// <p>The ID of the drift detection results of this operation. </p> <p>AWS CloudFormation generates new results, with a new drift detection ID, each time this operation is run. However, the number of drift results AWS CloudFormation retains for any given stack, and for how long, may vary. </p>
    pub stack_drift_detection_id: String,
}

/// Serialize `DescribeStackDriftDetectionStatusRequest` contents to a `SignedRequest`.
struct DescribeStackDriftDetectionStatusRequestSerializer;
impl DescribeStackDriftDetectionStatusRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackDriftDetectionStatusRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackDriftDetectionId"),
            &obj.stack_drift_detection_id,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackDriftDetectionStatusResponse {
    /// <p><p>The status of the stack drift detection operation.</p> <ul> <li> <p> <code>DETECTION<em>COMPLETE</code>: The stack drift detection operation has successfully completed for all resources in the stack that support drift detection. (Resources that do not currently support stack detection remain unchecked.)</p> <p>If you specified logical resource IDs for AWS CloudFormation to use as a filter for the stack drift detection operation, only the resources with those logical IDs are checked for drift.</p> </li> <li> <p> <code>DETECTION</em>FAILED</code>: The stack drift detection operation has failed for at least one resource in the stack. Results will be available for resources on which AWS CloudFormation successfully completed drift detection.</p> </li> <li> <p> <code>DETECTION<em>IN</em>PROGRESS</code>: The stack drift detection operation is currently in progress.</p> </li> </ul></p>
    pub detection_status: String,
    /// <p>The reason the stack drift detection operation has its current status.</p>
    pub detection_status_reason: Option<String>,
    /// <p>Total number of stack resources that have drifted. This is NULL until the drift detection operation reaches a status of <code>DETECTION_COMPLETE</code>. This value will be 0 for stacks whose drift status is <code>IN_SYNC</code>.</p>
    pub drifted_stack_resource_count: Option<i64>,
    /// <p>The ID of the drift detection results of this operation. </p> <p>AWS CloudFormation generates new results, with a new drift detection ID, each time this operation is run. However, the number of reports AWS CloudFormation retains for any given stack, and for how long, may vary.</p>
    pub stack_drift_detection_id: String,
    /// <p><p>Status of the stack&#39;s actual configuration compared to its expected configuration. </p> <ul> <li> <p> <code>DRIFTED</code>: The stack differs from its expected template configuration. A stack is considered to have drifted if one or more of its resources have drifted.</p> </li> <li> <p> <code>NOT<em>CHECKED</code>: AWS CloudFormation has not checked if the stack differs from its expected template configuration.</p> </li> <li> <p> <code>IN</em>SYNC</code>: The stack&#39;s actual configuration matches its expected template configuration.</p> </li> <li> <p> <code>UNKNOWN</code>: This value is reserved for future use.</p> </li> </ul></p>
    pub stack_drift_status: Option<String>,
    /// <p>The ID of the stack.</p>
    pub stack_id: String,
    /// <p>Time at which the stack drift detection operation was initiated.</p>
    pub timestamp: String,
}

struct DescribeStackDriftDetectionStatusResponseDeserializer;
impl DescribeStackDriftDetectionStatusResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackDriftDetectionStatusResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackDriftDetectionStatusResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DetectionStatus" => {
                        obj.detection_status = StackDriftDetectionStatusDeserializer::deserialize(
                            "DetectionStatus",
                            stack,
                        )?;
                    }
                    "DetectionStatusReason" => {
                        obj.detection_status_reason =
                            Some(StackDriftDetectionStatusReasonDeserializer::deserialize(
                                "DetectionStatusReason",
                                stack,
                            )?);
                    }
                    "DriftedStackResourceCount" => {
                        obj.drifted_stack_resource_count =
                            Some(BoxedIntegerDeserializer::deserialize(
                                "DriftedStackResourceCount",
                                stack,
                            )?);
                    }
                    "StackDriftDetectionId" => {
                        obj.stack_drift_detection_id =
                            StackDriftDetectionIdDeserializer::deserialize(
                                "StackDriftDetectionId",
                                stack,
                            )?;
                    }
                    "StackDriftStatus" => {
                        obj.stack_drift_status = Some(StackDriftStatusDeserializer::deserialize(
                            "StackDriftStatus",
                            stack,
                        )?);
                    }
                    "StackId" => {
                        obj.stack_id = StackIdDeserializer::deserialize("StackId", stack)?;
                    }
                    "Timestamp" => {
                        obj.timestamp = TimestampDeserializer::deserialize("Timestamp", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The input for <a>DescribeStackEvents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackEventsRequest {
    /// <p>A string that identifies the next page of events that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: Option<String>,
}

/// Serialize `DescribeStackEventsRequest` contents to a `SignedRequest`.
struct DescribeStackEventsRequestSerializer;
impl DescribeStackEventsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackEventsRequest) {
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

/// <p>The output for a <a>DescribeStackEvents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackEventsResponse {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of events. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackEvents</code> structures.</p>
    pub stack_events: Option<Vec<StackEvent>>,
}

struct DescribeStackEventsResponseDeserializer;
impl DescribeStackEventsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackEventsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackEventsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "StackEvents" => {
                        obj.stack_events
                            .get_or_insert(vec![])
                            .extend(StackEventsDeserializer::deserialize("StackEvents", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackInstanceRequest {
    /// <p>The ID of an AWS account that's associated with this stack instance.</p>
    pub stack_instance_account: String,
    /// <p>The name of a region that's associated with this stack instance.</p>
    pub stack_instance_region: String,
    /// <p>The name or the unique stack ID of the stack set that you want to get stack instance information for.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackInstanceRequest` contents to a `SignedRequest`.
struct DescribeStackInstanceRequestSerializer;
impl DescribeStackInstanceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackInstanceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackInstanceAccount"),
            &obj.stack_instance_account,
        );
        params.put(
            &format!("{}{}", prefix, "StackInstanceRegion"),
            &obj.stack_instance_region,
        );
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackInstanceResponse {
    /// <p>The stack instance that matches the specified request parameters.</p>
    pub stack_instance: Option<StackInstance>,
}

struct DescribeStackInstanceResponseDeserializer;
impl DescribeStackInstanceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackInstanceResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackInstanceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackInstance" => {
                        obj.stack_instance = Some(StackInstanceDeserializer::deserialize(
                            "StackInstance",
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
pub struct DescribeStackResourceDriftsRequest {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>A string that identifies the next page of stack resource drift results.</p>
    pub next_token: Option<String>,
    /// <p>The name of the stack for which you want drift information.</p>
    pub stack_name: String,
    /// <p><p>The resource drift status values to use as filters for the resource drift results returned.</p> <ul> <li> <p> <code>DELETED</code>: The resource differs from its expected template configuration in that the resource has been deleted.</p> </li> <li> <p> <code>MODIFIED</code>: One or more resource properties differ from their expected template values.</p> </li> <li> <p> <code>IN<em>SYNC</code>: The resources&#39;s actual configuration matches its expected template configuration.</p> </li> <li> <p> <code>NOT</em>CHECKED</code>: AWS CloudFormation does not currently return this value.</p> </li> </ul></p>
    pub stack_resource_drift_status_filters: Option<Vec<String>>,
}

/// Serialize `DescribeStackResourceDriftsRequest` contents to a `SignedRequest`.
struct DescribeStackResourceDriftsRequestSerializer;
impl DescribeStackResourceDriftsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackResourceDriftsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.stack_resource_drift_status_filters {
            StackResourceDriftStatusFiltersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StackResourceDriftStatusFilters"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourceDriftsResponse {
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>DescribeStackResourceDrifts</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>Drift information for the resources that have been checked for drift in the specified stack. This includes actual and expected configuration values for resources where AWS CloudFormation detects drift.</p> <p>For a given stack, there will be one <code>StackResourceDrift</code> for each stack resource that has been checked for drift. Resources that have not yet been checked for drift are not included. Resources that do not currently support drift detection are not checked, and so not included. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p>
    pub stack_resource_drifts: Vec<StackResourceDrift>,
}

struct DescribeStackResourceDriftsResponseDeserializer;
impl DescribeStackResourceDriftsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackResourceDriftsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackResourceDriftsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "StackResourceDrifts" => {
                        obj.stack_resource_drifts.extend(
                            StackResourceDriftsDeserializer::deserialize(
                                "StackResourceDrifts",
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
/// <p>The input for <a>DescribeStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourceRequest {
    /// <p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>
    pub logical_resource_id: String,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: String,
}

/// Serialize `DescribeStackResourceRequest` contents to a `SignedRequest`.
struct DescribeStackResourceRequestSerializer;
impl DescribeStackResourceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LogicalResourceId"),
            &obj.logical_resource_id,
        );
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

/// <p>The output for a <a>DescribeStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourceResponse {
    /// <p>A <code>StackResourceDetail</code> structure containing the description of the specified resource in the specified stack.</p>
    pub stack_resource_detail: Option<StackResourceDetail>,
}

struct DescribeStackResourceResponseDeserializer;
impl DescribeStackResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackResourceResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackResourceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackResourceDetail" => {
                        obj.stack_resource_detail =
                            Some(StackResourceDetailDeserializer::deserialize(
                                "StackResourceDetail",
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
/// <p>The input for <a>DescribeStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourcesRequest {
    /// <p>The logical name of the resource as specified in the template.</p> <p>Default: There is no default value.</p>
    pub logical_resource_id: Option<String>,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p> <p>For example, for an Amazon Elastic Compute Cloud (EC2) instance, <code>PhysicalResourceId</code> corresponds to the <code>InstanceId</code>. You can pass the EC2 <code>InstanceId</code> to <code>DescribeStackResources</code> to find which stack the instance belongs to and what other resources are part of the stack.</p> <p>Required: Conditional. If you do not specify <code>PhysicalResourceId</code>, you must specify <code>StackName</code>.</p> <p>Default: There is no default value.</p>
    pub physical_resource_id: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p> <p>Required: Conditional. If you do not specify <code>StackName</code>, you must specify <code>PhysicalResourceId</code>.</p>
    pub stack_name: Option<String>,
}

/// Serialize `DescribeStackResourcesRequest` contents to a `SignedRequest`.
struct DescribeStackResourcesRequestSerializer;
impl DescribeStackResourcesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackResourcesRequest) {
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

/// <p>The output for a <a>DescribeStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackResourcesResponse {
    /// <p>A list of <code>StackResource</code> structures.</p>
    pub stack_resources: Option<Vec<StackResource>>,
}

struct DescribeStackResourcesResponseDeserializer;
impl DescribeStackResourcesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackResourcesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackResourcesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackResources" => {
                        obj.stack_resources.get_or_insert(vec![]).extend(
                            StackResourcesDeserializer::deserialize("StackResources", stack)?,
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
pub struct DescribeStackSetOperationRequest {
    /// <p>The unique ID of the stack set operation. </p>
    pub operation_id: String,
    /// <p>The name or the unique stack ID of the stack set for the stack operation.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackSetOperationRequest` contents to a `SignedRequest`.
struct DescribeStackSetOperationRequestSerializer;
impl DescribeStackSetOperationRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackSetOperationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "OperationId"), &obj.operation_id);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackSetOperationResponse {
    /// <p>The specified stack set operation.</p>
    pub stack_set_operation: Option<StackSetOperation>,
}

struct DescribeStackSetOperationResponseDeserializer;
impl DescribeStackSetOperationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackSetOperationResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackSetOperationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackSetOperation" => {
                        obj.stack_set_operation = Some(StackSetOperationDeserializer::deserialize(
                            "StackSetOperation",
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
pub struct DescribeStackSetRequest {
    /// <p>The name or unique ID of the stack set whose description you want.</p>
    pub stack_set_name: String,
}

/// Serialize `DescribeStackSetRequest` contents to a `SignedRequest`.
struct DescribeStackSetRequestSerializer;
impl DescribeStackSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStackSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStackSetResponse {
    /// <p>The specified stack set.</p>
    pub stack_set: Option<StackSet>,
}

struct DescribeStackSetResponseDeserializer;
impl DescribeStackSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStackSetResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStackSetResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackSet" => {
                        obj.stack_set = Some(StackSetDeserializer::deserialize("StackSet", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The input for <a>DescribeStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStacksRequest {
    /// <p>A string that identifies the next page of stacks that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: Option<String>,
}

/// Serialize `DescribeStacksRequest` contents to a `SignedRequest`.
struct DescribeStacksRequestSerializer;
impl DescribeStacksRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeStacksRequest) {
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

/// <p>The output for a <a>DescribeStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeStacksResponse {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of stack structures.</p>
    pub stacks: Option<Vec<Stack>>,
}

struct DescribeStacksResponseDeserializer;
impl DescribeStacksResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeStacksResponse, XmlParseError> {
        deserialize_elements::<_, DescribeStacksResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "Stacks" => {
                    obj.stacks
                        .get_or_insert(vec![])
                        .extend(StacksDeserializer::deserialize("Stacks", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DescriptionDeserializer;
impl DescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectStackDriftRequest {
    /// <p>The logical names of any resources you want to use as filters.</p>
    pub logical_resource_ids: Option<Vec<String>>,
    /// <p>The name of the stack for which you want to detect drift. </p>
    pub stack_name: String,
}

/// Serialize `DetectStackDriftRequest` contents to a `SignedRequest`.
struct DetectStackDriftRequestSerializer;
impl DetectStackDriftRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetectStackDriftRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.logical_resource_ids {
            LogicalResourceIdsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "LogicalResourceIds"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectStackDriftResponse {
    /// <p>The ID of the drift detection results of this operation. </p> <p>AWS CloudFormation generates new results, with a new drift detection ID, each time this operation is run. However, the number of drift results AWS CloudFormation retains for any given stack, and for how long, may vary. </p>
    pub stack_drift_detection_id: String,
}

struct DetectStackDriftResponseDeserializer;
impl DetectStackDriftResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetectStackDriftResponse, XmlParseError> {
        deserialize_elements::<_, DetectStackDriftResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackDriftDetectionId" => {
                        obj.stack_drift_detection_id =
                            StackDriftDetectionIdDeserializer::deserialize(
                                "StackDriftDetectionId",
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectStackResourceDriftRequest {
    /// <p>The logical name of the resource for which to return drift information.</p>
    pub logical_resource_id: String,
    /// <p>The name of the stack to which the resource belongs.</p>
    pub stack_name: String,
}

/// Serialize `DetectStackResourceDriftRequest` contents to a `SignedRequest`.
struct DetectStackResourceDriftRequestSerializer;
impl DetectStackResourceDriftRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DetectStackResourceDriftRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LogicalResourceId"),
            &obj.logical_resource_id,
        );
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DetectStackResourceDriftResponse {
    /// <p>Information about whether the resource's actual configuration has drifted from its expected template configuration, including actual and expected property values and any differences detected.</p>
    pub stack_resource_drift: StackResourceDrift,
}

struct DetectStackResourceDriftResponseDeserializer;
impl DetectStackResourceDriftResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DetectStackResourceDriftResponse, XmlParseError> {
        deserialize_elements::<_, DetectStackResourceDriftResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackResourceDrift" => {
                        obj.stack_resource_drift = StackResourceDriftDeserializer::deserialize(
                            "StackResourceDrift",
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
struct DifferenceTypeDeserializer;
impl DifferenceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DisableRollbackDeserializer;
impl DisableRollbackDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct EnableTerminationProtectionDeserializer;
impl EnableTerminationProtectionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for an <a>EstimateTemplateCost</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EstimateTemplateCostRequest {
    /// <p>A list of <code>Parameter</code> structures that specify input parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must pass <code>TemplateBody</code> or <code>TemplateURL</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_url: Option<String>,
}

/// Serialize `EstimateTemplateCostRequest` contents to a `SignedRequest`.
struct EstimateTemplateCostRequestSerializer;
impl EstimateTemplateCostRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &EstimateTemplateCostRequest) {
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
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
    }
}

/// <p>The output for a <a>EstimateTemplateCost</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EstimateTemplateCostResponse {
    /// <p>An AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    pub url: Option<String>,
}

struct EstimateTemplateCostResponseDeserializer;
impl EstimateTemplateCostResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EstimateTemplateCostResponse, XmlParseError> {
        deserialize_elements::<_, EstimateTemplateCostResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Url" => {
                        obj.url = Some(UrlDeserializer::deserialize("Url", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct EvaluationTypeDeserializer;
impl EvaluationTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct EventIdDeserializer;
impl EventIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>ExecuteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExecuteChangeSetRequest {
    /// <p>The name or ARN of the change set that you want use to update the specified stack.</p>
    pub change_set_name: String,
    /// <p>A unique identifier for this <code>ExecuteChangeSet</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to execute a change set to update a stack with the same name. You might retry <code>ExecuteChangeSet</code> requests to ensure that AWS CloudFormation successfully received them.</p>
    pub client_request_token: Option<String>,
    /// <p>If you specified the name of a change set, specify the stack name or ID (ARN) that is associated with the change set you want to execute.</p>
    pub stack_name: Option<String>,
}

/// Serialize `ExecuteChangeSetRequest` contents to a `SignedRequest`.
struct ExecuteChangeSetRequestSerializer;
impl ExecuteChangeSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ExecuteChangeSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ChangeSetName"),
            &obj.change_set_name,
        );
        if let Some(ref field_value) = obj.client_request_token {
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
    }
}

/// <p>The output for the <a>ExecuteChangeSet</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExecuteChangeSetResponse {}

struct ExecuteChangeSetResponseDeserializer;
impl ExecuteChangeSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ExecuteChangeSetResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = ExecuteChangeSetResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ExecutionRoleNameDeserializer;
impl ExecutionRoleNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ExecutionStatusDeserializer;
impl ExecutionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Export, XmlParseError> {
        deserialize_elements::<_, Export, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ExportingStackId" => {
                    obj.exporting_stack_id =
                        Some(StackIdDeserializer::deserialize("ExportingStackId", stack)?);
                }
                "Name" => {
                    obj.name = Some(ExportNameDeserializer::deserialize("Name", stack)?);
                }
                "Value" => {
                    obj.value = Some(ExportValueDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ExportNameDeserializer;
impl ExportNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ExportValueDeserializer;
impl ExportValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ExportsDeserializer;
impl ExportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Export>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ExportDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct FailureToleranceCountDeserializer;
impl FailureToleranceCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct FailureTolerancePercentageDeserializer;
impl FailureTolerancePercentageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>GetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStackPolicyRequest {
    /// <p>The name or unique stack ID that is associated with the stack whose policy you want to get.</p>
    pub stack_name: String,
}

/// Serialize `GetStackPolicyRequest` contents to a `SignedRequest`.
struct GetStackPolicyRequestSerializer;
impl GetStackPolicyRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetStackPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

/// <p>The output for the <a>GetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetStackPolicyResponse {
    /// <p>Structure containing the stack policy body. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide.)</p>
    pub stack_policy_body: Option<String>,
}

struct GetStackPolicyResponseDeserializer;
impl GetStackPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetStackPolicyResponse, XmlParseError> {
        deserialize_elements::<_, GetStackPolicyResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "StackPolicyBody" => {
                    obj.stack_policy_body = Some(StackPolicyBodyDeserializer::deserialize(
                        "StackPolicyBody",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The input for a <a>GetTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateRequest {
    /// <p>The name or Amazon Resource Name (ARN) of a change set for which AWS CloudFormation returns the associated template. If you specify a name, you must also specify the <code>StackName</code>.</p>
    pub change_set_name: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: Option<String>,
    /// <p>For templates that include transforms, the stage of the template that AWS CloudFormation returns. To get the user-submitted template, specify <code>Original</code>. To get the template after AWS CloudFormation has processed all transforms, specify <code>Processed</code>. </p> <p>If the template doesn't include transforms, <code>Original</code> and <code>Processed</code> return the same template. By default, AWS CloudFormation specifies <code>Original</code>. </p>
    pub template_stage: Option<String>,
}

/// Serialize `GetTemplateRequest` contents to a `SignedRequest`.
struct GetTemplateRequestSerializer;
impl GetTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTemplateRequest) {
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

/// <p>The output for <a>GetTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateResponse {
    /// <p>The stage of the template that you can retrieve. For stacks, the <code>Original</code> and <code>Processed</code> templates are always available. For change sets, the <code>Original</code> template is always available. After AWS CloudFormation finishes creating the change set, the <code>Processed</code> template becomes available.</p>
    pub stages_available: Option<Vec<String>>,
    /// <p>Structure containing the template body. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>AWS CloudFormation returns the same template that was used when the stack was created.</p>
    pub template_body: Option<String>,
}

struct GetTemplateResponseDeserializer;
impl GetTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateResponse, XmlParseError> {
        deserialize_elements::<_, GetTemplateResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "StagesAvailable" => {
                    obj.stages_available.get_or_insert(vec![]).extend(
                        StageListDeserializer::deserialize("StagesAvailable", stack)?,
                    );
                }
                "TemplateBody" => {
                    obj.template_body = Some(TemplateBodyDeserializer::deserialize(
                        "TemplateBody",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The input for the <a>GetTemplateSummary</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateSummaryRequest {
    /// <p>The name or the stack ID that is associated with the stack, which are not always interchangeable. For running stacks, you can specify either the stack's name or its unique stack ID. For deleted stack, you must specify the unique stack ID.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub stack_name: Option<String>,
    /// <p>The name or unique ID of the stack set from which the stack was created.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub stack_set_name: Option<String>,
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information about templates, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information about templates, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>StackName</code>, <code>StackSetName</code>, <code>TemplateBody</code>, or <code>TemplateURL</code>.</p>
    pub template_url: Option<String>,
}

/// Serialize `GetTemplateSummaryRequest` contents to a `SignedRequest`.
struct GetTemplateSummaryRequestSerializer;
impl GetTemplateSummaryRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &GetTemplateSummaryRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.stack_name {
            params.put(&format!("{}{}", prefix, "StackName"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_set_name {
            params.put(&format!("{}{}", prefix, "StackSetName"), &field_value);
        }
        if let Some(ref field_value) = obj.template_body {
            params.put(&format!("{}{}", prefix, "TemplateBody"), &field_value);
        }
        if let Some(ref field_value) = obj.template_url {
            params.put(&format!("{}{}", prefix, "TemplateURL"), &field_value);
        }
    }
}

/// <p>The output for the <a>GetTemplateSummary</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetTemplateSummaryResponse {
    /// <p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
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

struct GetTemplateSummaryResponseDeserializer;
impl GetTemplateSummaryResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetTemplateSummaryResponse, XmlParseError> {
        deserialize_elements::<_, GetTemplateSummaryResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Capabilities" => {
                        obj.capabilities.get_or_insert(vec![]).extend(
                            CapabilitiesDeserializer::deserialize("Capabilities", stack)?,
                        );
                    }
                    "CapabilitiesReason" => {
                        obj.capabilities_reason =
                            Some(CapabilitiesReasonDeserializer::deserialize(
                                "CapabilitiesReason",
                                stack,
                            )?);
                    }
                    "DeclaredTransforms" => {
                        obj.declared_transforms.get_or_insert(vec![]).extend(
                            TransformsListDeserializer::deserialize("DeclaredTransforms", stack)?,
                        );
                    }
                    "Description" => {
                        obj.description =
                            Some(DescriptionDeserializer::deserialize("Description", stack)?);
                    }
                    "Metadata" => {
                        obj.metadata = Some(MetadataDeserializer::deserialize("Metadata", stack)?);
                    }
                    "Parameters" => {
                        obj.parameters.get_or_insert(vec![]).extend(
                            ParameterDeclarationsDeserializer::deserialize("Parameters", stack)?,
                        );
                    }
                    "ResourceTypes" => {
                        obj.resource_types.get_or_insert(vec![]).extend(
                            ResourceTypesDeserializer::deserialize("ResourceTypes", stack)?,
                        );
                    }
                    "Version" => {
                        obj.version = Some(VersionDeserializer::deserialize("Version", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct ImportsDeserializer;
impl ImportsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackNameDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct KeyDeserializer;
impl KeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LastUpdatedTimeDeserializer;
impl LastUpdatedTimeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LimitNameDeserializer;
impl LimitNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LimitValueDeserializer;
impl LimitValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for the <a>ListChangeSets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListChangeSetsRequest {
    /// <p>A string (provided by the <a>ListChangeSets</a> response output) that identifies the next page of change sets that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the Amazon Resource Name (ARN) of the stack for which you want to list change sets.</p>
    pub stack_name: String,
}

/// Serialize `ListChangeSetsRequest` contents to a `SignedRequest`.
struct ListChangeSetsRequestSerializer;
impl ListChangeSetsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListChangeSetsRequest) {
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

/// <p>The output for the <a>ListChangeSets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListChangeSetsResponse {
    /// <p>If the output exceeds 1 MB, a string that identifies the next page of change sets. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>ChangeSetSummary</code> structures that provides the ID and status of each change set for the specified stack.</p>
    pub summaries: Option<Vec<ChangeSetSummary>>,
}

struct ListChangeSetsResponseDeserializer;
impl ListChangeSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListChangeSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListChangeSetsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "Summaries" => {
                    obj.summaries.get_or_insert(vec![]).extend(
                        ChangeSetSummariesDeserializer::deserialize("Summaries", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListExportsRequest {
    /// <p>A string (provided by the <a>ListExports</a> response output) that identifies the next page of exported output values that you asked to retrieve.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListExportsRequest` contents to a `SignedRequest`.
struct ListExportsRequestSerializer;
impl ListExportsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListExportsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListExportsResponse {
    /// <p>The output for the <a>ListExports</a> action.</p>
    pub exports: Option<Vec<Export>>,
    /// <p>If the output exceeds 100 exported output values, a string that identifies the next page of exports. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
}

struct ListExportsResponseDeserializer;
impl ListExportsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListExportsResponse, XmlParseError> {
        deserialize_elements::<_, ListExportsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Exports" => {
                    obj.exports
                        .get_or_insert(vec![])
                        .extend(ExportsDeserializer::deserialize("Exports", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListImportsRequest {
    /// <p>The name of the exported output value. AWS CloudFormation returns the stack names that are importing this value. </p>
    pub export_name: String,
    /// <p>A string (provided by the <a>ListImports</a> response output) that identifies the next page of stacks that are importing the specified exported output value. </p>
    pub next_token: Option<String>,
}

/// Serialize `ListImportsRequest` contents to a `SignedRequest`.
struct ListImportsRequestSerializer;
impl ListImportsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListImportsRequest) {
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListImportsResponse {
    /// <p>A list of stack names that are importing the specified exported output value. </p>
    pub imports: Option<Vec<String>>,
    /// <p>A string that identifies the next page of exports. If there is no additional page, this value is null.</p>
    pub next_token: Option<String>,
}

struct ListImportsResponseDeserializer;
impl ListImportsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListImportsResponse, XmlParseError> {
        deserialize_elements::<_, ListImportsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Imports" => {
                    obj.imports
                        .get_or_insert(vec![])
                        .extend(ImportsDeserializer::deserialize("Imports", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackInstancesRequest {
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

/// Serialize `ListStackInstancesRequest` contents to a `SignedRequest`.
struct ListStackInstancesRequestSerializer;
impl ListStackInstancesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackInstancesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_instance_account {
            params.put(
                &format!("{}{}", prefix, "StackInstanceAccount"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.stack_instance_region {
            params.put(
                &format!("{}{}", prefix, "StackInstanceRegion"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackInstancesResponse {
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackInstanceSummary</code> structures that contain information about the specified stack instances.</p>
    pub summaries: Option<Vec<StackInstanceSummary>>,
}

struct ListStackInstancesResponseDeserializer;
impl ListStackInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackInstancesResponse, XmlParseError> {
        deserialize_elements::<_, ListStackInstancesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "Summaries" => {
                        obj.summaries.get_or_insert(vec![]).extend(
                            StackInstanceSummariesDeserializer::deserialize("Summaries", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The input for the <a>ListStackResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackResourcesRequest {
    /// <p>A string that identifies the next page of stack resources that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>The name or the unique stack ID that is associated with the stack, which are not always interchangeable:</p> <ul> <li> <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p> </li> <li> <p>Deleted stacks: You must specify the unique stack ID.</p> </li> </ul> <p>Default: There is no default value.</p>
    pub stack_name: String,
}

/// Serialize `ListStackResourcesRequest` contents to a `SignedRequest`.
struct ListStackResourcesRequestSerializer;
impl ListStackResourcesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackResourcesRequest) {
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

/// <p>The output for a <a>ListStackResources</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackResourcesResponse {
    /// <p>If the output exceeds 1 MB, a string that identifies the next page of stack resources. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackResourceSummary</code> structures.</p>
    pub stack_resource_summaries: Option<Vec<StackResourceSummary>>,
}

struct ListStackResourcesResponseDeserializer;
impl ListStackResourcesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackResourcesResponse, XmlParseError> {
        deserialize_elements::<_, ListStackResourcesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "StackResourceSummaries" => {
                        obj.stack_resource_summaries.get_or_insert(vec![]).extend(
                            StackResourceSummariesDeserializer::deserialize(
                                "StackResourceSummaries",
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
pub struct ListStackSetOperationResultsRequest {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSetOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The ID of the stack set operation.</p>
    pub operation_id: String,
    /// <p>The name or unique ID of the stack set that you want to get operation results for.</p>
    pub stack_set_name: String,
}

/// Serialize `ListStackSetOperationResultsRequest` contents to a `SignedRequest`.
struct ListStackSetOperationResultsRequestSerializer;
impl ListStackSetOperationResultsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetOperationResultsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "OperationId"), &obj.operation_id);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetOperationResultsResponse {
    /// <p>If the request doesn't return all results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetOperationResultSummary</code> structures that contain information about the specified operation results, for accounts and regions that are included in the operation.</p>
    pub summaries: Option<Vec<StackSetOperationResultSummary>>,
}

struct ListStackSetOperationResultsResponseDeserializer;
impl ListStackSetOperationResultsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetOperationResultsResponse, XmlParseError> {
        deserialize_elements::<_, ListStackSetOperationResultsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "Summaries" => {
                        obj.summaries.get_or_insert(vec![]).extend(
                            StackSetOperationResultSummariesDeserializer::deserialize(
                                "Summaries",
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
pub struct ListStackSetOperationsRequest {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSetOperations</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The name or unique ID of the stack set that you want to get operation summaries for.</p>
    pub stack_set_name: String,
}

/// Serialize `ListStackSetOperationsRequest` contents to a `SignedRequest`.
struct ListStackSetOperationsRequestSerializer;
impl ListStackSetOperationsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetOperationsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetOperationsResponse {
    /// <p>If the request doesn't return all results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListOperationResults</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetOperationSummary</code> structures that contain summary information about operations for the specified stack set.</p>
    pub summaries: Option<Vec<StackSetOperationSummary>>,
}

struct ListStackSetOperationsResponseDeserializer;
impl ListStackSetOperationsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetOperationsResponse, XmlParseError> {
        deserialize_elements::<_, ListStackSetOperationsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    "Summaries" => {
                        obj.summaries.get_or_insert(vec![]).extend(
                            StackSetOperationSummariesDeserializer::deserialize(
                                "Summaries",
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
pub struct ListStackSetsRequest {
    /// <p>The maximum number of results to be returned with a single call. If the number of available results exceeds this maximum, the response includes a <code>NextToken</code> value that you can assign to the <code>NextToken</code> request parameter to get the next set of results.</p>
    pub max_results: Option<i64>,
    /// <p>If the previous paginated request didn't return all of the remaining results, the response object's <code>NextToken</code> parameter value is set to a token. To retrieve the next set of results, call <code>ListStackSets</code> again and assign that token to the request object's <code>NextToken</code> parameter. If there are no remaining results, the previous response object's <code>NextToken</code> parameter is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>The status of the stack sets that you want to get summary information about.</p>
    pub status: Option<String>,
}

/// Serialize `ListStackSetsRequest` contents to a `SignedRequest`.
struct ListStackSetsRequestSerializer;
impl ListStackSetsRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStackSetsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.max_results {
            params.put(&format!("{}{}", prefix, "MaxResults"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.status {
            params.put(&format!("{}{}", prefix, "Status"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStackSetsResponse {
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListStackInstances</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to <code>null</code>.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSetSummary</code> structures that contain information about the user's stack sets.</p>
    pub summaries: Option<Vec<StackSetSummary>>,
}

struct ListStackSetsResponseDeserializer;
impl ListStackSetsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStackSetsResponse, XmlParseError> {
        deserialize_elements::<_, ListStackSetsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "Summaries" => {
                    obj.summaries.get_or_insert(vec![]).extend(
                        StackSetSummariesDeserializer::deserialize("Summaries", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The input for <a>ListStacks</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStacksRequest {
    /// <p>A string that identifies the next page of stacks that you want to retrieve.</p>
    pub next_token: Option<String>,
    /// <p>Stack status to use as a filter. Specify one or more stack status codes to list only stacks with the specified status codes. For a complete list of stack status codes, see the <code>StackStatus</code> parameter of the <a>Stack</a> data type.</p>
    pub stack_status_filter: Option<Vec<String>>,
}

/// Serialize `ListStacksRequest` contents to a `SignedRequest`.
struct ListStacksRequestSerializer;
impl ListStacksRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListStacksRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
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
pub struct ListStacksResponse {
    /// <p>If the output exceeds 1 MB in size, a string that identifies the next page of stacks. If no additional page exists, this value is null.</p>
    pub next_token: Option<String>,
    /// <p>A list of <code>StackSummary</code> structures containing information about the specified stacks.</p>
    pub stack_summaries: Option<Vec<StackSummary>>,
}

struct ListStacksResponseDeserializer;
impl ListStacksResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListStacksResponse, XmlParseError> {
        deserialize_elements::<_, ListStacksResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                "StackSummaries" => {
                    obj.stack_summaries.get_or_insert(vec![]).extend(
                        StackSummariesDeserializer::deserialize("StackSummaries", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct LogicalResourceIdDeserializer;
impl LogicalResourceIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `LogicalResourceIds` contents to a `SignedRequest`.
struct LogicalResourceIdsSerializer;
impl LogicalResourceIdsSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct MaxConcurrentCountDeserializer;
impl MaxConcurrentCountDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MaxConcurrentPercentageDeserializer;
impl MaxConcurrentPercentageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetadataDeserializer;
impl MetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MonitoringTimeInMinutesDeserializer;
impl MonitoringTimeInMinutesDeserializer {
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
struct NoEchoDeserializer;
impl NoEchoDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NotificationARNDeserializer;
impl NotificationARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NotificationARNsDeserializer;
impl NotificationARNsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(NotificationARNDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Output, XmlParseError> {
        deserialize_elements::<_, Output, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "ExportName" => {
                    obj.export_name =
                        Some(ExportNameDeserializer::deserialize("ExportName", stack)?);
                }
                "OutputKey" => {
                    obj.output_key = Some(OutputKeyDeserializer::deserialize("OutputKey", stack)?);
                }
                "OutputValue" => {
                    obj.output_value =
                        Some(OutputValueDeserializer::deserialize("OutputValue", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct OutputKeyDeserializer;
impl OutputKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OutputValueDeserializer;
impl OutputValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct OutputsDeserializer;
impl OutputsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Output>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(OutputDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The Parameter data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parameter {
    /// <p>The key associated with the parameter. If you don't specify a key and value for a particular parameter, AWS CloudFormation uses the default value that is specified in your template.</p>
    pub parameter_key: Option<String>,
    /// <p>The input value associated with the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>Read-only. The value that corresponds to a Systems Manager parameter key. This field is returned only for <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/parameters-section-structure.html#aws-ssm-parameter-types"> <code>SSM</code> parameter types</a> in the template.</p>
    pub resolved_value: Option<String>,
    /// <p>During a stack update, use the existing parameter value that the stack is using for a given parameter key. If you specify <code>true</code>, do not specify a parameter value.</p>
    pub use_previous_value: Option<bool>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Parameter, XmlParseError> {
        deserialize_elements::<_, Parameter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ParameterKey" => {
                    obj.parameter_key = Some(ParameterKeyDeserializer::deserialize(
                        "ParameterKey",
                        stack,
                    )?);
                }
                "ParameterValue" => {
                    obj.parameter_value = Some(ParameterValueDeserializer::deserialize(
                        "ParameterValue",
                        stack,
                    )?);
                }
                "ResolvedValue" => {
                    obj.resolved_value = Some(ParameterValueDeserializer::deserialize(
                        "ResolvedValue",
                        stack,
                    )?);
                }
                "UsePreviousValue" => {
                    obj.use_previous_value = Some(UsePreviousValueDeserializer::deserialize(
                        "UsePreviousValue",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
        if let Some(ref field_value) = obj.resolved_value {
            params.put(&format!("{}{}", prefix, "ResolvedValue"), &field_value);
        }
        if let Some(ref field_value) = obj.use_previous_value {
            params.put(&format!("{}{}", prefix, "UsePreviousValue"), &field_value);
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParameterConstraints, XmlParseError> {
        deserialize_elements::<_, ParameterConstraints, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AllowedValues" => {
                    obj.allowed_values.get_or_insert(vec![]).extend(
                        AllowedValuesDeserializer::deserialize("AllowedValues", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ParameterDeclaration, XmlParseError> {
        deserialize_elements::<_, ParameterDeclaration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value = Some(ParameterValueDeserializer::deserialize(
                        "DefaultValue",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "NoEcho" => {
                    obj.no_echo = Some(NoEchoDeserializer::deserialize("NoEcho", stack)?);
                }
                "ParameterConstraints" => {
                    obj.parameter_constraints =
                        Some(ParameterConstraintsDeserializer::deserialize(
                            "ParameterConstraints",
                            stack,
                        )?);
                }
                "ParameterKey" => {
                    obj.parameter_key = Some(ParameterKeyDeserializer::deserialize(
                        "ParameterKey",
                        stack,
                    )?);
                }
                "ParameterType" => {
                    obj.parameter_type = Some(ParameterTypeDeserializer::deserialize(
                        "ParameterType",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ParameterDeclarationsDeserializer;
impl ParameterDeclarationsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ParameterDeclaration>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ParameterDeclarationDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ParameterKeyDeserializer;
impl ParameterKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ParameterTypeDeserializer;
impl ParameterTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ParameterValueDeserializer;
impl ParameterValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ParametersDeserializer;
impl ParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Parameter>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ParameterDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PhysicalResourceIdContextDeserializer;
impl PhysicalResourceIdContextDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PhysicalResourceIdContextKeyValuePair>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(
                    PhysicalResourceIdContextKeyValuePairDeserializer::deserialize(
                        "member", stack,
                    )?,
                );
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Context information that enables AWS CloudFormation to uniquely identify a resource. AWS CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs are not enough to uniquely identify that resource. Each context key-value pair specifies a resource that contains the targeted resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PhysicalResourceIdContextKeyValuePair {
    /// <p>The resource context key.</p>
    pub key: String,
    /// <p>The resource context value.</p>
    pub value: String,
}

struct PhysicalResourceIdContextKeyValuePairDeserializer;
impl PhysicalResourceIdContextKeyValuePairDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PhysicalResourceIdContextKeyValuePair, XmlParseError> {
        deserialize_elements::<_, PhysicalResourceIdContextKeyValuePair, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Key" => {
                        obj.key = KeyDeserializer::deserialize("Key", stack)?;
                    }
                    "Value" => {
                        obj.value = ValueDeserializer::deserialize("Value", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct PropertiesDeserializer;
impl PropertiesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Information about a resource property whose actual value differs from its expected value, as defined in the stack template and any values specified as template parameters. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PropertyDifference {
    /// <p>The actual property value of the resource property.</p>
    pub actual_value: String,
    /// <p><p>The type of property difference.</p> <ul> <li> <p> <code>ADD</code>: A value has been added to a resource property that is an array or list data type.</p> </li> <li> <p> <code>REMOVE</code>: The property has been removed from the current resource configuration.</p> </li> <li> <p> <code>NOT_EQUAL</code>: The current property value differs from its expected value (as defined in the stack template and any values specified as template parameters).</p> </li> </ul></p>
    pub difference_type: String,
    /// <p>The expected property value of the resource property, as defined in the stack template and any values specified as template parameters.</p>
    pub expected_value: String,
    /// <p>The fully-qualified path to the resource property.</p>
    pub property_path: String,
}

struct PropertyDifferenceDeserializer;
impl PropertyDifferenceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PropertyDifference, XmlParseError> {
        deserialize_elements::<_, PropertyDifference, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ActualValue" => {
                    obj.actual_value =
                        PropertyValueDeserializer::deserialize("ActualValue", stack)?;
                }
                "DifferenceType" => {
                    obj.difference_type =
                        DifferenceTypeDeserializer::deserialize("DifferenceType", stack)?;
                }
                "ExpectedValue" => {
                    obj.expected_value =
                        PropertyValueDeserializer::deserialize("ExpectedValue", stack)?;
                }
                "PropertyPath" => {
                    obj.property_path =
                        PropertyPathDeserializer::deserialize("PropertyPath", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PropertyDifferencesDeserializer;
impl PropertyDifferencesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<PropertyDifference>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(PropertyDifferenceDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct PropertyNameDeserializer;
impl PropertyNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PropertyPathDeserializer;
impl PropertyPathDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PropertyValueDeserializer;
impl PropertyValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ReasonDeserializer;
impl ReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RegionDeserializer;
impl RegionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RegionListDeserializer;
impl RegionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RegionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RequiresRecreationDeserializer;
impl RequiresRecreationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ResourceAttributeDeserializer;
impl ResourceAttributeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceChange, XmlParseError> {
        deserialize_elements::<_, ResourceChange, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Action" => {
                    obj.action = Some(ChangeActionDeserializer::deserialize("Action", stack)?);
                }
                "Details" => {
                    obj.details.get_or_insert(vec![]).extend(
                        ResourceChangeDetailsDeserializer::deserialize("Details", stack)?,
                    );
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id = Some(LogicalResourceIdDeserializer::deserialize(
                        "LogicalResourceId",
                        stack,
                    )?);
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "Replacement" => {
                    obj.replacement =
                        Some(ReplacementDeserializer::deserialize("Replacement", stack)?);
                }
                "ResourceType" => {
                    obj.resource_type = Some(ResourceTypeDeserializer::deserialize(
                        "ResourceType",
                        stack,
                    )?);
                }
                "Scope" => {
                    obj.scope
                        .get_or_insert(vec![])
                        .extend(ScopeDeserializer::deserialize("Scope", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceChangeDetail, XmlParseError> {
        deserialize_elements::<_, ResourceChangeDetail, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CausingEntity" => {
                    obj.causing_entity = Some(CausingEntityDeserializer::deserialize(
                        "CausingEntity",
                        stack,
                    )?);
                }
                "ChangeSource" => {
                    obj.change_source = Some(ChangeSourceDeserializer::deserialize(
                        "ChangeSource",
                        stack,
                    )?);
                }
                "Evaluation" => {
                    obj.evaluation = Some(EvaluationTypeDeserializer::deserialize(
                        "Evaluation",
                        stack,
                    )?);
                }
                "Target" => {
                    obj.target = Some(ResourceTargetDefinitionDeserializer::deserialize(
                        "Target", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ResourceChangeDetailsDeserializer;
impl ResourceChangeDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ResourceChangeDetail>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ResourceChangeDetailDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct ResourcePropertiesDeserializer;
impl ResourcePropertiesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ResourceStatusDeserializer;
impl ResourceStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ResourceStatusReasonDeserializer;
impl ResourceStatusReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    /// <p>If the <code>Attribute</code> value is <code>Properties</code>, indicates whether a change to this property causes the resource to be recreated. The value can be <code>Never</code>, <code>Always</code>, or <code>Conditionally</code>. To determine the conditions for a <code>Conditionally</code> recreation, see the update behavior for that <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">property</a> in the AWS CloudFormation User Guide.</p>
    pub requires_recreation: Option<String>,
}

struct ResourceTargetDefinitionDeserializer;
impl ResourceTargetDefinitionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResourceTargetDefinition, XmlParseError> {
        deserialize_elements::<_, ResourceTargetDefinition, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Attribute" => {
                        obj.attribute = Some(ResourceAttributeDeserializer::deserialize(
                            "Attribute",
                            stack,
                        )?);
                    }
                    "Name" => {
                        obj.name = Some(PropertyNameDeserializer::deserialize("Name", stack)?);
                    }
                    "RequiresRecreation" => {
                        obj.requires_recreation =
                            Some(RequiresRecreationDeserializer::deserialize(
                                "RequiresRecreation",
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
struct ResourceTypeDeserializer;
impl ResourceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ResourceTypesDeserializer;
impl ResourceTypesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ResourceTypeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct RoleARNDeserializer;
impl RoleARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Structure containing the rollback triggers for AWS CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p> <p>Rollback triggers enable you to have AWS CloudFormation monitor the state of your application during stack creation and updating, and to roll back that operation if the application breaches the threshold of any of the alarms you've specified. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-rollback-triggers.html">Monitor and Roll Back Stack Operations</a>.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RollbackConfiguration {
    /// <p>The amount of time, in minutes, during which CloudFormation should monitor all the rollback triggers after the stack creation or update operation deploys all necessary resources.</p> <p>The default is 0 minutes.</p> <p>If you specify a monitoring period but do not specify any rollback triggers, CloudFormation still waits the specified period of time before cleaning up old resources after update operations. You can use this monitoring period to perform any manual stack validation desired, and manually cancel the stack creation or update (using <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CancelUpdateStack.html">CancelUpdateStack</a>, for example) as necessary.</p> <p>If you specify 0 for this parameter, CloudFormation still monitors the specified rollback triggers during stack creation and update operations. Then, for update operations, it begins disposing of old resources immediately once the operation completes.</p>
    pub monitoring_time_in_minutes: Option<i64>,
    /// <p>The triggers to monitor during stack creation or update actions. </p> <p>By default, AWS CloudFormation saves the rollback triggers specified for a stack and applies them to any subsequent update operations for the stack, unless you specify otherwise. If you do specify rollback triggers for this parameter, those triggers replace any list of triggers previously specified for the stack. This means:</p> <ul> <li> <p>To use the rollback triggers previously specified for this stack, if any, don't specify this parameter.</p> </li> <li> <p>To specify new or updated rollback triggers, you must specify <i>all</i> the triggers that you want used for this stack, even triggers you've specifed before (for example, when creating the stack or during a previous stack update). Any triggers that you don't include in the updated list of triggers are no longer applied to the stack.</p> </li> <li> <p>To remove all currently specified triggers, specify an empty list for this parameter.</p> </li> </ul> <p>If a specified trigger is missing, the entire stack operation fails and is rolled back. </p>
    pub rollback_triggers: Option<Vec<RollbackTrigger>>,
}

struct RollbackConfigurationDeserializer;
impl RollbackConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RollbackConfiguration, XmlParseError> {
        deserialize_elements::<_, RollbackConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MonitoringTimeInMinutes" => {
                    obj.monitoring_time_in_minutes =
                        Some(MonitoringTimeInMinutesDeserializer::deserialize(
                            "MonitoringTimeInMinutes",
                            stack,
                        )?);
                }
                "RollbackTriggers" => {
                    obj.rollback_triggers.get_or_insert(vec![]).extend(
                        RollbackTriggersDeserializer::deserialize("RollbackTriggers", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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
                &field_value,
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
    /// <p>The resource type of the rollback trigger. Currently, <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cw-alarm.html">AWS::CloudWatch::Alarm</a> is the only supported resource type.</p>
    pub type_: String,
}

struct RollbackTriggerDeserializer;
impl RollbackTriggerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RollbackTrigger, XmlParseError> {
        deserialize_elements::<_, RollbackTrigger, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Arn" => {
                    obj.arn = ArnDeserializer::deserialize("Arn", stack)?;
                }
                "Type" => {
                    obj.type_ = TypeDeserializer::deserialize("Type", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "Arn"), &obj.arn);
        params.put(&format!("{}{}", prefix, "Type"), &obj.type_);
    }
}

struct RollbackTriggersDeserializer;
impl RollbackTriggersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RollbackTrigger>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(RollbackTriggerDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ResourceAttributeDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The input for the <a>SetStackPolicy</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetStackPolicyRequest {
    /// <p>The name or unique stack ID that you want to associate a policy with.</p>
    pub stack_name: String,
    /// <p>Structure containing the stack policy body. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the AWS CloudFormation User Guide. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_body: Option<String>,
    /// <p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    pub stack_policy_url: Option<String>,
}

/// Serialize `SetStackPolicyRequest` contents to a `SignedRequest`.
struct SetStackPolicyRequestSerializer;
impl SetStackPolicyRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SetStackPolicyRequest) {
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetStackPolicyResponse {}

struct SetStackPolicyResponseDeserializer;
impl SetStackPolicyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetStackPolicyResponse, XmlParseError> {
        Ok(SetStackPolicyResponse::default())
    }
}
/// <p>The input for the <a>SignalResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SignalResourceRequest {
    /// <p>The logical ID of the resource that you want to signal. The logical ID is the name of the resource that given in the template.</p>
    pub logical_resource_id: String,
    /// <p>The stack name or unique stack ID that includes the resource that you want to signal.</p>
    pub stack_name: String,
    /// <p>The status of the signal, which is either success or failure. A failure signal causes AWS CloudFormation to immediately fail the stack creation or update.</p>
    pub status: String,
    /// <p>A unique ID of the signal. When you signal Amazon EC2 instances or Auto Scaling groups, specify the instance ID that you are signaling as the unique ID. If you send multiple signals to a single resource (such as signaling a wait condition), each signal requires a different unique ID.</p>
    pub unique_id: String,
}

/// Serialize `SignalResourceRequest` contents to a `SignedRequest`.
struct SignalResourceRequestSerializer;
impl SignalResourceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SignalResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "LogicalResourceId"),
            &obj.logical_resource_id,
        );
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        params.put(&format!("{}{}", prefix, "Status"), &obj.status);
        params.put(&format!("{}{}", prefix, "UniqueId"), &obj.unique_id);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SignalResourceResponse {}

struct SignalResourceResponseDeserializer;
impl SignalResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SignalResourceResponse, XmlParseError> {
        Ok(SignalResourceResponse::default())
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
    /// <p>Information on whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
    pub drift_information: Option<StackDriftInformation>,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Stack, XmlParseError> {
        deserialize_elements::<_, Stack, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Capabilities" => {
                    obj.capabilities.get_or_insert(vec![]).extend(
                        CapabilitiesDeserializer::deserialize("Capabilities", stack)?,
                    );
                }
                "ChangeSetId" => {
                    obj.change_set_id =
                        Some(ChangeSetIdDeserializer::deserialize("ChangeSetId", stack)?);
                }
                "CreationTime" => {
                    obj.creation_time =
                        CreationTimeDeserializer::deserialize("CreationTime", stack)?;
                }
                "DeletionTime" => {
                    obj.deletion_time = Some(DeletionTimeDeserializer::deserialize(
                        "DeletionTime",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "DisableRollback" => {
                    obj.disable_rollback = Some(DisableRollbackDeserializer::deserialize(
                        "DisableRollback",
                        stack,
                    )?);
                }
                "DriftInformation" => {
                    obj.drift_information = Some(StackDriftInformationDeserializer::deserialize(
                        "DriftInformation",
                        stack,
                    )?);
                }
                "EnableTerminationProtection" => {
                    obj.enable_termination_protection =
                        Some(EnableTerminationProtectionDeserializer::deserialize(
                            "EnableTerminationProtection",
                            stack,
                        )?);
                }
                "LastUpdatedTime" => {
                    obj.last_updated_time = Some(LastUpdatedTimeDeserializer::deserialize(
                        "LastUpdatedTime",
                        stack,
                    )?);
                }
                "NotificationARNs" => {
                    obj.notification_ar_ns.get_or_insert(vec![]).extend(
                        NotificationARNsDeserializer::deserialize("NotificationARNs", stack)?,
                    );
                }
                "Outputs" => {
                    obj.outputs
                        .get_or_insert(vec![])
                        .extend(OutputsDeserializer::deserialize("Outputs", stack)?);
                }
                "Parameters" => {
                    obj.parameters
                        .get_or_insert(vec![])
                        .extend(ParametersDeserializer::deserialize("Parameters", stack)?);
                }
                "ParentId" => {
                    obj.parent_id = Some(StackIdDeserializer::deserialize("ParentId", stack)?);
                }
                "RoleARN" => {
                    obj.role_arn = Some(RoleARNDeserializer::deserialize("RoleARN", stack)?);
                }
                "RollbackConfiguration" => {
                    obj.rollback_configuration =
                        Some(RollbackConfigurationDeserializer::deserialize(
                            "RollbackConfiguration",
                            stack,
                        )?);
                }
                "RootId" => {
                    obj.root_id = Some(StackIdDeserializer::deserialize("RootId", stack)?);
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackName" => {
                    obj.stack_name = StackNameDeserializer::deserialize("StackName", stack)?;
                }
                "StackStatus" => {
                    obj.stack_status = StackStatusDeserializer::deserialize("StackStatus", stack)?;
                }
                "StackStatusReason" => {
                    obj.stack_status_reason = Some(StackStatusReasonDeserializer::deserialize(
                        "StackStatusReason",
                        stack,
                    )?);
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagsDeserializer::deserialize("Tags", stack)?);
                }
                "TimeoutInMinutes" => {
                    obj.timeout_in_minutes = Some(TimeoutMinutesDeserializer::deserialize(
                        "TimeoutInMinutes",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackDriftDetectionIdDeserializer;
impl StackDriftDetectionIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackDriftDetectionStatusDeserializer;
impl StackDriftDetectionStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackDriftDetectionStatusReasonDeserializer;
impl StackDriftDetectionStatusReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Contains information about whether the stack's actual configuration differs, or has <i>drifted</i>, from its expected configuration, as defined in the stack template and any values specified as template parameters. A stack is considered to have drifted if one or more of its resources have drifted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackDriftInformation {
    /// <p>Most recent time when a drift detection operation was initiated on the stack, or any of its individual resources that support drift detection.</p>
    pub last_check_timestamp: Option<String>,
    /// <p><p>Status of the stack&#39;s actual configuration compared to its expected template configuration. </p> <ul> <li> <p> <code>DRIFTED</code>: The stack differs from its expected template configuration. A stack is considered to have drifted if one or more of its resources have drifted.</p> </li> <li> <p> <code>NOT<em>CHECKED</code>: AWS CloudFormation has not checked if the stack differs from its expected template configuration.</p> </li> <li> <p> <code>IN</em>SYNC</code>: The stack&#39;s actual configuration matches its expected template configuration.</p> </li> <li> <p> <code>UNKNOWN</code>: This value is reserved for future use.</p> </li> </ul></p>
    pub stack_drift_status: String,
}

struct StackDriftInformationDeserializer;
impl StackDriftInformationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackDriftInformation, XmlParseError> {
        deserialize_elements::<_, StackDriftInformation, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "LastCheckTimestamp" => {
                    obj.last_check_timestamp = Some(TimestampDeserializer::deserialize(
                        "LastCheckTimestamp",
                        stack,
                    )?);
                }
                "StackDriftStatus" => {
                    obj.stack_drift_status =
                        StackDriftStatusDeserializer::deserialize("StackDriftStatus", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains information about whether the stack's actual configuration differs, or has <i>drifted</i>, from its expected configuration, as defined in the stack template and any values specified as template parameters. A stack is considered to have drifted if one or more of its resources have drifted.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackDriftInformationSummary {
    /// <p>Most recent time when a drift detection operation was initiated on the stack, or any of its individual resources that support drift detection.</p>
    pub last_check_timestamp: Option<String>,
    /// <p><p>Status of the stack&#39;s actual configuration compared to its expected template configuration. </p> <ul> <li> <p> <code>DRIFTED</code>: The stack differs from its expected template configuration. A stack is considered to have drifted if one or more of its resources have drifted.</p> </li> <li> <p> <code>NOT<em>CHECKED</code>: AWS CloudFormation has not checked if the stack differs from its expected template configuration.</p> </li> <li> <p> <code>IN</em>SYNC</code>: The stack&#39;s actual configuration matches its expected template configuration.</p> </li> <li> <p> <code>UNKNOWN</code>: This value is reserved for future use.</p> </li> </ul></p>
    pub stack_drift_status: String,
}

struct StackDriftInformationSummaryDeserializer;
impl StackDriftInformationSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackDriftInformationSummary, XmlParseError> {
        deserialize_elements::<_, StackDriftInformationSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LastCheckTimestamp" => {
                        obj.last_check_timestamp = Some(TimestampDeserializer::deserialize(
                            "LastCheckTimestamp",
                            stack,
                        )?);
                    }
                    "StackDriftStatus" => {
                        obj.stack_drift_status =
                            StackDriftStatusDeserializer::deserialize("StackDriftStatus", stack)?;
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct StackDriftStatusDeserializer;
impl StackDriftStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    /// <p>Type of resource. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackEvent, XmlParseError> {
        deserialize_elements::<_, StackEvent, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ClientRequestToken" => {
                    obj.client_request_token = Some(ClientRequestTokenDeserializer::deserialize(
                        "ClientRequestToken",
                        stack,
                    )?);
                }
                "EventId" => {
                    obj.event_id = EventIdDeserializer::deserialize("EventId", stack)?;
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id = Some(LogicalResourceIdDeserializer::deserialize(
                        "LogicalResourceId",
                        stack,
                    )?);
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "ResourceProperties" => {
                    obj.resource_properties = Some(ResourcePropertiesDeserializer::deserialize(
                        "ResourceProperties",
                        stack,
                    )?);
                }
                "ResourceStatus" => {
                    obj.resource_status = Some(ResourceStatusDeserializer::deserialize(
                        "ResourceStatus",
                        stack,
                    )?);
                }
                "ResourceStatusReason" => {
                    obj.resource_status_reason =
                        Some(ResourceStatusReasonDeserializer::deserialize(
                            "ResourceStatusReason",
                            stack,
                        )?);
                }
                "ResourceType" => {
                    obj.resource_type = Some(ResourceTypeDeserializer::deserialize(
                        "ResourceType",
                        stack,
                    )?);
                }
                "StackId" => {
                    obj.stack_id = StackIdDeserializer::deserialize("StackId", stack)?;
                }
                "StackName" => {
                    obj.stack_name = StackNameDeserializer::deserialize("StackName", stack)?;
                }
                "Timestamp" => {
                    obj.timestamp = TimestampDeserializer::deserialize("Timestamp", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackEventsDeserializer;
impl StackEventsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackEvent>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackEventDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct StackIdDeserializer;
impl StackIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackInstance, XmlParseError> {
        deserialize_elements::<_, StackInstance, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Account" => {
                    obj.account = Some(AccountDeserializer::deserialize("Account", stack)?);
                }
                "ParameterOverrides" => {
                    obj.parameter_overrides.get_or_insert(vec![]).extend(
                        ParametersDeserializer::deserialize("ParameterOverrides", stack)?,
                    );
                }
                "Region" => {
                    obj.region = Some(RegionDeserializer::deserialize("Region", stack)?);
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                "Status" => {
                    obj.status = Some(StackInstanceStatusDeserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                "StatusReason" => {
                    obj.status_reason =
                        Some(ReasonDeserializer::deserialize("StatusReason", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackInstanceStatusDeserializer;
impl StackInstanceStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackInstanceSummariesDeserializer;
impl StackInstanceSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackInstanceSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackInstanceSummaryDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackInstanceSummary, XmlParseError> {
        deserialize_elements::<_, StackInstanceSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Account" => {
                    obj.account = Some(AccountDeserializer::deserialize("Account", stack)?);
                }
                "Region" => {
                    obj.region = Some(RegionDeserializer::deserialize("Region", stack)?);
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                "Status" => {
                    obj.status = Some(StackInstanceStatusDeserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                "StatusReason" => {
                    obj.status_reason =
                        Some(ReasonDeserializer::deserialize("StatusReason", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackNameDeserializer;
impl StackNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackPolicyBodyDeserializer;
impl StackPolicyBodyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The StackResource data type.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResource {
    /// <p>User defined description associated with the resource.</p>
    pub description: Option<String>,
    /// <p>Information about whether the resource's actual configuration differs, or has <i>drifted</i>, from its expected configuration, as defined in the stack template and any values specified as template parameters. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
    pub drift_information: Option<StackResourceDriftInformation>,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>
    pub physical_resource_id: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: String,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResource, XmlParseError> {
        deserialize_elements::<_, StackResource, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "DriftInformation" => {
                    obj.drift_information =
                        Some(StackResourceDriftInformationDeserializer::deserialize(
                            "DriftInformation",
                            stack,
                        )?);
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id =
                        LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)?;
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "ResourceStatus" => {
                    obj.resource_status =
                        ResourceStatusDeserializer::deserialize("ResourceStatus", stack)?;
                }
                "ResourceStatusReason" => {
                    obj.resource_status_reason =
                        Some(ResourceStatusReasonDeserializer::deserialize(
                            "ResourceStatusReason",
                            stack,
                        )?);
                }
                "ResourceType" => {
                    obj.resource_type =
                        ResourceTypeDeserializer::deserialize("ResourceType", stack)?;
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackName" => {
                    obj.stack_name = Some(StackNameDeserializer::deserialize("StackName", stack)?);
                }
                "Timestamp" => {
                    obj.timestamp = TimestampDeserializer::deserialize("Timestamp", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains detailed information about the specified stack resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceDetail {
    /// <p>User defined description associated with the resource.</p>
    pub description: Option<String>,
    /// <p>Information about whether the resource's actual configuration differs, or has <i>drifted</i>, from its expected configuration, as defined in the stack template and any values specified as template parameters. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
    pub drift_information: Option<StackResourceDriftInformation>,
    /// <p>Time the status was updated.</p>
    pub last_updated_timestamp: String,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The content of the <code>Metadata</code> attribute declared for the resource. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-attribute-metadata.html">Metadata Attribute</a> in the AWS CloudFormation User Guide.</p>
    pub metadata: Option<String>,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation.</p>
    pub physical_resource_id: Option<String>,
    /// <p>Current status of the resource.</p>
    pub resource_status: String,
    /// <p>Success/failure message associated with the resource.</p>
    pub resource_status_reason: Option<String>,
    /// <p>Type of resource. ((For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: String,
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
    /// <p>The name associated with the stack.</p>
    pub stack_name: Option<String>,
}

struct StackResourceDetailDeserializer;
impl StackResourceDetailDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceDetail, XmlParseError> {
        deserialize_elements::<_, StackResourceDetail, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "DriftInformation" => {
                    obj.drift_information =
                        Some(StackResourceDriftInformationDeserializer::deserialize(
                            "DriftInformation",
                            stack,
                        )?);
                }
                "LastUpdatedTimestamp" => {
                    obj.last_updated_timestamp =
                        TimestampDeserializer::deserialize("LastUpdatedTimestamp", stack)?;
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id =
                        LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)?;
                }
                "Metadata" => {
                    obj.metadata = Some(MetadataDeserializer::deserialize("Metadata", stack)?);
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "ResourceStatus" => {
                    obj.resource_status =
                        ResourceStatusDeserializer::deserialize("ResourceStatus", stack)?;
                }
                "ResourceStatusReason" => {
                    obj.resource_status_reason =
                        Some(ResourceStatusReasonDeserializer::deserialize(
                            "ResourceStatusReason",
                            stack,
                        )?);
                }
                "ResourceType" => {
                    obj.resource_type =
                        ResourceTypeDeserializer::deserialize("ResourceType", stack)?;
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackName" => {
                    obj.stack_name = Some(StackNameDeserializer::deserialize("StackName", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains the drift information for a resource that has been checked for drift. This includes actual and expected property values for resources in which AWS CloudFormation has detected drift. Only resource properties explicitly defined in the stack template are checked for drift. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Resources that do not currently support drift detection cannot be checked. For a list of resources that support drift detection, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p> <p>Use <a>DetectStackResourceDrift</a> to detect drift on individual resources, or <a>DetectStackDrift</a> to detect drift on all resources in a given stack that support drift detection.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceDrift {
    /// <p>A JSON structure containing the actual property values of the stack resource.</p> <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present. </p>
    pub actual_properties: Option<String>,
    /// <p>A JSON structure containing the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters. </p> <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present. </p>
    pub expected_properties: Option<String>,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: String,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by AWS CloudFormation. </p>
    pub physical_resource_id: Option<String>,
    /// <p>Context information that enables AWS CloudFormation to uniquely identify a resource. AWS CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs are not enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    pub physical_resource_id_context: Option<Vec<PhysicalResourceIdContextKeyValuePair>>,
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>. </p>
    pub property_differences: Option<Vec<PropertyDifference>>,
    /// <p>The type of the resource.</p>
    pub resource_type: String,
    /// <p>The ID of the stack.</p>
    pub stack_id: String,
    /// <p><p>Status of the resource&#39;s actual configuration compared to its expected configuration</p> <ul> <li> <p> <code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p> </li> <li> <p> <code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p> </li> <li> <p> <code>IN<em>SYNC</code>: The resources&#39;s actual configuration matches its expected template configuration.</p> </li> <li> <p> <code>NOT</em>CHECKED</code>: AWS CloudFormation does not currently return this value.</p> </li> </ul></p>
    pub stack_resource_drift_status: String,
    /// <p>Time at which AWS CloudFormation performed drift detection on the stack resource.</p>
    pub timestamp: String,
}

struct StackResourceDriftDeserializer;
impl StackResourceDriftDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceDrift, XmlParseError> {
        deserialize_elements::<_, StackResourceDrift, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ActualProperties" => {
                    obj.actual_properties = Some(PropertiesDeserializer::deserialize(
                        "ActualProperties",
                        stack,
                    )?);
                }
                "ExpectedProperties" => {
                    obj.expected_properties = Some(PropertiesDeserializer::deserialize(
                        "ExpectedProperties",
                        stack,
                    )?);
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id =
                        LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)?;
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "PhysicalResourceIdContext" => {
                    obj.physical_resource_id_context
                        .get_or_insert(vec![])
                        .extend(PhysicalResourceIdContextDeserializer::deserialize(
                            "PhysicalResourceIdContext",
                            stack,
                        )?);
                }
                "PropertyDifferences" => {
                    obj.property_differences.get_or_insert(vec![]).extend(
                        PropertyDifferencesDeserializer::deserialize("PropertyDifferences", stack)?,
                    );
                }
                "ResourceType" => {
                    obj.resource_type =
                        ResourceTypeDeserializer::deserialize("ResourceType", stack)?;
                }
                "StackId" => {
                    obj.stack_id = StackIdDeserializer::deserialize("StackId", stack)?;
                }
                "StackResourceDriftStatus" => {
                    obj.stack_resource_drift_status =
                        StackResourceDriftStatusDeserializer::deserialize(
                            "StackResourceDriftStatus",
                            stack,
                        )?;
                }
                "Timestamp" => {
                    obj.timestamp = TimestampDeserializer::deserialize("Timestamp", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Contains information about whether the resource's actual configuration differs, or has <i>drifted</i>, from its expected configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceDriftInformation {
    /// <p>When AWS CloudFormation last checked if the resource had drifted from its expected configuration.</p>
    pub last_check_timestamp: Option<String>,
    /// <p><p>Status of the resource&#39;s actual configuration compared to its expected configuration</p> <ul> <li> <p> <code>DELETED</code>: The resource differs from its expected configuration in that it has been deleted.</p> </li> <li> <p> <code>MODIFIED</code>: The resource differs from its expected configuration.</p> </li> <li> <p> <code>NOT<em>CHECKED</code>: AWS CloudFormation has not checked if the resource differs from its expected configuration.</p> <p>Any resources that do not currently support drift detection have a status of <code>NOT</em>CHECKED</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>. </p> </li> <li> <p> <code>IN_SYNC</code>: The resources&#39;s actual configuration matches its expected configuration.</p> </li> </ul></p>
    pub stack_resource_drift_status: String,
}

struct StackResourceDriftInformationDeserializer;
impl StackResourceDriftInformationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceDriftInformation, XmlParseError> {
        deserialize_elements::<_, StackResourceDriftInformation, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LastCheckTimestamp" => {
                        obj.last_check_timestamp = Some(TimestampDeserializer::deserialize(
                            "LastCheckTimestamp",
                            stack,
                        )?);
                    }
                    "StackResourceDriftStatus" => {
                        obj.stack_resource_drift_status =
                            StackResourceDriftStatusDeserializer::deserialize(
                                "StackResourceDriftStatus",
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
/// <p>Summarizes information about whether the resource's actual configuration differs, or has <i>drifted</i>, from its expected configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceDriftInformationSummary {
    /// <p>When AWS CloudFormation last checked if the resource had drifted from its expected configuration.</p>
    pub last_check_timestamp: Option<String>,
    /// <p><p>Status of the resource&#39;s actual configuration compared to its expected configuration</p> <ul> <li> <p> <code>DELETED</code>: The resource differs from its expected configuration in that it has been deleted.</p> </li> <li> <p> <code>MODIFIED</code>: The resource differs from its expected configuration.</p> </li> <li> <p> <code>NOT<em>CHECKED</code>: AWS CloudFormation has not checked if the resource differs from its expected configuration.</p> <p>Any resources that do not currently support drift detection have a status of <code>NOT</em>CHECKED</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>. If you performed an <a>ContinueUpdateRollback</a> operation on a stack, any resources included in <code>ResourcesToSkip</code> will also have a status of <code>NOT<em>CHECKED</code>. For more information on skipping resources during rollback operations, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-continueupdaterollback.html">Continue Rolling Back an Update</a> in the AWS CloudFormation User Guide.</p> </li> <li> <p> <code>IN</em>SYNC</code>: The resources&#39;s actual configuration matches its expected configuration.</p> </li> </ul></p>
    pub stack_resource_drift_status: String,
}

struct StackResourceDriftInformationSummaryDeserializer;
impl StackResourceDriftInformationSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceDriftInformationSummary, XmlParseError> {
        deserialize_elements::<_, StackResourceDriftInformationSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "LastCheckTimestamp" => {
                        obj.last_check_timestamp = Some(TimestampDeserializer::deserialize(
                            "LastCheckTimestamp",
                            stack,
                        )?);
                    }
                    "StackResourceDriftStatus" => {
                        obj.stack_resource_drift_status =
                            StackResourceDriftStatusDeserializer::deserialize(
                                "StackResourceDriftStatus",
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
struct StackResourceDriftStatusDeserializer;
impl StackResourceDriftStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `StackResourceDriftStatusFilters` contents to a `SignedRequest`.
struct StackResourceDriftStatusFiltersSerializer;
impl StackResourceDriftStatusFiltersSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct StackResourceDriftsDeserializer;
impl StackResourceDriftsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackResourceDrift>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackResourceDriftDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct StackResourceSummariesDeserializer;
impl StackResourceSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackResourceSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackResourceSummaryDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Contains high-level information about the specified stack resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackResourceSummary {
    /// <p>Information about whether the resource's actual configuration differs, or has <i>drifted</i>, from its expected configuration, as defined in the stack template and any values specified as template parameters. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
    pub drift_information: Option<StackResourceDriftInformationSummary>,
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
    /// <p>Type of resource. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html"> AWS Resource Types Reference</a> in the AWS CloudFormation User Guide.)</p>
    pub resource_type: String,
}

struct StackResourceSummaryDeserializer;
impl StackResourceSummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackResourceSummary, XmlParseError> {
        deserialize_elements::<_, StackResourceSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DriftInformation" => {
                    obj.drift_information = Some(
                        StackResourceDriftInformationSummaryDeserializer::deserialize(
                            "DriftInformation",
                            stack,
                        )?,
                    );
                }
                "LastUpdatedTimestamp" => {
                    obj.last_updated_timestamp =
                        TimestampDeserializer::deserialize("LastUpdatedTimestamp", stack)?;
                }
                "LogicalResourceId" => {
                    obj.logical_resource_id =
                        LogicalResourceIdDeserializer::deserialize("LogicalResourceId", stack)?;
                }
                "PhysicalResourceId" => {
                    obj.physical_resource_id = Some(PhysicalResourceIdDeserializer::deserialize(
                        "PhysicalResourceId",
                        stack,
                    )?);
                }
                "ResourceStatus" => {
                    obj.resource_status =
                        ResourceStatusDeserializer::deserialize("ResourceStatus", stack)?;
                }
                "ResourceStatusReason" => {
                    obj.resource_status_reason =
                        Some(ResourceStatusReasonDeserializer::deserialize(
                            "ResourceStatusReason",
                            stack,
                        )?);
                }
                "ResourceType" => {
                    obj.resource_type =
                        ResourceTypeDeserializer::deserialize("ResourceType", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackResourcesDeserializer;
impl StackResourcesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackResource>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackResourceDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A structure that contains information about a stack set. A stack set enables you to provision stacks into AWS accounts and across regions by using a single CloudFormation template. In the stack set, you specify the template to use, as well as any parameters and capabilities that the template requires. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSet {
    /// <p>The Amazon Resource Number (ARN) of the IAM role used to create or update the stack set.</p> <p>Use customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Prerequisites: Granting Permissions for Stack Set Operations</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    pub administration_role_arn: Option<String>,
    /// <p>The capabilities that are allowed in the stack set. Some stack set templates might include resources that can affect permissions in your AWS account—for example, by creating new AWS Identity and Access Management (IAM) users. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates.</a> </p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A description of the stack set that you specify when the stack set is created or updated.</p>
    pub description: Option<String>,
    /// <p>The name of the IAM execution role used to create or update the stack set. </p> <p>Use customized execution roles to control which stack resources users and groups can include in their stack sets. </p>
    pub execution_role_name: Option<String>,
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSet, XmlParseError> {
        deserialize_elements::<_, StackSet, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AdministrationRoleARN" => {
                    obj.administration_role_arn = Some(RoleARNDeserializer::deserialize(
                        "AdministrationRoleARN",
                        stack,
                    )?);
                }
                "Capabilities" => {
                    obj.capabilities.get_or_insert(vec![]).extend(
                        CapabilitiesDeserializer::deserialize("Capabilities", stack)?,
                    );
                }
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "ExecutionRoleName" => {
                    obj.execution_role_name = Some(ExecutionRoleNameDeserializer::deserialize(
                        "ExecutionRoleName",
                        stack,
                    )?);
                }
                "Parameters" => {
                    obj.parameters
                        .get_or_insert(vec![])
                        .extend(ParametersDeserializer::deserialize("Parameters", stack)?);
                }
                "StackSetARN" => {
                    obj.stack_set_arn =
                        Some(StackSetARNDeserializer::deserialize("StackSetARN", stack)?);
                }
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                "StackSetName" => {
                    obj.stack_set_name = Some(StackSetNameDeserializer::deserialize(
                        "StackSetName",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(StackSetStatusDeserializer::deserialize("Status", stack)?);
                }
                "Tags" => {
                    obj.tags
                        .get_or_insert(vec![])
                        .extend(TagsDeserializer::deserialize("Tags", stack)?);
                }
                "TemplateBody" => {
                    obj.template_body = Some(TemplateBodyDeserializer::deserialize(
                        "TemplateBody",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackSetARNDeserializer;
impl StackSetARNDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSetIdDeserializer;
impl StackSetIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSetNameDeserializer;
impl StackSetNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    /// <p>The name of the IAM execution role used to create or update the stack set.</p> <p>Use customized execution roles to control which stack resources users and groups can include in their stack sets. </p>
    pub execution_role_name: Option<String>,
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperation, XmlParseError> {
        deserialize_elements::<_, StackSetOperation, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Action" => {
                    obj.action = Some(StackSetOperationActionDeserializer::deserialize(
                        "Action", stack,
                    )?);
                }
                "AdministrationRoleARN" => {
                    obj.administration_role_arn = Some(RoleARNDeserializer::deserialize(
                        "AdministrationRoleARN",
                        stack,
                    )?);
                }
                "CreationTimestamp" => {
                    obj.creation_timestamp = Some(TimestampDeserializer::deserialize(
                        "CreationTimestamp",
                        stack,
                    )?);
                }
                "EndTimestamp" => {
                    obj.end_timestamp =
                        Some(TimestampDeserializer::deserialize("EndTimestamp", stack)?);
                }
                "ExecutionRoleName" => {
                    obj.execution_role_name = Some(ExecutionRoleNameDeserializer::deserialize(
                        "ExecutionRoleName",
                        stack,
                    )?);
                }
                "OperationId" => {
                    obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                        "OperationId",
                        stack,
                    )?);
                }
                "OperationPreferences" => {
                    obj.operation_preferences =
                        Some(StackSetOperationPreferencesDeserializer::deserialize(
                            "OperationPreferences",
                            stack,
                        )?);
                }
                "RetainStacks" => {
                    obj.retain_stacks = Some(RetainStacksNullableDeserializer::deserialize(
                        "RetainStacks",
                        stack,
                    )?);
                }
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                "Status" => {
                    obj.status = Some(StackSetOperationStatusDeserializer::deserialize(
                        "Status", stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackSetOperationActionDeserializer;
impl StackSetOperationActionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The user-specified preferences for how AWS CloudFormation performs a stack set operation. </p> <p>For more information on maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationPreferences, XmlParseError> {
        deserialize_elements::<_, StackSetOperationPreferences, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "FailureToleranceCount" => {
                        obj.failure_tolerance_count =
                            Some(FailureToleranceCountDeserializer::deserialize(
                                "FailureToleranceCount",
                                stack,
                            )?);
                    }
                    "FailureTolerancePercentage" => {
                        obj.failure_tolerance_percentage =
                            Some(FailureTolerancePercentageDeserializer::deserialize(
                                "FailureTolerancePercentage",
                                stack,
                            )?);
                    }
                    "MaxConcurrentCount" => {
                        obj.max_concurrent_count =
                            Some(MaxConcurrentCountDeserializer::deserialize(
                                "MaxConcurrentCount",
                                stack,
                            )?);
                    }
                    "MaxConcurrentPercentage" => {
                        obj.max_concurrent_percentage =
                            Some(MaxConcurrentPercentageDeserializer::deserialize(
                                "MaxConcurrentPercentage",
                                stack,
                            )?);
                    }
                    "RegionOrder" => {
                        obj.region_order
                            .get_or_insert(vec![])
                            .extend(RegionListDeserializer::deserialize("RegionOrder", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.failure_tolerance_percentage {
            params.put(
                &format!("{}{}", prefix, "FailureTolerancePercentage"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.max_concurrent_count {
            params.put(&format!("{}{}", prefix, "MaxConcurrentCount"), &field_value);
        }
        if let Some(ref field_value) = obj.max_concurrent_percentage {
            params.put(
                &format!("{}{}", prefix, "MaxConcurrentPercentage"),
                &field_value,
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSetOperationResultSummariesDeserializer;
impl StackSetOperationResultSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetOperationResultSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackSetOperationResultSummaryDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationResultSummary, XmlParseError> {
        deserialize_elements::<_, StackSetOperationResultSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Account" => {
                        obj.account = Some(AccountDeserializer::deserialize("Account", stack)?);
                    }
                    "AccountGateResult" => {
                        obj.account_gate_result = Some(AccountGateResultDeserializer::deserialize(
                            "AccountGateResult",
                            stack,
                        )?);
                    }
                    "Region" => {
                        obj.region = Some(RegionDeserializer::deserialize("Region", stack)?);
                    }
                    "Status" => {
                        obj.status = Some(StackSetOperationResultStatusDeserializer::deserialize(
                            "Status", stack,
                        )?);
                    }
                    "StatusReason" => {
                        obj.status_reason =
                            Some(ReasonDeserializer::deserialize("StatusReason", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct StackSetOperationStatusDeserializer;
impl StackSetOperationStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSetOperationSummariesDeserializer;
impl StackSetOperationSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetOperationSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackSetOperationSummaryDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetOperationSummary, XmlParseError> {
        deserialize_elements::<_, StackSetOperationSummary, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Action" => {
                        obj.action = Some(StackSetOperationActionDeserializer::deserialize(
                            "Action", stack,
                        )?);
                    }
                    "CreationTimestamp" => {
                        obj.creation_timestamp = Some(TimestampDeserializer::deserialize(
                            "CreationTimestamp",
                            stack,
                        )?);
                    }
                    "EndTimestamp" => {
                        obj.end_timestamp =
                            Some(TimestampDeserializer::deserialize("EndTimestamp", stack)?);
                    }
                    "OperationId" => {
                        obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
                            stack,
                        )?);
                    }
                    "Status" => {
                        obj.status = Some(StackSetOperationStatusDeserializer::deserialize(
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
struct StackSetStatusDeserializer;
impl StackSetStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSetSummariesDeserializer;
impl StackSetSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSetSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackSetSummaryDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSetSummary, XmlParseError> {
        deserialize_elements::<_, StackSetSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "StackSetId" => {
                    obj.stack_set_id =
                        Some(StackSetIdDeserializer::deserialize("StackSetId", stack)?);
                }
                "StackSetName" => {
                    obj.stack_set_name = Some(StackSetNameDeserializer::deserialize(
                        "StackSetName",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(StackSetStatusDeserializer::deserialize("Status", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StackStatusDeserializer;
impl StackStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StackSummariesDeserializer;
impl StackSummariesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StackSummary>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackSummaryDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The StackSummary Data Type</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StackSummary {
    /// <p>The time the stack was created.</p>
    pub creation_time: String,
    /// <p>The time the stack was deleted.</p>
    pub deletion_time: Option<String>,
    /// <p>Summarizes information on whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p>
    pub drift_information: Option<StackDriftInformationSummary>,
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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StackSummary, XmlParseError> {
        deserialize_elements::<_, StackSummary, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CreationTime" => {
                    obj.creation_time =
                        CreationTimeDeserializer::deserialize("CreationTime", stack)?;
                }
                "DeletionTime" => {
                    obj.deletion_time = Some(DeletionTimeDeserializer::deserialize(
                        "DeletionTime",
                        stack,
                    )?);
                }
                "DriftInformation" => {
                    obj.drift_information =
                        Some(StackDriftInformationSummaryDeserializer::deserialize(
                            "DriftInformation",
                            stack,
                        )?);
                }
                "LastUpdatedTime" => {
                    obj.last_updated_time = Some(LastUpdatedTimeDeserializer::deserialize(
                        "LastUpdatedTime",
                        stack,
                    )?);
                }
                "ParentId" => {
                    obj.parent_id = Some(StackIdDeserializer::deserialize("ParentId", stack)?);
                }
                "RootId" => {
                    obj.root_id = Some(StackIdDeserializer::deserialize("RootId", stack)?);
                }
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                "StackName" => {
                    obj.stack_name = StackNameDeserializer::deserialize("StackName", stack)?;
                }
                "StackStatus" => {
                    obj.stack_status = StackStatusDeserializer::deserialize("StackStatus", stack)?;
                }
                "StackStatusReason" => {
                    obj.stack_status_reason = Some(StackStatusReasonDeserializer::deserialize(
                        "StackStatusReason",
                        stack,
                    )?);
                }
                "TemplateDescription" => {
                    obj.template_description = Some(TemplateDescriptionDeserializer::deserialize(
                        "TemplateDescription",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StacksDeserializer;
impl StacksDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Stack>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(StackDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct StageListDeserializer;
impl StageListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TemplateStageDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StopStackSetOperationRequest {
    /// <p>The ID of the stack operation. </p>
    pub operation_id: String,
    /// <p>The name or unique ID of the stack set that you want to stop the operation for.</p>
    pub stack_set_name: String,
}

/// Serialize `StopStackSetOperationRequest` contents to a `SignedRequest`.
struct StopStackSetOperationRequestSerializer;
impl StopStackSetOperationRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StopStackSetOperationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "OperationId"), &obj.operation_id);
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StopStackSetOperationResponse {}

struct StopStackSetOperationResponseDeserializer;
impl StopStackSetOperationResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StopStackSetOperationResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = StopStackSetOperationResponse::default();

        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = TagKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = TagValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
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

        params.put(&format!("{}{}", prefix, "Key"), &obj.key);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TagsDeserializer;
impl TagsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TemplateDescriptionDeserializer;
impl TemplateDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

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
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TemplateParameter, XmlParseError> {
        deserialize_elements::<_, TemplateParameter, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DefaultValue" => {
                    obj.default_value = Some(ParameterValueDeserializer::deserialize(
                        "DefaultValue",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description =
                        Some(DescriptionDeserializer::deserialize("Description", stack)?);
                }
                "NoEcho" => {
                    obj.no_echo = Some(NoEchoDeserializer::deserialize("NoEcho", stack)?);
                }
                "ParameterKey" => {
                    obj.parameter_key = Some(ParameterKeyDeserializer::deserialize(
                        "ParameterKey",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct TemplateParametersDeserializer;
impl TemplateParametersDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<TemplateParameter>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TemplateParameterDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TemplateStageDeserializer;
impl TemplateStageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimeoutMinutesDeserializer;
impl TimeoutMinutesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TransformNameDeserializer;
impl TransformNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TransformsListDeserializer;
impl TransformsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TransformNameDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackInstancesRequest {
    /// <p>The names of one or more AWS accounts for which you want to update parameter values for stack instances. The overridden parameter values will be applied to all stack instances in the specified accounts and regions.</p>
    pub accounts: Vec<String>,
    /// <p>The unique identifier for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, the SDK generates one automatically. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p> A list of input parameters whose values you want to update for the specified stack instances. </p> <p>Any overridden parameter values will be applied to all stack instances in the specified accounts and regions. When specifying parameters and their values, be aware of how AWS CloudFormation sets parameter values during stack instance update operations:</p> <ul> <li> <p>To override the current value for a parameter, include the parameter and specify its value.</p> </li> <li> <p>To leave a parameter set to its present value, you can do one of the following:</p> <ul> <li> <p>Do not include the parameter in the list.</p> </li> <li> <p>Include the parameter and specify <code>UsePreviousValue</code> as <code>true</code>. (You cannot specify both a value and set <code>UsePreviousValue</code> to <code>true</code>.)</p> </li> </ul> </li> <li> <p>To set all overridden parameter back to the values specified in the stack set, specify a parameter list but do not include any parameters.</p> </li> <li> <p>To leave all parameters set to their present values, do not specify this property at all.</p> </li> </ul> <p>During stack set updates, any parameter values overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only override the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <code>UpdateStackSet</code> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    pub parameter_overrides: Option<Vec<Parameter>>,
    /// <p>The names of one or more regions in which you want to update parameter values for stack instances. The overridden parameter values will be applied to all stack instances in the specified accounts and regions.</p>
    pub regions: Vec<String>,
    /// <p>The name or unique ID of the stack set associated with the stack instances.</p>
    pub stack_set_name: String,
}

/// Serialize `UpdateStackInstancesRequest` contents to a `SignedRequest`.
struct UpdateStackInstancesRequestSerializer;
impl UpdateStackInstancesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackInstancesRequest) {
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
            params.put(&format!("{}{}", prefix, "OperationId"), &field_value);
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
            &obj.stack_set_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackInstancesResponse {
    /// <p>The unique identifier for this stack set operation. </p>
    pub operation_id: Option<String>,
}

struct UpdateStackInstancesResponseDeserializer;
impl UpdateStackInstancesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackInstancesResponse, XmlParseError> {
        deserialize_elements::<_, UpdateStackInstancesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "OperationId" => {
                        obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                            "OperationId",
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
/// <p>The input for an <a>UpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackRequest {
    /// <p><p>In some cases, you must explicity acknowledge that your stack template contains certain capabilities in order for AWS CloudFormation to update the stack.</p> <ul> <li> <p> <code>CAPABILITY<em>IAM</code> and <code>CAPABILITY</em>NAMED<em>IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your AWS account; for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY</em>IAM</code> or <code>CAPABILITY<em>NAMED</em>IAM</code> capability.</p> <ul> <li> <p>If you have IAM resources, you can specify either capability. </p> </li> <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY<em>NAMED</em>IAM</code>. </p> </li> <li> <p>If you don&#39;t specify either of these capabilities, AWS CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p> </li> <li> <p> <code>CAPABILITY<em>AUTO</em>EXPAND</code> </p> <p>Some template contain macros. Macros perform custom processing on templates; this can include simple actions like find-and-replace operations, all the way to extensive transformations of entire templates. Because of this, users typically create a change set from the processed template, so that they can review the changes resulting from the macros before actually updating the stack. If your stack template contains one or more macros, and you choose to update a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by AWS CloudFormation.</p> <p>Change sets do not currently support nested stacks. If you want to update a stack from a stack template that contains macros <i>and</i> nested stacks, you must update the stack directly from the template using this capability.</p> <important> <p>You should only update stacks directly from a stack template that contains macros if you know what processing the macro performs.</p> <p>Each macro relies on an underlying Lambda service function for processing stack templates. Be aware that the Lambda function owner can update the function operation without AWS CloudFormation being notified.</p> </important> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using AWS CloudFormation Macros to Perform Custom Processing on Templates</a>.</p> </li> </ul></p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A unique identifier for this <code>UpdateStack</code> request. Specify this token if you plan to retry requests so that AWS CloudFormation knows that you're not attempting to update a stack with the same name. You might retry <code>UpdateStack</code> requests to ensure that AWS CloudFormation successfully received them.</p> <p>All events triggered by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p> <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>. </p>
    pub client_request_token: Option<String>,
    /// <p>Amazon Simple Notification Service topic Amazon Resource Names (ARNs) that AWS CloudFormation associates with the stack. Specify an empty list to remove all notification topics.</p>
    pub notification_ar_ns: Option<Vec<String>>,
    /// <p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The template resource types that you have permissions to work with for this update stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>.</p> <p>If the list of resource types doesn't include a resource that you're updating, the stack update fails. By default, AWS CloudFormation grants permissions to all resource types. AWS Identity and Access Management (IAM) uses this parameter for AWS CloudFormation-specific condition keys in IAM policies. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with AWS Identity and Access Management</a>.</p>
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
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. (For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.)</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template that is located in an Amazon S3 bucket. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
    pub template_url: Option<String>,
    /// <p>Reuse the existing template that is associated with the stack that you are updating.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code>, <code>TemplateURL</code>, or set the <code>UsePreviousTemplate</code> to <code>true</code>.</p>
    pub use_previous_template: Option<bool>,
}

/// Serialize `UpdateStackRequest` contents to a `SignedRequest`.
struct UpdateStackRequestSerializer;
impl UpdateStackRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackRequest) {
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
            params.put(&format!("{}{}", prefix, "ClientRequestToken"), &field_value);
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
            params.put(&format!("{}{}", prefix, "RoleARN"), &field_value);
        }
        if let Some(ref field_value) = obj.rollback_configuration {
            RollbackConfigurationSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RollbackConfiguration"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
        if let Some(ref field_value) = obj.stack_policy_body {
            params.put(&format!("{}{}", prefix, "StackPolicyBody"), &field_value);
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_body {
            params.put(
                &format!("{}{}", prefix, "StackPolicyDuringUpdateBody"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.stack_policy_during_update_url {
            params.put(
                &format!("{}{}", prefix, "StackPolicyDuringUpdateURL"),
                &field_value,
            );
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
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value,
            );
        }
    }
}

/// <p>The output for an <a>UpdateStack</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackResponse {
    /// <p>Unique identifier of the stack.</p>
    pub stack_id: Option<String>,
}

struct UpdateStackResponseDeserializer;
impl UpdateStackResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackResponse, XmlParseError> {
        deserialize_elements::<_, UpdateStackResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "StackId" => {
                    obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackSetRequest {
    /// <p>The accounts in which to update associated stack instances. If you specify accounts, you must also specify the regions in which to update stack set instances.</p> <p>To update <i>all</i> the stack instances associated with this stack set, do not specify the <code>Accounts</code> or <code>Regions</code> properties.</p> <p>If the stack set update includes changes to the template (that is, if the <code>TemplateBody</code> or <code>TemplateURL</code> properties are specified), or the <code>Parameters</code> property, AWS CloudFormation marks all stack instances with a status of <code>OUTDATED</code> prior to updating the stack instances in the specified accounts and regions. If the stack set update does not include changes to the template or parameters, AWS CloudFormation updates the stack instances in the specified accounts and regions, while leaving all other stack instances with their existing stack instance status. </p>
    pub accounts: Option<Vec<String>>,
    /// <p>The Amazon Resource Number (ARN) of the IAM role to use to update this stack set.</p> <p>Specify an IAM role only if you are using customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html">Granting Permissions for Stack Set Operations</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p>If you specified a customized administrator role when you created the stack set, you must specify a customized administrator role, even if it is the same customized administrator role used with this stack set previously.</p>
    pub administration_role_arn: Option<String>,
    /// <p><p>In some cases, you must explicity acknowledge that your stack template contains certain capabilities in order for AWS CloudFormation to update the stack set and its associated stack instances.</p> <ul> <li> <p> <code>CAPABILITY<em>IAM</code> and <code>CAPABILITY</em>NAMED<em>IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your AWS account; for example, by creating new AWS Identity and Access Management (IAM) users. For those stacks sets, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY</em>IAM</code> or <code>CAPABILITY<em>NAMED</em>IAM</code> capability.</p> <ul> <li> <p>If you have IAM resources, you can specify either capability. </p> </li> <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY<em>NAMED</em>IAM</code>. </p> </li> <li> <p>If you don&#39;t specify either of these capabilities, AWS CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li> </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html"> AWS::IAM::InstanceProfile</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html"> AWS::IAM::UserToGroupAddition</a> </p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p> </li> <li> <p> <code>CAPABILITY<em>AUTO</em>EXPAND</code> </p> <p>Some templates contain macros. If your stack template contains one or more macros, and you choose to update a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using AWS CloudFormation Macros to Perform Custom Processing on Templates</a>.</p> <important> <p>Stack sets do not currently support macros in stack templates. (This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by AWS CloudFormation.) Even if you specify this capability, if you include a macro in your template the stack set operation will fail.</p> </important> </li> </ul></p>
    pub capabilities: Option<Vec<String>>,
    /// <p>A brief description of updates that you are making.</p>
    pub description: Option<String>,
    /// <p>The name of the IAM execution role to use to update the stack set. If you do not specify an execution role, AWS CloudFormation uses the <code>AWSCloudFormationStackSetExecutionRole</code> role for the stack set operation.</p> <p>Specify an IAM role only if you are using customized execution roles to control which stack resources users and groups can include in their stack sets. </p> <p> If you specify a customized execution role, AWS CloudFormation uses that role to update the stack. If you do not specify a customized execution role, AWS CloudFormation performs the update using the role previously associated with the stack set, so long as you have permissions to perform operations on the stack set.</p>
    pub execution_role_name: Option<String>,
    /// <p>The unique ID for this stack set operation. </p> <p>The operation ID also functions as an idempotency token, to ensure that AWS CloudFormation performs the stack set operation only once, even if you retry the request multiple times. You might retry stack set operation requests to ensure that AWS CloudFormation successfully received them.</p> <p>If you don't specify an operation ID, AWS CloudFormation generates one automatically.</p> <p>Repeating this stack set operation with a new operation ID retries all stack instances whose status is <code>OUTDATED</code>. </p>
    pub operation_id: Option<String>,
    /// <p>Preferences for how AWS CloudFormation performs this stack set operation.</p>
    pub operation_preferences: Option<StackSetOperationPreferences>,
    /// <p>A list of input parameters for the stack set template. </p>
    pub parameters: Option<Vec<Parameter>>,
    /// <p>The regions in which to update associated stack instances. If you specify regions, you must also specify accounts in which to update stack set instances.</p> <p>To update <i>all</i> the stack instances associated with this stack set, do not specify the <code>Accounts</code> or <code>Regions</code> properties.</p> <p>If the stack set update includes changes to the template (that is, if the <code>TemplateBody</code> or <code>TemplateURL</code> properties are specified), or the <code>Parameters</code> property, AWS CloudFormation marks all stack instances with a status of <code>OUTDATED</code> prior to updating the stack instances in the specified accounts and regions. If the stack set update does not include changes to the template or parameters, AWS CloudFormation updates the stack instances in the specified accounts and regions, while leaving all other stack instances with their existing stack instance status. </p>
    pub regions: Option<Vec<String>>,
    /// <p>The name or unique ID of the stack set that you want to update.</p>
    pub stack_set_name: String,
    /// <p>The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the stacks. You can specify a maximum number of 50 tags.</p> <p>If you specify tags for this parameter, those tags replace any list of tags that are currently associated with this stack set. This means:</p> <ul> <li> <p>If you don't specify this parameter, AWS CloudFormation doesn't modify the stack's tags. </p> </li> <li> <p>If you specify <i>any</i> tags using this parameter, you must specify <i>all</i> the tags that you want associated with this stack set, even tags you've specifed before (for example, when creating the stack set or during a previous update of the stack set.). Any tags that you don't include in the updated list of tags are removed from the stack set, and therefore from the stacks and resources as well. </p> </li> <li> <p>If you specify an empty value, AWS CloudFormation removes all currently associated tags.</p> </li> </ul> <p>If you specify new tags as part of an <code>UpdateStackSet</code> action, AWS CloudFormation checks to see if you have the required IAM permission to tag resources. If you omit tags that are currently associated with the stack set from the list of tags you specify, AWS CloudFormation assumes that you want to remove those tags from the stack set, and checks to see if you have permission to untag resources. If you don't have the necessary permission(s), the entire <code>UpdateStackSet</code> action fails with an <code>access denied</code> error, and the stack set is not updated.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true.</p>
    pub template_body: Option<String>,
    /// <p>The location of the file that contains the template body. The URL must point to a template (maximum size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true. </p>
    pub template_url: Option<String>,
    /// <p>Use the existing template that's associated with the stack set that you're updating.</p> <p>Conditional: You must specify only one of the following parameters: <code>TemplateBody</code> or <code>TemplateURL</code>—or set <code>UsePreviousTemplate</code> to true. </p>
    pub use_previous_template: Option<bool>,
}

/// Serialize `UpdateStackSetRequest` contents to a `SignedRequest`.
struct UpdateStackSetRequestSerializer;
impl UpdateStackSetRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateStackSetRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.accounts {
            AccountListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Accounts"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.administration_role_arn {
            params.put(
                &format!("{}{}", prefix, "AdministrationRoleARN"),
                &field_value,
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
            params.put(&format!("{}{}", prefix, "Description"), &field_value);
        }
        if let Some(ref field_value) = obj.execution_role_name {
            params.put(&format!("{}{}", prefix, "ExecutionRoleName"), &field_value);
        }
        if let Some(ref field_value) = obj.operation_id {
            params.put(&format!("{}{}", prefix, "OperationId"), &field_value);
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
        if let Some(ref field_value) = obj.regions {
            RegionListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Regions"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "StackSetName"),
            &obj.stack_set_name,
        );
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
            params.put(
                &format!("{}{}", prefix, "UsePreviousTemplate"),
                &field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStackSetResponse {
    /// <p>The unique ID for this stack set operation.</p>
    pub operation_id: Option<String>,
}

struct UpdateStackSetResponseDeserializer;
impl UpdateStackSetResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateStackSetResponse, XmlParseError> {
        deserialize_elements::<_, UpdateStackSetResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "OperationId" => {
                    obj.operation_id = Some(ClientRequestTokenDeserializer::deserialize(
                        "OperationId",
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
pub struct UpdateTerminationProtectionRequest {
    /// <p>Whether to enable termination protection on the specified stack.</p>
    pub enable_termination_protection: bool,
    /// <p>The name or unique ID of the stack for which you want to set termination protection.</p>
    pub stack_name: String,
}

/// Serialize `UpdateTerminationProtectionRequest` contents to a `SignedRequest`.
struct UpdateTerminationProtectionRequestSerializer;
impl UpdateTerminationProtectionRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &UpdateTerminationProtectionRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "EnableTerminationProtection"),
            &obj.enable_termination_protection,
        );
        params.put(&format!("{}{}", prefix, "StackName"), &obj.stack_name);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateTerminationProtectionResponse {
    /// <p>The unique ID of the stack.</p>
    pub stack_id: Option<String>,
}

struct UpdateTerminationProtectionResponseDeserializer;
impl UpdateTerminationProtectionResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateTerminationProtectionResponse, XmlParseError> {
        deserialize_elements::<_, UpdateTerminationProtectionResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "StackId" => {
                        obj.stack_id = Some(StackIdDeserializer::deserialize("StackId", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct UrlDeserializer;
impl UrlDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct UsePreviousValueDeserializer;
impl UsePreviousValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>The input for <a>ValidateTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidateTemplateRequest {
    /// <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_body: Option<String>,
    /// <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template Anatomy</a> in the AWS CloudFormation User Guide.</p> <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_url: Option<String>,
}

/// Serialize `ValidateTemplateRequest` contents to a `SignedRequest`.
struct ValidateTemplateRequestSerializer;
impl ValidateTemplateRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ValidateTemplateRequest) {
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

/// <p>The output for <a>ValidateTemplate</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ValidateTemplateResponse {
    /// <p>The capabilities found within the template. If your template contains IAM resources, you must specify the CAPABILITY_IAM or CAPABILITY_NAMED_IAM value for this parameter when you use the <a>CreateStack</a> or <a>UpdateStack</a> actions with your template; otherwise, those actions return an InsufficientCapabilities error.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in AWS CloudFormation Templates</a>.</p>
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

struct ValidateTemplateResponseDeserializer;
impl ValidateTemplateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ValidateTemplateResponse, XmlParseError> {
        deserialize_elements::<_, ValidateTemplateResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Capabilities" => {
                        obj.capabilities.get_or_insert(vec![]).extend(
                            CapabilitiesDeserializer::deserialize("Capabilities", stack)?,
                        );
                    }
                    "CapabilitiesReason" => {
                        obj.capabilities_reason =
                            Some(CapabilitiesReasonDeserializer::deserialize(
                                "CapabilitiesReason",
                                stack,
                            )?);
                    }
                    "DeclaredTransforms" => {
                        obj.declared_transforms.get_or_insert(vec![]).extend(
                            TransformsListDeserializer::deserialize("DeclaredTransforms", stack)?,
                        );
                    }
                    "Description" => {
                        obj.description =
                            Some(DescriptionDeserializer::deserialize("Description", stack)?);
                    }
                    "Parameters" => {
                        obj.parameters.get_or_insert(vec![]).extend(
                            TemplateParametersDeserializer::deserialize("Parameters", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
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
struct VersionDeserializer;
impl VersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// Errors returned by CancelUpdateStack
#[derive(Debug, PartialEq)]
pub enum CancelUpdateStackError {
    /// <p>A client request token already exists.</p>
    TokenAlreadyExists(String),
}

impl CancelUpdateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelUpdateStackError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(CancelUpdateStackError::TokenAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ContinueUpdateRollback
#[derive(Debug, PartialEq)]
pub enum ContinueUpdateRollbackError {
    /// <p>A client request token already exists.</p>
    TokenAlreadyExists(String),
}

impl ContinueUpdateRollbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ContinueUpdateRollbackError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(
                            ContinueUpdateRollbackError::TokenAlreadyExists(parsed_error.message),
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
    /// <p>The quota for the resource has already been reached.</p> <p>For information on resource and stack limitations, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    LimitExceeded(String),
}

impl CreateChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChangeSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExistsException" => {
                        return RusotoError::Service(CreateChangeSetError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientCapabilitiesException" => {
                        return RusotoError::Service(
                            CreateChangeSetError::InsufficientCapabilities(parsed_error.message),
                        )
                    }
                    "LimitExceededException" => {
                        return RusotoError::Service(CreateChangeSetError::LimitExceeded(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
    /// <p>The quota for the resource has already been reached.</p> <p>For information on resource and stack limitations, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>A client request token already exists.</p>
    TokenAlreadyExists(String),
}

impl CreateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStackError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AlreadyExistsException" => {
                        return RusotoError::Service(CreateStackError::AlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientCapabilitiesException" => {
                        return RusotoError::Service(CreateStackError::InsufficientCapabilities(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceededException" => {
                        return RusotoError::Service(CreateStackError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(CreateStackError::TokenAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by CreateStackInstances
#[derive(Debug, PartialEq)]
pub enum CreateStackInstancesError {
    /// <p>The specified operation isn't valid.</p>
    InvalidOperation(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on resource and stack limitations, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified operation ID already exists.</p>
    OperationIdAlreadyExists(String),
    /// <p>Another operation is currently in progress for this stack set. Only one operation can be performed for a stack set at a given time.</p>
    OperationInProgress(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
}

impl CreateStackInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStackInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidOperationException" => {
                        return RusotoError::Service(CreateStackInstancesError::InvalidOperation(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceededException" => {
                        return RusotoError::Service(CreateStackInstancesError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "OperationIdAlreadyExistsException" => {
                        return RusotoError::Service(
                            CreateStackInstancesError::OperationIdAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OperationInProgressException" => {
                        return RusotoError::Service(
                            CreateStackInstancesError::OperationInProgress(parsed_error.message),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(CreateStackInstancesError::StackSetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleRequestException" => {
                        return RusotoError::Service(CreateStackInstancesError::StaleRequest(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by CreateStackSet
#[derive(Debug, PartialEq)]
pub enum CreateStackSetError {
    /// <p>The specified resource exists, but has been changed.</p>
    CreatedButModified(String),
    /// <p>The quota for the resource has already been reached.</p> <p>For information on resource and stack limitations, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The specified name is already in use.</p>
    NameAlreadyExists(String),
}

impl CreateStackSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStackSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CreatedButModifiedException" => {
                        return RusotoError::Service(CreateStackSetError::CreatedButModified(
                            parsed_error.message,
                        ))
                    }
                    "LimitExceededException" => {
                        return RusotoError::Service(CreateStackSetError::LimitExceeded(
                            parsed_error.message,
                        ))
                    }
                    "NameAlreadyExistsException" => {
                        return RusotoError::Service(CreateStackSetError::NameAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DeleteChangeSet
#[derive(Debug, PartialEq)]
pub enum DeleteChangeSetError {
    /// <p>The specified change set can't be used to update the stack. For example, the change set status might be <code>CREATE_IN_PROGRESS</code>, or the stack status might be <code>UPDATE_IN_PROGRESS</code>.</p>
    InvalidChangeSetStatus(String),
}

impl DeleteChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChangeSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidChangeSetStatus" => {
                        return RusotoError::Service(DeleteChangeSetError::InvalidChangeSetStatus(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DeleteStack
#[derive(Debug, PartialEq)]
pub enum DeleteStackError {
    /// <p>A client request token already exists.</p>
    TokenAlreadyExists(String),
}

impl DeleteStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStackError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(DeleteStackError::TokenAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
}

impl DeleteStackInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStackInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidOperationException" => {
                        return RusotoError::Service(DeleteStackInstancesError::InvalidOperation(
                            parsed_error.message,
                        ))
                    }
                    "OperationIdAlreadyExistsException" => {
                        return RusotoError::Service(
                            DeleteStackInstancesError::OperationIdAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OperationInProgressException" => {
                        return RusotoError::Service(
                            DeleteStackInstancesError::OperationInProgress(parsed_error.message),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(DeleteStackInstancesError::StackSetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleRequestException" => {
                        return RusotoError::Service(DeleteStackInstancesError::StaleRequest(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
}

impl DeleteStackSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStackSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "OperationInProgressException" => {
                        return RusotoError::Service(DeleteStackSetError::OperationInProgress(
                            parsed_error.message,
                        ))
                    }
                    "StackSetNotEmptyException" => {
                        return RusotoError::Service(DeleteStackSetError::StackSetNotEmpty(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DescribeAccountLimits
#[derive(Debug, PartialEq)]
pub enum DescribeAccountLimitsError {}

impl DescribeAccountLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountLimitsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAccountLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountLimitsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeChangeSet
#[derive(Debug, PartialEq)]
pub enum DescribeChangeSetError {
    /// <p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
}

impl DescribeChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChangeSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ChangeSetNotFound" => {
                        return RusotoError::Service(DescribeChangeSetError::ChangeSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DescribeStackDriftDetectionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeStackDriftDetectionStatusError {}

impl DescribeStackDriftDetectionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeStackDriftDetectionStatusError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStackDriftDetectionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackDriftDetectionStatusError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeStackEvents
#[derive(Debug, PartialEq)]
pub enum DescribeStackEventsError {}

impl DescribeStackEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackEventsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStackEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackEventsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeStackInstance
#[derive(Debug, PartialEq)]
pub enum DescribeStackInstanceError {
    /// <p>The specified stack instance doesn't exist.</p>
    StackInstanceNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl DescribeStackInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackInstanceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "StackInstanceNotFoundException" => {
                        return RusotoError::Service(
                            DescribeStackInstanceError::StackInstanceNotFound(parsed_error.message),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(DescribeStackInstanceError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DescribeStackResource
#[derive(Debug, PartialEq)]
pub enum DescribeStackResourceError {}

impl DescribeStackResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackResourceError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStackResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackResourceError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeStackResourceDrifts
#[derive(Debug, PartialEq)]
pub enum DescribeStackResourceDriftsError {}

impl DescribeStackResourceDriftsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeStackResourceDriftsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStackResourceDriftsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackResourceDriftsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeStackResources
#[derive(Debug, PartialEq)]
pub enum DescribeStackResourcesError {}

impl DescribeStackResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackResourcesError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStackResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStackResourcesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeStackSet
#[derive(Debug, PartialEq)]
pub enum DescribeStackSetError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl DescribeStackSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(DescribeStackSetError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
}

impl DescribeStackSetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStackSetOperationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "OperationNotFoundException" => {
                        return RusotoError::Service(
                            DescribeStackSetOperationError::OperationNotFound(parsed_error.message),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(
                            DescribeStackSetOperationError::StackSetNotFound(parsed_error.message),
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by DescribeStacks
#[derive(Debug, PartialEq)]
pub enum DescribeStacksError {}

impl DescribeStacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStacksError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStacksError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DetectStackDrift
#[derive(Debug, PartialEq)]
pub enum DetectStackDriftError {}

impl DetectStackDriftError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectStackDriftError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DetectStackDriftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectStackDriftError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DetectStackResourceDrift
#[derive(Debug, PartialEq)]
pub enum DetectStackResourceDriftError {}

impl DetectStackResourceDriftError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectStackResourceDriftError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DetectStackResourceDriftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectStackResourceDriftError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by EstimateTemplateCost
#[derive(Debug, PartialEq)]
pub enum EstimateTemplateCostError {}

impl EstimateTemplateCostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EstimateTemplateCostError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for EstimateTemplateCostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EstimateTemplateCostError {
    fn description(&self) -> &str {
        match *self {}
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
}

impl ExecuteChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecuteChangeSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ChangeSetNotFound" => {
                        return RusotoError::Service(ExecuteChangeSetError::ChangeSetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "InsufficientCapabilitiesException" => {
                        return RusotoError::Service(
                            ExecuteChangeSetError::InsufficientCapabilities(parsed_error.message),
                        )
                    }
                    "InvalidChangeSetStatus" => {
                        return RusotoError::Service(ExecuteChangeSetError::InvalidChangeSetStatus(
                            parsed_error.message,
                        ))
                    }
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(ExecuteChangeSetError::TokenAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by GetStackPolicy
#[derive(Debug, PartialEq)]
pub enum GetStackPolicyError {}

impl GetStackPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStackPolicyError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetStackPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStackPolicyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetTemplate
#[derive(Debug, PartialEq)]
pub enum GetTemplateError {
    /// <p>The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the <code>ListChangeSets</code> action.</p>
    ChangeSetNotFound(String),
}

impl GetTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTemplateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ChangeSetNotFound" => {
                        return RusotoError::Service(GetTemplateError::ChangeSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by GetTemplateSummary
#[derive(Debug, PartialEq)]
pub enum GetTemplateSummaryError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl GetTemplateSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTemplateSummaryError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(GetTemplateSummaryError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListChangeSets
#[derive(Debug, PartialEq)]
pub enum ListChangeSetsError {}

impl ListChangeSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChangeSetsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListChangeSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChangeSetsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListExports
#[derive(Debug, PartialEq)]
pub enum ListExportsError {}

impl ListExportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListExportsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListExportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListExportsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListImports
#[derive(Debug, PartialEq)]
pub enum ListImportsError {}

impl ListImportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListImportsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListImportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListImportsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListStackInstances
#[derive(Debug, PartialEq)]
pub enum ListStackInstancesError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl ListStackInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStackInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(ListStackInstancesError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListStackResources
#[derive(Debug, PartialEq)]
pub enum ListStackResourcesError {}

impl ListStackResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStackResourcesError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListStackResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackResourcesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListStackSetOperationResults
#[derive(Debug, PartialEq)]
pub enum ListStackSetOperationResultsError {
    /// <p>The specified ID refers to an operation that doesn't exist.</p>
    OperationNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl ListStackSetOperationResultsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListStackSetOperationResultsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "OperationNotFoundException" => {
                        return RusotoError::Service(
                            ListStackSetOperationResultsError::OperationNotFound(
                                parsed_error.message,
                            ),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(
                            ListStackSetOperationResultsError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListStackSetOperations
#[derive(Debug, PartialEq)]
pub enum ListStackSetOperationsError {
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
}

impl ListStackSetOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStackSetOperationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(ListStackSetOperationsError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
        }
    }
}
/// Errors returned by ListStackSets
#[derive(Debug, PartialEq)]
pub enum ListStackSetsError {}

impl ListStackSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStackSetsError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListStackSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStackSetsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListStacks
#[derive(Debug, PartialEq)]
pub enum ListStacksError {}

impl ListStacksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStacksError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListStacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStacksError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by SetStackPolicy
#[derive(Debug, PartialEq)]
pub enum SetStackPolicyError {}

impl SetStackPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetStackPolicyError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetStackPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetStackPolicyError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by SignalResource
#[derive(Debug, PartialEq)]
pub enum SignalResourceError {}

impl SignalResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignalResourceError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SignalResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SignalResourceError {
    fn description(&self) -> &str {
        match *self {}
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
}

impl StopStackSetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopStackSetOperationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidOperationException" => {
                        return RusotoError::Service(StopStackSetOperationError::InvalidOperation(
                            parsed_error.message,
                        ))
                    }
                    "OperationNotFoundException" => {
                        return RusotoError::Service(StopStackSetOperationError::OperationNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(StopStackSetOperationError::StackSetNotFound(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
}

impl UpdateStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStackError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InsufficientCapabilitiesException" => {
                        return RusotoError::Service(UpdateStackError::InsufficientCapabilities(
                            parsed_error.message,
                        ))
                    }
                    "TokenAlreadyExistsException" => {
                        return RusotoError::Service(UpdateStackError::TokenAlreadyExists(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
}

impl UpdateStackInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStackInstancesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidOperationException" => {
                        return RusotoError::Service(UpdateStackInstancesError::InvalidOperation(
                            parsed_error.message,
                        ))
                    }
                    "OperationIdAlreadyExistsException" => {
                        return RusotoError::Service(
                            UpdateStackInstancesError::OperationIdAlreadyExists(
                                parsed_error.message,
                            ),
                        )
                    }
                    "OperationInProgressException" => {
                        return RusotoError::Service(
                            UpdateStackInstancesError::OperationInProgress(parsed_error.message),
                        )
                    }
                    "StackInstanceNotFoundException" => {
                        return RusotoError::Service(
                            UpdateStackInstancesError::StackInstanceNotFound(parsed_error.message),
                        )
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(UpdateStackInstancesError::StackSetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleRequestException" => {
                        return RusotoError::Service(UpdateStackInstancesError::StaleRequest(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
    /// <p>The specified stack instance doesn't exist.</p>
    StackInstanceNotFound(String),
    /// <p>The specified stack set doesn't exist.</p>
    StackSetNotFound(String),
    /// <p>Another operation has been performed on this stack set since the specified operation was performed. </p>
    StaleRequest(String),
}

impl UpdateStackSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStackSetError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidOperationException" => {
                        return RusotoError::Service(UpdateStackSetError::InvalidOperation(
                            parsed_error.message,
                        ))
                    }
                    "OperationIdAlreadyExistsException" => {
                        return RusotoError::Service(UpdateStackSetError::OperationIdAlreadyExists(
                            parsed_error.message,
                        ))
                    }
                    "OperationInProgressException" => {
                        return RusotoError::Service(UpdateStackSetError::OperationInProgress(
                            parsed_error.message,
                        ))
                    }
                    "StackInstanceNotFoundException" => {
                        return RusotoError::Service(UpdateStackSetError::StackInstanceNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StackSetNotFoundException" => {
                        return RusotoError::Service(UpdateStackSetError::StackSetNotFound(
                            parsed_error.message,
                        ))
                    }
                    "StaleRequestException" => {
                        return RusotoError::Service(UpdateStackSetError::StaleRequest(
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
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
            UpdateStackSetError::StackInstanceNotFound(ref cause) => cause,
            UpdateStackSetError::StackSetNotFound(ref cause) => cause,
            UpdateStackSetError::StaleRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTerminationProtection
#[derive(Debug, PartialEq)]
pub enum UpdateTerminationProtectionError {}

impl UpdateTerminationProtectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateTerminationProtectionError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UpdateTerminationProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTerminationProtectionError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ValidateTemplate
#[derive(Debug, PartialEq)]
pub enum ValidateTemplateError {}

impl ValidateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ValidateTemplateError> {
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
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ValidateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ValidateTemplateError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Trait representing the capabilities of the AWS CloudFormation API. AWS CloudFormation clients implement this trait.
pub trait CloudFormation {
    /// <p><p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE<em>IN</em>PROGRESS state.</p> </note></p>
    fn cancel_update_stack(
        &self,
        input: CancelUpdateStackRequest,
    ) -> Request<CancelUpdateStackRequest>;

    /// <p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>
    fn continue_update_rollback(
        &self,
        input: ContinueUpdateRollbackRequest,
    ) -> Request<ContinueUpdateRollbackRequest>;

    /// <p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>
    fn create_change_set(&self, input: CreateChangeSetRequest) -> Request<CreateChangeSetRequest>;

    /// <p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>
    fn create_stack(&self, input: CreateStackRequest) -> Request<CreateStackRequest>;

    /// <p>Creates stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. <code>Accounts</code> and <code>Regions</code> are required parameters—you must specify at least one account and one region. </p>
    fn create_stack_instances(
        &self,
        input: CreateStackInstancesRequest,
    ) -> Request<CreateStackInstancesRequest>;

    /// <p>Creates a stack set.</p>
    fn create_stack_set(&self, input: CreateStackSetRequest) -> Request<CreateStackSetRequest>;

    /// <p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>
    fn delete_change_set(&self, input: DeleteChangeSetRequest) -> Request<DeleteChangeSetRequest>;

    /// <p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>
    fn delete_stack(&self, input: DeleteStackRequest) -> Request<DeleteStackRequest>;

    /// <p>Deletes stack instances for the specified accounts, in the specified regions. </p>
    fn delete_stack_instances(
        &self,
        input: DeleteStackInstancesRequest,
    ) -> Request<DeleteStackInstancesRequest>;

    /// <p>Deletes a stack set. Before you can delete a stack set, all of its member stack instances must be deleted. For more information about how to do this, see <a>DeleteStackInstances</a>. </p>
    fn delete_stack_set(&self, input: DeleteStackSetRequest) -> Request<DeleteStackSetRequest>;

    /// <p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account. For more information about account limits, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">AWS CloudFormation Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsRequest,
    ) -> Request<DescribeAccountLimitsRequest>;

    /// <p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>
    fn describe_change_set(
        &self,
        input: DescribeChangeSetRequest,
    ) -> Request<DescribeChangeSetRequest>;

    /// <p>Returns information about a stack drift detection operation. A stack drift detection operation detects whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. A stack is considered to have drifted if one or more of its resources have drifted. For more information on stack and resource drift, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <a>DetectStackDrift</a> to initiate a stack drift detection operation. <code>DetectStackDrift</code> returns a <code>StackDriftDetectionId</code> you can use to monitor the progress of the operation using <code>DescribeStackDriftDetectionStatus</code>. Once the drift detection operation has completed, use <a>DescribeStackResourceDrifts</a> to return drift information about the stack and its resources.</p>
    fn describe_stack_drift_detection_status(
        &self,
        input: DescribeStackDriftDetectionStatusRequest,
    ) -> Request<DescribeStackDriftDetectionStatusRequest>;

    /// <p><p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack&#39;s event history, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note></p>
    fn describe_stack_events(
        &self,
        input: DescribeStackEventsRequest,
    ) -> Request<DescribeStackEventsRequest>;

    /// <p>Returns the stack instance that's associated with the specified stack set, AWS account, and region.</p> <p>For a list of stack instances that are associated with a specific stack set, use <a>ListStackInstances</a>.</p>
    fn describe_stack_instance(
        &self,
        input: DescribeStackInstanceRequest,
    ) -> Request<DescribeStackInstanceRequest>;

    /// <p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>
    fn describe_stack_resource(
        &self,
        input: DescribeStackResourceRequest,
    ) -> Request<DescribeStackResourceRequest>;

    /// <p>Returns drift information for the resources that have been checked for drift in the specified stack. This includes actual and expected configuration values for resources where AWS CloudFormation detects configuration drift.</p> <p>For a given stack, there will be one <code>StackResourceDrift</code> for each stack resource that has been checked for drift. Resources that have not yet been checked for drift are not included. Resources that do not currently support drift detection are not checked, and so not included. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p> <p>Use <a>DetectStackResourceDrift</a> to detect drift on individual resources, or <a>DetectStackDrift</a> to detect drift on all supported resources for a given stack.</p>
    fn describe_stack_resource_drifts(
        &self,
        input: DescribeStackResourceDriftsRequest,
    ) -> Request<DescribeStackResourceDriftsRequest>;

    /// <p><p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note></p>
    fn describe_stack_resources(
        &self,
        input: DescribeStackResourcesRequest,
    ) -> Request<DescribeStackResourcesRequest>;

    /// <p>Returns the description of the specified stack set. </p>
    fn describe_stack_set(
        &self,
        input: DescribeStackSetRequest,
    ) -> Request<DescribeStackSetRequest>;

    /// <p>Returns the description of the specified stack set operation. </p>
    fn describe_stack_set_operation(
        &self,
        input: DescribeStackSetOperationRequest,
    ) -> Request<DescribeStackSetOperationRequest>;

    /// <p><p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note></p>
    fn describe_stacks(&self, input: DescribeStacksRequest) -> Request<DescribeStacksRequest>;

    /// <p>Detects whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. For each resource in the stack that supports drift detection, AWS CloudFormation compares the actual configuration of the resource with its expected template configuration. Only resource properties explicitly defined in the stack template are checked for drift. A stack is considered to have drifted if one or more of its resources differ from their expected template configurations. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <code>DetectStackDrift</code> to detect drift on all supported resources for a given stack, or <a>DetectStackResourceDrift</a> to detect drift on individual resources.</p> <p>For a list of stack resources that currently support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p> <p> <code>DetectStackDrift</code> can take up to several minutes, depending on the number of resources contained within the stack. Use <a>DescribeStackDriftDetectionStatus</a> to monitor the progress of a detect stack drift operation. Once the drift detection operation has completed, use <a>DescribeStackResourceDrifts</a> to return drift information about the stack and its resources.</p> <p>When detecting drift on a stack, AWS CloudFormation does not detect drift on any nested stacks belonging to that stack. Perform <code>DetectStackDrift</code> directly on the nested stack itself.</p>
    fn detect_stack_drift(
        &self,
        input: DetectStackDriftRequest,
    ) -> Request<DetectStackDriftRequest>;

    /// <p>Returns information about whether a resource's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. This information includes actual and expected property values for resources in which AWS CloudFormation detects drift. Only resource properties explicitly defined in the stack template are checked for drift. For more information about stack and resource drift, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <code>DetectStackResourceDrift</code> to detect drift on individual resources, or <a>DetectStackDrift</a> to detect drift on all resources in a given stack that support drift detection.</p> <p>Resources that do not currently support drift detection cannot be checked. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p>
    fn detect_stack_resource_drift(
        &self,
        input: DetectStackResourceDriftRequest,
    ) -> Request<DetectStackResourceDriftRequest>;

    /// <p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    fn estimate_template_cost(
        &self,
        input: EstimateTemplateCostRequest,
    ) -> Request<EstimateTemplateCostRequest>;

    /// <p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>
    fn execute_change_set(
        &self,
        input: ExecuteChangeSetRequest,
    ) -> Request<ExecuteChangeSetRequest>;

    /// <p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>
    fn get_stack_policy(&self, input: GetStackPolicyRequest) -> Request<GetStackPolicyRequest>;

    /// <p><p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note></p>
    fn get_template(&self, input: GetTemplateRequest) -> Request<GetTemplateRequest>;

    /// <p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack or stack set.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a stack set, or a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>
    fn get_template_summary(
        &self,
        input: GetTemplateSummaryRequest,
    ) -> Request<GetTemplateSummaryRequest>;

    /// <p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>
    fn list_change_sets(&self, input: ListChangeSetsRequest) -> Request<ListChangeSetsRequest>;

    /// <p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html"> AWS CloudFormation Export Stack Output Values</a>.</p>
    fn list_exports(&self, input: ListExportsRequest) -> Request<ListExportsRequest>;

    /// <p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p>
    fn list_imports(&self, input: ListImportsRequest) -> Request<ListImportsRequest>;

    /// <p>Returns summary information about stack instances that are associated with the specified stack set. You can filter for stack instances that are associated with a specific AWS account name or region.</p>
    fn list_stack_instances(
        &self,
        input: ListStackInstancesRequest,
    ) -> Request<ListStackInstancesRequest>;

    /// <p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>
    fn list_stack_resources(
        &self,
        input: ListStackResourcesRequest,
    ) -> Request<ListStackResourcesRequest>;

    /// <p>Returns summary information about the results of a stack set operation. </p>
    fn list_stack_set_operation_results(
        &self,
        input: ListStackSetOperationResultsRequest,
    ) -> Request<ListStackSetOperationResultsRequest>;

    /// <p>Returns summary information about operations performed on a stack set. </p>
    fn list_stack_set_operations(
        &self,
        input: ListStackSetOperationsRequest,
    ) -> Request<ListStackSetOperationsRequest>;

    /// <p>Returns summary information about stack sets that are associated with the user.</p>
    fn list_stack_sets(&self, input: ListStackSetsRequest) -> Request<ListStackSetsRequest>;

    /// <p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>
    fn list_stacks(&self, input: ListStacksRequest) -> Request<ListStacksRequest>;

    /// <p>Sets a stack policy for a specified stack.</p>
    fn set_stack_policy(&self, input: SetStackPolicyRequest) -> Request<SetStackPolicyRequest>;

    /// <p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>
    fn signal_resource(&self, input: SignalResourceRequest) -> Request<SignalResourceRequest>;

    /// <p>Stops an in-progress operation on a stack set and its associated stack instances. </p>
    fn stop_stack_set_operation(
        &self,
        input: StopStackSetOperationRequest,
    ) -> Request<StopStackSetOperationRequest>;

    /// <p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html">Updating a Stack</a>.</p>
    fn update_stack(&self, input: UpdateStackRequest) -> Request<UpdateStackRequest>;

    /// <p>Updates the parameter values for stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. </p> <p>You can only update stack instances in regions and accounts where they already exist; to create additional stack instances, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CreateStackInstances.html">CreateStackInstances</a>. </p> <p>During stack set updates, any parameters overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only update the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    fn update_stack_instances(
        &self,
        input: UpdateStackInstancesRequest,
    ) -> Request<UpdateStackInstancesRequest>;

    /// <p>Updates the stack set, and associated stack instances in the specified accounts and regions.</p> <p>Even if the stack set operation created by updating the stack set fails (completely or partially, below or above a specified failure tolerance), the stack set is updated with your changes. Subsequent <a>CreateStackInstances</a> calls on the specified stack set use the updated stack set.</p>
    fn update_stack_set(&self, input: UpdateStackSetRequest) -> Request<UpdateStackSetRequest>;

    /// <p>Updates termination protection for the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p> For <a href="AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack.</p>
    fn update_termination_protection(
        &self,
        input: UpdateTerminationProtectionRequest,
    ) -> Request<UpdateTerminationProtectionRequest>;

    /// <p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>
    fn validate_template(&self, input: ValidateTemplateRequest)
        -> Request<ValidateTemplateRequest>;
}
/// A client for the AWS CloudFormation API.
#[derive(Clone)]
pub struct CloudFormationClient {
    client: Client,
    region: region::Region,
}

impl CloudFormationClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudFormationClient {
        CloudFormationClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudFormationClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CloudFormationClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl CloudFormation for CloudFormationClient {
    /// <p><p>Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration.</p> <note> <p>You can cancel only stacks that are in the UPDATE<em>IN</em>PROGRESS state.</p> </note></p>
    fn cancel_update_stack(
        &self,
        input: CancelUpdateStackRequest,
    ) -> Request<CancelUpdateStackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>For a specified stack that is in the <code>UPDATE_ROLLBACK_FAILED</code> state, continues rolling it back to the <code>UPDATE_ROLLBACK_COMPLETE</code> state. Depending on the cause of the failure, you can manually <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/troubleshooting.html#troubleshooting-errors-update-rollback-failed"> fix the error</a> and continue the rollback. By continuing the rollback, you can return your stack to a working state (the <code>UPDATE_ROLLBACK_COMPLETE</code> state), and then try to update the stack again.</p> <p>A stack goes into the <code>UPDATE_ROLLBACK_FAILED</code> state when AWS CloudFormation cannot roll back all changes after a failed stack update. For example, you might have a stack that is rolling back to an old database instance that was deleted outside of AWS CloudFormation. Because AWS CloudFormation doesn't know the database was deleted, it assumes that the database instance still exists and attempts to roll back to it, causing the update rollback to fail.</p>
    fn continue_update_rollback(
        &self,
        input: ContinueUpdateRollbackRequest,
    ) -> Request<ContinueUpdateRollbackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a change set for a stack that doesn't exist, the change set shows all of the resources that AWS CloudFormation will create. If you create a change set for an existing stack, AWS CloudFormation compares the stack's information with the information that you submit in the change set and lists the differences. Use change sets to understand which resources AWS CloudFormation will create or change, and how it will change resources in an existing stack, before you create or update a stack.</p> <p>To create a change set for a stack that doesn't exist, for the <code>ChangeSetType</code> parameter, specify <code>CREATE</code>. To create a change set for an existing stack, specify <code>UPDATE</code> for the <code>ChangeSetType</code> parameter. After the <code>CreateChangeSet</code> call successfully completes, AWS CloudFormation starts creating the change set. To check the status of the change set or to review it, use the <a>DescribeChangeSet</a> action.</p> <p>When you are satisfied with the changes the change set will make, execute the change set by using the <a>ExecuteChangeSet</a> action. AWS CloudFormation doesn't make changes until you execute the change set.</p>
    fn create_change_set(&self, input: CreateChangeSetRequest) -> Request<CreateChangeSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack via the <a>DescribeStacks</a> API.</p>
    fn create_stack(&self, input: CreateStackRequest) -> Request<CreateStackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. <code>Accounts</code> and <code>Regions</code> are required parameters—you must specify at least one account and one region. </p>
    fn create_stack_instances(
        &self,
        input: CreateStackInstancesRequest,
    ) -> Request<CreateStackInstancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a stack set.</p>
    fn create_stack_set(&self, input: CreateStackSetRequest) -> Request<CreateStackSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set.</p> <p>If the call successfully completes, AWS CloudFormation successfully deleted the change set.</p>
    fn delete_change_set(&self, input: DeleteChangeSetRequest) -> Request<DeleteChangeSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks do not show up in the <a>DescribeStacks</a> API if the deletion has been completed successfully.</p>
    fn delete_stack(&self, input: DeleteStackRequest) -> Request<DeleteStackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes stack instances for the specified accounts, in the specified regions. </p>
    fn delete_stack_instances(
        &self,
        input: DeleteStackInstancesRequest,
    ) -> Request<DeleteStackInstancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes a stack set. Before you can delete a stack set, all of its member stack instances must be deleted. For more information about how to do this, see <a>DeleteStackInstances</a>. </p>
    fn delete_stack_set(&self, input: DeleteStackSetRequest) -> Request<DeleteStackSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves your account's AWS CloudFormation limits, such as the maximum number of stacks that you can create in your account. For more information about account limits, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">AWS CloudFormation Limits</a> in the <i>AWS CloudFormation User Guide</i>.</p>
    fn describe_account_limits(
        &self,
        input: DescribeAccountLimitsRequest,
    ) -> Request<DescribeAccountLimitsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the inputs for the change set and a list of changes that AWS CloudFormation will make if you execute the change set. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-changesets.html">Updating Stacks Using Change Sets</a> in the AWS CloudFormation User Guide.</p>
    fn describe_change_set(
        &self,
        input: DescribeChangeSetRequest,
    ) -> Request<DescribeChangeSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns information about a stack drift detection operation. A stack drift detection operation detects whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. A stack is considered to have drifted if one or more of its resources have drifted. For more information on stack and resource drift, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <a>DetectStackDrift</a> to initiate a stack drift detection operation. <code>DetectStackDrift</code> returns a <code>StackDriftDetectionId</code> you can use to monitor the progress of the operation using <code>DescribeStackDriftDetectionStatus</code>. Once the drift detection operation has completed, use <a>DescribeStackResourceDrifts</a> to return drift information about the stack and its resources.</p>
    fn describe_stack_drift_detection_status(
        &self,
        input: DescribeStackDriftDetectionStatusRequest,
    ) -> Request<DescribeStackDriftDetectionStatusRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack&#39;s event history, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/concept-stack.html">Stacks</a> in the AWS CloudFormation User Guide.</p> <note> <p>You can list events for stacks that have failed to create or have been deleted by specifying the unique stack identifier (stack ID).</p> </note></p>
    fn describe_stack_events(
        &self,
        input: DescribeStackEventsRequest,
    ) -> Request<DescribeStackEventsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the stack instance that's associated with the specified stack set, AWS account, and region.</p> <p>For a list of stack instances that are associated with a specific stack set, use <a>ListStackInstances</a>.</p>
    fn describe_stack_instance(
        &self,
        input: DescribeStackInstanceRequest,
    ) -> Request<DescribeStackInstanceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a description of the specified resource in the specified stack.</p> <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>
    fn describe_stack_resource(
        &self,
        input: DescribeStackResourceRequest,
    ) -> Request<DescribeStackResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns drift information for the resources that have been checked for drift in the specified stack. This includes actual and expected configuration values for resources where AWS CloudFormation detects configuration drift.</p> <p>For a given stack, there will be one <code>StackResourceDrift</code> for each stack resource that has been checked for drift. Resources that have not yet been checked for drift are not included. Resources that do not currently support drift detection are not checked, and so not included. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p> <p>Use <a>DetectStackResourceDrift</a> to detect drift on individual resources, or <a>DetectStackDrift</a> to detect drift on all supported resources for a given stack.</p>
    fn describe_stack_resource_drifts(
        &self,
        input: DescribeStackResourceDriftsRequest,
    ) -> Request<DescribeStackResourceDriftsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Returns AWS resource descriptions for running and deleted stacks. If <code>StackName</code> is specified, all the associated resources that are part of the stack are returned. If <code>PhysicalResourceId</code> is specified, the associated resources of the stack that the resource belongs to are returned.</p> <note> <p>Only the first 100 resources will be returned. If your stack has more resources than this, you should use <code>ListStackResources</code> instead.</p> </note> <p>For deleted stacks, <code>DescribeStackResources</code> returns resource information for up to 90 days after the stack has been deleted.</p> <p>You must specify either <code>StackName</code> or <code>PhysicalResourceId</code>, but not both. In addition, you can specify <code>LogicalResourceId</code> to filter the returned result. For more information about resources, the <code>LogicalResourceId</code> and <code>PhysicalResourceId</code>, go to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/">AWS CloudFormation User Guide</a>.</p> <note> <p>A <code>ValidationError</code> is returned if you specify both <code>StackName</code> and <code>PhysicalResourceId</code> in the same request.</p> </note></p>
    fn describe_stack_resources(
        &self,
        input: DescribeStackResourcesRequest,
    ) -> Request<DescribeStackResourcesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the description of the specified stack set. </p>
    fn describe_stack_set(
        &self,
        input: DescribeStackSetRequest,
    ) -> Request<DescribeStackSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the description of the specified stack set operation. </p>
    fn describe_stack_set_operation(
        &self,
        input: DescribeStackSetOperationRequest,
    ) -> Request<DescribeStackSetOperationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created.</p> <note> <p>If the stack does not exist, an <code>AmazonCloudFormationException</code> is returned.</p> </note></p>
    fn describe_stacks(&self, input: DescribeStacksRequest) -> Request<DescribeStacksRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Detects whether a stack's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. For each resource in the stack that supports drift detection, AWS CloudFormation compares the actual configuration of the resource with its expected template configuration. Only resource properties explicitly defined in the stack template are checked for drift. A stack is considered to have drifted if one or more of its resources differ from their expected template configurations. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <code>DetectStackDrift</code> to detect drift on all supported resources for a given stack, or <a>DetectStackResourceDrift</a> to detect drift on individual resources.</p> <p>For a list of stack resources that currently support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p> <p> <code>DetectStackDrift</code> can take up to several minutes, depending on the number of resources contained within the stack. Use <a>DescribeStackDriftDetectionStatus</a> to monitor the progress of a detect stack drift operation. Once the drift detection operation has completed, use <a>DescribeStackResourceDrifts</a> to return drift information about the stack and its resources.</p> <p>When detecting drift on a stack, AWS CloudFormation does not detect drift on any nested stacks belonging to that stack. Perform <code>DetectStackDrift</code> directly on the nested stack itself.</p>
    fn detect_stack_drift(
        &self,
        input: DetectStackDriftRequest,
    ) -> Request<DetectStackDriftRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns information about whether a resource's actual configuration differs, or has <i>drifted</i>, from it's expected configuration, as defined in the stack template and any values specified as template parameters. This information includes actual and expected property values for resources in which AWS CloudFormation detects drift. Only resource properties explicitly defined in the stack template are checked for drift. For more information about stack and resource drift, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detecting Unregulated Configuration Changes to Stacks and Resources</a>.</p> <p>Use <code>DetectStackResourceDrift</code> to detect drift on individual resources, or <a>DetectStackDrift</a> to detect drift on all resources in a given stack that support drift detection.</p> <p>Resources that do not currently support drift detection cannot be checked. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift-resource-list.html">Resources that Support Drift Detection</a>.</p>
    fn detect_stack_resource_drift(
        &self,
        input: DetectStackResourceDriftRequest,
    ) -> Request<DetectStackResourceDriftRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the estimated monthly cost of a template. The return value is an AWS Simple Monthly Calculator URL with a query string that describes the resources required to run the template.</p>
    fn estimate_template_cost(
        &self,
        input: EstimateTemplateCostRequest,
    ) -> Request<EstimateTemplateCostRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, AWS CloudFormation starts updating the stack. Use the <a>DescribeStacks</a> action to view the status of the update.</p> <p>When you execute a change set, AWS CloudFormation deletes all other change sets associated with the stack because they aren't valid for the updated stack.</p> <p>If a stack policy is associated with the stack, AWS CloudFormation enforces the policy during the update. You can't specify a temporary stack policy that overrides the current policy.</p>
    fn execute_change_set(
        &self,
        input: ExecuteChangeSetRequest,
    ) -> Request<ExecuteChangeSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned.</p>
    fn get_stack_policy(&self, input: GetStackPolicyRequest) -> Request<GetStackPolicyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Returns the template body for a specified stack. You can get the template for running or deleted stacks.</p> <p>For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted.</p> <note> <p> If the template does not exist, a <code>ValidationError</code> is returned. </p> </note></p>
    fn get_template(&self, input: GetTemplateRequest) -> Request<GetTemplateRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns information about a new or existing template. The <code>GetTemplateSummary</code> action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a stack or stack set.</p> <p>You can use the <code>GetTemplateSummary</code> action when you submit a template, or you can get template information for a stack set, or a running or deleted stack.</p> <p>For deleted stacks, <code>GetTemplateSummary</code> returns the template information for up to 90 days after the stack has been deleted. If the template does not exist, a <code>ValidationError</code> is returned.</p>
    fn get_template_summary(
        &self,
        input: GetTemplateSummaryRequest,
    ) -> Request<GetTemplateSummaryRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the ID and status of each active change set for a stack. For example, AWS CloudFormation lists change sets that are in the <code>CREATE_IN_PROGRESS</code> or <code>CREATE_PENDING</code> state.</p>
    fn list_change_sets(&self, input: ListChangeSetsRequest) -> Request<ListChangeSetsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all exported output values in the account and region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-exports.html"> AWS CloudFormation Export Stack Output Values</a>.</p>
    fn list_exports(&self, input: ListExportsRequest) -> Request<ListExportsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, see <a>ListExports</a>. </p> <p>For more information about importing an exported output value, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/intrinsic-function-reference-importvalue.html"> <code>Fn::ImportValue</code> </a> function. </p>
    fn list_imports(&self, input: ListImportsRequest) -> Request<ListImportsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns summary information about stack instances that are associated with the specified stack set. You can filter for stack instances that are associated with a specific AWS account name or region.</p>
    fn list_stack_instances(
        &self,
        input: ListStackInstancesRequest,
    ) -> Request<ListStackInstancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns descriptions of all resources of the specified stack.</p> <p>For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted.</p>
    fn list_stack_resources(
        &self,
        input: ListStackResourcesRequest,
    ) -> Request<ListStackResourcesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns summary information about the results of a stack set operation. </p>
    fn list_stack_set_operation_results(
        &self,
        input: ListStackSetOperationResultsRequest,
    ) -> Request<ListStackSetOperationResultsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns summary information about operations performed on a stack set. </p>
    fn list_stack_set_operations(
        &self,
        input: ListStackSetOperationsRequest,
    ) -> Request<ListStackSetOperationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns summary information about stack sets that are associated with the user.</p>
    fn list_stack_sets(&self, input: ListStackSetsRequest) -> Request<ListStackSetsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns the summary information for stacks whose status matches the specified StackStatusFilter. Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatusFilter is specified, summary information for all stacks is returned (including existing stacks and stacks that have been deleted).</p>
    fn list_stacks(&self, input: ListStacksRequest) -> Request<ListStacksRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sets a stack policy for a specified stack.</p>
    fn set_stack_policy(&self, input: SetStackPolicyRequest) -> Request<SetStackPolicyRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Sends a signal to the specified resource with a success or failure status. You can use the SignalResource API in conjunction with a creation policy or update policy. AWS CloudFormation doesn't proceed with a stack creation or update until resources receive the required number of signals or the timeout period is exceeded. The SignalResource API is useful in cases where you want to send signals from anywhere other than an Amazon EC2 instance.</p>
    fn signal_resource(&self, input: SignalResourceRequest) -> Request<SignalResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Stops an in-progress operation on a stack set and its associated stack instances. </p>
    fn stop_stack_set_operation(
        &self,
        input: StopStackSetOperationRequest,
    ) -> Request<StopStackSetOperationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack via the <a>DescribeStacks</a> action.</p> <p>To get a copy of the template for an existing stack, you can use the <a>GetTemplate</a> action.</p> <p>For more information about creating an update template, updating a stack, and monitoring the progress of the update, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks.html">Updating a Stack</a>.</p>
    fn update_stack(&self, input: UpdateStackRequest) -> Request<UpdateStackRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the parameter values for stack instances for the specified accounts, within the specified regions. A stack instance refers to a stack in a specific account and region. </p> <p>You can only update stack instances in regions and accounts where they already exist; to create additional stack instances, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_CreateStackInstances.html">CreateStackInstances</a>. </p> <p>During stack set updates, any parameters overridden for a stack instance are not updated, but retain their overridden value.</p> <p>You can only update the parameter <i>values</i> that are specified in the stack set; to add or delete a parameter itself, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update the stack set template. If you add a parameter to a template, before you can override the parameter value specified in the stack set you must first use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_UpdateStackSet.html">UpdateStackSet</a> to update all stack instances with the updated template and parameter value specified in the stack set. Once a stack instance has been updated with the new parameter, you can then override the parameter value using <code>UpdateStackInstances</code>.</p>
    fn update_stack_instances(
        &self,
        input: UpdateStackInstancesRequest,
    ) -> Request<UpdateStackInstancesRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates the stack set, and associated stack instances in the specified accounts and regions.</p> <p>Even if the stack set operation created by updating the stack set fails (completely or partially, below or above a specified failure tolerance), the stack set is updated with your changes. Subsequent <a>CreateStackInstances</a> calls on the specified stack set use the updated stack set.</p>
    fn update_stack_set(&self, input: UpdateStackSetRequest) -> Request<UpdateStackSetRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Updates termination protection for the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>AWS CloudFormation User Guide</i>.</p> <p> For <a href="AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and cannot be changed directly on the nested stack.</p>
    fn update_termination_protection(
        &self,
        input: UpdateTerminationProtectionRequest,
    ) -> Request<UpdateTerminationProtectionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Validates a specified template. AWS CloudFormation first checks if the template is valid JSON. If it isn't, AWS CloudFormation checks if the template is valid YAML. If both these checks fail, AWS CloudFormation returns a template validation error.</p>
    fn validate_template(
        &self,
        input: ValidateTemplateRequest,
    ) -> Request<ValidateTemplateRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for CancelUpdateStackRequest {
    type Output = CancelUpdateStackResponse;
    type Error = CancelUpdateStackError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "CancelUpdateStack");
        params.put("Version", "2010-05-15");
        CancelUpdateStackRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelUpdateStackError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CancelUpdateStackResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = CancelUpdateStackResponseDeserializer::deserialize(
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

impl ServiceRequest for ContinueUpdateRollbackRequest {
    type Output = ContinueUpdateRollbackResponse;
    type Error = ContinueUpdateRollbackError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ContinueUpdateRollback");
        params.put("Version", "2010-05-15");
        ContinueUpdateRollbackRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ContinueUpdateRollbackError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ContinueUpdateRollbackResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ContinueUpdateRollbackResponseDeserializer::deserialize(
                        "ContinueUpdateRollbackResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateChangeSetRequest {
    type Output = CreateChangeSetResponse;
    type Error = CreateChangeSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateChangeSet");
        params.put("Version", "2010-05-15");
        CreateChangeSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateChangeSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateChangeSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = CreateChangeSetResponseDeserializer::deserialize(
                        "CreateChangeSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateStackRequest {
    type Output = CreateStackResponse;
    type Error = CreateStackError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStack");
        params.put("Version", "2010-05-15");
        CreateStackRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateStackError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = CreateStackResponseDeserializer::deserialize(
                        "CreateStackResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateStackInstancesRequest {
    type Output = CreateStackInstancesResponse;
    type Error = CreateStackInstancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStackInstances");
        params.put("Version", "2010-05-15");
        CreateStackInstancesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateStackInstancesError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackInstancesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = CreateStackInstancesResponseDeserializer::deserialize(
                        "CreateStackInstancesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for CreateStackSetRequest {
    type Output = CreateStackSetResponse;
    type Error = CreateStackSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateStackSet");
        params.put("Version", "2010-05-15");
        CreateStackSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateStackSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = CreateStackSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = CreateStackSetResponseDeserializer::deserialize(
                        "CreateStackSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteChangeSetRequest {
    type Output = DeleteChangeSetResponse;
    type Error = DeleteChangeSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteChangeSet");
        params.put("Version", "2010-05-15");
        DeleteChangeSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteChangeSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteChangeSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DeleteChangeSetResponseDeserializer::deserialize(
                        "DeleteChangeSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteStackRequest {
    type Output = DeleteStackResponse;
    type Error = DeleteStackError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStack");
        params.put("Version", "2010-05-15");
        DeleteStackRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteStackError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteStackResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result =
                        DeleteStackResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteStackInstancesRequest {
    type Output = DeleteStackInstancesResponse;
    type Error = DeleteStackInstancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStackInstances");
        params.put("Version", "2010-05-15");
        DeleteStackInstancesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteStackInstancesError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteStackInstancesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DeleteStackInstancesResponseDeserializer::deserialize(
                        "DeleteStackInstancesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteStackSetRequest {
    type Output = DeleteStackSetResponse;
    type Error = DeleteStackSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteStackSet");
        params.put("Version", "2010-05-15");
        DeleteStackSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteStackSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteStackSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DeleteStackSetResponseDeserializer::deserialize(
                        "DeleteStackSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeAccountLimitsRequest {
    type Output = DescribeAccountLimitsResponse;
    type Error = DescribeAccountLimitsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAccountLimits");
        params.put("Version", "2010-05-15");
        DescribeAccountLimitsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountLimitsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAccountLimitsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeAccountLimitsResponseDeserializer::deserialize(
                        "DescribeAccountLimitsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeChangeSetRequest {
    type Output = DescribeChangeSetResponse;
    type Error = DescribeChangeSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeChangeSet");
        params.put("Version", "2010-05-15");
        DescribeChangeSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeChangeSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeChangeSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeChangeSetResponseDeserializer::deserialize(
                        "DescribeChangeSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackDriftDetectionStatusRequest {
    type Output = DescribeStackDriftDetectionStatusResponse;
    type Error = DescribeStackDriftDetectionStatusError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackDriftDetectionStatus");
        params.put("Version", "2010-05-15");
        DescribeStackDriftDetectionStatusRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackDriftDetectionStatusError::from_response(
                        response,
                    ))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackDriftDetectionStatusResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackDriftDetectionStatusResponseDeserializer::deserialize(
                        "DescribeStackDriftDetectionStatusResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackEventsRequest {
    type Output = DescribeStackEventsResponse;
    type Error = DescribeStackEventsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackEvents");
        params.put("Version", "2010-05-15");
        DescribeStackEventsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeStackEventsError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackEventsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackEventsResponseDeserializer::deserialize(
                        "DescribeStackEventsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackInstanceRequest {
    type Output = DescribeStackInstanceResponse;
    type Error = DescribeStackInstanceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackInstance");
        params.put("Version", "2010-05-15");
        DescribeStackInstanceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackInstanceError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackInstanceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackInstanceResponseDeserializer::deserialize(
                        "DescribeStackInstanceResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackResourceRequest {
    type Output = DescribeStackResourceResponse;
    type Error = DescribeStackResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResource");
        params.put("Version", "2010-05-15");
        DescribeStackResourceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackResourceError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackResourceResponseDeserializer::deserialize(
                        "DescribeStackResourceResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackResourceDriftsRequest {
    type Output = DescribeStackResourceDriftsResponse;
    type Error = DescribeStackResourceDriftsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResourceDrifts");
        params.put("Version", "2010-05-15");
        DescribeStackResourceDriftsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackResourceDriftsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackResourceDriftsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackResourceDriftsResponseDeserializer::deserialize(
                        "DescribeStackResourceDriftsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackResourcesRequest {
    type Output = DescribeStackResourcesResponse;
    type Error = DescribeStackResourcesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackResources");
        params.put("Version", "2010-05-15");
        DescribeStackResourcesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackResourcesError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackResourcesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackResourcesResponseDeserializer::deserialize(
                        "DescribeStackResourcesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackSetRequest {
    type Output = DescribeStackSetResponse;
    type Error = DescribeStackSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackSet");
        params.put("Version", "2010-05-15");
        DescribeStackSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeStackSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackSetResponseDeserializer::deserialize(
                        "DescribeStackSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStackSetOperationRequest {
    type Output = DescribeStackSetOperationResponse;
    type Error = DescribeStackSetOperationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStackSetOperation");
        params.put("Version", "2010-05-15");
        DescribeStackSetOperationRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStackSetOperationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStackSetOperationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStackSetOperationResponseDeserializer::deserialize(
                        "DescribeStackSetOperationResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeStacksRequest {
    type Output = DescribeStacksResponse;
    type Error = DescribeStacksError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeStacks");
        params.put("Version", "2010-05-15");
        DescribeStacksRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeStacksError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeStacksResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeStacksResponseDeserializer::deserialize(
                        "DescribeStacksResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DetectStackDriftRequest {
    type Output = DetectStackDriftResponse;
    type Error = DetectStackDriftError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DetectStackDrift");
        params.put("Version", "2010-05-15");
        DetectStackDriftRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectStackDriftError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetectStackDriftResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DetectStackDriftResponseDeserializer::deserialize(
                        "DetectStackDriftResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DetectStackResourceDriftRequest {
    type Output = DetectStackResourceDriftResponse;
    type Error = DetectStackResourceDriftError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "DetectStackResourceDrift");
        params.put("Version", "2010-05-15");
        DetectStackResourceDriftRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DetectStackResourceDriftError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DetectStackResourceDriftResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DetectStackResourceDriftResponseDeserializer::deserialize(
                        "DetectStackResourceDriftResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for EstimateTemplateCostRequest {
    type Output = EstimateTemplateCostResponse;
    type Error = EstimateTemplateCostError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "EstimateTemplateCost");
        params.put("Version", "2010-05-15");
        EstimateTemplateCostRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(EstimateTemplateCostError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EstimateTemplateCostResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = EstimateTemplateCostResponseDeserializer::deserialize(
                        "EstimateTemplateCostResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ExecuteChangeSetRequest {
    type Output = ExecuteChangeSetResponse;
    type Error = ExecuteChangeSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ExecuteChangeSet");
        params.put("Version", "2010-05-15");
        ExecuteChangeSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ExecuteChangeSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ExecuteChangeSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ExecuteChangeSetResponseDeserializer::deserialize(
                        "ExecuteChangeSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetStackPolicyRequest {
    type Output = GetStackPolicyResponse;
    type Error = GetStackPolicyError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetStackPolicy");
        params.put("Version", "2010-05-15");
        GetStackPolicyRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStackPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetStackPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetStackPolicyResponseDeserializer::deserialize(
                        "GetStackPolicyResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetTemplateRequest {
    type Output = GetTemplateResponse;
    type Error = GetTemplateError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplate");
        params.put("Version", "2010-05-15");
        GetTemplateRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTemplateError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetTemplateResponseDeserializer::deserialize(
                        "GetTemplateResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetTemplateSummaryRequest {
    type Output = GetTemplateSummaryResponse;
    type Error = GetTemplateSummaryError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetTemplateSummary");
        params.put("Version", "2010-05-15");
        GetTemplateSummaryRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTemplateSummaryError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetTemplateSummaryResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetTemplateSummaryResponseDeserializer::deserialize(
                        "GetTemplateSummaryResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListChangeSetsRequest {
    type Output = ListChangeSetsResponse;
    type Error = ListChangeSetsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListChangeSets");
        params.put("Version", "2010-05-15");
        ListChangeSetsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListChangeSetsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListChangeSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListChangeSetsResponseDeserializer::deserialize(
                        "ListChangeSetsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListExportsRequest {
    type Output = ListExportsResponse;
    type Error = ListExportsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListExports");
        params.put("Version", "2010-05-15");
        ListExportsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListExportsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListExportsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListExportsResponseDeserializer::deserialize(
                        "ListExportsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListImportsRequest {
    type Output = ListImportsResponse;
    type Error = ListImportsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListImports");
        params.put("Version", "2010-05-15");
        ListImportsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListImportsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListImportsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListImportsResponseDeserializer::deserialize(
                        "ListImportsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStackInstancesRequest {
    type Output = ListStackInstancesResponse;
    type Error = ListStackInstancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackInstances");
        params.put("Version", "2010-05-15");
        ListStackInstancesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListStackInstancesError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackInstancesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStackInstancesResponseDeserializer::deserialize(
                        "ListStackInstancesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStackResourcesRequest {
    type Output = ListStackResourcesResponse;
    type Error = ListStackResourcesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackResources");
        params.put("Version", "2010-05-15");
        ListStackResourcesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListStackResourcesError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackResourcesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStackResourcesResponseDeserializer::deserialize(
                        "ListStackResourcesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStackSetOperationResultsRequest {
    type Output = ListStackSetOperationResultsResponse;
    type Error = ListStackSetOperationResultsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSetOperationResults");
        params.put("Version", "2010-05-15");
        ListStackSetOperationResultsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListStackSetOperationResultsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetOperationResultsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStackSetOperationResultsResponseDeserializer::deserialize(
                        "ListStackSetOperationResultsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStackSetOperationsRequest {
    type Output = ListStackSetOperationsResponse;
    type Error = ListStackSetOperationsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSetOperations");
        params.put("Version", "2010-05-15");
        ListStackSetOperationsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListStackSetOperationsError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetOperationsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStackSetOperationsResponseDeserializer::deserialize(
                        "ListStackSetOperationsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStackSetsRequest {
    type Output = ListStackSetsResponse;
    type Error = ListStackSetsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStackSets");
        params.put("Version", "2010-05-15");
        ListStackSetsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListStackSetsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStackSetsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStackSetsResponseDeserializer::deserialize(
                        "ListStackSetsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListStacksRequest {
    type Output = ListStacksResponse;
    type Error = ListStacksError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListStacks");
        params.put("Version", "2010-05-15");
        ListStacksRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListStacksError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListStacksResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListStacksResponseDeserializer::deserialize(
                        "ListStacksResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for SetStackPolicyRequest {
    type Output = SetStackPolicyResponse;
    type Error = SetStackPolicyError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "SetStackPolicy");
        params.put("Version", "2010-05-15");
        SetStackPolicyRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetStackPolicyError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetStackPolicyResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = SetStackPolicyResponseDeserializer::deserialize(
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

impl ServiceRequest for SignalResourceRequest {
    type Output = SignalResourceResponse;
    type Error = SignalResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "SignalResource");
        params.put("Version", "2010-05-15");
        SignalResourceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SignalResourceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SignalResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = SignalResourceResponseDeserializer::deserialize(
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

impl ServiceRequest for StopStackSetOperationRequest {
    type Output = StopStackSetOperationResponse;
    type Error = StopStackSetOperationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "StopStackSetOperation");
        params.put("Version", "2010-05-15");
        StopStackSetOperationRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopStackSetOperationError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = StopStackSetOperationResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = StopStackSetOperationResponseDeserializer::deserialize(
                        "StopStackSetOperationResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UpdateStackRequest {
    type Output = UpdateStackResponse;
    type Error = UpdateStackError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStack");
        params.put("Version", "2010-05-15");
        UpdateStackRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateStackError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = UpdateStackResponseDeserializer::deserialize(
                        "UpdateStackResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UpdateStackInstancesRequest {
    type Output = UpdateStackInstancesResponse;
    type Error = UpdateStackInstancesError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStackInstances");
        params.put("Version", "2010-05-15");
        UpdateStackInstancesRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateStackInstancesError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackInstancesResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = UpdateStackInstancesResponseDeserializer::deserialize(
                        "UpdateStackInstancesResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UpdateStackSetRequest {
    type Output = UpdateStackSetResponse;
    type Error = UpdateStackSetError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateStackSet");
        params.put("Version", "2010-05-15");
        UpdateStackSetRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateStackSetError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateStackSetResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = UpdateStackSetResponseDeserializer::deserialize(
                        "UpdateStackSetResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UpdateTerminationProtectionRequest {
    type Output = UpdateTerminationProtectionResponse;
    type Error = UpdateTerminationProtectionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "UpdateTerminationProtection");
        params.put("Version", "2010-05-15");
        UpdateTerminationProtectionRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTerminationProtectionError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UpdateTerminationProtectionResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = UpdateTerminationProtectionResponseDeserializer::deserialize(
                        "UpdateTerminationProtectionResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ValidateTemplateRequest {
    type Output = ValidateTemplateResponse;
    type Error = ValidateTemplateError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "cloudformation", region, "/");
        let mut params = Params::new();

        params.put("Action", "ValidateTemplate");
        params.put("Version", "2010-05-15");
        ValidateTemplateRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ValidateTemplateError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ValidateTemplateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ValidateTemplateResponseDeserializer::deserialize(
                        "ValidateTemplateResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
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
    fn test_parse_error_cloudformation_cancel_update_stack() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "cloudformation-cancel-update-stack.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            CloudFormationClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = CancelUpdateStackRequest::default();
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
            CloudFormationClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeStacksRequest::default();
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
            CloudFormationClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = GetTemplateRequest::default();
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
            CloudFormationClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListStacksRequest::default();
        let result = client.list_stacks(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
