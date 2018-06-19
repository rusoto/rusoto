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
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>An activation registers one or more on-premises servers or virtual machines (VMs) with AWS so that you can configure those servers or VMs using Run Command. A server or VM that has been registered with AWS is called a managed instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Activation {
    /// <p>The ID created by Systems Manager when you submitted the activation.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    /// <p>The date the activation was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A name for the managed instance when it is created.</p>
    #[serde(rename = "DefaultInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    /// <p>A user defined description of the activation.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date when this activation can no longer be used to register managed instances.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>Whether or not the activation is expired.</p>
    #[serde(rename = "Expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// <p>The Amazon Identity and Access Management (IAM) role to assign to the managed instance.</p>
    #[serde(rename = "IamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The maximum number of managed instances that can be registered using this activation.</p>
    #[serde(rename = "RegistrationLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i64>,
    /// <p>The number of managed instances already registered with this activation.</p>
    #[serde(rename = "RegistrationsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrations_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceRequest {
    /// <p><p>The resource ID you want to tag.</p> <p>Use the ID of the resource. Here are some examples:</p> <p>ManagedInstance: mi-012345abcde</p> <p>MaintenanceWindow: mw-012345abcde</p> <p>PatchBaseline: pb-012345abcde</p> <p>For the Document and Parameter values, use the name of the resource.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. You must specify the the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>Specifies the type of resource you are tagging.</p> <note> <p>The ManagedInstance type for this API action is for on-premises managed instances. You must specify the the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p><p> One or more tags. The value parameter is required, but if you don&#39;t want the tag to have a value, specify the parameter with no value, and we set the value to an empty string. </p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddTagsToResourceResult {}

/// <p>Describes an association of a Systems Manager document and an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Association {
    /// <p>The ID created by the system when you create an association. An association is a binding between a document and a set of targets with a schedule.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association name.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The version of the document used in the association.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The date on which the association was last run.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the association.</p>
    #[serde(rename = "Overview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The instances targeted by the request to create an association. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Describes the parameters for a document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociationDescription {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The association name.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The date when the association was made.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The date on which the association was last run.</p>
    #[serde(rename = "LastExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    /// <p>The last date on which the association was successfully run.</p>
    #[serde(rename = "LastSuccessfulExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_execution_date: Option<f64>,
    /// <p>The date when the association was last updated.</p>
    #[serde(rename = "LastUpdateAssociationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_association_date: Option<f64>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An Amazon S3 bucket where you want to store the output details of the request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>Information about the association.</p>
    #[serde(rename = "Overview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    /// <p>A description of the parameters for a document. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The association status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssociationStatus>,
    /// <p>The instances targeted by the request. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociationFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The filter value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Information about the association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociationOverview {
    /// <p>Returns the number of targets for the association status. For example, if you created an association with two instances, and one of them was successful, this would return the count of instances by status.</p>
    #[serde(rename = "AssociationStatusAggregatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_aggregated_count: Option<::std::collections::HashMap<String, i64>>,
    /// <p>A detailed status of the association.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The status of the association. Status can be: Pending, Success, or Failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes an association status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociationStatus {
    /// <p>A user-defined string.</p>
    #[serde(rename = "AdditionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    /// <p>The date when the status changed.</p>
    #[serde(rename = "Date")]
    pub date: f64,
    /// <p>The reason for the status.</p>
    #[serde(rename = "Message")]
    pub message: String,
    /// <p>The status.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Information about the association version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociationVersionInfo {
    /// <p>The ID created by the system when the association was created.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The name specified for the association version when the association version was created.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The association version.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The date the association version was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The version of a Systems Manager document used when the association version was created.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name specified when the association was created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The location in Amazon S3 specified for the association when the association version was created.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>Parameters specified when the association version was created.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The cron or rate schedule specified for the association when the association version was created.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets specified for the association when the association version was created. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Detailed information about the current state of an individual Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AutomationExecution {
    /// <p>The execution ID.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    /// <p>The execution status of the Automation.</p>
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    /// <p>The action of the currently executing step.</p>
    #[serde(rename = "CurrentAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    /// <p>The name of the currently executing step.</p>
    #[serde(rename = "CurrentStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    /// <p>The name of the Automation document used during the execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The version of the document to use during execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the user who executed the automation.</p>
    #[serde(rename = "ExecutedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    /// <p>The time the execution finished.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>The time the execution started.</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>A message describing why an execution has failed, if the status is set to Failed.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>The MaxConcurrency value specified by the user when the execution started.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The MaxErrors value specified by the user when the execution started.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The automation execution mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The list of execution outputs as defined in the automation document.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The key-value map of execution parameters, which were supplied when calling StartAutomationExecution.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The AutomationExecutionId of the parent automation.</p>
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    /// <p>A list of resolved targets in the rate control execution.</p>
    #[serde(rename = "ResolvedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    /// <p>A list of details about the current state of all steps that comprise an execution. An Automation document contains a list of steps that are executed in order.</p>
    #[serde(rename = "StepExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
    /// <p>A boolean value that indicates if the response contains the full list of the Automation step executions. If true, use the DescribeAutomationStepExecutions API action to get the full list of step executions.</p>
    #[serde(rename = "StepExecutionsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions_truncated: Option<bool>,
    /// <p>The target of the execution.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The parameter name.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>The specified targets.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>A filter used to match specific automation executions. This is used to limit the scope of Automation execution information returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AutomationExecutionFilter {
    /// <p>One or more keys to limit the results. Valid filter keys include the following: DocumentNamePrefix, ExecutionStatus, ExecutionId, ParentExecutionId, CurrentAction, StartTimeBefore, StartTimeAfter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values used to limit the execution information associated with the filter's key.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Details about a specific Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AutomationExecutionMetadata {
    /// <p>The execution ID.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    /// <p>The status of the execution. Valid values include: Running, Succeeded, Failed, Timed out, or Cancelled.</p>
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    /// <p>The action of the currently executing step.</p>
    #[serde(rename = "CurrentAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    /// <p>The name of the currently executing step.</p>
    #[serde(rename = "CurrentStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    /// <p>The name of the Automation document used during execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The document version used during the execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The IAM role ARN of the user who executed the Automation.</p>
    #[serde(rename = "ExecutedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    /// <p>The time the execution finished. This is not populated if the execution is still in progress.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>The time the execution started.&gt;</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>An Amazon S3 bucket where execution information is stored.</p>
    #[serde(rename = "LogFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    /// <p>The MaxConcurrency value specified by the user when starting the Automation.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The MaxErrors value specified by the user when starting the Automation.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The Automation execution mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The ExecutionId of the parent Automation.</p>
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    /// <p>A list of targets that resolved during the execution.</p>
    #[serde(rename = "ResolvedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p>The list of execution outputs as defined in the Automation document.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>The targets defined by the user when starting the Automation.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelCommandRequest {
    /// <p>The ID of the command you want to cancel.</p>
    #[serde(rename = "CommandId")]
    pub command_id: String,
    /// <p>(Optional) A list of instance IDs on which you want to cancel the command. If not provided, the command is canceled on every instance on which it was requested.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

/// <p>Whether or not the command was successfully canceled. There is no guarantee that a request can be canceled.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelCommandResult {}

/// <p>Describes a command request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Command {
    /// <p>A unique identifier for this command.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The number of targets for which the command invocation reached a terminal state. Terminal states include the following: Success, Failed, Execution Timed Out, Delivery Timed Out, Canceled, Terminated, or Undeliverable.</p>
    #[serde(rename = "CompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_count: Option<i64>,
    /// <p>The name of the document requested for execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The number of targets for which the status is Failed or Execution Timed Out.</p>
    #[serde(rename = "ErrorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    /// <p>If this time is reached and the command has not already started executing, it will not run. Calculated based on the ExpiresAfter user input provided as part of the SendCommand API.</p>
    #[serde(rename = "ExpiresAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<f64>,
    /// <p>The instance IDs against which this command was requested.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>The maximum number of instances that are allowed to execute the command at the same time. You can specify a number of instances, such as 10, or a percentage of instances, such as 10%. The default value is 50. For more information about how to use MaxConcurrency, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html">Executing a Command Using Systems Manager Run Command</a>.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before the system stops sending the command to additional targets. You can specify a number of errors, such as 10, or a percentage or errors, such as 10%. The default value is 0. For more information about how to use MaxErrors, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html">Executing a Command Using Systems Manager Run Command</a>.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>Configurations for sending notifications about command status changes. </p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>The parameter values to be inserted in the document when executing the command.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The date and time the command was requested.</p>
    #[serde(rename = "RequestedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    /// <p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes. </p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The status of the command.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html">Run Command Status</a>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to any instances.</p> </li> <li> <p>In Progress: The command has been sent to at least one instance but has not reached a final state on all instances.</p> </li> <li> <p>Success: The command successfully executed on all invocations. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The value of MaxErrors or more command invocations shows a status of Delivery Timed Out. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: The value of MaxErrors or more command invocations shows a status of Execution Timed Out. This is a terminal state.</p> </li> <li> <p>Failed: The value of MaxErrors or more command invocations shows a status of Failed. This is a terminal state.</p> </li> <li> <p>Incomplete: The command was attempted on all instances and one or more invocations does not have a value of Success but not enough invocations failed for the status to be Failed. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Rate Exceeded: The number of instances targeted by the command exceeded the account limit for pending invocations. The system has canceled the command before executing it on any instance. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The number of targets for the command.</p>
    #[serde(rename = "TargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_count: Option<i64>,
    /// <p>An array of search criteria that targets instances using a Key,Value combination that you specify. Targets is required if you don't provide one or more instance IDs in the call.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

/// <p>Describes a command filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CommandFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The filter value. </p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user executes SendCommand against three instances, then a command invocation is created for each requested instance ID. A command invocation returns status and detail information about a command you executed. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CommandInvocation {
    /// <p>The command against which this invocation was requested.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "CommandPlugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_plugins: Option<Vec<CommandPlugin>>,
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The document name that was requested for execution.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>The instance ID in which this invocation was requested.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the invocation target. For Amazon EC2 instances this is the value for the aws:Name tag. For on-premises instances, this is the name of the instance.</p>
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>Configurations for sending notifications about command status changes on a per instance basis.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The time and date the request was sent to this instance.</p>
    #[serde(rename = "RequestedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    /// <p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes on a per instance basis.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The URL to the plugin's StdErr file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, StandardErrorUrl is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The URL to the plugin's StdOut file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, StandardOutputUrl is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>Whether or not the invocation succeeded, failed, or is pending.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution for each invocation (each instance targeted by the command). StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html">Run Command Status</a>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success: The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit and don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p> Gets the trace output sent by the agent. </p>
    #[serde(rename = "TraceOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_output: Option<String>,
}

/// <p>Describes plugin details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CommandPlugin {
    /// <p>The name of the plugin. Must be one of the following: aws:updateAgent, aws:domainjoin, aws:applications, aws:runPowerShellScript, aws:psmodule, aws:cloudWatch, aws:runShellScript, or aws:updateSSMAgent. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Output of the plugin execution.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// <p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript </p> <p>test_folder is the name of the Amazon S3 bucket;</p> <p> ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix is the name of the S3 prefix;</p> <p>i-1234567876543 is the instance ID;</p> <p>awsrunShellScript is the name of the plugin.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript </p> <p>test_folder is the name of the Amazon S3 bucket;</p> <p> ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix is the name of the S3 prefix;</p> <p>i-1234567876543 is the instance ID;</p> <p>awsrunShellScript is the name of the plugin.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>A numeric response code generated after executing the plugin. </p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// <p>The time the plugin stopped executing. Could stop prematurely if, for example, a cancel command was sent. </p>
    #[serde(rename = "ResponseFinishDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_finish_date_time: Option<f64>,
    /// <p>The time the plugin started executing. </p>
    #[serde(rename = "ResponseStartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_start_date_time: Option<f64>,
    /// <p>The URL for the complete text written by the plugin to stderr. If execution is not yet complete, then this string is empty.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stdout in Amazon S3. If the Amazon S3 bucket for the command was not specified, then this string is empty.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>The status of this plugin. You can execute a document with multiple plugins.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the plugin execution. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html">Run Command Status</a>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success: The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist, or it might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit, and they don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

/// <p>A summary of the call execution that includes an execution ID, the type of execution (for example, <code>Command</code>), and the date/time of the execution using a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceExecutionSummary {
    /// <p>An ID created by the system when <code>PutComplianceItems</code> was called. For example, <code>CommandID</code> is a valid execution ID. You can use this ID in subsequent calls.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The time the execution ran as a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
    #[serde(rename = "ExecutionTime")]
    pub execution_time: f64,
    /// <p>The type of execution. For example, <code>Command</code> is a valid execution type.</p>
    #[serde(rename = "ExecutionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
}

/// <p>Information about the compliance as defined by the resource type. For example, for a patch resource type, <code>Items</code> includes information about the PatchSeverity, Classification, etc.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceItem {
    /// <p>The compliance type. For example, Association (for a State Manager association), Patch, or Custom:<code>string</code> are all valid compliance types.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A "Key": "Value" tag combination for the compliance item.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// <p>A summary for the compliance item. The summary includes an execution ID, the execution type (for example, command), and the execution time.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    /// <p>An ID for the compliance item. For example, if the compliance item is a Windows patch, the ID could be the number of the KB article; for example: KB4010320.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An ID for the resource. For a managed instance, this is the instance ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of resource. <code>ManagedInstance</code> is currently the only supported resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The severity of the compliance status. Severity can be one of the following: Critical, High, Medium, Low, Informational, Unspecified.</p>
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// <p>The status of the compliance item. An item is either COMPLIANT or NON_COMPLIANT.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A title for the compliance item. For example, if the compliance item is a Windows patch, the title could be the title of the KB article for the patch; for example: Security Update for Active Directory Federation Services.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Information about a compliance item.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComplianceItemEntry {
    /// <p>A "Key": "Value" tag combination for the compliance item.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The compliance item ID. For example, if the compliance item is a Windows patch, the ID could be the number of the KB article.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The severity of the compliance status. Severity can be one of the following: Critical, High, Medium, Low, Informational, Unspecified.</p>
    #[serde(rename = "Severity")]
    pub severity: String,
    /// <p>The status of the compliance item. An item is either COMPLIANT or NON_COMPLIANT.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>The title of the compliance item. For example, if the compliance item is a Windows patch, the title could be the title of the KB article for the patch; for example: Security Update for Active Directory Federation Services. </p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ComplianceStringFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The type of comparison that should be performed for the value: Equal, NotEqual, BeginWith, LessThan, or GreaterThan.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value for which to search.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>A summary of compliance information by compliance type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ComplianceSummaryItem {
    /// <p>The type of compliance item. For example, the compliance type can be Association, Patch, or Custom:string.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A list of COMPLIANT items for the specified compliance type.</p>
    #[serde(rename = "CompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    /// <p>A list of NON_COMPLIANT items for the specified compliance type.</p>
    #[serde(rename = "NonCompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
}

/// <p>A summary of resources that are compliant. The summary is organized according to the resource count for each compliance type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CompliantSummary {
    /// <p>The total number of resources that are compliant.</p>
    #[serde(rename = "CompliantCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_count: Option<i64>,
    /// <p>A summary of the compliance severity by compliance type.</p>
    #[serde(rename = "SeveritySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateActivationRequest {
    /// <p><p>The name of the registered, managed instance as it will appear in the Amazon EC2 console or when you use the AWS command line tools to list EC2 resources.</p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "DefaultInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    /// <p><p>A user-defined description of the resource that you want to register with Amazon EC2. </p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date by which this activation request should expire. The default value is 24 hours.</p>
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The Amazon Identity and Access Management (IAM) role that you want to assign to the managed instance. </p>
    #[serde(rename = "IamRole")]
    pub iam_role: String,
    /// <p>Specify the maximum number of managed instances you want to register. The default value is 1 instance.</p>
    #[serde(rename = "RegistrationLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateActivationResult {
    /// <p>The code the system generates when it processes the activation. The activation code functions like a password to validate the activation ID. </p>
    #[serde(rename = "ActivationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code: Option<String>,
    /// <p>The ID number generated by the system when it processed the activation. The activation ID functions like a user name.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssociationBatchRequest {
    /// <p>One or more associations.</p>
    #[serde(rename = "Entries")]
    pub entries: Vec<CreateAssociationBatchRequestEntry>,
}

/// <p>Describes the association of a Systems Manager document and an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAssociationBatchRequestEntry {
    /// <p>Specify a descriptive name for the association.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The ID of the instance. </p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the configuration document. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>A description of the parameters for a document. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression that specifies a schedule when the association runs.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The instances targeted by the request.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAssociationBatchResult {
    /// <p>Information about the associations that failed.</p>
    #[serde(rename = "Failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<FailedCreateAssociation>>,
    /// <p>Information about the associations that succeeded.</p>
    #[serde(rename = "Successful")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<AssociationDescription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssociationRequest {
    /// <p>Specify a descriptive name for the association.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The document version you want to associate with the target(s). Can be a specific version or the default version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An Amazon S3 bucket where you want to store the output details of the request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>The parameters for the documents runtime configuration. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A cron expression when the association will be applied to the target(s).</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets (either instances or tags) for the association.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAssociationResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentRequest {
    /// <p>A valid JSON or YAML string.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>Specify the document format for the request. The document format can be either JSON or YAML. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The type of document to create. Valid document types include: Policy, Automation, and Command.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p><p>A name for the Systems Manager document.</p> <important> <p>Do not use the following to begin the names of documents you create. They are reserved by AWS for use as document prefixes:</p> <ul> <li> <p> <code>aws</code> </p> </li> <li> <p> <code>amazon</code> </p> </li> <li> <p> <code>amzn</code> </p> </li> </ul> </important></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specify a target type to define the kinds of resources the document can run on. For example, to run a document on EC2 instances, specify the following value: /AWS::EC2::Instance. If you specify a value of '/' the document can run on all types of resources. If you don't specify a value, the document can't run on any resources. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDocumentResult {
    /// <p>Information about the Systems Manager document.</p>
    #[serde(rename = "DocumentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateMaintenanceWindowRequest {
    /// <p>Enables a Maintenance Window task to execute on managed instances, even if you have not registered those instances as targets. If enabled, then you must specify the unregistered instances (by instance ID) when you register a task with the Maintenance Window </p> <p>If you don't enable this option, then you must specify previously-registered targets when you register a task with the Maintenance Window. </p>
    #[serde(rename = "AllowUnassociatedTargets")]
    pub allow_unassociated_targets: bool,
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    pub cutoff: i64,
    /// <p>An optional description for the Maintenance Window. We recommend specifying a description to help you organize your Maintenance Windows. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the Maintenance Window in hours.</p>
    #[serde(rename = "Duration")]
    pub duration: i64,
    /// <p>The name of the Maintenance Window.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    pub schedule: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateMaintenanceWindowResult {
    /// <p>The ID of the created Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePatchBaselineRequest {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Defines the compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. The default value is UNSPECIFIED.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Defines the operating system the patch baseline applies to. The Default value is WINDOWS.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePatchBaselineResult {
    /// <p>The ID of the created patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceDataSyncRequest {
    /// <p>Amazon S3 configuration details for the sync.</p>
    #[serde(rename = "S3Destination")]
    pub s3_destination: ResourceDataSyncS3Destination,
    /// <p>A name for the configuration.</p>
    #[serde(rename = "SyncName")]
    pub sync_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateResourceDataSyncResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteActivationRequest {
    /// <p>The ID of the activation that you want to delete.</p>
    #[serde(rename = "ActivationId")]
    pub activation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteActivationResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssociationRequest {
    /// <p>The association ID that you want to delete.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteAssociationResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentRequest {
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDocumentResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMaintenanceWindowRequest {
    /// <p>The ID of the Maintenance Window to delete.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteMaintenanceWindowResult {
    /// <p>The ID of the deleted Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParameterRequest {
    /// <p>The name of the parameter to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteParameterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteParametersRequest {
    /// <p>The names of the parameters to delete.</p>
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteParametersResult {
    /// <p>The names of the deleted parameters.</p>
    #[serde(rename = "DeletedParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_parameters: Option<Vec<String>>,
    /// <p>The names of parameters that weren't deleted because the parameters are not valid.</p>
    #[serde(rename = "InvalidParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePatchBaselineRequest {
    /// <p>The ID of the patch baseline to delete.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeletePatchBaselineResult {
    /// <p>The ID of the deleted patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceDataSyncRequest {
    /// <p>The name of the configuration to delete.</p>
    #[serde(rename = "SyncName")]
    pub sync_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteResourceDataSyncResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterManagedInstanceRequest {
    /// <p>The ID assigned to the managed instance when you registered it using the activation process. </p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterManagedInstanceResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterPatchBaselineForPatchGroupRequest {
    /// <p>The ID of the patch baseline to deregister the patch group from.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The name of the patch group that should be deregistered from the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline the patch group was deregistered from.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch group deregistered from the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterTargetFromMaintenanceWindowRequest {
    /// <p>The system checks if the target is being referenced by a task. If the target is being referenced, the system returns an error and does not deregister the target from the Maintenance Window.</p>
    #[serde(rename = "Safe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
    /// <p>The ID of the Maintenance Window the target should be removed from.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The ID of the target definition to remove.</p>
    #[serde(rename = "WindowTargetId")]
    pub window_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterTargetFromMaintenanceWindowResult {
    /// <p>The ID of the Maintenance Window the target was removed from.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the removed target definition.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterTaskFromMaintenanceWindowRequest {
    /// <p>The ID of the Maintenance Window the task should be removed from.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The ID of the task to remove from the Maintenance Window.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeregisterTaskFromMaintenanceWindowResult {
    /// <p>The ID of the Maintenance Window the task was removed from.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the task removed from the Maintenance Window.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

/// <p>Filter for the DescribeActivation API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeActivationsFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "FilterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_key: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "FilterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeActivationsRequest {
    /// <p>A filter to view information about your activations.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeActivationsFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeActivationsResult {
    /// <p>A list of activations for your AWS account.</p>
    #[serde(rename = "ActivationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_list: Option<Vec<Activation>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssociationRequest {
    /// <p>The association ID for which you want information.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>Specify the association version to retrieve. To view the latest version, either specify <code>$LATEST</code> for this parameter, or omit this parameter. To view a list of all associations for an instance, use ListInstanceAssociations. To get a list of versions for a specific association, use ListAssociationVersions. </p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAssociationResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAutomationExecutionsRequest {
    /// <p>Filters used to limit the scope of executions that are requested.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AutomationExecutionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAutomationExecutionsResult {
    /// <p>The list of details about each automation execution which has occurred which matches the filter specification, if any.</p>
    #[serde(rename = "AutomationExecutionMetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_metadata_list: Option<Vec<AutomationExecutionMetadata>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAutomationStepExecutionsRequest {
    /// <p>The Automation execution ID for which you want step execution descriptions.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>One or more filters to limit the number of step executions returned by the request.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StepExecutionFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A boolean that indicates whether to list step executions in reverse order by start time. The default value is false.</p>
    #[serde(rename = "ReverseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAutomationStepExecutionsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of details about the current state of all steps that make up an execution.</p>
    #[serde(rename = "StepExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAvailablePatchesRequest {
    /// <p>Filters used to scope down the returned patches.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAvailablePatchesResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of patches. Each entry in the array is a patch structure.</p>
    #[serde(rename = "Patches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<Patch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentPermissionRequest {
    /// <p>The name of the document for which you are the owner.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The permission type for the document. The permission type can be <i>Share</i>.</p>
    #[serde(rename = "PermissionType")]
    pub permission_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDocumentPermissionResponse {
    /// <p>The account IDs that have permission to use this document. The ID can be either an AWS account or <i>All</i>.</p>
    #[serde(rename = "AccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentRequest {
    /// <p>The document version for which you want information. Can be a specific version or the default version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDocumentResult {
    /// <p>Information about the Systems Manager document.</p>
    #[serde(rename = "Document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEffectiveInstanceAssociationsRequest {
    /// <p>The instance ID for which you want to view all associations.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEffectiveInstanceAssociationsResult {
    /// <p>The associations for the requested instance.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<InstanceAssociation>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEffectivePatchesForPatchBaselineRequest {
    /// <p>The ID of the patch baseline to retrieve the effective patches for.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEffectivePatchesForPatchBaselineResult {
    /// <p>An array of patches and patch status.</p>
    #[serde(rename = "EffectivePatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_patches: Option<Vec<EffectivePatch>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstanceAssociationsStatusRequest {
    /// <p>The instance IDs for which you want association status information.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeInstanceAssociationsStatusResult {
    /// <p>Status information about the association.</p>
    #[serde(rename = "InstanceAssociationStatusInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_infos: Option<Vec<InstanceAssociationStatusInfo>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstanceInformationRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of instances.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstanceInformationStringFilter>>,
    /// <p>One or more filters. Use a filter to return a more specific list of instances.</p>
    #[serde(rename = "InstanceInformationFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_filter_list: Option<Vec<InstanceInformationFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeInstanceInformationResult {
    /// <p>The instance information list.</p>
    #[serde(rename = "InstanceInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_list: Option<Vec<InstanceInformation>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchStatesForPatchGroupRequest {
    /// <p>Each entry in the array is a structure containing:</p> <p>Key (string between 1 and 200 characters)</p> <p> Values (array containing a single string)</p> <p> Type (string "Equal", "NotEqual", "LessThan", "GreaterThan")</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstancePatchStateFilter>>,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the patch group for which the patch state information should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeInstancePatchStatesForPatchGroupResult {
    /// <p>The high-level patch state for the requested instances. </p>
    #[serde(rename = "InstancePatchStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchStatesRequest {
    /// <p>The ID of the instance whose patch state information should be retrieved.</p>
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
    /// <p>The maximum number of instances to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeInstancePatchStatesResult {
    /// <p>The high-level patch state for the requested instances.</p>
    #[serde(rename = "InstancePatchStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInstancePatchesRequest {
    /// <p>Each entry in the array is a structure containing:</p> <p>Key (string, between 1 and 128 characters)</p> <p>Values (array of strings, each string between 1 and 256 characters)</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The ID of the instance whose patch state information should be retrieved.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of patches to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeInstancePatchesResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Each entry in the array is a structure containing:</p> <p>Title (string)</p> <p>KBId (string)</p> <p>Classification (string)</p> <p>Severity (string)</p> <p>State (string: "INSTALLED", "INSTALLED OTHER", "MISSING", "NOT APPLICABLE", "FAILED")</p> <p>InstalledTime (DateTime)</p> <p>InstalledBy (string)</p>
    #[serde(rename = "Patches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<PatchComplianceData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsRequest {
    /// <p>Optional filters used to scope down the returned task invocations. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the specific task in the Maintenance Window task that should be retrieved.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the Maintenance Window execution the task is part of.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the task invocation results per invocation.</p>
    #[serde(rename = "WindowExecutionTaskInvocationIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_invocation_identities:
        Option<Vec<MaintenanceWindowExecutionTaskInvocationIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTasksRequest {
    /// <p>Optional filters used to scope down the returned tasks. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Maintenance Window execution whose task executions should be retrieved.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowExecutionTasksResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the task executions.</p>
    #[serde(rename = "WindowExecutionTaskIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_identities: Option<Vec<MaintenanceWindowExecutionTaskIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowExecutionsRequest {
    /// <p>Each entry in the array is a structure containing:</p> <p>Key (string, between 1 and 128 characters)</p> <p>Values (array of strings, each string is between 1 and 256 characters)</p> <p>The supported Keys are ExecutedBefore and ExecutedAfter with the value being a date/time string such as 2016-11-04T05:00:00Z.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Maintenance Window whose executions should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowExecutionsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the Maintenance Windows execution.</p>
    #[serde(rename = "WindowExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_executions: Option<Vec<MaintenanceWindowExecution>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowTargetsRequest {
    /// <p>Optional filters that can be used to narrow down the scope of the returned window targets. The supported filter keys are Type, WindowTargetId and OwnerInformation.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Maintenance Window whose targets should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowTargetsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the targets in the Maintenance Window.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<MaintenanceWindowTarget>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowTasksRequest {
    /// <p>Optional filters used to narrow down the scope of the returned tasks. The supported filter keys are WindowTaskId, TaskArn, Priority, and TaskType.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the Maintenance Window whose tasks should be retrieved.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowTasksResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the tasks in the Maintenance Window.</p>
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<MaintenanceWindowTask>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeMaintenanceWindowsRequest {
    /// <p>Optional filters used to narrow down the scope of the returned Maintenance Windows. Supported filter keys are Name and Enabled.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeMaintenanceWindowsResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the Maintenance Windows.</p>
    #[serde(rename = "WindowIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_identities: Option<Vec<MaintenanceWindowIdentity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeParametersRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ParametersFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters to limit the request results.</p>
    #[serde(rename = "ParameterFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeParametersResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Parameters returned by the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchBaselinesRequest {
    /// <p>Each element in the array is a structure containing: </p> <p>Key: (string, "NAME_PREFIX" or "OWNER")</p> <p>Value: (array of strings, exactly 1 entry, between 1 and 255 characters)</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patch baselines to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePatchBaselinesResult {
    /// <p>An array of PatchBaselineIdentity elements.</p>
    #[serde(rename = "BaselineIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identities: Option<Vec<PatchBaselineIdentity>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchGroupStateRequest {
    /// <p>The name of the patch group whose patch snapshot should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePatchGroupStateResult {
    /// <p>The number of instances in the patch group.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<i64>,
    /// <p>The number of instances with patches from the patch baseline that failed to install.</p>
    #[serde(rename = "InstancesWithFailedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_failed_patches: Option<i64>,
    /// <p>The number of instances with patches installed that aren't defined in the patch baseline.</p>
    #[serde(rename = "InstancesWithInstalledOtherPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_other_patches: Option<i64>,
    /// <p>The number of instances with installed patches.</p>
    #[serde(rename = "InstancesWithInstalledPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_patches: Option<i64>,
    /// <p>The number of instances with missing patches from the patch baseline.</p>
    #[serde(rename = "InstancesWithMissingPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_missing_patches: Option<i64>,
    /// <p>The number of instances with patches that aren't applicable.</p>
    #[serde(rename = "InstancesWithNotApplicablePatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_not_applicable_patches: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePatchGroupsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
    /// <p>The maximum number of patch groups to return (per page).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePatchGroupsResult {
    /// <p>Each entry in the array contains:</p> <p>PatchGroup: string (between 1 and 256 characters, Regex: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$)</p> <p>PatchBaselineIdentity: A PatchBaselineIdentity element. </p>
    #[serde(rename = "Mappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<PatchGroupPatchBaselineMapping>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A default version of a document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentDefaultVersionDescription {
    /// <p>The default version of the document.</p>
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// <p>The name of the document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes a Systems Manager document. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentDescription {
    /// <p>The date when the document was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The default version.</p>
    #[serde(rename = "DefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    /// <p>A description of the document. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The type of document. </p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p><p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// <p><p>Sha256 or Sha1.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "HashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    /// <p>The latest version of the document.</p>
    #[serde(rename = "LatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS user account that created the document.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>A description of the parameters for a document.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<DocumentParameter>>,
    /// <p>The list of OS platforms compatible with this Systems Manager document. </p>
    #[serde(rename = "PlatformTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    /// <p>The schema version.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The SHA1 hash of the document, which you can use for verification.</p>
    #[serde(rename = "Sha1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_1: Option<String>,
    /// <p>The status of the Systems Manager document.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags, or metadata, that have been applied to the document.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The target type which defines the kinds of resources the document can run on. For example, /AWS::EC2::Instance. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p>Describes a filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the filter.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Describes the name of a Systems Manager document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentIdentifier {
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document type.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS user account that created the document.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The operating system platform. </p>
    #[serde(rename = "PlatformTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    /// <p>The schema version.</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The tags, or metadata, that have been applied to the document.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The target type which defines the kinds of resources the document can run on. For example, /AWS::EC2::Instance. For a list of valid resource types, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">AWS Resource Types Reference</a> in the <i>AWS CloudFormation User Guide</i>. </p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of documents.</p> <p>For keys, you can specify one or more tags that have been applied to a document. </p> <p>Other valid values include Owner, Name, PlatformTypes, and DocumentType.</p> <p>Note that only one Owner can be specified in a request. For example: <code>Key=Owner,Values=Self</code>.</p> <p>If you use Name as a key, you can use a name prefix to return a list of documents. For example, in the AWS CLI, to return a list of all documents that begin with <code>Te</code>, run the following command:</p> <p> <code>aws ssm list-documents --filters Key=Name,Values=Te</code> </p> <p>If you specify more than two keys, only documents that are identified by all the tags are returned in the results. If you specify more than two values for a key, documents that are identified by any of the values are returned in the results.</p> <p>To specify a custom key and value pair, use the format <code>Key=tag:[tagName],Values=[valueName]</code>.</p> <p>For example, if you created a Key called region and are using the AWS CLI to call the <code>list-documents</code> command: </p> <p> <code>aws ssm list-documents --filters Key=tag:region,Values=east,west Key=Owner,Values=Self</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentKeyValuesFilter {
    /// <p>The name of the filter key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the filter key.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Parameters specified in a System Manager document that execute on the server when the command is run. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentParameter {
    /// <p>If specified, the default values for the parameters. Parameters without a default value are required. Parameters with a default value are optional.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of parameter. The type can be either String or StringList.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Version information about the document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DocumentVersionInfo {
    /// <p>The date the document was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>An identifier for the default version of the document.</p>
    #[serde(rename = "IsDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The document name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The EffectivePatch structure defines metadata about a patch along with the approval state of the patch in a particular patch baseline. The approval state includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EffectivePatch {
    /// <p>Provides metadata for a patch, including information such as the KB ID, severity, classification and a URL for where more information can be obtained about the patch.</p>
    #[serde(rename = "Patch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Patch>,
    /// <p>The status of the patch in a patch baseline. This includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>
    #[serde(rename = "PatchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_status: Option<PatchStatus>,
}

/// <p>Describes a failed association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailedCreateAssociation {
    /// <p>The association.</p>
    #[serde(rename = "Entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<CreateAssociationBatchRequestEntry>,
    /// <p>The source of the failure.</p>
    #[serde(rename = "Fault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    /// <p>A description of the failure.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Information about an Automation failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailureDetails {
    /// <p>Detailed information about the Automation step failure.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The stage of the Automation execution when the failure occurred. The stages include the following: InputValidation, PreVerification, Invocation, PostVerification.</p>
    #[serde(rename = "FailureStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_stage: Option<String>,
    /// <p>The type of Automation failure. Failure types include the following: Action, Permission, Throttling, Verification, Internal.</p>
    #[serde(rename = "FailureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAutomationExecutionRequest {
    /// <p>The unique identifier for an existing automation execution to examine. The execution ID is returned by StartAutomationExecution when the execution of an Automation document is initiated.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAutomationExecutionResult {
    /// <p>Detailed information about the current state of an automation execution.</p>
    #[serde(rename = "AutomationExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution: Option<AutomationExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCommandInvocationRequest {
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    #[serde(rename = "CommandId")]
    pub command_id: String,
    /// <p>(Required) The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>(Optional) The name of the plugin for which you want detailed results. If the document contains only one plugin, the name can be omitted and the details will be returned.</p>
    #[serde(rename = "PluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetCommandInvocationResult {
    /// <p>The parent command ID of the invocation plugin.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>The comment text for the command.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the document that was executed. For example, AWS-RunShellScript.</p>
    #[serde(rename = "DocumentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    /// <p>Duration since ExecutionStartDateTime.</p>
    #[serde(rename = "ExecutionElapsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_elapsed_time: Option<String>,
    /// <p>The date and time the plugin was finished executing. Date and time are written in ISO 8601 format. For example, June 7, 2017 is represented as 2017-06-7. The following sample AWS CLI command uses the <code>InvokedAfter</code> filter.</p> <p> <code>aws ssm list-commands --filters key=InvokedAfter,value=2017-06-07T00:00:00Z</code> </p> <p>If the plugin has not started to execute, the string is empty.</p>
    #[serde(rename = "ExecutionEndDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_date_time: Option<String>,
    /// <p>The date and time the plugin started executing. Date and time are written in ISO 8601 format. For example, June 7, 2017 is represented as 2017-06-7. The following sample AWS CLI command uses the <code>InvokedBefore</code> filter.</p> <p> <code>aws ssm list-commands --filters key=InvokedBefore,value=2017-06-07T00:00:00Z</code> </p> <p>If the plugin has not started to execute, the string is empty.</p>
    #[serde(rename = "ExecutionStartDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_date_time: Option<String>,
    /// <p>The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the plugin for which you want detailed results. For example, aws:RunShellScript is a plugin.</p>
    #[serde(rename = "PluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    /// <p>The error level response code for the plugin script. If the response code is -1, then the command has not started executing on the instance, or it was not received by the instance.</p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// <p>The first 8,000 characters written by the plugin to stderr. If the command has not finished executing, then this string is empty.</p>
    #[serde(rename = "StandardErrorContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_content: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stderr. If the command has not finished executing, then this string is empty.</p>
    #[serde(rename = "StandardErrorUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    /// <p>The first 24,000 characters written by the plugin to stdout. If the command has not finished executing, if ExecutionStatus is neither Succeeded nor Failed, then this string is empty.</p>
    #[serde(rename = "StandardOutputContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_content: Option<String>,
    /// <p>The URL for the complete text written by the plugin to stdout in Amazon S3. If an Amazon S3 bucket was not specified, then this string is empty.</p>
    #[serde(rename = "StandardOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    /// <p>The status of this invocation plugin. This status can be different than StatusDetails.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>A detailed status of the command execution for an invocation. StatusDetails includes more information than Status because it includes states resulting from error and concurrency control parameters. StatusDetails can show different results than Status. For more information about these statuses, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html">Run Command Status</a>. StatusDetails can be one of the following values:</p> <ul> <li> <p>Pending: The command has not been sent to the instance.</p> </li> <li> <p>In Progress: The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Delayed: The system attempted to send the command to the target, but the target was not available. The instance might not be available because of network issues, the instance was stopped, etc. The system will try to deliver the command again.</p> </li> <li> <p>Success: The command or plugin was executed successfully. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out: The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command&#39;s MaxErrors limit, but they do contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Execution Timed Out: The command started to execute on the instance, but the execution was not complete before the timeout expired. Execution timeouts count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed: The command wasn&#39;t executed successfully on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the MaxErrors limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled: The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable: The command can&#39;t be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don&#39;t count against the parent command&#39;s MaxErrors limit and don&#39;t contribute to whether the parent command status is Success or Incomplete. This is a terminal state.</p> </li> <li> <p>Terminated: The parent command exceeded its MaxErrors limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul></p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDefaultPatchBaselineRequest {
    /// <p>Returns the default patch baseline for the specified operating system.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDefaultPatchBaselineResult {
    /// <p>The ID of the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The operating system for the returned patch baseline. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeployablePatchSnapshotForInstanceRequest {
    /// <p>The ID of the instance for which the appropriate patch snapshot should be retrieved.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The user-defined snapshot ID.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeployablePatchSnapshotForInstanceResult {
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>Returns the specific operating system (for example Windows Server 2012 or Amazon Linux 2015.09) on the instance for the specified patch snapshot.</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// <p>A pre-signed Amazon S3 URL that can be used to download the patch snapshot.</p>
    #[serde(rename = "SnapshotDownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_download_url: Option<String>,
    /// <p>The user-defined snapshot ID.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentRequest {
    /// <p>Returns the document in the specified format. The document format can be either JSON or YAML. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document version for which you want information.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDocumentResult {
    /// <p>The contents of the Systems Manager document.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The document format, either JSON or YAML.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The document type.</p>
    #[serde(rename = "DocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    /// <p>The document version.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInventoryRequest {
    /// <p>Returns counts of inventory types based on one or more expressions. For example, if you aggregate by using an expression that uses the <code>AWS:InstanceInformation.PlatformType</code> type, you can see a count of how many Windows and Linux instances exist in your inventoried fleet.</p>
    #[serde(rename = "Aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of inventory item types to return.</p>
    #[serde(rename = "ResultAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_attributes: Option<Vec<ResultAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInventoryResult {
    /// <p>Collection of inventory entities such as a collection of instance inventory. </p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<InventoryResultEntity>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInventorySchemaRequest {
    /// <p>Returns inventory schemas that support aggregation. For example, this call returns the <code>AWS:InstanceInformation</code> type, because it supports aggregation based on the <code>PlatformName</code>, <code>PlatformType</code>, and <code>PlatformVersion</code> attributes.</p>
    #[serde(rename = "Aggregator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator: Option<bool>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the sub-type schema for a specified inventory type.</p>
    #[serde(rename = "SubType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<bool>,
    /// <p>The type of inventory item to return.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInventorySchemaResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Inventory schemas returned by the request.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<InventoryItemSchema>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionRequest {
    /// <p>The ID of the Maintenance Window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMaintenanceWindowExecutionResult {
    /// <p>The time the Maintenance Window finished executing.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the Maintenance Window started executing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the Maintenance Window execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the task executions from the Maintenance Window execution.</p>
    #[serde(rename = "TaskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
    /// <p>The ID of the Maintenance Window execution.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskInvocationRequest {
    /// <p>The invocation ID to retrieve.</p>
    #[serde(rename = "InvocationId")]
    pub invocation_id: String,
    /// <p>The ID of the specific task in the Maintenance Window task that should be retrieved. </p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the Maintenance Window execution for which the task is a part.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMaintenanceWindowExecutionTaskInvocationResult {
    /// <p>The time that the task finished executing on the target.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The execution ID.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The invocation ID.</p>
    #[serde(rename = "InvocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    /// <p>User-provided value to be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window. </p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The parameters used at the time that the task executed.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    /// <p>The time that the task started executing on the target.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The task status for an invocation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status. Details are only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The task execution ID.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>Retrieves the task type for a Maintenance Window. Task types include the following: LAMBDA, STEP_FUNCTION, AUTOMATION, RUN_COMMAND.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The Maintenance Window execution ID.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The Maintenance Window target ID.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskRequest {
    /// <p>The ID of the specific task execution in the Maintenance Window task that should be retrieved.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// <p>The ID of the Maintenance Window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    pub window_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMaintenanceWindowExecutionTaskResult {
    /// <p>The time the task execution completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The defined maximum number of task executions that could be run in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The defined maximum number of task execution errors allowed before scheduling of the task execution would have been stopped.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The priority of the task.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The role that was assumed when executing the task.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The time the task execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ARN of the executed task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ID of the specific task execution in the Maintenance Window task that was retrieved.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The parameters passed to the task when it was executed.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note> <p>The map has the following format:</p> <p>Key: string, between 1 and 255 characters</p> <p>Value: an array of strings, each string is between 1 and 255 characters</p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters: Option<
        Vec<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    >,
    /// <p>The type of task executed.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the Maintenance Window execution that includes the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowRequest {
    /// <p>The ID of the desired Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMaintenanceWindowResult {
    /// <p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The date the Maintenance Window was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>The description of the Maintenance Window.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the Maintenance Window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the Maintenance Windows is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The date the Maintenance Window was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the Maintenance Window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The ID of the created Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMaintenanceWindowTaskRequest {
    /// <p>The Maintenance Window ID that includes the task to retrieve.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The Maintenance Window task ID to retrieve.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetMaintenanceWindowTaskResult {
    /// <p>The retrieved task description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The location in Amazon S3 where the task results are logged.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets allowed to run this task in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before the task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The retrieved task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task when it executes. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The IAM service role to assume during task execution.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets where the task should execute.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The resource that the task used during execution. For RUN_COMMAND and AUTOMATION task types, the TaskArn is the Systems Manager Document name/ARN. For LAMBDA tasks, the value is the function name/ARN. For STEP_FUNCTION tasks, the value is the state machine ARN.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The parameters to pass to the task when it executes.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The parameters to pass to the task when it executes.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task to execute.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The retrieved Maintenance Window ID.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The retrieved Maintenance Window task ID.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParameterHistoryRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of a parameter you want to query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetParameterHistoryResult {
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters returned by the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterHistory>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParameterRequest {
    /// <p>The name of the parameter you want to query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetParameterResult {
    /// <p>Information about a parameter.</p>
    #[serde(rename = "Parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Parameter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersByPathRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters to limit the request results.</p>
    #[serde(rename = "ParameterFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
    /// <p>The hierarchy for the parameter. Hierarchies start with a forward slash (/) and end with the parameter name. A hierarchy can have a maximum of 15 levels. Here is an example of a hierarchy: <code>/Finance/Prod/IAD/WinServ2016/license33</code> </p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>Retrieve all parameters within a hierarchy.</p>
    #[serde(rename = "Recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// <p>Retrieve all parameters in a hierarchy with their value decrypted.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetParametersByPathResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parameters found in the specified hierarchy.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersRequest {
    /// <p>Names of the parameters for which you want to query information.</p>
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    /// <p>Return decrypted secure string value. Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>
    #[serde(rename = "WithDecryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetParametersResult {
    /// <p>A list of parameters that are not formatted correctly or do not run when executed.</p>
    #[serde(rename = "InvalidParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
    /// <p>A list of details for a parameter.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPatchBaselineForPatchGroupRequest {
    /// <p>Returns he operating system rule specified for patch groups using the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The name of the patch group whose patch baseline should be retrieved.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline that should be used for the patch group.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The operating system rule specified for patch groups using the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The name of the patch group.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPatchBaselineRequest {
    /// <p>The ID of the patch baseline to retrieve.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetPatchBaselineResult {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Returns the specified compliance severity level for approved patches in the patch baseline.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the retrieved patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The date the patch baseline was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The date the patch baseline was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Returns the operating system specified for the patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>Patch groups included in the patch baseline.</p>
    #[serde(rename = "PatchGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<Vec<String>>,
    /// <p>A list of explicitly rejected patches for the baseline.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

/// <p>Status information about the aggregated associations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceAggregatedAssociationOverview {
    /// <p>Detailed status information about the aggregated associations.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The number of associations for the instance(s).</p>
    #[serde(rename = "InstanceAssociationStatusAggregatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_aggregated_count:
        Option<::std::collections::HashMap<String, i64>>,
}

/// <p>One or more association documents on the instance. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceAssociation {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>Version information for the association on the instance.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The content of the association document for the instance(s).</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceAssociationOutputLocation {
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "S3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3OutputLocation>,
}

/// <p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceAssociationOutputUrl {
    /// <p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "S3OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_url: Option<S3OutputUrl>,
}

/// <p>Status information about the instance association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceAssociationStatusInfo {
    /// <p>The association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The name of the association applied to the instance.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>The version of the association applied to the instance.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>Detailed status information about the instance association.</p>
    #[serde(rename = "DetailedStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    /// <p>The association document verions.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>An error code returned by the request to create the association.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The date the instance association executed. </p>
    #[serde(rename = "ExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<f64>,
    /// <p>Summary information about association execution.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<String>,
    /// <p>The instance ID where the association was created.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the association.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<InstanceAssociationOutputUrl>,
    /// <p>Status information about the instance association.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a filter for a specific list of instances. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceInformation {
    /// <p>The activation ID created by Systems Manager when the server or VM was registered.</p>
    #[serde(rename = "ActivationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    /// <p>The version of the SSM Agent running on your Linux instance. </p>
    #[serde(rename = "AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_overview: Option<InstanceAggregatedAssociationOverview>,
    /// <p>The status of the association.</p>
    #[serde(rename = "AssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    /// <p>The fully qualified host name of the managed instance.</p>
    #[serde(rename = "ComputerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// <p>The IP address of the managed instance.</p>
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The Amazon Identity and Access Management (IAM) role assigned to EC2 instances or managed instances. </p>
    #[serde(rename = "IamRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// <p>The instance ID. </p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>Indicates whether latest version of the SSM Agent is running on your instance. Some older versions of Windows Server use the EC2Config service to process SSM requests. For this reason, this field does not indicate whether or not the latest version is installed on Windows managed instances.</p>
    #[serde(rename = "IsLatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<bool>,
    /// <p>The date the association was last executed.</p>
    #[serde(rename = "LastAssociationExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_association_execution_date: Option<f64>,
    /// <p>The date and time when agent last pinged Systems Manager service. </p>
    #[serde(rename = "LastPingDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping_date_time: Option<f64>,
    /// <p>The last date the association was successfully run.</p>
    #[serde(rename = "LastSuccessfulAssociationExecutionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_association_execution_date: Option<f64>,
    /// <p>The name of the managed instance.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Connection status of the SSM Agent. </p>
    #[serde(rename = "PingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_status: Option<String>,
    /// <p>The name of the operating system platform running on your instance. </p>
    #[serde(rename = "PlatformName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    /// <p>The operating system platform type. </p>
    #[serde(rename = "PlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// <p>The version of the OS platform running on your instance. </p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The date the server or VM was registered with AWS as a managed instance.</p>
    #[serde(rename = "RegistrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<f64>,
    /// <p>The type of instance. Instances are either EC2 instances or managed instances. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Describes a filter for a specific list of instances. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceInformationFilter {
    /// <p>The name of the filter. </p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "valueSet")]
    pub value_set: Vec<String>,
}

/// <p>The filters to describe or get information about your managed instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceInformationStringFilter {
    /// <p>The filter key name to describe your instances. For example:</p> <p>"InstanceIds"|"AgentVersion"|"PingStatus"|"PlatformTypes"|"ActivationIds"|"IamRole"|"ResourceType"|"AssociationStatus"|"Tag Key"</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Defines the high-level patch compliance state for a managed instance, providing information about the number of installed, missing, not applicable, and failed patches along with metadata about the operation when this information was gathered for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstancePatchState {
    /// <p>The ID of the patch baseline used to patch the instance.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The number of patches from the patch baseline that were attempted to be installed during the last patching operation, but failed to install.</p>
    #[serde(rename = "FailedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    /// <p>The number of patches from the patch baseline that are installed on the instance.</p>
    #[serde(rename = "InstalledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_count: Option<i64>,
    /// <p>The number of patches not specified in the patch baseline that are installed on the instance.</p>
    #[serde(rename = "InstalledOtherCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_other_count: Option<i64>,
    /// <p>The ID of the managed instance the high-level patch compliance information was collected for.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The number of patches from the patch baseline that are applicable for the instance but aren't currently installed.</p>
    #[serde(rename = "MissingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_count: Option<i64>,
    /// <p>The number of patches from the patch baseline that aren't applicable for the instance and hence aren't installed on the instance.</p>
    #[serde(rename = "NotApplicableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i64>,
    /// <p>The type of patching operation that was performed: SCAN (assess patch compliance state) or INSTALL (install missing patches).</p>
    #[serde(rename = "Operation")]
    pub operation: String,
    /// <p>The time the most recent patching operation completed on the instance.</p>
    #[serde(rename = "OperationEndTime")]
    pub operation_end_time: f64,
    /// <p>The time the most recent patching operation was started on the instance.</p>
    #[serde(rename = "OperationStartTime")]
    pub operation_start_time: f64,
    /// <p>Placeholder information. This field will always be empty in the current release of the service.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The name of the patch group the managed instance belongs to.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
    /// <p>The ID of the patch baseline snapshot used during the patching operation when this compliance data was collected.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

/// <p>Defines a filter used in DescribeInstancePatchStatesForPatchGroup used to scope down the information returned by the API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstancePatchStateFilter {
    /// <p>The key for the filter. Supported values are FailedCount, InstalledCount, InstalledOtherCount, MissingCount and NotApplicableCount.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of comparison that should be performed for the value: Equal, NotEqual, LessThan or GreaterThan.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The value for the filter, must be an integer greater than or equal to 0.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies the inventory type and attribute for the aggregation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryAggregator {
    /// <p>Nested aggregators to further refine aggregation for an inventory type.</p>
    #[serde(rename = "Aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    /// <p>The inventory type and attribute name for aggregation.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

/// <p>One or more filters. Use a filter to return a more specific list of results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryFilter {
    /// <p>The name of the filter key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The type of filter. Valid values include the following: "Equal"|"NotEqual"|"BeginWith"|"LessThan"|"GreaterThan"</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Inventory filter values. Example: inventory filter where instance IDs are specified as values Key=AWS:InstanceInformation.InstanceId,Values= i-a12b3c4d5e6g, i-1a2b3c4d5e6,Type=Equal </p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Information collected from managed instances based on your inventory policy document</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryItem {
    /// <p>The time the inventory information was collected.</p>
    #[serde(rename = "CaptureTime")]
    pub capture_time: String,
    /// <p>The inventory data of the inventory type.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The PutInventory API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>
    #[serde(rename = "ContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// <p>A map of associated properties for a specified inventory type. For example, with this attribute, you can specify the <code>ExecutionId</code>, <code>ExecutionType</code>, <code>ComplianceType</code> properties of the <code>AWS:ComplianceItem</code> type.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The schema version for the inventory item.</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>The name of the inventory type. Default inventory item type names start with AWS. Custom inventory type names will start with Custom. Default inventory item types include the following: AWS:AWSComponent, AWS:Application, AWS:InstanceInformation, AWS:Network, and AWS:WindowsUpdate.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

/// <p>Attributes are the entries within the inventory item content. It contains name and value.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InventoryItemAttribute {
    /// <p>The data type of the inventory item attribute. </p>
    #[serde(rename = "DataType")]
    pub data_type: String,
    /// <p>Name of the inventory item attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The inventory item schema definition. Users can use this to compose inventory query filters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InventoryItemSchema {
    /// <p>The schema attributes for inventory. This contains data type and attribute name.</p>
    #[serde(rename = "Attributes")]
    pub attributes: Vec<InventoryItemAttribute>,
    /// <p>The alias name of the inventory type. The alias name is used for display purposes.</p>
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The name of the inventory type. Default inventory item type names start with AWS. Custom inventory type names will start with Custom. Default inventory item types include the following: AWS:AWSComponent, AWS:Application, AWS:InstanceInformation, AWS:Network, and AWS:WindowsUpdate.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
    /// <p>The schema version for the inventory item.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Inventory query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InventoryResultEntity {
    /// <p>The data section in the inventory result entity JSON.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, InventoryResultItem>>,
    /// <p>ID of the inventory result entity. For example, for managed instance inventory the result will be the managed instance ID. For EC2 instance inventory, the result will be the instance ID. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The inventory result item.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InventoryResultItem {
    /// <p>The time inventory item data was captured.</p>
    #[serde(rename = "CaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    /// <p>Contains all the inventory data of the item type. Results include attribute names and values. </p>
    #[serde(rename = "Content")]
    pub content: Vec<::std::collections::HashMap<String, String>>,
    /// <p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The PutInventory API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>
    #[serde(rename = "ContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    /// <p>The schema version for the inventory result item/</p>
    #[serde(rename = "SchemaVersion")]
    pub schema_version: String,
    /// <p>The name of the inventory result item type.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationVersionsRequest {
    /// <p>The association ID for which you want to view all versions.</p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssociationVersionsResult {
    /// <p>Information about all versions of the association for the specified association ID.</p>
    #[serde(rename = "AssociationVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_versions: Option<Vec<AssociationVersionInfo>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "AssociationFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_filter_list: Option<Vec<AssociationFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAssociationsResult {
    /// <p>The associations.</p>
    #[serde(rename = "Associations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCommandInvocationsRequest {
    /// <p>(Optional) The invocations for a specific command ID.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>(Optional) If set this returns the response of the command executions and any command output. By default this is set to False. </p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<bool>,
    /// <p>(Optional) One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
    /// <p>(Optional) The command execution details for a specific instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCommandInvocationsResult {
    /// <p>(Optional) A list of all invocations. </p>
    #[serde(rename = "CommandInvocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_invocations: Option<Vec<CommandInvocation>>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCommandsRequest {
    /// <p>(Optional) If provided, lists only the specified command.</p>
    #[serde(rename = "CommandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// <p>(Optional) One or more filters. Use a filter to return a more specific list of results. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
    /// <p>(Optional) Lists commands issued against this instance ID.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListCommandsResult {
    /// <p>(Optional) The list of commands requested by the user. </p>
    #[serde(rename = "Commands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
    /// <p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListComplianceItemsRequest {
    /// <p>One or more compliance filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID for the resources from which to get compliance information. Currently, you can only specify one resource ID.</p>
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p>The type of resource from which to get compliance information. Currently, the only supported resource type is <code>ManagedInstance</code>.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListComplianceItemsResult {
    /// <p>A list of compliance information for the specified resource ID. </p>
    #[serde(rename = "ComplianceItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_items: Option<Vec<ComplianceItem>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListComplianceSummariesRequest {
    /// <p>One or more compliance or inventory filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. Currently, you can specify null or 50. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListComplianceSummariesResult {
    /// <p>A list of compliant and non-compliant summary counts based on compliance types. For example, this call returns State Manager associations, patches, or custom compliance types according to the filter criteria that you specified.</p>
    #[serde(rename = "ComplianceSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_items: Option<Vec<ComplianceSummaryItem>>,
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDocumentVersionsRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the document about which you want version information.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDocumentVersionsResult {
    /// <p>The document versions.</p>
    #[serde(rename = "DocumentVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_versions: Option<Vec<DocumentVersionInfo>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDocumentsRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "DocumentFilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_filter_list: Option<Vec<DocumentFilter>>,
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DocumentKeyValuesFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDocumentsResult {
    /// <p>The names of the Systems Manager documents.</p>
    #[serde(rename = "DocumentIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_identifiers: Option<Vec<DocumentIdentifier>>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInventoryEntriesRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>The instance ID for which you want inventory information.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of inventory item for which you want information.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListInventoryEntriesResult {
    /// <p>The time that inventory information was collected for the instance(s).</p>
    #[serde(rename = "CaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    /// <p>A list of inventory items on the instance(s).</p>
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>The instance ID targeted by the request to query inventory information.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The inventory schema version used by the instance(s).</p>
    #[serde(rename = "SchemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// <p>The type of inventory item returned by the request.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceComplianceSummariesRequest {
    /// <p>One or more filters. Use a filter to return a more specific list of results.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListResourceComplianceSummariesResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A summary count for specified or targeted managed instances. Summary count includes information about compliant and non-compliant State Manager associations, patch status, or custom items according to the filter criteria that you specify. </p>
    #[serde(rename = "ResourceComplianceSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_compliance_summary_items: Option<Vec<ResourceComplianceSummaryItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceDataSyncRequest {
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListResourceDataSyncResult {
    /// <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of your current Resource Data Sync configurations and their statuses.</p>
    #[serde(rename = "ResourceDataSyncItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_data_sync_items: Option<Vec<ResourceDataSyncItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The resource ID for which you want to see a list of tags.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Returns a list of tags for a specific resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForResourceResult {
    /// <p>A list of tags.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p><p>Information about an Amazon S3 bucket to write instance-level logs to.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingInfo {
    /// <p>The name of an Amazon S3 bucket where execution logs are stored .</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// <p>(Optional) The Amazon S3 bucket subfolder. </p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>The region where the Amazon S3 bucket is located.</p>
    #[serde(rename = "S3Region")]
    pub s3_region: String,
}

/// <p>The parameters for an AUTOMATION task type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowAutomationParameters {
    /// <p>The version of an Automation document to use during task execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p><p>The parameters for the AUTOMATION task.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For AUTOMATION task types, Systems Manager ignores any values specified for these parameters.</p> </note></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Describes the information about an execution of a Maintenance Window. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowExecution {
    /// <p>The time the execution finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the Status. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the Maintenance Window execution.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The ID of the Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// <p>Information about a task execution performed as part of a Maintenance Window execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowExecutionTaskIdentity {
    /// <p>The time the task execution finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The time the task execution started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status of the task execution. Only available for certain status values.</p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ARN of the executed task.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ID of the specific task execution in the Maintenance Window execution.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The type of executed task.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The ID of the Maintenance Window execution that ran the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

/// <p>Describes the information about a task invocation for a particular target as part of a task execution performed as part of a Maintenance Window execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowExecutionTaskInvocationIdentity {
    /// <p>The time the invocation finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ID of the action performed in the service that actually handled the task invocation. If the task type is RUN_COMMAND, this value is the command ID.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The ID of the task invocation.</p>
    #[serde(rename = "InvocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    /// <p>User-provided value that was specified when the target was registered with the Maintenance Window. This was also included in any CloudWatch events raised during the task invocation.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The parameters that were provided for the invocation when it was executed.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    /// <p>The time the invocation started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the task invocation.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The details explaining the status of the task invocation. Only available for certain Status values. </p>
    #[serde(rename = "StatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    /// <p>The ID of the specific task execution in the Maintenance Window execution.</p>
    #[serde(rename = "TaskExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    /// <p>The task type.</p>
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// <p>The ID of the Maintenance Window execution that ran the task.</p>
    #[serde(rename = "WindowExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    /// <p>The ID of the target definition in this Maintenance Window the invocation was performed for.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

/// <p>Filter used in the request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MaintenanceWindowFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Information about the Maintenance Window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowIdentity {
    /// <p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>A description of the Maintenance Window.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the Maintenance Window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the Maintenance Window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the Maintenance Window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// <p><p>The parameters for a LAMBDA task type.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Lambda tasks, Systems Manager ignores any values specified for TaskParameters and LoggingInfo.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowLambdaParameters {
    /// <p>Pass client-specific information to the Lambda function that you are invoking. You can then process the client information in your Lambda function as you choose through the context variable.</p>
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    /// <p>JSON to provide to your Lambda function as input.</p>
    #[serde(rename = "Payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: Option<Vec<u8>>,
    /// <p>(Optional) Specify a Lambda function version or alias name. If you specify a function version, the action uses the qualified function ARN to invoke a specific Lambda function. If you specify an alias name, the action uses the alias ARN to invoke the Lambda function version to which the alias points.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p><p>The parameters for a RUN_COMMAND task type.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Run Command tasks, Systems Manager uses specified values for <code>TaskParameters</code> and <code>LoggingInfo</code> only if no values are specified for <code>TaskInvocationParameters</code>. </p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowRunCommandParameters {
    /// <p>Information about the command(s) to execute.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.</p>
    #[serde(rename = "DocumentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    /// <p>SHA-256 or SHA-1. SHA-1 hashes have been deprecated.</p>
    #[serde(rename = "DocumentHashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    /// <p>Configurations for sending notifications about command status changes on a per-instance basis.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The Amazon S3 bucket subfolder.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>The parameters for the RUN_COMMAND task execution.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The IAM service role to assume during task execution.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>If this time is reached and the command has not already started executing, it doesn not execute.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

/// <p><p>The parameters for a STEP_FUNCTION task.</p> <p>For information about specifying and updating task parameters, see <a>RegisterTaskWithMaintenanceWindow</a> and <a>UpdateMaintenanceWindowTask</a>.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> <p>For Step Functions tasks, Systems Manager ignores any values specified for <code>TaskParameters</code> and <code>LoggingInfo</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowStepFunctionsParameters {
    /// <p>The inputs for the STEP_FUNCTION task.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>The name of the STEP_FUNCTION task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The target registered with the Maintenance Window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowTarget {
    /// <p>A description of the target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The target name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The type of target.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The Maintenance Window ID where the target is registered.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The ID of the target.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

/// <p>Information about a task defined for a Maintenance Window.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MaintenanceWindowTask {
    /// <p>A description of the task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Information about an Amazon S3 bucket to write task-level logs to.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets this task can be run for in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task in the Maintenance Window. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The role that should be assumed when executing the task</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The resource that the task uses during execution. For RUN_COMMAND and AUTOMATION task types, <code>TaskArn</code> is the Systems Manager document name or ARN. For LAMBDA tasks, it's the function name or ARN. For STEP_FUNCTION tasks, it's the state machine ARN.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p><p>The parameters that should be passed to the task when it is executed.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task. The type can be one of the following: RUN_COMMAND, AUTOMATION, LAMBDA, or STEP_FUNCTION.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Maintenance Window ID where the task is registered.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The task ID.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

/// <p>The parameters for task execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskInvocationParameters {
    /// <p>The parameters for an AUTOMATION task type.</p>
    #[serde(rename = "Automation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<MaintenanceWindowAutomationParameters>,
    /// <p>The parameters for a LAMBDA task type.</p>
    #[serde(rename = "Lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<MaintenanceWindowLambdaParameters>,
    /// <p>The parameters for a RUN_COMMAND task type.</p>
    #[serde(rename = "RunCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command: Option<MaintenanceWindowRunCommandParameters>,
    /// <p>The parameters for a STEP_FUNCTION task type.</p>
    #[serde(rename = "StepFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_functions: Option<MaintenanceWindowStepFunctionsParameters>,
}

/// <p>Defines the values for a task parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindowTaskParameterValueExpression {
    /// <p>This field contains an array of 0 or more strings, each 1 to 255 characters in length.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyDocumentPermissionRequest {
    /// <p>The AWS user accounts that should have access to the document. The account IDs can either be a group of account IDs or <i>All</i>.</p>
    #[serde(rename = "AccountIdsToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_add: Option<Vec<String>>,
    /// <p>The AWS user accounts that should no longer have access to the document. The AWS user account can either be a group of account IDs or <i>All</i>. This action has a higher priority than <i>AccountIdsToAdd</i>. If you specify an account ID to add and the same ID to remove, the system removes access to the document.</p>
    #[serde(rename = "AccountIdsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_remove: Option<Vec<String>>,
    /// <p>The name of the document that you want to share.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The permission type for the document. The permission type can be <i>Share</i>.</p>
    #[serde(rename = "PermissionType")]
    pub permission_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModifyDocumentPermissionResponse {}

/// <p>A summary of resources that are not compliant. The summary is organized according to resource type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NonCompliantSummary {
    /// <p>The total number of compliance items that are not compliant.</p>
    #[serde(rename = "NonCompliantCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_count: Option<i64>,
    /// <p>A summary of the non-compliance severity by compliance type</p>
    #[serde(rename = "SeveritySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

/// <p>Configurations for sending notifications.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// <p>An Amazon Resource Name (ARN) for a Simple Notification Service (SNS) topic. Run Command pushes notifications about command status changes to this topic.</p>
    #[serde(rename = "NotificationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    /// <p>The different events for which you can receive notifications. These events include the following: All (events), InProgress, Success, TimedOut, Cancelled, Failed. To learn more about these events, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html">Setting Up Events and Notifications</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "NotificationEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_events: Option<Vec<String>>,
    /// <p>Command: Receive notification when the status of a command changes. Invocation: For commands sent to multiple instances, receive notification on a per-instance basis when the status of a command changes. </p>
    #[serde(rename = "NotificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
}

/// <p>An Amazon EC2 Systems Manager parameter in Parameter Store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Parameter {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of parameter. Valid values include the following: String, String list, Secure string.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Information about parameter usage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterHistory {
    /// <p>Parameter names can include the following letters and symbols.</p> <p>a-zA-Z0-9_.-</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>Information about the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the query key used for this parameter.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Date the parameter was last changed or updated.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>
    #[serde(rename = "LastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of parameter used.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Metada includes information like the ARN of the last user and the date/time the parameter was last used.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ParameterMetadata {
    /// <p>A parameter name can include only the following letters and symbols.</p> <p>a-zA-Z0-9_.-</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>Description of the parameter actions.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the query key used for this parameter.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Date the parameter was last changed or updated.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>
    #[serde(rename = "LastModifiedUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// <p>The parameter name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of parameter. Valid parameter types include the following: String, String list, Secure string.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The parameter version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>One or more filters. Use a filter to return a more specific list of results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParameterStringFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    #[serde(rename = "Option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    /// <p>The value you want to search for.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>This data type is deprecated. Instead, use <a>ParameterStringFilter</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ParametersFilter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The filter values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Represents metadata about a patch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Patch {
    /// <p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>The URL where more information can be obtained about the patch.</p>
    #[serde(rename = "ContentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    /// <p>The description of the patch.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the patch (this is different than the Microsoft Knowledge Base ID).</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Microsoft Knowledge Base ID of the patch.</p>
    #[serde(rename = "KbNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_number: Option<String>,
    /// <p>The language of the patch if it's language-specific.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>The ID of the MSRC bulletin the patch is related to.</p>
    #[serde(rename = "MsrcNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_number: Option<String>,
    /// <p>The severity of the patch (for example Critical, Important, Moderate).</p>
    #[serde(rename = "MsrcSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_severity: Option<String>,
    /// <p>The specific product the patch is applicable for (for example, WindowsServer2016).</p>
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// <p>The product family the patch is applicable for (for example, Windows).</p>
    #[serde(rename = "ProductFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    /// <p>The date the patch was released.</p>
    #[serde(rename = "ReleaseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    /// <p>The title of the patch.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The name of the vendor providing the patch.</p>
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

/// <p>Defines the basic information about a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PatchBaselineIdentity {
    /// <p>The description of the patch baseline.</p>
    #[serde(rename = "BaselineDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_description: Option<String>,
    /// <p>The ID of the patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "BaselineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name: Option<String>,
    /// <p>Whether this is the default baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p>
    #[serde(rename = "DefaultBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_baseline: Option<bool>,
    /// <p>Defines the operating system the patch baseline applies to. The Default value is WINDOWS. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

/// <p>Information about the state of a patch on a particular instance as it relates to the patch baseline used to patch the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PatchComplianceData {
    /// <p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The date/time the patch was installed on the instance. Note that not all operating systems provide this level of information.</p>
    #[serde(rename = "InstalledTime")]
    pub installed_time: f64,
    /// <p>The operating system-specific ID of the patch.</p>
    #[serde(rename = "KBId")]
    pub kb_id: String,
    /// <p>The severity of the patch (for example, Critical, Important, Moderate).</p>
    #[serde(rename = "Severity")]
    pub severity: String,
    /// <p>The state of the patch on the instance (INSTALLED, INSTALLED_OTHER, MISSING, NOT_APPLICABLE or FAILED).</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>The title of the patch.</p>
    #[serde(rename = "Title")]
    pub title: String,
}

/// <p><p>Defines a patch filter.</p> <p>A patch filter consists of key/value pairs, but not all keys are valid for all operating system types. For example, the key <code>PRODUCT</code> is valid for all supported operating system types. The key <code>MSRC<em>SEVERITY</code>, however, is valid only for Windows operating systems, and the key <code>SECTION</code> is valid only for Ubuntu operating systems.</p> <p>Refer to the following sections for information about which keys may be used with each major operating system, and which values are valid for each key.</p> <p> <b>Windows Operating Systems</b> </p> <p>The supported keys for Windows operating systems are <code>PRODUCT</code>, <code>CLASSIFICATION</code>, and <code>MSRC</em>SEVERITY</code>. See the following lists for valid values for each of these keys.</p> <p> <i>Supported key:</i> <code>PRODUCT</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Windows7</code> </p> </li> <li> <p> <code>Windows8</code> </p> </li> <li> <p> <code>Windows8.1</code> </p> </li> <li> <p> <code>Windows8Embedded</code> </p> </li> <li> <p> <code>Windows10</code> </p> </li> <li> <p> <code>Windows10LTSB</code> </p> </li> <li> <p> <code>WindowsServer2008</code> </p> </li> <li> <p> <code>WindowsServer2008R2</code> </p> </li> <li> <p> <code>WindowsServer2012</code> </p> </li> <li> <p> <code>WindowsServer2012R2</code> </p> </li> <li> <p> <code>WindowsServer2016</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>CLASSIFICATION</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>CriticalUpdates</code> </p> </li> <li> <p> <code>DefinitionUpdates</code> </p> </li> <li> <p> <code>Drivers</code> </p> </li> <li> <p> <code>FeaturePacks</code> </p> </li> <li> <p> <code>SecurityUpdates</code> </p> </li> <li> <p> <code>ServicePacks</code> </p> </li> <li> <p> <code>Tools</code> </p> </li> <li> <p> <code>UpdateRollups</code> </p> </li> <li> <p> <code>Updates</code> </p> </li> <li> <p> <code>Upgrades</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>MSRC_SEVERITY</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Critical</code> </p> </li> <li> <p> <code>Important</code> </p> </li> <li> <p> <code>Moderate</code> </p> </li> <li> <p> <code>Low</code> </p> </li> <li> <p> <code>Unspecified</code> </p> </li> </ul> <p> <b>Ubuntu Operating Systems</b> </p> <p>The supported keys for Ubuntu operating systems are <code>PRODUCT</code>, <code>PRIORITY</code>, and <code>SECTION</code>. See the following lists for valid values for each of these keys.</p> <p> <i>Supported key:</i> <code>PRODUCT</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Ubuntu14.04</code> </p> </li> <li> <p> <code>Ubuntu16.04</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>PRIORITY</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Required</code> </p> </li> <li> <p> <code>Important</code> </p> </li> <li> <p> <code>Standard</code> </p> </li> <li> <p> <code>Optional</code> </p> </li> <li> <p> <code>Extra</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>SECTION</code> </p> <p>Only the length of the key value is validated. Minimum length is 1. Maximum length is 64.</p> <p> <b>Amazon Linux Operating Systems</b> </p> <p>The supported keys for Amazon Linux operating systems are <code>PRODUCT</code>, <code>CLASSIFICATION</code>, and <code>SEVERITY</code>. See the following lists for valid values for each of these keys.</p> <p> <i>Supported key:</i> <code>PRODUCT</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>AmazonLinux2012.03</code> </p> </li> <li> <p> <code>AmazonLinux2012.09</code> </p> </li> <li> <p> <code>AmazonLinux2013.03</code> </p> </li> <li> <p> <code>AmazonLinux2013.09</code> </p> </li> <li> <p> <code>AmazonLinux2014.03</code> </p> </li> <li> <p> <code>AmazonLinux2014.09</code> </p> </li> <li> <p> <code>AmazonLinux2015.03</code> </p> </li> <li> <p> <code>AmazonLinux2015.09</code> </p> </li> <li> <p> <code>AmazonLinux2016.03</code> </p> </li> <li> <p> <code>AmazonLinux2016.09</code> </p> </li> <li> <p> <code>AmazonLinux2017.03</code> </p> </li> <li> <p> <code>AmazonLinux2017.09</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>CLASSIFICATION</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Security</code> </p> </li> <li> <p> <code>Bugfix</code> </p> </li> <li> <p> <code>Enhancement</code> </p> </li> <li> <p> <code>Recommended</code> </p> </li> <li> <p> <code>Newpackage</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>SEVERITY</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Critical</code> </p> </li> <li> <p> <code>Important</code> </p> </li> <li> <p> <code>Medium</code> </p> </li> <li> <p> <code>Low</code> </p> </li> </ul> <p> <b>RedHat Enterprise Linux (RHEL) Operating Systems</b> </p> <p>The supported keys for RedHat Enterprise Linux operating systems are <code>PRODUCT</code>, <code>CLASSIFICATION</code>, and <code>SEVERITY</code>. See the following lists for valid values for each of these keys.</p> <p> <i>Supported key:</i> <code>PRODUCT</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>RedhatEnterpriseLinux6.5</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux6.6</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux6.7</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux6.8</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux6.9</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux7.0</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux7.1</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux7.2</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux7.3</code> </p> </li> <li> <p> <code>RedhatEnterpriseLinux7.4</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>CLASSIFICATION</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Security</code> </p> </li> <li> <p> <code>Bugfix</code> </p> </li> <li> <p> <code>Enhancement</code> </p> </li> <li> <p> <code>Recommended</code> </p> </li> <li> <p> <code>Newpackage</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>SEVERITY</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Critical</code> </p> </li> <li> <p> <code>Important</code> </p> </li> <li> <p> <code>Medium</code> </p> </li> <li> <p> <code>Low</code> </p> </li> </ul> <p> <b>SUSE Linux Enterprise Server (SUSE) Operating Systems</b> </p> <p>The supported keys for SUSE operating systems are <code>PRODUCT</code>, <code>CLASSIFICATION</code>, and <code>SEVERITY</code>. See the following lists for valid values for each of these keys.</p> <p> <i>Supported key:</i> <code>PRODUCT</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Suse12.0</code> </p> </li> <li> <p> <code>Suse12.1</code> </p> </li> <li> <p> <code>Suse12.2</code> </p> </li> <li> <p> <code>Suse12.3</code> </p> </li> <li> <p> <code>Suse12.4</code> </p> </li> <li> <p> <code>Suse12.5</code> </p> </li> <li> <p> <code>Suse12.6</code> </p> </li> <li> <p> <code>Suse12.7</code> </p> </li> <li> <p> <code>Suse12.8</code> </p> </li> <li> <p> <code>Suse12.9</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>CLASSIFICATION</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Security</code> </p> </li> <li> <p> <code>Recommended</code> </p> </li> <li> <p> <code>Optional</code> </p> </li> <li> <p> <code>Feature</code> </p> </li> <li> <p> <code>Document</code> </p> </li> <li> <p> <code>Yast</code> </p> </li> </ul> <p> <i>Supported key:</i> <code>SEVERITY</code> </p> <p> <i>Supported values:</i> </p> <ul> <li> <p> <code>Critical</code> </p> </li> <li> <p> <code>Important</code> </p> </li> <li> <p> <code>Moderate</code> </p> </li> <li> <p> <code>Low</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchFilter {
    /// <p>The key for the filter.</p> <p>See <a>PatchFilter</a> for lists of valid keys for each operating system type.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value for the filter key.</p> <p>See <a>PatchFilter</a> for lists of valid values for each key based on operating system type.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>A set of patch filters, typically used for approval rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchFilterGroup {
    /// <p>The set of patch filters that make up the group.</p>
    #[serde(rename = "PatchFilters")]
    pub patch_filters: Vec<PatchFilter>,
}

/// <p>The mapping between a patch group and the patch baseline the patch group is registered with.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PatchGroupPatchBaselineMapping {
    /// <p>The patch baseline the patch group is registered with.</p>
    #[serde(rename = "BaselineIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identity: Option<PatchBaselineIdentity>,
    /// <p>The name of the patch group registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

/// <p>Defines a filter used in Patch Manager APIs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PatchOrchestratorFilter {
    /// <p>The key for the filter.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value for the filter.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Defines an approval rule for a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchRule {
    /// <p>The number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.</p>
    #[serde(rename = "ApproveAfterDays")]
    pub approve_after_days: i64,
    /// <p>A compliance severity level for all approved patches in a patch baseline. Valid compliance severity levels include the following: Unspecified, Critical, High, Medium, Low, and Informational.</p>
    #[serde(rename = "ComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    /// <p>For instances identified by the approval rule filters, enables a patch baseline to apply non-security updates available in the specified repository. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "EnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_security: Option<bool>,
    /// <p>The patch filter group that defines the criteria for the rule.</p>
    #[serde(rename = "PatchFilterGroup")]
    pub patch_filter_group: PatchFilterGroup,
}

/// <p>A set of rules defining the approval rules for a patch baseline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchRuleGroup {
    /// <p>The rules that make up the rule group.</p>
    #[serde(rename = "PatchRules")]
    pub patch_rules: Vec<PatchRule>,
}

/// <p>Information about the patches to use to update the instances, including target operating systems and source repository. Applies to Linux instances only.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchSource {
    /// <p>The value of the yum repo configuration. For example:</p> <p> <code>cachedir=/var/cache/yum/$basesearch</code> </p> <p> <code>$releasever</code> </p> <p> <code>keepcache=0</code> </p> <p> <code>debualevel=2</code> </p>
    #[serde(rename = "Configuration")]
    pub configuration: String,
    /// <p>The name specified to identify the patch source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The specific operating system versions a patch repository applies to, such as "Ubuntu16.04", "AmazonLinux2016.09", "RedhatEnterpriseLinux7.2" or "Suse12.7". For lists of supported product values, see <a>PatchFilter</a>.</p>
    #[serde(rename = "Products")]
    pub products: Vec<String>,
}

/// <p>Information about the approval status of a patch.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PatchStatus {
    /// <p>The date the patch was approved (or will be approved if the status is PENDING_APPROVAL).</p>
    #[serde(rename = "ApprovalDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<f64>,
    /// <p>The compliance severity level for a patch.</p>
    #[serde(rename = "ComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    /// <p>The approval status of a patch (APPROVED, PENDING_APPROVAL, EXPLICIT_APPROVED, EXPLICIT_REJECTED).</p>
    #[serde(rename = "DeploymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutComplianceItemsRequest {
    /// <p>Specify the compliance type. For example, specify Association (for a State Manager association), Patch, or Custom:<code>string</code>.</p>
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    /// <p>A summary of the call execution that includes an execution ID, the type of execution (for example, <code>Command</code>), and the date/time of the execution using a datetime object that is saved in the following format: yyyy-MM-dd'T'HH:mm:ss'Z'.</p>
    #[serde(rename = "ExecutionSummary")]
    pub execution_summary: ComplianceExecutionSummary,
    /// <p>MD5 or SHA-256 content hash. The content hash is used to determine if existing information should be overwritten or ignored. If the content hashes match, the request to put compliance information is ignored.</p>
    #[serde(rename = "ItemContentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_content_hash: Option<String>,
    /// <p>Information about the compliance as defined by the resource type. For example, for a patch compliance type, <code>Items</code> includes information about the PatchSeverity, Classification, etc.</p>
    #[serde(rename = "Items")]
    pub items: Vec<ComplianceItemEntry>,
    /// <p>Specify an ID for this resource. For a managed instance, this is the instance ID.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Specify the type of resource. <code>ManagedInstance</code> is currently the only supported resource type.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutComplianceItemsResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutInventoryRequest {
    /// <p>One or more instance IDs where you want to add or update inventory items.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The inventory items that you want to add or update on instances.</p>
    #[serde(rename = "Items")]
    pub items: Vec<InventoryItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutInventoryResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutParameterRequest {
    /// <p>A regular expression used to validate the parameter value. For example, for String types with values restricted to numbers, you can specify the following: AllowedPattern=^\d+$ </p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p><p>Information about the parameter that you want to add to the system.</p> <important> <p>Do not enter personally identifiable information in this field.</p> </important></p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The KMS Key ID that you want to use to encrypt a parameter when you choose the SecureString data type. If you don't specify a key ID, the system uses the default key associated with your AWS account.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p><p>The fully qualified name of the parameter that you want to add to the system. The fully qualified name includes the complete hierarchy of the parameter path and name. For example: <code>/Dev/DBServer/MySQL/db-string13</code> </p> <p>For information about parameter name requirements and restrictions, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-su-create.html#sysman-paramstore-su-create-about">About Creating Systems Manager Parameters</a> in the <i>AWS Systems Manager User Guide</i>.</p> <note> <p>The maximum length constraint listed below includes capacity for additional system attributes that are not part of the name. The maximum length for the fully qualified parameter name is 1011 characters. </p> </note></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Overwrite an existing parameter. If not specified, will default to "false".</p>
    #[serde(rename = "Overwrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// <p>The type of parameter that you want to add to the system.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The parameter value that you want to add to the system.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutParameterResult {
    /// <p>The new version number of a parameter. If you edit a parameter value, Parameter Store automatically creates a new version and assigns this new version a unique ID. You can reference a parameter version ID in API actions or in Systems Manager documents (SSM documents). By default, if you don't specify a specific version, the system returns the latest parameter value when a parameter is called.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterDefaultPatchBaselineRequest {
    /// <p>The ID of the patch baseline that should be the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterDefaultPatchBaselineResult {
    /// <p>The ID of the default patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterPatchBaselineForPatchGroupRequest {
    /// <p>The ID of the patch baseline to register the patch group with.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>The name of the patch group that should be registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    pub patch_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterPatchBaselineForPatchGroupResult {
    /// <p>The ID of the patch baseline the patch group was registered with.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The name of the patch group registered with the patch baseline.</p>
    #[serde(rename = "PatchGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterTargetWithMaintenanceWindowRequest {
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>An optional description for the target.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An optional name for the target.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The type of target being registered with the Maintenance Window.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The targets (either instances or tags). </p> <p>Specify instances using the following format:</p> <p> <code>Key=InstanceIds,Values=&lt;instance-id-1&gt;,&lt;instance-id-2&gt;</code> </p> <p>Specify tags using either of the following formats:</p> <p> <code>Key=tag:&lt;tag-key&gt;,Values=&lt;tag-value-1&gt;,&lt;tag-value-2&gt;</code> </p> <p> <code>Key=tag-key,Values=&lt;tag-key-1&gt;,&lt;tag-key-2&gt;</code> </p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    /// <p>The ID of the Maintenance Window the target should be registered with.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterTargetWithMaintenanceWindowResult {
    /// <p>The ID of the target definition in this Maintenance Window.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterTaskWithMaintenanceWindowRequest {
    /// <p>User-provided idempotency token.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>An optional description for the task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>A structure containing information about an Amazon S3 bucket to write instance-level logs to. </p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The maximum number of targets this task can be run for in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: String,
    /// <p>The maximum number of errors allowed before this task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    pub max_errors: String,
    /// <p>An optional name for the task.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The role that should be assumed when executing the task.</p>
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// <p>The targets (either instances or Maintenance Window targets).</p> <p>Specify instances using the following format: </p> <p> <code>Key=InstanceIds,Values=&lt;instance-id-1&gt;,&lt;instance-id-2&gt;</code> </p> <p>Specify Maintenance Window targets using the following format:</p> <p> <code>Key=&lt;WindowTargetIds&gt;,Values=&lt;window-target-id-1&gt;,&lt;window-target-id-2&gt;</code> </p>
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    /// <p>The ARN of the task to execute </p>
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty. </p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The parameters that should be passed to the task when it is executed.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The type of task being registered.</p>
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// <p>The ID of the Maintenance Window the task should be added to.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RegisterTaskWithMaintenanceWindowResult {
    /// <p>The id of the task in the Maintenance Window.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    /// <p><p>The resource ID for which you want to remove tags. Use the ID of the resource. Here are some examples:</p> <p>ManagedInstance: mi-012345abcde</p> <p>MaintenanceWindow: mw-012345abcde</p> <p>PatchBaseline: pb-012345abcde</p> <p>For the Document and Parameter values, use the name of the resource.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. You must specify the the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>The type of resource of which you want to remove a tag.</p> <note> <p>The ManagedInstance type for this API action is only for on-premises managed instances. You must specify the the name of the managed instance in the following format: mi-ID_number. For example, mi-1a2b3c4d5e6f.</p> </note></p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>Tag keys that you want to remove from the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoveTagsFromResourceResult {}

/// <p>Information about targets that resolved during the Automation execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResolvedTargets {
    /// <p>A list of parameter values sent to targets that resolved during the Automation execution.</p>
    #[serde(rename = "ParameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<String>>,
    /// <p>A boolean value indicating whether the resolved target list is truncated.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

/// <p>Compliance summary information for a specific resource. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceComplianceSummaryItem {
    /// <p>The compliance type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>A list of items that are compliant for the resource.</p>
    #[serde(rename = "CompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    /// <p>Information about the execution.</p>
    #[serde(rename = "ExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    /// <p>A list of items that aren't compliant for the resource.</p>
    #[serde(rename = "NonCompliantSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
    /// <p>The highest severity item found for the resource. The resource is compliant for this item.</p>
    #[serde(rename = "OverallSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_severity: Option<String>,
    /// <p>The resource ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The compliance status for the resource.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about a Resource Data Sync configuration, including its current status and last successful sync.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResourceDataSyncItem {
    /// <p>The status reported by the last sync.</p>
    #[serde(rename = "LastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The last time the sync operations returned a status of <code>SUCCESSFUL</code> (UTC).</p>
    #[serde(rename = "LastSuccessfulSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_time: Option<f64>,
    /// <p>The status message details reported by the last sync.</p>
    #[serde(rename = "LastSyncStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status_message: Option<String>,
    /// <p>The last time the configuration attempted to sync (UTC).</p>
    #[serde(rename = "LastSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_time: Option<f64>,
    /// <p>Configuration information for the target Amazon S3 bucket.</p>
    #[serde(rename = "S3Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<ResourceDataSyncS3Destination>,
    /// <p>The date and time the configuration was created (UTC).</p>
    #[serde(rename = "SyncCreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_created_time: Option<f64>,
    /// <p>The name of the Resource Data Sync.</p>
    #[serde(rename = "SyncName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
}

/// <p>Information about the target Amazon S3 bucket for the Resource Data Sync.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDataSyncS3Destination {
    /// <p>The ARN of an encryption key for a destination in Amazon S3. Must belong to the same region as the destination Amazon S3 bucket.</p>
    #[serde(rename = "AWSKMSKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awskms_key_arn: Option<String>,
    /// <p>The name of the Amazon S3 bucket where the aggregated data is stored.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    /// <p>An Amazon S3 prefix for the bucket.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The AWS Region with the Amazon S3 bucket targeted by the Resource Data Sync.</p>
    #[serde(rename = "Region")]
    pub region: String,
    /// <p>A supported sync format. The following format is currently supported: JsonSerDe</p>
    #[serde(rename = "SyncFormat")]
    pub sync_format: String,
}

/// <p>The inventory item result attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResultAttribute {
    /// <p>Name of the inventory item type. Valid value: AWS:InstanceInformation. Default Value: AWS:InstanceInformation.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

/// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3OutputLocation {
    /// <p>The name of the Amazon S3 bucket.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The Amazon S3 bucket subfolder.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
}

/// <p>A URL for the Amazon S3 bucket where you want to store the results of this request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct S3OutputUrl {
    /// <p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendAutomationSignalRequest {
    /// <p>The unique identifier for an existing Automation execution that you want to send the signal to.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>The data sent with the signal. The data schema depends on the type of signal used in the request. </p>
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The type of signal. Valid signal types include the following: Approve and Reject </p>
    #[serde(rename = "SignalType")]
    pub signal_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SendAutomationSignalResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendCommandRequest {
    /// <p>User-specified information about the command, such as a brief description of what the command should do.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p><p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "DocumentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    /// <p><p>Sha256 or Sha1.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note></p>
    #[serde(rename = "DocumentHashType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    /// <p>Required. The name of the Systems Manager document to execute. This can be a public document or a custom document.</p>
    #[serde(rename = "DocumentName")]
    pub document_name: String,
    /// <p>The instance IDs where the command should execute. You can specify a maximum of 50 IDs. If you prefer not to list individual instance IDs, you can instead send commands to a fleet of instances using the Targets parameter, which accepts EC2 tags. For more information about how to use Targets, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Sending Commands to a Fleet</a>.</p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p>(Optional) The maximum number of instances that are allowed to execute the command at the same time. You can specify a number such as 10 or a percentage such as 10%. The default value is 50. For more information about how to use MaxConcurrency, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-velocity.html">Using Concurrency Controls</a>.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The maximum number of errors allowed without the command failing. When the command fails one more time beyond the value of MaxErrors, the systems stops sending the command to additional targets. You can specify a number like 10 or a percentage like 10%. The default value is 0. For more information about how to use MaxErrors, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-maxerrors.html">Using Error Controls</a>.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>Configurations for sending notifications.</p>
    #[serde(rename = "NotificationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    /// <p>The name of the S3 bucket where command execution responses should be stored.</p>
    #[serde(rename = "OutputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The directory structure within the S3 bucket where the responses should be stored.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>(Deprecated) You can no longer specify this parameter. The system ignores it. Instead, Systems Manager automatically determines the Amazon S3 bucket region.</p>
    #[serde(rename = "OutputS3Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    /// <p>The required and optional parameters specified in the document being executed.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The IAM role that Systems Manager uses to send notifications. </p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>(Optional) An array of search criteria that targets instances using a Key,Value combination that you specify. Targets is required if you don't provide one or more instance IDs in the call. For more information about how to use Targets, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Sending Commands to a Fleet</a>.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>If this time is reached and the command has not already started executing, it will not run.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SendCommandResult {
    /// <p>The request as it was received by Systems Manager. Also provides the command ID which can be used future references to this request.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
}

/// <p>The number of managed instances found for each patch severity level defined in the request filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SeveritySummary {
    /// <p>The total number of resources or compliance items that have a severity level of critical. Critical severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "CriticalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of high. High severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "HighCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of informational. Informational severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "InformationalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of low. Low severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "LowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of medium. Medium severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "MediumCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_count: Option<i64>,
    /// <p>The total number of resources or compliance items that have a severity level of unspecified. Unspecified severity is determined by the organization that published the compliance items.</p>
    #[serde(rename = "UnspecifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAutomationExecutionRequest {
    /// <p>User-provided idempotency token. The token must be unique, is case insensitive, enforces the UUID format, and can't be reused.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The name of the Automation document to use for this execution.</p>
    #[serde(rename = "DocumentName")]
    pub document_name: String,
    /// <p>The version of the Automation document to use for this execution.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The maximum number of targets allowed to run this task in parallel. You can specify a number, such as 10, or a percentage, such as 10%. The default value is 10.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The number of errors that are allowed before the system stops running the automation on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops running the automation when the fourth error is received. If you specify 0, then the system stops running the automation on additional targets after the first error result is returned. If you run an automation on 50 resources and set max-errors to 10%, then the system stops running the automation on additional targets when the sixth error is received.</p> <p>Executions that are already running an automation when max-errors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set max-concurrency to 1 so the executions proceed one at a time.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The execution mode of the automation. Valid modes include the following: Auto and Interactive. The default mode is Auto.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>A key-value map of execution parameters, which match the declared parameters in the Automation document.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The name of the parameter used as the target resource for the rate-controlled execution. Required if you specify Targets.</p>
    #[serde(rename = "TargetParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    /// <p>A key-value mapping to target resources. Required if you specify TargetParameterName.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartAutomationExecutionResult {
    /// <p>The unique ID of a newly scheduled automation execution.</p>
    #[serde(rename = "AutomationExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
}

/// <p>Detailed information about an the execution state of an Automation step.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StepExecution {
    /// <p>The action this step performs. The action determines the behavior of the step.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>If a step has finished execution, this contains the time the execution ended. If the step has not yet concluded, this field is not populated.</p>
    #[serde(rename = "ExecutionEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    /// <p>If a step has begun execution, this contains the time the step started. If the step is in Pending status, this field is not populated.</p>
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    /// <p>Information about the Automation failure.</p>
    #[serde(rename = "FailureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    /// <p>If a step failed, this message explains why the execution failed.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>Fully-resolved values passed into the step before execution.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<::std::collections::HashMap<String, String>>,
    /// <p>The maximum number of tries to run the action of the step. The default value is 1.</p>
    #[serde(rename = "MaxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// <p>The action to take if the step fails. The default value is Abort.</p>
    #[serde(rename = "OnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
    /// <p>Returned values from the execution of the step.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A user-specified list of parameters to override when executing a step.</p>
    #[serde(rename = "OverriddenParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>A message associated with the response code for an execution.</p>
    #[serde(rename = "Response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// <p>The response code returned by the execution of the step.</p>
    #[serde(rename = "ResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// <p>The unique ID of a step execution.</p>
    #[serde(rename = "StepExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_execution_id: Option<String>,
    /// <p>The name of this execution step.</p>
    #[serde(rename = "StepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    /// <p>The execution status for this step. Valid values include: Pending, InProgress, Success, Cancelled, Failed, and TimedOut.</p>
    #[serde(rename = "StepStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
    /// <p>The timeout seconds of the step.</p>
    #[serde(rename = "TimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
}

/// <p>A filter to limit the amount of step execution information returned by the call.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StepExecutionFilter {
    /// <p>One or more keys to limit the results. Valid filter keys include the following: StepName, Action, StepExecutionId, StepExecutionStatus, StartTimeBefore, StartTimeAfter.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The values of the filter key.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAutomationExecutionRequest {
    /// <p>The execution ID of the Automation to stop.</p>
    #[serde(rename = "AutomationExecutionId")]
    pub automation_execution_id: String,
    /// <p>The stop request type. Valid types include the following: Cancel and Complete. The default type is Cancel.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopAutomationExecutionResult {}

/// <p>Metadata that you assign to your AWS resources. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. In Systems Manager, you can apply tags to documents, managed instances, Maintenance Windows, Parameter Store parameters, and patch baselines.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The name of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p><p>An array of search criteria that targets instances using a Key,Value combination that you specify. <code>Targets</code> is required if you don&#39;t provide one or more instance IDs in the call.</p> <p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// <p>User-defined criteria for sending commands that target instances that meet the criteria. Key can be tag:&lt;Amazon EC2 tag&gt; or InstanceIds. For more information about how to send commands that target instances using Key,Value parameters, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Executing a Command Using Systems Manager Run Command</a>.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>User-defined criteria that maps to Key. For example, if you specified tag:ServerRole, you could specify value:WebServer to execute a command on instances that include Amazon EC2 tags of ServerRole,WebServer. For more information about how to send commands that target instances using Key,Value parameters, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/send-commands-multiple.html">Executing a Command Using Systems Manager Run Command</a>.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssociationRequest {
    /// <p>The ID of the association you want to update. </p>
    #[serde(rename = "AssociationId")]
    pub association_id: String,
    /// <p>The name of the association that you want to update.</p>
    #[serde(rename = "AssociationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    /// <p>This parameter is provided for concurrency control purposes. You must specify the latest association version in the service. If you want to ensure that this request succeeds, either specify <code>$LATEST</code>, or omit this parameter.</p>
    #[serde(rename = "AssociationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    /// <p>The document version you want update for the association. </p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the association document.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An Amazon S3 bucket where you want to store the results of this request.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// <p>The parameters you want to update for the association. If you create a parameter using Parameter Store, you can reference the parameter using {{ssm:parameter-name}}</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The cron expression used to schedule the association that you want to update.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The targets of the association.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateAssociationResult {
    /// <p>The description of the association that was updated.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssociationStatusRequest {
    /// <p>The association status.</p>
    #[serde(rename = "AssociationStatus")]
    pub association_status: AssociationStatus,
    /// <p>The ID of the instance.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The name of the Systems Manager document.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateAssociationStatusResult {
    /// <p>Information about the association.</p>
    #[serde(rename = "AssociationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentDefaultVersionRequest {
    /// <p>The version of a custom document that you want to set as the default version.</p>
    #[serde(rename = "DocumentVersion")]
    pub document_version: String,
    /// <p>The name of a custom document that you want to set as the default version.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDocumentDefaultVersionResult {
    /// <p>The description of a custom document that you want to set as the default version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DocumentDefaultVersionDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentRequest {
    /// <p>The content in a document that you want to update.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>Specify the document format for the new document version. Systems Manager supports JSON and YAML documents. JSON is the default format.</p>
    #[serde(rename = "DocumentFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    /// <p>The version of the document that you want to update.</p>
    #[serde(rename = "DocumentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    /// <p>The name of the document that you want to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specify a new target type for the document.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDocumentResult {
    /// <p>A description of the document that was updated.</p>
    #[serde(rename = "DocumentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowRequest {
    /// <p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>An optional description for the update request.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the Maintenance Window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the Maintenance Window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the Maintenance Window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If True, then all fields that are required by the CreateMaintenanceWindow action are also required for this API request. Optional fields that are not specified are set to null. </p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The ID of the Maintenance Window to update.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateMaintenanceWindowResult {
    /// <p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    /// <p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>
    #[serde(rename = "Cutoff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i64>,
    /// <p>An optional description of the update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The duration of the Maintenance Window in hours.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Whether the Maintenance Window is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the Maintenance Window.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The ID of the created Maintenance Window.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowTargetRequest {
    /// <p>An optional description for the update.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A name for the update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>If True, then all fields that are required by the RegisterTargetWithMaintenanceWindow action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The targets to add or replace.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The Maintenance Window ID with which to modify the target.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The target ID to modify.</p>
    #[serde(rename = "WindowTargetId")]
    pub window_target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateMaintenanceWindowTargetResult {
    /// <p>The updated description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated owner.</p>
    #[serde(rename = "OwnerInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    /// <p>The updated targets.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The Maintenance Window ID specified in the update request.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The target ID specified in the update request.</p>
    #[serde(rename = "WindowTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMaintenanceWindowTaskRequest {
    /// <p>The new task description to specify.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The new logging location in Amazon S3 to specify.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The new <code>MaxConcurrency</code> value you want to specify. <code>MaxConcurrency</code> is the number of targets that are allowed to run this task in parallel.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The new <code>MaxErrors</code> value to specify. <code>MaxErrors</code> is the maximum number of errors that are allowed before the task stops being scheduled.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The new task name to specify.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The new task priority to specify. The lower the number, the higher the priority. Tasks that have the same priority are scheduled in parallel.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>If True, then all fields that are required by the RegisterTaskWithMaintenanceWndow action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>The IAM service role ARN to modify. The system assumes this role during task execution. </p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The targets (either instances or tags) to modify. Instances are specified using Key=instanceids,Values=instanceID_1,instanceID_2. Tags are specified using Key=tag_name,Values=tag_value. </p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The task ARN to modify.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The parameters that the task should use during execution. Populate only the fields that match the task type. All other fields should be empty.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p>The parameters to modify.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note> <p>The map has the following format:</p> <p>Key: string, between 1 and 255 characters</p> <p>Value: an array of strings, each string is between 1 and 255 characters</p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The Maintenance Window ID that contains the task to modify.</p>
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// <p>The task ID to modify.</p>
    #[serde(rename = "WindowTaskId")]
    pub window_task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateMaintenanceWindowTaskResult {
    /// <p>The updated task description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>The updated logging information in Amazon S3.</p> <note> <p> <code>LoggingInfo</code> has been deprecated. To specify an S3 bucket to contain logs, instead use the <code>OutputS3BucketName</code> and <code>OutputS3KeyPrefix</code> options in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "LoggingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    /// <p>The updated MaxConcurrency value.</p>
    #[serde(rename = "MaxConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    /// <p>The updated MaxErrors value.</p>
    #[serde(rename = "MaxErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    /// <p>The updated task name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated priority value.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The updated service role ARN value.</p>
    #[serde(rename = "ServiceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The updated target values.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    /// <p>The updated task ARN value.</p>
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The updated parameter values.</p>
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    /// <p><p>The updated parameter values.</p> <note> <p> <code>TaskParameters</code> has been deprecated. To specify parameters to pass to a task when it runs, instead use the <code>Parameters</code> option in the <code>TaskInvocationParameters</code> structure. For information about how Systems Manager handles these options for the supported Maintenance Window task types, see <a>MaintenanceWindowTaskInvocationParameters</a>.</p> </note></p>
    #[serde(rename = "TaskParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<::std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    /// <p>The ID of the Maintenance Window that was updated.</p>
    #[serde(rename = "WindowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// <p>The task ID of the Maintenance Window that was updated.</p>
    #[serde(rename = "WindowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateManagedInstanceRoleRequest {
    /// <p>The IAM role you want to assign or change.</p>
    #[serde(rename = "IamRole")]
    pub iam_role: String,
    /// <p>The ID of the managed instance where you want to update the role.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateManagedInstanceRoleResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePatchBaselineRequest {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>Assigns a new compliance severity level to an existing patch baseline.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the patch baseline to update.</p>
    #[serde(rename = "BaselineId")]
    pub baseline_id: String,
    /// <p>A description of the patch baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p> <p>For information about accepted formats for lists of approved patches and rejected patches, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/patch-manager-approved-rejected-package-name-formats.html">Package Name Formats for Approved and Rejected Patch Lists</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>If True, then all fields that are required by the CreatePatchBaseline action are also required for this API request. Optional fields that are not specified are set to null.</p>
    #[serde(rename = "Replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatePatchBaselineResult {
    /// <p>A set of rules used to include patches in the baseline.</p>
    #[serde(rename = "ApprovalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    /// <p>A list of explicitly approved patches for the baseline.</p>
    #[serde(rename = "ApprovedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    /// <p>The compliance severity level assigned to the patch baseline after the update completed.</p>
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    /// <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the instances. The default value is 'false'. Applies to Linux instances only.</p>
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// <p>The ID of the deleted patch baseline.</p>
    #[serde(rename = "BaselineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    /// <p>The date when the patch baseline was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the Patch Baseline.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A set of global filters used to exclude patches from the baseline.</p>
    #[serde(rename = "GlobalFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    /// <p>The date when the patch baseline was last modified.</p>
    #[serde(rename = "ModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    /// <p>The name of the patch baseline.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system rule used by the updated patch baseline.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>A list of explicitly rejected patches for the baseline.</p>
    #[serde(rename = "RejectedPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    /// <p>Information about the patches to use to update the instances, including target operating systems and source repositories. Applies to Linux instances only.</p>
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// <p>The Targets parameter includes too many tags. Remove one or more tags and try the command again.</p>
    TooManyTagsError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsToResourceError {
    pub fn from_body(body: &str) -> AddTagsToResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        AddTagsToResourceError::InternalServerError(String::from(error_message))
                    }
                    "InvalidResourceId" => {
                        AddTagsToResourceError::InvalidResourceId(String::from(error_message))
                    }
                    "InvalidResourceType" => {
                        AddTagsToResourceError::InvalidResourceType(String::from(error_message))
                    }
                    "TooManyTagsError" => {
                        AddTagsToResourceError::TooManyTagsError(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToResourceError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToResourceError {
    fn from(err: serde_json::error::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsToResourceError {
    fn from(err: CredentialsError) -> AddTagsToResourceError {
        AddTagsToResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToResourceError {
    fn from(err: HttpDispatchError) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsToResourceError {
    fn from(err: io::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::InternalServerError(ref cause) => cause,
            AddTagsToResourceError::InvalidResourceId(ref cause) => cause,
            AddTagsToResourceError::InvalidResourceType(ref cause) => cause,
            AddTagsToResourceError::TooManyTagsError(ref cause) => cause,
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelCommand
#[derive(Debug, PartialEq)]
pub enum CancelCommandError {
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelCommandError {
    pub fn from_body(body: &str) -> CancelCommandError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateInstanceId" => {
                        CancelCommandError::DuplicateInstanceId(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CancelCommandError::InternalServerError(String::from(error_message))
                    }
                    "InvalidCommandId" => {
                        CancelCommandError::InvalidCommandId(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        CancelCommandError::InvalidInstanceId(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelCommandError::Validation(error_message.to_string())
                    }
                    _ => CancelCommandError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelCommandError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelCommandError {
    fn from(err: serde_json::error::Error) -> CancelCommandError {
        CancelCommandError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelCommandError {
    fn from(err: CredentialsError) -> CancelCommandError {
        CancelCommandError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelCommandError {
    fn from(err: HttpDispatchError) -> CancelCommandError {
        CancelCommandError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelCommandError {
    fn from(err: io::Error) -> CancelCommandError {
        CancelCommandError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelCommandError {
    fn description(&self) -> &str {
        match *self {
            CancelCommandError::DuplicateInstanceId(ref cause) => cause,
            CancelCommandError::InternalServerError(ref cause) => cause,
            CancelCommandError::InvalidCommandId(ref cause) => cause,
            CancelCommandError::InvalidInstanceId(ref cause) => cause,
            CancelCommandError::Validation(ref cause) => cause,
            CancelCommandError::Credentials(ref err) => err.description(),
            CancelCommandError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelCommandError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateActivation
#[derive(Debug, PartialEq)]
pub enum CreateActivationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateActivationError {
    pub fn from_body(body: &str) -> CreateActivationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        CreateActivationError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateActivationError::Validation(error_message.to_string())
                    }
                    _ => CreateActivationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateActivationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateActivationError {
    fn from(err: serde_json::error::Error) -> CreateActivationError {
        CreateActivationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateActivationError {
    fn from(err: CredentialsError) -> CreateActivationError {
        CreateActivationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateActivationError {
    fn from(err: HttpDispatchError) -> CreateActivationError {
        CreateActivationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateActivationError {
    fn from(err: io::Error) -> CreateActivationError {
        CreateActivationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateActivationError {
    fn description(&self) -> &str {
        match *self {
            CreateActivationError::InternalServerError(ref cause) => cause,
            CreateActivationError::Validation(ref cause) => cause,
            CreateActivationError::Credentials(ref err) => err.description(),
            CreateActivationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateActivationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssociation
#[derive(Debug, PartialEq)]
pub enum CreateAssociationError {
    /// <p>The specified association already exists.</p>
    AssociationAlreadyExists(String),
    /// <p>You can have at most 2,000 active associations.</p>
    AssociationLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAssociationError {
    pub fn from_body(body: &str) -> CreateAssociationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationAlreadyExists" => CreateAssociationError::AssociationAlreadyExists(
                        String::from(error_message),
                    ),
                    "AssociationLimitExceeded" => CreateAssociationError::AssociationLimitExceeded(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        CreateAssociationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        CreateAssociationError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentVersion" => {
                        CreateAssociationError::InvalidDocumentVersion(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        CreateAssociationError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidOutputLocation" => {
                        CreateAssociationError::InvalidOutputLocation(String::from(error_message))
                    }
                    "InvalidParameters" => {
                        CreateAssociationError::InvalidParameters(String::from(error_message))
                    }
                    "InvalidSchedule" => {
                        CreateAssociationError::InvalidSchedule(String::from(error_message))
                    }
                    "InvalidTarget" => {
                        CreateAssociationError::InvalidTarget(String::from(error_message))
                    }
                    "UnsupportedPlatformType" => {
                        CreateAssociationError::UnsupportedPlatformType(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAssociationError::Validation(error_message.to_string())
                    }
                    _ => CreateAssociationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAssociationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAssociationError {
    fn from(err: serde_json::error::Error) -> CreateAssociationError {
        CreateAssociationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAssociationError {
    fn from(err: CredentialsError) -> CreateAssociationError {
        CreateAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAssociationError {
    fn from(err: HttpDispatchError) -> CreateAssociationError {
        CreateAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAssociationError {
    fn from(err: io::Error) -> CreateAssociationError {
        CreateAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssociationError {
    fn description(&self) -> &str {
        match *self {
            CreateAssociationError::AssociationAlreadyExists(ref cause) => cause,
            CreateAssociationError::AssociationLimitExceeded(ref cause) => cause,
            CreateAssociationError::InternalServerError(ref cause) => cause,
            CreateAssociationError::InvalidDocument(ref cause) => cause,
            CreateAssociationError::InvalidDocumentVersion(ref cause) => cause,
            CreateAssociationError::InvalidInstanceId(ref cause) => cause,
            CreateAssociationError::InvalidOutputLocation(ref cause) => cause,
            CreateAssociationError::InvalidParameters(ref cause) => cause,
            CreateAssociationError::InvalidSchedule(ref cause) => cause,
            CreateAssociationError::InvalidTarget(ref cause) => cause,
            CreateAssociationError::UnsupportedPlatformType(ref cause) => cause,
            CreateAssociationError::Validation(ref cause) => cause,
            CreateAssociationError::Credentials(ref err) => err.description(),
            CreateAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAssociationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssociationBatch
#[derive(Debug, PartialEq)]
pub enum CreateAssociationBatchError {
    /// <p>You can have at most 2,000 active associations.</p>
    AssociationLimitExceeded(String),
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAssociationBatchError {
    pub fn from_body(body: &str) -> CreateAssociationBatchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationLimitExceeded" => {
                        CreateAssociationBatchError::AssociationLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "DuplicateInstanceId" => CreateAssociationBatchError::DuplicateInstanceId(
                        String::from(error_message),
                    ),
                    "InternalServerError" => CreateAssociationBatchError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidDocument" => {
                        CreateAssociationBatchError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentVersion" => {
                        CreateAssociationBatchError::InvalidDocumentVersion(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInstanceId" => {
                        CreateAssociationBatchError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidOutputLocation" => CreateAssociationBatchError::InvalidOutputLocation(
                        String::from(error_message),
                    ),
                    "InvalidParameters" => {
                        CreateAssociationBatchError::InvalidParameters(String::from(error_message))
                    }
                    "InvalidSchedule" => {
                        CreateAssociationBatchError::InvalidSchedule(String::from(error_message))
                    }
                    "InvalidTarget" => {
                        CreateAssociationBatchError::InvalidTarget(String::from(error_message))
                    }
                    "UnsupportedPlatformType" => {
                        CreateAssociationBatchError::UnsupportedPlatformType(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateAssociationBatchError::Validation(error_message.to_string())
                    }
                    _ => CreateAssociationBatchError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAssociationBatchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAssociationBatchError {
    fn from(err: serde_json::error::Error) -> CreateAssociationBatchError {
        CreateAssociationBatchError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAssociationBatchError {
    fn from(err: CredentialsError) -> CreateAssociationBatchError {
        CreateAssociationBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAssociationBatchError {
    fn from(err: HttpDispatchError) -> CreateAssociationBatchError {
        CreateAssociationBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAssociationBatchError {
    fn from(err: io::Error) -> CreateAssociationBatchError {
        CreateAssociationBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAssociationBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssociationBatchError {
    fn description(&self) -> &str {
        match *self {
            CreateAssociationBatchError::AssociationLimitExceeded(ref cause) => cause,
            CreateAssociationBatchError::DuplicateInstanceId(ref cause) => cause,
            CreateAssociationBatchError::InternalServerError(ref cause) => cause,
            CreateAssociationBatchError::InvalidDocument(ref cause) => cause,
            CreateAssociationBatchError::InvalidDocumentVersion(ref cause) => cause,
            CreateAssociationBatchError::InvalidInstanceId(ref cause) => cause,
            CreateAssociationBatchError::InvalidOutputLocation(ref cause) => cause,
            CreateAssociationBatchError::InvalidParameters(ref cause) => cause,
            CreateAssociationBatchError::InvalidSchedule(ref cause) => cause,
            CreateAssociationBatchError::InvalidTarget(ref cause) => cause,
            CreateAssociationBatchError::UnsupportedPlatformType(ref cause) => cause,
            CreateAssociationBatchError::Validation(ref cause) => cause,
            CreateAssociationBatchError::Credentials(ref err) => err.description(),
            CreateAssociationBatchError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateAssociationBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocument
#[derive(Debug, PartialEq)]
pub enum CreateDocumentError {
    /// <p>The specified document already exists.</p>
    DocumentAlreadyExists(String),
    /// <p>You can have at most 200 active Systems Manager documents.</p>
    DocumentLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The content for the document is not valid.</p>
    InvalidDocumentContent(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDocumentError {
    pub fn from_body(body: &str) -> CreateDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DocumentAlreadyExists" => {
                        CreateDocumentError::DocumentAlreadyExists(String::from(error_message))
                    }
                    "DocumentLimitExceeded" => {
                        CreateDocumentError::DocumentLimitExceeded(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CreateDocumentError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocumentContent" => {
                        CreateDocumentError::InvalidDocumentContent(String::from(error_message))
                    }
                    "InvalidDocumentSchemaVersion" => {
                        CreateDocumentError::InvalidDocumentSchemaVersion(String::from(
                            error_message,
                        ))
                    }
                    "MaxDocumentSizeExceeded" => {
                        CreateDocumentError::MaxDocumentSizeExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDocumentError::Validation(error_message.to_string())
                    }
                    _ => CreateDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDocumentError {
    fn from(err: serde_json::error::Error) -> CreateDocumentError {
        CreateDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDocumentError {
    fn from(err: CredentialsError) -> CreateDocumentError {
        CreateDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDocumentError {
    fn from(err: HttpDispatchError) -> CreateDocumentError {
        CreateDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDocumentError {
    fn from(err: io::Error) -> CreateDocumentError {
        CreateDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentError::DocumentAlreadyExists(ref cause) => cause,
            CreateDocumentError::DocumentLimitExceeded(ref cause) => cause,
            CreateDocumentError::InternalServerError(ref cause) => cause,
            CreateDocumentError::InvalidDocumentContent(ref cause) => cause,
            CreateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,
            CreateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,
            CreateDocumentError::Validation(ref cause) => cause,
            CreateDocumentError::Credentials(ref err) => err.description(),
            CreateDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum CreateMaintenanceWindowError {
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many Maintenance Windows or Patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateMaintenanceWindowError {
    pub fn from_body(body: &str) -> CreateMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "IdempotentParameterMismatch" => {
                        CreateMaintenanceWindowError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => CreateMaintenanceWindowError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ResourceLimitExceededException" => {
                        CreateMaintenanceWindowError::ResourceLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateMaintenanceWindowError::Validation(error_message.to_string())
                    }
                    _ => CreateMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> CreateMaintenanceWindowError {
        CreateMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMaintenanceWindowError {
    fn from(err: CredentialsError) -> CreateMaintenanceWindowError {
        CreateMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> CreateMaintenanceWindowError {
        CreateMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMaintenanceWindowError {
    fn from(err: io::Error) -> CreateMaintenanceWindowError {
        CreateMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            CreateMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,
            CreateMaintenanceWindowError::InternalServerError(ref cause) => cause,
            CreateMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
            CreateMaintenanceWindowError::Validation(ref cause) => cause,
            CreateMaintenanceWindowError::Credentials(ref err) => err.description(),
            CreateMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePatchBaseline
#[derive(Debug, PartialEq)]
pub enum CreatePatchBaselineError {
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many Maintenance Windows or Patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePatchBaselineError {
    pub fn from_body(body: &str) -> CreatePatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "IdempotentParameterMismatch" => {
                        CreatePatchBaselineError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        CreatePatchBaselineError::InternalServerError(String::from(error_message))
                    }
                    "ResourceLimitExceededException" => {
                        CreatePatchBaselineError::ResourceLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => CreatePatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePatchBaselineError {
    fn from(err: serde_json::error::Error) -> CreatePatchBaselineError {
        CreatePatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePatchBaselineError {
    fn from(err: CredentialsError) -> CreatePatchBaselineError {
        CreatePatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePatchBaselineError {
    fn from(err: HttpDispatchError) -> CreatePatchBaselineError {
        CreatePatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePatchBaselineError {
    fn from(err: io::Error) -> CreatePatchBaselineError {
        CreatePatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            CreatePatchBaselineError::IdempotentParameterMismatch(ref cause) => cause,
            CreatePatchBaselineError::InternalServerError(ref cause) => cause,
            CreatePatchBaselineError::ResourceLimitExceeded(ref cause) => cause,
            CreatePatchBaselineError::Validation(ref cause) => cause,
            CreatePatchBaselineError::Credentials(ref err) => err.description(),
            CreatePatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceDataSync
#[derive(Debug, PartialEq)]
pub enum CreateResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>A sync configuration with the same name already exists.</p>
    ResourceDataSyncAlreadyExists(String),
    /// <p>You have exceeded the allowed maximum sync configurations.</p>
    ResourceDataSyncCountExceeded(String),
    /// <p>The specified sync configuration is invalid.</p>
    ResourceDataSyncInvalidConfiguration(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateResourceDataSyncError {
    pub fn from_body(body: &str) -> CreateResourceDataSyncError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => CreateResourceDataSyncError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ResourceDataSyncAlreadyExistsException" => {
                        CreateResourceDataSyncError::ResourceDataSyncAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "ResourceDataSyncCountExceededException" => {
                        CreateResourceDataSyncError::ResourceDataSyncCountExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceDataSyncInvalidConfigurationException" => {
                        CreateResourceDataSyncError::ResourceDataSyncInvalidConfiguration(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        CreateResourceDataSyncError::Validation(error_message.to_string())
                    }
                    _ => CreateResourceDataSyncError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateResourceDataSyncError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateResourceDataSyncError {
    fn from(err: serde_json::error::Error) -> CreateResourceDataSyncError {
        CreateResourceDataSyncError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateResourceDataSyncError {
    fn from(err: CredentialsError) -> CreateResourceDataSyncError {
        CreateResourceDataSyncError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateResourceDataSyncError {
    fn from(err: HttpDispatchError) -> CreateResourceDataSyncError {
        CreateResourceDataSyncError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateResourceDataSyncError {
    fn from(err: io::Error) -> CreateResourceDataSyncError {
        CreateResourceDataSyncError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceDataSyncError::InternalServerError(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncAlreadyExists(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncCountExceeded(ref cause) => cause,
            CreateResourceDataSyncError::ResourceDataSyncInvalidConfiguration(ref cause) => cause,
            CreateResourceDataSyncError::Validation(ref cause) => cause,
            CreateResourceDataSyncError::Credentials(ref err) => err.description(),
            CreateResourceDataSyncError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateResourceDataSyncError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteActivation
#[derive(Debug, PartialEq)]
pub enum DeleteActivationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The activation is not valid. The activation might have been deleted, or the ActivationId and the ActivationCode do not match.</p>
    InvalidActivation(String),
    /// <p>The activation ID is not valid. Verify the you entered the correct ActivationId or ActivationCode and try again.</p>
    InvalidActivationId(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteActivationError {
    pub fn from_body(body: &str) -> DeleteActivationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeleteActivationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidActivation" => {
                        DeleteActivationError::InvalidActivation(String::from(error_message))
                    }
                    "InvalidActivationId" => {
                        DeleteActivationError::InvalidActivationId(String::from(error_message))
                    }
                    "TooManyUpdates" => {
                        DeleteActivationError::TooManyUpdates(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteActivationError::Validation(error_message.to_string())
                    }
                    _ => DeleteActivationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteActivationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteActivationError {
    fn from(err: serde_json::error::Error) -> DeleteActivationError {
        DeleteActivationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteActivationError {
    fn from(err: CredentialsError) -> DeleteActivationError {
        DeleteActivationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteActivationError {
    fn from(err: HttpDispatchError) -> DeleteActivationError {
        DeleteActivationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteActivationError {
    fn from(err: io::Error) -> DeleteActivationError {
        DeleteActivationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteActivationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteActivationError {
    fn description(&self) -> &str {
        match *self {
            DeleteActivationError::InternalServerError(ref cause) => cause,
            DeleteActivationError::InvalidActivation(ref cause) => cause,
            DeleteActivationError::InvalidActivationId(ref cause) => cause,
            DeleteActivationError::TooManyUpdates(ref cause) => cause,
            DeleteActivationError::Validation(ref cause) => cause,
            DeleteActivationError::Credentials(ref err) => err.description(),
            DeleteActivationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteActivationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAssociationError {
    pub fn from_body(body: &str) -> DeleteAssociationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationDoesNotExist" => {
                        DeleteAssociationError::AssociationDoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteAssociationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        DeleteAssociationError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        DeleteAssociationError::InvalidInstanceId(String::from(error_message))
                    }
                    "TooManyUpdates" => {
                        DeleteAssociationError::TooManyUpdates(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAssociationError::Validation(error_message.to_string())
                    }
                    _ => DeleteAssociationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAssociationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAssociationError {
    fn from(err: serde_json::error::Error) -> DeleteAssociationError {
        DeleteAssociationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAssociationError {
    fn from(err: CredentialsError) -> DeleteAssociationError {
        DeleteAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAssociationError {
    fn from(err: HttpDispatchError) -> DeleteAssociationError {
        DeleteAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAssociationError {
    fn from(err: io::Error) -> DeleteAssociationError {
        DeleteAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssociationError::AssociationDoesNotExist(ref cause) => cause,
            DeleteAssociationError::InternalServerError(ref cause) => cause,
            DeleteAssociationError::InvalidDocument(ref cause) => cause,
            DeleteAssociationError::InvalidInstanceId(ref cause) => cause,
            DeleteAssociationError::TooManyUpdates(ref cause) => cause,
            DeleteAssociationError::Validation(ref cause) => cause,
            DeleteAssociationError::Credentials(ref err) => err.description(),
            DeleteAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAssociationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocument
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentError {
    /// <p>You must disassociate a document from all instances before you can delete it.</p>
    AssociatedInstances(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it.</p>
    InvalidDocumentOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDocumentError {
    pub fn from_body(body: &str) -> DeleteDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociatedInstances" => {
                        DeleteDocumentError::AssociatedInstances(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteDocumentError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        DeleteDocumentError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentOperation" => {
                        DeleteDocumentError::InvalidDocumentOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDocumentError::Validation(error_message.to_string())
                    }
                    _ => DeleteDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDocumentError {
    fn from(err: serde_json::error::Error) -> DeleteDocumentError {
        DeleteDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDocumentError {
    fn from(err: CredentialsError) -> DeleteDocumentError {
        DeleteDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDocumentError {
    fn from(err: HttpDispatchError) -> DeleteDocumentError {
        DeleteDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDocumentError {
    fn from(err: io::Error) -> DeleteDocumentError {
        DeleteDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentError::AssociatedInstances(ref cause) => cause,
            DeleteDocumentError::InternalServerError(ref cause) => cause,
            DeleteDocumentError::InvalidDocument(ref cause) => cause,
            DeleteDocumentError::InvalidDocumentOperation(ref cause) => cause,
            DeleteDocumentError::Validation(ref cause) => cause,
            DeleteDocumentError::Credentials(ref err) => err.description(),
            DeleteDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeleteMaintenanceWindowError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMaintenanceWindowError {
    pub fn from_body(body: &str) -> DeleteMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DeleteMaintenanceWindowError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DeleteMaintenanceWindowError::Validation(error_message.to_string())
                    }
                    _ => DeleteMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> DeleteMaintenanceWindowError {
        DeleteMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMaintenanceWindowError {
    fn from(err: CredentialsError) -> DeleteMaintenanceWindowError {
        DeleteMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> DeleteMaintenanceWindowError {
        DeleteMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMaintenanceWindowError {
    fn from(err: io::Error) -> DeleteMaintenanceWindowError {
        DeleteMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeleteMaintenanceWindowError::InternalServerError(ref cause) => cause,
            DeleteMaintenanceWindowError::Validation(ref cause) => cause,
            DeleteMaintenanceWindowError::Credentials(ref err) => err.description(),
            DeleteMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameter
#[derive(Debug, PartialEq)]
pub enum DeleteParameterError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteParameterError {
    pub fn from_body(body: &str) -> DeleteParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeleteParameterError::InternalServerError(String::from(error_message))
                    }
                    "ParameterNotFound" => {
                        DeleteParameterError::ParameterNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteParameterError::Validation(error_message.to_string())
                    }
                    _ => DeleteParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteParameterError {
    fn from(err: serde_json::error::Error) -> DeleteParameterError {
        DeleteParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteParameterError {
    fn from(err: CredentialsError) -> DeleteParameterError {
        DeleteParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteParameterError {
    fn from(err: HttpDispatchError) -> DeleteParameterError {
        DeleteParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteParameterError {
    fn from(err: io::Error) -> DeleteParameterError {
        DeleteParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParameterError {
    fn description(&self) -> &str {
        match *self {
            DeleteParameterError::InternalServerError(ref cause) => cause,
            DeleteParameterError::ParameterNotFound(ref cause) => cause,
            DeleteParameterError::Validation(ref cause) => cause,
            DeleteParameterError::Credentials(ref err) => err.description(),
            DeleteParameterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteParameters
#[derive(Debug, PartialEq)]
pub enum DeleteParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteParametersError {
    pub fn from_body(body: &str) -> DeleteParametersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeleteParametersError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteParametersError::Validation(error_message.to_string())
                    }
                    _ => DeleteParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteParametersError {
    fn from(err: serde_json::error::Error) -> DeleteParametersError {
        DeleteParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteParametersError {
    fn from(err: CredentialsError) -> DeleteParametersError {
        DeleteParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteParametersError {
    fn from(err: HttpDispatchError) -> DeleteParametersError {
        DeleteParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteParametersError {
    fn from(err: io::Error) -> DeleteParametersError {
        DeleteParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteParametersError {
    fn description(&self) -> &str {
        match *self {
            DeleteParametersError::InternalServerError(ref cause) => cause,
            DeleteParametersError::Validation(ref cause) => cause,
            DeleteParametersError::Credentials(ref err) => err.description(),
            DeleteParametersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePatchBaseline
#[derive(Debug, PartialEq)]
pub enum DeletePatchBaselineError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned if an attempt is made to delete a patch baseline that is registered for a patch group.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePatchBaselineError {
    pub fn from_body(body: &str) -> DeletePatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeletePatchBaselineError::InternalServerError(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeletePatchBaselineError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => DeletePatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePatchBaselineError {
    fn from(err: serde_json::error::Error) -> DeletePatchBaselineError {
        DeletePatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePatchBaselineError {
    fn from(err: CredentialsError) -> DeletePatchBaselineError {
        DeletePatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePatchBaselineError {
    fn from(err: HttpDispatchError) -> DeletePatchBaselineError {
        DeletePatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePatchBaselineError {
    fn from(err: io::Error) -> DeletePatchBaselineError {
        DeletePatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            DeletePatchBaselineError::InternalServerError(ref cause) => cause,
            DeletePatchBaselineError::ResourceInUse(ref cause) => cause,
            DeletePatchBaselineError::Validation(ref cause) => cause,
            DeletePatchBaselineError::Credentials(ref err) => err.description(),
            DeletePatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourceDataSync
#[derive(Debug, PartialEq)]
pub enum DeleteResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified sync name was not found.</p>
    ResourceDataSyncNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteResourceDataSyncError {
    pub fn from_body(body: &str) -> DeleteResourceDataSyncError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DeleteResourceDataSyncError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ResourceDataSyncNotFoundException" => {
                        DeleteResourceDataSyncError::ResourceDataSyncNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeleteResourceDataSyncError::Validation(error_message.to_string())
                    }
                    _ => DeleteResourceDataSyncError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteResourceDataSyncError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteResourceDataSyncError {
    fn from(err: serde_json::error::Error) -> DeleteResourceDataSyncError {
        DeleteResourceDataSyncError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteResourceDataSyncError {
    fn from(err: CredentialsError) -> DeleteResourceDataSyncError {
        DeleteResourceDataSyncError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteResourceDataSyncError {
    fn from(err: HttpDispatchError) -> DeleteResourceDataSyncError {
        DeleteResourceDataSyncError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteResourceDataSyncError {
    fn from(err: io::Error) -> DeleteResourceDataSyncError {
        DeleteResourceDataSyncError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceDataSyncError::InternalServerError(ref cause) => cause,
            DeleteResourceDataSyncError::ResourceDataSyncNotFound(ref cause) => cause,
            DeleteResourceDataSyncError::Validation(ref cause) => cause,
            DeleteResourceDataSyncError::Credentials(ref err) => err.description(),
            DeleteResourceDataSyncError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteResourceDataSyncError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterManagedInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterManagedInstanceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterManagedInstanceError {
    pub fn from_body(body: &str) -> DeregisterManagedInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DeregisterManagedInstanceError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidInstanceId" => DeregisterManagedInstanceError::InvalidInstanceId(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DeregisterManagedInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeregisterManagedInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterManagedInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterManagedInstanceError {
    fn from(err: serde_json::error::Error) -> DeregisterManagedInstanceError {
        DeregisterManagedInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterManagedInstanceError {
    fn from(err: CredentialsError) -> DeregisterManagedInstanceError {
        DeregisterManagedInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterManagedInstanceError {
    fn from(err: HttpDispatchError) -> DeregisterManagedInstanceError {
        DeregisterManagedInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterManagedInstanceError {
    fn from(err: io::Error) -> DeregisterManagedInstanceError {
        DeregisterManagedInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterManagedInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterManagedInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterManagedInstanceError::InternalServerError(ref cause) => cause,
            DeregisterManagedInstanceError::InvalidInstanceId(ref cause) => cause,
            DeregisterManagedInstanceError::Validation(ref cause) => cause,
            DeregisterManagedInstanceError::Credentials(ref err) => err.description(),
            DeregisterManagedInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterManagedInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum DeregisterPatchBaselineForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterPatchBaselineForPatchGroupError {
    pub fn from_body(body: &str) -> DeregisterPatchBaselineForPatchGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeregisterPatchBaselineForPatchGroupError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "InvalidResourceId" => {
                        DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DeregisterPatchBaselineForPatchGroupError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DeregisterPatchBaselineForPatchGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterPatchBaselineForPatchGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterPatchBaselineForPatchGroupError {
    fn from(err: serde_json::error::Error) -> DeregisterPatchBaselineForPatchGroupError {
        DeregisterPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterPatchBaselineForPatchGroupError {
    fn from(err: CredentialsError) -> DeregisterPatchBaselineForPatchGroupError {
        DeregisterPatchBaselineForPatchGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterPatchBaselineForPatchGroupError {
    fn from(err: HttpDispatchError) -> DeregisterPatchBaselineForPatchGroupError {
        DeregisterPatchBaselineForPatchGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterPatchBaselineForPatchGroupError {
    fn from(err: io::Error) -> DeregisterPatchBaselineForPatchGroupError {
        DeregisterPatchBaselineForPatchGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            DeregisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
            DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,
            DeregisterPatchBaselineForPatchGroupError::Validation(ref cause) => cause,
            DeregisterPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),
            DeregisterPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterPatchBaselineForPatchGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTargetFromMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeregisterTargetFromMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>You specified the <code>Safe</code> option for the DeregisterTargetFromMaintenanceWindow operation, but the target is still referenced in a task.</p>
    TargetInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterTargetFromMaintenanceWindowError {
    pub fn from_body(body: &str) -> DeregisterTargetFromMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        DeregisterTargetFromMaintenanceWindowError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        DeregisterTargetFromMaintenanceWindowError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "TargetInUseException" => {
                        DeregisterTargetFromMaintenanceWindowError::TargetInUse(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DeregisterTargetFromMaintenanceWindowError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DeregisterTargetFromMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterTargetFromMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterTargetFromMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> DeregisterTargetFromMaintenanceWindowError {
        DeregisterTargetFromMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterTargetFromMaintenanceWindowError {
    fn from(err: CredentialsError) -> DeregisterTargetFromMaintenanceWindowError {
        DeregisterTargetFromMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterTargetFromMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> DeregisterTargetFromMaintenanceWindowError {
        DeregisterTargetFromMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterTargetFromMaintenanceWindowError {
    fn from(err: io::Error) -> DeregisterTargetFromMaintenanceWindowError {
        DeregisterTargetFromMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterTargetFromMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTargetFromMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTargetFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::InternalServerError(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::TargetInUse(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::Validation(ref cause) => cause,
            DeregisterTargetFromMaintenanceWindowError::Credentials(ref err) => err.description(),
            DeregisterTargetFromMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterTargetFromMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTaskFromMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum DeregisterTaskFromMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeregisterTaskFromMaintenanceWindowError {
    pub fn from_body(body: &str) -> DeregisterTaskFromMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        DeregisterTaskFromMaintenanceWindowError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        DeregisterTaskFromMaintenanceWindowError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DeregisterTaskFromMaintenanceWindowError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DeregisterTaskFromMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeregisterTaskFromMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeregisterTaskFromMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> DeregisterTaskFromMaintenanceWindowError {
        DeregisterTaskFromMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeregisterTaskFromMaintenanceWindowError {
    fn from(err: CredentialsError) -> DeregisterTaskFromMaintenanceWindowError {
        DeregisterTaskFromMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeregisterTaskFromMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> DeregisterTaskFromMaintenanceWindowError {
        DeregisterTaskFromMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeregisterTaskFromMaintenanceWindowError {
    fn from(err: io::Error) -> DeregisterTaskFromMaintenanceWindowError {
        DeregisterTaskFromMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeregisterTaskFromMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTaskFromMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTaskFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            DeregisterTaskFromMaintenanceWindowError::InternalServerError(ref cause) => cause,
            DeregisterTaskFromMaintenanceWindowError::Validation(ref cause) => cause,
            DeregisterTaskFromMaintenanceWindowError::Credentials(ref err) => err.description(),
            DeregisterTaskFromMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeregisterTaskFromMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeActivations
#[derive(Debug, PartialEq)]
pub enum DescribeActivationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeActivationsError {
    pub fn from_body(body: &str) -> DescribeActivationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeActivationsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilter" => {
                        DescribeActivationsError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        DescribeActivationsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeActivationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeActivationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeActivationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeActivationsError {
    fn from(err: serde_json::error::Error) -> DescribeActivationsError {
        DescribeActivationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeActivationsError {
    fn from(err: CredentialsError) -> DescribeActivationsError {
        DescribeActivationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeActivationsError {
    fn from(err: HttpDispatchError) -> DescribeActivationsError {
        DescribeActivationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeActivationsError {
    fn from(err: io::Error) -> DescribeActivationsError {
        DescribeActivationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeActivationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeActivationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeActivationsError::InternalServerError(ref cause) => cause,
            DescribeActivationsError::InvalidFilter(ref cause) => cause,
            DescribeActivationsError::InvalidNextToken(ref cause) => cause,
            DescribeActivationsError::Validation(ref cause) => cause,
            DescribeActivationsError::Credentials(ref err) => err.description(),
            DescribeActivationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeActivationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssociation
#[derive(Debug, PartialEq)]
pub enum DescribeAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The version you specified is not valid. Use ListAssociationVersions to view all versions of an association according to the association ID. Or, use the <code>$LATEST</code> parameter to view the latest version of the association.</p>
    InvalidAssociationVersion(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAssociationError {
    pub fn from_body(body: &str) -> DescribeAssociationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationDoesNotExist" => DescribeAssociationError::AssociationDoesNotExist(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        DescribeAssociationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidAssociationVersion" => {
                        DescribeAssociationError::InvalidAssociationVersion(String::from(
                            error_message,
                        ))
                    }
                    "InvalidDocument" => {
                        DescribeAssociationError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        DescribeAssociationError::InvalidInstanceId(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAssociationError::Validation(error_message.to_string())
                    }
                    _ => DescribeAssociationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAssociationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAssociationError {
    fn from(err: serde_json::error::Error) -> DescribeAssociationError {
        DescribeAssociationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAssociationError {
    fn from(err: CredentialsError) -> DescribeAssociationError {
        DescribeAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAssociationError {
    fn from(err: HttpDispatchError) -> DescribeAssociationError {
        DescribeAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAssociationError {
    fn from(err: io::Error) -> DescribeAssociationError {
        DescribeAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssociationError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssociationError::AssociationDoesNotExist(ref cause) => cause,
            DescribeAssociationError::InternalServerError(ref cause) => cause,
            DescribeAssociationError::InvalidAssociationVersion(ref cause) => cause,
            DescribeAssociationError::InvalidDocument(ref cause) => cause,
            DescribeAssociationError::InvalidInstanceId(ref cause) => cause,
            DescribeAssociationError::Validation(ref cause) => cause,
            DescribeAssociationError::Credentials(ref err) => err.description(),
            DescribeAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAssociationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutomationExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeAutomationExecutionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAutomationExecutionsError {
    pub fn from_body(body: &str) -> DescribeAutomationExecutionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeAutomationExecutionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidFilterKey" => DescribeAutomationExecutionsError::InvalidFilterKey(
                        String::from(error_message),
                    ),
                    "InvalidFilterValue" => DescribeAutomationExecutionsError::InvalidFilterValue(
                        String::from(error_message),
                    ),
                    "InvalidNextToken" => DescribeAutomationExecutionsError::InvalidNextToken(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeAutomationExecutionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAutomationExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAutomationExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAutomationExecutionsError {
    fn from(err: serde_json::error::Error) -> DescribeAutomationExecutionsError {
        DescribeAutomationExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAutomationExecutionsError {
    fn from(err: CredentialsError) -> DescribeAutomationExecutionsError {
        DescribeAutomationExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAutomationExecutionsError {
    fn from(err: HttpDispatchError) -> DescribeAutomationExecutionsError {
        DescribeAutomationExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAutomationExecutionsError {
    fn from(err: io::Error) -> DescribeAutomationExecutionsError {
        DescribeAutomationExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAutomationExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutomationExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutomationExecutionsError::InternalServerError(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidFilterKey(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidFilterValue(ref cause) => cause,
            DescribeAutomationExecutionsError::InvalidNextToken(ref cause) => cause,
            DescribeAutomationExecutionsError::Validation(ref cause) => cause,
            DescribeAutomationExecutionsError::Credentials(ref err) => err.description(),
            DescribeAutomationExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAutomationExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAutomationStepExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeAutomationStepExecutionsError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAutomationStepExecutionsError {
    pub fn from_body(body: &str) -> DescribeAutomationStepExecutionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AutomationExecutionNotFoundException" => {
                        DescribeAutomationStepExecutionsError::AutomationExecutionNotFound(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        DescribeAutomationStepExecutionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidFilterKey" => DescribeAutomationStepExecutionsError::InvalidFilterKey(
                        String::from(error_message),
                    ),
                    "InvalidFilterValue" => {
                        DescribeAutomationStepExecutionsError::InvalidFilterValue(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextToken" => DescribeAutomationStepExecutionsError::InvalidNextToken(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeAutomationStepExecutionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAutomationStepExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAutomationStepExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAutomationStepExecutionsError {
    fn from(err: serde_json::error::Error) -> DescribeAutomationStepExecutionsError {
        DescribeAutomationStepExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAutomationStepExecutionsError {
    fn from(err: CredentialsError) -> DescribeAutomationStepExecutionsError {
        DescribeAutomationStepExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAutomationStepExecutionsError {
    fn from(err: HttpDispatchError) -> DescribeAutomationStepExecutionsError {
        DescribeAutomationStepExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAutomationStepExecutionsError {
    fn from(err: io::Error) -> DescribeAutomationStepExecutionsError {
        DescribeAutomationStepExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAutomationStepExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAutomationStepExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAutomationStepExecutionsError::AutomationExecutionNotFound(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InternalServerError(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidFilterKey(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidFilterValue(ref cause) => cause,
            DescribeAutomationStepExecutionsError::InvalidNextToken(ref cause) => cause,
            DescribeAutomationStepExecutionsError::Validation(ref cause) => cause,
            DescribeAutomationStepExecutionsError::Credentials(ref err) => err.description(),
            DescribeAutomationStepExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAutomationStepExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAvailablePatches
#[derive(Debug, PartialEq)]
pub enum DescribeAvailablePatchesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAvailablePatchesError {
    pub fn from_body(body: &str) -> DescribeAvailablePatchesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeAvailablePatchesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeAvailablePatchesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAvailablePatchesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAvailablePatchesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAvailablePatchesError {
    fn from(err: serde_json::error::Error) -> DescribeAvailablePatchesError {
        DescribeAvailablePatchesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAvailablePatchesError {
    fn from(err: CredentialsError) -> DescribeAvailablePatchesError {
        DescribeAvailablePatchesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAvailablePatchesError {
    fn from(err: HttpDispatchError) -> DescribeAvailablePatchesError {
        DescribeAvailablePatchesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAvailablePatchesError {
    fn from(err: io::Error) -> DescribeAvailablePatchesError {
        DescribeAvailablePatchesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAvailablePatchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAvailablePatchesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAvailablePatchesError::InternalServerError(ref cause) => cause,
            DescribeAvailablePatchesError::Validation(ref cause) => cause,
            DescribeAvailablePatchesError::Credentials(ref err) => err.description(),
            DescribeAvailablePatchesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAvailablePatchesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDocument
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDocumentError {
    pub fn from_body(body: &str) -> DescribeDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeDocumentError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        DescribeDocumentError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentVersion" => {
                        DescribeDocumentError::InvalidDocumentVersion(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDocumentError::Validation(error_message.to_string())
                    }
                    _ => DescribeDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDocumentError {
    fn from(err: serde_json::error::Error) -> DescribeDocumentError {
        DescribeDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDocumentError {
    fn from(err: CredentialsError) -> DescribeDocumentError {
        DescribeDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDocumentError {
    fn from(err: HttpDispatchError) -> DescribeDocumentError {
        DescribeDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDocumentError {
    fn from(err: io::Error) -> DescribeDocumentError {
        DescribeDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentError::InternalServerError(ref cause) => cause,
            DescribeDocumentError::InvalidDocument(ref cause) => cause,
            DescribeDocumentError::InvalidDocumentVersion(ref cause) => cause,
            DescribeDocumentError::Validation(ref cause) => cause,
            DescribeDocumentError::Credentials(ref err) => err.description(),
            DescribeDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDocumentPermission
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentPermissionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
    InvalidPermissionType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDocumentPermissionError {
    pub fn from_body(body: &str) -> DescribeDocumentPermissionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeDocumentPermissionError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidDocument" => DescribeDocumentPermissionError::InvalidDocument(
                        String::from(error_message),
                    ),
                    "InvalidPermissionType" => {
                        DescribeDocumentPermissionError::InvalidPermissionType(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeDocumentPermissionError::Validation(error_message.to_string())
                    }
                    _ => DescribeDocumentPermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDocumentPermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDocumentPermissionError {
    fn from(err: serde_json::error::Error) -> DescribeDocumentPermissionError {
        DescribeDocumentPermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDocumentPermissionError {
    fn from(err: CredentialsError) -> DescribeDocumentPermissionError {
        DescribeDocumentPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDocumentPermissionError {
    fn from(err: HttpDispatchError) -> DescribeDocumentPermissionError {
        DescribeDocumentPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDocumentPermissionError {
    fn from(err: io::Error) -> DescribeDocumentPermissionError {
        DescribeDocumentPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDocumentPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentPermissionError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentPermissionError::InternalServerError(ref cause) => cause,
            DescribeDocumentPermissionError::InvalidDocument(ref cause) => cause,
            DescribeDocumentPermissionError::InvalidPermissionType(ref cause) => cause,
            DescribeDocumentPermissionError::Validation(ref cause) => cause,
            DescribeDocumentPermissionError::Credentials(ref err) => err.description(),
            DescribeDocumentPermissionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDocumentPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEffectiveInstanceAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeEffectiveInstanceAssociationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEffectiveInstanceAssociationsError {
    pub fn from_body(body: &str) -> DescribeEffectiveInstanceAssociationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeEffectiveInstanceAssociationsError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "InvalidInstanceId" => {
                        DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextToken" => {
                        DescribeEffectiveInstanceAssociationsError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeEffectiveInstanceAssociationsError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeEffectiveInstanceAssociationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEffectiveInstanceAssociationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEffectiveInstanceAssociationsError {
    fn from(err: serde_json::error::Error) -> DescribeEffectiveInstanceAssociationsError {
        DescribeEffectiveInstanceAssociationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEffectiveInstanceAssociationsError {
    fn from(err: CredentialsError) -> DescribeEffectiveInstanceAssociationsError {
        DescribeEffectiveInstanceAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEffectiveInstanceAssociationsError {
    fn from(err: HttpDispatchError) -> DescribeEffectiveInstanceAssociationsError {
        DescribeEffectiveInstanceAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEffectiveInstanceAssociationsError {
    fn from(err: io::Error) -> DescribeEffectiveInstanceAssociationsError {
        DescribeEffectiveInstanceAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEffectiveInstanceAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEffectiveInstanceAssociationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEffectiveInstanceAssociationsError::InternalServerError(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::InvalidNextToken(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::Validation(ref cause) => cause,
            DescribeEffectiveInstanceAssociationsError::Credentials(ref err) => err.description(),
            DescribeEffectiveInstanceAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEffectiveInstanceAssociationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEffectivePatchesForPatchBaseline
#[derive(Debug, PartialEq)]
pub enum DescribeEffectivePatchesForPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The operating systems you specified is not supported, or the operation is not supported for the operating system. Valid operating systems include: Windows, AmazonLinux, RedhatEnterpriseLinux, and Ubuntu.</p>
    UnsupportedOperatingSystem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEffectivePatchesForPatchBaselineError {
    pub fn from_body(body: &str) -> DescribeEffectivePatchesForPatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        DescribeEffectivePatchesForPatchBaselineError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "InvalidResourceId" => {
                        DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(
                            String::from(error_message),
                        )
                    }
                    "UnsupportedOperatingSystem" => {
                        DescribeEffectivePatchesForPatchBaselineError::UnsupportedOperatingSystem(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeEffectivePatchesForPatchBaselineError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeEffectivePatchesForPatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEffectivePatchesForPatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEffectivePatchesForPatchBaselineError {
    fn from(err: serde_json::error::Error) -> DescribeEffectivePatchesForPatchBaselineError {
        DescribeEffectivePatchesForPatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEffectivePatchesForPatchBaselineError {
    fn from(err: CredentialsError) -> DescribeEffectivePatchesForPatchBaselineError {
        DescribeEffectivePatchesForPatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEffectivePatchesForPatchBaselineError {
    fn from(err: HttpDispatchError) -> DescribeEffectivePatchesForPatchBaselineError {
        DescribeEffectivePatchesForPatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEffectivePatchesForPatchBaselineError {
    fn from(err: io::Error) -> DescribeEffectivePatchesForPatchBaselineError {
        DescribeEffectivePatchesForPatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEffectivePatchesForPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEffectivePatchesForPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::InternalServerError(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::UnsupportedOperatingSystem(
                ref cause,
            ) => cause,
            DescribeEffectivePatchesForPatchBaselineError::Validation(ref cause) => cause,
            DescribeEffectivePatchesForPatchBaselineError::Credentials(ref err) => {
                err.description()
            }
            DescribeEffectivePatchesForPatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEffectivePatchesForPatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstanceAssociationsStatus
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceAssociationsStatusError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstanceAssociationsStatusError {
    pub fn from_body(body: &str) -> DescribeInstanceAssociationsStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeInstanceAssociationsStatusError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidInstanceId" => {
                        DescribeInstanceAssociationsStatusError::InvalidInstanceId(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextToken" => {
                        DescribeInstanceAssociationsStatusError::InvalidNextToken(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeInstanceAssociationsStatusError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeInstanceAssociationsStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstanceAssociationsStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstanceAssociationsStatusError {
    fn from(err: serde_json::error::Error) -> DescribeInstanceAssociationsStatusError {
        DescribeInstanceAssociationsStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstanceAssociationsStatusError {
    fn from(err: CredentialsError) -> DescribeInstanceAssociationsStatusError {
        DescribeInstanceAssociationsStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstanceAssociationsStatusError {
    fn from(err: HttpDispatchError) -> DescribeInstanceAssociationsStatusError {
        DescribeInstanceAssociationsStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstanceAssociationsStatusError {
    fn from(err: io::Error) -> DescribeInstanceAssociationsStatusError {
        DescribeInstanceAssociationsStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstanceAssociationsStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstanceAssociationsStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstanceAssociationsStatusError::InternalServerError(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::InvalidInstanceId(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::InvalidNextToken(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::Validation(ref cause) => cause,
            DescribeInstanceAssociationsStatusError::Credentials(ref err) => err.description(),
            DescribeInstanceAssociationsStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstanceAssociationsStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstanceInformation
#[derive(Debug, PartialEq)]
pub enum DescribeInstanceInformationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified filter value is not valid.</p>
    InvalidInstanceInformationFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstanceInformationError {
    pub fn from_body(body: &str) -> DescribeInstanceInformationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeInstanceInformationError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidFilterKey" => DescribeInstanceInformationError::InvalidFilterKey(
                        String::from(error_message),
                    ),
                    "InvalidInstanceId" => DescribeInstanceInformationError::InvalidInstanceId(
                        String::from(error_message),
                    ),
                    "InvalidInstanceInformationFilterValue" => {
                        DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(
                            String::from(error_message),
                        )
                    }
                    "InvalidNextToken" => DescribeInstanceInformationError::InvalidNextToken(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeInstanceInformationError::Validation(error_message.to_string())
                    }
                    _ => DescribeInstanceInformationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstanceInformationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstanceInformationError {
    fn from(err: serde_json::error::Error) -> DescribeInstanceInformationError {
        DescribeInstanceInformationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstanceInformationError {
    fn from(err: CredentialsError) -> DescribeInstanceInformationError {
        DescribeInstanceInformationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstanceInformationError {
    fn from(err: HttpDispatchError) -> DescribeInstanceInformationError {
        DescribeInstanceInformationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstanceInformationError {
    fn from(err: io::Error) -> DescribeInstanceInformationError {
        DescribeInstanceInformationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstanceInformationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstanceInformationError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstanceInformationError::InternalServerError(ref cause) => cause,
            DescribeInstanceInformationError::InvalidFilterKey(ref cause) => cause,
            DescribeInstanceInformationError::InvalidInstanceId(ref cause) => cause,
            DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(ref cause) => {
                cause
            }
            DescribeInstanceInformationError::InvalidNextToken(ref cause) => cause,
            DescribeInstanceInformationError::Validation(ref cause) => cause,
            DescribeInstanceInformationError::Credentials(ref err) => err.description(),
            DescribeInstanceInformationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstanceInformationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatchStates
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchStatesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstancePatchStatesError {
    pub fn from_body(body: &str) -> DescribeInstancePatchStatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeInstancePatchStatesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidNextToken" => DescribeInstancePatchStatesError::InvalidNextToken(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeInstancePatchStatesError::Validation(error_message.to_string())
                    }
                    _ => DescribeInstancePatchStatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstancePatchStatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstancePatchStatesError {
    fn from(err: serde_json::error::Error) -> DescribeInstancePatchStatesError {
        DescribeInstancePatchStatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstancePatchStatesError {
    fn from(err: CredentialsError) -> DescribeInstancePatchStatesError {
        DescribeInstancePatchStatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancePatchStatesError {
    fn from(err: HttpDispatchError) -> DescribeInstancePatchStatesError {
        DescribeInstancePatchStatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstancePatchStatesError {
    fn from(err: io::Error) -> DescribeInstancePatchStatesError {
        DescribeInstancePatchStatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstancePatchStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchStatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchStatesError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchStatesError::InvalidNextToken(ref cause) => cause,
            DescribeInstancePatchStatesError::Validation(ref cause) => cause,
            DescribeInstancePatchStatesError::Credentials(ref err) => err.description(),
            DescribeInstancePatchStatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancePatchStatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatchStatesForPatchGroup
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchStatesForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstancePatchStatesForPatchGroupError {
    pub fn from_body(body: &str) -> DescribeInstancePatchStatesForPatchGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeInstancePatchStatesForPatchGroupError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "InvalidFilter" => {
                        DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(String::from(
                            error_message,
                        ))
                    }
                    "InvalidNextToken" => {
                        DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeInstancePatchStatesForPatchGroupError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeInstancePatchStatesForPatchGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstancePatchStatesForPatchGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstancePatchStatesForPatchGroupError {
    fn from(err: serde_json::error::Error) -> DescribeInstancePatchStatesForPatchGroupError {
        DescribeInstancePatchStatesForPatchGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstancePatchStatesForPatchGroupError {
    fn from(err: CredentialsError) -> DescribeInstancePatchStatesForPatchGroupError {
        DescribeInstancePatchStatesForPatchGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancePatchStatesForPatchGroupError {
    fn from(err: HttpDispatchError) -> DescribeInstancePatchStatesForPatchGroupError {
        DescribeInstancePatchStatesForPatchGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstancePatchStatesForPatchGroupError {
    fn from(err: io::Error) -> DescribeInstancePatchStatesForPatchGroupError {
        DescribeInstancePatchStatesForPatchGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstancePatchStatesForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchStatesForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchStatesForPatchGroupError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::Validation(ref cause) => cause,
            DescribeInstancePatchStatesForPatchGroupError::Credentials(ref err) => {
                err.description()
            }
            DescribeInstancePatchStatesForPatchGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancePatchStatesForPatchGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeInstancePatches
#[derive(Debug, PartialEq)]
pub enum DescribeInstancePatchesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeInstancePatchesError {
    pub fn from_body(body: &str) -> DescribeInstancePatchesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeInstancePatchesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidFilter" => {
                        DescribeInstancePatchesError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        DescribeInstancePatchesError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        DescribeInstancePatchesError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeInstancePatchesError::Validation(error_message.to_string())
                    }
                    _ => DescribeInstancePatchesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeInstancePatchesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeInstancePatchesError {
    fn from(err: serde_json::error::Error) -> DescribeInstancePatchesError {
        DescribeInstancePatchesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInstancePatchesError {
    fn from(err: CredentialsError) -> DescribeInstancePatchesError {
        DescribeInstancePatchesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInstancePatchesError {
    fn from(err: HttpDispatchError) -> DescribeInstancePatchesError {
        DescribeInstancePatchesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInstancePatchesError {
    fn from(err: io::Error) -> DescribeInstancePatchesError {
        DescribeInstancePatchesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInstancePatchesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInstancePatchesError {
    fn description(&self) -> &str {
        match *self {
            DescribeInstancePatchesError::InternalServerError(ref cause) => cause,
            DescribeInstancePatchesError::InvalidFilter(ref cause) => cause,
            DescribeInstancePatchesError::InvalidInstanceId(ref cause) => cause,
            DescribeInstancePatchesError::InvalidNextToken(ref cause) => cause,
            DescribeInstancePatchesError::Validation(ref cause) => cause,
            DescribeInstancePatchesError::Credentials(ref err) => err.description(),
            DescribeInstancePatchesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInstancePatchesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutionTaskInvocations
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionTaskInvocationsError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowExecutionTaskInvocationsError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(
                        String::from(body),
                    ),
                }
            }
            Err(_) => {
                DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(String::from(body))
            }
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn from(
        err: serde_json::error::Error,
    ) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
        DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(
            err.description().to_string(),
        )
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
        DescribeMaintenanceWindowExecutionTaskInvocationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
        DescribeMaintenanceWindowExecutionTaskInvocationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
        DescribeMaintenanceWindowExecutionTaskInvocationsError::HttpDispatch(
            HttpDispatchError::from(err),
        )
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionTaskInvocationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(ref cause) => {
                cause
            }
            DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(
                ref cause,
            ) => cause,
            DescribeMaintenanceWindowExecutionTaskInvocationsError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowExecutionTaskInvocationsError::Credentials(ref err) => {
                err.description()
            }
            DescribeMaintenanceWindowExecutionTaskInvocationsError::HttpDispatch(
                ref dispatch_error,
            ) => dispatch_error.description(),
            DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutionTasks
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionTasksError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowExecutionTasksError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        DescribeMaintenanceWindowExecutionTasksError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DescribeMaintenanceWindowExecutionTasksError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DescribeMaintenanceWindowExecutionTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceWindowExecutionTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionTasksError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowExecutionTasksError {
        DescribeMaintenanceWindowExecutionTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowExecutionTasksError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionTasksError {
        DescribeMaintenanceWindowExecutionTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionTasksError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionTasksError {
        DescribeMaintenanceWindowExecutionTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowExecutionTasksError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowExecutionTasksError {
        DescribeMaintenanceWindowExecutionTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowExecutionTasksError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceWindowExecutionTasksError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowExecutionTasksError::Credentials(ref err) => err.description(),
            DescribeMaintenanceWindowExecutionTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceWindowExecutionTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowExecutions
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowExecutionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowExecutionsError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeMaintenanceWindowExecutionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DescribeMaintenanceWindowExecutionsError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DescribeMaintenanceWindowExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceWindowExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionsError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowExecutionsError {
        DescribeMaintenanceWindowExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowExecutionsError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionsError {
        DescribeMaintenanceWindowExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionsError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionsError {
        DescribeMaintenanceWindowExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowExecutionsError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowExecutionsError {
        DescribeMaintenanceWindowExecutionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceWindowExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowExecutionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowExecutionsError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceWindowExecutionsError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowExecutionsError::Credentials(ref err) => err.description(),
            DescribeMaintenanceWindowExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceWindowExecutionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowTargets
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowTargetsError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowTargetsError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowTargetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => DescribeMaintenanceWindowTargetsError::DoesNotExist(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        DescribeMaintenanceWindowTargetsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeMaintenanceWindowTargetsError::Validation(error_message.to_string())
                    }
                    _ => DescribeMaintenanceWindowTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceWindowTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowTargetsError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowTargetsError {
        DescribeMaintenanceWindowTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowTargetsError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowTargetsError {
        DescribeMaintenanceWindowTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowTargetsError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowTargetsError {
        DescribeMaintenanceWindowTargetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowTargetsError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowTargetsError {
        DescribeMaintenanceWindowTargetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceWindowTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowTargetsError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowTargetsError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceWindowTargetsError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowTargetsError::Credentials(ref err) => err.description(),
            DescribeMaintenanceWindowTargetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceWindowTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindowTasks
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowTasksError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowTasksError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => DescribeMaintenanceWindowTasksError::DoesNotExist(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        DescribeMaintenanceWindowTasksError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeMaintenanceWindowTasksError::Validation(error_message.to_string())
                    }
                    _ => DescribeMaintenanceWindowTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceWindowTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowTasksError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowTasksError {
        DescribeMaintenanceWindowTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowTasksError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowTasksError {
        DescribeMaintenanceWindowTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowTasksError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowTasksError {
        DescribeMaintenanceWindowTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowTasksError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowTasksError {
        DescribeMaintenanceWindowTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceWindowTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowTasksError::DoesNotExist(ref cause) => cause,
            DescribeMaintenanceWindowTasksError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceWindowTasksError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowTasksError::Credentials(ref err) => err.description(),
            DescribeMaintenanceWindowTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceWindowTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMaintenanceWindows
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceWindowsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMaintenanceWindowsError {
    pub fn from_body(body: &str) -> DescribeMaintenanceWindowsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeMaintenanceWindowsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeMaintenanceWindowsError::Validation(error_message.to_string())
                    }
                    _ => DescribeMaintenanceWindowsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMaintenanceWindowsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMaintenanceWindowsError {
    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowsError {
        DescribeMaintenanceWindowsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMaintenanceWindowsError {
    fn from(err: CredentialsError) -> DescribeMaintenanceWindowsError {
        DescribeMaintenanceWindowsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMaintenanceWindowsError {
    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowsError {
        DescribeMaintenanceWindowsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMaintenanceWindowsError {
    fn from(err: io::Error) -> DescribeMaintenanceWindowsError {
        DescribeMaintenanceWindowsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMaintenanceWindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMaintenanceWindowsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMaintenanceWindowsError::InternalServerError(ref cause) => cause,
            DescribeMaintenanceWindowsError::Validation(ref cause) => cause,
            DescribeMaintenanceWindowsError::Credentials(ref err) => err.description(),
            DescribeMaintenanceWindowsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeMaintenanceWindowsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeParameters
#[derive(Debug, PartialEq)]
pub enum DescribeParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified filter option is not valid. Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    InvalidFilterOption(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeParametersError {
    pub fn from_body(body: &str) -> DescribeParametersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeParametersError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilterKey" => {
                        DescribeParametersError::InvalidFilterKey(String::from(error_message))
                    }
                    "InvalidFilterOption" => {
                        DescribeParametersError::InvalidFilterOption(String::from(error_message))
                    }
                    "InvalidFilterValue" => {
                        DescribeParametersError::InvalidFilterValue(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        DescribeParametersError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeParametersError::Validation(error_message.to_string())
                    }
                    _ => DescribeParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeParametersError {
    fn from(err: serde_json::error::Error) -> DescribeParametersError {
        DescribeParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeParametersError {
    fn from(err: CredentialsError) -> DescribeParametersError {
        DescribeParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeParametersError {
    fn from(err: HttpDispatchError) -> DescribeParametersError {
        DescribeParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeParametersError {
    fn from(err: io::Error) -> DescribeParametersError {
        DescribeParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeParametersError::InternalServerError(ref cause) => cause,
            DescribeParametersError::InvalidFilterKey(ref cause) => cause,
            DescribeParametersError::InvalidFilterOption(ref cause) => cause,
            DescribeParametersError::InvalidFilterValue(ref cause) => cause,
            DescribeParametersError::InvalidNextToken(ref cause) => cause,
            DescribeParametersError::Validation(ref cause) => cause,
            DescribeParametersError::Credentials(ref err) => err.description(),
            DescribeParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchBaselines
#[derive(Debug, PartialEq)]
pub enum DescribePatchBaselinesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePatchBaselinesError {
    pub fn from_body(body: &str) -> DescribePatchBaselinesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribePatchBaselinesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribePatchBaselinesError::Validation(error_message.to_string())
                    }
                    _ => DescribePatchBaselinesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePatchBaselinesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePatchBaselinesError {
    fn from(err: serde_json::error::Error) -> DescribePatchBaselinesError {
        DescribePatchBaselinesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePatchBaselinesError {
    fn from(err: CredentialsError) -> DescribePatchBaselinesError {
        DescribePatchBaselinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePatchBaselinesError {
    fn from(err: HttpDispatchError) -> DescribePatchBaselinesError {
        DescribePatchBaselinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePatchBaselinesError {
    fn from(err: io::Error) -> DescribePatchBaselinesError {
        DescribePatchBaselinesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePatchBaselinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchBaselinesError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchBaselinesError::InternalServerError(ref cause) => cause,
            DescribePatchBaselinesError::Validation(ref cause) => cause,
            DescribePatchBaselinesError::Credentials(ref err) => err.description(),
            DescribePatchBaselinesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePatchBaselinesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchGroupState
#[derive(Debug, PartialEq)]
pub enum DescribePatchGroupStateError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePatchGroupStateError {
    pub fn from_body(body: &str) -> DescribePatchGroupStateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribePatchGroupStateError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidNextToken" => {
                        DescribePatchGroupStateError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePatchGroupStateError::Validation(error_message.to_string())
                    }
                    _ => DescribePatchGroupStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePatchGroupStateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePatchGroupStateError {
    fn from(err: serde_json::error::Error) -> DescribePatchGroupStateError {
        DescribePatchGroupStateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePatchGroupStateError {
    fn from(err: CredentialsError) -> DescribePatchGroupStateError {
        DescribePatchGroupStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePatchGroupStateError {
    fn from(err: HttpDispatchError) -> DescribePatchGroupStateError {
        DescribePatchGroupStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePatchGroupStateError {
    fn from(err: io::Error) -> DescribePatchGroupStateError {
        DescribePatchGroupStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePatchGroupStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchGroupStateError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchGroupStateError::InternalServerError(ref cause) => cause,
            DescribePatchGroupStateError::InvalidNextToken(ref cause) => cause,
            DescribePatchGroupStateError::Validation(ref cause) => cause,
            DescribePatchGroupStateError::Credentials(ref err) => err.description(),
            DescribePatchGroupStateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePatchGroupStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePatchGroups
#[derive(Debug, PartialEq)]
pub enum DescribePatchGroupsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePatchGroupsError {
    pub fn from_body(body: &str) -> DescribePatchGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribePatchGroupsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePatchGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribePatchGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePatchGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePatchGroupsError {
    fn from(err: serde_json::error::Error) -> DescribePatchGroupsError {
        DescribePatchGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePatchGroupsError {
    fn from(err: CredentialsError) -> DescribePatchGroupsError {
        DescribePatchGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePatchGroupsError {
    fn from(err: HttpDispatchError) -> DescribePatchGroupsError {
        DescribePatchGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePatchGroupsError {
    fn from(err: io::Error) -> DescribePatchGroupsError {
        DescribePatchGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePatchGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePatchGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribePatchGroupsError::InternalServerError(ref cause) => cause,
            DescribePatchGroupsError::Validation(ref cause) => cause,
            DescribePatchGroupsError::Credentials(ref err) => err.description(),
            DescribePatchGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePatchGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAutomationExecution
#[derive(Debug, PartialEq)]
pub enum GetAutomationExecutionError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAutomationExecutionError {
    pub fn from_body(body: &str) -> GetAutomationExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AutomationExecutionNotFoundException" => {
                        GetAutomationExecutionError::AutomationExecutionNotFound(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => GetAutomationExecutionError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetAutomationExecutionError::Validation(error_message.to_string())
                    }
                    _ => GetAutomationExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAutomationExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAutomationExecutionError {
    fn from(err: serde_json::error::Error) -> GetAutomationExecutionError {
        GetAutomationExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAutomationExecutionError {
    fn from(err: CredentialsError) -> GetAutomationExecutionError {
        GetAutomationExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAutomationExecutionError {
    fn from(err: HttpDispatchError) -> GetAutomationExecutionError {
        GetAutomationExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAutomationExecutionError {
    fn from(err: io::Error) -> GetAutomationExecutionError {
        GetAutomationExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,
            GetAutomationExecutionError::InternalServerError(ref cause) => cause,
            GetAutomationExecutionError::Validation(ref cause) => cause,
            GetAutomationExecutionError::Credentials(ref err) => err.description(),
            GetAutomationExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAutomationExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCommandInvocation
#[derive(Debug, PartialEq)]
pub enum GetCommandInvocationError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The plugin name is not valid.</p>
    InvalidPluginName(String),
    /// <p>The command ID and instance ID you specified did not match any invocations. Verify the command ID adn the instance ID and try again. </p>
    InvocationDoesNotExist(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetCommandInvocationError {
    pub fn from_body(body: &str) -> GetCommandInvocationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetCommandInvocationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidCommandId" => {
                        GetCommandInvocationError::InvalidCommandId(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        GetCommandInvocationError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidPluginName" => {
                        GetCommandInvocationError::InvalidPluginName(String::from(error_message))
                    }
                    "InvocationDoesNotExist" => GetCommandInvocationError::InvocationDoesNotExist(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetCommandInvocationError::Validation(error_message.to_string())
                    }
                    _ => GetCommandInvocationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCommandInvocationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCommandInvocationError {
    fn from(err: serde_json::error::Error) -> GetCommandInvocationError {
        GetCommandInvocationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCommandInvocationError {
    fn from(err: CredentialsError) -> GetCommandInvocationError {
        GetCommandInvocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCommandInvocationError {
    fn from(err: HttpDispatchError) -> GetCommandInvocationError {
        GetCommandInvocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCommandInvocationError {
    fn from(err: io::Error) -> GetCommandInvocationError {
        GetCommandInvocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCommandInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCommandInvocationError {
    fn description(&self) -> &str {
        match *self {
            GetCommandInvocationError::InternalServerError(ref cause) => cause,
            GetCommandInvocationError::InvalidCommandId(ref cause) => cause,
            GetCommandInvocationError::InvalidInstanceId(ref cause) => cause,
            GetCommandInvocationError::InvalidPluginName(ref cause) => cause,
            GetCommandInvocationError::InvocationDoesNotExist(ref cause) => cause,
            GetCommandInvocationError::Validation(ref cause) => cause,
            GetCommandInvocationError::Credentials(ref err) => err.description(),
            GetCommandInvocationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetCommandInvocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDefaultPatchBaseline
#[derive(Debug, PartialEq)]
pub enum GetDefaultPatchBaselineError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDefaultPatchBaselineError {
    pub fn from_body(body: &str) -> GetDefaultPatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => GetDefaultPatchBaselineError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetDefaultPatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => GetDefaultPatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDefaultPatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDefaultPatchBaselineError {
    fn from(err: serde_json::error::Error) -> GetDefaultPatchBaselineError {
        GetDefaultPatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDefaultPatchBaselineError {
    fn from(err: CredentialsError) -> GetDefaultPatchBaselineError {
        GetDefaultPatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDefaultPatchBaselineError {
    fn from(err: HttpDispatchError) -> GetDefaultPatchBaselineError {
        GetDefaultPatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDefaultPatchBaselineError {
    fn from(err: io::Error) -> GetDefaultPatchBaselineError {
        GetDefaultPatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDefaultPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDefaultPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            GetDefaultPatchBaselineError::InternalServerError(ref cause) => cause,
            GetDefaultPatchBaselineError::Validation(ref cause) => cause,
            GetDefaultPatchBaselineError::Credentials(ref err) => err.description(),
            GetDefaultPatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDefaultPatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployablePatchSnapshotForInstance
#[derive(Debug, PartialEq)]
pub enum GetDeployablePatchSnapshotForInstanceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operating systems you specified is not supported, or the operation is not supported for the operating system. Valid operating systems include: Windows, AmazonLinux, RedhatEnterpriseLinux, and Ubuntu.</p>
    UnsupportedOperatingSystem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeployablePatchSnapshotForInstanceError {
    pub fn from_body(body: &str) -> GetDeployablePatchSnapshotForInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetDeployablePatchSnapshotForInstanceError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "UnsupportedOperatingSystem" => {
                        GetDeployablePatchSnapshotForInstanceError::UnsupportedOperatingSystem(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        GetDeployablePatchSnapshotForInstanceError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => GetDeployablePatchSnapshotForInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeployablePatchSnapshotForInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeployablePatchSnapshotForInstanceError {
    fn from(err: serde_json::error::Error) -> GetDeployablePatchSnapshotForInstanceError {
        GetDeployablePatchSnapshotForInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeployablePatchSnapshotForInstanceError {
    fn from(err: CredentialsError) -> GetDeployablePatchSnapshotForInstanceError {
        GetDeployablePatchSnapshotForInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeployablePatchSnapshotForInstanceError {
    fn from(err: HttpDispatchError) -> GetDeployablePatchSnapshotForInstanceError {
        GetDeployablePatchSnapshotForInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeployablePatchSnapshotForInstanceError {
    fn from(err: io::Error) -> GetDeployablePatchSnapshotForInstanceError {
        GetDeployablePatchSnapshotForInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeployablePatchSnapshotForInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeployablePatchSnapshotForInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetDeployablePatchSnapshotForInstanceError::InternalServerError(ref cause) => cause,
            GetDeployablePatchSnapshotForInstanceError::UnsupportedOperatingSystem(ref cause) => {
                cause
            }
            GetDeployablePatchSnapshotForInstanceError::Validation(ref cause) => cause,
            GetDeployablePatchSnapshotForInstanceError::Credentials(ref err) => err.description(),
            GetDeployablePatchSnapshotForInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeployablePatchSnapshotForInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocument
#[derive(Debug, PartialEq)]
pub enum GetDocumentError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDocumentError {
    pub fn from_body(body: &str) -> GetDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetDocumentError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        GetDocumentError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentVersion" => {
                        GetDocumentError::InvalidDocumentVersion(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDocumentError::Validation(error_message.to_string())
                    }
                    _ => GetDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDocumentError {
    fn from(err: serde_json::error::Error) -> GetDocumentError {
        GetDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDocumentError {
    fn from(err: CredentialsError) -> GetDocumentError {
        GetDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDocumentError {
    fn from(err: HttpDispatchError) -> GetDocumentError {
        GetDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDocumentError {
    fn from(err: io::Error) -> GetDocumentError {
        GetDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentError::InternalServerError(ref cause) => cause,
            GetDocumentError::InvalidDocument(ref cause) => cause,
            GetDocumentError::InvalidDocumentVersion(ref cause) => cause,
            GetDocumentError::Validation(ref cause) => cause,
            GetDocumentError::Credentials(ref err) => err.description(),
            GetDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInventory
#[derive(Debug, PartialEq)]
pub enum GetInventoryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The specified inventory item result attribute is not valid.</p>
    InvalidResultAttribute(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInventoryError {
    pub fn from_body(body: &str) -> GetInventoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetInventoryError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilter" => {
                        GetInventoryError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        GetInventoryError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidResultAttributeException" => {
                        GetInventoryError::InvalidResultAttribute(String::from(error_message))
                    }
                    "InvalidTypeNameException" => {
                        GetInventoryError::InvalidTypeName(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInventoryError::Validation(error_message.to_string())
                    }
                    _ => GetInventoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInventoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInventoryError {
    fn from(err: serde_json::error::Error) -> GetInventoryError {
        GetInventoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInventoryError {
    fn from(err: CredentialsError) -> GetInventoryError {
        GetInventoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInventoryError {
    fn from(err: HttpDispatchError) -> GetInventoryError {
        GetInventoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInventoryError {
    fn from(err: io::Error) -> GetInventoryError {
        GetInventoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInventoryError {
    fn description(&self) -> &str {
        match *self {
            GetInventoryError::InternalServerError(ref cause) => cause,
            GetInventoryError::InvalidFilter(ref cause) => cause,
            GetInventoryError::InvalidNextToken(ref cause) => cause,
            GetInventoryError::InvalidResultAttribute(ref cause) => cause,
            GetInventoryError::InvalidTypeName(ref cause) => cause,
            GetInventoryError::Validation(ref cause) => cause,
            GetInventoryError::Credentials(ref err) => err.description(),
            GetInventoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetInventoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInventorySchema
#[derive(Debug, PartialEq)]
pub enum GetInventorySchemaError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInventorySchemaError {
    pub fn from_body(body: &str) -> GetInventorySchemaError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetInventorySchemaError::InternalServerError(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        GetInventorySchemaError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidTypeNameException" => {
                        GetInventorySchemaError::InvalidTypeName(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInventorySchemaError::Validation(error_message.to_string())
                    }
                    _ => GetInventorySchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInventorySchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInventorySchemaError {
    fn from(err: serde_json::error::Error) -> GetInventorySchemaError {
        GetInventorySchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInventorySchemaError {
    fn from(err: CredentialsError) -> GetInventorySchemaError {
        GetInventorySchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInventorySchemaError {
    fn from(err: HttpDispatchError) -> GetInventorySchemaError {
        GetInventorySchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInventorySchemaError {
    fn from(err: io::Error) -> GetInventorySchemaError {
        GetInventorySchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInventorySchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInventorySchemaError {
    fn description(&self) -> &str {
        match *self {
            GetInventorySchemaError::InternalServerError(ref cause) => cause,
            GetInventorySchemaError::InvalidNextToken(ref cause) => cause,
            GetInventorySchemaError::InvalidTypeName(ref cause) => cause,
            GetInventorySchemaError::Validation(ref cause) => cause,
            GetInventorySchemaError::Credentials(ref err) => err.description(),
            GetInventorySchemaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInventorySchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMaintenanceWindowError {
    pub fn from_body(body: &str) -> GetMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        GetMaintenanceWindowError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => {
                        GetMaintenanceWindowError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetMaintenanceWindowError::Validation(error_message.to_string())
                    }
                    _ => GetMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowError {
        GetMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMaintenanceWindowError {
    fn from(err: CredentialsError) -> GetMaintenanceWindowError {
        GetMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> GetMaintenanceWindowError {
        GetMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMaintenanceWindowError {
    fn from(err: io::Error) -> GetMaintenanceWindowError {
        GetMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowError::InternalServerError(ref cause) => cause,
            GetMaintenanceWindowError::Validation(ref cause) => cause,
            GetMaintenanceWindowError::Credentials(ref err) => err.description(),
            GetMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecution
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMaintenanceWindowExecutionError {
    pub fn from_body(body: &str) -> GetMaintenanceWindowExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => GetMaintenanceWindowExecutionError::DoesNotExist(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        GetMaintenanceWindowExecutionError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetMaintenanceWindowExecutionError::Validation(error_message.to_string())
                    }
                    _ => GetMaintenanceWindowExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMaintenanceWindowExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMaintenanceWindowExecutionError {
    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowExecutionError {
        GetMaintenanceWindowExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMaintenanceWindowExecutionError {
    fn from(err: CredentialsError) -> GetMaintenanceWindowExecutionError {
        GetMaintenanceWindowExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMaintenanceWindowExecutionError {
    fn from(err: HttpDispatchError) -> GetMaintenanceWindowExecutionError {
        GetMaintenanceWindowExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMaintenanceWindowExecutionError {
    fn from(err: io::Error) -> GetMaintenanceWindowExecutionError {
        GetMaintenanceWindowExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionError::InternalServerError(ref cause) => cause,
            GetMaintenanceWindowExecutionError::Validation(ref cause) => cause,
            GetMaintenanceWindowExecutionError::Credentials(ref err) => err.description(),
            GetMaintenanceWindowExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMaintenanceWindowExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecutionTask
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMaintenanceWindowExecutionTaskError {
    pub fn from_body(body: &str) -> GetMaintenanceWindowExecutionTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        GetMaintenanceWindowExecutionTaskError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        GetMaintenanceWindowExecutionTaskError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => GetMaintenanceWindowExecutionTaskError::Validation(
                        error_message.to_string(),
                    ),
                    _ => GetMaintenanceWindowExecutionTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMaintenanceWindowExecutionTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMaintenanceWindowExecutionTaskError {
    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowExecutionTaskError {
        GetMaintenanceWindowExecutionTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMaintenanceWindowExecutionTaskError {
    fn from(err: CredentialsError) -> GetMaintenanceWindowExecutionTaskError {
        GetMaintenanceWindowExecutionTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMaintenanceWindowExecutionTaskError {
    fn from(err: HttpDispatchError) -> GetMaintenanceWindowExecutionTaskError {
        GetMaintenanceWindowExecutionTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMaintenanceWindowExecutionTaskError {
    fn from(err: io::Error) -> GetMaintenanceWindowExecutionTaskError {
        GetMaintenanceWindowExecutionTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionTaskError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionTaskError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskError::InternalServerError(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskError::Validation(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskError::Credentials(ref err) => err.description(),
            GetMaintenanceWindowExecutionTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMaintenanceWindowExecutionTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowExecutionTaskInvocation
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowExecutionTaskInvocationError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMaintenanceWindowExecutionTaskInvocationError {
    pub fn from_body(body: &str) -> GetMaintenanceWindowExecutionTaskInvocationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        GetMaintenanceWindowExecutionTaskInvocationError::DoesNotExist(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        GetMaintenanceWindowExecutionTaskInvocationError::InternalServerError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        GetMaintenanceWindowExecutionTaskInvocationError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => GetMaintenanceWindowExecutionTaskInvocationError::Unknown(String::from(
                        body,
                    )),
                }
            }
            Err(_) => GetMaintenanceWindowExecutionTaskInvocationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMaintenanceWindowExecutionTaskInvocationError {
    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowExecutionTaskInvocationError {
        GetMaintenanceWindowExecutionTaskInvocationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMaintenanceWindowExecutionTaskInvocationError {
    fn from(err: CredentialsError) -> GetMaintenanceWindowExecutionTaskInvocationError {
        GetMaintenanceWindowExecutionTaskInvocationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMaintenanceWindowExecutionTaskInvocationError {
    fn from(err: HttpDispatchError) -> GetMaintenanceWindowExecutionTaskInvocationError {
        GetMaintenanceWindowExecutionTaskInvocationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMaintenanceWindowExecutionTaskInvocationError {
    fn from(err: io::Error) -> GetMaintenanceWindowExecutionTaskInvocationError {
        GetMaintenanceWindowExecutionTaskInvocationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMaintenanceWindowExecutionTaskInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowExecutionTaskInvocationError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowExecutionTaskInvocationError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskInvocationError::InternalServerError(ref cause) => {
                cause
            }
            GetMaintenanceWindowExecutionTaskInvocationError::Validation(ref cause) => cause,
            GetMaintenanceWindowExecutionTaskInvocationError::Credentials(ref err) => {
                err.description()
            }
            GetMaintenanceWindowExecutionTaskInvocationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMaintenanceWindowExecutionTaskInvocationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMaintenanceWindowTask
#[derive(Debug, PartialEq)]
pub enum GetMaintenanceWindowTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMaintenanceWindowTaskError {
    pub fn from_body(body: &str) -> GetMaintenanceWindowTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        GetMaintenanceWindowTaskError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => GetMaintenanceWindowTaskError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        GetMaintenanceWindowTaskError::Validation(error_message.to_string())
                    }
                    _ => GetMaintenanceWindowTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMaintenanceWindowTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMaintenanceWindowTaskError {
    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowTaskError {
        GetMaintenanceWindowTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMaintenanceWindowTaskError {
    fn from(err: CredentialsError) -> GetMaintenanceWindowTaskError {
        GetMaintenanceWindowTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMaintenanceWindowTaskError {
    fn from(err: HttpDispatchError) -> GetMaintenanceWindowTaskError {
        GetMaintenanceWindowTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMaintenanceWindowTaskError {
    fn from(err: io::Error) -> GetMaintenanceWindowTaskError {
        GetMaintenanceWindowTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMaintenanceWindowTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMaintenanceWindowTaskError {
    fn description(&self) -> &str {
        match *self {
            GetMaintenanceWindowTaskError::DoesNotExist(ref cause) => cause,
            GetMaintenanceWindowTaskError::InternalServerError(ref cause) => cause,
            GetMaintenanceWindowTaskError::Validation(ref cause) => cause,
            GetMaintenanceWindowTaskError::Credentials(ref err) => err.description(),
            GetMaintenanceWindowTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetMaintenanceWindowTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameter
#[derive(Debug, PartialEq)]
pub enum GetParameterError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
    /// <p>The specified parameter version was not found. Verify the parameter name and version, and try again.</p>
    ParameterVersionNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetParameterError {
    pub fn from_body(body: &str) -> GetParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetParameterError::InternalServerError(String::from(error_message))
                    }
                    "InvalidKeyId" => GetParameterError::InvalidKeyId(String::from(error_message)),
                    "ParameterNotFound" => {
                        GetParameterError::ParameterNotFound(String::from(error_message))
                    }
                    "ParameterVersionNotFound" => {
                        GetParameterError::ParameterVersionNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetParameterError::Validation(error_message.to_string())
                    }
                    _ => GetParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetParameterError {
    fn from(err: serde_json::error::Error) -> GetParameterError {
        GetParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetParameterError {
    fn from(err: CredentialsError) -> GetParameterError {
        GetParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetParameterError {
    fn from(err: HttpDispatchError) -> GetParameterError {
        GetParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetParameterError {
    fn from(err: io::Error) -> GetParameterError {
        GetParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParameterError {
    fn description(&self) -> &str {
        match *self {
            GetParameterError::InternalServerError(ref cause) => cause,
            GetParameterError::InvalidKeyId(ref cause) => cause,
            GetParameterError::ParameterNotFound(ref cause) => cause,
            GetParameterError::ParameterVersionNotFound(ref cause) => cause,
            GetParameterError::Validation(ref cause) => cause,
            GetParameterError::Credentials(ref err) => err.description(),
            GetParameterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameterHistory
#[derive(Debug, PartialEq)]
pub enum GetParameterHistoryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter could not be found. Verify the name and try again.</p>
    ParameterNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetParameterHistoryError {
    pub fn from_body(body: &str) -> GetParameterHistoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetParameterHistoryError::InternalServerError(String::from(error_message))
                    }
                    "InvalidKeyId" => {
                        GetParameterHistoryError::InvalidKeyId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        GetParameterHistoryError::InvalidNextToken(String::from(error_message))
                    }
                    "ParameterNotFound" => {
                        GetParameterHistoryError::ParameterNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetParameterHistoryError::Validation(error_message.to_string())
                    }
                    _ => GetParameterHistoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetParameterHistoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetParameterHistoryError {
    fn from(err: serde_json::error::Error) -> GetParameterHistoryError {
        GetParameterHistoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetParameterHistoryError {
    fn from(err: CredentialsError) -> GetParameterHistoryError {
        GetParameterHistoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetParameterHistoryError {
    fn from(err: HttpDispatchError) -> GetParameterHistoryError {
        GetParameterHistoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetParameterHistoryError {
    fn from(err: io::Error) -> GetParameterHistoryError {
        GetParameterHistoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetParameterHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParameterHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetParameterHistoryError::InternalServerError(ref cause) => cause,
            GetParameterHistoryError::InvalidKeyId(ref cause) => cause,
            GetParameterHistoryError::InvalidNextToken(ref cause) => cause,
            GetParameterHistoryError::ParameterNotFound(ref cause) => cause,
            GetParameterHistoryError::Validation(ref cause) => cause,
            GetParameterHistoryError::Credentials(ref err) => err.description(),
            GetParameterHistoryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetParameterHistoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParameters
#[derive(Debug, PartialEq)]
pub enum GetParametersError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetParametersError {
    pub fn from_body(body: &str) -> GetParametersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetParametersError::InternalServerError(String::from(error_message))
                    }
                    "InvalidKeyId" => GetParametersError::InvalidKeyId(String::from(error_message)),
                    "ValidationException" => {
                        GetParametersError::Validation(error_message.to_string())
                    }
                    _ => GetParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetParametersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetParametersError {
    fn from(err: serde_json::error::Error) -> GetParametersError {
        GetParametersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetParametersError {
    fn from(err: CredentialsError) -> GetParametersError {
        GetParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetParametersError {
    fn from(err: HttpDispatchError) -> GetParametersError {
        GetParametersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetParametersError {
    fn from(err: io::Error) -> GetParametersError {
        GetParametersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParametersError {
    fn description(&self) -> &str {
        match *self {
            GetParametersError::InternalServerError(ref cause) => cause,
            GetParametersError::InvalidKeyId(ref cause) => cause,
            GetParametersError::Validation(ref cause) => cause,
            GetParametersError::Credentials(ref err) => err.description(),
            GetParametersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParametersByPath
#[derive(Debug, PartialEq)]
pub enum GetParametersByPathError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified filter option is not valid. Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel.</p>
    InvalidFilterOption(String),
    /// <p>The filter value is not valid. Verify the value and try again.</p>
    InvalidFilterValue(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetParametersByPathError {
    pub fn from_body(body: &str) -> GetParametersByPathError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetParametersByPathError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilterKey" => {
                        GetParametersByPathError::InvalidFilterKey(String::from(error_message))
                    }
                    "InvalidFilterOption" => {
                        GetParametersByPathError::InvalidFilterOption(String::from(error_message))
                    }
                    "InvalidFilterValue" => {
                        GetParametersByPathError::InvalidFilterValue(String::from(error_message))
                    }
                    "InvalidKeyId" => {
                        GetParametersByPathError::InvalidKeyId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        GetParametersByPathError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetParametersByPathError::Validation(error_message.to_string())
                    }
                    _ => GetParametersByPathError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetParametersByPathError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetParametersByPathError {
    fn from(err: serde_json::error::Error) -> GetParametersByPathError {
        GetParametersByPathError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetParametersByPathError {
    fn from(err: CredentialsError) -> GetParametersByPathError {
        GetParametersByPathError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetParametersByPathError {
    fn from(err: HttpDispatchError) -> GetParametersByPathError {
        GetParametersByPathError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetParametersByPathError {
    fn from(err: io::Error) -> GetParametersByPathError {
        GetParametersByPathError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetParametersByPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParametersByPathError {
    fn description(&self) -> &str {
        match *self {
            GetParametersByPathError::InternalServerError(ref cause) => cause,
            GetParametersByPathError::InvalidFilterKey(ref cause) => cause,
            GetParametersByPathError::InvalidFilterOption(ref cause) => cause,
            GetParametersByPathError::InvalidFilterValue(ref cause) => cause,
            GetParametersByPathError::InvalidKeyId(ref cause) => cause,
            GetParametersByPathError::InvalidNextToken(ref cause) => cause,
            GetParametersByPathError::Validation(ref cause) => cause,
            GetParametersByPathError::Credentials(ref err) => err.description(),
            GetParametersByPathError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetParametersByPathError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPatchBaseline
#[derive(Debug, PartialEq)]
pub enum GetPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPatchBaselineError {
    pub fn from_body(body: &str) -> GetPatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        GetPatchBaselineError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => {
                        GetPatchBaselineError::InternalServerError(String::from(error_message))
                    }
                    "InvalidResourceId" => {
                        GetPatchBaselineError::InvalidResourceId(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => GetPatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPatchBaselineError {
    fn from(err: serde_json::error::Error) -> GetPatchBaselineError {
        GetPatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPatchBaselineError {
    fn from(err: CredentialsError) -> GetPatchBaselineError {
        GetPatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPatchBaselineError {
    fn from(err: HttpDispatchError) -> GetPatchBaselineError {
        GetPatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPatchBaselineError {
    fn from(err: io::Error) -> GetPatchBaselineError {
        GetPatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            GetPatchBaselineError::DoesNotExist(ref cause) => cause,
            GetPatchBaselineError::InternalServerError(ref cause) => cause,
            GetPatchBaselineError::InvalidResourceId(ref cause) => cause,
            GetPatchBaselineError::Validation(ref cause) => cause,
            GetPatchBaselineError::Credentials(ref err) => err.description(),
            GetPatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum GetPatchBaselineForPatchGroupError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPatchBaselineForPatchGroupError {
    pub fn from_body(body: &str) -> GetPatchBaselineForPatchGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetPatchBaselineForPatchGroupError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        GetPatchBaselineForPatchGroupError::Validation(error_message.to_string())
                    }
                    _ => GetPatchBaselineForPatchGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPatchBaselineForPatchGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPatchBaselineForPatchGroupError {
    fn from(err: serde_json::error::Error) -> GetPatchBaselineForPatchGroupError {
        GetPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPatchBaselineForPatchGroupError {
    fn from(err: CredentialsError) -> GetPatchBaselineForPatchGroupError {
        GetPatchBaselineForPatchGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPatchBaselineForPatchGroupError {
    fn from(err: HttpDispatchError) -> GetPatchBaselineForPatchGroupError {
        GetPatchBaselineForPatchGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPatchBaselineForPatchGroupError {
    fn from(err: io::Error) -> GetPatchBaselineForPatchGroupError {
        GetPatchBaselineForPatchGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            GetPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
            GetPatchBaselineForPatchGroupError::Validation(ref cause) => cause,
            GetPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),
            GetPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPatchBaselineForPatchGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociationVersions
#[derive(Debug, PartialEq)]
pub enum ListAssociationVersionsError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssociationVersionsError {
    pub fn from_body(body: &str) -> ListAssociationVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationDoesNotExist" => {
                        ListAssociationVersionsError::AssociationDoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => ListAssociationVersionsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidNextToken" => {
                        ListAssociationVersionsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssociationVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListAssociationVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssociationVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssociationVersionsError {
    fn from(err: serde_json::error::Error) -> ListAssociationVersionsError {
        ListAssociationVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssociationVersionsError {
    fn from(err: CredentialsError) -> ListAssociationVersionsError {
        ListAssociationVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssociationVersionsError {
    fn from(err: HttpDispatchError) -> ListAssociationVersionsError {
        ListAssociationVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssociationVersionsError {
    fn from(err: io::Error) -> ListAssociationVersionsError {
        ListAssociationVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssociationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationVersionsError::AssociationDoesNotExist(ref cause) => cause,
            ListAssociationVersionsError::InternalServerError(ref cause) => cause,
            ListAssociationVersionsError::InvalidNextToken(ref cause) => cause,
            ListAssociationVersionsError::Validation(ref cause) => cause,
            ListAssociationVersionsError::Credentials(ref err) => err.description(),
            ListAssociationVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAssociationVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociations
#[derive(Debug, PartialEq)]
pub enum ListAssociationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAssociationsError {
    pub fn from_body(body: &str) -> ListAssociationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListAssociationsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListAssociationsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAssociationsError::Validation(error_message.to_string())
                    }
                    _ => ListAssociationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAssociationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAssociationsError {
    fn from(err: serde_json::error::Error) -> ListAssociationsError {
        ListAssociationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAssociationsError {
    fn from(err: CredentialsError) -> ListAssociationsError {
        ListAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAssociationsError {
    fn from(err: HttpDispatchError) -> ListAssociationsError {
        ListAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAssociationsError {
    fn from(err: io::Error) -> ListAssociationsError {
        ListAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationsError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationsError::InternalServerError(ref cause) => cause,
            ListAssociationsError::InvalidNextToken(ref cause) => cause,
            ListAssociationsError::Validation(ref cause) => cause,
            ListAssociationsError::Credentials(ref err) => err.description(),
            ListAssociationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAssociationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCommandInvocations
#[derive(Debug, PartialEq)]
pub enum ListCommandInvocationsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCommandInvocationsError {
    pub fn from_body(body: &str) -> ListCommandInvocationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => ListCommandInvocationsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidCommandId" => {
                        ListCommandInvocationsError::InvalidCommandId(String::from(error_message))
                    }
                    "InvalidFilterKey" => {
                        ListCommandInvocationsError::InvalidFilterKey(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        ListCommandInvocationsError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListCommandInvocationsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCommandInvocationsError::Validation(error_message.to_string())
                    }
                    _ => ListCommandInvocationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCommandInvocationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCommandInvocationsError {
    fn from(err: serde_json::error::Error) -> ListCommandInvocationsError {
        ListCommandInvocationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCommandInvocationsError {
    fn from(err: CredentialsError) -> ListCommandInvocationsError {
        ListCommandInvocationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCommandInvocationsError {
    fn from(err: HttpDispatchError) -> ListCommandInvocationsError {
        ListCommandInvocationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCommandInvocationsError {
    fn from(err: io::Error) -> ListCommandInvocationsError {
        ListCommandInvocationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCommandInvocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCommandInvocationsError {
    fn description(&self) -> &str {
        match *self {
            ListCommandInvocationsError::InternalServerError(ref cause) => cause,
            ListCommandInvocationsError::InvalidCommandId(ref cause) => cause,
            ListCommandInvocationsError::InvalidFilterKey(ref cause) => cause,
            ListCommandInvocationsError::InvalidInstanceId(ref cause) => cause,
            ListCommandInvocationsError::InvalidNextToken(ref cause) => cause,
            ListCommandInvocationsError::Validation(ref cause) => cause,
            ListCommandInvocationsError::Credentials(ref err) => err.description(),
            ListCommandInvocationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCommandInvocationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCommands
#[derive(Debug, PartialEq)]
pub enum ListCommandsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),

    InvalidCommandId(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCommandsError {
    pub fn from_body(body: &str) -> ListCommandsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListCommandsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidCommandId" => {
                        ListCommandsError::InvalidCommandId(String::from(error_message))
                    }
                    "InvalidFilterKey" => {
                        ListCommandsError::InvalidFilterKey(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        ListCommandsError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListCommandsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCommandsError::Validation(error_message.to_string())
                    }
                    _ => ListCommandsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCommandsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCommandsError {
    fn from(err: serde_json::error::Error) -> ListCommandsError {
        ListCommandsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCommandsError {
    fn from(err: CredentialsError) -> ListCommandsError {
        ListCommandsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCommandsError {
    fn from(err: HttpDispatchError) -> ListCommandsError {
        ListCommandsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCommandsError {
    fn from(err: io::Error) -> ListCommandsError {
        ListCommandsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCommandsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCommandsError {
    fn description(&self) -> &str {
        match *self {
            ListCommandsError::InternalServerError(ref cause) => cause,
            ListCommandsError::InvalidCommandId(ref cause) => cause,
            ListCommandsError::InvalidFilterKey(ref cause) => cause,
            ListCommandsError::InvalidInstanceId(ref cause) => cause,
            ListCommandsError::InvalidNextToken(ref cause) => cause,
            ListCommandsError::Validation(ref cause) => cause,
            ListCommandsError::Credentials(ref err) => err.description(),
            ListCommandsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCommandsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListComplianceItems
#[derive(Debug, PartialEq)]
pub enum ListComplianceItemsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListComplianceItemsError {
    pub fn from_body(body: &str) -> ListComplianceItemsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListComplianceItemsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilter" => {
                        ListComplianceItemsError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListComplianceItemsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidResourceId" => {
                        ListComplianceItemsError::InvalidResourceId(String::from(error_message))
                    }
                    "InvalidResourceType" => {
                        ListComplianceItemsError::InvalidResourceType(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListComplianceItemsError::Validation(error_message.to_string())
                    }
                    _ => ListComplianceItemsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListComplianceItemsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListComplianceItemsError {
    fn from(err: serde_json::error::Error) -> ListComplianceItemsError {
        ListComplianceItemsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListComplianceItemsError {
    fn from(err: CredentialsError) -> ListComplianceItemsError {
        ListComplianceItemsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListComplianceItemsError {
    fn from(err: HttpDispatchError) -> ListComplianceItemsError {
        ListComplianceItemsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListComplianceItemsError {
    fn from(err: io::Error) -> ListComplianceItemsError {
        ListComplianceItemsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListComplianceItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListComplianceItemsError {
    fn description(&self) -> &str {
        match *self {
            ListComplianceItemsError::InternalServerError(ref cause) => cause,
            ListComplianceItemsError::InvalidFilter(ref cause) => cause,
            ListComplianceItemsError::InvalidNextToken(ref cause) => cause,
            ListComplianceItemsError::InvalidResourceId(ref cause) => cause,
            ListComplianceItemsError::InvalidResourceType(ref cause) => cause,
            ListComplianceItemsError::Validation(ref cause) => cause,
            ListComplianceItemsError::Credentials(ref err) => err.description(),
            ListComplianceItemsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListComplianceItemsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListComplianceSummaries
#[derive(Debug, PartialEq)]
pub enum ListComplianceSummariesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListComplianceSummariesError {
    pub fn from_body(body: &str) -> ListComplianceSummariesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => ListComplianceSummariesError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidFilter" => {
                        ListComplianceSummariesError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListComplianceSummariesError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListComplianceSummariesError::Validation(error_message.to_string())
                    }
                    _ => ListComplianceSummariesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListComplianceSummariesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListComplianceSummariesError {
    fn from(err: serde_json::error::Error) -> ListComplianceSummariesError {
        ListComplianceSummariesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListComplianceSummariesError {
    fn from(err: CredentialsError) -> ListComplianceSummariesError {
        ListComplianceSummariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListComplianceSummariesError {
    fn from(err: HttpDispatchError) -> ListComplianceSummariesError {
        ListComplianceSummariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListComplianceSummariesError {
    fn from(err: io::Error) -> ListComplianceSummariesError {
        ListComplianceSummariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListComplianceSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListComplianceSummariesError {
    fn description(&self) -> &str {
        match *self {
            ListComplianceSummariesError::InternalServerError(ref cause) => cause,
            ListComplianceSummariesError::InvalidFilter(ref cause) => cause,
            ListComplianceSummariesError::InvalidNextToken(ref cause) => cause,
            ListComplianceSummariesError::Validation(ref cause) => cause,
            ListComplianceSummariesError::Credentials(ref err) => err.description(),
            ListComplianceSummariesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListComplianceSummariesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDocumentVersions
#[derive(Debug, PartialEq)]
pub enum ListDocumentVersionsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDocumentVersionsError {
    pub fn from_body(body: &str) -> ListDocumentVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListDocumentVersionsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        ListDocumentVersionsError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListDocumentVersionsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDocumentVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListDocumentVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDocumentVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDocumentVersionsError {
    fn from(err: serde_json::error::Error) -> ListDocumentVersionsError {
        ListDocumentVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDocumentVersionsError {
    fn from(err: CredentialsError) -> ListDocumentVersionsError {
        ListDocumentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDocumentVersionsError {
    fn from(err: HttpDispatchError) -> ListDocumentVersionsError {
        ListDocumentVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDocumentVersionsError {
    fn from(err: io::Error) -> ListDocumentVersionsError {
        ListDocumentVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDocumentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentVersionsError::InternalServerError(ref cause) => cause,
            ListDocumentVersionsError::InvalidDocument(ref cause) => cause,
            ListDocumentVersionsError::InvalidNextToken(ref cause) => cause,
            ListDocumentVersionsError::Validation(ref cause) => cause,
            ListDocumentVersionsError::Credentials(ref err) => err.description(),
            ListDocumentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDocumentVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDocuments
#[derive(Debug, PartialEq)]
pub enum ListDocumentsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified key is not valid.</p>
    InvalidFilterKey(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDocumentsError {
    pub fn from_body(body: &str) -> ListDocumentsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListDocumentsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilterKey" => {
                        ListDocumentsError::InvalidFilterKey(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListDocumentsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDocumentsError::Validation(error_message.to_string())
                    }
                    _ => ListDocumentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDocumentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDocumentsError {
    fn from(err: serde_json::error::Error) -> ListDocumentsError {
        ListDocumentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDocumentsError {
    fn from(err: CredentialsError) -> ListDocumentsError {
        ListDocumentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDocumentsError {
    fn from(err: HttpDispatchError) -> ListDocumentsError {
        ListDocumentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDocumentsError {
    fn from(err: io::Error) -> ListDocumentsError {
        ListDocumentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDocumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentsError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentsError::InternalServerError(ref cause) => cause,
            ListDocumentsError::InvalidFilterKey(ref cause) => cause,
            ListDocumentsError::InvalidNextToken(ref cause) => cause,
            ListDocumentsError::Validation(ref cause) => cause,
            ListDocumentsError::Credentials(ref err) => err.description(),
            ListDocumentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDocumentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInventoryEntries
#[derive(Debug, PartialEq)]
pub enum ListInventoryEntriesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListInventoryEntriesError {
    pub fn from_body(body: &str) -> ListInventoryEntriesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListInventoryEntriesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidFilter" => {
                        ListInventoryEntriesError::InvalidFilter(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        ListInventoryEntriesError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListInventoryEntriesError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidTypeNameException" => {
                        ListInventoryEntriesError::InvalidTypeName(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListInventoryEntriesError::Validation(error_message.to_string())
                    }
                    _ => ListInventoryEntriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListInventoryEntriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListInventoryEntriesError {
    fn from(err: serde_json::error::Error) -> ListInventoryEntriesError {
        ListInventoryEntriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInventoryEntriesError {
    fn from(err: CredentialsError) -> ListInventoryEntriesError {
        ListInventoryEntriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInventoryEntriesError {
    fn from(err: HttpDispatchError) -> ListInventoryEntriesError {
        ListInventoryEntriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInventoryEntriesError {
    fn from(err: io::Error) -> ListInventoryEntriesError {
        ListInventoryEntriesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInventoryEntriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInventoryEntriesError {
    fn description(&self) -> &str {
        match *self {
            ListInventoryEntriesError::InternalServerError(ref cause) => cause,
            ListInventoryEntriesError::InvalidFilter(ref cause) => cause,
            ListInventoryEntriesError::InvalidInstanceId(ref cause) => cause,
            ListInventoryEntriesError::InvalidNextToken(ref cause) => cause,
            ListInventoryEntriesError::InvalidTypeName(ref cause) => cause,
            ListInventoryEntriesError::Validation(ref cause) => cause,
            ListInventoryEntriesError::Credentials(ref err) => err.description(),
            ListInventoryEntriesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListInventoryEntriesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceComplianceSummaries
#[derive(Debug, PartialEq)]
pub enum ListResourceComplianceSummariesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
    InvalidFilter(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListResourceComplianceSummariesError {
    pub fn from_body(body: &str) -> ListResourceComplianceSummariesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListResourceComplianceSummariesError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidFilter" => ListResourceComplianceSummariesError::InvalidFilter(
                        String::from(error_message),
                    ),
                    "InvalidNextToken" => ListResourceComplianceSummariesError::InvalidNextToken(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        ListResourceComplianceSummariesError::Validation(error_message.to_string())
                    }
                    _ => ListResourceComplianceSummariesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourceComplianceSummariesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourceComplianceSummariesError {
    fn from(err: serde_json::error::Error) -> ListResourceComplianceSummariesError {
        ListResourceComplianceSummariesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceComplianceSummariesError {
    fn from(err: CredentialsError) -> ListResourceComplianceSummariesError {
        ListResourceComplianceSummariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceComplianceSummariesError {
    fn from(err: HttpDispatchError) -> ListResourceComplianceSummariesError {
        ListResourceComplianceSummariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceComplianceSummariesError {
    fn from(err: io::Error) -> ListResourceComplianceSummariesError {
        ListResourceComplianceSummariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceComplianceSummariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceComplianceSummariesError {
    fn description(&self) -> &str {
        match *self {
            ListResourceComplianceSummariesError::InternalServerError(ref cause) => cause,
            ListResourceComplianceSummariesError::InvalidFilter(ref cause) => cause,
            ListResourceComplianceSummariesError::InvalidNextToken(ref cause) => cause,
            ListResourceComplianceSummariesError::Validation(ref cause) => cause,
            ListResourceComplianceSummariesError::Credentials(ref err) => err.description(),
            ListResourceComplianceSummariesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceComplianceSummariesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceDataSync
#[derive(Debug, PartialEq)]
pub enum ListResourceDataSyncError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified token is not valid.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListResourceDataSyncError {
    pub fn from_body(body: &str) -> ListResourceDataSyncError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListResourceDataSyncError::InternalServerError(String::from(error_message))
                    }
                    "InvalidNextToken" => {
                        ListResourceDataSyncError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListResourceDataSyncError::Validation(error_message.to_string())
                    }
                    _ => ListResourceDataSyncError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourceDataSyncError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourceDataSyncError {
    fn from(err: serde_json::error::Error) -> ListResourceDataSyncError {
        ListResourceDataSyncError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceDataSyncError {
    fn from(err: CredentialsError) -> ListResourceDataSyncError {
        ListResourceDataSyncError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceDataSyncError {
    fn from(err: HttpDispatchError) -> ListResourceDataSyncError {
        ListResourceDataSyncError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListResourceDataSyncError {
    fn from(err: io::Error) -> ListResourceDataSyncError {
        ListResourceDataSyncError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListResourceDataSyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceDataSyncError {
    fn description(&self) -> &str {
        match *self {
            ListResourceDataSyncError::InternalServerError(ref cause) => cause,
            ListResourceDataSyncError::InvalidNextToken(ref cause) => cause,
            ListResourceDataSyncError::Validation(ref cause) => cause,
            ListResourceDataSyncError::Credentials(ref err) => err.description(),
            ListResourceDataSyncError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourceDataSyncError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
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
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListTagsForResourceError::InternalServerError(String::from(error_message))
                    }
                    "InvalidResourceId" => {
                        ListTagsForResourceError::InvalidResourceId(String::from(error_message))
                    }
                    "InvalidResourceType" => {
                        ListTagsForResourceError::InvalidResourceType(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
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
            ListTagsForResourceError::InternalServerError(ref cause) => cause,
            ListTagsForResourceError::InvalidResourceId(ref cause) => cause,
            ListTagsForResourceError::InvalidResourceType(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyDocumentPermission
#[derive(Debug, PartialEq)]
pub enum ModifyDocumentPermissionError {
    /// <p>You can have at most 200 active Systems Manager documents.</p>
    DocumentLimitExceeded(String),
    /// <p>The document cannot be shared with more AWS user accounts. You can share a document with a maximum of 20 accounts. You can publicly share up to five documents. If you need to increase this limit, contact AWS Support.</p>
    DocumentPermissionLimit(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
    InvalidPermissionType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ModifyDocumentPermissionError {
    pub fn from_body(body: &str) -> ModifyDocumentPermissionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DocumentLimitExceeded" => {
                        ModifyDocumentPermissionError::DocumentLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "DocumentPermissionLimit" => {
                        ModifyDocumentPermissionError::DocumentPermissionLimit(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => ModifyDocumentPermissionError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidDocument" => {
                        ModifyDocumentPermissionError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidPermissionType" => {
                        ModifyDocumentPermissionError::InvalidPermissionType(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ModifyDocumentPermissionError::Validation(error_message.to_string())
                    }
                    _ => ModifyDocumentPermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyDocumentPermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyDocumentPermissionError {
    fn from(err: serde_json::error::Error) -> ModifyDocumentPermissionError {
        ModifyDocumentPermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyDocumentPermissionError {
    fn from(err: CredentialsError) -> ModifyDocumentPermissionError {
        ModifyDocumentPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyDocumentPermissionError {
    fn from(err: HttpDispatchError) -> ModifyDocumentPermissionError {
        ModifyDocumentPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ModifyDocumentPermissionError {
    fn from(err: io::Error) -> ModifyDocumentPermissionError {
        ModifyDocumentPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ModifyDocumentPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyDocumentPermissionError {
    fn description(&self) -> &str {
        match *self {
            ModifyDocumentPermissionError::DocumentLimitExceeded(ref cause) => cause,
            ModifyDocumentPermissionError::DocumentPermissionLimit(ref cause) => cause,
            ModifyDocumentPermissionError::InternalServerError(ref cause) => cause,
            ModifyDocumentPermissionError::InvalidDocument(ref cause) => cause,
            ModifyDocumentPermissionError::InvalidPermissionType(ref cause) => cause,
            ModifyDocumentPermissionError::Validation(ref cause) => cause,
            ModifyDocumentPermissionError::Credentials(ref err) => err.description(),
            ModifyDocumentPermissionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyDocumentPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutComplianceItems
#[derive(Debug, PartialEq)]
pub enum PutComplianceItemsError {
    /// <p>You specified too many custom compliance types. You can specify a maximum of 10 different types. </p>
    ComplianceTypeCountLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>One or more content items is not valid.</p>
    InvalidItemContent(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// <p>The inventory item size has exceeded the size limit.</p>
    ItemSizeLimitExceeded(String),
    /// <p>The size of inventory data has exceeded the total size limit for the resource.</p>
    TotalSizeLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutComplianceItemsError {
    pub fn from_body(body: &str) -> PutComplianceItemsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ComplianceTypeCountLimitExceededException" => {
                        PutComplianceItemsError::ComplianceTypeCountLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        PutComplianceItemsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidItemContentException" => {
                        PutComplianceItemsError::InvalidItemContent(String::from(error_message))
                    }
                    "InvalidResourceId" => {
                        PutComplianceItemsError::InvalidResourceId(String::from(error_message))
                    }
                    "InvalidResourceType" => {
                        PutComplianceItemsError::InvalidResourceType(String::from(error_message))
                    }
                    "ItemSizeLimitExceededException" => {
                        PutComplianceItemsError::ItemSizeLimitExceeded(String::from(error_message))
                    }
                    "TotalSizeLimitExceededException" => {
                        PutComplianceItemsError::TotalSizeLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutComplianceItemsError::Validation(error_message.to_string())
                    }
                    _ => PutComplianceItemsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutComplianceItemsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutComplianceItemsError {
    fn from(err: serde_json::error::Error) -> PutComplianceItemsError {
        PutComplianceItemsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutComplianceItemsError {
    fn from(err: CredentialsError) -> PutComplianceItemsError {
        PutComplianceItemsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutComplianceItemsError {
    fn from(err: HttpDispatchError) -> PutComplianceItemsError {
        PutComplianceItemsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutComplianceItemsError {
    fn from(err: io::Error) -> PutComplianceItemsError {
        PutComplianceItemsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutComplianceItemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutComplianceItemsError {
    fn description(&self) -> &str {
        match *self {
            PutComplianceItemsError::ComplianceTypeCountLimitExceeded(ref cause) => cause,
            PutComplianceItemsError::InternalServerError(ref cause) => cause,
            PutComplianceItemsError::InvalidItemContent(ref cause) => cause,
            PutComplianceItemsError::InvalidResourceId(ref cause) => cause,
            PutComplianceItemsError::InvalidResourceType(ref cause) => cause,
            PutComplianceItemsError::ItemSizeLimitExceeded(ref cause) => cause,
            PutComplianceItemsError::TotalSizeLimitExceeded(ref cause) => cause,
            PutComplianceItemsError::Validation(ref cause) => cause,
            PutComplianceItemsError::Credentials(ref err) => err.description(),
            PutComplianceItemsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutComplianceItemsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutInventory
#[derive(Debug, PartialEq)]
pub enum PutInventoryError {
    /// <p>You have exceeded the limit for custom schemas. Delete one or more custom schemas and try again.</p>
    CustomSchemaCountLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>You specified invalid keys or values in the <code>Context</code> attribute for <code>InventoryItem</code>. Verify the keys and values, and try again.</p>
    InvalidInventoryItemContext(String),
    /// <p>One or more content items is not valid.</p>
    InvalidItemContent(String),
    /// <p>The parameter type name is not valid.</p>
    InvalidTypeName(String),
    /// <p>The inventory item has invalid content. </p>
    ItemContentMismatch(String),
    /// <p>The inventory item size has exceeded the size limit.</p>
    ItemSizeLimitExceeded(String),
    /// <p>The sub-type count exceeded the limit for the inventory type.</p>
    SubTypeCountLimitExceeded(String),
    /// <p>The size of inventory data has exceeded the total size limit for the resource.</p>
    TotalSizeLimitExceeded(String),
    /// <p>The <code>Context</code> attribute that you specified for the <code>InventoryItem</code> is not allowed for this inventory type. You can only use the <code>Context</code> attribute with inventory types like <code>AWS:ComplianceItem</code>.</p>
    UnsupportedInventoryItemContext(String),
    /// <p>Inventory item type schema version has to match supported versions in the service. Check output of GetInventorySchema to see the available schema version for each type.</p>
    UnsupportedInventorySchemaVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutInventoryError {
    pub fn from_body(body: &str) -> PutInventoryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CustomSchemaCountLimitExceededException" => {
                        PutInventoryError::CustomSchemaCountLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        PutInventoryError::InternalServerError(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        PutInventoryError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidInventoryItemContextException" => {
                        PutInventoryError::InvalidInventoryItemContext(String::from(error_message))
                    }
                    "InvalidItemContentException" => {
                        PutInventoryError::InvalidItemContent(String::from(error_message))
                    }
                    "InvalidTypeNameException" => {
                        PutInventoryError::InvalidTypeName(String::from(error_message))
                    }
                    "ItemContentMismatchException" => {
                        PutInventoryError::ItemContentMismatch(String::from(error_message))
                    }
                    "ItemSizeLimitExceededException" => {
                        PutInventoryError::ItemSizeLimitExceeded(String::from(error_message))
                    }
                    "SubTypeCountLimitExceededException" => {
                        PutInventoryError::SubTypeCountLimitExceeded(String::from(error_message))
                    }
                    "TotalSizeLimitExceededException" => {
                        PutInventoryError::TotalSizeLimitExceeded(String::from(error_message))
                    }
                    "UnsupportedInventoryItemContextException" => {
                        PutInventoryError::UnsupportedInventoryItemContext(String::from(
                            error_message,
                        ))
                    }
                    "UnsupportedInventorySchemaVersionException" => {
                        PutInventoryError::UnsupportedInventorySchemaVersion(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        PutInventoryError::Validation(error_message.to_string())
                    }
                    _ => PutInventoryError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutInventoryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutInventoryError {
    fn from(err: serde_json::error::Error) -> PutInventoryError {
        PutInventoryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutInventoryError {
    fn from(err: CredentialsError) -> PutInventoryError {
        PutInventoryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutInventoryError {
    fn from(err: HttpDispatchError) -> PutInventoryError {
        PutInventoryError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutInventoryError {
    fn from(err: io::Error) -> PutInventoryError {
        PutInventoryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutInventoryError {
    fn description(&self) -> &str {
        match *self {
            PutInventoryError::CustomSchemaCountLimitExceeded(ref cause) => cause,
            PutInventoryError::InternalServerError(ref cause) => cause,
            PutInventoryError::InvalidInstanceId(ref cause) => cause,
            PutInventoryError::InvalidInventoryItemContext(ref cause) => cause,
            PutInventoryError::InvalidItemContent(ref cause) => cause,
            PutInventoryError::InvalidTypeName(ref cause) => cause,
            PutInventoryError::ItemContentMismatch(ref cause) => cause,
            PutInventoryError::ItemSizeLimitExceeded(ref cause) => cause,
            PutInventoryError::SubTypeCountLimitExceeded(ref cause) => cause,
            PutInventoryError::TotalSizeLimitExceeded(ref cause) => cause,
            PutInventoryError::UnsupportedInventoryItemContext(ref cause) => cause,
            PutInventoryError::UnsupportedInventorySchemaVersion(ref cause) => cause,
            PutInventoryError::Validation(ref cause) => cause,
            PutInventoryError::Credentials(ref err) => err.description(),
            PutInventoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutInventoryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutParameter
#[derive(Debug, PartialEq)]
pub enum PutParameterError {
    /// <p>A hierarchy can have a maximum of 15 levels. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-working.html">Working with Systems Manager Parameters</a>. </p>
    HierarchyLevelLimitExceeded(String),
    /// <p>Parameter Store does not support changing a parameter type in a hierarchy. For example, you can't change a parameter from a String type to a SecureString type. You must create a new, unique parameter.</p>
    HierarchyTypeMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The request does not meet the regular expression requirement.</p>
    InvalidAllowedPattern(String),
    /// <p>The query key ID is not valid.</p>
    InvalidKeyId(String),
    /// <p>The parameter already exists. You can't create duplicate parameters.</p>
    ParameterAlreadyExists(String),
    /// <p>You have exceeded the number of parameters for this AWS account. Delete one or more parameters and try again.</p>
    ParameterLimitExceeded(String),
    /// <p>The parameter exceeded the maximum number of allowed versions.</p>
    ParameterMaxVersionLimitExceeded(String),
    /// <p>The parameter name is not valid.</p>
    ParameterPatternMismatch(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// <p>The parameter type is not supported.</p>
    UnsupportedParameterType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutParameterError {
    pub fn from_body(body: &str) -> PutParameterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "HierarchyLevelLimitExceededException" => {
                        PutParameterError::HierarchyLevelLimitExceeded(String::from(error_message))
                    }
                    "HierarchyTypeMismatchException" => {
                        PutParameterError::HierarchyTypeMismatch(String::from(error_message))
                    }
                    "InternalServerError" => {
                        PutParameterError::InternalServerError(String::from(error_message))
                    }
                    "InvalidAllowedPatternException" => {
                        PutParameterError::InvalidAllowedPattern(String::from(error_message))
                    }
                    "InvalidKeyId" => PutParameterError::InvalidKeyId(String::from(error_message)),
                    "ParameterAlreadyExists" => {
                        PutParameterError::ParameterAlreadyExists(String::from(error_message))
                    }
                    "ParameterLimitExceeded" => {
                        PutParameterError::ParameterLimitExceeded(String::from(error_message))
                    }
                    "ParameterMaxVersionLimitExceeded" => {
                        PutParameterError::ParameterMaxVersionLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ParameterPatternMismatchException" => {
                        PutParameterError::ParameterPatternMismatch(String::from(error_message))
                    }
                    "TooManyUpdates" => {
                        PutParameterError::TooManyUpdates(String::from(error_message))
                    }
                    "UnsupportedParameterType" => {
                        PutParameterError::UnsupportedParameterType(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutParameterError::Validation(error_message.to_string())
                    }
                    _ => PutParameterError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutParameterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutParameterError {
    fn from(err: serde_json::error::Error) -> PutParameterError {
        PutParameterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutParameterError {
    fn from(err: CredentialsError) -> PutParameterError {
        PutParameterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutParameterError {
    fn from(err: HttpDispatchError) -> PutParameterError {
        PutParameterError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutParameterError {
    fn from(err: io::Error) -> PutParameterError {
        PutParameterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutParameterError {
    fn description(&self) -> &str {
        match *self {
            PutParameterError::HierarchyLevelLimitExceeded(ref cause) => cause,
            PutParameterError::HierarchyTypeMismatch(ref cause) => cause,
            PutParameterError::InternalServerError(ref cause) => cause,
            PutParameterError::InvalidAllowedPattern(ref cause) => cause,
            PutParameterError::InvalidKeyId(ref cause) => cause,
            PutParameterError::ParameterAlreadyExists(ref cause) => cause,
            PutParameterError::ParameterLimitExceeded(ref cause) => cause,
            PutParameterError::ParameterMaxVersionLimitExceeded(ref cause) => cause,
            PutParameterError::ParameterPatternMismatch(ref cause) => cause,
            PutParameterError::TooManyUpdates(ref cause) => cause,
            PutParameterError::UnsupportedParameterType(ref cause) => cause,
            PutParameterError::Validation(ref cause) => cause,
            PutParameterError::Credentials(ref err) => err.description(),
            PutParameterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutParameterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterDefaultPatchBaseline
#[derive(Debug, PartialEq)]
pub enum RegisterDefaultPatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterDefaultPatchBaselineError {
    pub fn from_body(body: &str) -> RegisterDefaultPatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        RegisterDefaultPatchBaselineError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => {
                        RegisterDefaultPatchBaselineError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceId" => RegisterDefaultPatchBaselineError::InvalidResourceId(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        RegisterDefaultPatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => RegisterDefaultPatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterDefaultPatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterDefaultPatchBaselineError {
    fn from(err: serde_json::error::Error) -> RegisterDefaultPatchBaselineError {
        RegisterDefaultPatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterDefaultPatchBaselineError {
    fn from(err: CredentialsError) -> RegisterDefaultPatchBaselineError {
        RegisterDefaultPatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterDefaultPatchBaselineError {
    fn from(err: HttpDispatchError) -> RegisterDefaultPatchBaselineError {
        RegisterDefaultPatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterDefaultPatchBaselineError {
    fn from(err: io::Error) -> RegisterDefaultPatchBaselineError {
        RegisterDefaultPatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterDefaultPatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterDefaultPatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            RegisterDefaultPatchBaselineError::DoesNotExist(ref cause) => cause,
            RegisterDefaultPatchBaselineError::InternalServerError(ref cause) => cause,
            RegisterDefaultPatchBaselineError::InvalidResourceId(ref cause) => cause,
            RegisterDefaultPatchBaselineError::Validation(ref cause) => cause,
            RegisterDefaultPatchBaselineError::Credentials(ref err) => err.description(),
            RegisterDefaultPatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterDefaultPatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterPatchBaselineForPatchGroup
#[derive(Debug, PartialEq)]
pub enum RegisterPatchBaselineForPatchGroupError {
    /// <p>Error returned if an attempt is made to register a patch group with a patch baseline that is already registered with a different patch baseline.</p>
    AlreadyExists(String),
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many Maintenance Windows or Patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterPatchBaselineForPatchGroupError {
    pub fn from_body(body: &str) -> RegisterPatchBaselineForPatchGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        RegisterPatchBaselineForPatchGroupError::AlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "DoesNotExistException" => {
                        RegisterPatchBaselineForPatchGroupError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        RegisterPatchBaselineForPatchGroupError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidResourceId" => {
                        RegisterPatchBaselineForPatchGroupError::InvalidResourceId(String::from(
                            error_message,
                        ))
                    }
                    "ResourceLimitExceededException" => {
                        RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => RegisterPatchBaselineForPatchGroupError::Validation(
                        error_message.to_string(),
                    ),
                    _ => RegisterPatchBaselineForPatchGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterPatchBaselineForPatchGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterPatchBaselineForPatchGroupError {
    fn from(err: serde_json::error::Error) -> RegisterPatchBaselineForPatchGroupError {
        RegisterPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterPatchBaselineForPatchGroupError {
    fn from(err: CredentialsError) -> RegisterPatchBaselineForPatchGroupError {
        RegisterPatchBaselineForPatchGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterPatchBaselineForPatchGroupError {
    fn from(err: HttpDispatchError) -> RegisterPatchBaselineForPatchGroupError {
        RegisterPatchBaselineForPatchGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterPatchBaselineForPatchGroupError {
    fn from(err: io::Error) -> RegisterPatchBaselineForPatchGroupError {
        RegisterPatchBaselineForPatchGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterPatchBaselineForPatchGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterPatchBaselineForPatchGroupError {
    fn description(&self) -> &str {
        match *self {
            RegisterPatchBaselineForPatchGroupError::AlreadyExists(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::DoesNotExist(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::Validation(ref cause) => cause,
            RegisterPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),
            RegisterPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterPatchBaselineForPatchGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTargetWithMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum RegisterTargetWithMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many Maintenance Windows or Patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterTargetWithMaintenanceWindowError {
    pub fn from_body(body: &str) -> RegisterTargetWithMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        RegisterTargetWithMaintenanceWindowError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "IdempotentParameterMismatch" => {
                        RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        RegisterTargetWithMaintenanceWindowError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ResourceLimitExceededException" => {
                        RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => RegisterTargetWithMaintenanceWindowError::Validation(
                        error_message.to_string(),
                    ),
                    _ => RegisterTargetWithMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterTargetWithMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterTargetWithMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> RegisterTargetWithMaintenanceWindowError {
        RegisterTargetWithMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterTargetWithMaintenanceWindowError {
    fn from(err: CredentialsError) -> RegisterTargetWithMaintenanceWindowError {
        RegisterTargetWithMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterTargetWithMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> RegisterTargetWithMaintenanceWindowError {
        RegisterTargetWithMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterTargetWithMaintenanceWindowError {
    fn from(err: io::Error) -> RegisterTargetWithMaintenanceWindowError {
        RegisterTargetWithMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterTargetWithMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTargetWithMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            RegisterTargetWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => {
                cause
            }
            RegisterTargetWithMaintenanceWindowError::InternalServerError(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::Validation(ref cause) => cause,
            RegisterTargetWithMaintenanceWindowError::Credentials(ref err) => err.description(),
            RegisterTargetWithMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterTargetWithMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTaskWithMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum RegisterTaskWithMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>You attempted to register a LAMBDA or STEP_FUNCTION task in a region where the corresponding service is not available. </p>
    FeatureNotAvailable(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Error returned when the caller has exceeded the default resource limits. For example, too many Maintenance Windows or Patch baselines have been created.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    ResourceLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterTaskWithMaintenanceWindowError {
    pub fn from_body(body: &str) -> RegisterTaskWithMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        RegisterTaskWithMaintenanceWindowError::DoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "FeatureNotAvailableException" => {
                        RegisterTaskWithMaintenanceWindowError::FeatureNotAvailable(String::from(
                            error_message,
                        ))
                    }
                    "IdempotentParameterMismatch" => {
                        RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(
                            String::from(error_message),
                        )
                    }
                    "InternalServerError" => {
                        RegisterTaskWithMaintenanceWindowError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ResourceLimitExceededException" => {
                        RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => RegisterTaskWithMaintenanceWindowError::Validation(
                        error_message.to_string(),
                    ),
                    _ => RegisterTaskWithMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterTaskWithMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterTaskWithMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> RegisterTaskWithMaintenanceWindowError {
        RegisterTaskWithMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterTaskWithMaintenanceWindowError {
    fn from(err: CredentialsError) -> RegisterTaskWithMaintenanceWindowError {
        RegisterTaskWithMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterTaskWithMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> RegisterTaskWithMaintenanceWindowError {
        RegisterTaskWithMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterTaskWithMaintenanceWindowError {
    fn from(err: io::Error) -> RegisterTaskWithMaintenanceWindowError {
        RegisterTaskWithMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterTaskWithMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTaskWithMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            RegisterTaskWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::FeatureNotAvailable(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::InternalServerError(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::Validation(ref cause) => cause,
            RegisterTaskWithMaintenanceWindowError::Credentials(ref err) => err.description(),
            RegisterTaskWithMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterTaskWithMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
    InvalidResourceId(String),
    /// <p>The resource type is not valid. For example, if you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
    InvalidResourceType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_body(body: &str) -> RemoveTagsFromResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => RemoveTagsFromResourceError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidResourceId" => {
                        RemoveTagsFromResourceError::InvalidResourceId(String::from(error_message))
                    }
                    "InvalidResourceType" => RemoveTagsFromResourceError::InvalidResourceType(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        RemoveTagsFromResourceError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromResourceError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromResourceError {
    fn from(err: CredentialsError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromResourceError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsFromResourceError {
    fn from(err: io::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::InternalServerError(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidResourceId(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidResourceType(ref cause) => cause,
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendAutomationSignal
#[derive(Debug, PartialEq)]
pub enum SendAutomationSignalError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>The specified step name and execution ID don't exist. Verify the information and try again.</p>
    AutomationStepNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The signal is not valid for the current Automation execution.</p>
    InvalidAutomationSignal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendAutomationSignalError {
    pub fn from_body(body: &str) -> SendAutomationSignalError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AutomationExecutionNotFoundException" => {
                        SendAutomationSignalError::AutomationExecutionNotFound(String::from(
                            error_message,
                        ))
                    }
                    "AutomationStepNotFoundException" => {
                        SendAutomationSignalError::AutomationStepNotFound(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        SendAutomationSignalError::InternalServerError(String::from(error_message))
                    }
                    "InvalidAutomationSignalException" => {
                        SendAutomationSignalError::InvalidAutomationSignal(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        SendAutomationSignalError::Validation(error_message.to_string())
                    }
                    _ => SendAutomationSignalError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendAutomationSignalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendAutomationSignalError {
    fn from(err: serde_json::error::Error) -> SendAutomationSignalError {
        SendAutomationSignalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendAutomationSignalError {
    fn from(err: CredentialsError) -> SendAutomationSignalError {
        SendAutomationSignalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendAutomationSignalError {
    fn from(err: HttpDispatchError) -> SendAutomationSignalError {
        SendAutomationSignalError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendAutomationSignalError {
    fn from(err: io::Error) -> SendAutomationSignalError {
        SendAutomationSignalError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendAutomationSignalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendAutomationSignalError {
    fn description(&self) -> &str {
        match *self {
            SendAutomationSignalError::AutomationExecutionNotFound(ref cause) => cause,
            SendAutomationSignalError::AutomationStepNotFound(ref cause) => cause,
            SendAutomationSignalError::InternalServerError(ref cause) => cause,
            SendAutomationSignalError::InvalidAutomationSignal(ref cause) => cause,
            SendAutomationSignalError::Validation(ref cause) => cause,
            SendAutomationSignalError::Credentials(ref err) => err.description(),
            SendAutomationSignalError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SendAutomationSignalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SendCommand
#[derive(Debug, PartialEq)]
pub enum SendCommandError {
    /// <p>You cannot specify an instance ID in more than one association.</p>
    DuplicateInstanceId(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>One or more configuration items is not valid. Verify that a valid Amazon Resource Name (ARN) was provided for an Amazon SNS topic.</p>
    InvalidNotificationConfig(String),
    /// <p>The S3 bucket does not exist.</p>
    InvalidOutputFolder(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The role name can't contain invalid characters. Also verify that you specified an IAM role for notifications that includes the required trust policy. For information about configuring the IAM role for Run Command notifications, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/rc-sns-notifications.html">Configuring Amazon SNS Notifications for Run Command</a> in the <i>AWS Systems Manager User Guide</i>.</p>
    InvalidRole(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
    /// <p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
    UnsupportedPlatformType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SendCommandError {
    pub fn from_body(body: &str) -> SendCommandError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DuplicateInstanceId" => {
                        SendCommandError::DuplicateInstanceId(String::from(error_message))
                    }
                    "InternalServerError" => {
                        SendCommandError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        SendCommandError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        SendCommandError::InvalidInstanceId(String::from(error_message))
                    }
                    "InvalidNotificationConfig" => {
                        SendCommandError::InvalidNotificationConfig(String::from(error_message))
                    }
                    "InvalidOutputFolder" => {
                        SendCommandError::InvalidOutputFolder(String::from(error_message))
                    }
                    "InvalidParameters" => {
                        SendCommandError::InvalidParameters(String::from(error_message))
                    }
                    "InvalidRole" => SendCommandError::InvalidRole(String::from(error_message)),
                    "MaxDocumentSizeExceeded" => {
                        SendCommandError::MaxDocumentSizeExceeded(String::from(error_message))
                    }
                    "UnsupportedPlatformType" => {
                        SendCommandError::UnsupportedPlatformType(String::from(error_message))
                    }
                    "ValidationException" => {
                        SendCommandError::Validation(error_message.to_string())
                    }
                    _ => SendCommandError::Unknown(String::from(body)),
                }
            }
            Err(_) => SendCommandError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SendCommandError {
    fn from(err: serde_json::error::Error) -> SendCommandError {
        SendCommandError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SendCommandError {
    fn from(err: CredentialsError) -> SendCommandError {
        SendCommandError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SendCommandError {
    fn from(err: HttpDispatchError) -> SendCommandError {
        SendCommandError::HttpDispatch(err)
    }
}
impl From<io::Error> for SendCommandError {
    fn from(err: io::Error) -> SendCommandError {
        SendCommandError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SendCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendCommandError {
    fn description(&self) -> &str {
        match *self {
            SendCommandError::DuplicateInstanceId(ref cause) => cause,
            SendCommandError::InternalServerError(ref cause) => cause,
            SendCommandError::InvalidDocument(ref cause) => cause,
            SendCommandError::InvalidInstanceId(ref cause) => cause,
            SendCommandError::InvalidNotificationConfig(ref cause) => cause,
            SendCommandError::InvalidOutputFolder(ref cause) => cause,
            SendCommandError::InvalidParameters(ref cause) => cause,
            SendCommandError::InvalidRole(ref cause) => cause,
            SendCommandError::MaxDocumentSizeExceeded(ref cause) => cause,
            SendCommandError::UnsupportedPlatformType(ref cause) => cause,
            SendCommandError::Validation(ref cause) => cause,
            SendCommandError::Credentials(ref err) => err.description(),
            SendCommandError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SendCommandError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAutomationExecution
#[derive(Debug, PartialEq)]
pub enum StartAutomationExecutionError {
    /// <p>An Automation document with the specified name could not be found.</p>
    AutomationDefinitionNotFound(String),
    /// <p>An Automation document with the specified name and version could not be found.</p>
    AutomationDefinitionVersionNotFound(String),
    /// <p>The number of simultaneously running Automation executions exceeded the allowable limit.</p>
    AutomationExecutionLimitExceeded(String),
    /// <p>Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. </p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The supplied parameters for invoking the specified Automation document are incorrect. For example, they may not match the set of parameters permitted for the specified Automation document.</p>
    InvalidAutomationExecutionParameters(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartAutomationExecutionError {
    pub fn from_body(body: &str) -> StartAutomationExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AutomationDefinitionNotFoundException" => {
                        StartAutomationExecutionError::AutomationDefinitionNotFound(String::from(
                            error_message,
                        ))
                    }
                    "AutomationDefinitionVersionNotFoundException" => {
                        StartAutomationExecutionError::AutomationDefinitionVersionNotFound(
                            String::from(error_message),
                        )
                    }
                    "AutomationExecutionLimitExceededException" => {
                        StartAutomationExecutionError::AutomationExecutionLimitExceeded(
                            String::from(error_message),
                        )
                    }
                    "IdempotentParameterMismatch" => {
                        StartAutomationExecutionError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => StartAutomationExecutionError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidAutomationExecutionParametersException" => {
                        StartAutomationExecutionError::InvalidAutomationExecutionParameters(
                            String::from(error_message),
                        )
                    }
                    "InvalidTarget" => {
                        StartAutomationExecutionError::InvalidTarget(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartAutomationExecutionError::Validation(error_message.to_string())
                    }
                    _ => StartAutomationExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartAutomationExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartAutomationExecutionError {
    fn from(err: serde_json::error::Error) -> StartAutomationExecutionError {
        StartAutomationExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartAutomationExecutionError {
    fn from(err: CredentialsError) -> StartAutomationExecutionError {
        StartAutomationExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartAutomationExecutionError {
    fn from(err: HttpDispatchError) -> StartAutomationExecutionError {
        StartAutomationExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartAutomationExecutionError {
    fn from(err: io::Error) -> StartAutomationExecutionError {
        StartAutomationExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartAutomationExecutionError::AutomationDefinitionNotFound(ref cause) => cause,
            StartAutomationExecutionError::AutomationDefinitionVersionNotFound(ref cause) => cause,
            StartAutomationExecutionError::AutomationExecutionLimitExceeded(ref cause) => cause,
            StartAutomationExecutionError::IdempotentParameterMismatch(ref cause) => cause,
            StartAutomationExecutionError::InternalServerError(ref cause) => cause,
            StartAutomationExecutionError::InvalidAutomationExecutionParameters(ref cause) => cause,
            StartAutomationExecutionError::InvalidTarget(ref cause) => cause,
            StartAutomationExecutionError::Validation(ref cause) => cause,
            StartAutomationExecutionError::Credentials(ref err) => err.description(),
            StartAutomationExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartAutomationExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopAutomationExecution
#[derive(Debug, PartialEq)]
pub enum StopAutomationExecutionError {
    /// <p>There is no automation execution information for the requested automation execution ID.</p>
    AutomationExecutionNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified update status operation is not valid.</p>
    InvalidAutomationStatusUpdate(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopAutomationExecutionError {
    pub fn from_body(body: &str) -> StopAutomationExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AutomationExecutionNotFoundException" => {
                        StopAutomationExecutionError::AutomationExecutionNotFound(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => StopAutomationExecutionError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidAutomationStatusUpdateException" => {
                        StopAutomationExecutionError::InvalidAutomationStatusUpdate(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        StopAutomationExecutionError::Validation(error_message.to_string())
                    }
                    _ => StopAutomationExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopAutomationExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopAutomationExecutionError {
    fn from(err: serde_json::error::Error) -> StopAutomationExecutionError {
        StopAutomationExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopAutomationExecutionError {
    fn from(err: CredentialsError) -> StopAutomationExecutionError {
        StopAutomationExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopAutomationExecutionError {
    fn from(err: HttpDispatchError) -> StopAutomationExecutionError {
        StopAutomationExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopAutomationExecutionError {
    fn from(err: io::Error) -> StopAutomationExecutionError {
        StopAutomationExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopAutomationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAutomationExecutionError {
    fn description(&self) -> &str {
        match *self {
            StopAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,
            StopAutomationExecutionError::InternalServerError(ref cause) => cause,
            StopAutomationExecutionError::InvalidAutomationStatusUpdate(ref cause) => cause,
            StopAutomationExecutionError::Validation(ref cause) => cause,
            StopAutomationExecutionError::Credentials(ref err) => err.description(),
            StopAutomationExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopAutomationExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateAssociationError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>You have reached the maximum number versions allowed for an association. Each association has a limit of 1,000 versions. </p>
    AssociationVersionLimitExceeded(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The version you specified is not valid. Use ListAssociationVersions to view all versions of an association according to the association ID. Or, use the <code>$LATEST</code> parameter to view the latest version of the association.</p>
    InvalidAssociationVersion(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The output location is not valid or does not exist.</p>
    InvalidOutputLocation(String),
    /// <p>You must specify values for all required parameters in the Systems Manager document. You can only supply values to parameters defined in the Systems Manager document.</p>
    InvalidParameters(String),
    /// <p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
    InvalidSchedule(String),
    /// <p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
    InvalidTarget(String),
    /// <p>The update is not valid.</p>
    InvalidUpdate(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAssociationError {
    pub fn from_body(body: &str) -> UpdateAssociationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationDoesNotExist" => {
                        UpdateAssociationError::AssociationDoesNotExist(String::from(error_message))
                    }
                    "AssociationVersionLimitExceeded" => {
                        UpdateAssociationError::AssociationVersionLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        UpdateAssociationError::InternalServerError(String::from(error_message))
                    }
                    "InvalidAssociationVersion" => {
                        UpdateAssociationError::InvalidAssociationVersion(String::from(
                            error_message,
                        ))
                    }
                    "InvalidDocument" => {
                        UpdateAssociationError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentVersion" => {
                        UpdateAssociationError::InvalidDocumentVersion(String::from(error_message))
                    }
                    "InvalidOutputLocation" => {
                        UpdateAssociationError::InvalidOutputLocation(String::from(error_message))
                    }
                    "InvalidParameters" => {
                        UpdateAssociationError::InvalidParameters(String::from(error_message))
                    }
                    "InvalidSchedule" => {
                        UpdateAssociationError::InvalidSchedule(String::from(error_message))
                    }
                    "InvalidTarget" => {
                        UpdateAssociationError::InvalidTarget(String::from(error_message))
                    }
                    "InvalidUpdate" => {
                        UpdateAssociationError::InvalidUpdate(String::from(error_message))
                    }
                    "TooManyUpdates" => {
                        UpdateAssociationError::TooManyUpdates(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAssociationError::Validation(error_message.to_string())
                    }
                    _ => UpdateAssociationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAssociationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAssociationError {
    fn from(err: serde_json::error::Error) -> UpdateAssociationError {
        UpdateAssociationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAssociationError {
    fn from(err: CredentialsError) -> UpdateAssociationError {
        UpdateAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAssociationError {
    fn from(err: HttpDispatchError) -> UpdateAssociationError {
        UpdateAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAssociationError {
    fn from(err: io::Error) -> UpdateAssociationError {
        UpdateAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssociationError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssociationError::AssociationDoesNotExist(ref cause) => cause,
            UpdateAssociationError::AssociationVersionLimitExceeded(ref cause) => cause,
            UpdateAssociationError::InternalServerError(ref cause) => cause,
            UpdateAssociationError::InvalidAssociationVersion(ref cause) => cause,
            UpdateAssociationError::InvalidDocument(ref cause) => cause,
            UpdateAssociationError::InvalidDocumentVersion(ref cause) => cause,
            UpdateAssociationError::InvalidOutputLocation(ref cause) => cause,
            UpdateAssociationError::InvalidParameters(ref cause) => cause,
            UpdateAssociationError::InvalidSchedule(ref cause) => cause,
            UpdateAssociationError::InvalidTarget(ref cause) => cause,
            UpdateAssociationError::InvalidUpdate(ref cause) => cause,
            UpdateAssociationError::TooManyUpdates(ref cause) => cause,
            UpdateAssociationError::Validation(ref cause) => cause,
            UpdateAssociationError::Credentials(ref err) => err.description(),
            UpdateAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAssociationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssociationStatus
#[derive(Debug, PartialEq)]
pub enum UpdateAssociationStatusError {
    /// <p>The specified association does not exist.</p>
    AssociationDoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// <p>The updated status is the same as the current status.</p>
    StatusUnchanged(String),
    /// <p>There are concurrent updates for a resource that supports one update at a time.</p>
    TooManyUpdates(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAssociationStatusError {
    pub fn from_body(body: &str) -> UpdateAssociationStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AssociationDoesNotExist" => {
                        UpdateAssociationStatusError::AssociationDoesNotExist(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => UpdateAssociationStatusError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidDocument" => {
                        UpdateAssociationStatusError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidInstanceId" => {
                        UpdateAssociationStatusError::InvalidInstanceId(String::from(error_message))
                    }
                    "StatusUnchanged" => {
                        UpdateAssociationStatusError::StatusUnchanged(String::from(error_message))
                    }
                    "TooManyUpdates" => {
                        UpdateAssociationStatusError::TooManyUpdates(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAssociationStatusError::Validation(error_message.to_string())
                    }
                    _ => UpdateAssociationStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAssociationStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAssociationStatusError {
    fn from(err: serde_json::error::Error) -> UpdateAssociationStatusError {
        UpdateAssociationStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAssociationStatusError {
    fn from(err: CredentialsError) -> UpdateAssociationStatusError {
        UpdateAssociationStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAssociationStatusError {
    fn from(err: HttpDispatchError) -> UpdateAssociationStatusError {
        UpdateAssociationStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAssociationStatusError {
    fn from(err: io::Error) -> UpdateAssociationStatusError {
        UpdateAssociationStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAssociationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssociationStatusError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssociationStatusError::AssociationDoesNotExist(ref cause) => cause,
            UpdateAssociationStatusError::InternalServerError(ref cause) => cause,
            UpdateAssociationStatusError::InvalidDocument(ref cause) => cause,
            UpdateAssociationStatusError::InvalidInstanceId(ref cause) => cause,
            UpdateAssociationStatusError::StatusUnchanged(ref cause) => cause,
            UpdateAssociationStatusError::TooManyUpdates(ref cause) => cause,
            UpdateAssociationStatusError::Validation(ref cause) => cause,
            UpdateAssociationStatusError::Credentials(ref err) => err.description(),
            UpdateAssociationStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateAssociationStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocument
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentError {
    /// <p>The document has too many versions. Delete one or more document versions and try again.</p>
    DocumentVersionLimitExceeded(String),
    /// <p>The content of the association document matches another document. Change the content of the document and try again.</p>
    DuplicateDocumentContent(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The content for the document is not valid.</p>
    InvalidDocumentContent(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// <p>The size limit of a document is 64 KB.</p>
    MaxDocumentSizeExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDocumentError {
    pub fn from_body(body: &str) -> UpdateDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DocumentVersionLimitExceeded" => {
                        UpdateDocumentError::DocumentVersionLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "DuplicateDocumentContent" => {
                        UpdateDocumentError::DuplicateDocumentContent(String::from(error_message))
                    }
                    "InternalServerError" => {
                        UpdateDocumentError::InternalServerError(String::from(error_message))
                    }
                    "InvalidDocument" => {
                        UpdateDocumentError::InvalidDocument(String::from(error_message))
                    }
                    "InvalidDocumentContent" => {
                        UpdateDocumentError::InvalidDocumentContent(String::from(error_message))
                    }
                    "InvalidDocumentSchemaVersion" => {
                        UpdateDocumentError::InvalidDocumentSchemaVersion(String::from(
                            error_message,
                        ))
                    }
                    "InvalidDocumentVersion" => {
                        UpdateDocumentError::InvalidDocumentVersion(String::from(error_message))
                    }
                    "MaxDocumentSizeExceeded" => {
                        UpdateDocumentError::MaxDocumentSizeExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDocumentError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentError {
        UpdateDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentError {
    fn from(err: CredentialsError) -> UpdateDocumentError {
        UpdateDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentError {
    fn from(err: HttpDispatchError) -> UpdateDocumentError {
        UpdateDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentError {
    fn from(err: io::Error) -> UpdateDocumentError {
        UpdateDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentError::DocumentVersionLimitExceeded(ref cause) => cause,
            UpdateDocumentError::DuplicateDocumentContent(ref cause) => cause,
            UpdateDocumentError::InternalServerError(ref cause) => cause,
            UpdateDocumentError::InvalidDocument(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentContent(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,
            UpdateDocumentError::InvalidDocumentVersion(ref cause) => cause,
            UpdateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,
            UpdateDocumentError::Validation(ref cause) => cause,
            UpdateDocumentError::Credentials(ref err) => err.description(),
            UpdateDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentDefaultVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentDefaultVersionError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified document does not exist.</p>
    InvalidDocument(String),
    /// <p>The version of the document schema is not supported.</p>
    InvalidDocumentSchemaVersion(String),
    /// <p>The document version is not valid or does not exist.</p>
    InvalidDocumentVersion(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDocumentDefaultVersionError {
    pub fn from_body(body: &str) -> UpdateDocumentDefaultVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        UpdateDocumentDefaultVersionError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidDocument" => UpdateDocumentDefaultVersionError::InvalidDocument(
                        String::from(error_message),
                    ),
                    "InvalidDocumentSchemaVersion" => {
                        UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(
                            String::from(error_message),
                        )
                    }
                    "InvalidDocumentVersion" => {
                        UpdateDocumentDefaultVersionError::InvalidDocumentVersion(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateDocumentDefaultVersionError::Validation(error_message.to_string())
                    }
                    _ => UpdateDocumentDefaultVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDocumentDefaultVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDocumentDefaultVersionError {
    fn from(err: serde_json::error::Error) -> UpdateDocumentDefaultVersionError {
        UpdateDocumentDefaultVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDocumentDefaultVersionError {
    fn from(err: CredentialsError) -> UpdateDocumentDefaultVersionError {
        UpdateDocumentDefaultVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDocumentDefaultVersionError {
    fn from(err: HttpDispatchError) -> UpdateDocumentDefaultVersionError {
        UpdateDocumentDefaultVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDocumentDefaultVersionError {
    fn from(err: io::Error) -> UpdateDocumentDefaultVersionError {
        UpdateDocumentDefaultVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDocumentDefaultVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentDefaultVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentDefaultVersionError::InternalServerError(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocument(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(ref cause) => cause,
            UpdateDocumentDefaultVersionError::InvalidDocumentVersion(ref cause) => cause,
            UpdateDocumentDefaultVersionError::Validation(ref cause) => cause,
            UpdateDocumentDefaultVersionError::Credentials(ref err) => err.description(),
            UpdateDocumentDefaultVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDocumentDefaultVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindow
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateMaintenanceWindowError {
    pub fn from_body(body: &str) -> UpdateMaintenanceWindowError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        UpdateMaintenanceWindowError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => UpdateMaintenanceWindowError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        UpdateMaintenanceWindowError::Validation(error_message.to_string())
                    }
                    _ => UpdateMaintenanceWindowError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMaintenanceWindowError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMaintenanceWindowError {
    fn from(err: serde_json::error::Error) -> UpdateMaintenanceWindowError {
        UpdateMaintenanceWindowError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMaintenanceWindowError {
    fn from(err: CredentialsError) -> UpdateMaintenanceWindowError {
        UpdateMaintenanceWindowError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMaintenanceWindowError {
    fn from(err: HttpDispatchError) -> UpdateMaintenanceWindowError {
        UpdateMaintenanceWindowError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMaintenanceWindowError {
    fn from(err: io::Error) -> UpdateMaintenanceWindowError {
        UpdateMaintenanceWindowError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMaintenanceWindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowError::InternalServerError(ref cause) => cause,
            UpdateMaintenanceWindowError::Validation(ref cause) => cause,
            UpdateMaintenanceWindowError::Credentials(ref err) => err.description(),
            UpdateMaintenanceWindowError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMaintenanceWindowError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindowTarget
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowTargetError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateMaintenanceWindowTargetError {
    pub fn from_body(body: &str) -> UpdateMaintenanceWindowTargetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => UpdateMaintenanceWindowTargetError::DoesNotExist(
                        String::from(error_message),
                    ),
                    "InternalServerError" => {
                        UpdateMaintenanceWindowTargetError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateMaintenanceWindowTargetError::Validation(error_message.to_string())
                    }
                    _ => UpdateMaintenanceWindowTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMaintenanceWindowTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMaintenanceWindowTargetError {
    fn from(err: serde_json::error::Error) -> UpdateMaintenanceWindowTargetError {
        UpdateMaintenanceWindowTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMaintenanceWindowTargetError {
    fn from(err: CredentialsError) -> UpdateMaintenanceWindowTargetError {
        UpdateMaintenanceWindowTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMaintenanceWindowTargetError {
    fn from(err: HttpDispatchError) -> UpdateMaintenanceWindowTargetError {
        UpdateMaintenanceWindowTargetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMaintenanceWindowTargetError {
    fn from(err: io::Error) -> UpdateMaintenanceWindowTargetError {
        UpdateMaintenanceWindowTargetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMaintenanceWindowTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowTargetError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowTargetError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowTargetError::InternalServerError(ref cause) => cause,
            UpdateMaintenanceWindowTargetError::Validation(ref cause) => cause,
            UpdateMaintenanceWindowTargetError::Credentials(ref err) => err.description(),
            UpdateMaintenanceWindowTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMaintenanceWindowTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMaintenanceWindowTask
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceWindowTaskError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateMaintenanceWindowTaskError {
    pub fn from_body(body: &str) -> UpdateMaintenanceWindowTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        UpdateMaintenanceWindowTaskError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => UpdateMaintenanceWindowTaskError::InternalServerError(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        UpdateMaintenanceWindowTaskError::Validation(error_message.to_string())
                    }
                    _ => UpdateMaintenanceWindowTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMaintenanceWindowTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMaintenanceWindowTaskError {
    fn from(err: serde_json::error::Error) -> UpdateMaintenanceWindowTaskError {
        UpdateMaintenanceWindowTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMaintenanceWindowTaskError {
    fn from(err: CredentialsError) -> UpdateMaintenanceWindowTaskError {
        UpdateMaintenanceWindowTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMaintenanceWindowTaskError {
    fn from(err: HttpDispatchError) -> UpdateMaintenanceWindowTaskError {
        UpdateMaintenanceWindowTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMaintenanceWindowTaskError {
    fn from(err: io::Error) -> UpdateMaintenanceWindowTaskError {
        UpdateMaintenanceWindowTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMaintenanceWindowTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMaintenanceWindowTaskError {
    fn description(&self) -> &str {
        match *self {
            UpdateMaintenanceWindowTaskError::DoesNotExist(ref cause) => cause,
            UpdateMaintenanceWindowTaskError::InternalServerError(ref cause) => cause,
            UpdateMaintenanceWindowTaskError::Validation(ref cause) => cause,
            UpdateMaintenanceWindowTaskError::Credentials(ref err) => err.description(),
            UpdateMaintenanceWindowTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateMaintenanceWindowTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateManagedInstanceRole
#[derive(Debug, PartialEq)]
pub enum UpdateManagedInstanceRoleError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
    InvalidInstanceId(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateManagedInstanceRoleError {
    pub fn from_body(body: &str) -> UpdateManagedInstanceRoleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => UpdateManagedInstanceRoleError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidInstanceId" => UpdateManagedInstanceRoleError::InvalidInstanceId(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        UpdateManagedInstanceRoleError::Validation(error_message.to_string())
                    }
                    _ => UpdateManagedInstanceRoleError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateManagedInstanceRoleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateManagedInstanceRoleError {
    fn from(err: serde_json::error::Error) -> UpdateManagedInstanceRoleError {
        UpdateManagedInstanceRoleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateManagedInstanceRoleError {
    fn from(err: CredentialsError) -> UpdateManagedInstanceRoleError {
        UpdateManagedInstanceRoleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateManagedInstanceRoleError {
    fn from(err: HttpDispatchError) -> UpdateManagedInstanceRoleError {
        UpdateManagedInstanceRoleError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateManagedInstanceRoleError {
    fn from(err: io::Error) -> UpdateManagedInstanceRoleError {
        UpdateManagedInstanceRoleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateManagedInstanceRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateManagedInstanceRoleError {
    fn description(&self) -> &str {
        match *self {
            UpdateManagedInstanceRoleError::InternalServerError(ref cause) => cause,
            UpdateManagedInstanceRoleError::InvalidInstanceId(ref cause) => cause,
            UpdateManagedInstanceRoleError::Validation(ref cause) => cause,
            UpdateManagedInstanceRoleError::Credentials(ref err) => err.description(),
            UpdateManagedInstanceRoleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateManagedInstanceRoleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePatchBaseline
#[derive(Debug, PartialEq)]
pub enum UpdatePatchBaselineError {
    /// <p>Error returned when the ID specified for a resource, such as a Maintenance Window or Patch baseline, doesn't exist.</p> <p>For information about resource limits in Systems Manager, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_ssm">AWS Systems Manager Limits</a>.</p>
    DoesNotExist(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdatePatchBaselineError {
    pub fn from_body(body: &str) -> UpdatePatchBaselineError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DoesNotExistException" => {
                        UpdatePatchBaselineError::DoesNotExist(String::from(error_message))
                    }
                    "InternalServerError" => {
                        UpdatePatchBaselineError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdatePatchBaselineError::Validation(error_message.to_string())
                    }
                    _ => UpdatePatchBaselineError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdatePatchBaselineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdatePatchBaselineError {
    fn from(err: serde_json::error::Error) -> UpdatePatchBaselineError {
        UpdatePatchBaselineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePatchBaselineError {
    fn from(err: CredentialsError) -> UpdatePatchBaselineError {
        UpdatePatchBaselineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePatchBaselineError {
    fn from(err: HttpDispatchError) -> UpdatePatchBaselineError {
        UpdatePatchBaselineError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePatchBaselineError {
    fn from(err: io::Error) -> UpdatePatchBaselineError {
        UpdatePatchBaselineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePatchBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePatchBaselineError {
    fn description(&self) -> &str {
        match *self {
            UpdatePatchBaselineError::DoesNotExist(ref cause) => cause,
            UpdatePatchBaselineError::InternalServerError(ref cause) => cause,
            UpdatePatchBaselineError::Validation(ref cause) => cause,
            UpdatePatchBaselineError::Credentials(ref err) => err.description(),
            UpdatePatchBaselineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdatePatchBaselineError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon SSM API. Amazon SSM clients implement this trait.
pub trait Ssm {
    /// <p>Adds or overwrites one or more tags for the specified resource. Tags are metadata that you can assign to your documents, managed instances, Maintenance Windows, Parameter Store parameters, and patch baselines. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. For example, you could define a set of tags for your account's managed instances that helps you track each instance's owner and stack level. For example: Key=Owner and Value=DbAdmin, SysAdmin, or Dev. Or Key=Stack and Value=Production, Pre-Production, or Test.</p> <p>Each resource can have a maximum of 50 tags. </p> <p>We recommend that you devise a set of tag keys that meets your needs for each resource type. Using a consistent set of tag keys makes it easier for you to manage your resources. You can search and filter the resources based on the tags you add. Tags don't have any semantic meaning to Amazon EC2 and are interpreted strictly as a string of characters. </p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError>;

    /// <p>Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped.</p>
    fn cancel_command(
        &self,
        input: CancelCommandRequest,
    ) -> RusotoFuture<CancelCommandResult, CancelCommandError>;

    /// <p>Registers your on-premises server or virtual machine with Amazon EC2 so that you can manage these resources using Run Command. An on-premises server or virtual machine that has been registered with EC2 is called a managed instance. For more information about activations, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances.html">Setting Up Systems Manager in Hybrid Environments</a>.</p>
    fn create_activation(
        &self,
        input: CreateActivationRequest,
    ) -> RusotoFuture<CreateActivationResult, CreateActivationError>;

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>
    fn create_association(
        &self,
        input: CreateAssociationRequest,
    ) -> RusotoFuture<CreateAssociationResult, CreateAssociationError>;

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>
    fn create_association_batch(
        &self,
        input: CreateAssociationBatchRequest,
    ) -> RusotoFuture<CreateAssociationBatchResult, CreateAssociationBatchError>;

    /// <p>Creates a Systems Manager document.</p> <p>After you create a document, you can use CreateAssociation to associate it with one or more running instances.</p>
    fn create_document(
        &self,
        input: CreateDocumentRequest,
    ) -> RusotoFuture<CreateDocumentResult, CreateDocumentError>;

    /// <p>Creates a new Maintenance Window.</p>
    fn create_maintenance_window(
        &self,
        input: CreateMaintenanceWindowRequest,
    ) -> RusotoFuture<CreateMaintenanceWindowResult, CreateMaintenanceWindowError>;

    /// <p><p>Creates a patch baseline.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn create_patch_baseline(
        &self,
        input: CreatePatchBaselineRequest,
    ) -> RusotoFuture<CreatePatchBaselineResult, CreatePatchBaselineError>;

    /// <p>Creates a resource data sync configuration to a single bucket in Amazon S3. This is an asynchronous operation that returns immediately. After a successful initial sync is completed, the system continuously syncs data to the Amazon S3 bucket. To check the status of the sync, use the <a>ListResourceDataSync</a>.</p> <p>By default, data is not encrypted in Amazon S3. We strongly recommend that you enable encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure access to the Amazon S3 bucket by creating a restrictive bucket policy. To view an example of a restrictive Amazon S3 bucket policy for Resource Data Sync, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-configuring.html#sysman-inventory-datasync">Configuring Resource Data Sync for Inventory</a>.</p>
    fn create_resource_data_sync(
        &self,
        input: CreateResourceDataSyncRequest,
    ) -> RusotoFuture<CreateResourceDataSyncResult, CreateResourceDataSyncError>;

    /// <p>Deletes an activation. You are not required to delete an activation. If you delete an activation, you can no longer use it to register additional managed instances. Deleting an activation does not de-register managed instances. You must manually de-register managed instances.</p>
    fn delete_activation(
        &self,
        input: DeleteActivationRequest,
    ) -> RusotoFuture<DeleteActivationResult, DeleteActivationError>;

    /// <p>Disassociates the specified Systems Manager document from the specified instance.</p> <p>When you disassociate a document from an instance, it does not change the configuration of the instance. To change the configuration state of an instance after you disassociate a document, you must create a new document with the desired configuration and associate it with the instance.</p>
    fn delete_association(
        &self,
        input: DeleteAssociationRequest,
    ) -> RusotoFuture<DeleteAssociationResult, DeleteAssociationError>;

    /// <p>Deletes the Systems Manager document and all instance associations to the document.</p> <p>Before you delete the document, we recommend that you use <a>DeleteAssociation</a> to disassociate all instances that are associated with the document.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<DeleteDocumentResult, DeleteDocumentError>;

    /// <p>Deletes a Maintenance Window.</p>
    fn delete_maintenance_window(
        &self,
        input: DeleteMaintenanceWindowRequest,
    ) -> RusotoFuture<DeleteMaintenanceWindowResult, DeleteMaintenanceWindowError>;

    /// <p>Delete a parameter from the system.</p>
    fn delete_parameter(
        &self,
        input: DeleteParameterRequest,
    ) -> RusotoFuture<DeleteParameterResult, DeleteParameterError>;

    /// <p>Delete a list of parameters. This API is used to delete parameters by using the Amazon EC2 console.</p>
    fn delete_parameters(
        &self,
        input: DeleteParametersRequest,
    ) -> RusotoFuture<DeleteParametersResult, DeleteParametersError>;

    /// <p>Deletes a patch baseline.</p>
    fn delete_patch_baseline(
        &self,
        input: DeletePatchBaselineRequest,
    ) -> RusotoFuture<DeletePatchBaselineResult, DeletePatchBaselineError>;

    /// <p>Deletes a Resource Data Sync configuration. After the configuration is deleted, changes to inventory data on managed instances are no longer synced with the target Amazon S3 bucket. Deleting a sync configuration does not delete data in the target Amazon S3 bucket.</p>
    fn delete_resource_data_sync(
        &self,
        input: DeleteResourceDataSyncRequest,
    ) -> RusotoFuture<DeleteResourceDataSyncResult, DeleteResourceDataSyncError>;

    /// <p>Removes the server or virtual machine from the list of registered servers. You can reregister the instance again at any time. If you don't plan to use Run Command on the server, we suggest uninstalling the SSM Agent first.</p>
    fn deregister_managed_instance(
        &self,
        input: DeregisterManagedInstanceRequest,
    ) -> RusotoFuture<DeregisterManagedInstanceResult, DeregisterManagedInstanceError>;

    /// <p>Removes a patch group from a patch baseline.</p>
    fn deregister_patch_baseline_for_patch_group(
        &self,
        input: DeregisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        DeregisterPatchBaselineForPatchGroupResult,
        DeregisterPatchBaselineForPatchGroupError,
    >;

    /// <p>Removes a target from a Maintenance Window.</p>
    fn deregister_target_from_maintenance_window(
        &self,
        input: DeregisterTargetFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTargetFromMaintenanceWindowResult,
        DeregisterTargetFromMaintenanceWindowError,
    >;

    /// <p>Removes a task from a Maintenance Window.</p>
    fn deregister_task_from_maintenance_window(
        &self,
        input: DeregisterTaskFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTaskFromMaintenanceWindowResult,
        DeregisterTaskFromMaintenanceWindowError,
    >;

    /// <p>Details about the activation, including: the date and time the activation was created, the expiration date, the IAM role assigned to the instances in the activation, and the number of instances activated by this registration.</p>
    fn describe_activations(
        &self,
        input: DescribeActivationsRequest,
    ) -> RusotoFuture<DescribeActivationsResult, DescribeActivationsError>;

    /// <p>Describes the association for the specified target or instance. If you created the association by using the <code>Targets</code> parameter, then you must retrieve the association by using the association ID. If you created the association by specifying an instance ID and a Systems Manager document, then you retrieve the association by specifying the document name and the instance ID. </p>
    fn describe_association(
        &self,
        input: DescribeAssociationRequest,
    ) -> RusotoFuture<DescribeAssociationResult, DescribeAssociationError>;

    /// <p>Provides details about all active and terminated Automation executions.</p>
    fn describe_automation_executions(
        &self,
        input: DescribeAutomationExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationExecutionsResult, DescribeAutomationExecutionsError>;

    /// <p>Information about all active and terminated step executions in an Automation workflow.</p>
    fn describe_automation_step_executions(
        &self,
        input: DescribeAutomationStepExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationStepExecutionsResult, DescribeAutomationStepExecutionsError>;

    /// <p>Lists all patches that could possibly be included in a patch baseline.</p>
    fn describe_available_patches(
        &self,
        input: DescribeAvailablePatchesRequest,
    ) -> RusotoFuture<DescribeAvailablePatchesResult, DescribeAvailablePatchesError>;

    /// <p>Describes the specified Systems Manager document.</p>
    fn describe_document(
        &self,
        input: DescribeDocumentRequest,
    ) -> RusotoFuture<DescribeDocumentResult, DescribeDocumentError>;

    /// <p>Describes the permissions for a Systems Manager document. If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a user's AWS account ID) or publicly (<i>All</i>). </p>
    fn describe_document_permission(
        &self,
        input: DescribeDocumentPermissionRequest,
    ) -> RusotoFuture<DescribeDocumentPermissionResponse, DescribeDocumentPermissionError>;

    /// <p>All associations for the instance(s).</p>
    fn describe_effective_instance_associations(
        &self,
        input: DescribeEffectiveInstanceAssociationsRequest,
    ) -> RusotoFuture<
        DescribeEffectiveInstanceAssociationsResult,
        DescribeEffectiveInstanceAssociationsError,
    >;

    /// <p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Note that this API applies only to Windows patch baselines.</p>
    fn describe_effective_patches_for_patch_baseline(
        &self,
        input: DescribeEffectivePatchesForPatchBaselineRequest,
    ) -> RusotoFuture<
        DescribeEffectivePatchesForPatchBaselineResult,
        DescribeEffectivePatchesForPatchBaselineError,
    >;

    /// <p>The status of the associations for the instance(s).</p>
    fn describe_instance_associations_status(
        &self,
        input: DescribeInstanceAssociationsStatusRequest,
    ) -> RusotoFuture<
        DescribeInstanceAssociationsStatusResult,
        DescribeInstanceAssociationsStatusError,
    >;

    /// <p>Describes one or more of your instances. You can use this to get information about instances like the operating system platform, the SSM Agent version (Linux), status etc. If you specify one or more instance IDs, it returns information for those instances. If you do not specify instance IDs, it returns information for all your instances. If you specify an instance ID that is not valid or an instance that you do not own, you receive an error. </p>
    fn describe_instance_information(
        &self,
        input: DescribeInstanceInformationRequest,
    ) -> RusotoFuture<DescribeInstanceInformationResult, DescribeInstanceInformationError>;

    /// <p>Retrieves the high-level patch state of one or more instances.</p>
    fn describe_instance_patch_states(
        &self,
        input: DescribeInstancePatchStatesRequest,
    ) -> RusotoFuture<DescribeInstancePatchStatesResult, DescribeInstancePatchStatesError>;

    /// <p>Retrieves the high-level patch state for the instances in the specified patch group.</p>
    fn describe_instance_patch_states_for_patch_group(
        &self,
        input: DescribeInstancePatchStatesForPatchGroupRequest,
    ) -> RusotoFuture<
        DescribeInstancePatchStatesForPatchGroupResult,
        DescribeInstancePatchStatesForPatchGroupError,
    >;

    /// <p>Retrieves information about the patches on the specified instance and their state relative to the patch baseline being used for the instance.</p>
    fn describe_instance_patches(
        &self,
        input: DescribeInstancePatchesRequest,
    ) -> RusotoFuture<DescribeInstancePatchesResult, DescribeInstancePatchesError>;

    /// <p>Retrieves the individual task executions (one per target) for a particular task executed as part of a Maintenance Window execution.</p>
    fn describe_maintenance_window_execution_task_invocations(
        &self,
        input: DescribeMaintenanceWindowExecutionTaskInvocationsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTaskInvocationsResult,
        DescribeMaintenanceWindowExecutionTaskInvocationsError,
    >;

    /// <p>For a given Maintenance Window execution, lists the tasks that were executed.</p>
    fn describe_maintenance_window_execution_tasks(
        &self,
        input: DescribeMaintenanceWindowExecutionTasksRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTasksResult,
        DescribeMaintenanceWindowExecutionTasksError,
    >;

    /// <p>Lists the executions of a Maintenance Window. This includes information about when the Maintenance Window was scheduled to be active, and information about tasks registered and run with the Maintenance Window.</p>
    fn describe_maintenance_window_executions(
        &self,
        input: DescribeMaintenanceWindowExecutionsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionsResult,
        DescribeMaintenanceWindowExecutionsError,
    >;

    /// <p>Lists the targets registered with the Maintenance Window.</p>
    fn describe_maintenance_window_targets(
        &self,
        input: DescribeMaintenanceWindowTargetsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTargetsResult, DescribeMaintenanceWindowTargetsError>;

    /// <p>Lists the tasks in a Maintenance Window.</p>
    fn describe_maintenance_window_tasks(
        &self,
        input: DescribeMaintenanceWindowTasksRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTasksResult, DescribeMaintenanceWindowTasksError>;

    /// <p>Retrieves the Maintenance Windows in an AWS account.</p>
    fn describe_maintenance_windows(
        &self,
        input: DescribeMaintenanceWindowsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowsResult, DescribeMaintenanceWindowsError>;

    /// <p>Get information about a parameter.</p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResult, DescribeParametersError>;

    /// <p>Lists the patch baselines in your AWS account.</p>
    fn describe_patch_baselines(
        &self,
        input: DescribePatchBaselinesRequest,
    ) -> RusotoFuture<DescribePatchBaselinesResult, DescribePatchBaselinesError>;

    /// <p>Returns high-level aggregated patch compliance state for a patch group.</p>
    fn describe_patch_group_state(
        &self,
        input: DescribePatchGroupStateRequest,
    ) -> RusotoFuture<DescribePatchGroupStateResult, DescribePatchGroupStateError>;

    /// <p>Lists all patch groups that have been registered with patch baselines.</p>
    fn describe_patch_groups(
        &self,
        input: DescribePatchGroupsRequest,
    ) -> RusotoFuture<DescribePatchGroupsResult, DescribePatchGroupsError>;

    /// <p>Get detailed information about a particular Automation execution.</p>
    fn get_automation_execution(
        &self,
        input: GetAutomationExecutionRequest,
    ) -> RusotoFuture<GetAutomationExecutionResult, GetAutomationExecutionError>;

    /// <p>Returns detailed information about command execution for an invocation or plugin. </p>
    fn get_command_invocation(
        &self,
        input: GetCommandInvocationRequest,
    ) -> RusotoFuture<GetCommandInvocationResult, GetCommandInvocationError>;

    /// <p>Retrieves the default patch baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p> <p>If you do not specify an operating system value, the default patch baseline for Windows is returned.</p>
    fn get_default_patch_baseline(
        &self,
        input: GetDefaultPatchBaselineRequest,
    ) -> RusotoFuture<GetDefaultPatchBaselineResult, GetDefaultPatchBaselineError>;

    /// <p>Retrieves the current snapshot for the patch baseline the instance uses. This API is primarily used by the AWS-RunPatchBaseline Systems Manager document. </p>
    fn get_deployable_patch_snapshot_for_instance(
        &self,
        input: GetDeployablePatchSnapshotForInstanceRequest,
    ) -> RusotoFuture<
        GetDeployablePatchSnapshotForInstanceResult,
        GetDeployablePatchSnapshotForInstanceError,
    >;

    /// <p>Gets the contents of the specified Systems Manager document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResult, GetDocumentError>;

    /// <p>Query inventory information.</p>
    fn get_inventory(
        &self,
        input: GetInventoryRequest,
    ) -> RusotoFuture<GetInventoryResult, GetInventoryError>;

    /// <p>Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. </p>
    fn get_inventory_schema(
        &self,
        input: GetInventorySchemaRequest,
    ) -> RusotoFuture<GetInventorySchemaResult, GetInventorySchemaError>;

    /// <p>Retrieves a Maintenance Window.</p>
    fn get_maintenance_window(
        &self,
        input: GetMaintenanceWindowRequest,
    ) -> RusotoFuture<GetMaintenanceWindowResult, GetMaintenanceWindowError>;

    /// <p>Retrieves details about a specific task executed as part of a Maintenance Window execution.</p>
    fn get_maintenance_window_execution(
        &self,
        input: GetMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionResult, GetMaintenanceWindowExecutionError>;

    /// <p>Retrieves the details about a specific task executed as part of a Maintenance Window execution.</p>
    fn get_maintenance_window_execution_task(
        &self,
        input: GetMaintenanceWindowExecutionTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionTaskResult, GetMaintenanceWindowExecutionTaskError>;

    /// <p>Retrieves a task invocation. A task invocation is a specific task executing on a specific target. Maintenance Windows report status for all invocations. </p>
    fn get_maintenance_window_execution_task_invocation(
        &self,
        input: GetMaintenanceWindowExecutionTaskInvocationRequest,
    ) -> RusotoFuture<
        GetMaintenanceWindowExecutionTaskInvocationResult,
        GetMaintenanceWindowExecutionTaskInvocationError,
    >;

    /// <p>Lists the tasks in a Maintenance Window.</p>
    fn get_maintenance_window_task(
        &self,
        input: GetMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowTaskResult, GetMaintenanceWindowTaskError>;

    /// <p>Get information about a parameter by using the parameter name. </p>
    fn get_parameter(
        &self,
        input: GetParameterRequest,
    ) -> RusotoFuture<GetParameterResult, GetParameterError>;

    /// <p>Query a list of all parameters used by the AWS account.</p>
    fn get_parameter_history(
        &self,
        input: GetParameterHistoryRequest,
    ) -> RusotoFuture<GetParameterHistoryResult, GetParameterHistoryError>;

    /// <p>Get details of a parameter.</p>
    fn get_parameters(
        &self,
        input: GetParametersRequest,
    ) -> RusotoFuture<GetParametersResult, GetParametersError>;

    /// <p><p>Retrieve parameters in a specific hierarchy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-working.html">Working with Systems Manager Parameters</a>. </p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p> <note> <p>This API action doesn&#39;t support filtering by tags. </p> </note></p>
    fn get_parameters_by_path(
        &self,
        input: GetParametersByPathRequest,
    ) -> RusotoFuture<GetParametersByPathResult, GetParametersByPathError>;

    /// <p>Retrieves information about a patch baseline.</p>
    fn get_patch_baseline(
        &self,
        input: GetPatchBaselineRequest,
    ) -> RusotoFuture<GetPatchBaselineResult, GetPatchBaselineError>;

    /// <p>Retrieves the patch baseline that should be used for the specified patch group.</p>
    fn get_patch_baseline_for_patch_group(
        &self,
        input: GetPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<GetPatchBaselineForPatchGroupResult, GetPatchBaselineForPatchGroupError>;

    /// <p>Retrieves all versions of an association for a specific association ID.</p>
    fn list_association_versions(
        &self,
        input: ListAssociationVersionsRequest,
    ) -> RusotoFuture<ListAssociationVersionsResult, ListAssociationVersionsError>;

    /// <p>Lists the associations for the specified Systems Manager document or instance.</p>
    fn list_associations(
        &self,
        input: ListAssociationsRequest,
    ) -> RusotoFuture<ListAssociationsResult, ListAssociationsError>;

    /// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user executes SendCommand against three instances, then a command invocation is created for each requested instance ID. ListCommandInvocations provide status about command execution.</p>
    fn list_command_invocations(
        &self,
        input: ListCommandInvocationsRequest,
    ) -> RusotoFuture<ListCommandInvocationsResult, ListCommandInvocationsError>;

    /// <p>Lists the commands requested by users of the AWS account.</p>
    fn list_commands(
        &self,
        input: ListCommandsRequest,
    ) -> RusotoFuture<ListCommandsResult, ListCommandsError>;

    /// <p>For a specified resource ID, this API action returns a list of compliance statuses for different resource types. Currently, you can only specify one resource ID per call. List results depend on the criteria specified in the filter. </p>
    fn list_compliance_items(
        &self,
        input: ListComplianceItemsRequest,
    ) -> RusotoFuture<ListComplianceItemsResult, ListComplianceItemsError>;

    /// <p>Returns a summary count of compliant and non-compliant resources for a compliance type. For example, this call can return State Manager associations, patches, or custom compliance types according to the filter criteria that you specify. </p>
    fn list_compliance_summaries(
        &self,
        input: ListComplianceSummariesRequest,
    ) -> RusotoFuture<ListComplianceSummariesResult, ListComplianceSummariesError>;

    /// <p>List all versions for a document.</p>
    fn list_document_versions(
        &self,
        input: ListDocumentVersionsRequest,
    ) -> RusotoFuture<ListDocumentVersionsResult, ListDocumentVersionsError>;

    /// <p>Describes one or more of your Systems Manager documents.</p>
    fn list_documents(
        &self,
        input: ListDocumentsRequest,
    ) -> RusotoFuture<ListDocumentsResult, ListDocumentsError>;

    /// <p>A list of inventory items returned by the request.</p>
    fn list_inventory_entries(
        &self,
        input: ListInventoryEntriesRequest,
    ) -> RusotoFuture<ListInventoryEntriesResult, ListInventoryEntriesError>;

    /// <p>Returns a resource-level summary count. The summary includes information about compliant and non-compliant statuses and detailed compliance-item severity counts, according to the filter criteria you specify.</p>
    fn list_resource_compliance_summaries(
        &self,
        input: ListResourceComplianceSummariesRequest,
    ) -> RusotoFuture<ListResourceComplianceSummariesResult, ListResourceComplianceSummariesError>;

    /// <p>Lists your resource data sync configurations. Includes information about the last time a sync attempted to start, the last sync status, and the last time a sync successfully completed.</p> <p>The number of sync configurations might be too large to return using a single call to <code>ListResourceDataSync</code>. You can limit the number of sync configurations returned by using the <code>MaxResults</code> parameter. To determine whether there are more sync configurations to list, check the value of <code>NextToken</code> in the output. If there are more sync configurations to list, you can request them by specifying the <code>NextToken</code> returned in the call to the parameter of a subsequent call. </p>
    fn list_resource_data_sync(
        &self,
        input: ListResourceDataSyncRequest,
    ) -> RusotoFuture<ListResourceDataSyncResult, ListResourceDataSyncError>;

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError>;

    /// <p>Shares a Systems Manager document publicly or privately. If you share a document privately, you must specify the AWS user account IDs for those people who can use the document. If you share a document publicly, you must specify <i>All</i> as the account ID.</p>
    fn modify_document_permission(
        &self,
        input: ModifyDocumentPermissionRequest,
    ) -> RusotoFuture<ModifyDocumentPermissionResponse, ModifyDocumentPermissionError>;

    /// <p><p>Registers a compliance type and other compliance details on a designated resource. This action lets you register custom compliance details with a resource. This call overwrites existing compliance information on the resource, so you must provide a full list of compliance items each time that you send the request.</p> <p>ComplianceType can be one of the following:</p> <ul> <li> <p>ExecutionId: The execution ID when the patch, association, or custom compliance item was applied.</p> </li> <li> <p>ExecutionType: Specify patch, association, or Custom:<code>string</code>.</p> </li> <li> <p>ExecutionTime. The time the patch, association, or custom compliance item was applied to the instance.</p> </li> <li> <p>Id: The patch, association, or custom compliance ID.</p> </li> <li> <p>Title: A title.</p> </li> <li> <p>Status: The status of the compliance item. For example, <code>approved</code> for patches, or <code>Failed</code> for associations.</p> </li> <li> <p>Severity: A patch severity. For example, <code>critical</code>.</p> </li> <li> <p>DocumentName: A SSM document name. For example, AWS-RunPatchBaseline.</p> </li> <li> <p>DocumentVersion: An SSM document version number. For example, 4.</p> </li> <li> <p>Classification: A patch classification. For example, <code>security updates</code>.</p> </li> <li> <p>PatchBaselineId: A patch baseline ID.</p> </li> <li> <p>PatchSeverity: A patch severity. For example, <code>Critical</code>.</p> </li> <li> <p>PatchState: A patch state. For example, <code>InstancesWithFailedPatches</code>.</p> </li> <li> <p>PatchGroup: The name of a patch group.</p> </li> <li> <p>InstalledTime: The time the association, patch, or custom compliance item was applied to the resource. Specify the time by using the following format: yyyy-MM-dd&#39;T&#39;HH:mm:ss&#39;Z&#39;</p> </li> </ul></p>
    fn put_compliance_items(
        &self,
        input: PutComplianceItemsRequest,
    ) -> RusotoFuture<PutComplianceItemsResult, PutComplianceItemsError>;

    /// <p>Bulk update custom inventory items on one more instance. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist.</p>
    fn put_inventory(
        &self,
        input: PutInventoryRequest,
    ) -> RusotoFuture<PutInventoryResult, PutInventoryError>;

    /// <p>Add one or more parameters to the system.</p>
    fn put_parameter(
        &self,
        input: PutParameterRequest,
    ) -> RusotoFuture<PutParameterResult, PutParameterError>;

    /// <p>Defines the default patch baseline.</p>
    fn register_default_patch_baseline(
        &self,
        input: RegisterDefaultPatchBaselineRequest,
    ) -> RusotoFuture<RegisterDefaultPatchBaselineResult, RegisterDefaultPatchBaselineError>;

    /// <p>Registers a patch baseline for a patch group.</p>
    fn register_patch_baseline_for_patch_group(
        &self,
        input: RegisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        RegisterPatchBaselineForPatchGroupResult,
        RegisterPatchBaselineForPatchGroupError,
    >;

    /// <p>Registers a target with a Maintenance Window.</p>
    fn register_target_with_maintenance_window(
        &self,
        input: RegisterTargetWithMaintenanceWindowRequest,
    ) -> RusotoFuture<
        RegisterTargetWithMaintenanceWindowResult,
        RegisterTargetWithMaintenanceWindowError,
    >;

    /// <p>Adds a new task to a Maintenance Window.</p>
    fn register_task_with_maintenance_window(
        &self,
        input: RegisterTaskWithMaintenanceWindowRequest,
    ) -> RusotoFuture<RegisterTaskWithMaintenanceWindowResult, RegisterTaskWithMaintenanceWindowError>;

    /// <p>Removes all tags from the specified resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError>;

    /// <p>Sends a signal to an Automation execution to change the current behavior or status of the execution. </p>
    fn send_automation_signal(
        &self,
        input: SendAutomationSignalRequest,
    ) -> RusotoFuture<SendAutomationSignalResult, SendAutomationSignalError>;

    /// <p>Executes commands on one or more managed instances.</p>
    fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> RusotoFuture<SendCommandResult, SendCommandError>;

    /// <p>Initiates execution of an Automation document.</p>
    fn start_automation_execution(
        &self,
        input: StartAutomationExecutionRequest,
    ) -> RusotoFuture<StartAutomationExecutionResult, StartAutomationExecutionError>;

    /// <p>Stop an Automation that is currently executing.</p>
    fn stop_automation_execution(
        &self,
        input: StopAutomationExecutionRequest,
    ) -> RusotoFuture<StopAutomationExecutionResult, StopAutomationExecutionError>;

    /// <p>Updates an association. You can update the association name and version, the document version, schedule, parameters, and Amazon S3 output.</p>
    fn update_association(
        &self,
        input: UpdateAssociationRequest,
    ) -> RusotoFuture<UpdateAssociationResult, UpdateAssociationError>;

    /// <p>Updates the status of the Systems Manager document associated with the specified instance.</p>
    fn update_association_status(
        &self,
        input: UpdateAssociationStatusRequest,
    ) -> RusotoFuture<UpdateAssociationStatusResult, UpdateAssociationStatusError>;

    /// <p>The document you want to update.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<UpdateDocumentResult, UpdateDocumentError>;

    /// <p>Set the default version of a document. </p>
    fn update_document_default_version(
        &self,
        input: UpdateDocumentDefaultVersionRequest,
    ) -> RusotoFuture<UpdateDocumentDefaultVersionResult, UpdateDocumentDefaultVersionError>;

    /// <p>Updates an existing Maintenance Window. Only specified parameters are modified.</p>
    fn update_maintenance_window(
        &self,
        input: UpdateMaintenanceWindowRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowResult, UpdateMaintenanceWindowError>;

    /// <p>Modifies the target of an existing Maintenance Window. You can't change the target type, but you can change the following:</p> <p>The target from being an ID target to a Tag target, or a Tag target to an ID target.</p> <p>IDs for an ID target.</p> <p>Tags for a Tag target.</p> <p>Owner.</p> <p>Name.</p> <p>Description.</p> <p>If a parameter is null, then the corresponding field is not modified.</p>
    fn update_maintenance_window_target(
        &self,
        input: UpdateMaintenanceWindowTargetRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTargetResult, UpdateMaintenanceWindowTargetError>;

    /// <p>Modifies a task assigned to a Maintenance Window. You can't change the task type, but you can change the following values:</p> <ul> <li> <p>TaskARN. For example, you can change a RUN_COMMAND task from AWS-RunPowerShellScript to AWS-RunShellScript.</p> </li> <li> <p>ServiceRoleArn</p> </li> <li> <p>TaskInvocationParameters</p> </li> <li> <p>Priority</p> </li> <li> <p>MaxConcurrency</p> </li> <li> <p>MaxErrors</p> </li> </ul> <p>If a parameter is null, then the corresponding field is not modified. Also, if you set Replace to true, then all fields required by the <a>RegisterTaskWithMaintenanceWindow</a> action are required for this request. Optional fields that aren't specified are set to null.</p>
    fn update_maintenance_window_task(
        &self,
        input: UpdateMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTaskResult, UpdateMaintenanceWindowTaskError>;

    /// <p>Assigns or changes an Amazon Identity and Access Management (IAM) role to the managed instance.</p>
    fn update_managed_instance_role(
        &self,
        input: UpdateManagedInstanceRoleRequest,
    ) -> RusotoFuture<UpdateManagedInstanceRoleResult, UpdateManagedInstanceRoleError>;

    /// <p><p>Modifies an existing patch baseline. Fields not specified in the request are left unchanged.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn update_patch_baseline(
        &self,
        input: UpdatePatchBaselineRequest,
    ) -> RusotoFuture<UpdatePatchBaselineResult, UpdatePatchBaselineError>;
}
/// A client for the Amazon SSM API.
pub struct SsmClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SsmClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SsmClient {
        SsmClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SsmClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SsmClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Ssm for SsmClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds or overwrites one or more tags for the specified resource. Tags are metadata that you can assign to your documents, managed instances, Maintenance Windows, Parameter Store parameters, and patch baselines. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. For example, you could define a set of tags for your account's managed instances that helps you track each instance's owner and stack level. For example: Key=Owner and Value=DbAdmin, SysAdmin, or Dev. Or Key=Stack and Value=Production, Pre-Production, or Test.</p> <p>Each resource can have a maximum of 50 tags. </p> <p>We recommend that you devise a set of tag keys that meets your needs for each resource type. Using a consistent set of tag keys makes it easier for you to manage your resources. You can search and filter the resources based on the tags you add. Tags don't have any semantic meaning to Amazon EC2 and are interpreted strictly as a string of characters. </p> <p>For more information about tags, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceRequest,
    ) -> RusotoFuture<AddTagsToResourceResult, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddTagsToResourceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsToResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped.</p>
    fn cancel_command(
        &self,
        input: CancelCommandRequest,
    ) -> RusotoFuture<CancelCommandResult, CancelCommandError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CancelCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelCommandResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelCommandError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers your on-premises server or virtual machine with Amazon EC2 so that you can manage these resources using Run Command. An on-premises server or virtual machine that has been registered with EC2 is called a managed instance. For more information about activations, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances.html">Setting Up Systems Manager in Hybrid Environments</a>.</p>
    fn create_activation(
        &self,
        input: CreateActivationRequest,
    ) -> RusotoFuture<CreateActivationResult, CreateActivationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateActivation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateActivationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateActivationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>
    fn create_association(
        &self,
        input: CreateAssociationRequest,
    ) -> RusotoFuture<CreateAssociationResult, CreateAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAssociationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>
    fn create_association_batch(
        &self,
        input: CreateAssociationBatchRequest,
    ) -> RusotoFuture<CreateAssociationBatchResult, CreateAssociationBatchError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateAssociationBatch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAssociationBatchResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAssociationBatchError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a Systems Manager document.</p> <p>After you create a document, you can use CreateAssociation to associate it with one or more running instances.</p>
    fn create_document(
        &self,
        input: CreateDocumentRequest,
    ) -> RusotoFuture<CreateDocumentResult, CreateDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDocumentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new Maintenance Window.</p>
    fn create_maintenance_window(
        &self,
        input: CreateMaintenanceWindowRequest,
    ) -> RusotoFuture<CreateMaintenanceWindowResult, CreateMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a patch baseline.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn create_patch_baseline(
        &self,
        input: CreatePatchBaselineRequest,
    ) -> RusotoFuture<CreatePatchBaselineResult, CreatePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreatePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a resource data sync configuration to a single bucket in Amazon S3. This is an asynchronous operation that returns immediately. After a successful initial sync is completed, the system continuously syncs data to the Amazon S3 bucket. To check the status of the sync, use the <a>ListResourceDataSync</a>.</p> <p>By default, data is not encrypted in Amazon S3. We strongly recommend that you enable encryption in Amazon S3 to ensure secure data storage. We also recommend that you secure access to the Amazon S3 bucket by creating a restrictive bucket policy. To view an example of a restrictive Amazon S3 bucket policy for Resource Data Sync, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-inventory-configuring.html#sysman-inventory-datasync">Configuring Resource Data Sync for Inventory</a>.</p>
    fn create_resource_data_sync(
        &self,
        input: CreateResourceDataSyncRequest,
    ) -> RusotoFuture<CreateResourceDataSyncResult, CreateResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.CreateResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateResourceDataSyncResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateResourceDataSyncError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an activation. You are not required to delete an activation. If you delete an activation, you can no longer use it to register additional managed instances. Deleting an activation does not de-register managed instances. You must manually de-register managed instances.</p>
    fn delete_activation(
        &self,
        input: DeleteActivationRequest,
    ) -> RusotoFuture<DeleteActivationResult, DeleteActivationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteActivation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteActivationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteActivationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates the specified Systems Manager document from the specified instance.</p> <p>When you disassociate a document from an instance, it does not change the configuration of the instance. To change the configuration state of an instance after you disassociate a document, you must create a new document with the desired configuration and associate it with the instance.</p>
    fn delete_association(
        &self,
        input: DeleteAssociationRequest,
    ) -> RusotoFuture<DeleteAssociationResult, DeleteAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAssociationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the Systems Manager document and all instance associations to the document.</p> <p>Before you delete the document, we recommend that you use <a>DeleteAssociation</a> to disassociate all instances that are associated with the document.</p>
    fn delete_document(
        &self,
        input: DeleteDocumentRequest,
    ) -> RusotoFuture<DeleteDocumentResult, DeleteDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDocumentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a Maintenance Window.</p>
    fn delete_maintenance_window(
        &self,
        input: DeleteMaintenanceWindowRequest,
    ) -> RusotoFuture<DeleteMaintenanceWindowResult, DeleteMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Delete a parameter from the system.</p>
    fn delete_parameter(
        &self,
        input: DeleteParameterRequest,
    ) -> RusotoFuture<DeleteParameterResult, DeleteParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteParameterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Delete a list of parameters. This API is used to delete parameters by using the Amazon EC2 console.</p>
    fn delete_parameters(
        &self,
        input: DeleteParametersRequest,
    ) -> RusotoFuture<DeleteParametersResult, DeleteParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteParametersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a patch baseline.</p>
    fn delete_patch_baseline(
        &self,
        input: DeletePatchBaselineRequest,
    ) -> RusotoFuture<DeletePatchBaselineResult, DeletePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeletePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeletePatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a Resource Data Sync configuration. After the configuration is deleted, changes to inventory data on managed instances are no longer synced with the target Amazon S3 bucket. Deleting a sync configuration does not delete data in the target Amazon S3 bucket.</p>
    fn delete_resource_data_sync(
        &self,
        input: DeleteResourceDataSyncRequest,
    ) -> RusotoFuture<DeleteResourceDataSyncResult, DeleteResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeleteResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteResourceDataSyncResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteResourceDataSyncError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the server or virtual machine from the list of registered servers. You can reregister the instance again at any time. If you don't plan to use Run Command on the server, we suggest uninstalling the SSM Agent first.</p>
    fn deregister_managed_instance(
        &self,
        input: DeregisterManagedInstanceRequest,
    ) -> RusotoFuture<DeregisterManagedInstanceResult, DeregisterManagedInstanceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DeregisterManagedInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterManagedInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterManagedInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes a patch group from a patch baseline.</p>
    fn deregister_patch_baseline_for_patch_group(
        &self,
        input: DeregisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        DeregisterPatchBaselineForPatchGroupResult,
        DeregisterPatchBaselineForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterPatchBaselineForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterPatchBaselineForPatchGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterPatchBaselineForPatchGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes a target from a Maintenance Window.</p>
    fn deregister_target_from_maintenance_window(
        &self,
        input: DeregisterTargetFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTargetFromMaintenanceWindowResult,
        DeregisterTargetFromMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterTargetFromMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterTargetFromMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTargetFromMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes a task from a Maintenance Window.</p>
    fn deregister_task_from_maintenance_window(
        &self,
        input: DeregisterTaskFromMaintenanceWindowRequest,
    ) -> RusotoFuture<
        DeregisterTaskFromMaintenanceWindowResult,
        DeregisterTaskFromMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DeregisterTaskFromMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeregisterTaskFromMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTaskFromMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Details about the activation, including: the date and time the activation was created, the expiration date, the IAM role assigned to the instances in the activation, and the number of instances activated by this registration.</p>
    fn describe_activations(
        &self,
        input: DescribeActivationsRequest,
    ) -> RusotoFuture<DescribeActivationsResult, DescribeActivationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeActivations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeActivationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeActivationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the association for the specified target or instance. If you created the association by using the <code>Targets</code> parameter, then you must retrieve the association by using the association ID. If you created the association by specifying an instance ID and a Systems Manager document, then you retrieve the association by specifying the document name and the instance ID. </p>
    fn describe_association(
        &self,
        input: DescribeAssociationRequest,
    ) -> RusotoFuture<DescribeAssociationResult, DescribeAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssociationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides details about all active and terminated Automation executions.</p>
    fn describe_automation_executions(
        &self,
        input: DescribeAutomationExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationExecutionsResult, DescribeAutomationExecutionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAutomationExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAutomationExecutionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutomationExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Information about all active and terminated step executions in an Automation workflow.</p>
    fn describe_automation_step_executions(
        &self,
        input: DescribeAutomationStepExecutionsRequest,
    ) -> RusotoFuture<DescribeAutomationStepExecutionsResult, DescribeAutomationStepExecutionsError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAutomationStepExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAutomationStepExecutionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAutomationStepExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all patches that could possibly be included in a patch baseline.</p>
    fn describe_available_patches(
        &self,
        input: DescribeAvailablePatchesRequest,
    ) -> RusotoFuture<DescribeAvailablePatchesResult, DescribeAvailablePatchesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeAvailablePatches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAvailablePatchesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAvailablePatchesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the specified Systems Manager document.</p>
    fn describe_document(
        &self,
        input: DescribeDocumentRequest,
    ) -> RusotoFuture<DescribeDocumentResult, DescribeDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDocumentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the permissions for a Systems Manager document. If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a user's AWS account ID) or publicly (<i>All</i>). </p>
    fn describe_document_permission(
        &self,
        input: DescribeDocumentPermissionRequest,
    ) -> RusotoFuture<DescribeDocumentPermissionResponse, DescribeDocumentPermissionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeDocumentPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDocumentPermissionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>All associations for the instance(s).</p>
    fn describe_effective_instance_associations(
        &self,
        input: DescribeEffectiveInstanceAssociationsRequest,
    ) -> RusotoFuture<
        DescribeEffectiveInstanceAssociationsResult,
        DescribeEffectiveInstanceAssociationsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeEffectiveInstanceAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEffectiveInstanceAssociationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEffectiveInstanceAssociationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Note that this API applies only to Windows patch baselines.</p>
    fn describe_effective_patches_for_patch_baseline(
        &self,
        input: DescribeEffectivePatchesForPatchBaselineRequest,
    ) -> RusotoFuture<
        DescribeEffectivePatchesForPatchBaselineResult,
        DescribeEffectivePatchesForPatchBaselineError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeEffectivePatchesForPatchBaseline",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEffectivePatchesForPatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEffectivePatchesForPatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The status of the associations for the instance(s).</p>
    fn describe_instance_associations_status(
        &self,
        input: DescribeInstanceAssociationsStatusRequest,
    ) -> RusotoFuture<
        DescribeInstanceAssociationsStatusResult,
        DescribeInstanceAssociationsStatusError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeInstanceAssociationsStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInstanceAssociationsStatusResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstanceAssociationsStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes one or more of your instances. You can use this to get information about instances like the operating system platform, the SSM Agent version (Linux), status etc. If you specify one or more instance IDs, it returns information for those instances. If you do not specify instance IDs, it returns information for all your instances. If you specify an instance ID that is not valid or an instance that you do not own, you receive an error. </p>
    fn describe_instance_information(
        &self,
        input: DescribeInstanceInformationRequest,
    ) -> RusotoFuture<DescribeInstanceInformationResult, DescribeInstanceInformationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstanceInformation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInstanceInformationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstanceInformationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the high-level patch state of one or more instances.</p>
    fn describe_instance_patch_states(
        &self,
        input: DescribeInstancePatchStatesRequest,
    ) -> RusotoFuture<DescribeInstancePatchStatesResult, DescribeInstancePatchStatesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatchStates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInstancePatchStatesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchStatesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the high-level patch state for the instances in the specified patch group.</p>
    fn describe_instance_patch_states_for_patch_group(
        &self,
        input: DescribeInstancePatchStatesForPatchGroupRequest,
    ) -> RusotoFuture<
        DescribeInstancePatchStatesForPatchGroupResult,
        DescribeInstancePatchStatesForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeInstancePatchStatesForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInstancePatchStatesForPatchGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchStatesForPatchGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about the patches on the specified instance and their state relative to the patch baseline being used for the instance.</p>
    fn describe_instance_patches(
        &self,
        input: DescribeInstancePatchesRequest,
    ) -> RusotoFuture<DescribeInstancePatchesResult, DescribeInstancePatchesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatches");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInstancePatchesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInstancePatchesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the individual task executions (one per target) for a particular task executed as part of a Maintenance Window execution.</p>
    fn describe_maintenance_window_execution_task_invocations(
        &self,
        input: DescribeMaintenanceWindowExecutionTaskInvocationsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTaskInvocationsResult,
        DescribeMaintenanceWindowExecutionTaskInvocationsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutionTaskInvocations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowExecutionTaskInvocationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::from_body(
                            String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                        ),
                    )
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>For a given Maintenance Window execution, lists the tasks that were executed.</p>
    fn describe_maintenance_window_execution_tasks(
        &self,
        input: DescribeMaintenanceWindowExecutionTasksRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionTasksResult,
        DescribeMaintenanceWindowExecutionTasksError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutionTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowExecutionTasksResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowExecutionTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the executions of a Maintenance Window. This includes information about when the Maintenance Window was scheduled to be active, and information about tasks registered and run with the Maintenance Window.</p>
    fn describe_maintenance_window_executions(
        &self,
        input: DescribeMaintenanceWindowExecutionsRequest,
    ) -> RusotoFuture<
        DescribeMaintenanceWindowExecutionsResult,
        DescribeMaintenanceWindowExecutionsError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.DescribeMaintenanceWindowExecutions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowExecutionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the targets registered with the Maintenance Window.</p>
    fn describe_maintenance_window_targets(
        &self,
        input: DescribeMaintenanceWindowTargetsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTargetsResult, DescribeMaintenanceWindowTargetsError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowTargetsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowTargetsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tasks in a Maintenance Window.</p>
    fn describe_maintenance_window_tasks(
        &self,
        input: DescribeMaintenanceWindowTasksRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowTasksResult, DescribeMaintenanceWindowTasksError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTasks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowTasksResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the Maintenance Windows in an AWS account.</p>
    fn describe_maintenance_windows(
        &self,
        input: DescribeMaintenanceWindowsRequest,
    ) -> RusotoFuture<DescribeMaintenanceWindowsResult, DescribeMaintenanceWindowsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindows");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMaintenanceWindowsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMaintenanceWindowsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Get information about a parameter.</p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p>
    fn describe_parameters(
        &self,
        input: DescribeParametersRequest,
    ) -> RusotoFuture<DescribeParametersResult, DescribeParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribeParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeParametersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the patch baselines in your AWS account.</p>
    fn describe_patch_baselines(
        &self,
        input: DescribePatchBaselinesRequest,
    ) -> RusotoFuture<DescribePatchBaselinesResult, DescribePatchBaselinesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchBaselines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePatchBaselinesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePatchBaselinesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns high-level aggregated patch compliance state for a patch group.</p>
    fn describe_patch_group_state(
        &self,
        input: DescribePatchGroupStateRequest,
    ) -> RusotoFuture<DescribePatchGroupStateResult, DescribePatchGroupStateError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroupState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePatchGroupStateResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePatchGroupStateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all patch groups that have been registered with patch baselines.</p>
    fn describe_patch_groups(
        &self,
        input: DescribePatchGroupsRequest,
    ) -> RusotoFuture<DescribePatchGroupsResult, DescribePatchGroupsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePatchGroupsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePatchGroupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Get detailed information about a particular Automation execution.</p>
    fn get_automation_execution(
        &self,
        input: GetAutomationExecutionRequest,
    ) -> RusotoFuture<GetAutomationExecutionResult, GetAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAutomationExecutionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetAutomationExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns detailed information about command execution for an invocation or plugin. </p>
    fn get_command_invocation(
        &self,
        input: GetCommandInvocationRequest,
    ) -> RusotoFuture<GetCommandInvocationResult, GetCommandInvocationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetCommandInvocation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetCommandInvocationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetCommandInvocationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the default patch baseline. Note that Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.</p> <p>If you do not specify an operating system value, the default patch baseline for Windows is returned.</p>
    fn get_default_patch_baseline(
        &self,
        input: GetDefaultPatchBaselineRequest,
    ) -> RusotoFuture<GetDefaultPatchBaselineResult, GetDefaultPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetDefaultPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDefaultPatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDefaultPatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the current snapshot for the patch baseline the instance uses. This API is primarily used by the AWS-RunPatchBaseline Systems Manager document. </p>
    fn get_deployable_patch_snapshot_for_instance(
        &self,
        input: GetDeployablePatchSnapshotForInstanceRequest,
    ) -> RusotoFuture<
        GetDeployablePatchSnapshotForInstanceResult,
        GetDeployablePatchSnapshotForInstanceError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetDeployablePatchSnapshotForInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeployablePatchSnapshotForInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDeployablePatchSnapshotForInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the contents of the specified Systems Manager document.</p>
    fn get_document(
        &self,
        input: GetDocumentRequest,
    ) -> RusotoFuture<GetDocumentResult, GetDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDocumentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Query inventory information.</p>
    fn get_inventory(
        &self,
        input: GetInventoryRequest,
    ) -> RusotoFuture<GetInventoryResult, GetInventoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInventoryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInventoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. </p>
    fn get_inventory_schema(
        &self,
        input: GetInventorySchemaRequest,
    ) -> RusotoFuture<GetInventorySchemaResult, GetInventorySchemaError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetInventorySchema");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInventorySchemaResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetInventorySchemaError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a Maintenance Window.</p>
    fn get_maintenance_window(
        &self,
        input: GetMaintenanceWindowRequest,
    ) -> RusotoFuture<GetMaintenanceWindowResult, GetMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves details about a specific task executed as part of a Maintenance Window execution.</p>
    fn get_maintenance_window_execution(
        &self,
        input: GetMaintenanceWindowExecutionRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionResult, GetMaintenanceWindowExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMaintenanceWindowExecutionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the details about a specific task executed as part of a Maintenance Window execution.</p>
    fn get_maintenance_window_execution_task(
        &self,
        input: GetMaintenanceWindowExecutionTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowExecutionTaskResult, GetMaintenanceWindowExecutionTaskError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetMaintenanceWindowExecutionTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMaintenanceWindowExecutionTaskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a task invocation. A task invocation is a specific task executing on a specific target. Maintenance Windows report status for all invocations. </p>
    fn get_maintenance_window_execution_task_invocation(
        &self,
        input: GetMaintenanceWindowExecutionTaskInvocationRequest,
    ) -> RusotoFuture<
        GetMaintenanceWindowExecutionTaskInvocationResult,
        GetMaintenanceWindowExecutionTaskInvocationError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.GetMaintenanceWindowExecutionTaskInvocation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMaintenanceWindowExecutionTaskInvocationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowExecutionTaskInvocationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the tasks in a Maintenance Window.</p>
    fn get_maintenance_window_task(
        &self,
        input: GetMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<GetMaintenanceWindowTaskResult, GetMaintenanceWindowTaskError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMaintenanceWindowTaskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMaintenanceWindowTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Get information about a parameter by using the parameter name. </p>
    fn get_parameter(
        &self,
        input: GetParameterRequest,
    ) -> RusotoFuture<GetParameterResult, GetParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetParameterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Query a list of all parameters used by the AWS account.</p>
    fn get_parameter_history(
        &self,
        input: GetParameterHistoryRequest,
    ) -> RusotoFuture<GetParameterHistoryResult, GetParameterHistoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameterHistory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetParameterHistoryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetParameterHistoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Get details of a parameter.</p>
    fn get_parameters(
        &self,
        input: GetParametersRequest,
    ) -> RusotoFuture<GetParametersResult, GetParametersError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParameters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetParametersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetParametersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieve parameters in a specific hierarchy. For more information, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-working.html">Working with Systems Manager Parameters</a>. </p> <p>Request results are returned on a best-effort basis. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified. The number of items returned, however, can be between zero and the value of <code>MaxResults</code>. If the service reaches an internal limit while processing the results, it stops the operation and returns the matching values up to that point and a <code>NextToken</code>. You can specify the <code>NextToken</code> in a subsequent call to get the next set of results.</p> <note> <p>This API action doesn&#39;t support filtering by tags. </p> </note></p>
    fn get_parameters_by_path(
        &self,
        input: GetParametersByPathRequest,
    ) -> RusotoFuture<GetParametersByPathResult, GetParametersByPathError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetParametersByPath");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetParametersByPathResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetParametersByPathError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves information about a patch baseline.</p>
    fn get_patch_baseline(
        &self,
        input: GetPatchBaselineRequest,
    ) -> RusotoFuture<GetPatchBaselineResult, GetPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetPatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves the patch baseline that should be used for the specified patch group.</p>
    fn get_patch_baseline_for_patch_group(
        &self,
        input: GetPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<GetPatchBaselineForPatchGroupResult, GetPatchBaselineForPatchGroupError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.GetPatchBaselineForPatchGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPatchBaselineForPatchGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetPatchBaselineForPatchGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves all versions of an association for a specific association ID.</p>
    fn list_association_versions(
        &self,
        input: ListAssociationVersionsRequest,
    ) -> RusotoFuture<ListAssociationVersionsResult, ListAssociationVersionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListAssociationVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssociationVersionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociationVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the associations for the specified Systems Manager document or instance.</p>
    fn list_associations(
        &self,
        input: ListAssociationsRequest,
    ) -> RusotoFuture<ListAssociationsResult, ListAssociationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListAssociations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAssociationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user executes SendCommand against three instances, then a command invocation is created for each requested instance ID. ListCommandInvocations provide status about command execution.</p>
    fn list_command_invocations(
        &self,
        input: ListCommandInvocationsRequest,
    ) -> RusotoFuture<ListCommandInvocationsResult, ListCommandInvocationsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListCommandInvocations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCommandInvocationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCommandInvocationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the commands requested by users of the AWS account.</p>
    fn list_commands(
        &self,
        input: ListCommandsRequest,
    ) -> RusotoFuture<ListCommandsResult, ListCommandsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListCommands");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListCommandsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListCommandsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>For a specified resource ID, this API action returns a list of compliance statuses for different resource types. Currently, you can only specify one resource ID per call. List results depend on the criteria specified in the filter. </p>
    fn list_compliance_items(
        &self,
        input: ListComplianceItemsRequest,
    ) -> RusotoFuture<ListComplianceItemsResult, ListComplianceItemsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListComplianceItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListComplianceItemsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListComplianceItemsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a summary count of compliant and non-compliant resources for a compliance type. For example, this call can return State Manager associations, patches, or custom compliance types according to the filter criteria that you specify. </p>
    fn list_compliance_summaries(
        &self,
        input: ListComplianceSummariesRequest,
    ) -> RusotoFuture<ListComplianceSummariesResult, ListComplianceSummariesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListComplianceSummaries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListComplianceSummariesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListComplianceSummariesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>List all versions for a document.</p>
    fn list_document_versions(
        &self,
        input: ListDocumentVersionsRequest,
    ) -> RusotoFuture<ListDocumentVersionsResult, ListDocumentVersionsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListDocumentVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDocumentVersionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDocumentVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes one or more of your Systems Manager documents.</p>
    fn list_documents(
        &self,
        input: ListDocumentsRequest,
    ) -> RusotoFuture<ListDocumentsResult, ListDocumentsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListDocuments");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDocumentsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListDocumentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>A list of inventory items returned by the request.</p>
    fn list_inventory_entries(
        &self,
        input: ListInventoryEntriesRequest,
    ) -> RusotoFuture<ListInventoryEntriesResult, ListInventoryEntriesError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListInventoryEntries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListInventoryEntriesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListInventoryEntriesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a resource-level summary count. The summary includes information about compliant and non-compliant statuses and detailed compliance-item severity counts, according to the filter criteria you specify.</p>
    fn list_resource_compliance_summaries(
        &self,
        input: ListResourceComplianceSummariesRequest,
    ) -> RusotoFuture<ListResourceComplianceSummariesResult, ListResourceComplianceSummariesError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListResourceComplianceSummaries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourceComplianceSummariesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceComplianceSummariesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists your resource data sync configurations. Includes information about the last time a sync attempted to start, the last sync status, and the last time a sync successfully completed.</p> <p>The number of sync configurations might be too large to return using a single call to <code>ListResourceDataSync</code>. You can limit the number of sync configurations returned by using the <code>MaxResults</code> parameter. To determine whether there are more sync configurations to list, check the value of <code>NextToken</code> in the output. If there are more sync configurations to list, you can request them by specifying the <code>NextToken</code> returned in the call to the parameter of a subsequent call. </p>
    fn list_resource_data_sync(
        &self,
        input: ListResourceDataSyncRequest,
    ) -> RusotoFuture<ListResourceDataSyncResult, ListResourceDataSyncError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListResourceDataSync");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourceDataSyncResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceDataSyncError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of the tags assigned to the specified resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResult, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForResourceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Shares a Systems Manager document publicly or privately. If you share a document privately, you must specify the AWS user account IDs for those people who can use the document. If you share a document publicly, you must specify <i>All</i> as the account ID.</p>
    fn modify_document_permission(
        &self,
        input: ModifyDocumentPermissionRequest,
    ) -> RusotoFuture<ModifyDocumentPermissionResponse, ModifyDocumentPermissionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.ModifyDocumentPermission");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ModifyDocumentPermissionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ModifyDocumentPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Registers a compliance type and other compliance details on a designated resource. This action lets you register custom compliance details with a resource. This call overwrites existing compliance information on the resource, so you must provide a full list of compliance items each time that you send the request.</p> <p>ComplianceType can be one of the following:</p> <ul> <li> <p>ExecutionId: The execution ID when the patch, association, or custom compliance item was applied.</p> </li> <li> <p>ExecutionType: Specify patch, association, or Custom:<code>string</code>.</p> </li> <li> <p>ExecutionTime. The time the patch, association, or custom compliance item was applied to the instance.</p> </li> <li> <p>Id: The patch, association, or custom compliance ID.</p> </li> <li> <p>Title: A title.</p> </li> <li> <p>Status: The status of the compliance item. For example, <code>approved</code> for patches, or <code>Failed</code> for associations.</p> </li> <li> <p>Severity: A patch severity. For example, <code>critical</code>.</p> </li> <li> <p>DocumentName: A SSM document name. For example, AWS-RunPatchBaseline.</p> </li> <li> <p>DocumentVersion: An SSM document version number. For example, 4.</p> </li> <li> <p>Classification: A patch classification. For example, <code>security updates</code>.</p> </li> <li> <p>PatchBaselineId: A patch baseline ID.</p> </li> <li> <p>PatchSeverity: A patch severity. For example, <code>Critical</code>.</p> </li> <li> <p>PatchState: A patch state. For example, <code>InstancesWithFailedPatches</code>.</p> </li> <li> <p>PatchGroup: The name of a patch group.</p> </li> <li> <p>InstalledTime: The time the association, patch, or custom compliance item was applied to the resource. Specify the time by using the following format: yyyy-MM-dd&#39;T&#39;HH:mm:ss&#39;Z&#39;</p> </li> </ul></p>
    fn put_compliance_items(
        &self,
        input: PutComplianceItemsRequest,
    ) -> RusotoFuture<PutComplianceItemsResult, PutComplianceItemsError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutComplianceItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutComplianceItemsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutComplianceItemsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Bulk update custom inventory items on one more instance. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist.</p>
    fn put_inventory(
        &self,
        input: PutInventoryRequest,
    ) -> RusotoFuture<PutInventoryResult, PutInventoryError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutInventoryResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutInventoryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Add one or more parameters to the system.</p>
    fn put_parameter(
        &self,
        input: PutParameterRequest,
    ) -> RusotoFuture<PutParameterResult, PutParameterError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.PutParameter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutParameterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutParameterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Defines the default patch baseline.</p>
    fn register_default_patch_baseline(
        &self,
        input: RegisterDefaultPatchBaselineRequest,
    ) -> RusotoFuture<RegisterDefaultPatchBaselineResult, RegisterDefaultPatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.RegisterDefaultPatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterDefaultPatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterDefaultPatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers a patch baseline for a patch group.</p>
    fn register_patch_baseline_for_patch_group(
        &self,
        input: RegisterPatchBaselineForPatchGroupRequest,
    ) -> RusotoFuture<
        RegisterPatchBaselineForPatchGroupResult,
        RegisterPatchBaselineForPatchGroupError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterPatchBaselineForPatchGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterPatchBaselineForPatchGroupResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterPatchBaselineForPatchGroupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Registers a target with a Maintenance Window.</p>
    fn register_target_with_maintenance_window(
        &self,
        input: RegisterTargetWithMaintenanceWindowRequest,
    ) -> RusotoFuture<
        RegisterTargetWithMaintenanceWindowResult,
        RegisterTargetWithMaintenanceWindowError,
    > {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterTargetWithMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterTargetWithMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterTargetWithMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds a new task to a Maintenance Window.</p>
    fn register_task_with_maintenance_window(
        &self,
        input: RegisterTaskWithMaintenanceWindowRequest,
    ) -> RusotoFuture<RegisterTaskWithMaintenanceWindowResult, RegisterTaskWithMaintenanceWindowError>
    {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonSSM.RegisterTaskWithMaintenanceWindow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RegisterTaskWithMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RegisterTaskWithMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes all tags from the specified resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceRequest,
    ) -> RusotoFuture<RemoveTagsFromResourceResult, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.RemoveTagsFromResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RemoveTagsFromResourceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsFromResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sends a signal to an Automation execution to change the current behavior or status of the execution. </p>
    fn send_automation_signal(
        &self,
        input: SendAutomationSignalRequest,
    ) -> RusotoFuture<SendAutomationSignalResult, SendAutomationSignalError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.SendAutomationSignal");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SendAutomationSignalResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendAutomationSignalError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Executes commands on one or more managed instances.</p>
    fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> RusotoFuture<SendCommandResult, SendCommandError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.SendCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SendCommandResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SendCommandError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Initiates execution of an Automation document.</p>
    fn start_automation_execution(
        &self,
        input: StartAutomationExecutionRequest,
    ) -> RusotoFuture<StartAutomationExecutionResult, StartAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StartAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartAutomationExecutionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartAutomationExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stop an Automation that is currently executing.</p>
    fn stop_automation_execution(
        &self,
        input: StopAutomationExecutionRequest,
    ) -> RusotoFuture<StopAutomationExecutionResult, StopAutomationExecutionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.StopAutomationExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopAutomationExecutionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopAutomationExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an association. You can update the association name and version, the document version, schedule, parameters, and Amazon S3 output.</p>
    fn update_association(
        &self,
        input: UpdateAssociationRequest,
    ) -> RusotoFuture<UpdateAssociationResult, UpdateAssociationError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateAssociation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAssociationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the status of the Systems Manager document associated with the specified instance.</p>
    fn update_association_status(
        &self,
        input: UpdateAssociationStatusRequest,
    ) -> RusotoFuture<UpdateAssociationStatusResult, UpdateAssociationStatusError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateAssociationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateAssociationStatusResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAssociationStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The document you want to update.</p>
    fn update_document(
        &self,
        input: UpdateDocumentRequest,
    ) -> RusotoFuture<UpdateDocumentResult, UpdateDocumentError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDocumentResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Set the default version of a document. </p>
    fn update_document_default_version(
        &self,
        input: UpdateDocumentDefaultVersionRequest,
    ) -> RusotoFuture<UpdateDocumentDefaultVersionResult, UpdateDocumentDefaultVersionError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateDocumentDefaultVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDocumentDefaultVersionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentDefaultVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing Maintenance Window. Only specified parameters are modified.</p>
    fn update_maintenance_window(
        &self,
        input: UpdateMaintenanceWindowRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowResult, UpdateMaintenanceWindowError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindow");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateMaintenanceWindowResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the target of an existing Maintenance Window. You can't change the target type, but you can change the following:</p> <p>The target from being an ID target to a Tag target, or a Tag target to an ID target.</p> <p>IDs for an ID target.</p> <p>Tags for a Tag target.</p> <p>Owner.</p> <p>Name.</p> <p>Description.</p> <p>If a parameter is null, then the corresponding field is not modified.</p>
    fn update_maintenance_window_target(
        &self,
        input: UpdateMaintenanceWindowTargetRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTargetResult, UpdateMaintenanceWindowTargetError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindowTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateMaintenanceWindowTargetResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowTargetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies a task assigned to a Maintenance Window. You can't change the task type, but you can change the following values:</p> <ul> <li> <p>TaskARN. For example, you can change a RUN_COMMAND task from AWS-RunPowerShellScript to AWS-RunShellScript.</p> </li> <li> <p>ServiceRoleArn</p> </li> <li> <p>TaskInvocationParameters</p> </li> <li> <p>Priority</p> </li> <li> <p>MaxConcurrency</p> </li> <li> <p>MaxErrors</p> </li> </ul> <p>If a parameter is null, then the corresponding field is not modified. Also, if you set Replace to true, then all fields required by the <a>RegisterTaskWithMaintenanceWindow</a> action are required for this request. Optional fields that aren't specified are set to null.</p>
    fn update_maintenance_window_task(
        &self,
        input: UpdateMaintenanceWindowTaskRequest,
    ) -> RusotoFuture<UpdateMaintenanceWindowTaskResult, UpdateMaintenanceWindowTaskError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindowTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateMaintenanceWindowTaskResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMaintenanceWindowTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Assigns or changes an Amazon Identity and Access Management (IAM) role to the managed instance.</p>
    fn update_managed_instance_role(
        &self,
        input: UpdateManagedInstanceRoleRequest,
    ) -> RusotoFuture<UpdateManagedInstanceRoleResult, UpdateManagedInstanceRoleError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdateManagedInstanceRole");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateManagedInstanceRoleResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateManagedInstanceRoleError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Modifies an existing patch baseline. Fields not specified in the request are left unchanged.</p> <note> <p>For information about valid key and value pairs in <code>PatchFilters</code> for each supported operating system type, see <a href="http://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PatchFilter.html">PatchFilter</a>.</p> </note></p>
    fn update_patch_baseline(
        &self,
        input: UpdatePatchBaselineRequest,
    ) -> RusotoFuture<UpdatePatchBaselineResult, UpdatePatchBaselineError> {
        let mut request = SignedRequest::new("POST", "ssm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonSSM.UpdatePatchBaseline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdatePatchBaselineResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdatePatchBaselineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
