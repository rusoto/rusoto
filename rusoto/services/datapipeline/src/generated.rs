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
/// <p>Contains the parameters for ActivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ActivatePipelineInput {
    /// <p>A list of parameter values to pass to the pipeline at activation.</p>
    #[serde(rename = "parameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The date and time to resume the pipeline. By default, the pipeline resumes from the last completed execution.</p>
    #[serde(rename = "startTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

/// <p>Contains the output of ActivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ActivatePipelineOutput {}

/// <p>Contains the parameters for AddTags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsInput {
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The tags to add, as key/value pairs.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>Contains the output of AddTags.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddTagsOutput {}

/// <p>Contains the parameters for CreatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePipelineInput {
    /// <p>The description for the pipeline.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name for the pipeline. You can use the same name for multiple pipelines associated with your AWS account, because AWS Data Pipeline assigns each pipeline a unique pipeline identifier.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of tags to associate with the pipeline at creation. Tags let you control access to pipelines. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> in the <i>AWS Data Pipeline Developer Guide</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A unique identifier. This identifier is not the same as the pipeline identifier assigned by AWS Data Pipeline. You are responsible for defining the format and ensuring the uniqueness of this identifier. You use this parameter to ensure idempotency during repeated calls to <code>CreatePipeline</code>. For example, if the first call to <code>CreatePipeline</code> does not succeed, you can pass in the same unique identifier and pipeline name combination on a subsequent call to <code>CreatePipeline</code>. <code>CreatePipeline</code> ensures that if a pipeline already exists with the same name and unique identifier, a new pipeline is not created. Instead, you'll receive the pipeline identifier from the previous attempt. The uniqueness of the name and unique identifier combination is scoped to the AWS account or IAM user credentials.</p>
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
}

/// <p>Contains the output of CreatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatePipelineOutput {
    /// <p>The ID that AWS Data Pipeline assigns the newly created pipeline. For example, <code>df-06372391ZG65EXAMPLE</code>.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the parameters for DeactivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeactivatePipelineInput {
    /// <p>Indicates whether to cancel any running objects. The default is true, which sets the state of any running objects to <code>CANCELED</code>. If this value is false, the pipeline is deactivated after all running objects finish.</p>
    #[serde(rename = "cancelActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_active: Option<bool>,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the output of DeactivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeactivatePipelineOutput {}

/// <p>Contains the parameters for DeletePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePipelineInput {
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the parameters for DescribeObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeObjectsInput {
    /// <p>Indicates whether any expressions in the object should be evaluated when the object descriptions are returned.</p>
    #[serde(rename = "evaluateExpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_expressions: Option<bool>,
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>DescribeObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The IDs of the pipeline objects that contain the definitions to be described. You can pass as many as 25 identifiers in a single call to <code>DescribeObjects</code>.</p>
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
    /// <p>The ID of the pipeline that contains the object definitions.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the output of DescribeObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeObjectsOutput {
    /// <p>Indicates whether there are more results to return.</p>
    #[serde(rename = "hasMoreResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    /// <p>The starting point for the next page of results. To view the next page of results, call <code>DescribeObjects</code> again with this marker value. If the value is null, there are no more results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of object definitions.</p>
    #[serde(rename = "pipelineObjects")]
    pub pipeline_objects: Vec<PipelineObject>,
}

/// <p>Contains the parameters for DescribePipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePipelinesInput {
    /// <p>The IDs of the pipelines to describe. You can pass as many as 25 identifiers in a single call. To obtain pipeline IDs, call <a>ListPipelines</a>.</p>
    #[serde(rename = "pipelineIds")]
    pub pipeline_ids: Vec<String>,
}

/// <p>Contains the output of DescribePipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribePipelinesOutput {
    /// <p>An array of descriptions for the specified pipelines.</p>
    #[serde(rename = "pipelineDescriptionList")]
    pub pipeline_description_list: Vec<PipelineDescription>,
}

/// <p>Contains the parameters for EvaluateExpression.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EvaluateExpressionInput {
    /// <p>The expression to evaluate.</p>
    #[serde(rename = "expression")]
    pub expression: String,
    /// <p>The ID of the object.</p>
    #[serde(rename = "objectId")]
    pub object_id: String,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the output of EvaluateExpression.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EvaluateExpressionOutput {
    /// <p>The evaluated expression.</p>
    #[serde(rename = "evaluatedExpression")]
    pub evaluated_expression: String,
}

/// <p>A key-value pair that describes a property of a pipeline object. The value is specified as either a string value (<code>StringValue</code>) or a reference to another object (<code>RefValue</code>) but not as both.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// <p>The field identifier.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The field value, expressed as the identifier of another object.</p>
    #[serde(rename = "refValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_value: Option<String>,
    /// <p>The field value, expressed as a String.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>Contains the parameters for GetPipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPipelineDefinitionInput {
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The version of the pipeline definition to retrieve. Set this parameter to <code>latest</code> (default) to use the last definition saved to the pipeline or <code>active</code> to use the last definition that was activated.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Contains the output of GetPipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetPipelineDefinitionOutput {
    /// <p>The parameter objects used in the pipeline definition.</p>
    #[serde(rename = "parameterObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    /// <p>The parameter values used in the pipeline definition.</p>
    #[serde(rename = "parameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    /// <p>The objects defined in the pipeline.</p>
    #[serde(rename = "pipelineObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_objects: Option<Vec<PipelineObject>>,
}

/// <p><p>Identity information for the EC2 instance that is hosting the task runner. You can get this value by calling a metadata URI from the EC2 instance. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AESDG-chapter-instancedata.html">Instance Metadata</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> Passing in this value proves that your task runner is running on an EC2 instance, and ensures the proper AWS Data Pipeline service charges are applied to your pipeline.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceIdentity {
    /// <p>A description of an EC2 instance that is generated when the instance is launched and exposed to the instance via the instance metadata service in the form of a JSON representation of an object.</p>
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>A signature which can be used to verify the accuracy and authenticity of the information provided in the instance identity document.</p>
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// <p>Contains the parameters for ListPipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelinesInput {
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>ListPipelines</code> with the marker value from the previous call to retrieve the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Contains the output of ListPipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListPipelinesOutput {
    /// <p>Indicates whether there are more results that can be obtained by a subsequent call.</p>
    #[serde(rename = "hasMoreResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    /// <p>The starting point for the next page of results. To view the next page of results, call <code>ListPipelinesOutput</code> again with this marker value. If the value is null, there are no more results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The pipeline identifiers. If you require additional information about the pipelines, you can use these identifiers to call <a>DescribePipelines</a> and <a>GetPipelineDefinition</a>.</p>
    #[serde(rename = "pipelineIdList")]
    pub pipeline_id_list: Vec<PipelineIdName>,
}

/// <p>Contains a logical operation for comparing the value of a field with a specified value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Operator {
    /// <p> The logical operation to be performed: equal (<code>EQ</code>), equal reference (<code>REF_EQ</code>), less than or equal (<code>LE</code>), greater than or equal (<code>GE</code>), or between (<code>BETWEEN</code>). Equal reference (<code>REF_EQ</code>) can be used only with reference fields. The other comparison types can be used only with String fields. The comparison types you can use apply only to certain object fields, as detailed below. </p> <p> The comparison operators EQ and REF_EQ act on the following fields: </p> <ul> <li>name</li> <li>@sphere</li> <li>parent</li> <li>@componentParent</li> <li>@instanceParent</li> <li>@status</li> <li>@scheduledStartTime</li> <li>@scheduledEndTime</li> <li>@actualStartTime</li> <li>@actualEndTime</li> </ul> <p> The comparison operators <code>GE</code>, <code>LE</code>, and <code>BETWEEN</code> act on the following fields: </p> <ul> <li>@scheduledStartTime</li> <li>@scheduledEndTime</li> <li>@actualStartTime</li> <li>@actualEndTime</li> </ul> <p>Note that fields beginning with the at sign (@) are read-only and set by the web service. When you name fields, you should choose names containing only alpha-numeric values, as symbols may be reserved by AWS Data Pipeline. User-defined fields that you add to a pipeline should prefix their name with the string "my".</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value that the actual field value will be compared with.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The attributes allowed or specified with a parameter object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterAttribute {
    /// <p>The field identifier.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The field value, expressed as a String.</p>
    #[serde(rename = "stringValue")]
    pub string_value: String,
}

/// <p>Contains information about a parameter object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterObject {
    /// <p>The attributes of the parameter object.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<ParameterAttribute>,
    /// <p>The ID of the parameter object. </p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>A value or list of parameter values. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    /// <p>The ID of the parameter value.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The field value, expressed as a String.</p>
    #[serde(rename = "stringValue")]
    pub string_value: String,
}

/// <p>Contains pipeline metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PipelineDescription {
    /// <p>Description of the pipeline.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of read-only fields that contain metadata about the pipeline: @userId, @accountId, and @pipelineState.</p>
    #[serde(rename = "fields")]
    pub fields: Vec<Field>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The pipeline identifier that was assigned by AWS Data Pipeline. This is a string of the form <code>df-297EG78HU43EEXAMPLE</code>.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>A list of tags to associated with a pipeline. Tags let you control access to pipelines. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> in the <i>AWS Data Pipeline Developer Guide</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains the name and identifier of a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PipelineIdName {
    /// <p>The ID of the pipeline that was assigned by AWS Data Pipeline. This is a string of the form <code>df-297EG78HU43EEXAMPLE</code>.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about a pipeline object. This can be a logical, physical, or physical attempt pipeline object. The complete set of components of a pipeline defines the pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineObject {
    /// <p>Key-value pairs that define the properties of the object.</p>
    #[serde(rename = "fields")]
    pub fields: Vec<Field>,
    /// <p>The ID of the object.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The name of the object.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Contains the parameters for PollForTask.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PollForTaskInput {
    /// <p>The public DNS name of the calling task runner.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>Identity information for the EC2 instance that is hosting the task runner. You can get this value from the instance using <code>http://169.254.169.254/latest/meta-data/instance-id</code>. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/AESDG-chapter-instancedata.html">Instance Metadata</a> in the <i>Amazon Elastic Compute Cloud User Guide.</i> Passing in this value proves that your task runner is running on an EC2 instance, and ensures the proper AWS Data Pipeline service charges are applied to your pipeline.</p>
    #[serde(rename = "instanceIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity: Option<InstanceIdentity>,
    /// <p>The type of task the task runner is configured to accept and process. The worker group is set as a field on objects in the pipeline when they are created. You can only specify a single value for <code>workerGroup</code> in the call to <code>PollForTask</code>. There are no wildcard values permitted in <code>workerGroup</code>; the string must be an exact, case-sensitive, match.</p>
    #[serde(rename = "workerGroup")]
    pub worker_group: String,
}

/// <p>Contains the output of PollForTask.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PollForTaskOutput {
    /// <p>The information needed to complete the task that is being assigned to the task runner. One of the fields returned in this object is <code>taskId</code>, which contains an identifier for the task being assigned. The calling task runner uses <code>taskId</code> in subsequent calls to <a>ReportTaskProgress</a> and <a>SetTaskStatus</a>.</p>
    #[serde(rename = "taskObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_object: Option<TaskObject>,
}

/// <p>Contains the parameters for PutPipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutPipelineDefinitionInput {
    /// <p>The parameter objects used with the pipeline.</p>
    #[serde(rename = "parameterObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    /// <p>The parameter values used with the pipeline.</p>
    #[serde(rename = "parameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The objects that define the pipeline. These objects overwrite the existing pipeline definition.</p>
    #[serde(rename = "pipelineObjects")]
    pub pipeline_objects: Vec<PipelineObject>,
}

/// <p>Contains the output of PutPipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutPipelineDefinitionOutput {
    /// <p>Indicates whether there were validation errors, and the pipeline definition is stored but cannot be activated until you correct the pipeline and call <code>PutPipelineDefinition</code> to commit the corrected pipeline.</p>
    #[serde(rename = "errored")]
    pub errored: bool,
    /// <p>The validation errors that are associated with the objects defined in <code>pipelineObjects</code>.</p>
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    /// <p>The validation warnings that are associated with the objects defined in <code>pipelineObjects</code>.</p>
    #[serde(rename = "validationWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_warnings: Option<Vec<ValidationWarning>>,
}

/// <p>Defines the query to run against an object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Query {
    /// <p>List of selectors that define the query. An object must satisfy all of the selectors to match the query.</p>
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<Selector>>,
}

/// <p>Contains the parameters for QueryObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct QueryObjectsInput {
    /// <p>The maximum number of object names that <code>QueryObjects</code> will return in a single call. The default value is 100. </p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>QueryObjects</code> with the marker value from the previous call to retrieve the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The query that defines the objects to be returned. The <code>Query</code> object can contain a maximum of ten selectors. The conditions in the query are limited to top-level String fields in the object. These filters can be applied to components, instances, and attempts.</p>
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Query>,
    /// <p>Indicates whether the query applies to components or instances. The possible values are: <code>COMPONENT</code>, <code>INSTANCE</code>, and <code>ATTEMPT</code>.</p>
    #[serde(rename = "sphere")]
    pub sphere: String,
}

/// <p>Contains the output of QueryObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct QueryObjectsOutput {
    /// <p>Indicates whether there are more results that can be obtained by a subsequent call.</p>
    #[serde(rename = "hasMoreResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    /// <p>The identifiers that match the query selectors.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>The starting point for the next page of results. To view the next page of results, call <code>QueryObjects</code> again with this marker value. If the value is null, there are no more results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Contains the parameters for RemoveTags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsInput {
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The keys of the tags to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Contains the output of RemoveTags.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoveTagsOutput {}

/// <p>Contains the parameters for ReportTaskProgress.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReportTaskProgressInput {
    /// <p>Key-value pairs that define the properties of the ReportTaskProgressInput object.</p>
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    /// <p>The ID of the task assigned to the task runner. This value is provided in the response for <a>PollForTask</a>.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

/// <p>Contains the output of ReportTaskProgress.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReportTaskProgressOutput {
    /// <p>If true, the calling task runner should cancel processing of the task. The task runner does not need to call <a>SetTaskStatus</a> for canceled tasks.</p>
    #[serde(rename = "canceled")]
    pub canceled: bool,
}

/// <p>Contains the parameters for ReportTaskRunnerHeartbeat.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReportTaskRunnerHeartbeatInput {
    /// <p>The public DNS name of the task runner.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The ID of the task runner. This value should be unique across your AWS account. In the case of AWS Data Pipeline Task Runner launched on a resource managed by AWS Data Pipeline, the web service provides a unique identifier when it launches the application. If you have written a custom task runner, you should assign a unique identifier for the task runner.</p>
    #[serde(rename = "taskrunnerId")]
    pub taskrunner_id: String,
    /// <p>The type of task the task runner is configured to accept and process. The worker group is set as a field on objects in the pipeline when they are created. You can only specify a single value for <code>workerGroup</code>. There are no wildcard values permitted in <code>workerGroup</code>; the string must be an exact, case-sensitive, match.</p>
    #[serde(rename = "workerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_group: Option<String>,
}

/// <p>Contains the output of ReportTaskRunnerHeartbeat.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReportTaskRunnerHeartbeatOutput {
    /// <p>Indicates whether the calling task runner should terminate.</p>
    #[serde(rename = "terminate")]
    pub terminate: bool,
}

/// <p>A comparision that is used to determine whether a query should return this object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Selector {
    /// <p>The name of the field that the operator will be applied to. The field name is the "key" portion of the field definition in the pipeline definition syntax that is used by the AWS Data Pipeline API. If the field is not set on the object, the condition fails.</p>
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
}

/// <p>Contains the parameters for SetStatus.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetStatusInput {
    /// <p>The IDs of the objects. The corresponding objects can be either physical or components, but not a mix of both types.</p>
    #[serde(rename = "objectIds")]
    pub object_ids: Vec<String>,
    /// <p>The ID of the pipeline that contains the objects.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The status to be set on all the objects specified in <code>objectIds</code>. For components, use <code>PAUSE</code> or <code>RESUME</code>. For instances, use <code>TRY_CANCEL</code>, <code>RERUN</code>, or <code>MARK_FINISHED</code>.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>Contains the parameters for SetTaskStatus.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetTaskStatusInput {
    /// <p>If an error occurred during the task, this value specifies the error code. This value is set on the physical attempt object. It is used to display error information to the user. It should not start with string "Service_" which is reserved by the system.</p>
    #[serde(rename = "errorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    /// <p>If an error occurred during the task, this value specifies a text description of the error. This value is set on the physical attempt object. It is used to display error information to the user. The web service does not parse this value.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>If an error occurred during the task, this value specifies the stack trace associated with the error. This value is set on the physical attempt object. It is used to display error information to the user. The web service does not parse this value.</p>
    #[serde(rename = "errorStackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_stack_trace: Option<String>,
    /// <p>The ID of the task assigned to the task runner. This value is provided in the response for <a>PollForTask</a>.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// <p>If <code>FINISHED</code>, the task successfully completed. If <code>FAILED</code>, the task ended unsuccessfully. Preconditions use false.</p>
    #[serde(rename = "taskStatus")]
    pub task_status: String,
}

/// <p>Contains the output of SetTaskStatus.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SetTaskStatusOutput {}

/// <p>Tags are key/value pairs defined by a user and associated with a pipeline to control access. AWS Data Pipeline allows you to associate ten tags per pipeline. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> in the <i>AWS Data Pipeline Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key name of a tag defined by a user. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> in the <i>AWS Data Pipeline Developer Guide</i>.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional value portion of a tag defined by a user. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-control-access.html">Controlling User Access to Pipelines</a> in the <i>AWS Data Pipeline Developer Guide</i>.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Contains information about a pipeline task that is assigned to a task runner.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TaskObject {
    /// <p>The ID of the pipeline task attempt object. AWS Data Pipeline uses this value to track how many times a task is attempted.</p>
    #[serde(rename = "attemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<String>,
    /// <p>Connection information for the location where the task runner will publish the output of the task.</p>
    #[serde(rename = "objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<::std::collections::HashMap<String, PipelineObject>>,
    /// <p>The ID of the pipeline that provided the task.</p>
    #[serde(rename = "pipelineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    /// <p>An internal identifier for the task. This ID is passed to the <a>SetTaskStatus</a> and <a>ReportTaskProgress</a> actions.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// <p>Contains the parameters for ValidatePipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ValidatePipelineDefinitionInput {
    /// <p>The parameter objects used with the pipeline.</p>
    #[serde(rename = "parameterObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    /// <p>The parameter values used with the pipeline.</p>
    #[serde(rename = "parameterValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    /// <p>The objects that define the pipeline changes to validate against the pipeline.</p>
    #[serde(rename = "pipelineObjects")]
    pub pipeline_objects: Vec<PipelineObject>,
}

/// <p>Contains the output of ValidatePipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ValidatePipelineDefinitionOutput {
    /// <p>Indicates whether there were validation errors.</p>
    #[serde(rename = "errored")]
    pub errored: bool,
    /// <p>Any validation errors that were found.</p>
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    /// <p>Any validation warnings that were found.</p>
    #[serde(rename = "validationWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_warnings: Option<Vec<ValidationWarning>>,
}

/// <p>Defines a validation error. Validation errors prevent pipeline activation. The set of validation errors that can be returned are defined by AWS Data Pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ValidationError {
    /// <p>A description of the validation error.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// <p>The identifier of the object that contains the validation error.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Defines a validation warning. Validation warnings do not prevent pipeline activation. The set of validation warnings that can be returned are defined by AWS Data Pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ValidationWarning {
    /// <p>The identifier of the object that contains the validation warning.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A description of the validation warning.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Errors returned by ActivatePipeline
#[derive(Debug, PartialEq)]
pub enum ActivatePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ActivatePipelineError {
    pub fn from_body(body: &str) -> ActivatePipelineError {
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
                    "InternalServiceError" => {
                        ActivatePipelineError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ActivatePipelineError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        ActivatePipelineError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        ActivatePipelineError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ActivatePipelineError::Validation(error_message.to_string())
                    }
                    _ => ActivatePipelineError::Unknown(String::from(body)),
                }
            }
            Err(_) => ActivatePipelineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ActivatePipelineError {
    fn from(err: serde_json::error::Error) -> ActivatePipelineError {
        ActivatePipelineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ActivatePipelineError {
    fn from(err: CredentialsError) -> ActivatePipelineError {
        ActivatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ActivatePipelineError {
    fn from(err: HttpDispatchError) -> ActivatePipelineError {
        ActivatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for ActivatePipelineError {
    fn from(err: io::Error) -> ActivatePipelineError {
        ActivatePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ActivatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ActivatePipelineError {
    fn description(&self) -> &str {
        match *self {
            ActivatePipelineError::InternalServiceError(ref cause) => cause,
            ActivatePipelineError::InvalidRequest(ref cause) => cause,
            ActivatePipelineError::PipelineDeleted(ref cause) => cause,
            ActivatePipelineError::PipelineNotFound(ref cause) => cause,
            ActivatePipelineError::Validation(ref cause) => cause,
            ActivatePipelineError::Credentials(ref err) => err.description(),
            ActivatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ActivatePipelineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddTagsError {
    pub fn from_body(body: &str) -> AddTagsError {
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
                    "InternalServiceError" => {
                        AddTagsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AddTagsError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        AddTagsError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        AddTagsError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => AddTagsError::Validation(error_message.to_string()),
                    _ => AddTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsError {
    fn from(err: serde_json::error::Error) -> AddTagsError {
        AddTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddTagsError {
    fn from(err: CredentialsError) -> AddTagsError {
        AddTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsError {
    fn from(err: HttpDispatchError) -> AddTagsError {
        AddTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddTagsError {
    fn from(err: io::Error) -> AddTagsError {
        AddTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsError {
    fn description(&self) -> &str {
        match *self {
            AddTagsError::InternalServiceError(ref cause) => cause,
            AddTagsError::InvalidRequest(ref cause) => cause,
            AddTagsError::PipelineDeleted(ref cause) => cause,
            AddTagsError::PipelineNotFound(ref cause) => cause,
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePipeline
#[derive(Debug, PartialEq)]
pub enum CreatePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
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

impl CreatePipelineError {
    pub fn from_body(body: &str) -> CreatePipelineError {
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
                    "InternalServiceError" => {
                        CreatePipelineError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreatePipelineError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePipelineError::Validation(error_message.to_string())
                    }
                    _ => CreatePipelineError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePipelineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePipelineError {
    fn from(err: serde_json::error::Error) -> CreatePipelineError {
        CreatePipelineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePipelineError {
    fn from(err: CredentialsError) -> CreatePipelineError {
        CreatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePipelineError {
    fn from(err: HttpDispatchError) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePipelineError {
    fn from(err: io::Error) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePipelineError {
    fn description(&self) -> &str {
        match *self {
            CreatePipelineError::InternalServiceError(ref cause) => cause,
            CreatePipelineError::InvalidRequest(ref cause) => cause,
            CreatePipelineError::Validation(ref cause) => cause,
            CreatePipelineError::Credentials(ref err) => err.description(),
            CreatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePipelineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeactivatePipeline
#[derive(Debug, PartialEq)]
pub enum DeactivatePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeactivatePipelineError {
    pub fn from_body(body: &str) -> DeactivatePipelineError {
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
                    "InternalServiceError" => {
                        DeactivatePipelineError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeactivatePipelineError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        DeactivatePipelineError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        DeactivatePipelineError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeactivatePipelineError::Validation(error_message.to_string())
                    }
                    _ => DeactivatePipelineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeactivatePipelineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeactivatePipelineError {
    fn from(err: serde_json::error::Error) -> DeactivatePipelineError {
        DeactivatePipelineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeactivatePipelineError {
    fn from(err: CredentialsError) -> DeactivatePipelineError {
        DeactivatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeactivatePipelineError {
    fn from(err: HttpDispatchError) -> DeactivatePipelineError {
        DeactivatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeactivatePipelineError {
    fn from(err: io::Error) -> DeactivatePipelineError {
        DeactivatePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeactivatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeactivatePipelineError {
    fn description(&self) -> &str {
        match *self {
            DeactivatePipelineError::InternalServiceError(ref cause) => cause,
            DeactivatePipelineError::InvalidRequest(ref cause) => cause,
            DeactivatePipelineError::PipelineDeleted(ref cause) => cause,
            DeactivatePipelineError::PipelineNotFound(ref cause) => cause,
            DeactivatePipelineError::Validation(ref cause) => cause,
            DeactivatePipelineError::Credentials(ref err) => err.description(),
            DeactivatePipelineError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeactivatePipelineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePipeline
#[derive(Debug, PartialEq)]
pub enum DeletePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePipelineError {
    pub fn from_body(body: &str) -> DeletePipelineError {
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
                    "InternalServiceError" => {
                        DeletePipelineError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeletePipelineError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        DeletePipelineError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePipelineError::Validation(error_message.to_string())
                    }
                    _ => DeletePipelineError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePipelineError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePipelineError {
    fn from(err: serde_json::error::Error) -> DeletePipelineError {
        DeletePipelineError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePipelineError {
    fn from(err: CredentialsError) -> DeletePipelineError {
        DeletePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePipelineError {
    fn from(err: HttpDispatchError) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePipelineError {
    fn from(err: io::Error) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePipelineError {
    fn description(&self) -> &str {
        match *self {
            DeletePipelineError::InternalServiceError(ref cause) => cause,
            DeletePipelineError::InvalidRequest(ref cause) => cause,
            DeletePipelineError::PipelineNotFound(ref cause) => cause,
            DeletePipelineError::Validation(ref cause) => cause,
            DeletePipelineError::Credentials(ref err) => err.description(),
            DeletePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePipelineError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeObjects
#[derive(Debug, PartialEq)]
pub enum DescribeObjectsError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeObjectsError {
    pub fn from_body(body: &str) -> DescribeObjectsError {
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
                    "InternalServiceError" => {
                        DescribeObjectsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeObjectsError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        DescribeObjectsError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        DescribeObjectsError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeObjectsError::Validation(error_message.to_string())
                    }
                    _ => DescribeObjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeObjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeObjectsError {
    fn from(err: serde_json::error::Error) -> DescribeObjectsError {
        DescribeObjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeObjectsError {
    fn from(err: CredentialsError) -> DescribeObjectsError {
        DescribeObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeObjectsError {
    fn from(err: HttpDispatchError) -> DescribeObjectsError {
        DescribeObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeObjectsError {
    fn from(err: io::Error) -> DescribeObjectsError {
        DescribeObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeObjectsError {
    fn description(&self) -> &str {
        match *self {
            DescribeObjectsError::InternalServiceError(ref cause) => cause,
            DescribeObjectsError::InvalidRequest(ref cause) => cause,
            DescribeObjectsError::PipelineDeleted(ref cause) => cause,
            DescribeObjectsError::PipelineNotFound(ref cause) => cause,
            DescribeObjectsError::Validation(ref cause) => cause,
            DescribeObjectsError::Credentials(ref err) => err.description(),
            DescribeObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePipelines
#[derive(Debug, PartialEq)]
pub enum DescribePipelinesError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribePipelinesError {
    pub fn from_body(body: &str) -> DescribePipelinesError {
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
                    "InternalServiceError" => {
                        DescribePipelinesError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribePipelinesError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        DescribePipelinesError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        DescribePipelinesError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribePipelinesError::Validation(error_message.to_string())
                    }
                    _ => DescribePipelinesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribePipelinesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribePipelinesError {
    fn from(err: serde_json::error::Error) -> DescribePipelinesError {
        DescribePipelinesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePipelinesError {
    fn from(err: CredentialsError) -> DescribePipelinesError {
        DescribePipelinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePipelinesError {
    fn from(err: HttpDispatchError) -> DescribePipelinesError {
        DescribePipelinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePipelinesError {
    fn from(err: io::Error) -> DescribePipelinesError {
        DescribePipelinesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribePipelinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePipelinesError {
    fn description(&self) -> &str {
        match *self {
            DescribePipelinesError::InternalServiceError(ref cause) => cause,
            DescribePipelinesError::InvalidRequest(ref cause) => cause,
            DescribePipelinesError::PipelineDeleted(ref cause) => cause,
            DescribePipelinesError::PipelineNotFound(ref cause) => cause,
            DescribePipelinesError::Validation(ref cause) => cause,
            DescribePipelinesError::Credentials(ref err) => err.description(),
            DescribePipelinesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribePipelinesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EvaluateExpression
#[derive(Debug, PartialEq)]
pub enum EvaluateExpressionError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// <p>The specified task was not found. </p>
    TaskNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EvaluateExpressionError {
    pub fn from_body(body: &str) -> EvaluateExpressionError {
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
                    "InternalServiceError" => {
                        EvaluateExpressionError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        EvaluateExpressionError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        EvaluateExpressionError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        EvaluateExpressionError::PipelineNotFound(String::from(error_message))
                    }
                    "TaskNotFoundException" => {
                        EvaluateExpressionError::TaskNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        EvaluateExpressionError::Validation(error_message.to_string())
                    }
                    _ => EvaluateExpressionError::Unknown(String::from(body)),
                }
            }
            Err(_) => EvaluateExpressionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EvaluateExpressionError {
    fn from(err: serde_json::error::Error) -> EvaluateExpressionError {
        EvaluateExpressionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EvaluateExpressionError {
    fn from(err: CredentialsError) -> EvaluateExpressionError {
        EvaluateExpressionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EvaluateExpressionError {
    fn from(err: HttpDispatchError) -> EvaluateExpressionError {
        EvaluateExpressionError::HttpDispatch(err)
    }
}
impl From<io::Error> for EvaluateExpressionError {
    fn from(err: io::Error) -> EvaluateExpressionError {
        EvaluateExpressionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EvaluateExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EvaluateExpressionError {
    fn description(&self) -> &str {
        match *self {
            EvaluateExpressionError::InternalServiceError(ref cause) => cause,
            EvaluateExpressionError::InvalidRequest(ref cause) => cause,
            EvaluateExpressionError::PipelineDeleted(ref cause) => cause,
            EvaluateExpressionError::PipelineNotFound(ref cause) => cause,
            EvaluateExpressionError::TaskNotFound(ref cause) => cause,
            EvaluateExpressionError::Validation(ref cause) => cause,
            EvaluateExpressionError::Credentials(ref err) => err.description(),
            EvaluateExpressionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EvaluateExpressionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPipelineDefinition
#[derive(Debug, PartialEq)]
pub enum GetPipelineDefinitionError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPipelineDefinitionError {
    pub fn from_body(body: &str) -> GetPipelineDefinitionError {
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
                    "InternalServiceError" => GetPipelineDefinitionError::InternalServiceError(
                        String::from(error_message),
                    ),
                    "InvalidRequestException" => {
                        GetPipelineDefinitionError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        GetPipelineDefinitionError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        GetPipelineDefinitionError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPipelineDefinitionError::Validation(error_message.to_string())
                    }
                    _ => GetPipelineDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPipelineDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPipelineDefinitionError {
    fn from(err: serde_json::error::Error) -> GetPipelineDefinitionError {
        GetPipelineDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPipelineDefinitionError {
    fn from(err: CredentialsError) -> GetPipelineDefinitionError {
        GetPipelineDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPipelineDefinitionError {
    fn from(err: HttpDispatchError) -> GetPipelineDefinitionError {
        GetPipelineDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPipelineDefinitionError {
    fn from(err: io::Error) -> GetPipelineDefinitionError {
        GetPipelineDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPipelineDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPipelineDefinitionError {
    fn description(&self) -> &str {
        match *self {
            GetPipelineDefinitionError::InternalServiceError(ref cause) => cause,
            GetPipelineDefinitionError::InvalidRequest(ref cause) => cause,
            GetPipelineDefinitionError::PipelineDeleted(ref cause) => cause,
            GetPipelineDefinitionError::PipelineNotFound(ref cause) => cause,
            GetPipelineDefinitionError::Validation(ref cause) => cause,
            GetPipelineDefinitionError::Credentials(ref err) => err.description(),
            GetPipelineDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetPipelineDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPipelines
#[derive(Debug, PartialEq)]
pub enum ListPipelinesError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
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

impl ListPipelinesError {
    pub fn from_body(body: &str) -> ListPipelinesError {
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
                    "InternalServiceError" => {
                        ListPipelinesError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPipelinesError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPipelinesError::Validation(error_message.to_string())
                    }
                    _ => ListPipelinesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPipelinesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPipelinesError {
    fn from(err: serde_json::error::Error) -> ListPipelinesError {
        ListPipelinesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPipelinesError {
    fn from(err: CredentialsError) -> ListPipelinesError {
        ListPipelinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPipelinesError {
    fn from(err: HttpDispatchError) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPipelinesError {
    fn from(err: io::Error) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPipelinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelinesError {
    fn description(&self) -> &str {
        match *self {
            ListPipelinesError::InternalServiceError(ref cause) => cause,
            ListPipelinesError::InvalidRequest(ref cause) => cause,
            ListPipelinesError::Validation(ref cause) => cause,
            ListPipelinesError::Credentials(ref err) => err.description(),
            ListPipelinesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPipelinesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PollForTask
#[derive(Debug, PartialEq)]
pub enum PollForTaskError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified task was not found. </p>
    TaskNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PollForTaskError {
    pub fn from_body(body: &str) -> PollForTaskError {
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
                    "InternalServiceError" => {
                        PollForTaskError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        PollForTaskError::InvalidRequest(String::from(error_message))
                    }
                    "TaskNotFoundException" => {
                        PollForTaskError::TaskNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PollForTaskError::Validation(error_message.to_string())
                    }
                    _ => PollForTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => PollForTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PollForTaskError {
    fn from(err: serde_json::error::Error) -> PollForTaskError {
        PollForTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PollForTaskError {
    fn from(err: CredentialsError) -> PollForTaskError {
        PollForTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PollForTaskError {
    fn from(err: HttpDispatchError) -> PollForTaskError {
        PollForTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for PollForTaskError {
    fn from(err: io::Error) -> PollForTaskError {
        PollForTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PollForTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PollForTaskError {
    fn description(&self) -> &str {
        match *self {
            PollForTaskError::InternalServiceError(ref cause) => cause,
            PollForTaskError::InvalidRequest(ref cause) => cause,
            PollForTaskError::TaskNotFound(ref cause) => cause,
            PollForTaskError::Validation(ref cause) => cause,
            PollForTaskError::Credentials(ref err) => err.description(),
            PollForTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PollForTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutPipelineDefinition
#[derive(Debug, PartialEq)]
pub enum PutPipelineDefinitionError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutPipelineDefinitionError {
    pub fn from_body(body: &str) -> PutPipelineDefinitionError {
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
                    "InternalServiceError" => PutPipelineDefinitionError::InternalServiceError(
                        String::from(error_message),
                    ),
                    "InvalidRequestException" => {
                        PutPipelineDefinitionError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        PutPipelineDefinitionError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        PutPipelineDefinitionError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutPipelineDefinitionError::Validation(error_message.to_string())
                    }
                    _ => PutPipelineDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutPipelineDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutPipelineDefinitionError {
    fn from(err: serde_json::error::Error) -> PutPipelineDefinitionError {
        PutPipelineDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutPipelineDefinitionError {
    fn from(err: CredentialsError) -> PutPipelineDefinitionError {
        PutPipelineDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutPipelineDefinitionError {
    fn from(err: HttpDispatchError) -> PutPipelineDefinitionError {
        PutPipelineDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutPipelineDefinitionError {
    fn from(err: io::Error) -> PutPipelineDefinitionError {
        PutPipelineDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutPipelineDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutPipelineDefinitionError {
    fn description(&self) -> &str {
        match *self {
            PutPipelineDefinitionError::InternalServiceError(ref cause) => cause,
            PutPipelineDefinitionError::InvalidRequest(ref cause) => cause,
            PutPipelineDefinitionError::PipelineDeleted(ref cause) => cause,
            PutPipelineDefinitionError::PipelineNotFound(ref cause) => cause,
            PutPipelineDefinitionError::Validation(ref cause) => cause,
            PutPipelineDefinitionError::Credentials(ref err) => err.description(),
            PutPipelineDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutPipelineDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by QueryObjects
#[derive(Debug, PartialEq)]
pub enum QueryObjectsError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl QueryObjectsError {
    pub fn from_body(body: &str) -> QueryObjectsError {
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
                    "InternalServiceError" => {
                        QueryObjectsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        QueryObjectsError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        QueryObjectsError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        QueryObjectsError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        QueryObjectsError::Validation(error_message.to_string())
                    }
                    _ => QueryObjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => QueryObjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for QueryObjectsError {
    fn from(err: serde_json::error::Error) -> QueryObjectsError {
        QueryObjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for QueryObjectsError {
    fn from(err: CredentialsError) -> QueryObjectsError {
        QueryObjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for QueryObjectsError {
    fn from(err: HttpDispatchError) -> QueryObjectsError {
        QueryObjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for QueryObjectsError {
    fn from(err: io::Error) -> QueryObjectsError {
        QueryObjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for QueryObjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for QueryObjectsError {
    fn description(&self) -> &str {
        match *self {
            QueryObjectsError::InternalServiceError(ref cause) => cause,
            QueryObjectsError::InvalidRequest(ref cause) => cause,
            QueryObjectsError::PipelineDeleted(ref cause) => cause,
            QueryObjectsError::PipelineNotFound(ref cause) => cause,
            QueryObjectsError::Validation(ref cause) => cause,
            QueryObjectsError::Credentials(ref err) => err.description(),
            QueryObjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            QueryObjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveTagsError {
    pub fn from_body(body: &str) -> RemoveTagsError {
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
                    "InternalServiceError" => {
                        RemoveTagsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RemoveTagsError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        RemoveTagsError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        RemoveTagsError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => RemoveTagsError::Validation(error_message.to_string()),
                    _ => RemoveTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsError {
    fn from(err: serde_json::error::Error) -> RemoveTagsError {
        RemoveTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTagsError {
    fn from(err: CredentialsError) -> RemoveTagsError {
        RemoveTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsError {
    fn from(err: HttpDispatchError) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveTagsError {
    fn from(err: io::Error) -> RemoveTagsError {
        RemoveTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsError::InternalServiceError(ref cause) => cause,
            RemoveTagsError::InvalidRequest(ref cause) => cause,
            RemoveTagsError::PipelineDeleted(ref cause) => cause,
            RemoveTagsError::PipelineNotFound(ref cause) => cause,
            RemoveTagsError::Validation(ref cause) => cause,
            RemoveTagsError::Credentials(ref err) => err.description(),
            RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReportTaskProgress
#[derive(Debug, PartialEq)]
pub enum ReportTaskProgressError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// <p>The specified task was not found. </p>
    TaskNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ReportTaskProgressError {
    pub fn from_body(body: &str) -> ReportTaskProgressError {
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
                    "InternalServiceError" => {
                        ReportTaskProgressError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ReportTaskProgressError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        ReportTaskProgressError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        ReportTaskProgressError::PipelineNotFound(String::from(error_message))
                    }
                    "TaskNotFoundException" => {
                        ReportTaskProgressError::TaskNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReportTaskProgressError::Validation(error_message.to_string())
                    }
                    _ => ReportTaskProgressError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReportTaskProgressError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReportTaskProgressError {
    fn from(err: serde_json::error::Error) -> ReportTaskProgressError {
        ReportTaskProgressError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReportTaskProgressError {
    fn from(err: CredentialsError) -> ReportTaskProgressError {
        ReportTaskProgressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReportTaskProgressError {
    fn from(err: HttpDispatchError) -> ReportTaskProgressError {
        ReportTaskProgressError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReportTaskProgressError {
    fn from(err: io::Error) -> ReportTaskProgressError {
        ReportTaskProgressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReportTaskProgressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReportTaskProgressError {
    fn description(&self) -> &str {
        match *self {
            ReportTaskProgressError::InternalServiceError(ref cause) => cause,
            ReportTaskProgressError::InvalidRequest(ref cause) => cause,
            ReportTaskProgressError::PipelineDeleted(ref cause) => cause,
            ReportTaskProgressError::PipelineNotFound(ref cause) => cause,
            ReportTaskProgressError::TaskNotFound(ref cause) => cause,
            ReportTaskProgressError::Validation(ref cause) => cause,
            ReportTaskProgressError::Credentials(ref err) => err.description(),
            ReportTaskProgressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ReportTaskProgressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReportTaskRunnerHeartbeat
#[derive(Debug, PartialEq)]
pub enum ReportTaskRunnerHeartbeatError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
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

impl ReportTaskRunnerHeartbeatError {
    pub fn from_body(body: &str) -> ReportTaskRunnerHeartbeatError {
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
                    "InternalServiceError" => ReportTaskRunnerHeartbeatError::InternalServiceError(
                        String::from(error_message),
                    ),
                    "InvalidRequestException" => {
                        ReportTaskRunnerHeartbeatError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReportTaskRunnerHeartbeatError::Validation(error_message.to_string())
                    }
                    _ => ReportTaskRunnerHeartbeatError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReportTaskRunnerHeartbeatError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReportTaskRunnerHeartbeatError {
    fn from(err: serde_json::error::Error) -> ReportTaskRunnerHeartbeatError {
        ReportTaskRunnerHeartbeatError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReportTaskRunnerHeartbeatError {
    fn from(err: CredentialsError) -> ReportTaskRunnerHeartbeatError {
        ReportTaskRunnerHeartbeatError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReportTaskRunnerHeartbeatError {
    fn from(err: HttpDispatchError) -> ReportTaskRunnerHeartbeatError {
        ReportTaskRunnerHeartbeatError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReportTaskRunnerHeartbeatError {
    fn from(err: io::Error) -> ReportTaskRunnerHeartbeatError {
        ReportTaskRunnerHeartbeatError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReportTaskRunnerHeartbeatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReportTaskRunnerHeartbeatError {
    fn description(&self) -> &str {
        match *self {
            ReportTaskRunnerHeartbeatError::InternalServiceError(ref cause) => cause,
            ReportTaskRunnerHeartbeatError::InvalidRequest(ref cause) => cause,
            ReportTaskRunnerHeartbeatError::Validation(ref cause) => cause,
            ReportTaskRunnerHeartbeatError::Credentials(ref err) => err.description(),
            ReportTaskRunnerHeartbeatError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ReportTaskRunnerHeartbeatError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetStatus
#[derive(Debug, PartialEq)]
pub enum SetStatusError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetStatusError {
    pub fn from_body(body: &str) -> SetStatusError {
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
                    "InternalServiceError" => {
                        SetStatusError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetStatusError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        SetStatusError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        SetStatusError::PipelineNotFound(String::from(error_message))
                    }
                    "ValidationException" => SetStatusError::Validation(error_message.to_string()),
                    _ => SetStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetStatusError {
    fn from(err: serde_json::error::Error) -> SetStatusError {
        SetStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetStatusError {
    fn from(err: CredentialsError) -> SetStatusError {
        SetStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetStatusError {
    fn from(err: HttpDispatchError) -> SetStatusError {
        SetStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetStatusError {
    fn from(err: io::Error) -> SetStatusError {
        SetStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetStatusError {
    fn description(&self) -> &str {
        match *self {
            SetStatusError::InternalServiceError(ref cause) => cause,
            SetStatusError::InvalidRequest(ref cause) => cause,
            SetStatusError::PipelineDeleted(ref cause) => cause,
            SetStatusError::PipelineNotFound(ref cause) => cause,
            SetStatusError::Validation(ref cause) => cause,
            SetStatusError::Credentials(ref err) => err.description(),
            SetStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTaskStatus
#[derive(Debug, PartialEq)]
pub enum SetTaskStatusError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// <p>The specified task was not found. </p>
    TaskNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetTaskStatusError {
    pub fn from_body(body: &str) -> SetTaskStatusError {
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
                    "InternalServiceError" => {
                        SetTaskStatusError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetTaskStatusError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => {
                        SetTaskStatusError::PipelineDeleted(String::from(error_message))
                    }
                    "PipelineNotFoundException" => {
                        SetTaskStatusError::PipelineNotFound(String::from(error_message))
                    }
                    "TaskNotFoundException" => {
                        SetTaskStatusError::TaskNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetTaskStatusError::Validation(error_message.to_string())
                    }
                    _ => SetTaskStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetTaskStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetTaskStatusError {
    fn from(err: serde_json::error::Error) -> SetTaskStatusError {
        SetTaskStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetTaskStatusError {
    fn from(err: CredentialsError) -> SetTaskStatusError {
        SetTaskStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetTaskStatusError {
    fn from(err: HttpDispatchError) -> SetTaskStatusError {
        SetTaskStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetTaskStatusError {
    fn from(err: io::Error) -> SetTaskStatusError {
        SetTaskStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetTaskStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTaskStatusError {
    fn description(&self) -> &str {
        match *self {
            SetTaskStatusError::InternalServiceError(ref cause) => cause,
            SetTaskStatusError::InvalidRequest(ref cause) => cause,
            SetTaskStatusError::PipelineDeleted(ref cause) => cause,
            SetTaskStatusError::PipelineNotFound(ref cause) => cause,
            SetTaskStatusError::TaskNotFound(ref cause) => cause,
            SetTaskStatusError::Validation(ref cause) => cause,
            SetTaskStatusError::Credentials(ref err) => err.description(),
            SetTaskStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetTaskStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ValidatePipelineDefinition
#[derive(Debug, PartialEq)]
pub enum ValidatePipelineDefinitionError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeleted(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ValidatePipelineDefinitionError {
    pub fn from_body(body: &str) -> ValidatePipelineDefinitionError {
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
                    "InternalServiceError" => {
                        ValidatePipelineDefinitionError::InternalServiceError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => {
                        ValidatePipelineDefinitionError::InvalidRequest(String::from(error_message))
                    }
                    "PipelineDeletedException" => ValidatePipelineDefinitionError::PipelineDeleted(
                        String::from(error_message),
                    ),
                    "PipelineNotFoundException" => {
                        ValidatePipelineDefinitionError::PipelineNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ValidatePipelineDefinitionError::Validation(error_message.to_string())
                    }
                    _ => ValidatePipelineDefinitionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ValidatePipelineDefinitionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ValidatePipelineDefinitionError {
    fn from(err: serde_json::error::Error) -> ValidatePipelineDefinitionError {
        ValidatePipelineDefinitionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ValidatePipelineDefinitionError {
    fn from(err: CredentialsError) -> ValidatePipelineDefinitionError {
        ValidatePipelineDefinitionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ValidatePipelineDefinitionError {
    fn from(err: HttpDispatchError) -> ValidatePipelineDefinitionError {
        ValidatePipelineDefinitionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ValidatePipelineDefinitionError {
    fn from(err: io::Error) -> ValidatePipelineDefinitionError {
        ValidatePipelineDefinitionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ValidatePipelineDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ValidatePipelineDefinitionError {
    fn description(&self) -> &str {
        match *self {
            ValidatePipelineDefinitionError::InternalServiceError(ref cause) => cause,
            ValidatePipelineDefinitionError::InvalidRequest(ref cause) => cause,
            ValidatePipelineDefinitionError::PipelineDeleted(ref cause) => cause,
            ValidatePipelineDefinitionError::PipelineNotFound(ref cause) => cause,
            ValidatePipelineDefinitionError::Validation(ref cause) => cause,
            ValidatePipelineDefinitionError::Credentials(ref err) => err.description(),
            ValidatePipelineDefinitionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ValidatePipelineDefinitionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Data Pipeline API. AWS Data Pipeline clients implement this trait.
pub trait DataPipeline {
    /// <p>Validates the specified pipeline and starts processing pipeline tasks. If the pipeline does not pass validation, activation fails.</p> <p>If you need to pause the pipeline to investigate an issue with a component, such as a data source or script, call <a>DeactivatePipeline</a>.</p> <p>To activate a finished pipeline, modify the end date for the pipeline and then activate it.</p>
    fn activate_pipeline(
        &self,
        input: ActivatePipelineInput,
    ) -> RusotoFuture<ActivatePipelineOutput, ActivatePipelineError>;

    /// <p>Adds or modifies tags for the specified pipeline.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Creates a new, empty pipeline. Use <a>PutPipelineDefinition</a> to populate the pipeline.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError>;

    /// <p>Deactivates the specified running pipeline. The pipeline is set to the <code>DEACTIVATING</code> state until the deactivation process completes.</p> <p>To resume a deactivated pipeline, use <a>ActivatePipeline</a>. By default, the pipeline resumes from the last completed execution. Optionally, you can specify the date and time to resume the pipeline.</p>
    fn deactivate_pipeline(
        &self,
        input: DeactivatePipelineInput,
    ) -> RusotoFuture<DeactivatePipelineOutput, DeactivatePipelineError>;

    /// <p>Deletes a pipeline, its pipeline definition, and its run history. AWS Data Pipeline attempts to cancel instances associated with the pipeline that are currently being processed by task runners.</p> <p>Deleting a pipeline cannot be undone. You cannot query or restore a deleted pipeline. To temporarily pause a pipeline instead of deleting it, call <a>SetStatus</a> with the status set to <code>PAUSE</code> on individual components. Components that are paused by <a>SetStatus</a> can be resumed.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError>;

    /// <p>Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object.</p>
    fn describe_objects(
        &self,
        input: DescribeObjectsInput,
    ) -> RusotoFuture<DescribeObjectsOutput, DescribeObjectsError>;

    /// <p>Retrieves metadata about one or more pipelines. The information retrieved includes the name of the pipeline, the pipeline identifier, its current state, and the user account that owns the pipeline. Using account credentials, you can retrieve metadata about pipelines that you or your IAM users have created. If you are using an IAM user account, you can retrieve metadata about only those pipelines for which you have read permissions.</p> <p>To retrieve the full pipeline definition instead of metadata about the pipeline, call <a>GetPipelineDefinition</a>.</p>
    fn describe_pipelines(
        &self,
        input: DescribePipelinesInput,
    ) -> RusotoFuture<DescribePipelinesOutput, DescribePipelinesError>;

    /// <p>Task runners call <code>EvaluateExpression</code> to evaluate a string in the context of the specified object. For example, a task runner can evaluate SQL queries stored in Amazon S3.</p>
    fn evaluate_expression(
        &self,
        input: EvaluateExpressionInput,
    ) -> RusotoFuture<EvaluateExpressionOutput, EvaluateExpressionError>;

    /// <p>Gets the definition of the specified pipeline. You can call <code>GetPipelineDefinition</code> to retrieve the pipeline definition that you provided using <a>PutPipelineDefinition</a>.</p>
    fn get_pipeline_definition(
        &self,
        input: GetPipelineDefinitionInput,
    ) -> RusotoFuture<GetPipelineDefinitionOutput, GetPipelineDefinitionError>;

    /// <p>Lists the pipeline identifiers for all active pipelines that you have permission to access.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError>;

    /// <p>Task runners call <code>PollForTask</code> to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the <code>workerGroup</code> parameter. The task returned can come from any of the pipelines that match the <code>workerGroup</code> value passed in by the task runner and that was launched using the IAM user credentials specified by the task runner.</p> <p>If tasks are ready in the work queue, <code>PollForTask</code> returns a response immediately. If no tasks are available in the queue, <code>PollForTask</code> uses long-polling and holds on to a poll connection for up to a 90 seconds, during which time the first newly scheduled task is handed to the task runner. To accomodate this, set the socket timeout in your task runner to 90 seconds. The task runner should not call <code>PollForTask</code> again on the same <code>workerGroup</code> until it receives a response, and this can take up to 90 seconds. </p>
    fn poll_for_task(
        &self,
        input: PollForTaskInput,
    ) -> RusotoFuture<PollForTaskOutput, PollForTaskError>;

    /// <p>Adds tasks, schedules, and preconditions to the specified pipeline. You can use <code>PutPipelineDefinition</code> to populate a new pipeline.</p> <p> <code>PutPipelineDefinition</code> also validates the configuration as it adds it to the pipeline. Changes to the pipeline are saved unless one of the following three validation errors exists in the pipeline. </p> <ol> <li>An object is missing a name or identifier field.</li> <li>A string or reference field is empty.</li> <li>The number of objects in the pipeline exceeds the maximum allowed objects.</li> <li>The pipeline is in a FINISHED state.</li> </ol> <p> Pipeline object definitions are passed to the <code>PutPipelineDefinition</code> action and returned by the <a>GetPipelineDefinition</a> action. </p>
    fn put_pipeline_definition(
        &self,
        input: PutPipelineDefinitionInput,
    ) -> RusotoFuture<PutPipelineDefinitionOutput, PutPipelineDefinitionError>;

    /// <p>Queries the specified pipeline for the names of objects that match the specified set of conditions.</p>
    fn query_objects(
        &self,
        input: QueryObjectsInput,
    ) -> RusotoFuture<QueryObjectsOutput, QueryObjectsError>;

    /// <p>Removes existing tags from the specified pipeline.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError>;

    /// <p>Task runners call <code>ReportTaskProgress</code> when assigned a task to acknowledge that it has the task. If the web service does not receive this acknowledgement within 2 minutes, it assigns the task in a subsequent <a>PollForTask</a> call. After this initial acknowledgement, the task runner only needs to report progress every 15 minutes to maintain its ownership of the task. You can change this reporting time from 15 minutes by specifying a <code>reportProgressTimeout</code> field in your pipeline.</p> <p>If a task runner does not report its status after 5 minutes, AWS Data Pipeline assumes that the task runner is unable to process the task and reassigns the task in a subsequent response to <a>PollForTask</a>. Task runners should call <code>ReportTaskProgress</code> every 60 seconds.</p>
    fn report_task_progress(
        &self,
        input: ReportTaskProgressInput,
    ) -> RusotoFuture<ReportTaskProgressOutput, ReportTaskProgressError>;

    /// <p>Task runners call <code>ReportTaskRunnerHeartbeat</code> every 15 minutes to indicate that they are operational. If the AWS Data Pipeline Task Runner is launched on a resource managed by AWS Data Pipeline, the web service can use this call to detect when the task runner application has failed and restart a new instance.</p>
    fn report_task_runner_heartbeat(
        &self,
        input: ReportTaskRunnerHeartbeatInput,
    ) -> RusotoFuture<ReportTaskRunnerHeartbeatOutput, ReportTaskRunnerHeartbeatError>;

    /// <p>Requests that the status of the specified physical or logical pipeline objects be updated in the specified pipeline. This update might not occur immediately, but is eventually consistent. The status that can be set depends on the type of object (for example, DataNode or Activity). You cannot perform this operation on <code>FINISHED</code> pipelines and attempting to do so returns <code>InvalidRequestException</code>.</p>
    fn set_status(&self, input: SetStatusInput) -> RusotoFuture<(), SetStatusError>;

    /// <p>Task runners call <code>SetTaskStatus</code> to notify AWS Data Pipeline that a task is completed and provide information about the final status. A task runner makes this call regardless of whether the task was sucessful. A task runner does not need to call <code>SetTaskStatus</code> for tasks that are canceled by the web service during a call to <a>ReportTaskProgress</a>.</p>
    fn set_task_status(
        &self,
        input: SetTaskStatusInput,
    ) -> RusotoFuture<SetTaskStatusOutput, SetTaskStatusError>;

    /// <p>Validates the specified pipeline definition to ensure that it is well formed and can be run without error.</p>
    fn validate_pipeline_definition(
        &self,
        input: ValidatePipelineDefinitionInput,
    ) -> RusotoFuture<ValidatePipelineDefinitionOutput, ValidatePipelineDefinitionError>;
}
/// A client for the AWS Data Pipeline API.
pub struct DataPipelineClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl DataPipelineClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> DataPipelineClient {
        DataPipelineClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> DataPipelineClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DataPipelineClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> DataPipeline for DataPipelineClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Validates the specified pipeline and starts processing pipeline tasks. If the pipeline does not pass validation, activation fails.</p> <p>If you need to pause the pipeline to investigate an issue with a component, such as a data source or script, call <a>DeactivatePipeline</a>.</p> <p>To activate a finished pipeline, modify the end date for the pipeline and then activate it.</p>
    fn activate_pipeline(
        &self,
        input: ActivatePipelineInput,
    ) -> RusotoFuture<ActivatePipelineOutput, ActivatePipelineError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ActivatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ActivatePipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ActivatePipelineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or modifies tags for the specified pipeline.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.AddTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new, empty pipeline. Use <a>PutPipelineDefinition</a> to populate the pipeline.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> RusotoFuture<CreatePipelineOutput, CreatePipelineError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.CreatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreatePipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreatePipelineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deactivates the specified running pipeline. The pipeline is set to the <code>DEACTIVATING</code> state until the deactivation process completes.</p> <p>To resume a deactivated pipeline, use <a>ActivatePipeline</a>. By default, the pipeline resumes from the last completed execution. Optionally, you can specify the date and time to resume the pipeline.</p>
    fn deactivate_pipeline(
        &self,
        input: DeactivatePipelineInput,
    ) -> RusotoFuture<DeactivatePipelineOutput, DeactivatePipelineError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DeactivatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeactivatePipelineOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeactivatePipelineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a pipeline, its pipeline definition, and its run history. AWS Data Pipeline attempts to cancel instances associated with the pipeline that are currently being processed by task runners.</p> <p>Deleting a pipeline cannot be undone. You cannot query or restore a deleted pipeline. To temporarily pause a pipeline instead of deleting it, call <a>SetStatus</a> with the status set to <code>PAUSE</code> on individual components. Components that are paused by <a>SetStatus</a> can be resumed.</p>
    fn delete_pipeline(&self, input: DeletePipelineInput) -> RusotoFuture<(), DeletePipelineError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DeletePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeletePipelineError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object.</p>
    fn describe_objects(
        &self,
        input: DescribeObjectsInput,
    ) -> RusotoFuture<DescribeObjectsOutput, DescribeObjectsError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DescribeObjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeObjectsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeObjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves metadata about one or more pipelines. The information retrieved includes the name of the pipeline, the pipeline identifier, its current state, and the user account that owns the pipeline. Using account credentials, you can retrieve metadata about pipelines that you or your IAM users have created. If you are using an IAM user account, you can retrieve metadata about only those pipelines for which you have read permissions.</p> <p>To retrieve the full pipeline definition instead of metadata about the pipeline, call <a>GetPipelineDefinition</a>.</p>
    fn describe_pipelines(
        &self,
        input: DescribePipelinesInput,
    ) -> RusotoFuture<DescribePipelinesOutput, DescribePipelinesError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DescribePipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribePipelinesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribePipelinesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Task runners call <code>EvaluateExpression</code> to evaluate a string in the context of the specified object. For example, a task runner can evaluate SQL queries stored in Amazon S3.</p>
    fn evaluate_expression(
        &self,
        input: EvaluateExpressionInput,
    ) -> RusotoFuture<EvaluateExpressionOutput, EvaluateExpressionError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.EvaluateExpression");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EvaluateExpressionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EvaluateExpressionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the definition of the specified pipeline. You can call <code>GetPipelineDefinition</code> to retrieve the pipeline definition that you provided using <a>PutPipelineDefinition</a>.</p>
    fn get_pipeline_definition(
        &self,
        input: GetPipelineDefinitionInput,
    ) -> RusotoFuture<GetPipelineDefinitionOutput, GetPipelineDefinitionError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.GetPipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetPipelineDefinitionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetPipelineDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the pipeline identifiers for all active pipelines that you have permission to access.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> RusotoFuture<ListPipelinesOutput, ListPipelinesError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ListPipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListPipelinesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListPipelinesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Task runners call <code>PollForTask</code> to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the <code>workerGroup</code> parameter. The task returned can come from any of the pipelines that match the <code>workerGroup</code> value passed in by the task runner and that was launched using the IAM user credentials specified by the task runner.</p> <p>If tasks are ready in the work queue, <code>PollForTask</code> returns a response immediately. If no tasks are available in the queue, <code>PollForTask</code> uses long-polling and holds on to a poll connection for up to a 90 seconds, during which time the first newly scheduled task is handed to the task runner. To accomodate this, set the socket timeout in your task runner to 90 seconds. The task runner should not call <code>PollForTask</code> again on the same <code>workerGroup</code> until it receives a response, and this can take up to 90 seconds. </p>
    fn poll_for_task(
        &self,
        input: PollForTaskInput,
    ) -> RusotoFuture<PollForTaskOutput, PollForTaskError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.PollForTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PollForTaskOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PollForTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds tasks, schedules, and preconditions to the specified pipeline. You can use <code>PutPipelineDefinition</code> to populate a new pipeline.</p> <p> <code>PutPipelineDefinition</code> also validates the configuration as it adds it to the pipeline. Changes to the pipeline are saved unless one of the following three validation errors exists in the pipeline. </p> <ol> <li>An object is missing a name or identifier field.</li> <li>A string or reference field is empty.</li> <li>The number of objects in the pipeline exceeds the maximum allowed objects.</li> <li>The pipeline is in a FINISHED state.</li> </ol> <p> Pipeline object definitions are passed to the <code>PutPipelineDefinition</code> action and returned by the <a>GetPipelineDefinition</a> action. </p>
    fn put_pipeline_definition(
        &self,
        input: PutPipelineDefinitionInput,
    ) -> RusotoFuture<PutPipelineDefinitionOutput, PutPipelineDefinitionError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.PutPipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutPipelineDefinitionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutPipelineDefinitionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Queries the specified pipeline for the names of objects that match the specified set of conditions.</p>
    fn query_objects(
        &self,
        input: QueryObjectsInput,
    ) -> RusotoFuture<QueryObjectsOutput, QueryObjectsError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.QueryObjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<QueryObjectsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(QueryObjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes existing tags from the specified pipeline.</p>
    fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> RusotoFuture<RemoveTagsOutput, RemoveTagsError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.RemoveTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RemoveTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RemoveTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Task runners call <code>ReportTaskProgress</code> when assigned a task to acknowledge that it has the task. If the web service does not receive this acknowledgement within 2 minutes, it assigns the task in a subsequent <a>PollForTask</a> call. After this initial acknowledgement, the task runner only needs to report progress every 15 minutes to maintain its ownership of the task. You can change this reporting time from 15 minutes by specifying a <code>reportProgressTimeout</code> field in your pipeline.</p> <p>If a task runner does not report its status after 5 minutes, AWS Data Pipeline assumes that the task runner is unable to process the task and reassigns the task in a subsequent response to <a>PollForTask</a>. Task runners should call <code>ReportTaskProgress</code> every 60 seconds.</p>
    fn report_task_progress(
        &self,
        input: ReportTaskProgressInput,
    ) -> RusotoFuture<ReportTaskProgressOutput, ReportTaskProgressError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ReportTaskProgress");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ReportTaskProgressOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReportTaskProgressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Task runners call <code>ReportTaskRunnerHeartbeat</code> every 15 minutes to indicate that they are operational. If the AWS Data Pipeline Task Runner is launched on a resource managed by AWS Data Pipeline, the web service can use this call to detect when the task runner application has failed and restart a new instance.</p>
    fn report_task_runner_heartbeat(
        &self,
        input: ReportTaskRunnerHeartbeatInput,
    ) -> RusotoFuture<ReportTaskRunnerHeartbeatOutput, ReportTaskRunnerHeartbeatError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ReportTaskRunnerHeartbeat");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ReportTaskRunnerHeartbeatOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReportTaskRunnerHeartbeatError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Requests that the status of the specified physical or logical pipeline objects be updated in the specified pipeline. This update might not occur immediately, but is eventually consistent. The status that can be set depends on the type of object (for example, DataNode or Activity). You cannot perform this operation on <code>FINISHED</code> pipelines and attempting to do so returns <code>InvalidRequestException</code>.</p>
    fn set_status(&self, input: SetStatusInput) -> RusotoFuture<(), SetStatusError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.SetStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Task runners call <code>SetTaskStatus</code> to notify AWS Data Pipeline that a task is completed and provide information about the final status. A task runner makes this call regardless of whether the task was sucessful. A task runner does not need to call <code>SetTaskStatus</code> for tasks that are canceled by the web service during a call to <a>ReportTaskProgress</a>.</p>
    fn set_task_status(
        &self,
        input: SetTaskStatusInput,
    ) -> RusotoFuture<SetTaskStatusOutput, SetTaskStatusError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.SetTaskStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<SetTaskStatusOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(SetTaskStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Validates the specified pipeline definition to ensure that it is well formed and can be run without error.</p>
    fn validate_pipeline_definition(
        &self,
        input: ValidatePipelineDefinitionInput,
    ) -> RusotoFuture<ValidatePipelineDefinitionOutput, ValidatePipelineDefinitionError> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ValidatePipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ValidatePipelineDefinitionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ValidatePipelineDefinitionError::from_body(
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
