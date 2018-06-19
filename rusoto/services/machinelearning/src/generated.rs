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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateBatchPredictionOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>. This value is identical to the value of the <code>BatchPredictionId</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateDataSourceFromRDSOutput {
    /// <p>A user-supplied ID that uniquely identifies the datasource. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateDataSourceFromRedshiftOutput {
    /// <p>A user-supplied ID that uniquely identifies the datasource. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateDataSourceFromS3Output {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. This value should be identical to the value of the <code>DataSourceID</code> in the request. </p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateEvaluationOutput {
    /// <p>The user-supplied ID that uniquely identifies the <code>Evaluation</code>. This value should be identical to the value of the <code>EvaluationId</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateMLModelOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelId</code> in the request. </p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRealtimeEndpointInput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p><p>Represents the output of an <code>CreateRealtimeEndpoint</code> operation.</p> <p>The result contains the <code>MLModelId</code> and the endpoint information for the <code>MLModel</code>.</p> <note> <p>The endpoint information includes the URI of the <code>MLModel</code>; that is, the location to send online prediction requests for the specified <code>MLModel</code>.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DeleteBatchPredictionInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
}

/// <p> Represents the output of a <code>DeleteBatchPrediction</code> operation.</p> <p>You can use the <code>GetBatchPrediction</code> operation and check the value of the <code>Status</code> parameter to see whether a <code>BatchPrediction</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBatchPredictionOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>. This value should be identical to the value of the <code>BatchPredictionID</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDataSourceInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>.</p>
    #[serde(rename = "DataSourceId")]
    pub data_source_id: String,
}

/// <p> Represents the output of a <code>DeleteDataSource</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDataSourceOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>DataSource</code>. This value should be identical to the value of the <code>DataSourceID</code> in the request.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEvaluationInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code> to delete.</p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
}

/// <p> Represents the output of a <code>DeleteEvaluation</code> operation. The output indicates that Amazon Machine Learning (Amazon ML) received the request.</p> <p>You can use the <code>GetEvaluation</code> operation and check the value of the <code>Status</code> parameter to see whether an <code>Evaluation</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteEvaluationOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>. This value should be identical to the value of the <code>EvaluationId</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMLModelInput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p>Represents the output of a <code>DeleteMLModel</code> operation.</p> <p>You can use the <code>GetMLModel</code> operation and check the value of the <code>Status</code> parameter to see whether an <code>MLModel</code> is marked as <code>DELETED</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteMLModelOutput {
    /// <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelID</code> in the request.</p>
    #[serde(rename = "MLModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_model_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRealtimeEndpointInput {
    /// <p>The ID assigned to the <code>MLModel</code> during creation.</p>
    #[serde(rename = "MLModelId")]
    pub ml_model_id: String,
}

/// <p>Represents the output of an <code>DeleteRealtimeEndpoint</code> operation.</p> <p>The result contains the <code>MLModelId</code> and the endpoint information for the <code>MLModel</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetBatchPredictionInput {
    /// <p>An ID assigned to the <code>BatchPrediction</code> at creation.</p>
    #[serde(rename = "BatchPredictionId")]
    pub batch_prediction_id: String,
}

/// <p>Represents the output of a <code>GetBatchPrediction</code> operation and describes a <code>BatchPrediction</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct GetEvaluationInput {
    /// <p>The ID of the <code>Evaluation</code> to retrieve. The evaluation of each <code>MLModel</code> is recorded and cataloged. The ID provides the means to access the information. </p>
    #[serde(rename = "EvaluationId")]
    pub evaluation_id: String,
}

/// <p>Represents the output of a <code>GetEvaluation</code> operation and describes an <code>Evaluation</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct PerformanceMetrics {
    #[serde(rename = "Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct PredictOutput {
    #[serde(rename = "Prediction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
}

/// <p><p>The output from a <code>Predict</code> operation: </p> <ul> <li> <p> <code>Details</code> - Contains the following attributes: <code>DetailsAttributes.PREDICTIVE<em>MODEL</em>TYPE - REGRESSION | BINARY | MULTICLASS</code> <code>DetailsAttributes.ALGORITHM - SGD</code> </p> </li> <li> <p> <code>PredictedLabel</code> - Present for either a <code>BINARY</code> or <code>MULTICLASS</code> <code>MLModel</code> request. </p> </li> <li> <p> <code>PredictedScores</code> - Contains the raw classification score corresponding to each label. </p> </li> <li> <p> <code>PredictedValue</code> - Present for a <code>REGRESSION</code> <code>MLModel</code> request. </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct RDSDatabaseCredentials {
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The datasource details that are specific to Amazon RDS.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct RedshiftDatabaseCredentials {
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes the <code>DataSource</code> details specific to Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct UpdateBatchPredictionOutput {
    /// <p>The ID assigned to the <code>BatchPrediction</code> during creation. This value should be identical to the value of the <code>BatchPredictionId</code> in the request.</p>
    #[serde(rename = "BatchPredictionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prediction_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateDataSourceOutput {
    /// <p>The ID assigned to the <code>DataSource</code> during creation. This value should be identical to the value of the <code>DataSourceID</code> in the request.</p>
    #[serde(rename = "DataSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateEvaluationOutput {
    /// <p>The ID assigned to the <code>Evaluation</code> during creation. This value should be identical to the value of the <code>Evaluation</code> in the request.</p>
    #[serde(rename = "EvaluationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
                    "InternalServerException" => {
                        AddTagsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        AddTagsError::InvalidInput(String::from(error_message))
                    }
                    "InvalidTagException" => AddTagsError::InvalidTag(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        AddTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "TagLimitExceededException" => {
                        AddTagsError::TagLimitExceeded(String::from(error_message))
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
            AddTagsError::InternalServer(ref cause) => cause,
            AddTagsError::InvalidInput(ref cause) => cause,
            AddTagsError::InvalidTag(ref cause) => cause,
            AddTagsError::ResourceNotFound(ref cause) => cause,
            AddTagsError::TagLimitExceeded(ref cause) => cause,
            AddTagsError::Validation(ref cause) => cause,
            AddTagsError::Credentials(ref err) => err.description(),
            AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBatchPrediction
#[derive(Debug, PartialEq)]
pub enum CreateBatchPredictionError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBatchPredictionError {
    pub fn from_body(body: &str) -> CreateBatchPredictionError {
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
                    "IdempotentParameterMismatchException" => {
                        CreateBatchPredictionError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerException" => {
                        CreateBatchPredictionError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateBatchPredictionError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBatchPredictionError::Validation(error_message.to_string())
                    }
                    _ => CreateBatchPredictionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBatchPredictionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBatchPredictionError {
    fn from(err: serde_json::error::Error) -> CreateBatchPredictionError {
        CreateBatchPredictionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBatchPredictionError {
    fn from(err: CredentialsError) -> CreateBatchPredictionError {
        CreateBatchPredictionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBatchPredictionError {
    fn from(err: HttpDispatchError) -> CreateBatchPredictionError {
        CreateBatchPredictionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBatchPredictionError {
    fn from(err: io::Error) -> CreateBatchPredictionError {
        CreateBatchPredictionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBatchPredictionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBatchPredictionError {
    fn description(&self) -> &str {
        match *self {
            CreateBatchPredictionError::IdempotentParameterMismatch(ref cause) => cause,
            CreateBatchPredictionError::InternalServer(ref cause) => cause,
            CreateBatchPredictionError::InvalidInput(ref cause) => cause,
            CreateBatchPredictionError::Validation(ref cause) => cause,
            CreateBatchPredictionError::Credentials(ref err) => err.description(),
            CreateBatchPredictionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateBatchPredictionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDataSourceFromRDS
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromRDSError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDataSourceFromRDSError {
    pub fn from_body(body: &str) -> CreateDataSourceFromRDSError {
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
                    "IdempotentParameterMismatchException" => {
                        CreateDataSourceFromRDSError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerException" => {
                        CreateDataSourceFromRDSError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDataSourceFromRDSError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDataSourceFromRDSError::Validation(error_message.to_string())
                    }
                    _ => CreateDataSourceFromRDSError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDataSourceFromRDSError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDataSourceFromRDSError {
    fn from(err: serde_json::error::Error) -> CreateDataSourceFromRDSError {
        CreateDataSourceFromRDSError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDataSourceFromRDSError {
    fn from(err: CredentialsError) -> CreateDataSourceFromRDSError {
        CreateDataSourceFromRDSError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDataSourceFromRDSError {
    fn from(err: HttpDispatchError) -> CreateDataSourceFromRDSError {
        CreateDataSourceFromRDSError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDataSourceFromRDSError {
    fn from(err: io::Error) -> CreateDataSourceFromRDSError {
        CreateDataSourceFromRDSError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDataSourceFromRDSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDataSourceFromRDSError {
    fn description(&self) -> &str {
        match *self {
            CreateDataSourceFromRDSError::IdempotentParameterMismatch(ref cause) => cause,
            CreateDataSourceFromRDSError::InternalServer(ref cause) => cause,
            CreateDataSourceFromRDSError::InvalidInput(ref cause) => cause,
            CreateDataSourceFromRDSError::Validation(ref cause) => cause,
            CreateDataSourceFromRDSError::Credentials(ref err) => err.description(),
            CreateDataSourceFromRDSError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDataSourceFromRDSError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDataSourceFromRedshift
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromRedshiftError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDataSourceFromRedshiftError {
    pub fn from_body(body: &str) -> CreateDataSourceFromRedshiftError {
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
                    "IdempotentParameterMismatchException" => {
                        CreateDataSourceFromRedshiftError::IdempotentParameterMismatch(
                            String::from(error_message),
                        )
                    }
                    "InternalServerException" => CreateDataSourceFromRedshiftError::InternalServer(
                        String::from(error_message),
                    ),
                    "InvalidInputException" => {
                        CreateDataSourceFromRedshiftError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDataSourceFromRedshiftError::Validation(error_message.to_string())
                    }
                    _ => CreateDataSourceFromRedshiftError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDataSourceFromRedshiftError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDataSourceFromRedshiftError {
    fn from(err: serde_json::error::Error) -> CreateDataSourceFromRedshiftError {
        CreateDataSourceFromRedshiftError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDataSourceFromRedshiftError {
    fn from(err: CredentialsError) -> CreateDataSourceFromRedshiftError {
        CreateDataSourceFromRedshiftError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDataSourceFromRedshiftError {
    fn from(err: HttpDispatchError) -> CreateDataSourceFromRedshiftError {
        CreateDataSourceFromRedshiftError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDataSourceFromRedshiftError {
    fn from(err: io::Error) -> CreateDataSourceFromRedshiftError {
        CreateDataSourceFromRedshiftError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDataSourceFromRedshiftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDataSourceFromRedshiftError {
    fn description(&self) -> &str {
        match *self {
            CreateDataSourceFromRedshiftError::IdempotentParameterMismatch(ref cause) => cause,
            CreateDataSourceFromRedshiftError::InternalServer(ref cause) => cause,
            CreateDataSourceFromRedshiftError::InvalidInput(ref cause) => cause,
            CreateDataSourceFromRedshiftError::Validation(ref cause) => cause,
            CreateDataSourceFromRedshiftError::Credentials(ref err) => err.description(),
            CreateDataSourceFromRedshiftError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDataSourceFromRedshiftError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDataSourceFromS3
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceFromS3Error {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDataSourceFromS3Error {
    pub fn from_body(body: &str) -> CreateDataSourceFromS3Error {
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
                    "IdempotentParameterMismatchException" => {
                        CreateDataSourceFromS3Error::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerException" => {
                        CreateDataSourceFromS3Error::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateDataSourceFromS3Error::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDataSourceFromS3Error::Validation(error_message.to_string())
                    }
                    _ => CreateDataSourceFromS3Error::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDataSourceFromS3Error::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDataSourceFromS3Error {
    fn from(err: serde_json::error::Error) -> CreateDataSourceFromS3Error {
        CreateDataSourceFromS3Error::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDataSourceFromS3Error {
    fn from(err: CredentialsError) -> CreateDataSourceFromS3Error {
        CreateDataSourceFromS3Error::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDataSourceFromS3Error {
    fn from(err: HttpDispatchError) -> CreateDataSourceFromS3Error {
        CreateDataSourceFromS3Error::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDataSourceFromS3Error {
    fn from(err: io::Error) -> CreateDataSourceFromS3Error {
        CreateDataSourceFromS3Error::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDataSourceFromS3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDataSourceFromS3Error {
    fn description(&self) -> &str {
        match *self {
            CreateDataSourceFromS3Error::IdempotentParameterMismatch(ref cause) => cause,
            CreateDataSourceFromS3Error::InternalServer(ref cause) => cause,
            CreateDataSourceFromS3Error::InvalidInput(ref cause) => cause,
            CreateDataSourceFromS3Error::Validation(ref cause) => cause,
            CreateDataSourceFromS3Error::Credentials(ref err) => err.description(),
            CreateDataSourceFromS3Error::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDataSourceFromS3Error::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEvaluation
#[derive(Debug, PartialEq)]
pub enum CreateEvaluationError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEvaluationError {
    pub fn from_body(body: &str) -> CreateEvaluationError {
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
                    "IdempotentParameterMismatchException" => {
                        CreateEvaluationError::IdempotentParameterMismatch(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerException" => {
                        CreateEvaluationError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateEvaluationError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateEvaluationError::Validation(error_message.to_string())
                    }
                    _ => CreateEvaluationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEvaluationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEvaluationError {
    fn from(err: serde_json::error::Error) -> CreateEvaluationError {
        CreateEvaluationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEvaluationError {
    fn from(err: CredentialsError) -> CreateEvaluationError {
        CreateEvaluationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEvaluationError {
    fn from(err: HttpDispatchError) -> CreateEvaluationError {
        CreateEvaluationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEvaluationError {
    fn from(err: io::Error) -> CreateEvaluationError {
        CreateEvaluationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEvaluationError {
    fn description(&self) -> &str {
        match *self {
            CreateEvaluationError::IdempotentParameterMismatch(ref cause) => cause,
            CreateEvaluationError::InternalServer(ref cause) => cause,
            CreateEvaluationError::InvalidInput(ref cause) => cause,
            CreateEvaluationError::Validation(ref cause) => cause,
            CreateEvaluationError::Credentials(ref err) => err.description(),
            CreateEvaluationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateEvaluationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateMLModel
#[derive(Debug, PartialEq)]
pub enum CreateMLModelError {
    /// <p>A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original request.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateMLModelError {
    pub fn from_body(body: &str) -> CreateMLModelError {
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
                    "IdempotentParameterMismatchException" => {
                        CreateMLModelError::IdempotentParameterMismatch(String::from(error_message))
                    }
                    "InternalServerException" => {
                        CreateMLModelError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateMLModelError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateMLModelError::Validation(error_message.to_string())
                    }
                    _ => CreateMLModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateMLModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateMLModelError {
    fn from(err: serde_json::error::Error) -> CreateMLModelError {
        CreateMLModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateMLModelError {
    fn from(err: CredentialsError) -> CreateMLModelError {
        CreateMLModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateMLModelError {
    fn from(err: HttpDispatchError) -> CreateMLModelError {
        CreateMLModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateMLModelError {
    fn from(err: io::Error) -> CreateMLModelError {
        CreateMLModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateMLModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateMLModelError {
    fn description(&self) -> &str {
        match *self {
            CreateMLModelError::IdempotentParameterMismatch(ref cause) => cause,
            CreateMLModelError::InternalServer(ref cause) => cause,
            CreateMLModelError::InvalidInput(ref cause) => cause,
            CreateMLModelError::Validation(ref cause) => cause,
            CreateMLModelError::Credentials(ref err) => err.description(),
            CreateMLModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateMLModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRealtimeEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateRealtimeEndpointError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRealtimeEndpointError {
    pub fn from_body(body: &str) -> CreateRealtimeEndpointError {
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
                    "InternalServerException" => {
                        CreateRealtimeEndpointError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateRealtimeEndpointError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateRealtimeEndpointError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRealtimeEndpointError::Validation(error_message.to_string())
                    }
                    _ => CreateRealtimeEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRealtimeEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRealtimeEndpointError {
    fn from(err: serde_json::error::Error) -> CreateRealtimeEndpointError {
        CreateRealtimeEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRealtimeEndpointError {
    fn from(err: CredentialsError) -> CreateRealtimeEndpointError {
        CreateRealtimeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRealtimeEndpointError {
    fn from(err: HttpDispatchError) -> CreateRealtimeEndpointError {
        CreateRealtimeEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRealtimeEndpointError {
    fn from(err: io::Error) -> CreateRealtimeEndpointError {
        CreateRealtimeEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRealtimeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRealtimeEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateRealtimeEndpointError::InternalServer(ref cause) => cause,
            CreateRealtimeEndpointError::InvalidInput(ref cause) => cause,
            CreateRealtimeEndpointError::ResourceNotFound(ref cause) => cause,
            CreateRealtimeEndpointError::Validation(ref cause) => cause,
            CreateRealtimeEndpointError::Credentials(ref err) => err.description(),
            CreateRealtimeEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateRealtimeEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBatchPrediction
#[derive(Debug, PartialEq)]
pub enum DeleteBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBatchPredictionError {
    pub fn from_body(body: &str) -> DeleteBatchPredictionError {
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
                    "InternalServerException" => {
                        DeleteBatchPredictionError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteBatchPredictionError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteBatchPredictionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBatchPredictionError::Validation(error_message.to_string())
                    }
                    _ => DeleteBatchPredictionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBatchPredictionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBatchPredictionError {
    fn from(err: serde_json::error::Error) -> DeleteBatchPredictionError {
        DeleteBatchPredictionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBatchPredictionError {
    fn from(err: CredentialsError) -> DeleteBatchPredictionError {
        DeleteBatchPredictionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBatchPredictionError {
    fn from(err: HttpDispatchError) -> DeleteBatchPredictionError {
        DeleteBatchPredictionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBatchPredictionError {
    fn from(err: io::Error) -> DeleteBatchPredictionError {
        DeleteBatchPredictionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBatchPredictionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBatchPredictionError {
    fn description(&self) -> &str {
        match *self {
            DeleteBatchPredictionError::InternalServer(ref cause) => cause,
            DeleteBatchPredictionError::InvalidInput(ref cause) => cause,
            DeleteBatchPredictionError::ResourceNotFound(ref cause) => cause,
            DeleteBatchPredictionError::Validation(ref cause) => cause,
            DeleteBatchPredictionError::Credentials(ref err) => err.description(),
            DeleteBatchPredictionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBatchPredictionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDataSourceError {
    pub fn from_body(body: &str) -> DeleteDataSourceError {
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
                    "InternalServerException" => {
                        DeleteDataSourceError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteDataSourceError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDataSourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDataSourceError::Validation(error_message.to_string())
                    }
                    _ => DeleteDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDataSourceError {
    fn from(err: serde_json::error::Error) -> DeleteDataSourceError {
        DeleteDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDataSourceError {
    fn from(err: CredentialsError) -> DeleteDataSourceError {
        DeleteDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDataSourceError {
    fn from(err: HttpDispatchError) -> DeleteDataSourceError {
        DeleteDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDataSourceError {
    fn from(err: io::Error) -> DeleteDataSourceError {
        DeleteDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDataSourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteDataSourceError::InternalServer(ref cause) => cause,
            DeleteDataSourceError::InvalidInput(ref cause) => cause,
            DeleteDataSourceError::ResourceNotFound(ref cause) => cause,
            DeleteDataSourceError::Validation(ref cause) => cause,
            DeleteDataSourceError::Credentials(ref err) => err.description(),
            DeleteDataSourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEvaluation
#[derive(Debug, PartialEq)]
pub enum DeleteEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEvaluationError {
    pub fn from_body(body: &str) -> DeleteEvaluationError {
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
                    "InternalServerException" => {
                        DeleteEvaluationError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteEvaluationError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteEvaluationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEvaluationError::Validation(error_message.to_string())
                    }
                    _ => DeleteEvaluationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEvaluationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEvaluationError {
    fn from(err: serde_json::error::Error) -> DeleteEvaluationError {
        DeleteEvaluationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEvaluationError {
    fn from(err: CredentialsError) -> DeleteEvaluationError {
        DeleteEvaluationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEvaluationError {
    fn from(err: HttpDispatchError) -> DeleteEvaluationError {
        DeleteEvaluationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEvaluationError {
    fn from(err: io::Error) -> DeleteEvaluationError {
        DeleteEvaluationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEvaluationError {
    fn description(&self) -> &str {
        match *self {
            DeleteEvaluationError::InternalServer(ref cause) => cause,
            DeleteEvaluationError::InvalidInput(ref cause) => cause,
            DeleteEvaluationError::ResourceNotFound(ref cause) => cause,
            DeleteEvaluationError::Validation(ref cause) => cause,
            DeleteEvaluationError::Credentials(ref err) => err.description(),
            DeleteEvaluationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteEvaluationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMLModel
#[derive(Debug, PartialEq)]
pub enum DeleteMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteMLModelError {
    pub fn from_body(body: &str) -> DeleteMLModelError {
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
                    "InternalServerException" => {
                        DeleteMLModelError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteMLModelError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteMLModelError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteMLModelError::Validation(error_message.to_string())
                    }
                    _ => DeleteMLModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteMLModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteMLModelError {
    fn from(err: serde_json::error::Error) -> DeleteMLModelError {
        DeleteMLModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteMLModelError {
    fn from(err: CredentialsError) -> DeleteMLModelError {
        DeleteMLModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteMLModelError {
    fn from(err: HttpDispatchError) -> DeleteMLModelError {
        DeleteMLModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteMLModelError {
    fn from(err: io::Error) -> DeleteMLModelError {
        DeleteMLModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteMLModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMLModelError {
    fn description(&self) -> &str {
        match *self {
            DeleteMLModelError::InternalServer(ref cause) => cause,
            DeleteMLModelError::InvalidInput(ref cause) => cause,
            DeleteMLModelError::ResourceNotFound(ref cause) => cause,
            DeleteMLModelError::Validation(ref cause) => cause,
            DeleteMLModelError::Credentials(ref err) => err.description(),
            DeleteMLModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteMLModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRealtimeEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteRealtimeEndpointError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRealtimeEndpointError {
    pub fn from_body(body: &str) -> DeleteRealtimeEndpointError {
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
                    "InternalServerException" => {
                        DeleteRealtimeEndpointError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteRealtimeEndpointError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteRealtimeEndpointError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRealtimeEndpointError::Validation(error_message.to_string())
                    }
                    _ => DeleteRealtimeEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRealtimeEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRealtimeEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteRealtimeEndpointError {
        DeleteRealtimeEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRealtimeEndpointError {
    fn from(err: CredentialsError) -> DeleteRealtimeEndpointError {
        DeleteRealtimeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRealtimeEndpointError {
    fn from(err: HttpDispatchError) -> DeleteRealtimeEndpointError {
        DeleteRealtimeEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRealtimeEndpointError {
    fn from(err: io::Error) -> DeleteRealtimeEndpointError {
        DeleteRealtimeEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRealtimeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRealtimeEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteRealtimeEndpointError::InternalServer(ref cause) => cause,
            DeleteRealtimeEndpointError::InvalidInput(ref cause) => cause,
            DeleteRealtimeEndpointError::ResourceNotFound(ref cause) => cause,
            DeleteRealtimeEndpointError::Validation(ref cause) => cause,
            DeleteRealtimeEndpointError::Credentials(ref err) => err.description(),
            DeleteRealtimeEndpointError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRealtimeEndpointError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
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
                    "InternalServerException" => {
                        DeleteTagsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DeleteTagsError::InvalidInput(String::from(error_message))
                    }
                    "InvalidTagException" => {
                        DeleteTagsError::InvalidTag(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => DeleteTagsError::Validation(error_message.to_string()),
                    _ => DeleteTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagsError {
    fn from(err: serde_json::error::Error) -> DeleteTagsError {
        DeleteTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::InternalServer(ref cause) => cause,
            DeleteTagsError::InvalidInput(ref cause) => cause,
            DeleteTagsError::InvalidTag(ref cause) => cause,
            DeleteTagsError::ResourceNotFound(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBatchPredictions
#[derive(Debug, PartialEq)]
pub enum DescribeBatchPredictionsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBatchPredictionsError {
    pub fn from_body(body: &str) -> DescribeBatchPredictionsError {
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
                    "InternalServerException" => {
                        DescribeBatchPredictionsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeBatchPredictionsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBatchPredictionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeBatchPredictionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBatchPredictionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBatchPredictionsError {
    fn from(err: serde_json::error::Error) -> DescribeBatchPredictionsError {
        DescribeBatchPredictionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBatchPredictionsError {
    fn from(err: CredentialsError) -> DescribeBatchPredictionsError {
        DescribeBatchPredictionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBatchPredictionsError {
    fn from(err: HttpDispatchError) -> DescribeBatchPredictionsError {
        DescribeBatchPredictionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBatchPredictionsError {
    fn from(err: io::Error) -> DescribeBatchPredictionsError {
        DescribeBatchPredictionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBatchPredictionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBatchPredictionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBatchPredictionsError::InternalServer(ref cause) => cause,
            DescribeBatchPredictionsError::InvalidInput(ref cause) => cause,
            DescribeBatchPredictionsError::Validation(ref cause) => cause,
            DescribeBatchPredictionsError::Credentials(ref err) => err.description(),
            DescribeBatchPredictionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeBatchPredictionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDataSources
#[derive(Debug, PartialEq)]
pub enum DescribeDataSourcesError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDataSourcesError {
    pub fn from_body(body: &str) -> DescribeDataSourcesError {
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
                    "InternalServerException" => {
                        DescribeDataSourcesError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeDataSourcesError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDataSourcesError::Validation(error_message.to_string())
                    }
                    _ => DescribeDataSourcesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDataSourcesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDataSourcesError {
    fn from(err: serde_json::error::Error) -> DescribeDataSourcesError {
        DescribeDataSourcesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDataSourcesError {
    fn from(err: CredentialsError) -> DescribeDataSourcesError {
        DescribeDataSourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDataSourcesError {
    fn from(err: HttpDispatchError) -> DescribeDataSourcesError {
        DescribeDataSourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDataSourcesError {
    fn from(err: io::Error) -> DescribeDataSourcesError {
        DescribeDataSourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDataSourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDataSourcesError {
    fn description(&self) -> &str {
        match *self {
            DescribeDataSourcesError::InternalServer(ref cause) => cause,
            DescribeDataSourcesError::InvalidInput(ref cause) => cause,
            DescribeDataSourcesError::Validation(ref cause) => cause,
            DescribeDataSourcesError::Credentials(ref err) => err.description(),
            DescribeDataSourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDataSourcesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvaluations
#[derive(Debug, PartialEq)]
pub enum DescribeEvaluationsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEvaluationsError {
    pub fn from_body(body: &str) -> DescribeEvaluationsError {
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
                    "InternalServerException" => {
                        DescribeEvaluationsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeEvaluationsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEvaluationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEvaluationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEvaluationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEvaluationsError {
    fn from(err: serde_json::error::Error) -> DescribeEvaluationsError {
        DescribeEvaluationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEvaluationsError {
    fn from(err: CredentialsError) -> DescribeEvaluationsError {
        DescribeEvaluationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEvaluationsError {
    fn from(err: HttpDispatchError) -> DescribeEvaluationsError {
        DescribeEvaluationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEvaluationsError {
    fn from(err: io::Error) -> DescribeEvaluationsError {
        DescribeEvaluationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEvaluationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEvaluationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEvaluationsError::InternalServer(ref cause) => cause,
            DescribeEvaluationsError::InvalidInput(ref cause) => cause,
            DescribeEvaluationsError::Validation(ref cause) => cause,
            DescribeEvaluationsError::Credentials(ref err) => err.description(),
            DescribeEvaluationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEvaluationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeMLModels
#[derive(Debug, PartialEq)]
pub enum DescribeMLModelsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeMLModelsError {
    pub fn from_body(body: &str) -> DescribeMLModelsError {
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
                    "InternalServerException" => {
                        DescribeMLModelsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeMLModelsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeMLModelsError::Validation(error_message.to_string())
                    }
                    _ => DescribeMLModelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeMLModelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeMLModelsError {
    fn from(err: serde_json::error::Error) -> DescribeMLModelsError {
        DescribeMLModelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeMLModelsError {
    fn from(err: CredentialsError) -> DescribeMLModelsError {
        DescribeMLModelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeMLModelsError {
    fn from(err: HttpDispatchError) -> DescribeMLModelsError {
        DescribeMLModelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeMLModelsError {
    fn from(err: io::Error) -> DescribeMLModelsError {
        DescribeMLModelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeMLModelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeMLModelsError {
    fn description(&self) -> &str {
        match *self {
            DescribeMLModelsError::InternalServer(ref cause) => cause,
            DescribeMLModelsError::InvalidInput(ref cause) => cause,
            DescribeMLModelsError::Validation(ref cause) => cause,
            DescribeMLModelsError::Credentials(ref err) => err.description(),
            DescribeMLModelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeMLModelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
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
                    "InternalServerException" => {
                        DescribeTagsError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        DescribeTagsError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTagsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::InternalServer(ref cause) => cause,
            DescribeTagsError::InvalidInput(ref cause) => cause,
            DescribeTagsError::ResourceNotFound(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBatchPrediction
#[derive(Debug, PartialEq)]
pub enum GetBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetBatchPredictionError {
    pub fn from_body(body: &str) -> GetBatchPredictionError {
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
                    "InternalServerException" => {
                        GetBatchPredictionError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetBatchPredictionError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetBatchPredictionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetBatchPredictionError::Validation(error_message.to_string())
                    }
                    _ => GetBatchPredictionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBatchPredictionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBatchPredictionError {
    fn from(err: serde_json::error::Error) -> GetBatchPredictionError {
        GetBatchPredictionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBatchPredictionError {
    fn from(err: CredentialsError) -> GetBatchPredictionError {
        GetBatchPredictionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBatchPredictionError {
    fn from(err: HttpDispatchError) -> GetBatchPredictionError {
        GetBatchPredictionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBatchPredictionError {
    fn from(err: io::Error) -> GetBatchPredictionError {
        GetBatchPredictionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBatchPredictionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBatchPredictionError {
    fn description(&self) -> &str {
        match *self {
            GetBatchPredictionError::InternalServer(ref cause) => cause,
            GetBatchPredictionError::InvalidInput(ref cause) => cause,
            GetBatchPredictionError::ResourceNotFound(ref cause) => cause,
            GetBatchPredictionError::Validation(ref cause) => cause,
            GetBatchPredictionError::Credentials(ref err) => err.description(),
            GetBatchPredictionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBatchPredictionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataSource
#[derive(Debug, PartialEq)]
pub enum GetDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDataSourceError {
    pub fn from_body(body: &str) -> GetDataSourceError {
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
                    "InternalServerException" => {
                        GetDataSourceError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetDataSourceError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetDataSourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDataSourceError::Validation(error_message.to_string())
                    }
                    _ => GetDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDataSourceError {
    fn from(err: serde_json::error::Error) -> GetDataSourceError {
        GetDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDataSourceError {
    fn from(err: CredentialsError) -> GetDataSourceError {
        GetDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDataSourceError {
    fn from(err: HttpDispatchError) -> GetDataSourceError {
        GetDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDataSourceError {
    fn from(err: io::Error) -> GetDataSourceError {
        GetDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataSourceError {
    fn description(&self) -> &str {
        match *self {
            GetDataSourceError::InternalServer(ref cause) => cause,
            GetDataSourceError::InvalidInput(ref cause) => cause,
            GetDataSourceError::ResourceNotFound(ref cause) => cause,
            GetDataSourceError::Validation(ref cause) => cause,
            GetDataSourceError::Credentials(ref err) => err.description(),
            GetDataSourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEvaluation
#[derive(Debug, PartialEq)]
pub enum GetEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetEvaluationError {
    pub fn from_body(body: &str) -> GetEvaluationError {
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
                    "InternalServerException" => {
                        GetEvaluationError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetEvaluationError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetEvaluationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEvaluationError::Validation(error_message.to_string())
                    }
                    _ => GetEvaluationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEvaluationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEvaluationError {
    fn from(err: serde_json::error::Error) -> GetEvaluationError {
        GetEvaluationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEvaluationError {
    fn from(err: CredentialsError) -> GetEvaluationError {
        GetEvaluationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEvaluationError {
    fn from(err: HttpDispatchError) -> GetEvaluationError {
        GetEvaluationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEvaluationError {
    fn from(err: io::Error) -> GetEvaluationError {
        GetEvaluationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEvaluationError {
    fn description(&self) -> &str {
        match *self {
            GetEvaluationError::InternalServer(ref cause) => cause,
            GetEvaluationError::InvalidInput(ref cause) => cause,
            GetEvaluationError::ResourceNotFound(ref cause) => cause,
            GetEvaluationError::Validation(ref cause) => cause,
            GetEvaluationError::Credentials(ref err) => err.description(),
            GetEvaluationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetEvaluationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMLModel
#[derive(Debug, PartialEq)]
pub enum GetMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetMLModelError {
    pub fn from_body(body: &str) -> GetMLModelError {
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
                    "InternalServerException" => {
                        GetMLModelError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        GetMLModelError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetMLModelError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetMLModelError::Validation(error_message.to_string()),
                    _ => GetMLModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetMLModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetMLModelError {
    fn from(err: serde_json::error::Error) -> GetMLModelError {
        GetMLModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetMLModelError {
    fn from(err: CredentialsError) -> GetMLModelError {
        GetMLModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetMLModelError {
    fn from(err: HttpDispatchError) -> GetMLModelError {
        GetMLModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetMLModelError {
    fn from(err: io::Error) -> GetMLModelError {
        GetMLModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetMLModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMLModelError {
    fn description(&self) -> &str {
        match *self {
            GetMLModelError::InternalServer(ref cause) => cause,
            GetMLModelError::InvalidInput(ref cause) => cause,
            GetMLModelError::ResourceNotFound(ref cause) => cause,
            GetMLModelError::Validation(ref cause) => cause,
            GetMLModelError::Credentials(ref err) => err.description(),
            GetMLModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetMLModelError::Unknown(ref cause) => cause,
        }
    }
}
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PredictError {
    pub fn from_body(body: &str) -> PredictError {
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
                    "InternalServerException" => {
                        PredictError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        PredictError::InvalidInput(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PredictError::LimitExceeded(String::from(error_message))
                    }
                    "PredictorNotMountedException" => {
                        PredictError::PredictorNotMounted(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PredictError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PredictError::Validation(error_message.to_string()),
                    _ => PredictError::Unknown(String::from(body)),
                }
            }
            Err(_) => PredictError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PredictError {
    fn from(err: serde_json::error::Error) -> PredictError {
        PredictError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PredictError {
    fn from(err: CredentialsError) -> PredictError {
        PredictError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PredictError {
    fn from(err: HttpDispatchError) -> PredictError {
        PredictError::HttpDispatch(err)
    }
}
impl From<io::Error> for PredictError {
    fn from(err: io::Error) -> PredictError {
        PredictError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PredictError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PredictError {
    fn description(&self) -> &str {
        match *self {
            PredictError::InternalServer(ref cause) => cause,
            PredictError::InvalidInput(ref cause) => cause,
            PredictError::LimitExceeded(ref cause) => cause,
            PredictError::PredictorNotMounted(ref cause) => cause,
            PredictError::ResourceNotFound(ref cause) => cause,
            PredictError::Validation(ref cause) => cause,
            PredictError::Credentials(ref err) => err.description(),
            PredictError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PredictError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBatchPrediction
#[derive(Debug, PartialEq)]
pub enum UpdateBatchPredictionError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateBatchPredictionError {
    pub fn from_body(body: &str) -> UpdateBatchPredictionError {
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
                    "InternalServerException" => {
                        UpdateBatchPredictionError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateBatchPredictionError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateBatchPredictionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateBatchPredictionError::Validation(error_message.to_string())
                    }
                    _ => UpdateBatchPredictionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateBatchPredictionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateBatchPredictionError {
    fn from(err: serde_json::error::Error) -> UpdateBatchPredictionError {
        UpdateBatchPredictionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateBatchPredictionError {
    fn from(err: CredentialsError) -> UpdateBatchPredictionError {
        UpdateBatchPredictionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateBatchPredictionError {
    fn from(err: HttpDispatchError) -> UpdateBatchPredictionError {
        UpdateBatchPredictionError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateBatchPredictionError {
    fn from(err: io::Error) -> UpdateBatchPredictionError {
        UpdateBatchPredictionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateBatchPredictionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBatchPredictionError {
    fn description(&self) -> &str {
        match *self {
            UpdateBatchPredictionError::InternalServer(ref cause) => cause,
            UpdateBatchPredictionError::InvalidInput(ref cause) => cause,
            UpdateBatchPredictionError::ResourceNotFound(ref cause) => cause,
            UpdateBatchPredictionError::Validation(ref cause) => cause,
            UpdateBatchPredictionError::Credentials(ref err) => err.description(),
            UpdateBatchPredictionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateBatchPredictionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDataSource
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourceError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDataSourceError {
    pub fn from_body(body: &str) -> UpdateDataSourceError {
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
                    "InternalServerException" => {
                        UpdateDataSourceError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateDataSourceError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateDataSourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDataSourceError::Validation(error_message.to_string())
                    }
                    _ => UpdateDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDataSourceError {
    fn from(err: serde_json::error::Error) -> UpdateDataSourceError {
        UpdateDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDataSourceError {
    fn from(err: CredentialsError) -> UpdateDataSourceError {
        UpdateDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDataSourceError {
    fn from(err: HttpDispatchError) -> UpdateDataSourceError {
        UpdateDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDataSourceError {
    fn from(err: io::Error) -> UpdateDataSourceError {
        UpdateDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDataSourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateDataSourceError::InternalServer(ref cause) => cause,
            UpdateDataSourceError::InvalidInput(ref cause) => cause,
            UpdateDataSourceError::ResourceNotFound(ref cause) => cause,
            UpdateDataSourceError::Validation(ref cause) => cause,
            UpdateDataSourceError::Credentials(ref err) => err.description(),
            UpdateDataSourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEvaluation
#[derive(Debug, PartialEq)]
pub enum UpdateEvaluationError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEvaluationError {
    pub fn from_body(body: &str) -> UpdateEvaluationError {
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
                    "InternalServerException" => {
                        UpdateEvaluationError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateEvaluationError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateEvaluationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEvaluationError::Validation(error_message.to_string())
                    }
                    _ => UpdateEvaluationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEvaluationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEvaluationError {
    fn from(err: serde_json::error::Error) -> UpdateEvaluationError {
        UpdateEvaluationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEvaluationError {
    fn from(err: CredentialsError) -> UpdateEvaluationError {
        UpdateEvaluationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEvaluationError {
    fn from(err: HttpDispatchError) -> UpdateEvaluationError {
        UpdateEvaluationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEvaluationError {
    fn from(err: io::Error) -> UpdateEvaluationError {
        UpdateEvaluationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEvaluationError {
    fn description(&self) -> &str {
        match *self {
            UpdateEvaluationError::InternalServer(ref cause) => cause,
            UpdateEvaluationError::InvalidInput(ref cause) => cause,
            UpdateEvaluationError::ResourceNotFound(ref cause) => cause,
            UpdateEvaluationError::Validation(ref cause) => cause,
            UpdateEvaluationError::Credentials(ref err) => err.description(),
            UpdateEvaluationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateEvaluationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMLModel
#[derive(Debug, PartialEq)]
pub enum UpdateMLModelError {
    /// <p>An error on the server occurred when trying to process a request.</p>
    InternalServer(String),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidInput(String),
    /// <p>A specified resource cannot be located.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateMLModelError {
    pub fn from_body(body: &str) -> UpdateMLModelError {
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
                    "InternalServerException" => {
                        UpdateMLModelError::InternalServer(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        UpdateMLModelError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateMLModelError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateMLModelError::Validation(error_message.to_string())
                    }
                    _ => UpdateMLModelError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateMLModelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateMLModelError {
    fn from(err: serde_json::error::Error) -> UpdateMLModelError {
        UpdateMLModelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateMLModelError {
    fn from(err: CredentialsError) -> UpdateMLModelError {
        UpdateMLModelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateMLModelError {
    fn from(err: HttpDispatchError) -> UpdateMLModelError {
        UpdateMLModelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateMLModelError {
    fn from(err: io::Error) -> UpdateMLModelError {
        UpdateMLModelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateMLModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMLModelError {
    fn description(&self) -> &str {
        match *self {
            UpdateMLModelError::InternalServer(ref cause) => cause,
            UpdateMLModelError::InvalidInput(ref cause) => cause,
            UpdateMLModelError::ResourceNotFound(ref cause) => cause,
            UpdateMLModelError::Validation(ref cause) => cause,
            UpdateMLModelError::Credentials(ref err) => err.description(),
            UpdateMLModelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateMLModelError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Machine Learning API. Amazon Machine Learning clients implement this trait.
pub trait MachineLearning {
    /// <p>Adds one or more tags to an object, up to a limit of 10. Each tag consists of a key and an optional value. If you add a tag using a key that is already associated with the ML object, <code>AddTags</code> updates the tag's value.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError>;

    /// <p>Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a <code>DataSource</code>. This operation creates a new <code>BatchPrediction</code>, and uses an <code>MLModel</code> and the data files referenced by the <code>DataSource</code> as information sources. </p> <p><code>CreateBatchPrediction</code> is an asynchronous operation. In response to <code>CreateBatchPrediction</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>BatchPrediction</code> status to <code>PENDING</code>. After the <code>BatchPrediction</code> completes, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can poll for status updates by using the <a>GetBatchPrediction</a> operation and checking the <code>Status</code> parameter of the result. After the <code>COMPLETED</code> status appears, the results are available in the location specified by the <code>OutputUri</code> parameter.</p>
    fn create_batch_prediction(
        &self,
        input: CreateBatchPredictionInput,
    ) -> RusotoFuture<CreateBatchPredictionOutput, CreateBatchPredictionError>;

    /// <p>Creates a <code>DataSource</code> object from an <a href="http://aws.amazon.com/rds/"> Amazon Relational Database Service</a> (Amazon RDS). A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRDS</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRDS</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used only to perform <code>&gt;CreateMLModel</code>&gt;, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML cannot accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p>
    fn create_data_source_from_rds(
        &self,
        input: CreateDataSourceFromRDSInput,
    ) -> RusotoFuture<CreateDataSourceFromRDSOutput, CreateDataSourceFromRDSError>;

    /// <p><p>Creates a <code>DataSource</code> from a database hosted on an Amazon Redshift cluster. A <code>DataSource</code> references data that can be used to perform either <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRedshift</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRedshift</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in <code>COMPLETED</code> or <code>PENDING</code> states can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can&#39;t accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observations should be contained in the database hosted on an Amazon Redshift cluster and should be specified by a <code>SelectSqlQuery</code> query. Amazon ML executes an <code>Unload</code> command in Amazon Redshift to transfer the result set of the <code>SelectSqlQuery</code> query to <code>S3StagingLocation</code>.</p> <p>After the <code>DataSource</code> has been created, it&#39;s ready for use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also requires a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p> &lt;?oxy<em>insert</em>start author=&quot;laurama&quot; timestamp=&quot;20160406T153842-0700&quot;&gt;<p>You can&#39;t change an existing datasource, but you can copy and modify the settings from an existing Amazon Redshift datasource to create a new datasource. To do so, call <code>GetDataSource</code> for an existing datasource and copy the values to a <code>CreateDataSource</code> call. Change the settings that you want to change and make sure that all required fields have the appropriate values.</p> &lt;?oxy<em>insert</em>end&gt;</p>
    fn create_data_source_from_redshift(
        &self,
        input: CreateDataSourceFromRedshiftInput,
    ) -> RusotoFuture<CreateDataSourceFromRedshiftOutput, CreateDataSourceFromRedshiftError>;

    /// <p>Creates a <code>DataSource</code> object. A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromS3</code> is an asynchronous operation. In response to <code>CreateDataSourceFromS3</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> has been created and is ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code> or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can't accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observation data used in a <code>DataSource</code> should be ready to use; that is, it should have a consistent structure, and missing data values should be kept to a minimum. The observation data must reside in one or more .csv files in an Amazon Simple Storage Service (Amazon S3) location, along with a schema that describes the data items by name and type. The same schema must be used for all of the data files referenced by the <code>DataSource</code>. </p> <p>After the <code>DataSource</code> has been created, it's ready to use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also needs a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p>
    fn create_data_source_from_s3(
        &self,
        input: CreateDataSourceFromS3Input,
    ) -> RusotoFuture<CreateDataSourceFromS3Output, CreateDataSourceFromS3Error>;

    /// <p>Creates a new <code>Evaluation</code> of an <code>MLModel</code>. An <code>MLModel</code> is evaluated on a set of observations associated to a <code>DataSource</code>. Like a <code>DataSource</code> for an <code>MLModel</code>, the <code>DataSource</code> for an <code>Evaluation</code> contains values for the <code>Target Variable</code>. The <code>Evaluation</code> compares the predicted result for each observation to the actual outcome and provides a summary so that you know how effective the <code>MLModel</code> functions on the test data. Evaluation generates a relevant performance metric, such as BinaryAUC, RegressionRMSE or MulticlassAvgFScore based on the corresponding <code>MLModelType</code>: <code>BINARY</code>, <code>REGRESSION</code> or <code>MULTICLASS</code>. </p> <p><code>CreateEvaluation</code> is an asynchronous operation. In response to <code>CreateEvaluation</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the evaluation status to <code>PENDING</code>. After the <code>Evaluation</code> is created and ready for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetEvaluation</code> operation to check progress of the evaluation during the creation operation.</p>
    fn create_evaluation(
        &self,
        input: CreateEvaluationInput,
    ) -> RusotoFuture<CreateEvaluationOutput, CreateEvaluationError>;

    /// <p>Creates a new <code>MLModel</code> using the <code>DataSource</code> and the recipe as information sources. </p> <p>An <code>MLModel</code> is nearly immutable. Users can update only the <code>MLModelName</code> and the <code>ScoreThreshold</code> in an <code>MLModel</code> without creating a new <code>MLModel</code>. </p> <p><code>CreateMLModel</code> is an asynchronous operation. In response to <code>CreateMLModel</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>MLModel</code> status to <code>PENDING</code>. After the <code>MLModel</code> has been created and ready is for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetMLModel</code> operation to check the progress of the <code>MLModel</code> during the creation operation.</p> <p> <code>CreateMLModel</code> requires a <code>DataSource</code> with computed statistics, which can be created by setting <code>ComputeStatistics</code> to <code>true</code> in <code>CreateDataSourceFromRDS</code>, <code>CreateDataSourceFromS3</code>, or <code>CreateDataSourceFromRedshift</code> operations. </p>
    fn create_ml_model(
        &self,
        input: CreateMLModelInput,
    ) -> RusotoFuture<CreateMLModelOutput, CreateMLModelError>;

    /// <p>Creates a real-time endpoint for the <code>MLModel</code>. The endpoint contains the URI of the <code>MLModel</code>; that is, the location to send real-time prediction requests for the specified <code>MLModel</code>.</p>
    fn create_realtime_endpoint(
        &self,
        input: CreateRealtimeEndpointInput,
    ) -> RusotoFuture<CreateRealtimeEndpointOutput, CreateRealtimeEndpointError>;

    /// <p>Assigns the DELETED status to a <code>BatchPrediction</code>, rendering it unusable.</p> <p>After using the <code>DeleteBatchPrediction</code> operation, you can use the <a>GetBatchPrediction</a> operation to verify that the status of the <code>BatchPrediction</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteBatchPrediction</code> operation is irreversible.</p>
    fn delete_batch_prediction(
        &self,
        input: DeleteBatchPredictionInput,
    ) -> RusotoFuture<DeleteBatchPredictionOutput, DeleteBatchPredictionError>;

    /// <p>Assigns the DELETED status to a <code>DataSource</code>, rendering it unusable.</p> <p>After using the <code>DeleteDataSource</code> operation, you can use the <a>GetDataSource</a> operation to verify that the status of the <code>DataSource</code> changed to DELETED.</p> <p><b>Caution:</b> The results of the <code>DeleteDataSource</code> operation are irreversible.</p>
    fn delete_data_source(
        &self,
        input: DeleteDataSourceInput,
    ) -> RusotoFuture<DeleteDataSourceOutput, DeleteDataSourceError>;

    /// <p><p>Assigns the <code>DELETED</code> status to an <code>Evaluation</code>, rendering it unusable.</p> <p>After invoking the <code>DeleteEvaluation</code> operation, you can use the <code>GetEvaluation</code> operation to verify that the status of the <code>Evaluation</code> changed to <code>DELETED</code>.</p> <caution><title>Caution</title> <p>The results of the <code>DeleteEvaluation</code> operation are irreversible.</p></caution></p>
    fn delete_evaluation(
        &self,
        input: DeleteEvaluationInput,
    ) -> RusotoFuture<DeleteEvaluationOutput, DeleteEvaluationError>;

    /// <p>Assigns the <code>DELETED</code> status to an <code>MLModel</code>, rendering it unusable.</p> <p>After using the <code>DeleteMLModel</code> operation, you can use the <code>GetMLModel</code> operation to verify that the status of the <code>MLModel</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteMLModel</code> operation is irreversible.</p>
    fn delete_ml_model(
        &self,
        input: DeleteMLModelInput,
    ) -> RusotoFuture<DeleteMLModelOutput, DeleteMLModelError>;

    /// <p>Deletes a real time endpoint of an <code>MLModel</code>.</p>
    fn delete_realtime_endpoint(
        &self,
        input: DeleteRealtimeEndpointInput,
    ) -> RusotoFuture<DeleteRealtimeEndpointOutput, DeleteRealtimeEndpointError>;

    /// <p>Deletes the specified tags associated with an ML object. After this operation is complete, you can't recover deleted tags.</p> <p>If you specify a tag that doesn't exist, Amazon ML ignores it.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError>;

    /// <p>Returns a list of <code>BatchPrediction</code> operations that match the search criteria in the request.</p>
    fn describe_batch_predictions(
        &self,
        input: DescribeBatchPredictionsInput,
    ) -> RusotoFuture<DescribeBatchPredictionsOutput, DescribeBatchPredictionsError>;

    /// <p>Returns a list of <code>DataSource</code> that match the search criteria in the request.</p>
    fn describe_data_sources(
        &self,
        input: DescribeDataSourcesInput,
    ) -> RusotoFuture<DescribeDataSourcesOutput, DescribeDataSourcesError>;

    /// <p>Returns a list of <code>DescribeEvaluations</code> that match the search criteria in the request.</p>
    fn describe_evaluations(
        &self,
        input: DescribeEvaluationsInput,
    ) -> RusotoFuture<DescribeEvaluationsOutput, DescribeEvaluationsError>;

    /// <p>Returns a list of <code>MLModel</code> that match the search criteria in the request.</p>
    fn describe_ml_models(
        &self,
        input: DescribeMLModelsInput,
    ) -> RusotoFuture<DescribeMLModelsOutput, DescribeMLModelsError>;

    /// <p>Describes one or more of the tags for your Amazon ML object.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError>;

    /// <p>Returns a <code>BatchPrediction</code> that includes detailed metadata, status, and data file information for a <code>Batch Prediction</code> request.</p>
    fn get_batch_prediction(
        &self,
        input: GetBatchPredictionInput,
    ) -> RusotoFuture<GetBatchPredictionOutput, GetBatchPredictionError>;

    /// <p>Returns a <code>DataSource</code> that includes metadata and data file information, as well as the current status of the <code>DataSource</code>.</p> <p><code>GetDataSource</code> provides results in normal or verbose format. The verbose format adds the schema description and the list of files pointed to by the DataSource to the normal format.</p>
    fn get_data_source(
        &self,
        input: GetDataSourceInput,
    ) -> RusotoFuture<GetDataSourceOutput, GetDataSourceError>;

    /// <p>Returns an <code>Evaluation</code> that includes metadata as well as the current status of the <code>Evaluation</code>.</p>
    fn get_evaluation(
        &self,
        input: GetEvaluationInput,
    ) -> RusotoFuture<GetEvaluationOutput, GetEvaluationError>;

    /// <p>Returns an <code>MLModel</code> that includes detailed metadata, data source information, and the current status of the <code>MLModel</code>.</p> <p><code>GetMLModel</code> provides results in normal or verbose format. </p>
    fn get_ml_model(
        &self,
        input: GetMLModelInput,
    ) -> RusotoFuture<GetMLModelOutput, GetMLModelError>;

    /// <p><p>Generates a prediction for the observation using the specified <code>ML Model</code>.</p> <note><title>Note</title> <p>Not all response parameters will be populated. Whether a response parameter is populated depends on the type of model requested.</p></note></p>
    fn predict(&self, input: PredictInput) -> RusotoFuture<PredictOutput, PredictError>;

    /// <p>Updates the <code>BatchPredictionName</code> of a <code>BatchPrediction</code>.</p> <p>You can use the <code>GetBatchPrediction</code> operation to view the contents of the updated data element.</p>
    fn update_batch_prediction(
        &self,
        input: UpdateBatchPredictionInput,
    ) -> RusotoFuture<UpdateBatchPredictionOutput, UpdateBatchPredictionError>;

    /// <p>Updates the <code>DataSourceName</code> of a <code>DataSource</code>.</p> <p>You can use the <code>GetDataSource</code> operation to view the contents of the updated data element.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceInput,
    ) -> RusotoFuture<UpdateDataSourceOutput, UpdateDataSourceError>;

    /// <p>Updates the <code>EvaluationName</code> of an <code>Evaluation</code>.</p> <p>You can use the <code>GetEvaluation</code> operation to view the contents of the updated data element.</p>
    fn update_evaluation(
        &self,
        input: UpdateEvaluationInput,
    ) -> RusotoFuture<UpdateEvaluationOutput, UpdateEvaluationError>;

    /// <p>Updates the <code>MLModelName</code> and the <code>ScoreThreshold</code> of an <code>MLModel</code>.</p> <p>You can use the <code>GetMLModel</code> operation to view the contents of the updated data element.</p>
    fn update_ml_model(
        &self,
        input: UpdateMLModelInput,
    ) -> RusotoFuture<UpdateMLModelOutput, UpdateMLModelError>;
}
/// A client for the Amazon Machine Learning API.
pub struct MachineLearningClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl MachineLearningClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> MachineLearningClient {
        MachineLearningClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> MachineLearningClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        MachineLearningClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> MachineLearning for MachineLearningClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds one or more tags to an object, up to a limit of 10. Each tag consists of a key and an optional value. If you add a tag using a key that is already associated with the ML object, <code>AddTags</code> updates the tag's value.</p>
    fn add_tags(&self, input: AddTagsInput) -> RusotoFuture<AddTagsOutput, AddTagsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.AddTags");
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

    /// <p>Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a <code>DataSource</code>. This operation creates a new <code>BatchPrediction</code>, and uses an <code>MLModel</code> and the data files referenced by the <code>DataSource</code> as information sources. </p> <p><code>CreateBatchPrediction</code> is an asynchronous operation. In response to <code>CreateBatchPrediction</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>BatchPrediction</code> status to <code>PENDING</code>. After the <code>BatchPrediction</code> completes, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can poll for status updates by using the <a>GetBatchPrediction</a> operation and checking the <code>Status</code> parameter of the result. After the <code>COMPLETED</code> status appears, the results are available in the location specified by the <code>OutputUri</code> parameter.</p>
    fn create_batch_prediction(
        &self,
        input: CreateBatchPredictionInput,
    ) -> RusotoFuture<CreateBatchPredictionOutput, CreateBatchPredictionError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateBatchPrediction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBatchPredictionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateBatchPredictionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a <code>DataSource</code> object from an <a href="http://aws.amazon.com/rds/"> Amazon Relational Database Service</a> (Amazon RDS). A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRDS</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRDS</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used only to perform <code>&gt;CreateMLModel</code>&gt;, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML cannot accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p>
    fn create_data_source_from_rds(
        &self,
        input: CreateDataSourceFromRDSInput,
    ) -> RusotoFuture<CreateDataSourceFromRDSOutput, CreateDataSourceFromRDSError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateDataSourceFromRDS");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDataSourceFromRDSOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDataSourceFromRDSError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a <code>DataSource</code> from a database hosted on an Amazon Redshift cluster. A <code>DataSource</code> references data that can be used to perform either <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromRedshift</code> is an asynchronous operation. In response to <code>CreateDataSourceFromRedshift</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> is created and ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in <code>COMPLETED</code> or <code>PENDING</code> states can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can&#39;t accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observations should be contained in the database hosted on an Amazon Redshift cluster and should be specified by a <code>SelectSqlQuery</code> query. Amazon ML executes an <code>Unload</code> command in Amazon Redshift to transfer the result set of the <code>SelectSqlQuery</code> query to <code>S3StagingLocation</code>.</p> <p>After the <code>DataSource</code> has been created, it&#39;s ready for use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also requires a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p> &lt;?oxy<em>insert</em>start author=&quot;laurama&quot; timestamp=&quot;20160406T153842-0700&quot;&gt;<p>You can&#39;t change an existing datasource, but you can copy and modify the settings from an existing Amazon Redshift datasource to create a new datasource. To do so, call <code>GetDataSource</code> for an existing datasource and copy the values to a <code>CreateDataSource</code> call. Change the settings that you want to change and make sure that all required fields have the appropriate values.</p> &lt;?oxy<em>insert</em>end&gt;</p>
    fn create_data_source_from_redshift(
        &self,
        input: CreateDataSourceFromRedshiftInput,
    ) -> RusotoFuture<CreateDataSourceFromRedshiftOutput, CreateDataSourceFromRedshiftError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonML_20141212.CreateDataSourceFromRedshift",
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

                    serde_json::from_str::<CreateDataSourceFromRedshiftOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDataSourceFromRedshiftError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a <code>DataSource</code> object. A <code>DataSource</code> references data that can be used to perform <code>CreateMLModel</code>, <code>CreateEvaluation</code>, or <code>CreateBatchPrediction</code> operations.</p> <p><code>CreateDataSourceFromS3</code> is an asynchronous operation. In response to <code>CreateDataSourceFromS3</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>DataSource</code> status to <code>PENDING</code>. After the <code>DataSource</code> has been created and is ready for use, Amazon ML sets the <code>Status</code> parameter to <code>COMPLETED</code>. <code>DataSource</code> in the <code>COMPLETED</code> or <code>PENDING</code> state can be used to perform only <code>CreateMLModel</code>, <code>CreateEvaluation</code> or <code>CreateBatchPrediction</code> operations. </p> <p> If Amazon ML can't accept the input source, it sets the <code>Status</code> parameter to <code>FAILED</code> and includes an error message in the <code>Message</code> attribute of the <code>GetDataSource</code> operation response. </p> <p>The observation data used in a <code>DataSource</code> should be ready to use; that is, it should have a consistent structure, and missing data values should be kept to a minimum. The observation data must reside in one or more .csv files in an Amazon Simple Storage Service (Amazon S3) location, along with a schema that describes the data items by name and type. The same schema must be used for all of the data files referenced by the <code>DataSource</code>. </p> <p>After the <code>DataSource</code> has been created, it's ready to use in evaluations and batch predictions. If you plan to use the <code>DataSource</code> to train an <code>MLModel</code>, the <code>DataSource</code> also needs a recipe. A recipe describes how each input variable will be used in training an <code>MLModel</code>. Will the variable be included or excluded from training? Will the variable be manipulated; for example, will it be combined with another variable or will it be split apart into word combinations? The recipe provides answers to these questions.</p>
    fn create_data_source_from_s3(
        &self,
        input: CreateDataSourceFromS3Input,
    ) -> RusotoFuture<CreateDataSourceFromS3Output, CreateDataSourceFromS3Error> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateDataSourceFromS3");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDataSourceFromS3Output>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateDataSourceFromS3Error::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new <code>Evaluation</code> of an <code>MLModel</code>. An <code>MLModel</code> is evaluated on a set of observations associated to a <code>DataSource</code>. Like a <code>DataSource</code> for an <code>MLModel</code>, the <code>DataSource</code> for an <code>Evaluation</code> contains values for the <code>Target Variable</code>. The <code>Evaluation</code> compares the predicted result for each observation to the actual outcome and provides a summary so that you know how effective the <code>MLModel</code> functions on the test data. Evaluation generates a relevant performance metric, such as BinaryAUC, RegressionRMSE or MulticlassAvgFScore based on the corresponding <code>MLModelType</code>: <code>BINARY</code>, <code>REGRESSION</code> or <code>MULTICLASS</code>. </p> <p><code>CreateEvaluation</code> is an asynchronous operation. In response to <code>CreateEvaluation</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the evaluation status to <code>PENDING</code>. After the <code>Evaluation</code> is created and ready for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetEvaluation</code> operation to check progress of the evaluation during the creation operation.</p>
    fn create_evaluation(
        &self,
        input: CreateEvaluationInput,
    ) -> RusotoFuture<CreateEvaluationOutput, CreateEvaluationError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEvaluationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateEvaluationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new <code>MLModel</code> using the <code>DataSource</code> and the recipe as information sources. </p> <p>An <code>MLModel</code> is nearly immutable. Users can update only the <code>MLModelName</code> and the <code>ScoreThreshold</code> in an <code>MLModel</code> without creating a new <code>MLModel</code>. </p> <p><code>CreateMLModel</code> is an asynchronous operation. In response to <code>CreateMLModel</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>MLModel</code> status to <code>PENDING</code>. After the <code>MLModel</code> has been created and ready is for use, Amazon ML sets the status to <code>COMPLETED</code>. </p> <p>You can use the <code>GetMLModel</code> operation to check the progress of the <code>MLModel</code> during the creation operation.</p> <p> <code>CreateMLModel</code> requires a <code>DataSource</code> with computed statistics, which can be created by setting <code>ComputeStatistics</code> to <code>true</code> in <code>CreateDataSourceFromRDS</code>, <code>CreateDataSourceFromS3</code>, or <code>CreateDataSourceFromRedshift</code> operations. </p>
    fn create_ml_model(
        &self,
        input: CreateMLModelInput,
    ) -> RusotoFuture<CreateMLModelOutput, CreateMLModelError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateMLModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateMLModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a real-time endpoint for the <code>MLModel</code>. The endpoint contains the URI of the <code>MLModel</code>; that is, the location to send real-time prediction requests for the specified <code>MLModel</code>.</p>
    fn create_realtime_endpoint(
        &self,
        input: CreateRealtimeEndpointInput,
    ) -> RusotoFuture<CreateRealtimeEndpointOutput, CreateRealtimeEndpointError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.CreateRealtimeEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRealtimeEndpointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateRealtimeEndpointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Assigns the DELETED status to a <code>BatchPrediction</code>, rendering it unusable.</p> <p>After using the <code>DeleteBatchPrediction</code> operation, you can use the <a>GetBatchPrediction</a> operation to verify that the status of the <code>BatchPrediction</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteBatchPrediction</code> operation is irreversible.</p>
    fn delete_batch_prediction(
        &self,
        input: DeleteBatchPredictionInput,
    ) -> RusotoFuture<DeleteBatchPredictionOutput, DeleteBatchPredictionError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteBatchPrediction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBatchPredictionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBatchPredictionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Assigns the DELETED status to a <code>DataSource</code>, rendering it unusable.</p> <p>After using the <code>DeleteDataSource</code> operation, you can use the <a>GetDataSource</a> operation to verify that the status of the <code>DataSource</code> changed to DELETED.</p> <p><b>Caution:</b> The results of the <code>DeleteDataSource</code> operation are irreversible.</p>
    fn delete_data_source(
        &self,
        input: DeleteDataSourceInput,
    ) -> RusotoFuture<DeleteDataSourceOutput, DeleteDataSourceError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDataSourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDataSourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Assigns the <code>DELETED</code> status to an <code>Evaluation</code>, rendering it unusable.</p> <p>After invoking the <code>DeleteEvaluation</code> operation, you can use the <code>GetEvaluation</code> operation to verify that the status of the <code>Evaluation</code> changed to <code>DELETED</code>.</p> <caution><title>Caution</title> <p>The results of the <code>DeleteEvaluation</code> operation are irreversible.</p></caution></p>
    fn delete_evaluation(
        &self,
        input: DeleteEvaluationInput,
    ) -> RusotoFuture<DeleteEvaluationOutput, DeleteEvaluationError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteEvaluationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEvaluationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Assigns the <code>DELETED</code> status to an <code>MLModel</code>, rendering it unusable.</p> <p>After using the <code>DeleteMLModel</code> operation, you can use the <code>GetMLModel</code> operation to verify that the status of the <code>MLModel</code> changed to DELETED.</p> <p><b>Caution:</b> The result of the <code>DeleteMLModel</code> operation is irreversible.</p>
    fn delete_ml_model(
        &self,
        input: DeleteMLModelInput,
    ) -> RusotoFuture<DeleteMLModelOutput, DeleteMLModelError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteMLModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteMLModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a real time endpoint of an <code>MLModel</code>.</p>
    fn delete_realtime_endpoint(
        &self,
        input: DeleteRealtimeEndpointInput,
    ) -> RusotoFuture<DeleteRealtimeEndpointOutput, DeleteRealtimeEndpointError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteRealtimeEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRealtimeEndpointOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRealtimeEndpointError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified tags associated with an ML object. After this operation is complete, you can't recover deleted tags.</p> <p>If you specify a tag that doesn't exist, Amazon ML ignores it.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsInput,
    ) -> RusotoFuture<DeleteTagsOutput, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of <code>BatchPrediction</code> operations that match the search criteria in the request.</p>
    fn describe_batch_predictions(
        &self,
        input: DescribeBatchPredictionsInput,
    ) -> RusotoFuture<DescribeBatchPredictionsOutput, DescribeBatchPredictionsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeBatchPredictions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeBatchPredictionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBatchPredictionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of <code>DataSource</code> that match the search criteria in the request.</p>
    fn describe_data_sources(
        &self,
        input: DescribeDataSourcesInput,
    ) -> RusotoFuture<DescribeDataSourcesOutput, DescribeDataSourcesError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeDataSources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDataSourcesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDataSourcesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of <code>DescribeEvaluations</code> that match the search criteria in the request.</p>
    fn describe_evaluations(
        &self,
        input: DescribeEvaluationsInput,
    ) -> RusotoFuture<DescribeEvaluationsOutput, DescribeEvaluationsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeEvaluations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEvaluationsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEvaluationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of <code>MLModel</code> that match the search criteria in the request.</p>
    fn describe_ml_models(
        &self,
        input: DescribeMLModelsInput,
    ) -> RusotoFuture<DescribeMLModelsOutput, DescribeMLModelsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeMLModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeMLModelsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeMLModelsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes one or more of the tags for your Amazon ML object.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsInput,
    ) -> RusotoFuture<DescribeTagsOutput, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTagsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a <code>BatchPrediction</code> that includes detailed metadata, status, and data file information for a <code>Batch Prediction</code> request.</p>
    fn get_batch_prediction(
        &self,
        input: GetBatchPredictionInput,
    ) -> RusotoFuture<GetBatchPredictionOutput, GetBatchPredictionError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetBatchPrediction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetBatchPredictionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetBatchPredictionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a <code>DataSource</code> that includes metadata and data file information, as well as the current status of the <code>DataSource</code>.</p> <p><code>GetDataSource</code> provides results in normal or verbose format. The verbose format adds the schema description and the list of files pointed to by the DataSource to the normal format.</p>
    fn get_data_source(
        &self,
        input: GetDataSourceInput,
    ) -> RusotoFuture<GetDataSourceOutput, GetDataSourceError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDataSourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDataSourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an <code>Evaluation</code> that includes metadata as well as the current status of the <code>Evaluation</code>.</p>
    fn get_evaluation(
        &self,
        input: GetEvaluationInput,
    ) -> RusotoFuture<GetEvaluationOutput, GetEvaluationError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetEvaluationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetEvaluationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an <code>MLModel</code> that includes detailed metadata, data source information, and the current status of the <code>MLModel</code>.</p> <p><code>GetMLModel</code> provides results in normal or verbose format. </p>
    fn get_ml_model(
        &self,
        input: GetMLModelInput,
    ) -> RusotoFuture<GetMLModelOutput, GetMLModelError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.GetMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetMLModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetMLModelError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Generates a prediction for the observation using the specified <code>ML Model</code>.</p> <note><title>Note</title> <p>Not all response parameters will be populated. Whether a response parameter is populated depends on the type of model requested.</p></note></p>
    fn predict(&self, input: PredictInput) -> RusotoFuture<PredictOutput, PredictError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.Predict");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PredictOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PredictError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the <code>BatchPredictionName</code> of a <code>BatchPrediction</code>.</p> <p>You can use the <code>GetBatchPrediction</code> operation to view the contents of the updated data element.</p>
    fn update_batch_prediction(
        &self,
        input: UpdateBatchPredictionInput,
    ) -> RusotoFuture<UpdateBatchPredictionOutput, UpdateBatchPredictionError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateBatchPrediction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateBatchPredictionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateBatchPredictionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the <code>DataSourceName</code> of a <code>DataSource</code>.</p> <p>You can use the <code>GetDataSource</code> operation to view the contents of the updated data element.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceInput,
    ) -> RusotoFuture<UpdateDataSourceOutput, UpdateDataSourceError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDataSourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDataSourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the <code>EvaluationName</code> of an <code>Evaluation</code>.</p> <p>You can use the <code>GetEvaluation</code> operation to view the contents of the updated data element.</p>
    fn update_evaluation(
        &self,
        input: UpdateEvaluationInput,
    ) -> RusotoFuture<UpdateEvaluationOutput, UpdateEvaluationError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateEvaluation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateEvaluationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEvaluationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the <code>MLModelName</code> and the <code>ScoreThreshold</code> of an <code>MLModel</code>.</p> <p>You can use the <code>GetMLModel</code> operation to view the contents of the updated data element.</p>
    fn update_ml_model(
        &self,
        input: UpdateMLModelInput,
    ) -> RusotoFuture<UpdateMLModelOutput, UpdateMLModelError> {
        let mut request = SignedRequest::new("POST", "machinelearning", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonML_20141212.UpdateMLModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateMLModelOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateMLModelError::from_body(
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
