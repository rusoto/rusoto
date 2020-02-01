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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsInput {
    /// <p>The ID of the ML object to tag. For example, <code>exampleModelId</code>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the ML object to tag. </p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The key-value pairs to use to create tags. If you specify a key without specifying a value, Amazon ML creates a tag with the specified key and a value of null.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>Amazon ML returns the following elements. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsOutput {
    /// <p>The ID of the ML object that was tagged.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of the ML object that was tagged.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p> Represents the output of a <code>GetBatchPrediction</code> operation.</p> <p> The content consists of the detailed metadata, the status, and the data file information of a <code>Batch Prediction</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPrediction {
    /// <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
    #[serde(rename = "BatchPredictionDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_data_source_id: Option<String>,
    /// <p>The ID assigned to the <code>BatchPrediction</code> at creation. This value should be identical to the value of the <code>BatchPredictionID</code> in the request. </p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>BatchPrediction</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account that invoked the <code>BatchPrediction</code>. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    #[serde(rename = "InvalidRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_record_count: Option<i64>,
    /// <p>The time of the most recent edit to the <code>BatchPrediction</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The ID of the <code>MLModel</code> that generated predictions for the <code>BatchPrediction</code> request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>A description of the most recent details about processing the batch prediction request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The location of an Amazon S3 bucket or directory to receive the operation results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p>
    #[serde(rename = "OutputUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The status of the <code>BatchPrediction</code>. This element can have one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to generate predictions for a batch of observations.</li> <li> <code>INPROGRESS</code> - The process is underway.</li> <li> <code>FAILED</code> - The request to perform a batch prediction did not run to completion. It is not usable.</li> <li> <code>COMPLETED</code> - The batch prediction process completed successfully.</li> <li> <code>DELETED</code> - The <code>BatchPrediction</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBatchPredictionInput {
    /// <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
    #[serde(rename = "BatchPredictionDataSourceId")]
    pub batch_prediction_data_source_id: String,
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>. <code>BatchPredictionName</code> can only use the UTF-8 character set.</p>
    #[serde(rename = "BatchPredictionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_name: Option<String>,
    /// <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
    /// <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p> <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="http://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    #[serde(rename = "OutputUri")]
    pub output_uri: String,
}

/// <p> Represents the output of a <code>CreateBatchPrediction</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p>The <code>CreateBatchPrediction</code> operation is asynchronous. You can poll for status updates by using the <code>&gt;GetBatchPrediction</code> operation and checking the <code>Status</code> parameter of the result. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBatchPredictionOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>. This value is identical to the value of the <code>BatchPredictionId</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceFromRDSInput {
    /// <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training. This parameter must be set to <code>true</code> if the <code></code>DataSource<code></code> needs to be used for <code>MLModel</code> training. </p>
    #[serde(rename = "ComputeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_statistics: Option<bool>,
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. Typically, an Amazon Resource Number (ARN) becomes the ID for a <code>DataSource</code>.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>A user-supplied name or description of the <code>DataSource</code>.</p>
    #[serde(rename = "DataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p><p>The data specification of an Amazon RDS <code>DataSource</code>:</p> <ul> <li><p>DatabaseInformation - <ul> <li> <code>DatabaseName</code> - The name of the Amazon RDS database.</li> <li> <code>InstanceIdentifier </code> - A unique identifier for the Amazon RDS database instance.</li> </ul> </p></li> <li><p>DatabaseCredentials - AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon RDS database.</p></li> <li><p>ResourceRole - A role (DataPipelineDefaultResourceRole) assumed by an EC2 instance to carry out the copy task from Amazon RDS to Amazon Simple Storage Service (Amazon S3). For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p></li> <li><p>ServiceRole - A role (DataPipelineDefaultRole) assumed by the AWS Data Pipeline service to monitor the progress of the copy task from Amazon RDS to Amazon S3. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p></li> <li><p>SecurityInfo - The security information to use to access an RDS DB instance. You need to set up appropriate ingress rules for the security entity IDs provided to allow access to the Amazon RDS instance. Specify a [<code>SubnetId</code>, <code>SecurityGroupIds</code>] pair for a VPC-based RDS DB instance.</p></li> <li><p>SelectSqlQuery - A query that is used to retrieve the observation data for the <code>Datasource</code>.</p></li> <li><p>S3StagingLocation - The Amazon S3 location for staging Amazon RDS data. The data retrieved from Amazon RDS using <code>SelectSqlQuery</code> is stored in this location.</p></li> <li><p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p></li> <li><p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p></li> <li> <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p> <br> <p> Sample - <code> &quot;{&quot;splitting&quot;:{&quot;percentBegin&quot;:10,&quot;percentEnd&quot;:60}}&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "RDSData")]
    pub rds_data: RDSDataSpec,
    /// <p>The role that Amazon ML assumes on behalf of the user to create and activate a data pipeline in the user's account and copy data using the <code>SelectSqlQuery</code> query from Amazon RDS to Amazon S3.</p> <p> </p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> Represents the output of a <code>CreateDataSourceFromRDS</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p>The <code>CreateDataSourceFromRDS</code>&gt; operation is asynchronous. You can poll for updates by using the <code>GetBatchPrediction</code> operation and checking the <code>Status</code> parameter. You can inspect the <code>Message</code> when <code>Status</code> shows up as <code>FAILED</code>. You can also check the progress of the copy operation by going to the <code>DataPipeline</code> console and looking up the pipeline using the <code>pipelineId </code> from the describe call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceFromRDSOutput {
    /// <p>A user-supplied ID that uniquely identifies the datasource. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceFromRedshiftInput {
    /// <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training. This parameter must be set to <code>true</code> if the <code>DataSource</code> needs to be used for <code>MLModel</code> training.</p>
    #[serde(rename = "ComputeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_statistics: Option<bool>,
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>A user-supplied name or description of the <code>DataSource</code>. </p>
    #[serde(rename = "DataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p><p>The data specification of an Amazon Redshift <code>DataSource</code>:</p> <ul> <li><p>DatabaseInformation - <ul> <li> <code>DatabaseName</code> - The name of the Amazon Redshift database. </li> <li> <code> ClusterIdentifier</code> - The unique ID for the Amazon Redshift cluster.</li> </ul></p></li> <li><p>DatabaseCredentials - The AWS Identity and Access Management (IAM) credentials that are used to connect to the Amazon Redshift database.</p></li> <li><p>SelectSqlQuery - The query that is used to retrieve the observation data for the <code>Datasource</code>.</p></li> <li><p>S3StagingLocation - The Amazon Simple Storage Service (Amazon S3) location for staging Amazon Redshift data. The data retrieved from Amazon Redshift using the <code>SelectSqlQuery</code> query is stored in this location.</p></li> <li><p>DataSchemaUri - The Amazon S3 location of the <code>DataSchema</code>.</p></li> <li><p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p></li> <li> <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>DataSource</code>.</p> <p> Sample - <code> &quot;{&quot;splitting&quot;:{&quot;percentBegin&quot;:10,&quot;percentEnd&quot;:60}}&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "DataSpec")]
    pub data_spec: RedshiftDataSpec,
    /// <p>A fully specified role Amazon Resource Name (ARN). Amazon ML assumes the role on behalf of the user to create the following: </p> <p> <ul> <li><p>A security group to allow Amazon ML to execute the <code>SelectSqlQuery</code> query on an Amazon Redshift cluster</p></li> <li><p>An Amazon S3 bucket policy to grant Amazon ML read/write permissions on the <code>S3StagingLocation</code></p></li> </ul> </p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> Represents the output of a <code>CreateDataSourceFromRedshift</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p>The <code>CreateDataSourceFromRedshift</code> operation is asynchronous. You can poll for updates by using the <code>GetBatchPrediction</code> operation and checking the <code>Status</code> parameter. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceFromRedshiftOutput {
    /// <p>A user-supplied ID that uniquely identifies the datasource. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceFromS3Input {
    /// <p>The compute statistics for a <code>DataSource</code>. The statistics are generated from the observation data referenced by a <code>DataSource</code>. Amazon ML uses the statistics internally during <code>MLModel</code> training. This parameter must be set to <code>true</code> if the <code></code>DataSource<code></code> needs to be used for <code>MLModel</code> training.</p>
    #[serde(rename = "ComputeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_statistics: Option<bool>,
    /// <p>A user-supplied identifier that uniquely identifies the <code>DataSource</code>. </p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>A user-supplied name or description of the <code>DataSource</code>. </p>
    #[serde(rename = "DataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p><p>The data specification of a <code>DataSource</code>:</p> <ul> <li><p>DataLocationS3 - The Amazon S3 location of the observation data.</p></li> <li><p>DataSchemaLocationS3 - The Amazon S3 location of the <code>DataSchema</code>.</p></li> <li><p>DataSchema - A JSON string representing the schema. This is not required if <code>DataSchemaUri</code> is specified. </p></li> <li> <p>DataRearrangement - A JSON string that represents the splitting and rearrangement requirements for the <code>Datasource</code>. </p> <p> Sample - <code> &quot;{&quot;splitting&quot;:{&quot;percentBegin&quot;:10,&quot;percentEnd&quot;:60}}&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "DataSpec")]
    pub data_spec: S3DataSpec,
}

/// <p> Represents the output of a <code>CreateDataSourceFromS3</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p>The <code>CreateDataSourceFromS3</code> operation is asynchronous. You can poll for updates by using the <code>GetBatchPrediction</code> operation and checking the <code>Status</code> parameter. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceFromS3Output {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEvaluationInput {
    /// <p>The ID of the <code>DataSource</code> for the evaluation. The schema of the <code>DataSource</code> must match the schema used to create the <code>MLModel</code>.</p>
    #[serde(rename = "EvaluationDataSourceId")]
    pub evaluation_data_source_id: String,
    /// <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>.</p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
    /// <p>A user-supplied name or description of the <code>Evaluation</code>.</p>
    #[serde(rename = "EvaluationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_name: Option<String>,
    /// <p>The ID of the <code>MLModel</code> to evaluate.</p> <p>The schema used in creating the <code>MLModel</code> must match the schema of the <code>DataSource</code> used in the <code>Evaluation</code>.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p> Represents the output of a <code>CreateEvaluation</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p><code>CreateEvaluation</code> operation is asynchronous. You can poll for status updates by using the <code>GetEvcaluation</code> operation and checking the <code>Status</code> parameter. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEvaluationOutput {
    /// <p>The user-supplied ID that uniquely identifies the <code>Evaluation</code>. This value should be identical to the value of the <code>EvaluationId</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMLModelInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
    /// <p>A user-supplied name or description of the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_name: Option<String>,
    /// <p>The category of supervised learning that this <code>MLModel</code> will address. Choose from the following types:</p> <ul> <li>Choose <code>REGRESSION</code> if the <code>MLModel</code> will be used to predict a numeric value.</li> <li>Choose <code>BINARY</code> if the <code>MLModel</code> result has two possible values.</li> <li>Choose <code>MULTICLASS</code> if the <code>MLModel</code> result has a limited number of values. </li> </ul> <p> For more information, see the <a href="http://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    #[serde(rename = "MLModelType")]
    pub ml_model_type: String,
    /// <p><p>A list of the training parameters in the <code>MLModel</code>. The list is implemented as a map of key-value pairs.</p> <p>The following is the current set of training parameters: </p> <ul> <li> <p><code>sgd.maxMLModelSizeInBytes</code> - The maximum allowed size of the model. Depending on the input data, the size of the model might affect its performance.</p> <p> The value is an integer that ranges from <code>100000</code> to <code>2147483648</code>. The default value is <code>33554432</code>.</p> </li> <li><p><code>sgd.maxPasses</code> - The number of times that the training process traverses the observations to build the <code>MLModel</code>. The value is an integer that ranges from <code>1</code> to <code>10000</code>. The default value is <code>10</code>.</p></li> <li> <p><code>sgd.shuffleType</code> - Whether Amazon ML shuffles the training data. Shuffling the data improves a model&#39;s ability to find the optimal solution for a variety of data types. The valid values are <code>auto</code> and <code>none</code>. The default value is <code>none</code>. We &lt;?oxy<em>insert</em>start author=&quot;laurama&quot; timestamp=&quot;20160329T131121-0700&quot;&gt;strongly recommend that you shuffle your data.&lt;?oxy<em>insert</em>end&gt;</p> </li> <li> <p><code>sgd.l1RegularizationAmount</code> - The coefficient regularization L1 norm. It controls overfitting the data by penalizing large coefficients. This tends to drive coefficients to zero, resulting in a sparse feature set. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX<em>DOUBLE</code>. The default is to not use L1 normalization. This parameter can&#39;t be used when <code>L2</code> is specified. Use this parameter sparingly.</p> </li> <li> <p><code>sgd.l2RegularizationAmount</code> - The coefficient regularization L2 norm. It controls overfitting the data by penalizing large coefficients. This tends to drive coefficients to small, nonzero values. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX</em>DOUBLE</code>. The default is to not use L2 normalization. This parameter can&#39;t be used when <code>L1</code> is specified. Use this parameter sparingly.</p> </li> </ul></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The data recipe for creating the <code>MLModel</code>. You must specify either the recipe or its URI. If you don't specify a recipe or its URI, Amazon ML creates a default.</p>
    #[serde(rename = "Recipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    /// <p>The Amazon Simple Storage Service (Amazon S3) location and file name that contains the <code>MLModel</code> recipe. You must specify either the recipe or its URI. If you don't specify a recipe or its URI, Amazon ML creates a default.</p>
    #[serde(rename = "RecipeUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_uri: Option<String>,
    /// <p>The <code>DataSource</code> that points to the training data.</p>
    #[serde(rename = "TrainingDataSourceId")]
    pub training_data_source_id: String,
}

/// <p> Represents the output of a <code>CreateMLModel</code> operation, and is an acknowledgement that Amazon ML received the request.</p> <p>The <code>CreateMLModel</code> operation is asynchronous. You can poll for status updates by using the <code>GetMLModel</code> operation and checking the <code>Status</code> parameter. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMLModelOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelId</code> in the request. </p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRealtimeEndpointInput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p><p>Represents the output of an <code>CreateRealtimeEndpoint</code> operation.</p> <p>The result contains the <code>MLModelId</code> and the endpoint information for the <code>MLModel</code>.</p> <note> <p>The endpoint information includes the URI of the <code>MLModel</code>; that is, the location to send online prediction requests for the specified <code>MLModel</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRealtimeEndpointOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelId</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>The endpoint information of the <code>MLModel</code> </p>
    #[serde(rename = "RealtimeEndpointInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_endpoint_info: Option<RealtimeEndpointInfo>,
}

/// <p> Represents the output of the <code>GetDataSource</code> operation. </p> <p> The content consists of the detailed metadata and data file information and the current status of the <code>DataSource</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSource {
    /// <p> The parameter is <code>true</code> if statistics need to be generated from the observation data. </p>
    #[serde(rename = "ComputeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_statistics: Option<bool>,
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>DataSource</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account from which the <code>DataSource</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The location and name of the data in Amazon Simple Storage Service (Amazon S3) that is used by a <code>DataSource</code>.</p>
    #[serde(rename = "DataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_s3: Option<String>,
    /// <p>A JSON string that represents the splitting and rearrangement requirement used when this <code>DataSource</code> was created.</p>
    #[serde(rename = "DataRearrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_rearrangement: Option<String>,
    /// <p>The total number of observations contained in the data files that the <code>DataSource</code> references.</p>
    #[serde(rename = "DataSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size_in_bytes: Option<i64>,
    /// <p>The ID that is assigned to the <code>DataSource</code> during creation.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The time of the most recent edit to the <code>BatchPrediction</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A description of the most recent details about creating the <code>DataSource</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>DataSource</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of data files referenced by the <code>DataSource</code>.</p>
    #[serde(rename = "NumberOfFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_files: Option<i64>,
    #[serde(rename = "RDSMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_metadata: Option<RDSMetadata>,
    #[serde(rename = "RedshiftMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_metadata: Option<RedshiftMetadata>,
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The current status of the <code>DataSource</code>. This element can have one of the following values: </p> <ul> <li>PENDING - Amazon Machine Learning (Amazon ML) submitted a request to create a <code>DataSource</code>.</li> <li>INPROGRESS - The creation process is underway.</li> <li>FAILED - The request to create a <code>DataSource</code> did not run to completion. It is not usable.</li> <li>COMPLETED - The creation process completed successfully.</li> <li>DELETED - The <code>DataSource</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBatchPredictionInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
}

/// <p> Represents the output of a <code>DeleteBatchPrediction</code> operation.</p> <p>You can use the <code>GetBatchPrediction</code> operation and check the value of the <code>Status</code> parameter to see whether a <code>BatchPrediction</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBatchPredictionOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>. This value should be identical to the value of the <code>BatchPredictionID</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataSourceInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
}

/// <p> Represents the output of a <code>DeleteDataSource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDataSourceOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. This value should be identical to the value of the <code>DataSourceID</code> in the request.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEvaluationInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code> to delete.</p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
}

/// <p> Represents the output of a <code>DeleteEvaluation</code> operation. The output indicates that Amazon Machine Learning (Amazon ML) received the request.</p> <p>You can use the <code>GetEvaluation</code> operation and check the value of the <code>Status</code> parameter to see whether an <code>Evaluation</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEvaluationOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>. This value should be identical to the value of the <code>EvaluationId</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMLModelInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p>Represents the output of a <code>DeleteMLModel</code> operation.</p> <p>You can use the <code>GetMLModel</code> operation and check the value of the <code>Status</code> parameter to see whether an <code>MLModel</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMLModelOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelID</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRealtimeEndpointInput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p>Represents the output of an <code>DeleteRealtimeEndpoint</code> operation.</p> <p>The result contains the <code>MLModelId</code> and the endpoint information for the <code>MLModel</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRealtimeEndpointOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelId</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>The endpoint information of the <code>MLModel</code> </p>
    #[serde(rename = "RealtimeEndpointInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_endpoint_info: Option<RealtimeEndpointInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsInput {
    /// <p>The ID of the tagged ML object. For example, <code>exampleModelId</code>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the tagged ML object.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>One or more tags to delete.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Amazon ML returns the following elements. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsOutput {
    /// <p>The ID of the ML object from which tags were deleted.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of the ML object from which tags were deleted.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBatchPredictionsInput {
    /// <p>The equal to operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values that exactly match the value specified with <code>EQ</code>.</p>
    #[serde(rename = "EQ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    /// <p><p>Use one of the following variables to filter a list of <code>BatchPrediction</code>:</p> <ul> <li> <code>CreatedAt</code> - Sets the search criteria to the <code>BatchPrediction</code> creation date.</li> <li> <code>Status</code> - Sets the search criteria to the <code>BatchPrediction</code> status.</li> <li> <code>Name</code> - Sets the search criteria to the contents of the <code>BatchPrediction</code><b> </b> <code>Name</code>.</li> <li> <code>IAMUser</code> - Sets the search criteria to the user account that invoked the <code>BatchPrediction</code> creation.</li> <li> <code>MLModelId</code> - Sets the search criteria to the <code>MLModel</code> used in the <code>BatchPrediction</code>.</li> <li> <code>DataSourceId</code> - Sets the search criteria to the <code>DataSource</code> used in the <code>BatchPrediction</code>.</li> <li> <code>DataURI</code> - Sets the search criteria to the data file(s) used in the <code>BatchPrediction</code>. The URL can identify either a file or an Amazon Simple Storage Solution (Amazon S3) bucket or directory.</li> </ul></p>
    #[serde(rename = "FilterVariable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_variable: Option<String>,
    /// <p>The greater than or equal to operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values that are greater than or equal to the value specified with <code>GE</code>. </p>
    #[serde(rename = "GE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<String>,
    /// <p>The greater than operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values that are greater than the value specified with <code>GT</code>.</p>
    #[serde(rename = "GT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    /// <p>The less than or equal to operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values that are less than or equal to the value specified with <code>LE</code>.</p>
    #[serde(rename = "LE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub le: Option<String>,
    /// <p>The less than operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values that are less than the value specified with <code>LT</code>.</p>
    #[serde(rename = "LT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// <p>The number of pages of information to include in the result. The range of acceptable values is <code>1</code> through <code>100</code>. The default value is <code>100</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The not equal to operator. The <code>BatchPrediction</code> results will have <code>FilterVariable</code> values not equal to the value specified with <code>NE</code>.</p>
    #[serde(rename = "NE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne: Option<String>,
    /// <p>An ID of the page in the paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A string that is found at the beginning of a variable, such as <code>Name</code> or <code>Id</code>.</p> <p>For example, a <code>Batch Prediction</code> operation could have the <code>Name</code> <code>2014-09-09-HolidayGiftMailer</code>. To search for this <code>BatchPrediction</code>, select <code>Name</code> for the <code>FilterVariable</code> and any of the following strings for the <code>Prefix</code>: </p> <ul> <li><p>2014-09</p></li> <li><p>2014-09-09</p></li> <li><p>2014-09-09-Holiday</p></li> </ul></p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A two-value parameter that determines the sequence of the resulting list of <code>MLModel</code>s.</p> <ul> <li> <code>asc</code> - Arranges the list in ascending order (A-Z, 0-9).</li> <li> <code>dsc</code> - Arranges the list in descending order (Z-A, 9-0).</li> </ul> <p>Results are sorted by <code>FilterVariable</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the output of a <code>DescribeBatchPredictions</code> operation. The content is essentially a list of <code>BatchPrediction</code>s.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBatchPredictionsOutput {
    /// <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>BatchPrediction</code> objects that meet the search criteria. </p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchPrediction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSourcesInput {
    /// <p>The equal to operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values that exactly match the value specified with <code>EQ</code>.</p>
    #[serde(rename = "EQ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    /// <p><p>Use one of the following variables to filter a list of <code>DataSource</code>:</p> <ul> <li> <code>CreatedAt</code> - Sets the search criteria to <code>DataSource</code> creation dates.</li> <li> <code>Status</code> - Sets the search criteria to <code>DataSource</code> statuses.</li> <li> <code>Name</code> - Sets the search criteria to the contents of <code>DataSource</code> <b> </b> <code>Name</code>.</li> <li> <code>DataUri</code> - Sets the search criteria to the URI of data files used to create the <code>DataSource</code>. The URI can identify either a file or an Amazon Simple Storage Service (Amazon S3) bucket or directory.</li> <li> <code>IAMUser</code> - Sets the search criteria to the user account that invoked the <code>DataSource</code> creation.</li> </ul></p>
    #[serde(rename = "FilterVariable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_variable: Option<String>,
    /// <p>The greater than or equal to operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values that are greater than or equal to the value specified with <code>GE</code>. </p>
    #[serde(rename = "GE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<String>,
    /// <p>The greater than operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values that are greater than the value specified with <code>GT</code>.</p>
    #[serde(rename = "GT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    /// <p>The less than or equal to operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values that are less than or equal to the value specified with <code>LE</code>.</p>
    #[serde(rename = "LE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub le: Option<String>,
    /// <p>The less than operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values that are less than the value specified with <code>LT</code>.</p>
    #[serde(rename = "LT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// <p> The maximum number of <code>DataSource</code> to include in the result.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The not equal to operator. The <code>DataSource</code> results will have <code>FilterVariable</code> values not equal to the value specified with <code>NE</code>.</p>
    #[serde(rename = "NE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne: Option<String>,
    /// <p>The ID of the page in the paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A string that is found at the beginning of a variable, such as <code>Name</code> or <code>Id</code>.</p> <p>For example, a <code>DataSource</code> could have the <code>Name</code> <code>2014-09-09-HolidayGiftMailer</code>. To search for this <code>DataSource</code>, select <code>Name</code> for the <code>FilterVariable</code> and any of the following strings for the <code>Prefix</code>: </p> <ul> <li><p>2014-09</p></li> <li><p>2014-09-09</p></li> <li><p>2014-09-09-Holiday</p></li> </ul></p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A two-value parameter that determines the sequence of the resulting list of <code>DataSource</code>.</p> <ul> <li> <code>asc</code> - Arranges the list in ascending order (A-Z, 0-9).</li> <li> <code>dsc</code> - Arranges the list in descending order (Z-A, 9-0).</li> </ul> <p>Results are sorted by <code>FilterVariable</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the query results from a <a>DescribeDataSources</a> operation. The content is essentially a list of <code>DataSource</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSourcesOutput {
    /// <p>An ID of the next page in the paginated results that indicates at least one more page follows.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>DataSource</code> that meet the search criteria. </p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DataSource>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEvaluationsInput {
    /// <p>The equal to operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values that exactly match the value specified with <code>EQ</code>.</p>
    #[serde(rename = "EQ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    /// <p><p>Use one of the following variable to filter a list of <code>Evaluation</code> objects:</p> <ul> <li> <code>CreatedAt</code> - Sets the search criteria to the <code>Evaluation</code> creation date.</li> <li> <code>Status</code> - Sets the search criteria to the <code>Evaluation</code> status.</li> <li> <code>Name</code> - Sets the search criteria to the contents of <code>Evaluation</code> <b> </b> <code>Name</code>.</li> <li> <code>IAMUser</code> - Sets the search criteria to the user account that invoked an <code>Evaluation</code>.</li> <li> <code>MLModelId</code> - Sets the search criteria to the <code>MLModel</code> that was evaluated.</li> <li> <code>DataSourceId</code> - Sets the search criteria to the <code>DataSource</code> used in <code>Evaluation</code>.</li> <li> <code>DataUri</code> - Sets the search criteria to the data file(s) used in <code>Evaluation</code>. The URL can identify either a file or an Amazon Simple Storage Solution (Amazon S3) bucket or directory.</li> </ul></p>
    #[serde(rename = "FilterVariable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_variable: Option<String>,
    /// <p>The greater than or equal to operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values that are greater than or equal to the value specified with <code>GE</code>. </p>
    #[serde(rename = "GE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<String>,
    /// <p>The greater than operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values that are greater than the value specified with <code>GT</code>.</p>
    #[serde(rename = "GT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    /// <p>The less than or equal to operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values that are less than or equal to the value specified with <code>LE</code>.</p>
    #[serde(rename = "LE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub le: Option<String>,
    /// <p>The less than operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values that are less than the value specified with <code>LT</code>.</p>
    #[serde(rename = "LT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// <p> The maximum number of <code>Evaluation</code> to include in the result.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The not equal to operator. The <code>Evaluation</code> results will have <code>FilterVariable</code> values not equal to the value specified with <code>NE</code>.</p>
    #[serde(rename = "NE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne: Option<String>,
    /// <p>The ID of the page in the paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A string that is found at the beginning of a variable, such as <code>Name</code> or <code>Id</code>.</p> <p>For example, an <code>Evaluation</code> could have the <code>Name</code> <code>2014-09-09-HolidayGiftMailer</code>. To search for this <code>Evaluation</code>, select <code>Name</code> for the <code>FilterVariable</code> and any of the following strings for the <code>Prefix</code>: </p> <ul> <li><p>2014-09</p></li> <li><p>2014-09-09</p></li> <li><p>2014-09-09-Holiday</p></li> </ul></p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A two-value parameter that determines the sequence of the resulting list of <code>Evaluation</code>.</p> <ul> <li> <code>asc</code> - Arranges the list in ascending order (A-Z, 0-9).</li> <li> <code>dsc</code> - Arranges the list in descending order (Z-A, 9-0).</li> </ul> <p>Results are sorted by <code>FilterVariable</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the query results from a <code>DescribeEvaluations</code> operation. The content is essentially a list of <code>Evaluation</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEvaluationsOutput {
    /// <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>Evaluation</code> that meet the search criteria. </p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Evaluation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMLModelsInput {
    /// <p>The equal to operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values that exactly match the value specified with <code>EQ</code>.</p>
    #[serde(rename = "EQ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    /// <p><p>Use one of the following variables to filter a list of <code>MLModel</code>:</p> <ul> <li> <code>CreatedAt</code> - Sets the search criteria to <code>MLModel</code> creation date.</li> <li> <code>Status</code> - Sets the search criteria to <code>MLModel</code> status.</li> <li> <code>Name</code> - Sets the search criteria to the contents of <code>MLModel</code><b> </b> <code>Name</code>.</li> <li> <code>IAMUser</code> - Sets the search criteria to the user account that invoked the <code>MLModel</code> creation.</li> <li> <code>TrainingDataSourceId</code> - Sets the search criteria to the <code>DataSource</code> used to train one or more <code>MLModel</code>.</li> <li> <code>RealtimeEndpointStatus</code> - Sets the search criteria to the <code>MLModel</code> real-time endpoint status.</li> <li> <code>MLModelType</code> - Sets the search criteria to <code>MLModel</code> type: binary, regression, or multi-class.</li> <li> <code>Algorithm</code> - Sets the search criteria to the algorithm that the <code>MLModel</code> uses.</li> <li> <code>TrainingDataURI</code> - Sets the search criteria to the data file(s) used in training a <code>MLModel</code>. The URL can identify either a file or an Amazon Simple Storage Service (Amazon S3) bucket or directory.</li> </ul></p>
    #[serde(rename = "FilterVariable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_variable: Option<String>,
    /// <p>The greater than or equal to operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values that are greater than or equal to the value specified with <code>GE</code>. </p>
    #[serde(rename = "GE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<String>,
    /// <p>The greater than operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values that are greater than the value specified with <code>GT</code>.</p>
    #[serde(rename = "GT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    /// <p>The less than or equal to operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values that are less than or equal to the value specified with <code>LE</code>.</p>
    #[serde(rename = "LE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub le: Option<String>,
    /// <p>The less than operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values that are less than the value specified with <code>LT</code>.</p>
    #[serde(rename = "LT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// <p>The number of pages of information to include in the result. The range of acceptable values is <code>1</code> through <code>100</code>. The default value is <code>100</code>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The not equal to operator. The <code>MLModel</code> results will have <code>FilterVariable</code> values not equal to the value specified with <code>NE</code>.</p>
    #[serde(rename = "NE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ne: Option<String>,
    /// <p>The ID of the page in the paginated results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A string that is found at the beginning of a variable, such as <code>Name</code> or <code>Id</code>.</p> <p>For example, an <code>MLModel</code> could have the <code>Name</code> <code>2014-09-09-HolidayGiftMailer</code>. To search for this <code>MLModel</code>, select <code>Name</code> for the <code>FilterVariable</code> and any of the following strings for the <code>Prefix</code>: </p> <ul> <li><p>2014-09</p></li> <li><p>2014-09-09</p></li> <li><p>2014-09-09-Holiday</p></li> </ul></p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>A two-value parameter that determines the sequence of the resulting list of <code>MLModel</code>.</p> <ul> <li> <code>asc</code> - Arranges the list in ascending order (A-Z, 0-9).</li> <li> <code>dsc</code> - Arranges the list in descending order (Z-A, 9-0).</li> </ul> <p>Results are sorted by <code>FilterVariable</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// <p>Represents the output of a <code>DescribeMLModels</code> operation. The content is essentially a list of <code>MLModel</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMLModelsOutput {
    /// <p>The ID of the next page in the paginated results that indicates at least one more page follows.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of <code>MLModel</code> that meet the search criteria.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MLModel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsInput {
    /// <p>The ID of the ML object. For example, <code>exampleModelId</code>. </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the ML object.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p>Amazon ML returns the following elements. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsOutput {
    /// <p>The ID of the tagged ML object.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of the tagged ML object.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>A list of tags associated with the ML object.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p> Represents the output of <code>GetEvaluation</code> operation. </p> <p>The content consists of the detailed metadata and data file information and the current status of the <code>Evaluation</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Evaluation {
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>Evaluation</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account that invoked the evaluation. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The ID of the <code>DataSource</code> that is used to evaluate the <code>MLModel</code>.</p>
    #[serde(rename = "EvaluationDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_data_source_id: Option<String>,
    /// <p>The ID that is assigned to the <code>Evaluation</code> at creation.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location and name of the data in Amazon Simple Storage Server (Amazon S3) that is used in the evaluation.</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    /// <p>The time of the most recent edit to the <code>Evaluation</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The ID of the <code>MLModel</code> that is the focus of the evaluation.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>A description of the most recent details about evaluating the <code>MLModel</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>Evaluation</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Measurements of how well the <code>MLModel</code> performed, using observations referenced by the <code>DataSource</code>. One of the following metrics is returned, based on the type of the <code>MLModel</code>: </p> <ul> <li> <p>BinaryAUC: A binary <code>MLModel</code> uses the Area Under the Curve (AUC) technique to measure performance. </p> </li> <li> <p>RegressionRMSE: A regression <code>MLModel</code> uses the Root Mean Square Error (RMSE) technique to measure performance. RMSE measures the difference between predicted and actual values for a single variable.</p> </li> <li> <p>MulticlassAvgFScore: A multiclass <code>MLModel</code> uses the F1 score technique to measure performance. </p> </li> </ul> <p> For more information about performance metrics, please see the <a href="http://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>. </p>
    #[serde(rename = "PerformanceMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metrics: Option<PerformanceMetrics>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The status of the evaluation. This element can have one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to evaluate an <code>MLModel</code>.</li> <li> <code>INPROGRESS</code> - The evaluation is underway.</li> <li> <code>FAILED</code> - The request to evaluate an <code>MLModel</code> did not run to completion. It is not usable.</li> <li> <code>COMPLETED</code> - The evaluation process completed successfully.</li> <li> <code>DELETED</code> - The <code>Evaluation</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBatchPredictionInput {
    /// <p>An ID assigned to the <code>BatchPrediction</code> at creation.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
}

/// <p>Represents the output of a <code>GetBatchPrediction</code> operation and describes a <code>BatchPrediction</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBatchPredictionOutput {
    /// <p>The ID of the <code>DataSource</code> that was used to create the <code>BatchPrediction</code>. </p>
    #[serde(rename = "BatchPredictionDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_data_source_id: Option<String>,
    /// <p>An ID assigned to the <code>BatchPrediction</code> at creation. This value should be identical to the value of the <code>BatchPredictionID</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
    /// <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>BatchPrediction</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>BatchPrediction</code> is in the <code>COMPLETED</code> state.</p>
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time when the <code>BatchPrediction</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account that invoked the <code>BatchPrediction</code>. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>BatchPrediction</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>BatchPrediction</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p>
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    /// <p>The number of invalid records that Amazon Machine Learning saw while processing the <code>BatchPrediction</code>.</p>
    #[serde(rename = "InvalidRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_record_count: Option<i64>,
    /// <p>The time of the most recent edit to <code>BatchPrediction</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A link to the file that contains logs of the <code>CreateBatchPrediction</code> operation.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The ID of the <code>MLModel</code> that generated predictions for the <code>BatchPrediction</code> request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>A description of the most recent details about processing the batch prediction request.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The location of an Amazon S3 bucket or directory to receive the operation results.</p>
    #[serde(rename = "OutputUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<String>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>BatchPrediction</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>BatchPrediction</code> is in the <code>PENDING</code> state.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The status of the <code>BatchPrediction</code>, which can be one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to generate batch predictions.</li> <li> <code>INPROGRESS</code> - The batch predictions are in progress.</li> <li> <code>FAILED</code> - The request to perform a batch prediction did not run to completion. It is not usable.</li> <li> <code>COMPLETED</code> - The batch prediction process completed successfully.</li> <li> <code>DELETED</code> - The <code>BatchPrediction</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The number of total records that Amazon Machine Learning saw while processing the <code>BatchPrediction</code>.</p>
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataSourceInput {
    /// <p>The ID assigned to the <code>DataSource</code> at creation.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>Specifies whether the <code>GetDataSource</code> operation should return <code>DataSourceSchema</code>.</p> <p>If true, <code>DataSourceSchema</code> is returned.</p> <p>If false, <code>DataSourceSchema</code> is not returned.</p>
    #[serde(rename = "Verbose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

/// <p>Represents the output of a <code>GetDataSource</code> operation and describes a <code>DataSource</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataSourceOutput {
    /// <p> The parameter is <code>true</code> if statistics need to be generated from the observation data. </p>
    #[serde(rename = "ComputeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_statistics: Option<bool>,
    /// <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>DataSource</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>DataSource</code> is in the <code>COMPLETED</code> state and the <code>ComputeStatistics</code> is set to true.</p>
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>DataSource</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account from which the <code>DataSource</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "DataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_s3: Option<String>,
    /// <p>A JSON string that represents the splitting and rearrangement requirement used when this <code>DataSource</code> was created.</p>
    #[serde(rename = "DataRearrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_rearrangement: Option<String>,
    /// <p>The total size of observations in the data files.</p>
    #[serde(rename = "DataSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size_in_bytes: Option<i64>,
    /// <p>The ID assigned to the <code>DataSource</code> at creation. This value should be identical to the value of the <code>DataSourceId</code> in the request.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    /// <p><p>The schema used by all of the data files of this <code>DataSource</code>.</p> <note><title>Note</title> <p>This parameter is provided as part of the verbose format.</p></note></p>
    #[serde(rename = "DataSourceSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_schema: Option<String>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>DataSource</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p>
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The time of the most recent edit to the <code>DataSource</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A link to the file containing logs of <code>CreateDataSourceFrom*</code> operations.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The user-supplied description of the most recent details about creating the <code>DataSource</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>DataSource</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of data files referenced by the <code>DataSource</code>.</p>
    #[serde(rename = "NumberOfFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_files: Option<i64>,
    #[serde(rename = "RDSMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_metadata: Option<RDSMetadata>,
    #[serde(rename = "RedshiftMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_metadata: Option<RedshiftMetadata>,
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>DataSource</code> is in the <code>PENDING</code> state.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The current status of the <code>DataSource</code>. This element can have one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon ML submitted a request to create a <code>DataSource</code>.</li> <li> <code>INPROGRESS</code> - The creation process is underway.</li> <li> <code>FAILED</code> - The request to create a <code>DataSource</code> did not run to completion. It is not usable.</li> <li> <code>COMPLETED</code> - The creation process completed successfully.</li> <li> <code>DELETED</code> - The <code>DataSource</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEvaluationInput {
    /// <p>The ID of the <code>Evaluation</code> to retrieve. The evaluation of each <code>MLModel</code> is recorded and cataloged. The ID provides the means to access the information. </p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
}

/// <p>Represents the output of a <code>GetEvaluation</code> operation and describes an <code>Evaluation</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEvaluationOutput {
    /// <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>Evaluation</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>Evaluation</code> is in the <code>COMPLETED</code> state.</p>
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>Evaluation</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account that invoked the evaluation. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The <code>DataSource</code> used for this evaluation.</p>
    #[serde(rename = "EvaluationDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_data_source_id: Option<String>,
    /// <p>The evaluation ID which is same as the <code>EvaluationId</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>Evaluation</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>Evaluation</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p>
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    /// <p>The time of the most recent edit to the <code>Evaluation</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A link to the file that contains logs of the <code>CreateEvaluation</code> operation.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The ID of the <code>MLModel</code> that was the focus of the evaluation.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p>A description of the most recent details about evaluating the <code>MLModel</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>Evaluation</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Measurements of how well the <code>MLModel</code> performed using observations referenced by the <code>DataSource</code>. One of the following metric is returned based on the type of the <code>MLModel</code>: </p> <ul> <li> <p>BinaryAUC: A binary <code>MLModel</code> uses the Area Under the Curve (AUC) technique to measure performance. </p> </li> <li> <p>RegressionRMSE: A regression <code>MLModel</code> uses the Root Mean Square Error (RMSE) technique to measure performance. RMSE measures the difference between predicted and actual values for a single variable.</p> </li> <li> <p>MulticlassAvgFScore: A multiclass <code>MLModel</code> uses the F1 score technique to measure performance. </p> </li> </ul> <p> For more information about performance metrics, please see the <a href="http://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>. </p>
    #[serde(rename = "PerformanceMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metrics: Option<PerformanceMetrics>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>Evaluation</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>Evaluation</code> is in the <code>PENDING</code> state.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The status of the evaluation. This element can have one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon Machine Language (Amazon ML) submitted a request to evaluate an <code>MLModel</code>.</li> <li> <code>INPROGRESS</code> - The evaluation is underway.</li> <li> <code>FAILED</code> - The request to evaluate an <code>MLModel</code> did not run to completion. It is not usable.</li> <li> <code>COMPLETED</code> - The evaluation process completed successfully.</li> <li> <code>DELETED</code> - The <code>Evaluation</code> is marked as deleted. It is not usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMLModelInput {
    /// <p>The ID assigned to the <code>MLModel</code> at creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
    /// <p>Specifies whether the <code>GetMLModel</code> operation should return <code>Recipe</code>.</p> <p>If true, <code>Recipe</code> is returned.</p> <p>If false, <code>Recipe</code> is not returned.</p>
    #[serde(rename = "Verbose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

/// <p>Represents the output of a <code>GetMLModel</code> operation, and provides detailed information about a <code>MLModel</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMLModelOutput {
    /// <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>MLModel</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>MLModel</code> is in the <code>COMPLETED</code> state.</p>
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>MLModel</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account from which the <code>MLModel</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The current endpoint of the <code>MLModel</code></p>
    #[serde(rename = "EndpointInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_info: Option<RealtimeEndpointInfo>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>MLModel</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>MLModel</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p>
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    /// <p>The time of the most recent edit to the <code>MLModel</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>A link to the file that contains logs of the <code>CreateMLModel</code> operation.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>The MLModel ID<?oxy_insert_start author="annbech" timestamp="20160328T151251-0700">,<?oxy_insert_end> which is same as the <code>MLModelId</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p><p>Identifies the <code>MLModel</code> category. The following are the available types: </p> <ul> <li>REGRESSION -- Produces a numeric result. For example, &quot;What price should a house be listed at?&quot;</li> <li>BINARY -- Produces one of two possible results. For example, &quot;Is this an e-commerce website?&quot;</li> <li>MULTICLASS -- Produces one of several possible results. For example, &quot;Is this a HIGH, LOW or MEDIUM risk trade?&quot;</li> </ul></p>
    #[serde(rename = "MLModelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_type: Option<String>,
    /// <p>A description of the most recent details about accessing the <code>MLModel</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>MLModel</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The recipe to use when training the <code>MLModel</code>. The <code>Recipe</code> provides detailed information about the observation data to use during training, and manipulations to perform on the observation data during training.</p> <note><title>Note</title> <p>This parameter is provided as part of the verbose format.</p></note></p>
    #[serde(rename = "Recipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    /// <p><p>The schema used by all of the data files referenced by the <code>DataSource</code>.</p> <note><title>Note</title> <p>This parameter is provided as part of the verbose format.</p></note></p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>The scoring threshold is used in binary classification <code>MLModel</code><?oxy_insert_start author="laurama" timestamp="20160329T114851-0700"> <?oxy_insert_end>models. It marks the boundary between a positive prediction and a negative prediction.</p> <p>Output values greater than or equal to the threshold receive a positive result from the MLModel, such as <code>true</code>. Output values less than the threshold receive a negative response from the MLModel, such as <code>false</code>.</p>
    #[serde(rename = "ScoreThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
    /// <p>The time of the most recent edit to the <code>ScoreThreshold</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "ScoreThresholdLastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold_last_updated_at: Option<f64>,
    #[serde(rename = "SizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    /// <p>The epoch time when Amazon Machine Learning marked the <code>MLModel</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>MLModel</code> is in the <code>PENDING</code> state.</p>
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The current status of the <code>MLModel</code>. This element can have one of the following values:</p> <ul> <li> <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to describe a <code>MLModel</code>.</li> <li> <code>INPROGRESS</code> - The request is processing.</li> <li> <code>FAILED</code> - The request did not run to completion. The ML model isn&#39;t usable.</li> <li> <code>COMPLETED</code> - The request completed successfully.</li> <li> <code>DELETED</code> - The <code>MLModel</code> is marked as deleted. It isn&#39;t usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of the training <code>DataSource</code>.</p>
    #[serde(rename = "TrainingDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source_id: Option<String>,
    /// <p><p>A list of the training parameters in the <code>MLModel</code>. The list is implemented as a map of key-value pairs.</p> <p>The following is the current set of training parameters: </p> <ul> <li> <p><code>sgd.maxMLModelSizeInBytes</code> - The maximum allowed size of the model. Depending on the input data, the size of the model might affect its performance.</p> <p> The value is an integer that ranges from <code>100000</code> to <code>2147483648</code>. The default value is <code>33554432</code>.</p> </li> <li><p><code>sgd.maxPasses</code> - The number of times that the training process traverses the observations to build the <code>MLModel</code>. The value is an integer that ranges from <code>1</code> to <code>10000</code>. The default value is <code>10</code>.</p></li> <li><p><code>sgd.shuffleType</code> - Whether Amazon ML shuffles the training data. Shuffling data improves a model&#39;s ability to find the optimal solution for a variety of data types. The valid values are <code>auto</code> and <code>none</code>. The default value is <code>none</code>. We strongly recommend that you shuffle your data.</p></li> <li> <p><code>sgd.l1RegularizationAmount</code> - The coefficient regularization L1 norm. It controls overfitting the data by penalizing large coefficients. This tends to drive coefficients to zero, resulting in a sparse feature set. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX<em>DOUBLE</code>. The default is to not use L1 normalization. This parameter can&#39;t be used when <code>L2</code> is specified. Use this parameter sparingly.</p> </li> <li> <p><code>sgd.l2RegularizationAmount</code> - The coefficient regularization L2 norm. It controls overfitting the data by penalizing large coefficients. This tends to drive coefficients to small, nonzero values. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX</em>DOUBLE</code>. The default is to not use L2 normalization. This parameter can&#39;t be used when <code>L1</code> is specified. Use this parameter sparingly.</p> </li> </ul></p>
    #[serde(rename = "TrainingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p> Represents the output of a <code>GetMLModel</code> operation. </p> <p>The content consists of the detailed metadata and the current status of the <code>MLModel</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MLModel {
    /// <p><p>The algorithm used to train the <code>MLModel</code>. The following algorithm is supported:</p> <ul> <li> <code>SGD</code> -- Stochastic gradient descent. The goal of <code>SGD</code> is to minimize the gradient of the loss function. </li> </ul></p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "ComputeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_time: Option<i64>,
    /// <p>The time that the <code>MLModel</code> was created. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The AWS user account from which the <code>MLModel</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    #[serde(rename = "CreatedByIamUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_iam_user: Option<String>,
    /// <p>The current endpoint of the <code>MLModel</code>.</p>
    #[serde(rename = "EndpointInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_info: Option<RealtimeEndpointInfo>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    #[serde(rename = "InputDataLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_location_s3: Option<String>,
    /// <p>The time of the most recent edit to the <code>MLModel</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The ID assigned to the <code>MLModel</code> at creation.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
    /// <p><p>Identifies the <code>MLModel</code> category. The following are the available types:</p> <ul> <li> <code>REGRESSION</code> - Produces a numeric result. For example, &quot;What price should a house be listed at?&quot;</li> <li> <code>BINARY</code> - Produces one of two possible results. For example, &quot;Is this a child-friendly web site?&quot;.</li> <li> <code>MULTICLASS</code> - Produces one of several possible results. For example, &quot;Is this a HIGH-, LOW-, or MEDIUM&lt;?oxy<em>delete author=&quot;annbech&quot; timestamp=&quot;20160328T175050-0700&quot; content=&quot; &quot;&gt;&lt;?oxy</em>insert<em>start author=&quot;annbech&quot; timestamp=&quot;20160328T175050-0700&quot;&gt;-&lt;?oxy</em>insert_end&gt;risk trade?&quot;.</li> </ul></p>
    #[serde(rename = "MLModelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_type: Option<String>,
    /// <p>A description of the most recent details about accessing the <code>MLModel</code>.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>A user-supplied name or description of the <code>MLModel</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ScoreThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
    /// <p>The time of the most recent edit to the <code>ScoreThreshold</code>. The time is expressed in epoch time.</p>
    #[serde(rename = "ScoreThresholdLastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold_last_updated_at: Option<f64>,
    #[serde(rename = "SizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p><p>The current status of an <code>MLModel</code>. This element can have one of the following values: </p> <ul> <li> <code>PENDING</code> - Amazon Machine Learning (Amazon ML) submitted a request to create an <code>MLModel</code>.</li> <li> <code>INPROGRESS</code> - The creation process is underway.</li> <li> <code>FAILED</code> - The request to create an <code>MLModel</code> didn&#39;t run to completion. The model isn&#39;t usable.</li> <li> <code>COMPLETED</code> - The creation process completed successfully.</li> <li> <code>DELETED</code> - The <code>MLModel</code> is marked as deleted. It isn&#39;t usable.</li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of the training <code>DataSource</code>. The <code>CreateMLModel</code> operation uses the <code>TrainingDataSourceId</code>.</p>
    #[serde(rename = "TrainingDataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source_id: Option<String>,
    /// <p><p>A list of the training parameters in the <code>MLModel</code>. The list is implemented as a map of key-value pairs.</p> <p>The following is the current set of training parameters: </p> <ul> <li> <p><code>sgd.maxMLModelSizeInBytes</code> - The maximum allowed size of the model. Depending on the input data, the size of the model might affect its performance.</p> <p> The value is an integer that ranges from <code>100000</code> to <code>2147483648</code>. The default value is <code>33554432</code>.</p> </li> <li><p><code>sgd.maxPasses</code> - The number of times that the training process traverses the observations to build the <code>MLModel</code>. The value is an integer that ranges from <code>1</code> to <code>10000</code>. The default value is <code>10</code>.</p></li> <li><p><code>sgd.shuffleType</code> - Whether Amazon ML shuffles the training data. Shuffling the data improves a model&#39;s ability to find the optimal solution for a variety of data types. The valid values are <code>auto</code> and <code>none</code>. The default value is <code>none</code>.</p></li> <li> <p><code>sgd.l1RegularizationAmount</code> - The coefficient regularization L1 norm, which controls overfitting the data by penalizing large coefficients. This parameter tends to drive coefficients to zero, resulting in sparse feature set. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX<em>DOUBLE</code>. The default is to not use L1 normalization. This parameter can&#39;t be used when <code>L2</code> is specified. Use this parameter sparingly.</p> </li> <li> <p><code>sgd.l2RegularizationAmount</code> - The coefficient regularization L2 norm, which controls overfitting the data by penalizing large coefficients. This tends to drive coefficients to small, nonzero values. If you use this parameter, start by specifying a small value, such as <code>1.0E-08</code>.</p> <p>The value is a double that ranges from <code>0</code> to <code>MAX</em>DOUBLE</code>. The default is to not use L2 normalization. This parameter can&#39;t be used when <code>L1</code> is specified. Use this parameter sparingly.</p> </li> </ul></p>
    #[serde(rename = "TrainingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Measurements of how well the <code>MLModel</code> performed on known observations. One of the following metrics is returned, based on the type of the <code>MLModel</code>: </p> <ul> <li> <p>BinaryAUC: The binary <code>MLModel</code> uses the Area Under the Curve (AUC) technique to measure performance. </p> </li> <li> <p>RegressionRMSE: The regression <code>MLModel</code> uses the Root Mean Square Error (RMSE) technique to measure performance. RMSE measures the difference between predicted and actual values for a single variable.</p> </li> <li> <p>MulticlassAvgFScore: The multiclass <code>MLModel</code> uses the F1 score technique to measure performance. </p> </li> </ul> <p> For more information about performance metrics, please see the <a href="http://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PerformanceMetrics {
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PredictInput {
    /// <p>A unique identifier of the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
    #[serde(rename = "PredictEndpoint")]
    pub predict_endpoint: String,
    #[serde(rename = "Record")]
    pub record: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PredictOutput {
    #[serde(rename = "Prediction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
}

/// <p><p>The output from a <code>Predict</code> operation: </p> <ul> <li> <p> <code>Details</code> - Contains the following attributes: <code>DetailsAttributes.PREDICTIVE<em>MODEL</em>TYPE - REGRESSION | BINARY | MULTICLASS</code> <code>DetailsAttributes.ALGORITHM - SGD</code> </p> </li> <li> <p> <code>PredictedLabel</code> - Present for either a <code>BINARY</code> or <code>MULTICLASS</code> <code>MLModel</code> request. </p> </li> <li> <p> <code>PredictedScores</code> - Contains the raw classification score corresponding to each label. </p> </li> <li> <p> <code>PredictedValue</code> - Present for a <code>REGRESSION</code> <code>MLModel</code> request. </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Prediction {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The prediction label for either a <code>BINARY</code> or <code>MULTICLASS</code> <code>MLModel</code>.</p>
    #[serde(rename = "predictedLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_label: Option<String>,
    #[serde(rename = "predictedScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_scores: Option<::std::collections::HashMap<String, f32>>,
    /// <p>The prediction value for <code>REGRESSION</code> <code>MLModel</code>.</p>
    #[serde(rename = "predictedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_value: Option<f32>,
}

/// <p>The data specification of an Amazon Relational Database Service (Amazon RDS) <code>DataSource</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RDSDataSpec {
    /// <p><p>A JSON string that represents the splitting and rearrangement processing to be applied to a <code>DataSource</code>. If the <code>DataRearrangement</code> parameter is not provided, all of the input data is used to create the <code>Datasource</code>.</p> <p>There are multiple parameters that control what data is used to create a datasource:</p> <ul> <li><p><b><code>percentBegin</code></b></p> <p>Use <code>percentBegin</code> to indicate the beginning of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>percentEnd</code></b></p> <p>Use <code>percentEnd</code> to indicate the end of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>complement</code></b></p> <p>The <code>complement</code> parameter instructs Amazon ML to use the data that is not included in the range of <code>percentBegin</code> to <code>percentEnd</code> to create a datasource. The <code>complement</code> parameter is useful if you need to create complementary datasources for training and evaluation. To create a complementary datasource, use the same values for <code>percentBegin</code> and <code>percentEnd</code>, along with the <code>complement</code> parameter.</p> <p>For example, the following two datasources do not share any data, and can be used to train and evaluate a model. The first datasource has 25 percent of the data, and the second one has 75 percent of the data.</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> <li><p><b><code>strategy</code></b></p> <p>To change how Amazon ML splits the data for a datasource, use the <code>strategy</code> parameter.</p> <p>The default value for the <code>strategy</code> parameter is <code>sequential</code>, meaning that Amazon ML takes all of the data records between the <code>percentBegin</code> and <code>percentEnd</code> parameters for the datasource, in the order that the records appear in the input data.</p> <p>The following two <code>DataRearrangement</code> lines are examples of sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> <p>To randomly split the input data into the proportions indicated by the percentBegin and percentEnd parameters, set the <code>strategy</code> parameter to <code>random</code> and provide a string that is used as the seed value for the random data splitting (for example, you can use the S3 path to your data as the random seed string). If you choose the random split strategy, Amazon ML assigns each row of data a pseudo-random number between 0 and 100, and then selects the rows that have an assigned number between <code>percentBegin</code> and <code>percentEnd</code>. Pseudo-random numbers are assigned using both the input seed string value and the byte offset as a seed, so changing the data results in a different split. Any existing ordering is preserved. The random splitting strategy ensures that variables in the training and evaluation data are distributed similarly. It is useful in the cases where the input data may have an implicit sort order, which would otherwise result in training and evaluation datasources containing non-similar data records.</p> <p>The following two <code>DataRearrangement</code> lines are examples of non-sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> </ul></p>
    #[serde(rename = "DataRearrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_rearrangement: Option<String>,
    /// <p><p>A JSON string that represents the schema for an Amazon RDS <code>DataSource</code>. The <code>DataSchema</code> defines the structure of the observation data in the data file(s) referenced in the <code>DataSource</code>.</p> <p>A <code>DataSchema</code> is not required if you specify a <code>DataSchemaUri</code></p> <p>Define your <code>DataSchema</code> as a series of key-value pairs. <code>attributes</code> and <code>excludedVariableNames</code> have an array of key-value pairs for their value. Use the following format to define your <code>DataSchema</code>.</p> <p>{ &quot;version&quot;: &quot;1.0&quot;,</p> <p> &quot;recordAnnotationFieldName&quot;: &quot;F1&quot;,</p> <p> &quot;recordWeightFieldName&quot;: &quot;F2&quot;,</p> <p> &quot;targetFieldName&quot;: &quot;F3&quot;,</p> <p> &quot;dataFormat&quot;: &quot;CSV&quot;,</p> <p> &quot;dataFileContainsHeader&quot;: true,</p> <p> &quot;attributes&quot;: [</p> <p> { &quot;fieldName&quot;: &quot;F1&quot;, &quot;fieldType&quot;: &quot;TEXT&quot; }, { &quot;fieldName&quot;: &quot;F2&quot;, &quot;fieldType&quot;: &quot;NUMERIC&quot; }, { &quot;fieldName&quot;: &quot;F3&quot;, &quot;fieldType&quot;: &quot;CATEGORICAL&quot; }, { &quot;fieldName&quot;: &quot;F4&quot;, &quot;fieldType&quot;: &quot;NUMERIC&quot; }, { &quot;fieldName&quot;: &quot;F5&quot;, &quot;fieldType&quot;: &quot;CATEGORICAL&quot; }, { &quot;fieldName&quot;: &quot;F6&quot;, &quot;fieldType&quot;: &quot;TEXT&quot; }, { &quot;fieldName&quot;: &quot;F7&quot;, &quot;fieldType&quot;: &quot;WEIGHTED<em>INT</em>SEQUENCE&quot; }, { &quot;fieldName&quot;: &quot;F8&quot;, &quot;fieldType&quot;: &quot;WEIGHTED<em>STRING</em>SEQUENCE&quot; } ],</p> <p> &quot;excludedVariableNames&quot;: [ &quot;F6&quot; ] } </p> &lt;?oxy<em>insert</em>end&gt;</p>
    #[serde(rename = "DataSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<String>,
    /// <p>The Amazon S3 location of the <code>DataSchema</code>. </p>
    #[serde(rename = "DataSchemaUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema_uri: Option<String>,
    /// <p>The AWS Identity and Access Management (IAM) credentials that are used connect to the Amazon RDS database.</p>
    #[serde(rename = "DatabaseCredentials")]
    pub database_credentials: RDSDatabaseCredentials,
    /// <p>Describes the <code>DatabaseName</code> and <code>InstanceIdentifier</code> of an Amazon RDS database.</p>
    #[serde(rename = "DatabaseInformation")]
    pub database_information: RDSDatabase,
    /// <p>The role (DataPipelineDefaultResourceRole) assumed by an Amazon Elastic Compute Cloud (Amazon EC2) instance to carry out the copy operation from Amazon RDS to an Amazon S3 task. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
    #[serde(rename = "ResourceRole")]
    pub resource_role: String,
    /// <p>The Amazon S3 location for staging Amazon RDS data. The data retrieved from Amazon RDS using <code>SelectSqlQuery</code> is stored in this location.</p>
    #[serde(rename = "S3StagingLocation")]
    pub s3_staging_location: String,
    /// <p>The security group IDs to be used to access a VPC-based RDS DB instance. Ensure that there are appropriate ingress rules set up to allow access to the RDS DB instance. This attribute is used by Data Pipeline to carry out the copy operation from Amazon RDS to an Amazon S3 task.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The query that is used to retrieve the observation data for the <code>DataSource</code>.</p>
    #[serde(rename = "SelectSqlQuery")]
    pub select_sql_query: String,
    /// <p>The role (DataPipelineDefaultRole) assumed by AWS Data Pipeline service to monitor the progress of the copy task from Amazon RDS to Amazon S3. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
    #[serde(rename = "ServiceRole")]
    pub service_role: String,
    /// <p>The subnet ID to be used to access a VPC-based RDS DB instance. This attribute is used by Data Pipeline to carry out the copy task from Amazon RDS to Amazon S3.</p>
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// <p>The database details of an Amazon RDS database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RDSDatabase {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The ID of an RDS DB instance.</p>
    #[serde(rename = "InstanceIdentifier")]
    pub instance_identifier: String,
}

/// <p>The database credentials to connect to a database on an RDS DB instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RDSDatabaseCredentials {
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The datasource details that are specific to Amazon RDS.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RDSMetadata {
    /// <p>The ID of the Data Pipeline instance that is used to carry to copy data from Amazon RDS to Amazon S3. You can use the ID to find details about the instance in the Data Pipeline console.</p>
    #[serde(rename = "DataPipelineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_pipeline_id: Option<String>,
    /// <p>The database details required to connect to an Amazon RDS.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<RDSDatabase>,
    #[serde(rename = "DatabaseUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_user_name: Option<String>,
    /// <p>The role (DataPipelineDefaultResourceRole) assumed by an Amazon EC2 instance to carry out the copy task from Amazon RDS to Amazon S3. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
    #[serde(rename = "ResourceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    /// <p>The SQL query that is supplied during <a>CreateDataSourceFromRDS</a>. Returns only if <code>Verbose</code> is true in <code>GetDataSourceInput</code>. </p>
    #[serde(rename = "SelectSqlQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_sql_query: Option<String>,
    /// <p>The role (DataPipelineDefaultRole) assumed by the Data Pipeline service to monitor the progress of the copy task from Amazon RDS to Amazon S3. For more information, see <a href="http://docs.aws.amazon.com/datapipeline/latest/DeveloperGuide/dp-iam-roles.html">Role templates</a> for data pipelines.</p>
    #[serde(rename = "ServiceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
}

/// <p> Describes the real-time endpoint information for an <code>MLModel</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RealtimeEndpointInfo {
    /// <p>The time that the request to create the real-time endpoint for the <code>MLModel</code> was received. The time is expressed in epoch time.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p><p> The current status of the real-time endpoint for the <code>MLModel</code>. This element can have one of the following values: </p> <ul> <li> <code>NONE</code> - Endpoint does not exist or was previously deleted.</li> <li> <code>READY</code> - Endpoint is ready to be used for real-time predictions.</li> <li> <code>UPDATING</code> - Updating/creating the endpoint. </li> </ul></p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p><p>The URI that specifies where to send real-time prediction requests for the <code>MLModel</code>.</p> <note><title>Note</title> <p>The application must wait until the real-time endpoint is ready before using this URI.</p> </note></p>
    #[serde(rename = "EndpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// <p> The maximum processing rate for the real-time endpoint for <code>MLModel</code>, measured in incoming requests per second.</p>
    #[serde(rename = "PeakRequestsPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_requests_per_second: Option<i64>,
}

/// <p>Describes the data specification of an Amazon Redshift <code>DataSource</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RedshiftDataSpec {
    /// <p><p>A JSON string that represents the splitting and rearrangement processing to be applied to a <code>DataSource</code>. If the <code>DataRearrangement</code> parameter is not provided, all of the input data is used to create the <code>Datasource</code>.</p> <p>There are multiple parameters that control what data is used to create a datasource:</p> <ul> <li><p><b><code>percentBegin</code></b></p> <p>Use <code>percentBegin</code> to indicate the beginning of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>percentEnd</code></b></p> <p>Use <code>percentEnd</code> to indicate the end of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>complement</code></b></p> <p>The <code>complement</code> parameter instructs Amazon ML to use the data that is not included in the range of <code>percentBegin</code> to <code>percentEnd</code> to create a datasource. The <code>complement</code> parameter is useful if you need to create complementary datasources for training and evaluation. To create a complementary datasource, use the same values for <code>percentBegin</code> and <code>percentEnd</code>, along with the <code>complement</code> parameter.</p> <p>For example, the following two datasources do not share any data, and can be used to train and evaluate a model. The first datasource has 25 percent of the data, and the second one has 75 percent of the data.</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> <li><p><b><code>strategy</code></b></p> <p>To change how Amazon ML splits the data for a datasource, use the <code>strategy</code> parameter.</p> <p>The default value for the <code>strategy</code> parameter is <code>sequential</code>, meaning that Amazon ML takes all of the data records between the <code>percentBegin</code> and <code>percentEnd</code> parameters for the datasource, in the order that the records appear in the input data.</p> <p>The following two <code>DataRearrangement</code> lines are examples of sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> <p>To randomly split the input data into the proportions indicated by the percentBegin and percentEnd parameters, set the <code>strategy</code> parameter to <code>random</code> and provide a string that is used as the seed value for the random data splitting (for example, you can use the S3 path to your data as the random seed string). If you choose the random split strategy, Amazon ML assigns each row of data a pseudo-random number between 0 and 100, and then selects the rows that have an assigned number between <code>percentBegin</code> and <code>percentEnd</code>. Pseudo-random numbers are assigned using both the input seed string value and the byte offset as a seed, so changing the data results in a different split. Any existing ordering is preserved. The random splitting strategy ensures that variables in the training and evaluation data are distributed similarly. It is useful in the cases where the input data may have an implicit sort order, which would otherwise result in training and evaluation datasources containing non-similar data records.</p> <p>The following two <code>DataRearrangement</code> lines are examples of non-sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> </ul></p>
    #[serde(rename = "DataRearrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_rearrangement: Option<String>,
    /// <p>A JSON string that represents the schema for an Amazon Redshift <code>DataSource</code>. The <code>DataSchema</code> defines the structure of the observation data in the data file(s) referenced in the <code>DataSource</code>.</p> <p>A <code>DataSchema</code> is not required if you specify a <code>DataSchemaUri</code>.</p> <p>Define your <code>DataSchema</code> as a series of key-value pairs. <code>attributes</code> and <code>excludedVariableNames</code> have an array of key-value pairs for their value. Use the following format to define your <code>DataSchema</code>.</p> <p>{ "version": "1.0",</p> <p> "recordAnnotationFieldName": "F1",</p> <p> "recordWeightFieldName": "F2",</p> <p> "targetFieldName": "F3",</p> <p> "dataFormat": "CSV",</p> <p> "dataFileContainsHeader": true,</p> <p> "attributes": [</p> <p> { "fieldName": "F1", "fieldType": "TEXT" }, { "fieldName": "F2", "fieldType": "NUMERIC" }, { "fieldName": "F3", "fieldType": "CATEGORICAL" }, { "fieldName": "F4", "fieldType": "NUMERIC" }, { "fieldName": "F5", "fieldType": "CATEGORICAL" }, { "fieldName": "F6", "fieldType": "TEXT" }, { "fieldName": "F7", "fieldType": "WEIGHTED_INT_SEQUENCE" }, { "fieldName": "F8", "fieldType": "WEIGHTED_STRING_SEQUENCE" } ],</p> <p> "excludedVariableNames": [ "F6" ] } </p>
    #[serde(rename = "DataSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<String>,
    /// <p>Describes the schema location for an Amazon Redshift <code>DataSource</code>.</p>
    #[serde(rename = "DataSchemaUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema_uri: Option<String>,
    /// <p>Describes AWS Identity and Access Management (IAM) credentials that are used connect to the Amazon Redshift database.</p>
    #[serde(rename = "DatabaseCredentials")]
    pub database_credentials: RedshiftDatabaseCredentials,
    /// <p>Describes the <code>DatabaseName</code> and <code>ClusterIdentifier</code> for an Amazon Redshift <code>DataSource</code>.</p>
    #[serde(rename = "DatabaseInformation")]
    pub database_information: RedshiftDatabase,
    /// <p>Describes an Amazon S3 location to store the result set of the <code>SelectSqlQuery</code> query.</p>
    #[serde(rename = "S3StagingLocation")]
    pub s3_staging_location: String,
    /// <p>Describes the SQL Query to execute on an Amazon Redshift database for an Amazon Redshift <code>DataSource</code>.</p>
    #[serde(rename = "SelectSqlQuery")]
    pub select_sql_query: String,
}

/// <p>Describes the database details required to connect to an Amazon Redshift database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedshiftDatabase {
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: String,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
}

/// <p> Describes the database credentials for connecting to a database on an Amazon Redshift cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RedshiftDatabaseCredentials {
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes the <code>DataSource</code> details specific to Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RedshiftMetadata {
    #[serde(rename = "DatabaseUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_user_name: Option<String>,
    #[serde(rename = "RedshiftDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_database: Option<RedshiftDatabase>,
    /// <p> The SQL query that is specified during <a>CreateDataSourceFromRedshift</a>. Returns only if <code>Verbose</code> is true in GetDataSourceInput. </p>
    #[serde(rename = "SelectSqlQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_sql_query: Option<String>,
}

/// <p> Describes the data specification of a <code>DataSource</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3DataSpec {
    /// <p>The location of the data file(s) used by a <code>DataSource</code>. The URI specifies a data file or an Amazon Simple Storage Service (Amazon S3) directory or bucket containing data files.</p>
    #[serde(rename = "DataLocationS3")]
    pub data_location_s3: String,
    /// <p><p>A JSON string that represents the splitting and rearrangement processing to be applied to a <code>DataSource</code>. If the <code>DataRearrangement</code> parameter is not provided, all of the input data is used to create the <code>Datasource</code>.</p> <p>There are multiple parameters that control what data is used to create a datasource:</p> <ul> <li><p><b><code>percentBegin</code></b></p> <p>Use <code>percentBegin</code> to indicate the beginning of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>percentEnd</code></b></p> <p>Use <code>percentEnd</code> to indicate the end of the range of the data used to create the Datasource. If you do not include <code>percentBegin</code> and <code>percentEnd</code>, Amazon ML includes all of the data when creating the datasource.</p></li> <li><p><b><code>complement</code></b></p> <p>The <code>complement</code> parameter instructs Amazon ML to use the data that is not included in the range of <code>percentBegin</code> to <code>percentEnd</code> to create a datasource. The <code>complement</code> parameter is useful if you need to create complementary datasources for training and evaluation. To create a complementary datasource, use the same values for <code>percentBegin</code> and <code>percentEnd</code>, along with the <code>complement</code> parameter.</p> <p>For example, the following two datasources do not share any data, and can be used to train and evaluate a model. The first datasource has 25 percent of the data, and the second one has 75 percent of the data.</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:0, &quot;percentEnd&quot;:25, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> <li><p><b><code>strategy</code></b></p> <p>To change how Amazon ML splits the data for a datasource, use the <code>strategy</code> parameter.</p> <p>The default value for the <code>strategy</code> parameter is <code>sequential</code>, meaning that Amazon ML takes all of the data records between the <code>percentBegin</code> and <code>percentEnd</code> parameters for the datasource, in the order that the records appear in the input data.</p> <p>The following two <code>DataRearrangement</code> lines are examples of sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;sequential&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> <p>To randomly split the input data into the proportions indicated by the percentBegin and percentEnd parameters, set the <code>strategy</code> parameter to <code>random</code> and provide a string that is used as the seed value for the random data splitting (for example, you can use the S3 path to your data as the random seed string). If you choose the random split strategy, Amazon ML assigns each row of data a pseudo-random number between 0 and 100, and then selects the rows that have an assigned number between <code>percentBegin</code> and <code>percentEnd</code>. Pseudo-random numbers are assigned using both the input seed string value and the byte offset as a seed, so changing the data results in a different split. Any existing ordering is preserved. The random splitting strategy ensures that variables in the training and evaluation data are distributed similarly. It is useful in the cases where the input data may have an implicit sort order, which would otherwise result in training and evaluation datasources containing non-similar data records.</p> <p>The following two <code>DataRearrangement</code> lines are examples of non-sequentially ordered training and evaluation datasources:</p> <p>Datasource for evaluation: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;}}</code></p> <p>Datasource for training: <code>{&quot;splitting&quot;:{&quot;percentBegin&quot;:70, &quot;percentEnd&quot;:100, &quot;strategy&quot;:&quot;random&quot;, &quot;randomSeed&quot;=&quot;s3://my<em>s3</em>path/bucket/file.csv&quot;, &quot;complement&quot;:&quot;true&quot;}}</code></p> </li> </ul></p>
    #[serde(rename = "DataRearrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_rearrangement: Option<String>,
    /// <p><p> A JSON string that represents the schema for an Amazon S3 <code>DataSource</code>. The <code>DataSchema</code> defines the structure of the observation data in the data file(s) referenced in the <code>DataSource</code>.</p> <p>You must provide either the <code>DataSchema</code> or the <code>DataSchemaLocationS3</code>.</p> <p>Define your <code>DataSchema</code> as a series of key-value pairs. <code>attributes</code> and <code>excludedVariableNames</code> have an array of key-value pairs for their value. Use the following format to define your <code>DataSchema</code>.</p> <p>{ &quot;version&quot;: &quot;1.0&quot;,</p> <p> &quot;recordAnnotationFieldName&quot;: &quot;F1&quot;,</p> <p> &quot;recordWeightFieldName&quot;: &quot;F2&quot;,</p> <p> &quot;targetFieldName&quot;: &quot;F3&quot;,</p> <p> &quot;dataFormat&quot;: &quot;CSV&quot;,</p> <p> &quot;dataFileContainsHeader&quot;: true,</p> <p> &quot;attributes&quot;: [</p> <p> { &quot;fieldName&quot;: &quot;F1&quot;, &quot;fieldType&quot;: &quot;TEXT&quot; }, { &quot;fieldName&quot;: &quot;F2&quot;, &quot;fieldType&quot;: &quot;NUMERIC&quot; }, { &quot;fieldName&quot;: &quot;F3&quot;, &quot;fieldType&quot;: &quot;CATEGORICAL&quot; }, { &quot;fieldName&quot;: &quot;F4&quot;, &quot;fieldType&quot;: &quot;NUMERIC&quot; }, { &quot;fieldName&quot;: &quot;F5&quot;, &quot;fieldType&quot;: &quot;CATEGORICAL&quot; }, { &quot;fieldName&quot;: &quot;F6&quot;, &quot;fieldType&quot;: &quot;TEXT&quot; }, { &quot;fieldName&quot;: &quot;F7&quot;, &quot;fieldType&quot;: &quot;WEIGHTED<em>INT</em>SEQUENCE&quot; }, { &quot;fieldName&quot;: &quot;F8&quot;, &quot;fieldType&quot;: &quot;WEIGHTED<em>STRING</em>SEQUENCE&quot; } ],</p> <p> &quot;excludedVariableNames&quot;: [ &quot;F6&quot; ] } </p> &lt;?oxy<em>insert</em>end&gt;</p>
    #[serde(rename = "DataSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<String>,
    /// <p>Describes the schema location in Amazon S3. You must provide either the <code>DataSchema</code> or the <code>DataSchemaLocationS3</code>.</p>
    #[serde(rename = "DataSchemaLocationS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema_location_s3: Option<String>,
}

/// <p>A custom key-value pair associated with an ML object, such as an ML model.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A unique identifier for the tag. Valid characters include Unicode letters, digits, white space, _, ., /, =, +, -, %, and @.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>An optional string, typically used to describe or define the tag. Valid characters include Unicode letters, digits, white space, _, ., /, =, +, -, %, and @.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBatchPredictionInput {
    /// <p>The ID assigned to the <code>BatchPrediction</code> during creation.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
    /// <p>A new user-supplied name or description of the <code>BatchPrediction</code>.</p>
    #[serde(rename = "BatchPredictionName")]
    pub batch_prediction_name: String,
}

/// <p>Represents the output of an <code>UpdateBatchPrediction</code> operation.</p> <p>You can see the updated content by using the <code>GetBatchPrediction</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBatchPredictionOutput {
    /// <p>The ID assigned to the <code>BatchPrediction</code> during creation. This value should be identical to the value of the <code>BatchPredictionId</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSourceInput {
    /// <p>The ID assigned to the <code>DataSource</code> during creation.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
    /// <p>A new user-supplied name or description of the <code>DataSource</code> that will replace the current description. </p>
    #[serde(rename = "DataSourceName")]
    pub data_source_name: String,
}

/// <p>Represents the output of an <code>UpdateDataSource</code> operation.</p> <p>You can see the updated content by using the <code>GetBatchPrediction</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataSourceOutput {
    /// <p>The ID assigned to the <code>DataSource</code> during creation. This value should be identical to the value of the <code>DataSourceID</code> in the request.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEvaluationInput {
    /// <p>The ID assigned to the <code>Evaluation</code> during creation.</p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
    /// <p>A new user-supplied name or description of the <code>Evaluation</code> that will replace the current content. </p>
    #[serde(rename = "EvaluationName")]
    pub evaluation_name: String,
}

/// <p>Represents the output of an <code>UpdateEvaluation</code> operation.</p> <p>You can see the updated content by using the <code>GetEvaluation</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEvaluationOutput {
    /// <p>The ID assigned to the <code>Evaluation</code> during creation. This value should be identical to the value of the <code>Evaluation</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMLModelInput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
    /// <p>A user-supplied name or description of the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_name: Option<String>,
    /// <p>The <code>ScoreThreshold</code> used in binary classification <code>MLModel</code> that marks the boundary between a positive prediction and a negative prediction.</p> <p>Output values greater than or equal to the <code>ScoreThreshold</code> receive a positive result from the <code>MLModel</code>, such as <code>true</code>. Output values less than the <code>ScoreThreshold</code> receive a negative response from the <code>MLModel</code>, such as <code>false</code>.</p>
    #[serde(rename = "ScoreThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
}

/// <p>Represents the output of an <code>UpdateMLModel</code> operation.</p> <p>You can see the updated content by using the <code>GetMLModel</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMLModelOutput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation. This value should be identical to the value of the <code>MLModelID</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
}

/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),

    InvalidTag(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),

    TagLimitExceeded(String),
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(AddTagsError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AddTagsError::InvalidInput(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(AddTagsError::InvalidTag(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddTagsError::ResourceNotFound(err.msg))
                }
                "TagLimitExceededException" => {
                    return RusotoError::Service(AddTagsError::TagLimitExceeded(err.msg))
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
            AddTagsError::InternalServer(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidTag(ref cause) => write!(f, "{}", cause),
            AddTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsError::TagLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CreateBatchPrediction
#[derive(Debug, PartialEq)]
pub enum CreateBatchPredictionError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateBatchPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBatchPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateBatchPredictionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateBatchPredictionError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateBatchPredictionError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBatchPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBatchPredictionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateBatchPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateBatchPredictionError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBatchPredictionError {}
/// Errors returned by CreateDataSourceFromRDS
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromRDSError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateDataSourceFromRDSError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSourceFromRDSError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDataSourceFromRDSError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataSourceFromRDSError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDataSourceFromRDSError::InvalidInput(
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
impl fmt::Display for CreateDataSourceFromRDSError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSourceFromRDSError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataSourceFromRDSError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataSourceFromRDSError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSourceFromRDSError {}
/// Errors returned by CreateDataSourceFromRedshift
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromRedshiftError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateDataSourceFromRedshiftError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDataSourceFromRedshiftError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDataSourceFromRedshiftError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataSourceFromRedshiftError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDataSourceFromRedshiftError::InvalidInput(
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
impl fmt::Display for CreateDataSourceFromRedshiftError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSourceFromRedshiftError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataSourceFromRedshiftError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataSourceFromRedshiftError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSourceFromRedshiftError {}
/// Errors returned by CreateDataSourceFromS3
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromS3Error {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateDataSourceFromS3Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSourceFromS3Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDataSourceFromS3Error::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataSourceFromS3Error::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDataSourceFromS3Error::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataSourceFromS3Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSourceFromS3Error::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDataSourceFromS3Error::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataSourceFromS3Error::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSourceFromS3Error {}
/// Errors returned by CreateEvaluation
#[derive(Debug, PartialEq)]
pub enum CreateEvaluationError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateEvaluationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateEvaluationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateEvaluationError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateEvaluationError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEvaluationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEvaluationError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            CreateEvaluationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateEvaluationError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEvaluationError {}
/// Errors returned by CreateMLModel
#[derive(Debug, PartialEq)]
pub enum CreateMLModelError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl CreateMLModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMLModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateMLModelError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateMLModelError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateMLModelError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMLModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMLModelError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            CreateMLModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateMLModelError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMLModelError {}
/// Errors returned by CreateRealtimeEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateRealtimeEndpointError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl CreateRealtimeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRealtimeEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateRealtimeEndpointError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateRealtimeEndpointError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateRealtimeEndpointError::ResourceNotFound(
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
impl fmt::Display for CreateRealtimeEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRealtimeEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRealtimeEndpointError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateRealtimeEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRealtimeEndpointError {}
/// Errors returned by DeleteBatchPrediction
#[derive(Debug, PartialEq)]
pub enum DeleteBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteBatchPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBatchPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteBatchPredictionError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteBatchPredictionError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBatchPredictionError::ResourceNotFound(
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
impl fmt::Display for DeleteBatchPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBatchPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteBatchPredictionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteBatchPredictionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBatchPredictionError {}
/// Errors returned by DeleteDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDataSourceError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDataSourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDataSourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataSourceError {}
/// Errors returned by DeleteEvaluation
#[derive(Debug, PartialEq)]
pub enum DeleteEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteEvaluationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEvaluationError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteEvaluationError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEvaluationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEvaluationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEvaluationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEvaluationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteEvaluationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEvaluationError {}
/// Errors returned by DeleteMLModel
#[derive(Debug, PartialEq)]
pub enum DeleteMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteMLModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMLModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteMLModelError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteMLModelError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMLModelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMLModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMLModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteMLModelError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteMLModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMLModelError {}
/// Errors returned by DeleteRealtimeEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteRealtimeEndpointError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteRealtimeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRealtimeEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteRealtimeEndpointError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteRealtimeEndpointError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteRealtimeEndpointError::ResourceNotFound(
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
impl fmt::Display for DeleteRealtimeEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRealtimeEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteRealtimeEndpointError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteRealtimeEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRealtimeEndpointError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),

    InvalidTag(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteTagsError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidInput(err.msg))
                }
                "InvalidTagException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidTag(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InvalidTag(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DescribeBatchPredictions
#[derive(Debug, PartialEq)]
pub enum DescribeBatchPredictionsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl DescribeBatchPredictionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBatchPredictionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeBatchPredictionsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeBatchPredictionsError::InvalidInput(
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
impl fmt::Display for DescribeBatchPredictionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBatchPredictionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeBatchPredictionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBatchPredictionsError {}
/// Errors returned by DescribeDataSources
#[derive(Debug, PartialEq)]
pub enum DescribeDataSourcesError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl DescribeDataSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDataSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDataSourcesError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeDataSourcesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDataSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataSourcesError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDataSourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSourcesError {}
/// Errors returned by DescribeEvaluations
#[derive(Debug, PartialEq)]
pub enum DescribeEvaluationsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl DescribeEvaluationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEvaluationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEvaluationsError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeEvaluationsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEvaluationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEvaluationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEvaluationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEvaluationsError {}
/// Errors returned by DescribeMLModels
#[derive(Debug, PartialEq)]
pub enum DescribeMLModelsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
}

impl DescribeMLModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMLModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeMLModelsError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeMLModelsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMLModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMLModelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeMLModelsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMLModelsError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeTagsError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeTagsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by GetBatchPrediction
#[derive(Debug, PartialEq)]
pub enum GetBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl GetBatchPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBatchPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetBatchPredictionError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetBatchPredictionError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBatchPredictionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBatchPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBatchPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetBatchPredictionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetBatchPredictionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBatchPredictionError {}
/// Errors returned by GetDataSource
#[derive(Debug, PartialEq)]
pub enum GetDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl GetDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDataSourceError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDataSourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDataSourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDataSourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataSourceError {}
/// Errors returned by GetEvaluation
#[derive(Debug, PartialEq)]
pub enum GetEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl GetEvaluationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetEvaluationError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetEvaluationError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEvaluationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEvaluationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEvaluationError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEvaluationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetEvaluationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEvaluationError {}
/// Errors returned by GetMLModel
#[derive(Debug, PartialEq)]
pub enum GetMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl GetMLModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMLModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetMLModelError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMLModelError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMLModelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMLModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMLModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetMLModelError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetMLModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMLModelError {}
/// Errors returned by Predict
#[derive(Debug, PartialEq)]
pub enum PredictError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>The subscriber exceeded the maximum number of operations. This exception can occur when listing objects such as <code>DataSource</code>.</p>
    LimitExceeded(String),
    /// <p>The exception is thrown when a predict request is made to an unmounted <code>MLModel</code>.</p>
    PredictorNotMounted(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl PredictError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PredictError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PredictError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PredictError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PredictError::LimitExceeded(err.msg))
                }
                "PredictorNotMountedException" => {
                    return RusotoError::Service(PredictError::PredictorNotMounted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PredictError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PredictError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PredictError::InternalServer(ref cause) => write!(f, "{}", cause),
            PredictError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PredictError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PredictError::PredictorNotMounted(ref cause) => write!(f, "{}", cause),
            PredictError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PredictError {}
/// Errors returned by UpdateBatchPrediction
#[derive(Debug, PartialEq)]
pub enum UpdateBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl UpdateBatchPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBatchPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateBatchPredictionError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateBatchPredictionError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateBatchPredictionError::ResourceNotFound(
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
impl fmt::Display for UpdateBatchPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBatchPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateBatchPredictionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateBatchPredictionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBatchPredictionError {}
/// Errors returned by UpdateDataSource
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl UpdateDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDataSourceError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDataSourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataSourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSourceError {}
/// Errors returned by UpdateEvaluation
#[derive(Debug, PartialEq)]
pub enum UpdateEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl UpdateEvaluationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateEvaluationError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateEvaluationError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateEvaluationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEvaluationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEvaluationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateEvaluationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateEvaluationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEvaluationError {}
/// Errors returned by UpdateMLModel
#[derive(Debug, PartialEq)]
pub enum UpdateMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
}

impl UpdateMLModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMLModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateMLModelError::InternalServer(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateMLModelError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateMLModelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMLModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMLModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateMLModelError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateMLModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMLModelError {}
/// Trait representing the capabilities of the Amazon Machine Learning API. Amazon Machine Learning clients implement this trait.
#[async_trait]
pub trait MachineLearning {
    /// <p>Adds one or more tags to an object, up to a limit of 10. Each tag consists of a key and an optional value. If you add a tag using a key that is already associated with the ML object, <code>AddTags</code> updates the tag's value.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>>;

    /// <p>Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a <code>DataSource</code>. This operation creates a new <code>BatchPrediction</code>, and uses an <code>MLModel</code> and the data files referenced by the <code>DataSource</code> as information sources. </p> <p><code>CreateBatchPrediction</code> is an asynchronous operation. In response to <code>CreateBatchPrediction</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>BatchPrediction</code> status to <code>PENDING</code>. After the <code>BatchPrediction</code> completes, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can poll for status updates by using the <a>GetBatchPrediction</a> operation and checking the <code>Status</code> parameter of the result. After the <code>COMPLETED</code> status appears, the results are available in the location specified by the <code>OutputUri</code> parameter.</p>
    async fn create_batch_prediction(
        &self,
        input: CreateBatchPredictionInput,
    ) -> Result<CreateBatchPredictionOutput, RusotoError<CreateBatchPredictionError>>;

    /// <p>Creates a <code>DataSource</code> object from an <a href="http://aws.amazon.com/rds/"> Amazon Relational Database Service</a> (Amazon RDS). A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRDS</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRDS</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used only to perform <code>&gt;CreateMLModel</code>&gt;, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML cannot accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p>
    async fn create_data_source_from_rds(
        &self,
        input: CreateDataSourceFromRDSInput,
    ) -> Result<CreateDataSourceFromRDSOutput, RusotoError<CreateDataSourceFromRDSError>>;

    /// <p><p>Creates a <code>DataSource</code> from a database hosted on an Amazon Redshift cluster. A <code>DataSource</code> references data that can be used to perform either <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRedshift</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRedshift</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in <code>COMPLETED</code> or <code>PENDING</code> states can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can&#39;t accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observations should be contained in the database hosted on an Amazon Redshift cluster and should be specified by a <code>SelectSqlQuery</code> query. Amazon ML executes an <code>Unload</code> command in Amazon Redshift to transfer the result set of the <code>SelectSqlQuery</code> query to <code>S3StagingLocation</code>.</p> <p>After the <code>DataSource</code> has been created, it&#39;s ready for use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also requires a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p> &lt;?oxy<em>insert</em>start author=&quot;laurama&quot; timestamp=&quot;20160406T153842-0700&quot;&gt;<p>You can&#39;t change an existing datasource, but you can copy and modify the settings from an existing Amazon Redshift datasource to create a new datasource. To do so, call <code>GetDataSource</code> for an existing datasource and copy the values to a <code>CreateDataSource</code> call. Change the settings that you want to change and make sure that all required fields have the appropriate values.</p> &lt;?oxy<em>insert</em>end&gt;</p>
    async fn create_data_source_from_redshift(
        &self,
        input: CreateDataSourceFromRedshiftInput,
    ) -> Result<CreateDataSourceFromRedshiftOutput, RusotoError<CreateDataSourceFromRedshiftError>>;

    /// <p>Creates a <code>DataSource</code> object. A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromS3</code> is an asynchronous operation. In response to <code>CreateDataSourceFromS3</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> has been created and is ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code> or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can't accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observation data used in a <code>DataSource</code> should be ready to use; that is, it should have a consistent structure, and missing data values should be kept to a minimum. The observation data must reside in one or more .csv files in an Amazon Simple Storage Service (Amazon S3) location, along with a schema that describes the data items by name and type. The same schema must be used for all of the data files referenced by the <code>DataSource</code>. </p> <p>After the <code>DataSource</code> has been created, it's ready to use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also needs a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p>
    async fn create_data_source_from_s3(
        &self,
        input: CreateDataSourceFromS3Input,
    ) -> Result<CreateDataSourceFromS3Output, RusotoError<CreateDataSourceFromS3Error>>;

    /// <p>Creates a new <code>Evaluation</code> of an <code>MLModel</code>. An <code>MLModel</code> is evaluated on a set of observations associated to a <code>DataSource</code>. Like a <code>DataSource</code> for an <code>MLModel</code>, the <code>DataSource</code> for an <code>Evaluation</code> contains values for the <code>Target Variable</code>. The <code>Evaluation</code> compares the predicted result for each observation to the actual outcome and provides a summary so that you know how effective the <code>MLModel</code> functions on the test data. Evaluation generates a relevant performance metric, such as BinaryAUC, RegressionRMSE or MulticlassAvgFScore based on the corresponding <code>MLModelType</code>: <code>BINARY</code>, <code>REGRESSION</code> or <code>MULTICLASS</code>. </p> <p><code>CreateEvaluation</code> is an asynchronous operation. In response to <code>CreateEvaluation</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the evaluation status to <code>PENDING</code>. After the <code>Evaluation</code> is created and ready for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetEvaluation</code> operation to check progress of the evaluation during the creation operation.</p>
    async fn create_evaluation(
        &self,
        input: CreateEvaluationInput,
    ) -> Result<CreateEvaluationOutput, RusotoError<CreateEvaluationError>>;

    /// <p>Creates a new <code>MLModel</code> using the <code>DataSource</code> and the recipe as information sources. </p> <p>An <code>MLModel</code> is nearly immutable. Users can update only the <code>MLModelName</code> and the <code>ScoreThreshold</code> in an <code>MLModel</code> without creating a new <code>MLModel</code>. </p> <p><code>CreateMLModel</code> is an asynchronous operation. In response to <code>CreateMLModel</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>MLModel</code> status to <code>PENDING</code>. After the <code>MLModel</code> has been created and ready is for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetMLModel</code> operation to check the progress of the <code>MLModel</code> during the creation operation.</p> <p> <code>CreateMLModel</code> requires a <code>DataSource</code> with computed statistics, which can be created by setting <code>ComputeStatistics</code> to <code>true</code> in <code>CreateDataSourceFromRDS</code>, <code>CreateDataSourceFromS3</code>, or <code>CreateDataSourceFromRedshift</code> operations. </p>
    async fn create_ml_model(
        &self,
        input: CreateMLModelInput,
    ) -> Result<CreateMLModelOutput, RusotoError<CreateMLModelError>>;

    /// <p>Creates a real-time endpoint for the <code>MLModel</code>. The endpoint contains the URI of the <code>MLModel</code>; that is, the location to send real-time prediction requests for the specified <code>MLModel</code>.</p>
    async fn create_realtime_endpoint(
        &self,
        input: CreateRealtimeEndpointInput,
    ) -> Result<CreateRealtimeEndpointOutput, RusotoError<CreateRealtimeEndpointError>>;

    /// <p>Assigns the DELETED status to a <code>BatchPrediction</code>, rendering it unusable.</p> <p>After using the <code>DeleteBatchPrediction</code> operation, you can use the <a>GetBatchPrediction</a> operation to verify that the status of the <code>BatchPrediction</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteBatchPrediction</code> operation is irreversible.</p>
    async fn delete_batch_prediction(
        &self,
        input: DeleteBatchPredictionInput,
    ) -> Result<DeleteBatchPredictionOutput, RusotoError<DeleteBatchPredictionError>>;

    /// <p>Assigns the DELETED status to a <code>DataSource</code>, rendering it unusable.</p> <p>After using the <code>DeleteDataSource</code> operation, you can use the <a>GetDataSource</a> operation to verify that the status of the <code>DataSource</code> changed to DELETED.</p> <p><b>Caution:</b> The results of the <code>DeleteDataSource</code> operation are irreversible.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceInput,
    ) -> Result<DeleteDataSourceOutput, RusotoError<DeleteDataSourceError>>;

    /// <p><p>Assigns the <code>DELETED</code> status to an <code>Evaluation</code>, rendering it unusable.</p> <p>After invoking the <code>DeleteEvaluation</code> operation, you can use the <code>GetEvaluation</code> operation to verify that the status of the <code>Evaluation</code> changed to <code>DELETED</code>.</p> <caution><title>Caution</title> <p>The results of the <code>DeleteEvaluation</code> operation are irreversible.</p></caution></p>
    async fn delete_evaluation(
        &self,
        input: DeleteEvaluationInput,
    ) -> Result<DeleteEvaluationOutput, RusotoError<DeleteEvaluationError>>;

    /// <p>Assigns the <code>DELETED</code> status to an <code>MLModel</code>, rendering it unusable.</p> <p>After using the <code>DeleteMLModel</code> operation, you can use the <code>GetMLModel</code> operation to verify that the status of the <code>MLModel</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteMLModel</code> operation is irreversible.</p>
    async fn delete_ml_model(
        &self,
        input: DeleteMLModelInput,
    ) -> Result<DeleteMLModelOutput, RusotoError<DeleteMLModelError>>;

    /// <p>Deletes a real time endpoint of an <code>MLModel</code>.</p>
    async fn delete_realtime_endpoint(
        &self,
        input: DeleteRealtimeEndpointInput,
    ) -> Result<DeleteRealtimeEndpointOutput, RusotoError<DeleteRealtimeEndpointError>>;

    /// <p>Deletes the specified tags associated with an ML object. After this operation is complete, you can't recover deleted tags.</p> <p>If you specify a tag that doesn't exist, Amazon ML ignores it.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> Result<DeleteTagsOutput, RusotoError<DeleteTagsError>>;

    /// <p>Returns a list of <code>BatchPrediction</code> operations that match the search criteria in the request.</p>
    async fn describe_batch_predictions(
        &self,
        input: DescribeBatchPredictionsInput,
    ) -> Result<DescribeBatchPredictionsOutput, RusotoError<DescribeBatchPredictionsError>>;

    /// <p>Returns a list of <code>DataSource</code> that match the search criteria in the request.</p>
    async fn describe_data_sources(
        &self,
        input: DescribeDataSourcesInput,
    ) -> Result<DescribeDataSourcesOutput, RusotoError<DescribeDataSourcesError>>;

    /// <p>Returns a list of <code>DescribeEvaluations</code> that match the search criteria in the request.</p>
    async fn describe_evaluations(
        &self,
        input: DescribeEvaluationsInput,
    ) -> Result<DescribeEvaluationsOutput, RusotoError<DescribeEvaluationsError>>;

    /// <p>Returns a list of <code>MLModel</code> that match the search criteria in the request.</p>
    async fn describe_ml_models(
        &self,
        input: DescribeMLModelsInput,
    ) -> Result<DescribeMLModelsOutput, RusotoError<DescribeMLModelsError>>;

    /// <p>Describes one or more of the tags for your Amazon ML object.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>>;

    /// <p>Returns a <code>BatchPrediction</code> that includes detailed metadata, status, and data file information for a <code>Batch Prediction</code> request.</p>
    async fn get_batch_prediction(
        &self,
        input: GetBatchPredictionInput,
    ) -> Result<GetBatchPredictionOutput, RusotoError<GetBatchPredictionError>>;

    /// <p>Returns a <code>DataSource</code> that includes metadata and data file information, as well as the current status of the <code>DataSource</code>.</p> <p><code>GetDataSource</code> provides results in normal or verbose format. The verbose format adds the schema description and the list of files pointed to by the DataSource to the normal format.</p>
    async fn get_data_source(
        &self,
        input: GetDataSourceInput,
    ) -> Result<GetDataSourceOutput, RusotoError<GetDataSourceError>>;

    /// <p>Returns an <code>Evaluation</code> that includes metadata as well as the current status of the <code>Evaluation</code>.</p>
    async fn get_evaluation(
        &self,
        input: GetEvaluationInput,
    ) -> Result<GetEvaluationOutput, RusotoError<GetEvaluationError>>;

    /// <p>Returns an <code>MLModel</code> that includes detailed metadata, data source information, and the current status of the <code>MLModel</code>.</p> <p><code>GetMLModel</code> provides results in normal or verbose format. </p>
    async fn get_ml_model(
        &self,
        input: GetMLModelInput,
    ) -> Result<GetMLModelOutput, RusotoError<GetMLModelError>>;

    /// <p><p>Generates a prediction for the observation using the specified <code>ML Model</code>.</p> <note><title>Note</title> <p>Not all response parameters will be populated. Whether a response parameter is populated depends on the type of model requested.</p></note></p>
    async fn predict(
        &self,
        input: PredictInput,
    ) -> Result<PredictOutput, RusotoError<PredictError>>;

    /// <p>Updates the <code>BatchPredictionName</code> of a <code>BatchPrediction</code>.</p> <p>You can use the <code>GetBatchPrediction</code> operation to view the contents of the updated data element.</p>
    async fn update_batch_prediction(
        &self,
        input: UpdateBatchPredictionInput,
    ) -> Result<UpdateBatchPredictionOutput, RusotoError<UpdateBatchPredictionError>>;

    /// <p>Updates the <code>DataSourceName</code> of a <code>DataSource</code>.</p> <p>You can use the <code>GetDataSource</code> operation to view the contents of the updated data element.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceInput,
    ) -> Result<UpdateDataSourceOutput, RusotoError<UpdateDataSourceError>>;

    /// <p>Updates the <code>EvaluationName</code> of an <code>Evaluation</code>.</p> <p>You can use the <code>GetEvaluation</code> operation to view the contents of the updated data element.</p>
    async fn update_evaluation(
        &self,
        input: UpdateEvaluationInput,
    ) -> Result<UpdateEvaluationOutput, RusotoError<UpdateEvaluationError>>;

    /// <p>Updates the <code>MLModelName</code> and the <code>ScoreThreshold</code> of an <code>MLModel</code>.</p> <p>You can use the <code>GetMLModel</code> operation to view the contents of the updated data element.</p>
    async fn update_ml_model(
        &self,
        input: UpdateMLModelInput,
    ) -> Result<UpdateMLModelOutput, RusotoError<UpdateMLModelError>>;
}
/// A client for the Amazon Machine Learning API.
#[derive(Clone)]
pub struct MachineLearningClient {
    client: Client,
    region: region::Region,
}

impl MachineLearningClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MachineLearningClient {
        MachineLearningClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MachineLearningClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MachineLearningClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MachineLearningClient {
        MachineLearningClient { client, region }
    }
}

#[async_trait]
impl MachineLearning for MachineLearningClient {
    /// <p>Adds one or more tags to an object, up to a limit of 10. Each tag consists of a key and an optional value. If you add a tag using a key that is already associated with the ML object, <code>AddTags</code> updates the tag's value.</p>
    async fn add_tags(
        &self,
        input: AddTagsInput,
    ) -> Result<AddTagsOutput, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.AddTags");
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

    /// <p>Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a <code>DataSource</code>. This operation creates a new <code>BatchPrediction</code>, and uses an <code>MLModel</code> and the data files referenced by the <code>DataSource</code> as information sources. </p> <p><code>CreateBatchPrediction</code> is an asynchronous operation. In response to <code>CreateBatchPrediction</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>BatchPrediction</code> status to <code>PENDING</code>. After the <code>BatchPrediction</code> completes, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can poll for status updates by using the <a>GetBatchPrediction</a> operation and checking the <code>Status</code> parameter of the result. After the <code>COMPLETED</code> status appears, the results are available in the location specified by the <code>OutputUri</code> parameter.</p>
    async fn create_batch_prediction(
        &self,
        input: CreateBatchPredictionInput,
    ) -> Result<CreateBatchPredictionOutput, RusotoError<CreateBatchPredictionError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateBatchPrediction");
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
                .deserialize::<CreateBatchPredictionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBatchPredictionError::from_response(response))
        }
    }

    /// <p>Creates a <code>DataSource</code> object from an <a href="http://aws.amazon.com/rds/"> Amazon Relational Database Service</a> (Amazon RDS). A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRDS</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRDS</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used only to perform <code>&gt;CreateMLModel</code>&gt;, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML cannot accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p>
    async fn create_data_source_from_rds(
        &self,
        input: CreateDataSourceFromRDSInput,
    ) -> Result<CreateDataSourceFromRDSOutput, RusotoError<CreateDataSourceFromRDSError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateDataSourceFromRDS");
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
                .deserialize::<CreateDataSourceFromRDSOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSourceFromRDSError::from_response(response))
        }
    }

    /// <p><p>Creates a <code>DataSource</code> from a database hosted on an Amazon Redshift cluster. A <code>DataSource</code> references data that can be used to perform either <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRedshift</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRedshift</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in <code>COMPLETED</code> or <code>PENDING</code> states can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can&#39;t accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observations should be contained in the database hosted on an Amazon Redshift cluster and should be specified by a <code>SelectSqlQuery</code> query. Amazon ML executes an <code>Unload</code> command in Amazon Redshift to transfer the result set of the <code>SelectSqlQuery</code> query to <code>S3StagingLocation</code>.</p> <p>After the <code>DataSource</code> has been created, it&#39;s ready for use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also requires a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p> &lt;?oxy<em>insert</em>start author=&quot;laurama&quot; timestamp=&quot;20160406T153842-0700&quot;&gt;<p>You can&#39;t change an existing datasource, but you can copy and modify the settings from an existing Amazon Redshift datasource to create a new datasource. To do so, call <code>GetDataSource</code> for an existing datasource and copy the values to a <code>CreateDataSource</code> call. Change the settings that you want to change and make sure that all required fields have the appropriate values.</p> &lt;?oxy<em>insert</em>end&gt;</p>
    async fn create_data_source_from_redshift(
        &self,
        input: CreateDataSourceFromRedshiftInput,
    ) -> Result<CreateDataSourceFromRedshiftOutput, RusotoError<CreateDataSourceFromRedshiftError>>
    {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonML_20141212.CreateDataSourceFromRedshift",
        );
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
                .deserialize::<CreateDataSourceFromRedshiftOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSourceFromRedshiftError::from_response(response))
        }
    }

    /// <p>Creates a <code>DataSource</code> object. A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromS3</code> is an asynchronous operation. In response to <code>CreateDataSourceFromS3</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> has been created and is ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code> or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can't accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observation data used in a <code>DataSource</code> should be ready to use; that is, it should have a consistent structure, and missing data values should be kept to a minimum. The observation data must reside in one or more .csv files in an Amazon Simple Storage Service (Amazon S3) location, along with a schema that describes the data items by name and type. The same schema must be used for all of the data files referenced by the <code>DataSource</code>. </p> <p>After the <code>DataSource</code> has been created, it's ready to use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also needs a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p>
    async fn create_data_source_from_s3(
        &self,
        input: CreateDataSourceFromS3Input,
    ) -> Result<CreateDataSourceFromS3Output, RusotoError<CreateDataSourceFromS3Error>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateDataSourceFromS3");
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
                .deserialize::<CreateDataSourceFromS3Output, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataSourceFromS3Error::from_response(response))
        }
    }

    /// <p>Creates a new <code>Evaluation</code> of an <code>MLModel</code>. An <code>MLModel</code> is evaluated on a set of observations associated to a <code>DataSource</code>. Like a <code>DataSource</code> for an <code>MLModel</code>, the <code>DataSource</code> for an <code>Evaluation</code> contains values for the <code>Target Variable</code>. The <code>Evaluation</code> compares the predicted result for each observation to the actual outcome and provides a summary so that you know how effective the <code>MLModel</code> functions on the test data. Evaluation generates a relevant performance metric, such as BinaryAUC, RegressionRMSE or MulticlassAvgFScore based on the corresponding <code>MLModelType</code>: <code>BINARY</code>, <code>REGRESSION</code> or <code>MULTICLASS</code>. </p> <p><code>CreateEvaluation</code> is an asynchronous operation. In response to <code>CreateEvaluation</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the evaluation status to <code>PENDING</code>. After the <code>Evaluation</code> is created and ready for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetEvaluation</code> operation to check progress of the evaluation during the creation operation.</p>
    async fn create_evaluation(
        &self,
        input: CreateEvaluationInput,
    ) -> Result<CreateEvaluationOutput, RusotoError<CreateEvaluationError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateEvaluationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEvaluationError::from_response(response))
        }
    }

    /// <p>Creates a new <code>MLModel</code> using the <code>DataSource</code> and the recipe as information sources. </p> <p>An <code>MLModel</code> is nearly immutable. Users can update only the <code>MLModelName</code> and the <code>ScoreThreshold</code> in an <code>MLModel</code> without creating a new <code>MLModel</code>. </p> <p><code>CreateMLModel</code> is an asynchronous operation. In response to <code>CreateMLModel</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>MLModel</code> status to <code>PENDING</code>. After the <code>MLModel</code> has been created and ready is for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetMLModel</code> operation to check the progress of the <code>MLModel</code> during the creation operation.</p> <p> <code>CreateMLModel</code> requires a <code>DataSource</code> with computed statistics, which can be created by setting <code>ComputeStatistics</code> to <code>true</code> in <code>CreateDataSourceFromRDS</code>, <code>CreateDataSourceFromS3</code>, or <code>CreateDataSourceFromRedshift</code> operations. </p>
    async fn create_ml_model(
        &self,
        input: CreateMLModelInput,
    ) -> Result<CreateMLModelOutput, RusotoError<CreateMLModelError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateMLModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMLModelError::from_response(response))
        }
    }

    /// <p>Creates a real-time endpoint for the <code>MLModel</code>. The endpoint contains the URI of the <code>MLModel</code>; that is, the location to send real-time prediction requests for the specified <code>MLModel</code>.</p>
    async fn create_realtime_endpoint(
        &self,
        input: CreateRealtimeEndpointInput,
    ) -> Result<CreateRealtimeEndpointOutput, RusotoError<CreateRealtimeEndpointError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateRealtimeEndpoint");
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
                .deserialize::<CreateRealtimeEndpointOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRealtimeEndpointError::from_response(response))
        }
    }

    /// <p>Assigns the DELETED status to a <code>BatchPrediction</code>, rendering it unusable.</p> <p>After using the <code>DeleteBatchPrediction</code> operation, you can use the <a>GetBatchPrediction</a> operation to verify that the status of the <code>BatchPrediction</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteBatchPrediction</code> operation is irreversible.</p>
    async fn delete_batch_prediction(
        &self,
        input: DeleteBatchPredictionInput,
    ) -> Result<DeleteBatchPredictionOutput, RusotoError<DeleteBatchPredictionError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteBatchPrediction");
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
                .deserialize::<DeleteBatchPredictionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBatchPredictionError::from_response(response))
        }
    }

    /// <p>Assigns the DELETED status to a <code>DataSource</code>, rendering it unusable.</p> <p>After using the <code>DeleteDataSource</code> operation, you can use the <a>GetDataSource</a> operation to verify that the status of the <code>DataSource</code> changed to DELETED.</p> <p><b>Caution:</b> The results of the <code>DeleteDataSource</code> operation are irreversible.</p>
    async fn delete_data_source(
        &self,
        input: DeleteDataSourceInput,
    ) -> Result<DeleteDataSourceOutput, RusotoError<DeleteDataSourceError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDataSourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataSourceError::from_response(response))
        }
    }

    /// <p><p>Assigns the <code>DELETED</code> status to an <code>Evaluation</code>, rendering it unusable.</p> <p>After invoking the <code>DeleteEvaluation</code> operation, you can use the <code>GetEvaluation</code> operation to verify that the status of the <code>Evaluation</code> changed to <code>DELETED</code>.</p> <caution><title>Caution</title> <p>The results of the <code>DeleteEvaluation</code> operation are irreversible.</p></caution></p>
    async fn delete_evaluation(
        &self,
        input: DeleteEvaluationInput,
    ) -> Result<DeleteEvaluationOutput, RusotoError<DeleteEvaluationError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteEvaluationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEvaluationError::from_response(response))
        }
    }

    /// <p>Assigns the <code>DELETED</code> status to an <code>MLModel</code>, rendering it unusable.</p> <p>After using the <code>DeleteMLModel</code> operation, you can use the <code>GetMLModel</code> operation to verify that the status of the <code>MLModel</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteMLModel</code> operation is irreversible.</p>
    async fn delete_ml_model(
        &self,
        input: DeleteMLModelInput,
    ) -> Result<DeleteMLModelOutput, RusotoError<DeleteMLModelError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteMLModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMLModelError::from_response(response))
        }
    }

    /// <p>Deletes a real time endpoint of an <code>MLModel</code>.</p>
    async fn delete_realtime_endpoint(
        &self,
        input: DeleteRealtimeEndpointInput,
    ) -> Result<DeleteRealtimeEndpointOutput, RusotoError<DeleteRealtimeEndpointError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteRealtimeEndpoint");
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
                .deserialize::<DeleteRealtimeEndpointOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteRealtimeEndpointError::from_response(response))
        }
    }

    /// <p>Deletes the specified tags associated with an ML object. After this operation is complete, you can't recover deleted tags.</p> <p>If you specify a tag that doesn't exist, Amazon ML ignores it.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> Result<DeleteTagsOutput, RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>BatchPrediction</code> operations that match the search criteria in the request.</p>
    async fn describe_batch_predictions(
        &self,
        input: DescribeBatchPredictionsInput,
    ) -> Result<DescribeBatchPredictionsOutput, RusotoError<DescribeBatchPredictionsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeBatchPredictions");
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
                .deserialize::<DescribeBatchPredictionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeBatchPredictionsError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>DataSource</code> that match the search criteria in the request.</p>
    async fn describe_data_sources(
        &self,
        input: DescribeDataSourcesInput,
    ) -> Result<DescribeDataSourcesOutput, RusotoError<DescribeDataSourcesError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeDataSources");
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
                .deserialize::<DescribeDataSourcesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDataSourcesError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>DescribeEvaluations</code> that match the search criteria in the request.</p>
    async fn describe_evaluations(
        &self,
        input: DescribeEvaluationsInput,
    ) -> Result<DescribeEvaluationsOutput, RusotoError<DescribeEvaluationsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeEvaluations");
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
                .deserialize::<DescribeEvaluationsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEvaluationsError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>MLModel</code> that match the search criteria in the request.</p>
    async fn describe_ml_models(
        &self,
        input: DescribeMLModelsInput,
    ) -> Result<DescribeMLModelsOutput, RusotoError<DescribeMLModelsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeMLModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeMLModelsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMLModelsError::from_response(response))
        }
    }

    /// <p>Describes one or more of the tags for your Amazon ML object.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> Result<DescribeTagsOutput, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTagsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTagsError::from_response(response))
        }
    }

    /// <p>Returns a <code>BatchPrediction</code> that includes detailed metadata, status, and data file information for a <code>Batch Prediction</code> request.</p>
    async fn get_batch_prediction(
        &self,
        input: GetBatchPredictionInput,
    ) -> Result<GetBatchPredictionOutput, RusotoError<GetBatchPredictionError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetBatchPrediction");
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
                .deserialize::<GetBatchPredictionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetBatchPredictionError::from_response(response))
        }
    }

    /// <p>Returns a <code>DataSource</code> that includes metadata and data file information, as well as the current status of the <code>DataSource</code>.</p> <p><code>GetDataSource</code> provides results in normal or verbose format. The verbose format adds the schema description and the list of files pointed to by the DataSource to the normal format.</p>
    async fn get_data_source(
        &self,
        input: GetDataSourceInput,
    ) -> Result<GetDataSourceOutput, RusotoError<GetDataSourceError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDataSourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataSourceError::from_response(response))
        }
    }

    /// <p>Returns an <code>Evaluation</code> that includes metadata as well as the current status of the <code>Evaluation</code>.</p>
    async fn get_evaluation(
        &self,
        input: GetEvaluationInput,
    ) -> Result<GetEvaluationOutput, RusotoError<GetEvaluationError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetEvaluationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetEvaluationError::from_response(response))
        }
    }

    /// <p>Returns an <code>MLModel</code> that includes detailed metadata, data source information, and the current status of the <code>MLModel</code>.</p> <p><code>GetMLModel</code> provides results in normal or verbose format. </p>
    async fn get_ml_model(
        &self,
        input: GetMLModelInput,
    ) -> Result<GetMLModelOutput, RusotoError<GetMLModelError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetMLModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetMLModelError::from_response(response))
        }
    }

    /// <p><p>Generates a prediction for the observation using the specified <code>ML Model</code>.</p> <note><title>Note</title> <p>Not all response parameters will be populated. Whether a response parameter is populated depends on the type of model requested.</p></note></p>
    async fn predict(
        &self,
        input: PredictInput,
    ) -> Result<PredictOutput, RusotoError<PredictError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.Predict");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PredictOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PredictError::from_response(response))
        }
    }

    /// <p>Updates the <code>BatchPredictionName</code> of a <code>BatchPrediction</code>.</p> <p>You can use the <code>GetBatchPrediction</code> operation to view the contents of the updated data element.</p>
    async fn update_batch_prediction(
        &self,
        input: UpdateBatchPredictionInput,
    ) -> Result<UpdateBatchPredictionOutput, RusotoError<UpdateBatchPredictionError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateBatchPrediction");
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
                .deserialize::<UpdateBatchPredictionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBatchPredictionError::from_response(response))
        }
    }

    /// <p>Updates the <code>DataSourceName</code> of a <code>DataSource</code>.</p> <p>You can use the <code>GetDataSource</code> operation to view the contents of the updated data element.</p>
    async fn update_data_source(
        &self,
        input: UpdateDataSourceInput,
    ) -> Result<UpdateDataSourceOutput, RusotoError<UpdateDataSourceError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateDataSourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataSourceError::from_response(response))
        }
    }

    /// <p>Updates the <code>EvaluationName</code> of an <code>Evaluation</code>.</p> <p>You can use the <code>GetEvaluation</code> operation to view the contents of the updated data element.</p>
    async fn update_evaluation(
        &self,
        input: UpdateEvaluationInput,
    ) -> Result<UpdateEvaluationOutput, RusotoError<UpdateEvaluationError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateEvaluationOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEvaluationError::from_response(response))
        }
    }

    /// <p>Updates the <code>MLModelName</code> and the <code>ScoreThreshold</code> of an <code>MLModel</code>.</p> <p>You can use the <code>GetMLModel</code> operation to view the contents of the updated data element.</p>
    async fn update_ml_model(
        &self,
        input: UpdateMLModelInput,
    ) -> Result<UpdateMLModelOutput, RusotoError<UpdateMLModelError>> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateMLModelOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMLModelError::from_response(response))
        }
    }
}
