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

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Contains the parameters for ActivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivatePipelineOutput {}

/// <p>Contains the parameters for AddTags.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsOutput {}

/// <p>Contains the parameters for CreatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePipelineOutput {
    /// <p>The ID that AWS Data Pipeline assigns the newly created pipeline. For example, <code>df-06372391ZG65EXAMPLE</code>.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the parameters for DeactivatePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeactivatePipelineOutput {}

/// <p>Contains the parameters for DeletePipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePipelineInput {
    /// <p>The ID of the pipeline.</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Contains the parameters for DescribeObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePipelinesInput {
    /// <p>The IDs of the pipelines to describe. You can pass as many as 25 identifiers in a single call. To obtain pipeline IDs, call <a>ListPipelines</a>.</p>
    #[serde(rename = "pipelineIds")]
    pub pipeline_ids: Vec<String>,
}

/// <p>Contains the output of DescribePipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePipelinesOutput {
    /// <p>An array of descriptions for the specified pipelines.</p>
    #[serde(rename = "pipelineDescriptionList")]
    pub pipeline_description_list: Vec<PipelineDescription>,
}

/// <p>Contains the parameters for EvaluateExpression.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPipelinesInput {
    /// <p>The starting point for the results to be returned. For the first call, this value should be empty. As long as there are more results, continue to call <code>ListPipelines</code> with the marker value from the previous call to retrieve the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>Contains the output of ListPipelines.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PollForTaskOutput {
    /// <p>The information needed to complete the task that is being assigned to the task runner. One of the fields returned in this object is <code>taskId</code>, which contains an identifier for the task being assigned. The calling task runner uses <code>taskId</code> in subsequent calls to <a>ReportTaskProgress</a> and <a>SetTaskStatus</a>.</p>
    #[serde(rename = "taskObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_object: Option<TaskObject>,
}

/// <p>Contains the parameters for PutPipelineDefinition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Query {
    /// <p>List of selectors that define the query. An object must satisfy all of the selectors to match the query.</p>
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<Selector>>,
}

/// <p>Contains the parameters for QueryObjects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsOutput {}

/// <p>Contains the parameters for ReportTaskProgress.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReportTaskProgressOutput {
    /// <p>If true, the calling task runner should cancel processing of the task. The task runner does not need to call <a>SetTaskStatus</a> for canceled tasks.</p>
    #[serde(rename = "canceled")]
    pub canceled: bool,
}

/// <p>Contains the parameters for ReportTaskRunnerHeartbeat.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReportTaskRunnerHeartbeatOutput {
    /// <p>Indicates whether the calling task runner should terminate.</p>
    #[serde(rename = "terminate")]
    pub terminate: bool,
}

/// <p>A comparision that is used to determine whether a query should return this object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl ActivatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ActivatePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ActivatePipelineError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ActivatePipelineError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(ActivatePipelineError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(ActivatePipelineError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ActivatePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivatePipelineError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ActivatePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ActivatePipelineError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            ActivatePipelineError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ActivatePipelineError {}
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
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(AddTagsError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddTagsError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(AddTagsError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(AddTagsError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AddTagsError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            AddTagsError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CreatePipeline
#[derive(Debug, PartialEq)]
pub enum CreatePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
}

impl CreatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(CreatePipelineError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePipelineError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            CreatePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePipelineError {}
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
}

impl DeactivatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeactivatePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DeactivatePipelineError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeactivatePipelineError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(DeactivatePipelineError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(DeactivatePipelineError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeactivatePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeactivatePipelineError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeactivatePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeactivatePipelineError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            DeactivatePipelineError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeactivatePipelineError {}
/// Errors returned by DeletePipeline
#[derive(Debug, PartialEq)]
pub enum DeletePipelineError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFound(String),
}

impl DeletePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePipelineError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DeletePipelineError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeletePipelineError::InvalidRequest(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(DeletePipelineError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePipelineError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePipelineError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DeletePipelineError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeletePipelineError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePipelineError {}
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
}

impl DescribeObjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeObjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeObjectsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeObjectsError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(DescribeObjectsError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(DescribeObjectsError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeObjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeObjectsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeObjectsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeObjectsError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            DescribeObjectsError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeObjectsError {}
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
}

impl DescribePipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePipelinesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribePipelinesError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePipelinesError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(DescribePipelinesError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(DescribePipelinesError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePipelinesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePipelinesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribePipelinesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribePipelinesError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            DescribePipelinesError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePipelinesError {}
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
}

impl EvaluateExpressionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EvaluateExpressionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(EvaluateExpressionError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(EvaluateExpressionError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(EvaluateExpressionError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(EvaluateExpressionError::PipelineNotFound(err.msg))
                }
                "TaskNotFoundException" => {
                    return RusotoError::Service(EvaluateExpressionError::TaskNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EvaluateExpressionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvaluateExpressionError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            EvaluateExpressionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            EvaluateExpressionError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            EvaluateExpressionError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
            EvaluateExpressionError::TaskNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EvaluateExpressionError {}
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
}

impl GetPipelineDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPipelineDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(GetPipelineDefinitionError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetPipelineDefinitionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(GetPipelineDefinitionError::PipelineDeleted(
                        err.msg,
                    ))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(GetPipelineDefinitionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPipelineDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPipelineDefinitionError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetPipelineDefinitionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetPipelineDefinitionError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            GetPipelineDefinitionError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPipelineDefinitionError {}
/// Errors returned by ListPipelines
#[derive(Debug, PartialEq)]
pub enum ListPipelinesError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
}

impl ListPipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPipelinesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ListPipelinesError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPipelinesError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPipelinesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPipelinesError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ListPipelinesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPipelinesError {}
/// Errors returned by PollForTask
#[derive(Debug, PartialEq)]
pub enum PollForTaskError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
    /// <p>The specified task was not found. </p>
    TaskNotFound(String),
}

impl PollForTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PollForTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(PollForTaskError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PollForTaskError::InvalidRequest(err.msg))
                }
                "TaskNotFoundException" => {
                    return RusotoError::Service(PollForTaskError::TaskNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PollForTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PollForTaskError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            PollForTaskError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PollForTaskError::TaskNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PollForTaskError {}
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
}

impl PutPipelineDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPipelineDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(PutPipelineDefinitionError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutPipelineDefinitionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(PutPipelineDefinitionError::PipelineDeleted(
                        err.msg,
                    ))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(PutPipelineDefinitionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPipelineDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPipelineDefinitionError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            PutPipelineDefinitionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutPipelineDefinitionError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            PutPipelineDefinitionError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPipelineDefinitionError {}
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
}

impl QueryObjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<QueryObjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(QueryObjectsError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(QueryObjectsError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(QueryObjectsError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(QueryObjectsError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for QueryObjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryObjectsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            QueryObjectsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            QueryObjectsError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            QueryObjectsError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for QueryObjectsError {}
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
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(RemoveTagsError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RemoveTagsError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(RemoveTagsError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(RemoveTagsError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
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
}

impl ReportTaskProgressError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReportTaskProgressError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(ReportTaskProgressError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ReportTaskProgressError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(ReportTaskProgressError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(ReportTaskProgressError::PipelineNotFound(err.msg))
                }
                "TaskNotFoundException" => {
                    return RusotoError::Service(ReportTaskProgressError::TaskNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReportTaskProgressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReportTaskProgressError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            ReportTaskProgressError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ReportTaskProgressError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            ReportTaskProgressError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
            ReportTaskProgressError::TaskNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReportTaskProgressError {}
/// Errors returned by ReportTaskRunnerHeartbeat
#[derive(Debug, PartialEq)]
pub enum ReportTaskRunnerHeartbeatError {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(String),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequest(String),
}

impl ReportTaskRunnerHeartbeatError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReportTaskRunnerHeartbeatError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(
                        ReportTaskRunnerHeartbeatError::InternalServiceError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ReportTaskRunnerHeartbeatError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReportTaskRunnerHeartbeatError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReportTaskRunnerHeartbeatError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ReportTaskRunnerHeartbeatError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReportTaskRunnerHeartbeatError {}
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
}

impl SetStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(SetStatusError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SetStatusError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(SetStatusError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(SetStatusError::PipelineNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetStatusError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            SetStatusError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SetStatusError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            SetStatusError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetStatusError {}
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
}

impl SetTaskStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetTaskStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(SetTaskStatusError::InternalServiceError(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SetTaskStatusError::InvalidRequest(err.msg))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(SetTaskStatusError::PipelineDeleted(err.msg))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(SetTaskStatusError::PipelineNotFound(err.msg))
                }
                "TaskNotFoundException" => {
                    return RusotoError::Service(SetTaskStatusError::TaskNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetTaskStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetTaskStatusError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            SetTaskStatusError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SetTaskStatusError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            SetTaskStatusError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
            SetTaskStatusError::TaskNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetTaskStatusError {}
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
}

impl ValidatePipelineDefinitionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ValidatePipelineDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(
                        ValidatePipelineDefinitionError::InternalServiceError(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ValidatePipelineDefinitionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "PipelineDeletedException" => {
                    return RusotoError::Service(ValidatePipelineDefinitionError::PipelineDeleted(
                        err.msg,
                    ))
                }
                "PipelineNotFoundException" => {
                    return RusotoError::Service(ValidatePipelineDefinitionError::PipelineNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ValidatePipelineDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidatePipelineDefinitionError::InternalServiceError(ref cause) => {
                write!(f, "{}", cause)
            }
            ValidatePipelineDefinitionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ValidatePipelineDefinitionError::PipelineDeleted(ref cause) => write!(f, "{}", cause),
            ValidatePipelineDefinitionError::PipelineNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ValidatePipelineDefinitionError {}
/// Trait representing the capabilities of the AWS Data Pipeline API. AWS Data Pipeline clients implement this trait.
#[async_trait]
pub trait DataPipeline {
    /// <p>Validates the specified pipeline and starts processing pipeline tasks. If the pipeline does not pass validation, activation fails.</p> <p>If you need to pause the pipeline to investigate an issue with a component, such as a data source or script, call <a>DeactivatePipeline</a>.</p> <p>To activate a finished pipeline, modify the end date for the pipeline and then activate it.</p>
    async fn activate_pipeline(
        &self,
        input: ActivatePipelineInput,
    ) -> Result<ActivatePipelineOutput, RusotoError<ActivatePipelineError>>;

    /// <p>Adds or modifies tags for the specified pipeline.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Creates a new, empty pipeline. Use <a>PutPipelineDefinition</a> to populate the pipeline.</p>
    async fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> Result<CreatePipelineOutput, RusotoError<CreatePipelineError>>;

    /// <p>Deactivates the specified running pipeline. The pipeline is set to the <code>DEACTIVATING</code> state until the deactivation process completes.</p> <p>To resume a deactivated pipeline, use <a>ActivatePipeline</a>. By default, the pipeline resumes from the last completed execution. Optionally, you can specify the date and time to resume the pipeline.</p>
    async fn deactivate_pipeline(
        &self,
        input: DeactivatePipelineInput,
    ) -> Result<DeactivatePipelineOutput, RusotoError<DeactivatePipelineError>>;

    /// <p>Deletes a pipeline, its pipeline definition, and its run history. AWS Data Pipeline attempts to cancel instances associated with the pipeline that are currently being processed by task runners.</p> <p>Deleting a pipeline cannot be undone. You cannot query or restore a deleted pipeline. To temporarily pause a pipeline instead of deleting it, call <a>SetStatus</a> with the status set to <code>PAUSE</code> on individual components. Components that are paused by <a>SetStatus</a> can be resumed.</p>
    async fn delete_pipeline(
        &self,
        input: DeletePipelineInput,
    ) -> Result<(), RusotoError<DeletePipelineError>>;

    /// <p>Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object.</p>
    async fn describe_objects(
        &self,
        input: DescribeObjectsInput,
    ) -> Result<DescribeObjectsOutput, RusotoError<DescribeObjectsError>>;

    /// <p>Retrieves metadata about one or more pipelines. The information retrieved includes the name of the pipeline, the pipeline identifier, its current state, and the user account that owns the pipeline. Using account credentials, you can retrieve metadata about pipelines that you or your IAM users have created. If you are using an IAM user account, you can retrieve metadata about only those pipelines for which you have read permissions.</p> <p>To retrieve the full pipeline definition instead of metadata about the pipeline, call <a>GetPipelineDefinition</a>.</p>
    async fn describe_pipelines(
        &self,
        input: DescribePipelinesInput,
    ) -> Result<DescribePipelinesOutput, RusotoError<DescribePipelinesError>>;

    /// <p>Task runners call <code>EvaluateExpression</code> to evaluate a string in the context of the specified object. For example, a task runner can evaluate SQL queries stored in Amazon S3.</p>
    async fn evaluate_expression(
        &self,
        input: EvaluateExpressionInput,
    ) -> Result<EvaluateExpressionOutput, RusotoError<EvaluateExpressionError>>;

    /// <p>Gets the definition of the specified pipeline. You can call <code>GetPipelineDefinition</code> to retrieve the pipeline definition that you provided using <a>PutPipelineDefinition</a>.</p>
    async fn get_pipeline_definition(
        &self,
        input: GetPipelineDefinitionInput,
    ) -> Result<GetPipelineDefinitionOutput, RusotoError<GetPipelineDefinitionError>>;

    /// <p>Lists the pipeline identifiers for all active pipelines that you have permission to access.</p>
    async fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> Result<ListPipelinesOutput, RusotoError<ListPipelinesError>>;

    /// <p>Task runners call <code>PollForTask</code> to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the <code>workerGroup</code> parameter. The task returned can come from any of the pipelines that match the <code>workerGroup</code> value passed in by the task runner and that was launched using the IAM user credentials specified by the task runner.</p> <p>If tasks are ready in the work queue, <code>PollForTask</code> returns a response immediately. If no tasks are available in the queue, <code>PollForTask</code> uses long-polling and holds on to a poll connection for up to a 90 seconds, during which time the first newly scheduled task is handed to the task runner. To accomodate this, set the socket timeout in your task runner to 90 seconds. The task runner should not call <code>PollForTask</code> again on the same <code>workerGroup</code> until it receives a response, and this can take up to 90 seconds. </p>
    async fn poll_for_task(
        &self,
        input: PollForTaskInput,
    ) -> Result<PollForTaskOutput, RusotoError<PollForTaskError>>;

    /// <p>Adds tasks, schedules, and preconditions to the specified pipeline. You can use <code>PutPipelineDefinition</code> to populate a new pipeline.</p> <p> <code>PutPipelineDefinition</code> also validates the configuration as it adds it to the pipeline. Changes to the pipeline are saved unless one of the following three validation errors exists in the pipeline. </p> <ol> <li>An object is missing a name or identifier field.</li> <li>A string or reference field is empty.</li> <li>The number of objects in the pipeline exceeds the maximum allowed objects.</li> <li>The pipeline is in a FINISHED state.</li> </ol> <p> Pipeline object definitions are passed to the <code>PutPipelineDefinition</code> action and returned by the <a>GetPipelineDefinition</a> action. </p>
    async fn put_pipeline_definition(
        &self,
        input: PutPipelineDefinitionInput,
    ) -> Result<PutPipelineDefinitionOutput, RusotoError<PutPipelineDefinitionError>>;

    /// <p>Queries the specified pipeline for the names of objects that match the specified set of conditions.</p>
    async fn query_objects(
        &self,
        input: QueryObjectsInput,
    ) -> Result<QueryObjectsOutput, RusotoError<QueryObjectsError>>;

    /// <p>Removes existing tags from the specified pipeline.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>>;

    /// <p>Task runners call <code>ReportTaskProgress</code> when assigned a task to acknowledge that it has the task. If the web service does not receive this acknowledgement within 2 minutes, it assigns the task in a subsequent <a>PollForTask</a> call. After this initial acknowledgement, the task runner only needs to report progress every 15 minutes to maintain its ownership of the task. You can change this reporting time from 15 minutes by specifying a <code>reportProgressTimeout</code> field in your pipeline.</p> <p>If a task runner does not report its status after 5 minutes, AWS Data Pipeline assumes that the task runner is unable to process the task and reassigns the task in a subsequent response to <a>PollForTask</a>. Task runners should call <code>ReportTaskProgress</code> every 60 seconds.</p>
    async fn report_task_progress(
        &self,
        input: ReportTaskProgressInput,
    ) -> Result<ReportTaskProgressOutput, RusotoError<ReportTaskProgressError>>;

    /// <p>Task runners call <code>ReportTaskRunnerHeartbeat</code> every 15 minutes to indicate that they are operational. If the AWS Data Pipeline Task Runner is launched on a resource managed by AWS Data Pipeline, the web service can use this call to detect when the task runner application has failed and restart a new instance.</p>
    async fn report_task_runner_heartbeat(
        &self,
        input: ReportTaskRunnerHeartbeatInput,
    ) -> Result<ReportTaskRunnerHeartbeatOutput, RusotoError<ReportTaskRunnerHeartbeatError>>;

    /// <p>Requests that the status of the specified physical or logical pipeline objects be updated in the specified pipeline. This update might not occur immediately, but is eventually consistent. The status that can be set depends on the type of object (for example, DataNode or Activity). You cannot perform this operation on <code>FINISHED</code> pipelines and attempting to do so returns <code>InvalidRequestException</code>.</p>
    async fn set_status(&self, input: SetStatusInput) -> Result<(), RusotoError<SetStatusError>>;

    /// <p>Task runners call <code>SetTaskStatus</code> to notify AWS Data Pipeline that a task is completed and provide information about the final status. A task runner makes this call regardless of whether the task was sucessful. A task runner does not need to call <code>SetTaskStatus</code> for tasks that are canceled by the web service during a call to <a>ReportTaskProgress</a>.</p>
    async fn set_task_status(
        &self,
        input: SetTaskStatusInput,
    ) -> Result<SetTaskStatusOutput, RusotoError<SetTaskStatusError>>;

    /// <p>Validates the specified pipeline definition to ensure that it is well formed and can be run without error.</p>
    async fn validate_pipeline_definition(
        &self,
        input: ValidatePipelineDefinitionInput,
    ) -> Result<ValidatePipelineDefinitionOutput, RusotoError<ValidatePipelineDefinitionError>>;
}
/// A client for the AWS Data Pipeline API.
#[derive(Clone)]
pub struct DataPipelineClient {
    client: Client,
    region: region::Region,
}

impl DataPipelineClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DataPipelineClient {
        DataPipelineClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DataPipelineClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DataPipelineClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DataPipelineClient {
        DataPipelineClient { client, region }
    }
}

#[async_trait]
impl DataPipeline for DataPipelineClient {
    /// <p>Validates the specified pipeline and starts processing pipeline tasks. If the pipeline does not pass validation, activation fails.</p> <p>If you need to pause the pipeline to investigate an issue with a component, such as a data source or script, call <a>DeactivatePipeline</a>.</p> <p>To activate a finished pipeline, modify the end date for the pipeline and then activate it.</p>
    async fn activate_pipeline(
        &self,
        input: ActivatePipelineInput,
    ) -> Result<ActivatePipelineOutput, RusotoError<ActivatePipelineError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ActivatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ActivatePipelineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ActivatePipelineError::from_response(response))
        }
    }

    /// <p>Adds or modifies tags for the specified pipeline.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.AddTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AddTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsError::from_response(response))
        }
    }

    /// <p>Creates a new, empty pipeline. Use <a>PutPipelineDefinition</a> to populate the pipeline.</p>
    async fn create_pipeline(
        &self,
        input: CreatePipelineInput,
    ) -> Result<CreatePipelineOutput, RusotoError<CreatePipelineError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.CreatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreatePipelineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePipelineError::from_response(response))
        }
    }

    /// <p>Deactivates the specified running pipeline. The pipeline is set to the <code>DEACTIVATING</code> state until the deactivation process completes.</p> <p>To resume a deactivated pipeline, use <a>ActivatePipeline</a>. By default, the pipeline resumes from the last completed execution. Optionally, you can specify the date and time to resume the pipeline.</p>
    async fn deactivate_pipeline(
        &self,
        input: DeactivatePipelineInput,
    ) -> Result<DeactivatePipelineOutput, RusotoError<DeactivatePipelineError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DeactivatePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeactivatePipelineOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeactivatePipelineError::from_response(response))
        }
    }

    /// <p>Deletes a pipeline, its pipeline definition, and its run history. AWS Data Pipeline attempts to cancel instances associated with the pipeline that are currently being processed by task runners.</p> <p>Deleting a pipeline cannot be undone. You cannot query or restore a deleted pipeline. To temporarily pause a pipeline instead of deleting it, call <a>SetStatus</a> with the status set to <code>PAUSE</code> on individual components. Components that are paused by <a>SetStatus</a> can be resumed.</p>
    async fn delete_pipeline(
        &self,
        input: DeletePipelineInput,
    ) -> Result<(), RusotoError<DeletePipelineError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DeletePipeline");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePipelineError::from_response(response))
        }
    }

    /// <p>Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object.</p>
    async fn describe_objects(
        &self,
        input: DescribeObjectsInput,
    ) -> Result<DescribeObjectsOutput, RusotoError<DescribeObjectsError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DescribeObjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeObjectsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeObjectsError::from_response(response))
        }
    }

    /// <p>Retrieves metadata about one or more pipelines. The information retrieved includes the name of the pipeline, the pipeline identifier, its current state, and the user account that owns the pipeline. Using account credentials, you can retrieve metadata about pipelines that you or your IAM users have created. If you are using an IAM user account, you can retrieve metadata about only those pipelines for which you have read permissions.</p> <p>To retrieve the full pipeline definition instead of metadata about the pipeline, call <a>GetPipelineDefinition</a>.</p>
    async fn describe_pipelines(
        &self,
        input: DescribePipelinesInput,
    ) -> Result<DescribePipelinesOutput, RusotoError<DescribePipelinesError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.DescribePipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribePipelinesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePipelinesError::from_response(response))
        }
    }

    /// <p>Task runners call <code>EvaluateExpression</code> to evaluate a string in the context of the specified object. For example, a task runner can evaluate SQL queries stored in Amazon S3.</p>
    async fn evaluate_expression(
        &self,
        input: EvaluateExpressionInput,
    ) -> Result<EvaluateExpressionOutput, RusotoError<EvaluateExpressionError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.EvaluateExpression");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<EvaluateExpressionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EvaluateExpressionError::from_response(response))
        }
    }

    /// <p>Gets the definition of the specified pipeline. You can call <code>GetPipelineDefinition</code> to retrieve the pipeline definition that you provided using <a>PutPipelineDefinition</a>.</p>
    async fn get_pipeline_definition(
        &self,
        input: GetPipelineDefinitionInput,
    ) -> Result<GetPipelineDefinitionOutput, RusotoError<GetPipelineDefinitionError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.GetPipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPipelineDefinitionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPipelineDefinitionError::from_response(response))
        }
    }

    /// <p>Lists the pipeline identifiers for all active pipelines that you have permission to access.</p>
    async fn list_pipelines(
        &self,
        input: ListPipelinesInput,
    ) -> Result<ListPipelinesOutput, RusotoError<ListPipelinesError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ListPipelines");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListPipelinesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPipelinesError::from_response(response))
        }
    }

    /// <p>Task runners call <code>PollForTask</code> to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the <code>workerGroup</code> parameter. The task returned can come from any of the pipelines that match the <code>workerGroup</code> value passed in by the task runner and that was launched using the IAM user credentials specified by the task runner.</p> <p>If tasks are ready in the work queue, <code>PollForTask</code> returns a response immediately. If no tasks are available in the queue, <code>PollForTask</code> uses long-polling and holds on to a poll connection for up to a 90 seconds, during which time the first newly scheduled task is handed to the task runner. To accomodate this, set the socket timeout in your task runner to 90 seconds. The task runner should not call <code>PollForTask</code> again on the same <code>workerGroup</code> until it receives a response, and this can take up to 90 seconds. </p>
    async fn poll_for_task(
        &self,
        input: PollForTaskInput,
    ) -> Result<PollForTaskOutput, RusotoError<PollForTaskError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.PollForTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PollForTaskOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PollForTaskError::from_response(response))
        }
    }

    /// <p>Adds tasks, schedules, and preconditions to the specified pipeline. You can use <code>PutPipelineDefinition</code> to populate a new pipeline.</p> <p> <code>PutPipelineDefinition</code> also validates the configuration as it adds it to the pipeline. Changes to the pipeline are saved unless one of the following three validation errors exists in the pipeline. </p> <ol> <li>An object is missing a name or identifier field.</li> <li>A string or reference field is empty.</li> <li>The number of objects in the pipeline exceeds the maximum allowed objects.</li> <li>The pipeline is in a FINISHED state.</li> </ol> <p> Pipeline object definitions are passed to the <code>PutPipelineDefinition</code> action and returned by the <a>GetPipelineDefinition</a> action. </p>
    async fn put_pipeline_definition(
        &self,
        input: PutPipelineDefinitionInput,
    ) -> Result<PutPipelineDefinitionOutput, RusotoError<PutPipelineDefinitionError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.PutPipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<PutPipelineDefinitionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutPipelineDefinitionError::from_response(response))
        }
    }

    /// <p>Queries the specified pipeline for the names of objects that match the specified set of conditions.</p>
    async fn query_objects(
        &self,
        input: QueryObjectsInput,
    ) -> Result<QueryObjectsOutput, RusotoError<QueryObjectsError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.QueryObjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<QueryObjectsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(QueryObjectsError::from_response(response))
        }
    }

    /// <p>Removes existing tags from the specified pipeline.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsInput,
    ) -> Result<RemoveTagsOutput, RusotoError<RemoveTagsError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.RemoveTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<RemoveTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsError::from_response(response))
        }
    }

    /// <p>Task runners call <code>ReportTaskProgress</code> when assigned a task to acknowledge that it has the task. If the web service does not receive this acknowledgement within 2 minutes, it assigns the task in a subsequent <a>PollForTask</a> call. After this initial acknowledgement, the task runner only needs to report progress every 15 minutes to maintain its ownership of the task. You can change this reporting time from 15 minutes by specifying a <code>reportProgressTimeout</code> field in your pipeline.</p> <p>If a task runner does not report its status after 5 minutes, AWS Data Pipeline assumes that the task runner is unable to process the task and reassigns the task in a subsequent response to <a>PollForTask</a>. Task runners should call <code>ReportTaskProgress</code> every 60 seconds.</p>
    async fn report_task_progress(
        &self,
        input: ReportTaskProgressInput,
    ) -> Result<ReportTaskProgressOutput, RusotoError<ReportTaskProgressError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ReportTaskProgress");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ReportTaskProgressOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ReportTaskProgressError::from_response(response))
        }
    }

    /// <p>Task runners call <code>ReportTaskRunnerHeartbeat</code> every 15 minutes to indicate that they are operational. If the AWS Data Pipeline Task Runner is launched on a resource managed by AWS Data Pipeline, the web service can use this call to detect when the task runner application has failed and restart a new instance.</p>
    async fn report_task_runner_heartbeat(
        &self,
        input: ReportTaskRunnerHeartbeatInput,
    ) -> Result<ReportTaskRunnerHeartbeatOutput, RusotoError<ReportTaskRunnerHeartbeatError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ReportTaskRunnerHeartbeat");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ReportTaskRunnerHeartbeatOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ReportTaskRunnerHeartbeatError::from_response(response))
        }
    }

    /// <p>Requests that the status of the specified physical or logical pipeline objects be updated in the specified pipeline. This update might not occur immediately, but is eventually consistent. The status that can be set depends on the type of object (for example, DataNode or Activity). You cannot perform this operation on <code>FINISHED</code> pipelines and attempting to do so returns <code>InvalidRequestException</code>.</p>
    async fn set_status(&self, input: SetStatusInput) -> Result<(), RusotoError<SetStatusError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.SetStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetStatusError::from_response(response))
        }
    }

    /// <p>Task runners call <code>SetTaskStatus</code> to notify AWS Data Pipeline that a task is completed and provide information about the final status. A task runner makes this call regardless of whether the task was sucessful. A task runner does not need to call <code>SetTaskStatus</code> for tasks that are canceled by the web service during a call to <a>ReportTaskProgress</a>.</p>
    async fn set_task_status(
        &self,
        input: SetTaskStatusInput,
    ) -> Result<SetTaskStatusOutput, RusotoError<SetTaskStatusError>> {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.SetTaskStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SetTaskStatusOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SetTaskStatusError::from_response(response))
        }
    }

    /// <p>Validates the specified pipeline definition to ensure that it is well formed and can be run without error.</p>
    async fn validate_pipeline_definition(
        &self,
        input: ValidatePipelineDefinitionInput,
    ) -> Result<ValidatePipelineDefinitionOutput, RusotoError<ValidatePipelineDefinitionError>>
    {
        let mut request = SignedRequest::new("POST", "datapipeline", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DataPipeline.ValidatePipelineDefinition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ValidatePipelineDefinitionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ValidatePipelineDefinitionError::from_response(response))
        }
    }
}
