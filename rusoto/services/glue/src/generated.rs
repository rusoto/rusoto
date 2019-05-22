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
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Defines an action to be initiated by a trigger.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    /// <p>Arguments to be passed to the job run.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of a job to be executed.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name of the SecurityConfiguration structure to be used with this action.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The JobRun timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchCreatePartitionRequest {
    /// <p>The ID of the catalog in which the partion is to be created. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the metadata database in which the partition is to be created.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be created.</p>
    #[serde(rename = "PartitionInputList")]
    pub partition_input_list: Vec<PartitionInput>,
    /// <p>The name of the metadata table in which the partition is to be created.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchCreatePartitionResponse {
    /// <p>Errors encountered when trying to create the requested partitions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A list of names of the connections to delete.</p>
    #[serde(rename = "ConnectionNameList")]
    pub connection_name_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDeleteConnectionResponse {
    /// <p>A map of the names of connections that were not successfully deleted to error details.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, ErrorDetail>>,
    /// <p>A list of names of the connection definitions that were successfully deleted.</p>
    #[serde(rename = "Succeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeletePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of <code>PartitionInput</code> structures that define the partitions to be deleted.</p>
    #[serde(rename = "PartitionsToDelete")]
    pub partitions_to_delete: Vec<PartitionValueList>,
    /// <p>The name of the table where the partitions to be deleted is located.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDeletePartitionResponse {
    /// <p>Errors encountered when trying to delete the requested partitions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PartitionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the tables to delete reside. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of the table to delete.</p>
    #[serde(rename = "TablesToDelete")]
    pub tables_to_delete: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDeleteTableResponse {
    /// <p>A list of errors encountered in attempting to delete the specified tables.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDeleteTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionIds")]
    pub version_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchDeleteTableVersionResponse {
    /// <p>A list of errors encountered while trying to delete the specified table versions.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TableVersionError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetPartitionRequest {
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partitions reside.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A list of partition values identifying the partitions to retrieve.</p>
    #[serde(rename = "PartitionsToGet")]
    pub partitions_to_get: Vec<PartitionValueList>,
    /// <p>The name of the partitions' table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetPartitionResponse {
    /// <p>A list of the requested partitions.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
    /// <p>A list of the partition values in the request for which partions were not returned.</p>
    #[serde(rename = "UnprocessedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<Vec<PartitionValueList>>,
}

/// <p>Records an error that occurred when attempting to stop a specified job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchStopJobRunError {
    /// <p>Specifies details about the error that was encountered.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The name of the job definition used in the job run in question.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The JobRunId of the job run in question.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchStopJobRunRequest {
    /// <p>The name of the job definition for which to stop job runs.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>A list of the JobRunIds that should be stopped for that job definition.</p>
    #[serde(rename = "JobRunIds")]
    pub job_run_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchStopJobRunResponse {
    /// <p>A list of the errors that were encountered in tryng to stop JobRuns, including the JobRunId for which each error was encountered and details about the error.</p>
    #[serde(rename = "Errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchStopJobRunError>>,
    /// <p>A list of the JobRuns that were successfully submitted for stopping.</p>
    #[serde(rename = "SuccessfulSubmissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_submissions: Option<Vec<BatchStopJobRunSuccessfulSubmission>>,
}

/// <p>Records a successful request to stop a specified JobRun.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchStopJobRunSuccessfulSubmission {
    /// <p>The name of the job definition used in the job run that was stopped.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The JobRunId of the job run that was stopped.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

/// <p>Specifies a table definition in the Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CatalogEntry {
    /// <p>The database in which the table metadata resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table in question.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>A structure containing migration status information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CatalogImportStatus {
    /// <p>True if the migration has completed, or False otherwise.</p>
    #[serde(rename = "ImportCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_completed: Option<bool>,
    /// <p>The time that the migration was started.</p>
    #[serde(rename = "ImportTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_time: Option<f64>,
    /// <p>The name of the person who initiated the migration.</p>
    #[serde(rename = "ImportedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_by: Option<String>,
}

/// <p>Classifiers are triggered during a crawl task. A classifier checks whether a given file is in a format it can handle, and if it is, the classifier creates a schema in the form of a <code>StructType</code> object that matches that data format.</p> <p>You can use the standard classifiers that AWS Glue supplies, or you can write your own classifiers to best categorize your data sources and specify the appropriate schemas to use for them. A classifier can be a <code>grok</code> classifier, an <code>XML</code> classifier, or a <code>JSON</code> classifier, as specified in one of the fields in the <code>Classifier</code> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Classifier {
    /// <p>A <code>GrokClassifier</code> object.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<GrokClassifier>,
    /// <p>A <code>JsonClassifier</code> object.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<JsonClassifier>,
    /// <p>An <code>XMLClassifier</code> object.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<XMLClassifier>,
}

/// <p>Specifies how CloudWatch data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchEncryption {
    /// <p>The encryption mode to use for CloudWatch data.</p>
    #[serde(rename = "CloudWatchEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_mode: Option<String>,
    /// <p>The AWS ARN of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Represents a directional edge in a directed acyclic graph (DAG).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenEdge {
    /// <p>The ID of the node at which the edge starts.</p>
    #[serde(rename = "Source")]
    pub source: String,
    /// <p>The ID of the node at which the edge ends.</p>
    #[serde(rename = "Target")]
    pub target: String,
    /// <p>The target of the edge.</p>
    #[serde(rename = "TargetParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter: Option<String>,
}

/// <p>Represents a node in a directed acyclic graph (DAG)</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenNode {
    /// <p>Properties of the node, in the form of name-value pairs.</p>
    #[serde(rename = "Args")]
    pub args: Vec<CodeGenNodeArg>,
    /// <p>A node identifier that is unique within the node's graph.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The line number of the node.</p>
    #[serde(rename = "LineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i64>,
    /// <p>The type of node this is.</p>
    #[serde(rename = "NodeType")]
    pub node_type: String,
}

/// <p>An argument or property of a node.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenNodeArg {
    /// <p>The name of the argument or property.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>True if the value is used as a parameter.</p>
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<bool>,
    /// <p>The value of the argument or property.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A column in a <code>Table</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// <p>Free-form text comment.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the <code>Column</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The datatype of data in the <code>Column</code>.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Defines a condition under which a trigger fires.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// <p>The name of the Job to whose JobRuns this condition applies and on which this trigger waits.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>A logical operator.</p>
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
    /// <p>The condition state. Currently, the values supported are SUCCEEDED, STOPPED, TIMEOUT and FAILED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Defines a connection to a data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connection {
    /// <p><p>These key-value pairs define parameters for the connection:</p> <ul> <li> <p> <code>HOST</code> - The host URI: either the fully qualified domain name (FQDN) or the IPv4 address of the database host.</p> </li> <li> <p> <code>PORT</code> - The port number, between 1024 and 65535, of the port on which the database host is listening for database connections.</p> </li> <li> <p> <code>USER<em>NAME</code> - The name under which to log in to the database. The value string for <code>USER</em>NAME</code> is &quot;<code>USERNAME</code>&quot;.</p> </li> <li> <p> <code>PASSWORD</code> - A password, if one is used, for the user name.</p> </li> <li> <p> <code>ENCRYPTED<em>PASSWORD</code> - When you enable connection password protection by setting <code>ConnectionPasswordEncryption</code> in the Data Catalog encryption settings, this field stores the key you designate to encrypt the password.</p> </li> <li> <p> <code>JDBC</em>DRIVER<em>JAR</em>URI</code> - The S3 path of the a jar file that contains the JDBC driver to use.</p> </li> <li> <p> <code>JDBC<em>DRIVER</em>CLASS<em>NAME</code> - The class name of the JDBC driver to use.</p> </li> <li> <p> <code>JDBC</em>ENGINE</code> - The name of the JDBC engine to use.</p> </li> <li> <p> <code>JDBC<em>ENGINE</em>VERSION</code> - The version of the JDBC engine to use.</p> </li> <li> <p> <code>CONFIG<em>FILES</code> - (Reserved for future use).</p> </li> <li> <p> <code>INSTANCE</em>ID</code> - The instance ID to use.</p> </li> <li> <p> <code>JDBC<em>CONNECTION</em>URL</code> - The URL for the JDBC connection.</p> </li> <li> <p> <code>JDBC<em>ENFORCE</em>SSL</code> - A Boolean string (true, false) specifying whether SSL with hostname matching will be enforced for the JDBC connection on the client. The default is false.</p> </li> </ul></p>
    #[serde(rename = "ConnectionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the connection. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>The time this connection definition was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Description of the connection.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user, group or role that last updated this connection definition.</p>
    #[serde(rename = "LastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>The last time this connection definition was updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A list of criteria that can be used in selecting this connection.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    /// <p>The name of the connection definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A map of physical connection requirements, such as VPC and SecurityGroup, needed for making this connection successfully.</p>
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

/// <p>A structure used to specify a connection to create or update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConnectionInput {
    /// <p>These key-value pairs define parameters for the connection.</p>
    #[serde(rename = "ConnectionProperties")]
    pub connection_properties: ::std::collections::HashMap<String, String>,
    /// <p>The type of the connection. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    /// <p>Description of the connection.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of criteria that can be used in selecting this connection.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
    /// <p>The name of the connection.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A map of physical connection requirements, such as VPC and SecurityGroup, needed for making this connection successfully.</p>
    #[serde(rename = "PhysicalConnectionRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
}

/// <p>The data structure used by the Data Catalog to encrypt the password as part of <code>CreateConnection</code> or <code>UpdateConnection</code> and store it in the <code>ENCRYPTED_PASSWORD</code> field in the connection properties. You can enable catalog encryption or only password encryption.</p> <p>When a <code>CreationConnection</code> request arrives containing a password, the Data Catalog first encrypts the password using your KMS key, and then encrypts the whole connection object again if catalog encryption is also enabled.</p> <p>This encryption requires that you set KMS key permissions to enable or restrict access on the password key according to your security requirements. For example, you may want only admin users to have decrypt permission on the password key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionPasswordEncryption {
    /// <p>A KMS key used to protect access to the JDBC source. </p> <p>All users in your account should be granted the <code>kms:encrypt</code> permission to encrypt passwords before storing them in the Data Catalog (through the AWS Glue <code>CreateConnection</code> operation).</p> <p>The decrypt permission should be granted only to KMS key admins and IAM roles designated for AWS Glue crawlers.</p>
    #[serde(rename = "AwsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key_id: Option<String>,
    /// <p>When the <code>ReturnConnectionPasswordEncrypted</code> flag is set to "true", passwords remain encrypted in the responses of <code>GetConnection</code> and <code>GetConnections</code>. This encryption takes effect independently from catalog encryption. </p>
    #[serde(rename = "ReturnConnectionPasswordEncrypted")]
    pub return_connection_password_encrypted: bool,
}

/// <p>Specifies the connections used by a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionsList {
    /// <p>A list of connections used by the job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
}

/// <p>Specifies a crawler program that examines a data source and uses classifiers to try to determine its schema. If successful, the crawler records metadata concerning the data source in the AWS Glue Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Crawler {
    /// <p>A list of custom classifiers associated with the crawler.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>Crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>If the crawler is running, contains the total time elapsed since the last crawl began.</p>
    #[serde(rename = "CrawlElapsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_elapsed_time: Option<i64>,
    /// <p>The name of the SecurityConfiguration structure to be used by this Crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The time when the crawler was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The database where metadata is written by this crawler.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The status of the last crawl, and potentially error information if an error occurred.</p>
    #[serde(rename = "LastCrawl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_crawl: Option<LastCrawlInfo>,
    /// <p>The time the crawler was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The crawler name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The IAM role (or ARN of an IAM role) used to access customer resources, such as data in Amazon S3.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>For scheduled crawlers, the schedule when the crawler runs.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>Sets the behavior when the crawler finds a changed or deleted object.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>Indicates whether the crawler is running, or whether a run is pending.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The prefix added to the names of tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>A collection of targets to crawl.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
    /// <p>The version of the crawler.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Metrics for a specified crawler.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CrawlerMetrics {
    /// <p>The name of the crawler.</p>
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<String>,
    /// <p>The duration of the crawler's most recent run, in seconds.</p>
    #[serde(rename = "LastRuntimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_runtime_seconds: Option<f64>,
    /// <p>The median duration of this crawler's runs, in seconds.</p>
    #[serde(rename = "MedianRuntimeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median_runtime_seconds: Option<f64>,
    /// <p>True if the crawler is still estimating how long it will take to complete this run.</p>
    #[serde(rename = "StillEstimating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_estimating: Option<bool>,
    /// <p>The number of tables created by this crawler.</p>
    #[serde(rename = "TablesCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_created: Option<i64>,
    /// <p>The number of tables deleted by this crawler.</p>
    #[serde(rename = "TablesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_deleted: Option<i64>,
    /// <p>The number of tables updated by this crawler.</p>
    #[serde(rename = "TablesUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_updated: Option<i64>,
    /// <p>The estimated time left to complete a running crawl.</p>
    #[serde(rename = "TimeLeftSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_left_seconds: Option<f64>,
}

/// <p>Specifies data stores to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrawlerTargets {
    /// <p>Specifies DynamoDB targets.</p>
    #[serde(rename = "DynamoDBTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_targets: Option<Vec<DynamoDBTarget>>,
    /// <p>Specifies JDBC targets.</p>
    #[serde(rename = "JdbcTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,
    /// <p>Specifies Amazon S3 targets.</p>
    #[serde(rename = "S3Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_targets: Option<Vec<S3Target>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClassifierRequest {
    /// <p>A <code>GrokClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<CreateGrokClassifierRequest>,
    /// <p>A <code>JsonClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<CreateJsonClassifierRequest>,
    /// <p>An <code>XMLClassifier</code> object specifying the classifier to create.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<CreateXMLClassifierRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectionRequest {
    /// <p>The ID of the Data Catalog in which to create the connection. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>ConnectionInput</code> object defining the connection to create.</p>
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCrawlerRequest {
    /// <p>A list of custom classifiers that the user has registered. By default, all built-in classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>Crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used by this Crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The AWS Glue database where results are written, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A description of the new crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new crawler.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The IAM role (or ARN of an IAM role) used by the new crawler to access customer resources.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>Policy for the crawler's update and deletion behavior.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>The table prefix used for catalog tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>A list of collection of targets to crawl.</p>
    #[serde(rename = "Targets")]
    pub targets: CrawlerTargets,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatabaseRequest {
    /// <p>The ID of the Data Catalog in which to create the database. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>DatabaseInput</code> object defining the metadata database to create in the catalog.</p>
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDevEndpointRequest {
    /// <p>The name to be assigned to the new DevEndpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p>Path(s) to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint. Multiple values must be complete paths separated by a comma.</p> <p>Please note that only pure Python libraries can currently be used on a DevEndpoint. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) to allocate to this DevEndpoint.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The public key to be used by this DevEndpoint for authentication. This attribute is provided for backward compatibility, as the recommended attribute to use is public keys.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p><p>A list of public keys to be used by the DevEndpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p> <note> <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys: call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p> </note></p>
    #[serde(rename = "PublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// <p>The IAM role for the DevEndpoint.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The name of the SecurityConfiguration structure to be used with this DevEndpoint.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>Security group IDs for the security groups to be used by the new DevEndpoint.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The subnet ID for the new DevEndpoint to use.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDevEndpointResponse {
    /// <p>The AWS availability zone where this DevEndpoint is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The point in time at which this DevEndpoint was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name assigned to the new DevEndpoint.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>Path to one or more Java Jars in an S3 bucket that will be loaded in your DevEndpoint.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p>Path(s) to one or more Python libraries in an S3 bucket that will be loaded in your DevEndpoint.</p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The reason for a current failure in this DevEndpoint.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) allocated to this DevEndpoint.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>The AWS ARN of the role assigned to the new DevEndpoint.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the SecurityConfiguration structure being used with this DevEndpoint.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The security groups assigned to the new DevEndpoint.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The current status of the new DevEndpoint.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet ID assigned to the new DevEndpoint.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The ID of the VPC used by this DevEndpoint.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The address of the YARN endpoint used by this DevEndpoint.</p>
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    /// <p>The Apache Zeppelin port for the remote Apache Spark interpreter.</p>
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i64>,
}

/// <p>Specifies a <code>grok</code> classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGrokClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, Amazon CloudWatch Logs, and so on.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>Optional custom grok patterns used by this classifier.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern used by this classifier.</p>
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,
    /// <p>The name of the new classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// <p>The JobCommand that executes this job.</p>
    #[serde(rename = "Command")]
    pub command: JobCommand,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The default arguments for this job.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>Description of the job being defined.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>AWS Glue supports running jobs on a <code>JobCommand.Name</code>="pythonshell" with allocated processing as low as 0.0625 DPU, which can be specified using <code>MaxCapacity</code>. Glue ETL jobs running in any other way cannot have fractional DPU allocations.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job if it fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The name you assign to this job definition. It must be unique in your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name or ARN of the IAM role associated with this job.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The name of the SecurityConfiguration structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateJobResponse {
    /// <p>The unique name that was provided for this job definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies a JSON classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJsonClassifierRequest {
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of JsonPath, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    pub json_path: String,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePartitionRequest {
    /// <p>The ID of the catalog in which the partion is to be created. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the metadata database in which the partition is to be created.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>PartitionInput</code> structure defining the partition to be created.</p>
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,
    /// <p>The name of the metadata table in which the partition is to be created.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateScriptRequest {
    /// <p>A list of the edges in the DAG.</p>
    #[serde(rename = "DagEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    /// <p>A list of the nodes in the DAG.</p>
    #[serde(rename = "DagNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
    /// <p>The programming language of the resulting code from the DAG.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateScriptResponse {
    /// <p>The Python script generated from the DAG.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    /// <p>The Scala code generated from the DAG.</p>
    #[serde(rename = "ScalaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSecurityConfigurationRequest {
    /// <p>The encryption configuration for the new security configuration.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>The name for the new security configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSecurityConfigurationResponse {
    /// <p>The time at which the new security configuration was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name assigned to the new security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTableRequest {
    /// <p>The ID of the Data Catalog in which to create the <code>Table</code>. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The catalog database in which to create the new table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The <code>TableInput</code> object that defines the metadata table to create in the catalog.</p>
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTriggerRequest {
    /// <p>The actions initiated by this trigger when it fires.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    /// <p>A description of the new trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the trigger.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A predicate to specify when the new trigger should fire.</p> <p>This field is required when the trigger type is CONDITIONAL.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p> <p>This field is required when the trigger type is SCHEDULED.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>Set to true to start SCHEDULED and CONDITIONAL triggers when created. True not supported for ON_DEMAND triggers.</p>
    #[serde(rename = "StartOnCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on_creation: Option<bool>,
    /// <p>The type of the new trigger.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTriggerResponse {
    /// <p>The name of the trigger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog in which to create the function. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which to create the function.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>FunctionInput</code> object that defines the function to create in the Data Catalog.</p>
    #[serde(rename = "FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateUserDefinedFunctionResponse {}

/// <p>Specifies an XML classifier for <code>CreateClassifier</code> to create.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateXMLClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. Note that this cannot identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

/// <p>Contains configuration information for maintaining Data Catalog security.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataCatalogEncryptionSettings {
    /// <p>When password protection is enabled, the Data Catalog uses a customer-provided key to encrypt the password as part of <code>CreateConnection</code> or <code>UpdateConnection</code> and store it in the <code>ENCRYPTED_PASSWORD</code> field in the connection properties. You can enable catalog encryption or only password encryption.</p>
    #[serde(rename = "ConnectionPasswordEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_password_encryption: Option<ConnectionPasswordEncryption>,
    /// <p>Specifies encryption-at-rest configuration for the Data Catalog.</p>
    #[serde(rename = "EncryptionAtRest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
}

/// <p>The <code>Database</code> object represents a logical grouping of tables that may reside in a Hive metastore or an RDBMS.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Database {
    /// <p>The time at which the metadata database was created in the catalog.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>Description of the database.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The location of the database (for example, an HDFS path).</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>Name of the database. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>These key-value pairs define parameters and properties of the database.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The structure used to create or update a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DatabaseInput {
    /// <p>Description of the database</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The location of the database (for example, an HDFS path).</p>
    #[serde(rename = "LocationUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    /// <p>Name of the database. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Thes key-value pairs define parameters and properties of the database.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClassifierRequest {
    /// <p>Name of the classifier to remove.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the connection to delete.</p>
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCrawlerRequest {
    /// <p>Name of the crawler to remove.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the database resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the Database to delete. For Hive compatibility, this must be all lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDevEndpointRequest {
    /// <p>The name of the DevEndpoint.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDevEndpointResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteJobRequest {
    /// <p>The name of the job definition to delete.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteJobResponse {
    /// <p>The name of the job definition that was deleted.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be deleted resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    pub partition_values: Vec<String>,
    /// <p>The name of the table where the partition to be deleted is located.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourcePolicyRequest {
    /// <p>The hash value returned when this policy was set.</p>
    #[serde(rename = "PolicyHashCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteResourcePolicyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSecurityConfigurationRequest {
    /// <p>The name of the security configuration to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSecurityConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table to be deleted. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>The ID of the table version to be deleted. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTableVersionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTriggerRequest {
    /// <p>The name of the trigger to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTriggerResponse {
    /// <p>The name of the trigger that was deleted.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be deleted is located. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the function definition to be deleted.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserDefinedFunctionResponse {}

/// <p>A development endpoint where a developer can remotely debug ETL scripts.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DevEndpoint {
    /// <p>The AWS availability zone where this DevEndpoint is located.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The point in time at which this DevEndpoint was created.</p>
    #[serde(rename = "CreatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    /// <p>The name of the DevEndpoint.</p>
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// <p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p> <p>Please note that only pure Java/Scala libraries can currently be used on a DevEndpoint.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p>Path(s) to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint. Multiple values must be complete paths separated by a comma.</p> <p>Please note that only pure Python libraries can currently be used on a DevEndpoint. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// <p>The reason for a current failure in this DevEndpoint.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The point in time at which this DevEndpoint was last modified.</p>
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    /// <p>The status of the last update.</p>
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    /// <p>The number of AWS Glue Data Processing Units (DPUs) allocated to this DevEndpoint.</p>
    #[serde(rename = "NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i64>,
    /// <p>A private IP address to access the DevEndpoint within a VPC, if the DevEndpoint is created within one. The PrivateAddress field is present only when you create the DevEndpoint within your virtual private cloud (VPC).</p>
    #[serde(rename = "PrivateAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_address: Option<String>,
    /// <p>The public IP address used by this DevEndpoint. The PublicAddress field is present only when you create a non-VPC (virtual private cloud) DevEndpoint.</p>
    #[serde(rename = "PublicAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address: Option<String>,
    /// <p>The public key to be used by this DevEndpoint for authentication. This attribute is provided for backward compatibility, as the recommended attribute to use is public keys.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p><p>A list of public keys to be used by the DevEndpoints for authentication. The use of this attribute is preferred over a single public key because the public keys allow you to have a different private key per client.</p> <note> <p>If you previously created an endpoint with a public key, you must remove that key to be able to set a list of public keys: call the <code>UpdateDevEndpoint</code> API with the public key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the <code>addPublicKeys</code> attribute.</p> </note></p>
    #[serde(rename = "PublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    /// <p>The AWS ARN of the IAM role used in this DevEndpoint.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used with this DevEndpoint.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>A list of security group identifiers used in this DevEndpoint.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The current status of this DevEndpoint.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The subnet ID for this DevEndpoint.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The ID of the virtual private cloud (VPC) used by this DevEndpoint.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// <p>The YARN endpoint address used by this DevEndpoint.</p>
    #[serde(rename = "YarnEndpointAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_endpoint_address: Option<String>,
    /// <p>The Apache Zeppelin port for the remote Apache Spark interpreter.</p>
    #[serde(rename = "ZeppelinRemoteSparkInterpreterPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_remote_spark_interpreter_port: Option<i64>,
}

/// <p>Custom libraries to be loaded into a DevEndpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DevEndpointCustomLibraries {
    /// <p>Path to one or more Java Jars in an S3 bucket that should be loaded in your DevEndpoint.</p> <p>Please note that only pure Java/Scala libraries can currently be used on a DevEndpoint.</p>
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// <p>Path(s) to one or more Python libraries in an S3 bucket that should be loaded in your DevEndpoint. Multiple values must be complete paths separated by a comma.</p> <p>Please note that only pure Python libraries can currently be used on a DevEndpoint. Libraries that rely on C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data analysis library, are not yet supported.</p>
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
}

/// <p>Specifies a DynamoDB table to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamoDBTarget {
    /// <p>The name of the DynamoDB table to crawl.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Specifies encryption-at-rest configuration for the Data Catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionAtRest {
    /// <p>The encryption-at-rest mode for encrypting Data Catalog data.</p>
    #[serde(rename = "CatalogEncryptionMode")]
    pub catalog_encryption_mode: String,
    /// <p>The ID of the AWS KMS key to use for encryption at rest.</p>
    #[serde(rename = "SseAwsKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_aws_kms_key_id: Option<String>,
}

/// <p>Specifies an encryption configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// <p>The encryption configuration for CloudWatch.</p>
    #[serde(rename = "CloudWatchEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,
    /// <p>The encryption configuration for Job Bookmarks.</p>
    #[serde(rename = "JobBookmarksEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,
    /// <p>The encryption configuration for S3 data.</p>
    #[serde(rename = "S3Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption: Option<Vec<S3Encryption>>,
}

/// <p>Contains details about an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ErrorDetail {
    /// <p>The code associated with this error.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message describing the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p>An execution property of a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionProperty {
    /// <p>The maximum number of concurrent runs allowed for the job. The default is 1. An error is returned when this threshold is reached. The maximum value you can specify is controlled by a service limit.</p>
    #[serde(rename = "MaxConcurrentRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCatalogImportStatusRequest {
    /// <p>The ID of the catalog to migrate. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCatalogImportStatusResponse {
    /// <p>The status of the specified catalog migration.</p>
    #[serde(rename = "ImportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<CatalogImportStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClassifierRequest {
    /// <p>Name of the classifier to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetClassifierResponse {
    /// <p>The requested classifier.</p>
    #[serde(rename = "Classifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<Classifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClassifiersRequest {
    /// <p>Size of the list to return (optional).</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An optional continuation token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetClassifiersResponse {
    /// <p>The requested list of classifier objects.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<Classifier>>,
    /// <p>A continuation token.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>Allow you to retrieve the connection metadata without displaying the password. For instance, the AWS Glue console uses this flag to retrieve connections, since the console does not display passwords. Set this parameter where the caller may not have permission to use the KMS key to decrypt the password, but does have permission to access the rest of the connection metadata (that is, the other connection properties).</p>
    #[serde(rename = "HidePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
    /// <p>The name of the connection definition to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConnectionResponse {
    /// <p>The requested connection definition.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

/// <p>Filters the connection definitions returned by the <code>GetConnections</code> API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionsFilter {
    /// <p>The type of connections to return. Currently, only JDBC is supported; SFTP is not supported.</p>
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>A criteria string that must match the criteria recorded in the connection definition for that connection definition to be returned.</p>
    #[serde(rename = "MatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionsRequest {
    /// <p>The ID of the Data Catalog in which the connections reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A filter that controls which connections will be returned.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetConnectionsFilter>,
    /// <p>Allow you to retrieve the connection metadata without displaying the password. For instance, the AWS Glue console uses this flag to retrieve connections, since the console does not display passwords. Set this parameter where the caller may not have permission to use the KMS key to decrypt the password, but does have permission to access the rest of the connection metadata (that is, the other connection properties).</p>
    #[serde(rename = "HidePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_password: Option<bool>,
    /// <p>The maximum number of connections to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConnectionsResponse {
    /// <p>A list of requested connection definitions.</p>
    #[serde(rename = "ConnectionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_list: Option<Vec<Connection>>,
    /// <p>A continuation token, if the list of connections returned does not include the last of the filtered connections.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlerMetricsRequest {
    /// <p>A list of the names of crawlers about which to retrieve metrics.</p>
    #[serde(rename = "CrawlerNameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name_list: Option<Vec<String>>,
    /// <p>The maximum size of a list to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCrawlerMetricsResponse {
    /// <p>A list of metrics for the specified crawler.</p>
    #[serde(rename = "CrawlerMetricsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_metrics_list: Option<Vec<CrawlerMetrics>>,
    /// <p>A continuation token, if the returned list does not contain the last metric available.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlerRequest {
    /// <p>Name of the crawler to retrieve metadata for.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCrawlerResponse {
    /// <p>The metadata for the specified crawler.</p>
    #[serde(rename = "Crawler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler: Option<Crawler>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCrawlersRequest {
    /// <p>The number of crawlers to return on each call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCrawlersResponse {
    /// <p>A list of crawler metadata.</p>
    #[serde(rename = "Crawlers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawlers: Option<Vec<Crawler>>,
    /// <p>A continuation token, if the returned list has not reached the end of those defined in this customer account.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataCatalogEncryptionSettingsRequest {
    /// <p>The ID of the Data Catalog for which to retrieve the security configuration. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDataCatalogEncryptionSettingsResponse {
    /// <p>The requested security configuration.</p>
    #[serde(rename = "DataCatalogEncryptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_encryption_settings: Option<DataCatalogEncryptionSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the database resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database to retrieve. For Hive compatibility, this should be all lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDatabaseResponse {
    /// <p>The definition of the specified database in the catalog.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDatabasesRequest {
    /// <p>The ID of the Data Catalog from which to retrieve <code>Databases</code>. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The maximum number of databases to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDatabasesResponse {
    /// <p>A list of <code>Database</code> objects from the specified catalog.</p>
    #[serde(rename = "DatabaseList")]
    pub database_list: Vec<Database>,
    /// <p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataflowGraphRequest {
    /// <p>The Python script to transform.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDataflowGraphResponse {
    /// <p>A list of the edges in the resulting DAG.</p>
    #[serde(rename = "DagEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_edges: Option<Vec<CodeGenEdge>>,
    /// <p>A list of the nodes in the resulting DAG.</p>
    #[serde(rename = "DagNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dag_nodes: Option<Vec<CodeGenNode>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevEndpointRequest {
    /// <p>Name of the DevEndpoint for which to retrieve information.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDevEndpointResponse {
    /// <p>A DevEndpoint definition.</p>
    #[serde(rename = "DevEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoint: Option<DevEndpoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevEndpointsRequest {
    /// <p>The maximum size of information to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDevEndpointsResponse {
    /// <p>A list of DevEndpoint definitions.</p>
    #[serde(rename = "DevEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_endpoints: Option<Vec<DevEndpoint>>,
    /// <p>A continuation token, if not all DevEndpoint definitions have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRequest {
    /// <p>The name of the job definition to retrieve.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobResponse {
    /// <p>The requested job definition.</p>
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRunRequest {
    /// <p>Name of the job definition being run.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>True if a list of predecessor runs should be returned.</p>
    #[serde(rename = "PredecessorsIncluded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessors_included: Option<bool>,
    /// <p>The ID of the job run.</p>
    #[serde(rename = "RunId")]
    pub run_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobRunResponse {
    /// <p>The requested job-run metadata.</p>
    #[serde(rename = "JobRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRunsRequest {
    /// <p>The name of the job definition for which to retrieve all job runs.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobRunsResponse {
    /// <p>A list of job-run metatdata objects.</p>
    #[serde(rename = "JobRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    /// <p>A continuation token, if not all reequested job runs have been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobsRequest {
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobsResponse {
    /// <p>A list of job definitions.</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>A continuation token, if not all job definitions have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMappingRequest {
    /// <p>Parameters for the mapping.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>A list of target tables.</p>
    #[serde(rename = "Sinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    /// <p>Specifies the source table.</p>
    #[serde(rename = "Source")]
    pub source: CatalogEntry,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetMappingResponse {
    /// <p>A list of mappings to the specified targets.</p>
    #[serde(rename = "Mapping")]
    pub mapping: Vec<MappingEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPartitionRequest {
    /// <p>The ID of the Data Catalog where the partition in question resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partition resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    pub partition_values: Vec<String>,
    /// <p>The name of the partition's table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPartitionResponse {
    /// <p>The requested information, in the form of a <code>Partition</code> object.</p>
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPartitionsRequest {
    /// <p>The ID of the Data Catalog where the partitions in question reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the partitions reside.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>An expression filtering the partitions to be returned.</p> <p>The expression uses SQL syntax similar to the SQL <code>WHERE</code> filter clause. The SQL statement parser <a href="http://jsqlparser.sourceforge.net/home.php">JSQLParser</a> parses the expression. </p> <p> <i>Operators</i>: The following are the operators that you can use in the <code>Expression</code> API call:</p> <dl> <dt>=</dt> <dd> <p>Checks if the values of the two operands are equal or not; if yes, then the condition becomes true.</p> <p>Example: Assume 'variable a' holds 10 and 'variable b' holds 20. </p> <p>(a = b) is not true.</p> </dd> <dt>&lt; &gt;</dt> <dd> <p>Checks if the values of two operands are equal or not; if the values are not equal, then the condition becomes true.</p> <p>Example: (a &lt; &gt; b) is true.</p> </dd> <dt>&gt;</dt> <dd> <p>Checks if the value of the left operand is greater than the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &gt; b) is not true.</p> </dd> <dt>&lt;</dt> <dd> <p>Checks if the value of the left operand is less than the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &lt; b) is true.</p> </dd> <dt>&gt;=</dt> <dd> <p>Checks if the value of the left operand is greater than or equal to the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &gt;= b) is not true.</p> </dd> <dt>&lt;=</dt> <dd> <p>Checks if the value of the left operand is less than or equal to the value of the right operand; if yes, then the condition becomes true.</p> <p>Example: (a &lt;= b) is true.</p> </dd> <dt>AND, OR, IN, BETWEEN, LIKE, NOT, IS NULL</dt> <dd> <p>Logical operators.</p> </dd> </dl> <p> <i>Supported Partition Key Types</i>: The following are the the supported partition keys.</p> <ul> <li> <p> <code>string</code> </p> </li> <li> <p> <code>date</code> </p> </li> <li> <p> <code>timestamp</code> </p> </li> <li> <p> <code>int</code> </p> </li> <li> <p> <code>bigint</code> </p> </li> <li> <p> <code>long</code> </p> </li> <li> <p> <code>tinyint</code> </p> </li> <li> <p> <code>smallint</code> </p> </li> <li> <p> <code>decimal</code> </p> </li> </ul> <p>If an invalid type is encountered, an exception is thrown. </p> <p>The following list shows the valid operators on each type. When you define a crawler, the <code>partitionKey</code> type is created as a <code>STRING</code>, to be compatible with the catalog partitions. </p> <p> <i>Sample API Call</i>: </p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The maximum number of partitions to return in a single response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call to retrieve these partitions.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The segment of the table's partitions to scan in this request.</p>
    #[serde(rename = "Segment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    /// <p>The name of the partitions' table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPartitionsResponse {
    /// <p>A continuation token, if the returned list of partitions does not does not include the last one.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of requested partitions.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPlanRequest {
    /// <p>The programming language of the code to perform the mapping.</p>
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>Parameters for the mapping.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>The list of mappings from a source table to target tables.</p>
    #[serde(rename = "Mapping")]
    pub mapping: Vec<MappingEntry>,
    /// <p>The target tables.</p>
    #[serde(rename = "Sinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinks: Option<Vec<CatalogEntry>>,
    /// <p>The source table.</p>
    #[serde(rename = "Source")]
    pub source: CatalogEntry,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPlanResponse {
    /// <p>A Python script to perform the mapping.</p>
    #[serde(rename = "PythonScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_script: Option<String>,
    /// <p>Scala code to perform the mapping.</p>
    #[serde(rename = "ScalaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scala_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcePolicyRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetResourcePolicyResponse {
    /// <p>The date and time at which the policy was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>Contains the hash value associated with this policy.</p>
    #[serde(rename = "PolicyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    /// <p>Contains the requested policy document, in JSON format.</p>
    #[serde(rename = "PolicyInJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_in_json: Option<String>,
    /// <p>The date and time at which the policy was last updated.</p>
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSecurityConfigurationRequest {
    /// <p>The name of the security configuration to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSecurityConfigurationResponse {
    /// <p>The requested security configuration</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<SecurityConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSecurityConfigurationsRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSecurityConfigurationsResponse {
    /// <p>A continuation token, if there are more security configurations to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of security configurations.</p>
    #[serde(rename = "SecurityConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table for which to retrieve the definition. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTableResponse {
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableVersionRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>The ID value of the table version to be retrieved. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1. </p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTableVersionResponse {
    /// <p>The requested table version.</p>
    #[serde(rename = "TableVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_version: Option<TableVersion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTableVersionsRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The maximum number of table versions to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is not the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTableVersionsResponse {
    /// <p>A continuation token, if the list of available versions does not include the last one.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of strings identifying available versions of the specified table.</p>
    #[serde(rename = "TableVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_versions: Option<Vec<TableVersion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTablesRequest {
    /// <p>The ID of the Data Catalog where the tables reside. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The database in the catalog whose tables to list. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A regular expression pattern. If present, only those tables whose names match the pattern are returned.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The maximum number of tables to return in a single response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, included if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTablesResponse {
    /// <p>A continuation token, present if the current list segment is not the last.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the requested <code>Table</code> objects.</p>
    #[serde(rename = "TableList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Table>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTriggerRequest {
    /// <p>The name of the trigger to retrieve.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTriggerResponse {
    /// <p>The requested trigger definition.</p>
    #[serde(rename = "Trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTriggersRequest {
    /// <p>The name of the job for which to retrieve triggers. The trigger that can start this job will be returned, and if there is no such trigger, all triggers will be returned.</p>
    #[serde(rename = "DependentJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_job_name: Option<String>,
    /// <p>The maximum size of the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTriggersResponse {
    /// <p>A continuation token, if not all the requested triggers have yet been returned.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of triggers for the specified job.</p>
    #[serde(rename = "Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be retrieved is located. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserDefinedFunctionResponse {
    /// <p>The requested function definition.</p>
    #[serde(rename = "UserDefinedFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_function: Option<UserDefinedFunction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserDefinedFunctionsRequest {
    /// <p>The ID of the Data Catalog where the functions to be retrieved are located. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the functions are located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The maximum number of functions to return in one response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional function-name pattern string that filters the function definitions returned.</p>
    #[serde(rename = "Pattern")]
    pub pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserDefinedFunctionsResponse {
    /// <p>A continuation token, if the list of functions returned does not include the last requested function.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of requested function definitions.</p>
    #[serde(rename = "UserDefinedFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_functions: Option<Vec<UserDefinedFunction>>,
}

/// <p>A classifier that uses <code>grok</code> patterns.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GrokClassifier {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, and so on.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The time this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Optional custom grok patterns defined by this classifier. For more information, see custom patterns in <a href="http://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html">Writing Custom Classifers</a>.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern applied to a data store by this classifier. For more information, see built-in patterns in <a href="http://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html">Writing Custom Classifers</a>.</p>
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,
    /// <p>The time this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportCatalogToGlueRequest {
    /// <p>The ID of the catalog to import. Currently, this should be the AWS account ID.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportCatalogToGlueResponse {}

/// <p>Specifies a JDBC data store to crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JdbcTarget {
    /// <p>The name of the connection to use to connect to the JDBC target.</p>
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    /// <p>A list of glob patterns used to exclude from the crawl. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/add-crawler.html">Catalog Tables with a Crawler</a>.</p>
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    /// <p>The path of the JDBC target.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>Specifies a job definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Job {
    /// <p>The JobCommand that executes this job.</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The time and date that this job definition was created.</p>
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>The default arguments for this job, specified as name-value pairs.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>Description of the job being defined.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>The last point in time when this job definition was modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>AWS Glue supports running jobs on a <code>JobCommand.Name</code>="pythonshell" with allocated processing as low as 0.0625 DPU, which can be specified using <code>MaxCapacity</code>. Glue ETL jobs running in any other way cannot have fractional DPU allocations.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job after a JobRun fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>The name you assign to this job definition.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name or ARN of the IAM role associated with this job.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>Defines a point which a job can resume processing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobBookmarkEntry {
    /// <p>The attempt ID number.</p>
    #[serde(rename = "Attempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// <p>The bookmark itself.</p>
    #[serde(rename = "JobBookmark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark: Option<String>,
    /// <p>Name of the job in question.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The run ID number.</p>
    #[serde(rename = "Run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<i64>,
    /// <p>Version of the job.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies how Job bookmark data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobBookmarksEncryption {
    /// <p>The encryption mode to use for Job bookmarks data.</p>
    #[serde(rename = "JobBookmarksEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmarks_encryption_mode: Option<String>,
    /// <p>The AWS ARN of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Specifies code executed when a job is run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobCommand {
    /// <p>The name of the job command: this must be <code>glueetl</code>, for an Apache Spark ETL job, or <code>pythonshell</code>, for a Python shell job.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the S3 path to a script that executes a job (required).</p>
    #[serde(rename = "ScriptLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_location: Option<String>,
}

/// <p>Contains information about a job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobRun {
    /// <p>The job arguments associated with this run. These override equivalent default arguments set for the job.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The number of the attempt to run this job.</p>
    #[serde(rename = "Attempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// <p>The date and time this job run completed.</p>
    #[serde(rename = "CompletedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<f64>,
    /// <p>An error message associated with this job run.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The amount of time (in seconds) that the job run consumed resources.</p>
    #[serde(rename = "ExecutionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i64>,
    /// <p>The ID of this job run.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the job definition being used in this run.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current state of the job run.</p>
    #[serde(rename = "JobRunState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_state: Option<String>,
    /// <p>The last time this job run was modified.</p>
    #[serde(rename = "LastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The name of the log group for secure logging, that can be server-side encrypted in CloudWatch using KMS. This name can be <code>/aws-glue/jobs/</code>, in which case the default encryption is <code>NONE</code>. If you add a role name and SecurityConfiguration name (in other words, <code>/aws-glue/jobs-yourRoleName-yourSecurityConfigurationName/</code>), then that security configuration will be used to encrypt the log group.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>AWS Glue supports running jobs on a <code>JobCommand.Name</code>="pythonshell" with allocated processing as low as 0.0625 DPU, which can be specified using <code>MaxCapacity</code>. Glue ETL jobs running in any other way cannot have fractional DPU allocations.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>A list of predecessors to this job run.</p>
    #[serde(rename = "PredecessorRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessor_runs: Option<Vec<Predecessor>>,
    /// <p>The ID of the previous run of this job. For example, the JobRunId specified in the StartJobRun action.</p>
    #[serde(rename = "PreviousRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_run_id: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used with this job run.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The date and time at which this job run was started.</p>
    #[serde(rename = "StartedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<f64>,
    /// <p>The JobRun timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The name of the trigger that started this job run.</p>
    #[serde(rename = "TriggerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
}

/// <p>Specifies information used to update an existing job definition. Note that the previous job definition will be completely overwritten by this information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct JobUpdate {
    /// <p>The JobCommand that executes this job (required).</p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<JobCommand>,
    /// <p>The connections used for this job.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsList>,
    /// <p>The default arguments for this job.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>Description of the job being defined.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An ExecutionProperty specifying the maximum number of concurrent runs allowed for this job.</p>
    #[serde(rename = "ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<ExecutionProperty>,
    /// <p>This field is reserved for future use.</p>
    #[serde(rename = "LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// <p>AWS Glue supports running jobs on a <code>JobCommand.Name</code>="pythonshell" with allocated processing as low as 0.0625 DPU, which can be specified using <code>MaxCapacity</code>. Glue ETL jobs running in any other way cannot have fractional DPU allocations.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>The maximum number of times to retry this job if it fails.</p>
    #[serde(rename = "MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// <p>Specifies configuration properties of a job notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name or ARN of the IAM role associated with this job (required).</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used with this job.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The job timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>A classifier for <code>JSON</code> content.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JsonClassifier {
    /// <p>The time this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of JsonPath, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    pub json_path: String,
    /// <p>The time this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Status and error information about the most recent crawl.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LastCrawlInfo {
    /// <p>If an error occurred, the error information about the last crawl.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The log group for the last crawl.</p>
    #[serde(rename = "LogGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// <p>The log stream for the last crawl.</p>
    #[serde(rename = "LogStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream: Option<String>,
    /// <p>The prefix for a message about this crawl.</p>
    #[serde(rename = "MessagePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_prefix: Option<String>,
    /// <p>The time at which the crawl started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Status of the last crawl.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The location of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Location {
    /// <p>A DynamoDB Table location.</p>
    #[serde(rename = "DynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<Vec<CodeGenNodeArg>>,
    /// <p>A JDBC location.</p>
    #[serde(rename = "Jdbc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc: Option<Vec<CodeGenNodeArg>>,
    /// <p>An Amazon S3 location.</p>
    #[serde(rename = "S3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<Vec<CodeGenNodeArg>>,
}

/// <p>Defines a mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingEntry {
    /// <p>The source path.</p>
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    /// <p>The name of the source table.</p>
    #[serde(rename = "SourceTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table: Option<String>,
    /// <p>The source type.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The target path.</p>
    #[serde(rename = "TargetPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    /// <p>The target table.</p>
    #[serde(rename = "TargetTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_table: Option<String>,
    /// <p>The target type.</p>
    #[serde(rename = "TargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

/// <p>Specifies configuration properties of a notification.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationProperty {
    /// <p>After a job run starts, the number of minutes to wait before sending a job run delay notification.</p>
    #[serde(rename = "NotifyDelayAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_delay_after: Option<i64>,
}

/// <p>Specifies the sort order of a sorted column.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// <p>The name of the column.</p>
    #[serde(rename = "Column")]
    pub column: String,
    /// <p>Indicates that the column is sorted in ascending order (<code>== 1</code>), or in descending order (<code>==0</code>).</p>
    #[serde(rename = "SortOrder")]
    pub sort_order: i64,
}

/// <p>Represents a slice of table data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Partition {
    /// <p>The time at which the partition was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the catalog database where the table in question is located.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The last time at which the partition was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time at which column statistics were computed for this partition.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>These key-value pairs define partition parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Provides information about the physical location where the partition is stored.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The name of the table in question.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The values of the partition.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Contains information about a partition error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PartitionError {
    /// <p>Details about the partition error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The values that define the partition.</p>
    #[serde(rename = "PartitionValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
}

/// <p>The structure used to create and update a partion.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PartitionInput {
    /// <p>The last time at which the partition was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The last time at which column statistics were computed for this partition.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>These key-value pairs define partition parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Provides information about the physical location where the partition is stored.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The values of the partition.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Contains a list of values defining partitions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartitionValueList {
    /// <p>The list of values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies the physical requirements for a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalConnectionRequirements {
    /// <p>The connection's availability zone. This field is redundant, since the specified subnet implies the availability zone to be used. The field must be populated now, but will be deprecated in the future.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The security group ID list used by the connection.</p>
    #[serde(rename = "SecurityGroupIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_list: Option<Vec<String>>,
    /// <p>The subnet ID used by the connection.</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

/// <p>A job run that was used in the predicate of a conditional trigger that triggered this job run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Predecessor {
    /// <p>The name of the job definition used by the predecessor job run.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The job-run ID of the predecessor job run.</p>
    #[serde(rename = "RunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

/// <p>Defines the predicate of the trigger, which determines when it fires.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Predicate {
    /// <p>A list of the conditions that determine when the trigger will fire.</p>
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// <p>Optional field if only one condition is listed. If multiple conditions are listed, then this field is required.</p>
    #[serde(rename = "Logical")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDataCatalogEncryptionSettingsRequest {
    /// <p>The ID of the Data Catalog for which to set the security configuration. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The security configuration to set.</p>
    #[serde(rename = "DataCatalogEncryptionSettings")]
    pub data_catalog_encryption_settings: DataCatalogEncryptionSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutDataCatalogEncryptionSettingsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutResourcePolicyRequest {
    /// <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a null value is used, the call will not depend on the existence of a policy.</p>
    #[serde(rename = "PolicyExistsCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_exists_condition: Option<String>,
    /// <p>This is the hash value returned when the previous policy was set using PutResourcePolicy. Its purpose is to prevent concurrent modifications of a policy. Do not use this parameter if no previous policy has been set.</p>
    #[serde(rename = "PolicyHashCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash_condition: Option<String>,
    /// <p>Contains the policy document to set, in JSON format.</p>
    #[serde(rename = "PolicyInJson")]
    pub policy_in_json: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutResourcePolicyResponse {
    /// <p>A hash of the policy that has just been set. This must be included in a subsequent call that overwrites or updates this policy.</p>
    #[serde(rename = "PolicyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResetJobBookmarkRequest {
    /// <p>The name of the job in question.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResetJobBookmarkResponse {
    /// <p>The reset bookmark entry.</p>
    #[serde(rename = "JobBookmarkEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_bookmark_entry: Option<JobBookmarkEntry>,
}

/// <p>URIs for function resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUri {
    /// <p>The type of the resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The URI for accessing the resource.</p>
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Specifies how S3 data should be encrypted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Encryption {
    /// <p>The AWS ARN of the KMS key to be used to encrypt the data.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The encryption mode to use for S3 data.</p>
    #[serde(rename = "S3EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_mode: Option<String>,
}

/// <p>Specifies a data store in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Target {
    /// <p>A list of glob patterns used to exclude from the crawl. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/add-crawler.html">Catalog Tables with a Crawler</a>.</p>
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
    /// <p>The path to the Amazon S3 target.</p>
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// <p>A scheduling object using a <code>cron</code> statement to schedule an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Schedule {
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// <p>The state of the schedule.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Crawler policy for update and deletion behavior.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaChangePolicy {
    /// <p>The deletion behavior when the crawler finds a deleted object.</p>
    #[serde(rename = "DeleteBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_behavior: Option<String>,
    /// <p>The update behavior when the crawler finds a changed schema.</p>
    #[serde(rename = "UpdateBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<String>,
}

/// <p>Specifies a security configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityConfiguration {
    /// <p>The time at which this security configuration was created.</p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p>The encryption configuration associated with this security configuration.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The name of the security configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Defines a non-overlapping region of a table's partitions, allowing multiple requests to be executed in parallel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Segment {
    /// <p>The zero-based index number of the this segment. For example, if the total number of segments is 4, SegmentNumber values will range from zero through three.</p>
    #[serde(rename = "SegmentNumber")]
    pub segment_number: i64,
    /// <p>The total numer of segments.</p>
    #[serde(rename = "TotalSegments")]
    pub total_segments: i64,
}

/// <p>Information about a serialization/deserialization program (SerDe) which serves as an extractor and loader.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerDeInfo {
    /// <p>Name of the SerDe.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>These key-value pairs define initialization parameters for the SerDe.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Usually the class that implements the SerDe. An example is: <code>org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe</code>.</p>
    #[serde(rename = "SerializationLibrary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_library: Option<String>,
}

/// <p>Specifies skewed values in a table. Skewed are ones that occur with very high frequency.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkewedInfo {
    /// <p>A list of names of columns that contain skewed values.</p>
    #[serde(rename = "SkewedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_names: Option<Vec<String>>,
    /// <p>A mapping of skewed values to the columns that contain them.</p>
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_value_location_maps: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of values that appear so frequently as to be considered skewed.</p>
    #[serde(rename = "SkewedColumnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCrawlerRequest {
    /// <p>Name of the crawler to start.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCrawlerScheduleRequest {
    /// <p>Name of the crawler to schedule.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartCrawlerScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartJobRunRequest {
    /// <p>The job arguments specifically for this run. They override the equivalent default arguments set for in the job definition itself.</p> <p>You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.</p> <p>For information about how to specify and consume your own Job arguments, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling AWS Glue APIs in Python</a> topic in the developer guide.</p> <p>For information about the key-value pairs that AWS Glue consumes to set up your job, see the <a href="http://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by AWS Glue</a> topic in the developer guide.</p>
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the job definition to use.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>The ID of a previous JobRun to retry.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
    /// <p>AWS Glue supports running jobs on a <code>JobCommand.Name</code>="pythonshell" with allocated processing as low as 0.0625 DPU, which can be specified using <code>MaxCapacity</code>. Glue ETL jobs running in any other way cannot have fractional DPU allocations.</p>
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    /// <p>Specifies configuration properties of a job run notification.</p>
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,
    /// <p>The name of the SecurityConfiguration structure to be used with this job run.</p>
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<String>,
    /// <p>The JobRun timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartJobRunResponse {
    /// <p>The ID assigned to this job run.</p>
    #[serde(rename = "JobRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTriggerRequest {
    /// <p>The name of the trigger to start.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartTriggerResponse {
    /// <p>The name of the trigger that was started.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopCrawlerRequest {
    /// <p>Name of the crawler to stop.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopCrawlerScheduleRequest {
    /// <p>Name of the crawler whose schedule state to set.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopCrawlerScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTriggerRequest {
    /// <p>The name of the trigger to stop.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopTriggerResponse {
    /// <p>The name of the trigger that was stopped.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes the physical storage of table data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageDescriptor {
    /// <p>A list of reducer grouping columns, clustering columns, and bucketing columns in the table.</p>
    #[serde(rename = "BucketColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_columns: Option<Vec<String>>,
    /// <p>A list of the <code>Columns</code> in the table.</p>
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    /// <p>True if the data in the table is compressed, or False if not.</p>
    #[serde(rename = "Compressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    /// <p>The input format: <code>SequenceFileInputFormat</code> (binary), or <code>TextInputFormat</code>, or a custom format.</p>
    #[serde(rename = "InputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    /// <p>The physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>Must be specified if the table contains any dimension columns.</p>
    #[serde(rename = "NumberOfBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_buckets: Option<i64>,
    /// <p>The output format: <code>SequenceFileOutputFormat</code> (binary), or <code>IgnoreKeyTextOutputFormat</code>, or a custom format.</p>
    #[serde(rename = "OutputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// <p>User-supplied properties in key-value form.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Serialization/deserialization (SerDe) information.</p>
    #[serde(rename = "SerdeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde_info: Option<SerDeInfo>,
    /// <p>Information about values that appear very frequently in a column (skewed values).</p>
    #[serde(rename = "SkewedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_info: Option<SkewedInfo>,
    /// <p>A list specifying the sort order of each bucket in the table.</p>
    #[serde(rename = "SortColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_columns: Option<Vec<Order>>,
    /// <p>True if the table data is stored in subdirectories, or False if not.</p>
    #[serde(rename = "StoredAsSubDirectories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_as_sub_directories: Option<bool>,
}

/// <p>Represents a collection of related data organized in columns and rows.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Table {
    /// <p>Time when the table definition was created in the Data Catalog.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>Person or entity who created the table.</p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>Name of the metadata database where the table metadata resides. For Hive compatibility, this must be all lowercase.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Description of the table.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Last time the table was accessed. This is usually taken from HDFS, and may not be reliable.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>Last time column statistics were computed for this table.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>Name of the table. For Hive compatibility, this must be entirely lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Owner of the table.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>These key-value pairs define properties associated with the table.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p> <p>When creating a table used by Athena, and you do not specify any <code>partitionKeys</code>, you must at least set the value of <code>partitionKeys</code> to an empty list. For example:</p> <p> <code>"PartitionKeys": []</code> </p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    /// <p>Retention time for this table.</p>
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i64>,
    /// <p>A storage descriptor containing information about the physical storage of this table.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>
    #[serde(rename = "TableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    /// <p>Last time the table was updated.</p>
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    /// <p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewExpandedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    /// <p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewOriginalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

/// <p>An error record for table operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TableError {
    /// <p>Detail about the error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>Name of the table. For Hive compatibility, this must be entirely lowercase.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>Structure used to create or update the table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TableInput {
    /// <p>Description of the table.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Last time the table was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>Last time column statistics were computed for this table.</p>
    #[serde(rename = "LastAnalyzedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_time: Option<f64>,
    /// <p>Name of the table. For Hive compatibility, this is folded to lowercase when it is stored.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Owner of the table.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>These key-value pairs define properties associated with the table.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of columns by which the table is partitioned. Only primitive types are supported as partition keys.</p> <p>When creating a table used by Athena, and you do not specify any <code>partitionKeys</code>, you must at least set the value of <code>partitionKeys</code> to an empty list. For example:</p> <p> <code>"PartitionKeys": []</code> </p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    /// <p>Retention time for this table.</p>
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i64>,
    /// <p>A storage descriptor containing information about the physical storage of this table.</p>
    #[serde(rename = "StorageDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,
    /// <p>The type of this table (<code>EXTERNAL_TABLE</code>, <code>VIRTUAL_VIEW</code>, etc.).</p>
    #[serde(rename = "TableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
    /// <p>If the table is a view, the expanded text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewExpandedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_expanded_text: Option<String>,
    /// <p>If the table is a view, the original text of the view; otherwise <code>null</code>.</p>
    #[serde(rename = "ViewOriginalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_original_text: Option<String>,
}

/// <p>Specifies a version of a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TableVersion {
    /// <p>The table in question</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    /// <p>The ID value that identifies this table version. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>An error record for table-version operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TableVersionError {
    /// <p>Detail about the error.</p>
    #[serde(rename = "ErrorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    /// <p>The name of the table in question.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The ID value of the version in question. A <code>VersionID</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Information about a specific trigger.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Trigger {
    /// <p>The actions initiated by this trigger.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>A description of this trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Name of the trigger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The predicate of this trigger, which defines when it will fire.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The current state of the trigger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The type of trigger that this is.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A structure used to provide information used to update a trigger. This object will update the the previous trigger definition by overwriting it completely.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TriggerUpdate {
    /// <p>The actions initiated by this trigger.</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>A description of this trigger.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The predicate of this trigger, which defines when it will fire.</p>
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClassifierRequest {
    /// <p>A <code>GrokClassifier</code> object with updated fields.</p>
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<UpdateGrokClassifierRequest>,
    /// <p>A <code>JsonClassifier</code> object with updated fields.</p>
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<UpdateJsonClassifierRequest>,
    /// <p>An <code>XMLClassifier</code> object with updated fields.</p>
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_classifier: Option<UpdateXMLClassifierRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConnectionRequest {
    /// <p>The ID of the Data Catalog in which the connection resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>ConnectionInput</code> object that redefines the connection in question.</p>
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,
    /// <p>The name of the connection definition to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateConnectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCrawlerRequest {
    /// <p>A list of custom classifiers that the user has registered. By default, all built-in classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// <p>Crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="http://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Configuring a Crawler</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The name of the SecurityConfiguration structure to be used by this Crawler.</p>
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<String>,
    /// <p>The AWS Glue database where results are stored, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>A description of the new crawler.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new crawler.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The IAM role (or ARN of an IAM role) used by the new crawler to access customer resources.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>A <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>Policy for the crawler's update and deletion behavior.</p>
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// <p>The table prefix used for catalog tables that are created.</p>
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
    /// <p>A list of targets to crawl.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CrawlerTargets>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCrawlerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCrawlerScheduleRequest {
    /// <p>Name of the crawler whose schedule to update.</p>
    #[serde(rename = "CrawlerName")]
    pub crawler_name: String,
    /// <p>The updated <code>cron</code> expression used to specify the schedule (see <a href="http://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCrawlerScheduleResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatabaseRequest {
    /// <p>The ID of the Data Catalog in which the metadata database resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>A <code>DatabaseInput</code> object specifying the new definition of the metadata database in the catalog.</p>
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,
    /// <p>The name of the database to update in the catalog. For Hive compatibility, this is folded to lowercase.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDatabaseResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDevEndpointRequest {
    /// <p>The list of public keys for the DevEndpoint to use.</p>
    #[serde(rename = "AddPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_public_keys: Option<Vec<String>>,
    /// <p>Custom Python or Java libraries to be loaded in the DevEndpoint.</p>
    #[serde(rename = "CustomLibraries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_libraries: Option<DevEndpointCustomLibraries>,
    /// <p>The list of public keys to be deleted from the DevEndpoint.</p>
    #[serde(rename = "DeletePublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_public_keys: Option<Vec<String>>,
    /// <p>The name of the DevEndpoint to be updated.</p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The public key for the DevEndpoint to use.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p>True if the list of custom libraries to be loaded in the development endpoint needs to be updated, or False otherwise.</p>
    #[serde(rename = "UpdateEtlLibraries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_etl_libraries: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDevEndpointResponse {}

/// <p>Specifies a grok classifier to update when passed to <code>UpdateClassifier</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGrokClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, Amazon CloudWatch Logs, and so on.</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>Optional custom grok patterns used by this classifier.</p>
    #[serde(rename = "CustomPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<String>,
    /// <p>The grok pattern used by this classifier.</p>
    #[serde(rename = "GrokPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_pattern: Option<String>,
    /// <p>The name of the <code>GrokClassifier</code>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobRequest {
    /// <p>Name of the job definition to update.</p>
    #[serde(rename = "JobName")]
    pub job_name: String,
    /// <p>Specifies the values with which to update the job definition.</p>
    #[serde(rename = "JobUpdate")]
    pub job_update: JobUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateJobResponse {
    /// <p>Returns the name of the updated job definition.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

/// <p>Specifies a JSON classifier to be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJsonClassifierRequest {
    /// <p>A <code>JsonPath</code> string defining the JSON data for the classifier to classify. AWS Glue supports a subset of JsonPath, as described in <a href="https://docs.aws.amazon.com/glue/latest/dg/custom-classifier.html#custom-classifier-json">Writing JsonPath Custom Classifiers</a>.</p>
    #[serde(rename = "JsonPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePartitionRequest {
    /// <p>The ID of the Data Catalog where the partition to be updated resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table in question resides.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The new partition object to which to update the partition.</p>
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,
    /// <p>A list of the values defining the partition.</p>
    #[serde(rename = "PartitionValueList")]
    pub partition_value_list: Vec<String>,
    /// <p>The name of the table where the partition to be updated is located.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePartitionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTableRequest {
    /// <p>The ID of the Data Catalog where the table resides. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>By default, <code>UpdateTable</code> always creates an archived version of the table before updating it. If <code>skipArchive</code> is set to true, however, <code>UpdateTable</code> does not create the archived version.</p>
    #[serde(rename = "SkipArchive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_archive: Option<bool>,
    /// <p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateTableResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTriggerRequest {
    /// <p>The name of the trigger to update.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The new values with which to update the trigger.</p>
    #[serde(rename = "TriggerUpdate")]
    pub trigger_update: TriggerUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateTriggerResponse {
    /// <p>The resulting trigger definition.</p>
    #[serde(rename = "Trigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUserDefinedFunctionRequest {
    /// <p>The ID of the Data Catalog where the function to be updated is located. If none is supplied, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>The name of the catalog database where the function to be updated is located.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A <code>FunctionInput</code> object that re-defines the function in the Data Catalog.</p>
    #[serde(rename = "FunctionInput")]
    pub function_input: UserDefinedFunctionInput,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateUserDefinedFunctionResponse {}

/// <p>Specifies an XML classifier to be updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateXMLClassifierRequest {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. Note that this cannot identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
}

/// <p>Represents the equivalent of a Hive user-defined function (<code>UDF</code>) definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UserDefinedFunction {
    /// <p>The Java class that contains the function code.</p>
    #[serde(rename = "ClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// <p>The time at which the function was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The owner of the function.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>The owner type.</p>
    #[serde(rename = "OwnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    /// <p>The resource URIs for the function.</p>
    #[serde(rename = "ResourceUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

/// <p>A structure used to create or updata a user-defined function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UserDefinedFunctionInput {
    /// <p>The Java class that contains the function code.</p>
    #[serde(rename = "ClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The owner of the function.</p>
    #[serde(rename = "OwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// <p>The owner type.</p>
    #[serde(rename = "OwnerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    /// <p>The resource URIs for the function.</p>
    #[serde(rename = "ResourceUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uris: Option<Vec<ResourceUri>>,
}

/// <p>A classifier for <code>XML</code> content.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct XMLClassifier {
    /// <p>An identifier of the data format that the classifier matches.</p>
    #[serde(rename = "Classification")]
    pub classification: String,
    /// <p>The time this classifier was registered.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time this classifier was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>The name of the classifier.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The XML tag designating the element that contains each record in an XML document being parsed. Note that this cannot identify a self-closing element (closed by <code>/&gt;</code>). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, <code>&lt;row item_a="A" item_b="B"&gt;&lt;/row&gt;</code> is okay, but <code>&lt;row item_a="A" item_b="B" /&gt;</code> is not).</p>
    #[serde(rename = "RowTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_tag: Option<String>,
    /// <p>The version of this classifier.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// Errors returned by BatchCreatePartition
#[derive(Debug, PartialEq)]
pub enum BatchCreatePartitionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl BatchCreatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(BatchCreatePartitionError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchCreatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(BatchCreatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchCreatePartitionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchCreatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchCreatePartitionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        BatchCreatePartitionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchCreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchCreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchCreatePartitionError::AlreadyExists(ref cause) => cause,
            BatchCreatePartitionError::EntityNotFound(ref cause) => cause,
            BatchCreatePartitionError::GlueEncryption(ref cause) => cause,
            BatchCreatePartitionError::InternalService(ref cause) => cause,
            BatchCreatePartitionError::InvalidInput(ref cause) => cause,
            BatchCreatePartitionError::OperationTimeout(ref cause) => cause,
            BatchCreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteConnection
#[derive(Debug, PartialEq)]
pub enum BatchDeleteConnectionError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteConnectionError::InternalService(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteConnectionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteConnectionError::InternalService(ref cause) => cause,
            BatchDeleteConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeletePartition
#[derive(Debug, PartialEq)]
pub enum BatchDeletePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeletePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeletePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeletePartitionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeletePartitionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeletePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeletePartitionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeletePartitionError::EntityNotFound(ref cause) => cause,
            BatchDeletePartitionError::InternalService(ref cause) => cause,
            BatchDeletePartitionError::InvalidInput(ref cause) => cause,
            BatchDeletePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteTable
#[derive(Debug, PartialEq)]
pub enum BatchDeleteTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeleteTableError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeleteTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteTableError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteTableError::EntityNotFound(ref cause) => cause,
            BatchDeleteTableError::InternalService(ref cause) => cause,
            BatchDeleteTableError::InvalidInput(ref cause) => cause,
            BatchDeleteTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchDeleteTableVersion
#[derive(Debug, PartialEq)]
pub enum BatchDeleteTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchDeleteTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchDeleteTableVersionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDeleteTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDeleteTableVersionError {
    fn description(&self) -> &str {
        match *self {
            BatchDeleteTableVersionError::EntityNotFound(ref cause) => cause,
            BatchDeleteTableVersionError::InternalService(ref cause) => cause,
            BatchDeleteTableVersionError::InvalidInput(ref cause) => cause,
            BatchDeleteTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetPartition
#[derive(Debug, PartialEq)]
pub enum BatchGetPartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl BatchGetPartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetPartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(BatchGetPartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(BatchGetPartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(BatchGetPartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(BatchGetPartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(BatchGetPartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetPartitionError {
    fn description(&self) -> &str {
        match *self {
            BatchGetPartitionError::EntityNotFound(ref cause) => cause,
            BatchGetPartitionError::GlueEncryption(ref cause) => cause,
            BatchGetPartitionError::InternalService(ref cause) => cause,
            BatchGetPartitionError::InvalidInput(ref cause) => cause,
            BatchGetPartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchStopJobRun
#[derive(Debug, PartialEq)]
pub enum GlueBatchStopJobRunError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GlueBatchStopJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GlueBatchStopJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GlueBatchStopJobRunError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GlueBatchStopJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GlueBatchStopJobRunError {
    fn description(&self) -> &str {
        match *self {
            GlueBatchStopJobRunError::InternalService(ref cause) => cause,
            GlueBatchStopJobRunError::InvalidInput(ref cause) => cause,
            GlueBatchStopJobRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateClassifier
#[derive(Debug, PartialEq)]
pub enum CreateClassifierError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl CreateClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateClassifierError::AlreadyExists(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateClassifierError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClassifierError {
    fn description(&self) -> &str {
        match *self {
            CreateClassifierError::AlreadyExists(ref cause) => cause,
            CreateClassifierError::InvalidInput(ref cause) => cause,
            CreateClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConnectionError::AlreadyExists(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateConnectionError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateConnectionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectionError::AlreadyExists(ref cause) => cause,
            CreateConnectionError::GlueEncryption(ref cause) => cause,
            CreateConnectionError::InvalidInput(ref cause) => cause,
            CreateConnectionError::OperationTimeout(ref cause) => cause,
            CreateConnectionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCrawler
#[derive(Debug, PartialEq)]
pub enum CreateCrawlerError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateCrawlerError::AlreadyExists(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateCrawlerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateCrawlerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateCrawlerError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            CreateCrawlerError::AlreadyExists(ref cause) => cause,
            CreateCrawlerError::InvalidInput(ref cause) => cause,
            CreateCrawlerError::OperationTimeout(ref cause) => cause,
            CreateCrawlerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDatabase
#[derive(Debug, PartialEq)]
pub enum CreateDatabaseError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDatabaseError::AlreadyExists(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateDatabaseError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateDatabaseError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            CreateDatabaseError::AlreadyExists(ref cause) => cause,
            CreateDatabaseError::GlueEncryption(ref cause) => cause,
            CreateDatabaseError::InternalService(ref cause) => cause,
            CreateDatabaseError::InvalidInput(ref cause) => cause,
            CreateDatabaseError::OperationTimeout(ref cause) => cause,
            CreateDatabaseError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateDevEndpointError {
    /// <p>Access to a resource was denied.</p>
    AccessDenied(String),
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDevEndpointError::AccessDenied(err.msg))
                }
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDevEndpointError::AlreadyExists(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateDevEndpointError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateDevEndpointError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDevEndpointError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateDevEndpointError::AccessDenied(ref cause) => cause,
            CreateDevEndpointError::AlreadyExists(ref cause) => cause,
            CreateDevEndpointError::IdempotentParameterMismatch(ref cause) => cause,
            CreateDevEndpointError::InternalService(ref cause) => cause,
            CreateDevEndpointError::InvalidInput(ref cause) => cause,
            CreateDevEndpointError::OperationTimeout(ref cause) => cause,
            CreateDevEndpointError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateJobError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateJobError::ConcurrentModification(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateJobError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateJobError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateJobError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateJobError::AlreadyExists(ref cause) => cause,
            CreateJobError::ConcurrentModification(ref cause) => cause,
            CreateJobError::IdempotentParameterMismatch(ref cause) => cause,
            CreateJobError::InternalService(ref cause) => cause,
            CreateJobError::InvalidInput(ref cause) => cause,
            CreateJobError::OperationTimeout(ref cause) => cause,
            CreateJobError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePartition
#[derive(Debug, PartialEq)]
pub enum CreatePartitionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreatePartitionError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreatePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreatePartitionError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreatePartitionError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePartitionError {
    fn description(&self) -> &str {
        match *self {
            CreatePartitionError::AlreadyExists(ref cause) => cause,
            CreatePartitionError::EntityNotFound(ref cause) => cause,
            CreatePartitionError::GlueEncryption(ref cause) => cause,
            CreatePartitionError::InternalService(ref cause) => cause,
            CreatePartitionError::InvalidInput(ref cause) => cause,
            CreatePartitionError::OperationTimeout(ref cause) => cause,
            CreatePartitionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateScript
#[derive(Debug, PartialEq)]
pub enum CreateScriptError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl CreateScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateScriptError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateScriptError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateScriptError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateScriptError {
    fn description(&self) -> &str {
        match *self {
            CreateScriptError::InternalService(ref cause) => cause,
            CreateScriptError::InvalidInput(ref cause) => cause,
            CreateScriptError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateSecurityConfigurationError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::AlreadyExists(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        CreateSecurityConfigurationError::OperationTimeout(err.msg),
                    )
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateSecurityConfigurationError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateSecurityConfigurationError::AlreadyExists(ref cause) => cause,
            CreateSecurityConfigurationError::InternalService(ref cause) => cause,
            CreateSecurityConfigurationError::InvalidInput(ref cause) => cause,
            CreateSecurityConfigurationError::OperationTimeout(ref cause) => cause,
            CreateSecurityConfigurationError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTable
#[derive(Debug, PartialEq)]
pub enum CreateTableError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateTableError::AlreadyExists(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateTableError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateTableError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTableError {
    fn description(&self) -> &str {
        match *self {
            CreateTableError::AlreadyExists(ref cause) => cause,
            CreateTableError::EntityNotFound(ref cause) => cause,
            CreateTableError::GlueEncryption(ref cause) => cause,
            CreateTableError::InternalService(ref cause) => cause,
            CreateTableError::InvalidInput(ref cause) => cause,
            CreateTableError::OperationTimeout(ref cause) => cause,
            CreateTableError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTrigger
#[derive(Debug, PartialEq)]
pub enum CreateTriggerError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>The same unique identifier was associated with two different records.</p>
    IdempotentParameterMismatch(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateTriggerError::AlreadyExists(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(CreateTriggerError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateTriggerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(CreateTriggerError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTriggerError {
    fn description(&self) -> &str {
        match *self {
            CreateTriggerError::AlreadyExists(ref cause) => cause,
            CreateTriggerError::ConcurrentModification(ref cause) => cause,
            CreateTriggerError::IdempotentParameterMismatch(ref cause) => cause,
            CreateTriggerError::InternalService(ref cause) => cause,
            CreateTriggerError::InvalidInput(ref cause) => cause,
            CreateTriggerError::OperationTimeout(ref cause) => cause,
            CreateTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum CreateUserDefinedFunctionError {
    /// <p>A resource to be created or added already exists.</p>
    AlreadyExists(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl CreateUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::AlreadyExists(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(CreateUserDefinedFunctionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(
                        CreateUserDefinedFunctionError::ResourceNumberLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            CreateUserDefinedFunctionError::AlreadyExists(ref cause) => cause,
            CreateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            CreateUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            CreateUserDefinedFunctionError::InternalService(ref cause) => cause,
            CreateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            CreateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
            CreateUserDefinedFunctionError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClassifier
#[derive(Debug, PartialEq)]
pub enum DeleteClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteClassifierError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClassifierError {
    fn description(&self) -> &str {
        match *self {
            DeleteClassifierError::EntityNotFound(ref cause) => cause,
            DeleteClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteConnectionError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::EntityNotFound(ref cause) => cause,
            DeleteConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCrawler
#[derive(Debug, PartialEq)]
pub enum DeleteCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl DeleteCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(DeleteCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteCrawlerError::OperationTimeout(err.msg))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(DeleteCrawlerError::SchedulerTransitioning(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCrawlerError {
    fn description(&self) -> &str {
        match *self {
            DeleteCrawlerError::CrawlerRunning(ref cause) => cause,
            DeleteCrawlerError::EntityNotFound(ref cause) => cause,
            DeleteCrawlerError::OperationTimeout(ref cause) => cause,
            DeleteCrawlerError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteDatabaseError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatabaseError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatabaseError::EntityNotFound(ref cause) => cause,
            DeleteDatabaseError::InternalService(ref cause) => cause,
            DeleteDatabaseError::InvalidInput(ref cause) => cause,
            DeleteDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDevEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteDevEndpointError::EntityNotFound(ref cause) => cause,
            DeleteDevEndpointError::InternalService(ref cause) => cause,
            DeleteDevEndpointError::InvalidInput(ref cause) => cause,
            DeleteDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteJob
#[derive(Debug, PartialEq)]
pub enum DeleteJobError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobError::InternalService(ref cause) => cause,
            DeleteJobError::InvalidInput(ref cause) => cause,
            DeleteJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePartition
#[derive(Debug, PartialEq)]
pub enum DeletePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeletePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeletePartitionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeletePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeletePartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePartitionError {
    fn description(&self) -> &str {
        match *self {
            DeletePartitionError::EntityNotFound(ref cause) => cause,
            DeletePartitionError::InternalService(ref cause) => cause,
            DeletePartitionError::InvalidInput(ref cause) => cause,
            DeletePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResourcePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteResourcePolicyError {
    /// <p>A specified condition was not satisfied.</p>
    ConditionCheckFailure(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConditionCheckFailureException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::ConditionCheckFailure(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteResourcePolicyError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourcePolicyError::ConditionCheckFailure(ref cause) => cause,
            DeleteResourcePolicyError::EntityNotFound(ref cause) => cause,
            DeleteResourcePolicyError::InternalService(ref cause) => cause,
            DeleteResourcePolicyError::InvalidInput(ref cause) => cause,
            DeleteResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteSecurityConfigurationError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteSecurityConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        DeleteSecurityConfigurationError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteSecurityConfigurationError::EntityNotFound(ref cause) => cause,
            DeleteSecurityConfigurationError::InternalService(ref cause) => cause,
            DeleteSecurityConfigurationError::InvalidInput(ref cause) => cause,
            DeleteSecurityConfigurationError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTable
#[derive(Debug, PartialEq)]
pub enum DeleteTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteTableError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableError::EntityNotFound(ref cause) => cause,
            DeleteTableError::InternalService(ref cause) => cause,
            DeleteTableError::InvalidInput(ref cause) => cause,
            DeleteTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTableVersion
#[derive(Debug, PartialEq)]
pub enum DeleteTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteTableVersionError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTableVersionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTableVersionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTableVersionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableVersionError::EntityNotFound(ref cause) => cause,
            DeleteTableVersionError::InternalService(ref cause) => cause,
            DeleteTableVersionError::InvalidInput(ref cause) => cause,
            DeleteTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTrigger
#[derive(Debug, PartialEq)]
pub enum DeleteTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTriggerError {
    fn description(&self) -> &str {
        match *self {
            DeleteTriggerError::ConcurrentModification(ref cause) => cause,
            DeleteTriggerError::InternalService(ref cause) => cause,
            DeleteTriggerError::InvalidInput(ref cause) => cause,
            DeleteTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum DeleteUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl DeleteUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(DeleteUserDefinedFunctionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            DeleteUserDefinedFunctionError::InternalService(ref cause) => cause,
            DeleteUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            DeleteUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCatalogImportStatus
#[derive(Debug, PartialEq)]
pub enum GetCatalogImportStatusError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCatalogImportStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCatalogImportStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetCatalogImportStatusError::InternalService(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCatalogImportStatusError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCatalogImportStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCatalogImportStatusError {
    fn description(&self) -> &str {
        match *self {
            GetCatalogImportStatusError::InternalService(ref cause) => cause,
            GetCatalogImportStatusError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifier
#[derive(Debug, PartialEq)]
pub enum GetClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetClassifierError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetClassifierError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifierError {
    fn description(&self) -> &str {
        match *self {
            GetClassifierError::EntityNotFound(ref cause) => cause,
            GetClassifierError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClassifiers
#[derive(Debug, PartialEq)]
pub enum GetClassifiersError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetClassifiersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClassifiersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetClassifiersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClassifiersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClassifiersError {
    fn description(&self) -> &str {
        match *self {
            GetClassifiersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnection
#[derive(Debug, PartialEq)]
pub enum GetConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetConnectionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionError::EntityNotFound(ref cause) => cause,
            GetConnectionError::GlueEncryption(ref cause) => cause,
            GetConnectionError::InvalidInput(ref cause) => cause,
            GetConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnections
#[derive(Debug, PartialEq)]
pub enum GetConnectionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetConnectionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetConnectionsError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetConnectionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetConnectionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectionsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectionsError::EntityNotFound(ref cause) => cause,
            GetConnectionsError::GlueEncryption(ref cause) => cause,
            GetConnectionsError::InvalidInput(ref cause) => cause,
            GetConnectionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawler
#[derive(Debug, PartialEq)]
pub enum GetCrawlerError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerError::EntityNotFound(ref cause) => cause,
            GetCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlerMetrics
#[derive(Debug, PartialEq)]
pub enum GetCrawlerMetricsError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlerMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlerMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlerMetricsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlerMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlerMetricsError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlerMetricsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCrawlers
#[derive(Debug, PartialEq)]
pub enum GetCrawlersError {
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetCrawlersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCrawlersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetCrawlersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCrawlersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCrawlersError {
    fn description(&self) -> &str {
        match *self {
            GetCrawlersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataCatalogEncryptionSettings
#[derive(Debug, PartialEq)]
pub enum GetDataCatalogEncryptionSettingsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDataCatalogEncryptionSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDataCatalogEncryptionSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::InvalidInput(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        GetDataCatalogEncryptionSettingsError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDataCatalogEncryptionSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataCatalogEncryptionSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetDataCatalogEncryptionSettingsError::InternalService(ref cause) => cause,
            GetDataCatalogEncryptionSettingsError::InvalidInput(ref cause) => cause,
            GetDataCatalogEncryptionSettingsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabase
#[derive(Debug, PartialEq)]
pub enum GetDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDatabaseError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabaseError {
    fn description(&self) -> &str {
        match *self {
            GetDatabaseError::EntityNotFound(ref cause) => cause,
            GetDatabaseError::GlueEncryption(ref cause) => cause,
            GetDatabaseError::InternalService(ref cause) => cause,
            GetDatabaseError::InvalidInput(ref cause) => cause,
            GetDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatabases
#[derive(Debug, PartialEq)]
pub enum GetDatabasesError {
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetDatabasesError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDatabasesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDatabasesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDatabasesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDatabasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatabasesError {
    fn description(&self) -> &str {
        match *self {
            GetDatabasesError::GlueEncryption(ref cause) => cause,
            GetDatabasesError::InternalService(ref cause) => cause,
            GetDatabasesError::InvalidInput(ref cause) => cause,
            GetDatabasesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataflowGraph
#[derive(Debug, PartialEq)]
pub enum GetDataflowGraphError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDataflowGraphError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataflowGraphError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetDataflowGraphError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDataflowGraphError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDataflowGraphError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDataflowGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataflowGraphError {
    fn description(&self) -> &str {
        match *self {
            GetDataflowGraphError::InternalService(ref cause) => cause,
            GetDataflowGraphError::InvalidInput(ref cause) => cause,
            GetDataflowGraphError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoint
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointError::EntityNotFound(ref cause) => cause,
            GetDevEndpointError::InternalService(ref cause) => cause,
            GetDevEndpointError::InvalidInput(ref cause) => cause,
            GetDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevEndpoints
#[derive(Debug, PartialEq)]
pub enum GetDevEndpointsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetDevEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDevEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetDevEndpointsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetDevEndpointsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDevEndpointsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetDevEndpointsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDevEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevEndpointsError {
    fn description(&self) -> &str {
        match *self {
            GetDevEndpointsError::EntityNotFound(ref cause) => cause,
            GetDevEndpointsError::InternalService(ref cause) => cause,
            GetDevEndpointsError::InvalidInput(ref cause) => cause,
            GetDevEndpointsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::EntityNotFound(ref cause) => cause,
            GetJobError::InternalService(ref cause) => cause,
            GetJobError::InvalidInput(ref cause) => cause,
            GetJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRun
#[derive(Debug, PartialEq)]
pub enum GetJobRunError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobRunError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunError::EntityNotFound(ref cause) => cause,
            GetJobRunError::InternalService(ref cause) => cause,
            GetJobRunError::InvalidInput(ref cause) => cause,
            GetJobRunError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobRuns
#[derive(Debug, PartialEq)]
pub enum GetJobRunsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobRunsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobRunsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobRunsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobRunsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobRunsError {
    fn description(&self) -> &str {
        match *self {
            GetJobRunsError::EntityNotFound(ref cause) => cause,
            GetJobRunsError::InternalService(ref cause) => cause,
            GetJobRunsError::InvalidInput(ref cause) => cause,
            GetJobRunsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobs
#[derive(Debug, PartialEq)]
pub enum GetJobsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetJobsError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetJobsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetJobsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetJobsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobsError {
    fn description(&self) -> &str {
        match *self {
            GetJobsError::EntityNotFound(ref cause) => cause,
            GetJobsError::InternalService(ref cause) => cause,
            GetJobsError::InvalidInput(ref cause) => cause,
            GetJobsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMapping
#[derive(Debug, PartialEq)]
pub enum GetMappingError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMappingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetMappingError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetMappingError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetMappingError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetMappingError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMappingError {
    fn description(&self) -> &str {
        match *self {
            GetMappingError::EntityNotFound(ref cause) => cause,
            GetMappingError::InternalService(ref cause) => cause,
            GetMappingError::InvalidInput(ref cause) => cause,
            GetMappingError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartition
#[derive(Debug, PartialEq)]
pub enum GetPartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetPartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetPartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetPartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionError::EntityNotFound(ref cause) => cause,
            GetPartitionError::GlueEncryption(ref cause) => cause,
            GetPartitionError::InternalService(ref cause) => cause,
            GetPartitionError::InvalidInput(ref cause) => cause,
            GetPartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPartitions
#[derive(Debug, PartialEq)]
pub enum GetPartitionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPartitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPartitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetPartitionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetPartitionsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetPartitionsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPartitionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPartitionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPartitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPartitionsError {
    fn description(&self) -> &str {
        match *self {
            GetPartitionsError::EntityNotFound(ref cause) => cause,
            GetPartitionsError::GlueEncryption(ref cause) => cause,
            GetPartitionsError::InternalService(ref cause) => cause,
            GetPartitionsError::InvalidInput(ref cause) => cause,
            GetPartitionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPlan
#[derive(Debug, PartialEq)]
pub enum GetPlanError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetPlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPlanError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetPlanError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetPlanError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetPlanError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPlanError {
    fn description(&self) -> &str {
        match *self {
            GetPlanError::InternalService(ref cause) => cause,
            GetPlanError::InvalidInput(ref cause) => cause,
            GetPlanError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourcePolicy
#[derive(Debug, PartialEq)]
pub enum GetResourcePolicyError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourcePolicyError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetResourcePolicyError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            GetResourcePolicyError::EntityNotFound(ref cause) => cause,
            GetResourcePolicyError::InternalService(ref cause) => cause,
            GetResourcePolicyError::InvalidInput(ref cause) => cause,
            GetResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSecurityConfiguration
#[derive(Debug, PartialEq)]
pub enum GetSecurityConfigurationError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetSecurityConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSecurityConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetSecurityConfigurationError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSecurityConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSecurityConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetSecurityConfigurationError::EntityNotFound(ref cause) => cause,
            GetSecurityConfigurationError::InternalService(ref cause) => cause,
            GetSecurityConfigurationError::InvalidInput(ref cause) => cause,
            GetSecurityConfigurationError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSecurityConfigurations
#[derive(Debug, PartialEq)]
pub enum GetSecurityConfigurationsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetSecurityConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSecurityConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetSecurityConfigurationsError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSecurityConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSecurityConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            GetSecurityConfigurationsError::EntityNotFound(ref cause) => cause,
            GetSecurityConfigurationsError::InternalService(ref cause) => cause,
            GetSecurityConfigurationsError::InvalidInput(ref cause) => cause,
            GetSecurityConfigurationsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTable
#[derive(Debug, PartialEq)]
pub enum GetTableError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableError {
    fn description(&self) -> &str {
        match *self {
            GetTableError::EntityNotFound(ref cause) => cause,
            GetTableError::GlueEncryption(ref cause) => cause,
            GetTableError::InternalService(ref cause) => cause,
            GetTableError::InvalidInput(ref cause) => cause,
            GetTableError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTableVersion
#[derive(Debug, PartialEq)]
pub enum GetTableVersionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableVersionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableVersionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableVersionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableVersionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableVersionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableVersionError {
    fn description(&self) -> &str {
        match *self {
            GetTableVersionError::EntityNotFound(ref cause) => cause,
            GetTableVersionError::GlueEncryption(ref cause) => cause,
            GetTableVersionError::InternalService(ref cause) => cause,
            GetTableVersionError::InvalidInput(ref cause) => cause,
            GetTableVersionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTableVersions
#[derive(Debug, PartialEq)]
pub enum GetTableVersionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTableVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTableVersionsError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTableVersionsError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTableVersionsError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTableVersionsError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTableVersionsError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTableVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTableVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetTableVersionsError::EntityNotFound(ref cause) => cause,
            GetTableVersionsError::GlueEncryption(ref cause) => cause,
            GetTableVersionsError::InternalService(ref cause) => cause,
            GetTableVersionsError::InvalidInput(ref cause) => cause,
            GetTableVersionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTables
#[derive(Debug, PartialEq)]
pub enum GetTablesError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTablesError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetTablesError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTablesError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTablesError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTablesError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTablesError {
    fn description(&self) -> &str {
        match *self {
            GetTablesError::EntityNotFound(ref cause) => cause,
            GetTablesError::GlueEncryption(ref cause) => cause,
            GetTablesError::InternalService(ref cause) => cause,
            GetTablesError::InvalidInput(ref cause) => cause,
            GetTablesError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTrigger
#[derive(Debug, PartialEq)]
pub enum GetTriggerError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggerError {
    fn description(&self) -> &str {
        match *self {
            GetTriggerError::EntityNotFound(ref cause) => cause,
            GetTriggerError::InternalService(ref cause) => cause,
            GetTriggerError::InvalidInput(ref cause) => cause,
            GetTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTriggers
#[derive(Debug, PartialEq)]
pub enum GetTriggersError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetTriggersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTriggersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetTriggersError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetTriggersError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTriggersError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetTriggersError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTriggersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTriggersError {
    fn description(&self) -> &str {
        match *self {
            GetTriggersError::EntityNotFound(ref cause) => cause,
            GetTriggersError::InternalService(ref cause) => cause,
            GetTriggersError::InvalidInput(ref cause) => cause,
            GetTriggersError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetUserDefinedFunctionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            GetUserDefinedFunctionError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserDefinedFunctions
#[derive(Debug, PartialEq)]
pub enum GetUserDefinedFunctionsError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl GetUserDefinedFunctionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserDefinedFunctionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(GetUserDefinedFunctionsError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserDefinedFunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserDefinedFunctionsError {
    fn description(&self) -> &str {
        match *self {
            GetUserDefinedFunctionsError::EntityNotFound(ref cause) => cause,
            GetUserDefinedFunctionsError::GlueEncryption(ref cause) => cause,
            GetUserDefinedFunctionsError::InternalService(ref cause) => cause,
            GetUserDefinedFunctionsError::InvalidInput(ref cause) => cause,
            GetUserDefinedFunctionsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCatalogToGlue
#[derive(Debug, PartialEq)]
pub enum ImportCatalogToGlueError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ImportCatalogToGlueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportCatalogToGlueError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ImportCatalogToGlueError::InternalService(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ImportCatalogToGlueError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportCatalogToGlueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCatalogToGlueError {
    fn description(&self) -> &str {
        match *self {
            ImportCatalogToGlueError::InternalService(ref cause) => cause,
            ImportCatalogToGlueError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDataCatalogEncryptionSettings
#[derive(Debug, PartialEq)]
pub enum PutDataCatalogEncryptionSettingsError {
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl PutDataCatalogEncryptionSettingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDataCatalogEncryptionSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::InternalService(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::InvalidInput(err.msg),
                    )
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(
                        PutDataCatalogEncryptionSettingsError::OperationTimeout(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutDataCatalogEncryptionSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDataCatalogEncryptionSettingsError {
    fn description(&self) -> &str {
        match *self {
            PutDataCatalogEncryptionSettingsError::InternalService(ref cause) => cause,
            PutDataCatalogEncryptionSettingsError::InvalidInput(ref cause) => cause,
            PutDataCatalogEncryptionSettingsError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by PutResourcePolicy
#[derive(Debug, PartialEq)]
pub enum PutResourcePolicyError {
    /// <p>A specified condition was not satisfied.</p>
    ConditionCheckFailure(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl PutResourcePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutResourcePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConditionCheckFailureException" => {
                    return RusotoError::Service(PutResourcePolicyError::ConditionCheckFailure(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(PutResourcePolicyError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(PutResourcePolicyError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutResourcePolicyError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(PutResourcePolicyError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutResourcePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutResourcePolicyError {
    fn description(&self) -> &str {
        match *self {
            PutResourcePolicyError::ConditionCheckFailure(ref cause) => cause,
            PutResourcePolicyError::EntityNotFound(ref cause) => cause,
            PutResourcePolicyError::InternalService(ref cause) => cause,
            PutResourcePolicyError::InvalidInput(ref cause) => cause,
            PutResourcePolicyError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetJobBookmark
#[derive(Debug, PartialEq)]
pub enum ResetJobBookmarkError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl ResetJobBookmarkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetJobBookmarkError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(ResetJobBookmarkError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ResetJobBookmarkError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ResetJobBookmarkError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(ResetJobBookmarkError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ResetJobBookmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetJobBookmarkError {
    fn description(&self) -> &str {
        match *self {
            ResetJobBookmarkError::EntityNotFound(ref cause) => cause,
            ResetJobBookmarkError::InternalService(ref cause) => cause,
            ResetJobBookmarkError::InvalidInput(ref cause) => cause,
            ResetJobBookmarkError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawler
#[derive(Debug, PartialEq)]
pub enum StartCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StartCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(StartCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerError::CrawlerRunning(ref cause) => cause,
            StartCrawlerError::EntityNotFound(ref cause) => cause,
            StartCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StartCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>There is no applicable schedule.</p>
    NoSchedule(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is already running.</p>
    SchedulerRunning(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl StartCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::EntityNotFound(err.msg))
                }
                "NoScheduleException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::NoSchedule(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerRunningException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::SchedulerRunning(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(StartCrawlerScheduleError::SchedulerTransitioning(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StartCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StartCrawlerScheduleError::NoSchedule(ref cause) => cause,
            StartCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerRunning(ref cause) => cause,
            StartCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by StartJobRun
#[derive(Debug, PartialEq)]
pub enum StartJobRunError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(StartJobRunError::ConcurrentRunsExceeded(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartJobRunError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartJobRunError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartJobRunError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartJobRunError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(StartJobRunError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartJobRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartJobRunError {
    fn description(&self) -> &str {
        match *self {
            StartJobRunError::ConcurrentRunsExceeded(ref cause) => cause,
            StartJobRunError::EntityNotFound(ref cause) => cause,
            StartJobRunError::InternalService(ref cause) => cause,
            StartJobRunError::InvalidInput(ref cause) => cause,
            StartJobRunError::OperationTimeout(ref cause) => cause,
            StartJobRunError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StartTrigger
#[derive(Debug, PartialEq)]
pub enum StartTriggerError {
    /// <p>Too many jobs are being run concurrently.</p>
    ConcurrentRunsExceeded(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl StartTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentRunsExceededException" => {
                    return RusotoError::Service(StartTriggerError::ConcurrentRunsExceeded(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StartTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StartTriggerError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(StartTriggerError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTriggerError {
    fn description(&self) -> &str {
        match *self {
            StartTriggerError::ConcurrentRunsExceeded(ref cause) => cause,
            StartTriggerError::EntityNotFound(ref cause) => cause,
            StartTriggerError::InternalService(ref cause) => cause,
            StartTriggerError::InvalidInput(ref cause) => cause,
            StartTriggerError::OperationTimeout(ref cause) => cause,
            StartTriggerError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawler
#[derive(Debug, PartialEq)]
pub enum StopCrawlerError {
    /// <p>The specified crawler is not running.</p>
    CrawlerNotRunning(String),
    /// <p>The specified crawler is stopping.</p>
    CrawlerStopping(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StopCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerNotRunningException" => {
                    return RusotoError::Service(StopCrawlerError::CrawlerNotRunning(err.msg))
                }
                "CrawlerStoppingException" => {
                    return RusotoError::Service(StopCrawlerError::CrawlerStopping(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopCrawlerError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopCrawlerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerError::CrawlerNotRunning(ref cause) => cause,
            StopCrawlerError::CrawlerStopping(ref cause) => cause,
            StopCrawlerError::EntityNotFound(ref cause) => cause,
            StopCrawlerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by StopCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum StopCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is not running.</p>
    SchedulerNotRunning(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
}

impl StopCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::EntityNotFound(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerNotRunningException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::SchedulerNotRunning(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(StopCrawlerScheduleError::SchedulerTransitioning(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            StopCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            StopCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerNotRunning(ref cause) => cause,
            StopCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
        }
    }
}
/// Errors returned by StopTrigger
#[derive(Debug, PartialEq)]
pub enum StopTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl StopTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StopTriggerError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(StopTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StopTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(StopTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTriggerError {
    fn description(&self) -> &str {
        match *self {
            StopTriggerError::ConcurrentModification(ref cause) => cause,
            StopTriggerError::EntityNotFound(ref cause) => cause,
            StopTriggerError::InternalService(ref cause) => cause,
            StopTriggerError::InvalidInput(ref cause) => cause,
            StopTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateClassifier
#[derive(Debug, PartialEq)]
pub enum UpdateClassifierError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateClassifierError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateClassifierError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateClassifierError::OperationTimeout(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateClassifierError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClassifierError {
    fn description(&self) -> &str {
        match *self {
            UpdateClassifierError::EntityNotFound(ref cause) => cause,
            UpdateClassifierError::InvalidInput(ref cause) => cause,
            UpdateClassifierError::OperationTimeout(ref cause) => cause,
            UpdateClassifierError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConnection
#[derive(Debug, PartialEq)]
pub enum UpdateConnectionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateConnectionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateConnectionError::GlueEncryption(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateConnectionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateConnectionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConnectionError {
    fn description(&self) -> &str {
        match *self {
            UpdateConnectionError::EntityNotFound(ref cause) => cause,
            UpdateConnectionError::GlueEncryption(ref cause) => cause,
            UpdateConnectionError::InvalidInput(ref cause) => cause,
            UpdateConnectionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawler
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerError {
    /// <p>The operation cannot be performed because the crawler is already running.</p>
    CrawlerRunning(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateCrawlerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCrawlerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CrawlerRunningException" => {
                    return RusotoError::Service(UpdateCrawlerError::CrawlerRunning(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateCrawlerError::EntityNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateCrawlerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateCrawlerError::OperationTimeout(err.msg))
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateCrawlerError::VersionMismatch(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerError::CrawlerRunning(ref cause) => cause,
            UpdateCrawlerError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerError::InvalidInput(ref cause) => cause,
            UpdateCrawlerError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCrawlerSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateCrawlerScheduleError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>The specified scheduler is transitioning.</p>
    SchedulerTransitioning(String),
    /// <p>There was a version conflict.</p>
    VersionMismatch(String),
}

impl UpdateCrawlerScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCrawlerScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::EntityNotFound(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::OperationTimeout(
                        err.msg,
                    ))
                }
                "SchedulerTransitioningException" => {
                    return RusotoError::Service(
                        UpdateCrawlerScheduleError::SchedulerTransitioning(err.msg),
                    )
                }
                "VersionMismatchException" => {
                    return RusotoError::Service(UpdateCrawlerScheduleError::VersionMismatch(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCrawlerScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCrawlerScheduleError {
    fn description(&self) -> &str {
        match *self {
            UpdateCrawlerScheduleError::EntityNotFound(ref cause) => cause,
            UpdateCrawlerScheduleError::InvalidInput(ref cause) => cause,
            UpdateCrawlerScheduleError::OperationTimeout(ref cause) => cause,
            UpdateCrawlerScheduleError::SchedulerTransitioning(ref cause) => cause,
            UpdateCrawlerScheduleError::VersionMismatch(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateDatabaseError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateDatabaseError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateDatabaseError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateDatabaseError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDatabaseError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateDatabaseError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDatabaseError {
    fn description(&self) -> &str {
        match *self {
            UpdateDatabaseError::EntityNotFound(ref cause) => cause,
            UpdateDatabaseError::GlueEncryption(ref cause) => cause,
            UpdateDatabaseError::InternalService(ref cause) => cause,
            UpdateDatabaseError::InvalidInput(ref cause) => cause,
            UpdateDatabaseError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateDevEndpointError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateDevEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDevEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateDevEndpointError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateDevEndpointError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDevEndpointError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateDevEndpointError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDevEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDevEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateDevEndpointError::EntityNotFound(ref cause) => cause,
            UpdateDevEndpointError::InternalService(ref cause) => cause,
            UpdateDevEndpointError::InvalidInput(ref cause) => cause,
            UpdateDevEndpointError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateJobError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateJobError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateJobError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateJobError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateJobError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateJobError::ConcurrentModification(ref cause) => cause,
            UpdateJobError::EntityNotFound(ref cause) => cause,
            UpdateJobError::InternalService(ref cause) => cause,
            UpdateJobError::InvalidInput(ref cause) => cause,
            UpdateJobError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePartition
#[derive(Debug, PartialEq)]
pub enum UpdatePartitionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdatePartitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePartitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdatePartitionError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdatePartitionError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdatePartitionError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdatePartitionError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdatePartitionError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePartitionError {
    fn description(&self) -> &str {
        match *self {
            UpdatePartitionError::EntityNotFound(ref cause) => cause,
            UpdatePartitionError::GlueEncryption(ref cause) => cause,
            UpdatePartitionError::InternalService(ref cause) => cause,
            UpdatePartitionError::InvalidInput(ref cause) => cause,
            UpdatePartitionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTable
#[derive(Debug, PartialEq)]
pub enum UpdateTableError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
    /// <p>A resource numerical limit was exceeded.</p>
    ResourceNumberLimitExceeded(String),
}

impl UpdateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTableError::ConcurrentModification(err.msg))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateTableError::EntityNotFound(err.msg))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateTableError::GlueEncryption(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTableError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateTableError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateTableError::OperationTimeout(err.msg))
                }
                "ResourceNumberLimitExceededException" => {
                    return RusotoError::Service(UpdateTableError::ResourceNumberLimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTableError {
    fn description(&self) -> &str {
        match *self {
            UpdateTableError::ConcurrentModification(ref cause) => cause,
            UpdateTableError::EntityNotFound(ref cause) => cause,
            UpdateTableError::GlueEncryption(ref cause) => cause,
            UpdateTableError::InternalService(ref cause) => cause,
            UpdateTableError::InvalidInput(ref cause) => cause,
            UpdateTableError::OperationTimeout(ref cause) => cause,
            UpdateTableError::ResourceNumberLimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTrigger
#[derive(Debug, PartialEq)]
pub enum UpdateTriggerError {
    /// <p>Two processes are trying to modify a resource simultaneously.</p>
    ConcurrentModification(String),
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateTriggerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTriggerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTriggerError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateTriggerError::EntityNotFound(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateTriggerError::InternalService(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateTriggerError::InvalidInput(err.msg))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateTriggerError::OperationTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTriggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTriggerError {
    fn description(&self) -> &str {
        match *self {
            UpdateTriggerError::ConcurrentModification(ref cause) => cause,
            UpdateTriggerError::EntityNotFound(ref cause) => cause,
            UpdateTriggerError::InternalService(ref cause) => cause,
            UpdateTriggerError::InvalidInput(ref cause) => cause,
            UpdateTriggerError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUserDefinedFunction
#[derive(Debug, PartialEq)]
pub enum UpdateUserDefinedFunctionError {
    /// <p>A specified entity does not exist</p>
    EntityNotFound(String),
    /// <p>An encryption operation failed.</p>
    GlueEncryption(String),
    /// <p>An internal service error occurred.</p>
    InternalService(String),
    /// <p>The input provided was not valid.</p>
    InvalidInput(String),
    /// <p>The operation timed out.</p>
    OperationTimeout(String),
}

impl UpdateUserDefinedFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUserDefinedFunctionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "EntityNotFoundException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::EntityNotFound(
                        err.msg,
                    ))
                }
                "GlueEncryptionException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::GlueEncryption(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::InvalidInput(
                        err.msg,
                    ))
                }
                "OperationTimeoutException" => {
                    return RusotoError::Service(UpdateUserDefinedFunctionError::OperationTimeout(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUserDefinedFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUserDefinedFunctionError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserDefinedFunctionError::EntityNotFound(ref cause) => cause,
            UpdateUserDefinedFunctionError::GlueEncryption(ref cause) => cause,
            UpdateUserDefinedFunctionError::InternalService(ref cause) => cause,
            UpdateUserDefinedFunctionError::InvalidInput(ref cause) => cause,
            UpdateUserDefinedFunctionError::OperationTimeout(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Glue API. AWS Glue clients implement this trait.
pub trait Glue {
    /// <p>Creates one or more partitions in a batch operation.</p>
    fn batch_create_partition(
        &self,
        input: BatchCreatePartitionRequest,
    ) -> RusotoFuture<BatchCreatePartitionResponse, BatchCreatePartitionError>;

    /// <p>Deletes a list of connection definitions from the Data Catalog.</p>
    fn batch_delete_connection(
        &self,
        input: BatchDeleteConnectionRequest,
    ) -> RusotoFuture<BatchDeleteConnectionResponse, BatchDeleteConnectionError>;

    /// <p>Deletes one or more partitions in a batch operation.</p>
    fn batch_delete_partition(
        &self,
        input: BatchDeletePartitionRequest,
    ) -> RusotoFuture<BatchDeletePartitionResponse, BatchDeletePartitionError>;

    /// <p><p>Deletes multiple tables at once.</p> <note> <p>After completing this operation, you will no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>BatchDeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn batch_delete_table(
        &self,
        input: BatchDeleteTableRequest,
    ) -> RusotoFuture<BatchDeleteTableResponse, BatchDeleteTableError>;

    /// <p>Deletes a specified batch of versions of a table.</p>
    fn batch_delete_table_version(
        &self,
        input: BatchDeleteTableVersionRequest,
    ) -> RusotoFuture<BatchDeleteTableVersionResponse, BatchDeleteTableVersionError>;

    /// <p>Retrieves partitions in a batch request.</p>
    fn batch_get_partition(
        &self,
        input: BatchGetPartitionRequest,
    ) -> RusotoFuture<BatchGetPartitionResponse, BatchGetPartitionError>;

    /// <p>Stops one or more job runs for a specified job definition.</p>
    fn batch_stop_job_run(
        &self,
        input: BatchStopJobRunRequest,
    ) -> RusotoFuture<BatchStopJobRunResponse, GlueBatchStopJobRunError>;

    /// <p>Creates a classifier in the user's account. This may be a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, or abbrev <code>JsonClassifier</code>, depending on which field of the request is present.</p>
    fn create_classifier(
        &self,
        input: CreateClassifierRequest,
    ) -> RusotoFuture<CreateClassifierResponse, CreateClassifierError>;

    /// <p>Creates a connection definition in the Data Catalog.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<CreateConnectionResponse, CreateConnectionError>;

    /// <p>Creates a new crawler with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in the <i>s3Targets</i> field, the <i>jdbcTargets</i> field, or the <i>DynamoDBTargets</i> field.</p>
    fn create_crawler(
        &self,
        input: CreateCrawlerRequest,
    ) -> RusotoFuture<CreateCrawlerResponse, CreateCrawlerError>;

    /// <p>Creates a new database in a Data Catalog.</p>
    fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> RusotoFuture<CreateDatabaseResponse, CreateDatabaseError>;

    /// <p>Creates a new DevEndpoint.</p>
    fn create_dev_endpoint(
        &self,
        input: CreateDevEndpointRequest,
    ) -> RusotoFuture<CreateDevEndpointResponse, CreateDevEndpointError>;

    /// <p>Creates a new job definition.</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError>;

    /// <p>Creates a new partition.</p>
    fn create_partition(
        &self,
        input: CreatePartitionRequest,
    ) -> RusotoFuture<CreatePartitionResponse, CreatePartitionError>;

    /// <p>Transforms a directed acyclic graph (DAG) into code.</p>
    fn create_script(
        &self,
        input: CreateScriptRequest,
    ) -> RusotoFuture<CreateScriptResponse, CreateScriptError>;

    /// <p>Creates a new security configuration.</p>
    fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationRequest,
    ) -> RusotoFuture<CreateSecurityConfigurationResponse, CreateSecurityConfigurationError>;

    /// <p>Creates a new table definition in the Data Catalog.</p>
    fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> RusotoFuture<CreateTableResponse, CreateTableError>;

    /// <p>Creates a new trigger.</p>
    fn create_trigger(
        &self,
        input: CreateTriggerRequest,
    ) -> RusotoFuture<CreateTriggerResponse, CreateTriggerError>;

    /// <p>Creates a new function definition in the Data Catalog.</p>
    fn create_user_defined_function(
        &self,
        input: CreateUserDefinedFunctionRequest,
    ) -> RusotoFuture<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError>;

    /// <p>Removes a classifier from the Data Catalog.</p>
    fn delete_classifier(
        &self,
        input: DeleteClassifierRequest,
    ) -> RusotoFuture<DeleteClassifierResponse, DeleteClassifierError>;

    /// <p>Deletes a connection from the Data Catalog.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<DeleteConnectionResponse, DeleteConnectionError>;

    /// <p>Removes a specified crawler from the Data Catalog, unless the crawler state is <code>RUNNING</code>.</p>
    fn delete_crawler(
        &self,
        input: DeleteCrawlerRequest,
    ) -> RusotoFuture<DeleteCrawlerResponse, DeleteCrawlerError>;

    /// <p><p>Removes a specified Database from a Data Catalog.</p> <note> <p>After completing this operation, you will no longer have access to the tables (and all table versions and partitions that might belong to the tables) and the user-defined functions in the deleted database. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>DeleteDatabase</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, <code>DeletePartition</code> or <code>BatchDeletePartition</code>, <code>DeleteUserDefinedFunction</code>, and <code>DeleteTable</code> or <code>BatchDeleteTable</code>, to delete any resources that belong to the database.</p> </note></p>
    fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> RusotoFuture<DeleteDatabaseResponse, DeleteDatabaseError>;

    /// <p>Deletes a specified DevEndpoint.</p>
    fn delete_dev_endpoint(
        &self,
        input: DeleteDevEndpointRequest,
    ) -> RusotoFuture<DeleteDevEndpointResponse, DeleteDevEndpointError>;

    /// <p>Deletes a specified job definition. If the job definition is not found, no exception is thrown.</p>
    fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> RusotoFuture<DeleteJobResponse, DeleteJobError>;

    /// <p>Deletes a specified partition.</p>
    fn delete_partition(
        &self,
        input: DeletePartitionRequest,
    ) -> RusotoFuture<DeletePartitionResponse, DeletePartitionError>;

    /// <p>Deletes a specified policy.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<DeleteResourcePolicyResponse, DeleteResourcePolicyError>;

    /// <p>Deletes a specified security configuration.</p>
    fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationRequest,
    ) -> RusotoFuture<DeleteSecurityConfigurationResponse, DeleteSecurityConfigurationError>;

    /// <p><p>Removes a table definition from the Data Catalog.</p> <note> <p>After completing this operation, you will no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>DeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> RusotoFuture<DeleteTableResponse, DeleteTableError>;

    /// <p>Deletes a specified version of a table.</p>
    fn delete_table_version(
        &self,
        input: DeleteTableVersionRequest,
    ) -> RusotoFuture<DeleteTableVersionResponse, DeleteTableVersionError>;

    /// <p>Deletes a specified trigger. If the trigger is not found, no exception is thrown.</p>
    fn delete_trigger(
        &self,
        input: DeleteTriggerRequest,
    ) -> RusotoFuture<DeleteTriggerResponse, DeleteTriggerError>;

    /// <p>Deletes an existing function definition from the Data Catalog.</p>
    fn delete_user_defined_function(
        &self,
        input: DeleteUserDefinedFunctionRequest,
    ) -> RusotoFuture<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError>;

    /// <p>Retrieves the status of a migration operation.</p>
    fn get_catalog_import_status(
        &self,
        input: GetCatalogImportStatusRequest,
    ) -> RusotoFuture<GetCatalogImportStatusResponse, GetCatalogImportStatusError>;

    /// <p>Retrieve a classifier by name.</p>
    fn get_classifier(
        &self,
        input: GetClassifierRequest,
    ) -> RusotoFuture<GetClassifierResponse, GetClassifierError>;

    /// <p>Lists all classifier objects in the Data Catalog.</p>
    fn get_classifiers(
        &self,
        input: GetClassifiersRequest,
    ) -> RusotoFuture<GetClassifiersResponse, GetClassifiersError>;

    /// <p>Retrieves a connection definition from the Data Catalog.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError>;

    /// <p>Retrieves a list of connection definitions from the Data Catalog.</p>
    fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> RusotoFuture<GetConnectionsResponse, GetConnectionsError>;

    /// <p>Retrieves metadata for a specified crawler.</p>
    fn get_crawler(
        &self,
        input: GetCrawlerRequest,
    ) -> RusotoFuture<GetCrawlerResponse, GetCrawlerError>;

    /// <p>Retrieves metrics about specified crawlers.</p>
    fn get_crawler_metrics(
        &self,
        input: GetCrawlerMetricsRequest,
    ) -> RusotoFuture<GetCrawlerMetricsResponse, GetCrawlerMetricsError>;

    /// <p>Retrieves metadata for all crawlers defined in the customer account.</p>
    fn get_crawlers(
        &self,
        input: GetCrawlersRequest,
    ) -> RusotoFuture<GetCrawlersResponse, GetCrawlersError>;

    /// <p>Retrieves the security configuration for a specified catalog.</p>
    fn get_data_catalog_encryption_settings(
        &self,
        input: GetDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<GetDataCatalogEncryptionSettingsResponse, GetDataCatalogEncryptionSettingsError>;

    /// <p>Retrieves the definition of a specified database.</p>
    fn get_database(
        &self,
        input: GetDatabaseRequest,
    ) -> RusotoFuture<GetDatabaseResponse, GetDatabaseError>;

    /// <p>Retrieves all Databases defined in a given Data Catalog.</p>
    fn get_databases(
        &self,
        input: GetDatabasesRequest,
    ) -> RusotoFuture<GetDatabasesResponse, GetDatabasesError>;

    /// <p>Transforms a Python script into a directed acyclic graph (DAG). </p>
    fn get_dataflow_graph(
        &self,
        input: GetDataflowGraphRequest,
    ) -> RusotoFuture<GetDataflowGraphResponse, GetDataflowGraphError>;

    /// <p><p>Retrieves information about a specified DevEndpoint.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address, and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoint(
        &self,
        input: GetDevEndpointRequest,
    ) -> RusotoFuture<GetDevEndpointResponse, GetDevEndpointError>;

    /// <p><p>Retrieves all the DevEndpoints in this AWS account.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoints(
        &self,
        input: GetDevEndpointsRequest,
    ) -> RusotoFuture<GetDevEndpointsResponse, GetDevEndpointsError>;

    /// <p>Retrieves an existing job definition.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError>;

    /// <p>Retrieves the metadata for a given job run.</p>
    fn get_job_run(
        &self,
        input: GetJobRunRequest,
    ) -> RusotoFuture<GetJobRunResponse, GetJobRunError>;

    /// <p>Retrieves metadata for all runs of a given job definition.</p>
    fn get_job_runs(
        &self,
        input: GetJobRunsRequest,
    ) -> RusotoFuture<GetJobRunsResponse, GetJobRunsError>;

    /// <p>Retrieves all current job definitions.</p>
    fn get_jobs(&self, input: GetJobsRequest) -> RusotoFuture<GetJobsResponse, GetJobsError>;

    /// <p>Creates mappings.</p>
    fn get_mapping(
        &self,
        input: GetMappingRequest,
    ) -> RusotoFuture<GetMappingResponse, GetMappingError>;

    /// <p>Retrieves information about a specified partition.</p>
    fn get_partition(
        &self,
        input: GetPartitionRequest,
    ) -> RusotoFuture<GetPartitionResponse, GetPartitionError>;

    /// <p>Retrieves information about the partitions in a table.</p>
    fn get_partitions(
        &self,
        input: GetPartitionsRequest,
    ) -> RusotoFuture<GetPartitionsResponse, GetPartitionsError>;

    /// <p>Gets code to perform a specified mapping.</p>
    fn get_plan(&self, input: GetPlanRequest) -> RusotoFuture<GetPlanResponse, GetPlanError>;

    /// <p>Retrieves a specified resource policy.</p>
    fn get_resource_policy(
        &self,
    ) -> RusotoFuture<GetResourcePolicyResponse, GetResourcePolicyError>;

    /// <p>Retrieves a specified security configuration.</p>
    fn get_security_configuration(
        &self,
        input: GetSecurityConfigurationRequest,
    ) -> RusotoFuture<GetSecurityConfigurationResponse, GetSecurityConfigurationError>;

    /// <p>Retrieves a list of all security configurations.</p>
    fn get_security_configurations(
        &self,
        input: GetSecurityConfigurationsRequest,
    ) -> RusotoFuture<GetSecurityConfigurationsResponse, GetSecurityConfigurationsError>;

    /// <p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>
    fn get_table(&self, input: GetTableRequest) -> RusotoFuture<GetTableResponse, GetTableError>;

    /// <p>Retrieves a specified version of a table.</p>
    fn get_table_version(
        &self,
        input: GetTableVersionRequest,
    ) -> RusotoFuture<GetTableVersionResponse, GetTableVersionError>;

    /// <p>Retrieves a list of strings that identify available versions of a specified table.</p>
    fn get_table_versions(
        &self,
        input: GetTableVersionsRequest,
    ) -> RusotoFuture<GetTableVersionsResponse, GetTableVersionsError>;

    /// <p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>
    fn get_tables(
        &self,
        input: GetTablesRequest,
    ) -> RusotoFuture<GetTablesResponse, GetTablesError>;

    /// <p>Retrieves the definition of a trigger.</p>
    fn get_trigger(
        &self,
        input: GetTriggerRequest,
    ) -> RusotoFuture<GetTriggerResponse, GetTriggerError>;

    /// <p>Gets all the triggers associated with a job.</p>
    fn get_triggers(
        &self,
        input: GetTriggersRequest,
    ) -> RusotoFuture<GetTriggersResponse, GetTriggersError>;

    /// <p>Retrieves a specified function definition from the Data Catalog.</p>
    fn get_user_defined_function(
        &self,
        input: GetUserDefinedFunctionRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError>;

    /// <p>Retrieves a multiple function definitions from the Data Catalog.</p>
    fn get_user_defined_functions(
        &self,
        input: GetUserDefinedFunctionsRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError>;

    /// <p>Imports an existing Athena Data Catalog to AWS Glue</p>
    fn import_catalog_to_glue(
        &self,
        input: ImportCatalogToGlueRequest,
    ) -> RusotoFuture<ImportCatalogToGlueResponse, ImportCatalogToGlueError>;

    /// <p>Sets the security configuration for a specified catalog. Once the configuration has been set, the specified encryption is applied to every catalog write thereafter.</p>
    fn put_data_catalog_encryption_settings(
        &self,
        input: PutDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<PutDataCatalogEncryptionSettingsResponse, PutDataCatalogEncryptionSettingsError>;

    /// <p>Sets the Data Catalog resource policy for access control.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError>;

    /// <p>Resets a bookmark entry.</p>
    fn reset_job_bookmark(
        &self,
        input: ResetJobBookmarkRequest,
    ) -> RusotoFuture<ResetJobBookmarkResponse, ResetJobBookmarkError>;

    /// <p>Starts a crawl using the specified crawler, regardless of what is scheduled. If the crawler is already running, returns a <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-exceptions.html#aws-glue-api-exceptions-CrawlerRunningException">CrawlerRunningException</a>.</p>
    fn start_crawler(
        &self,
        input: StartCrawlerRequest,
    ) -> RusotoFuture<StartCrawlerResponse, StartCrawlerError>;

    /// <p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>
    fn start_crawler_schedule(
        &self,
        input: StartCrawlerScheduleRequest,
    ) -> RusotoFuture<StartCrawlerScheduleResponse, StartCrawlerScheduleError>;

    /// <p>Starts a job run using a job definition.</p>
    fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> RusotoFuture<StartJobRunResponse, StartJobRunError>;

    /// <p>Starts an existing trigger. See <a href="http://docs.aws.amazon.com/glue/latest/dg/trigger-job.html">Triggering Jobs</a> for information about how different types of trigger are started.</p>
    fn start_trigger(
        &self,
        input: StartTriggerRequest,
    ) -> RusotoFuture<StartTriggerResponse, StartTriggerError>;

    /// <p>If the specified crawler is running, stops the crawl.</p>
    fn stop_crawler(
        &self,
        input: StopCrawlerRequest,
    ) -> RusotoFuture<StopCrawlerResponse, StopCrawlerError>;

    /// <p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>
    fn stop_crawler_schedule(
        &self,
        input: StopCrawlerScheduleRequest,
    ) -> RusotoFuture<StopCrawlerScheduleResponse, StopCrawlerScheduleError>;

    /// <p>Stops a specified trigger.</p>
    fn stop_trigger(
        &self,
        input: StopTriggerRequest,
    ) -> RusotoFuture<StopTriggerResponse, StopTriggerError>;

    /// <p>Modifies an existing classifier (a <code>GrokClassifier</code>, <code>XMLClassifier</code>, or <code>JsonClassifier</code>, depending on which field is present).</p>
    fn update_classifier(
        &self,
        input: UpdateClassifierRequest,
    ) -> RusotoFuture<UpdateClassifierResponse, UpdateClassifierError>;

    /// <p>Updates a connection definition in the Data Catalog.</p>
    fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> RusotoFuture<UpdateConnectionResponse, UpdateConnectionError>;

    /// <p>Updates a crawler. If a crawler is running, you must stop it using <code>StopCrawler</code> before updating it.</p>
    fn update_crawler(
        &self,
        input: UpdateCrawlerRequest,
    ) -> RusotoFuture<UpdateCrawlerResponse, UpdateCrawlerError>;

    /// <p>Updates the schedule of a crawler using a <code>cron</code> expression. </p>
    fn update_crawler_schedule(
        &self,
        input: UpdateCrawlerScheduleRequest,
    ) -> RusotoFuture<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError>;

    /// <p>Updates an existing database definition in a Data Catalog.</p>
    fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> RusotoFuture<UpdateDatabaseResponse, UpdateDatabaseError>;

    /// <p>Updates a specified DevEndpoint.</p>
    fn update_dev_endpoint(
        &self,
        input: UpdateDevEndpointRequest,
    ) -> RusotoFuture<UpdateDevEndpointResponse, UpdateDevEndpointError>;

    /// <p>Updates an existing job definition.</p>
    fn update_job(
        &self,
        input: UpdateJobRequest,
    ) -> RusotoFuture<UpdateJobResponse, UpdateJobError>;

    /// <p>Updates a partition.</p>
    fn update_partition(
        &self,
        input: UpdatePartitionRequest,
    ) -> RusotoFuture<UpdatePartitionResponse, UpdatePartitionError>;

    /// <p>Updates a metadata table in the Data Catalog.</p>
    fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> RusotoFuture<UpdateTableResponse, UpdateTableError>;

    /// <p>Updates a trigger definition.</p>
    fn update_trigger(
        &self,
        input: UpdateTriggerRequest,
    ) -> RusotoFuture<UpdateTriggerResponse, UpdateTriggerError>;

    /// <p>Updates an existing function definition in the Data Catalog.</p>
    fn update_user_defined_function(
        &self,
        input: UpdateUserDefinedFunctionRequest,
    ) -> RusotoFuture<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError>;
}
/// A client for the AWS Glue API.
#[derive(Clone)]
pub struct GlueClient {
    client: Client,
    region: region::Region,
}

impl GlueClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GlueClient {
        GlueClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GlueClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        GlueClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Glue for GlueClient {
    /// <p>Creates one or more partitions in a batch operation.</p>
    fn batch_create_partition(
        &self,
        input: BatchCreatePartitionRequest,
    ) -> RusotoFuture<BatchCreatePartitionResponse, BatchCreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchCreatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchCreatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchCreatePartitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a list of connection definitions from the Data Catalog.</p>
    fn batch_delete_connection(
        &self,
        input: BatchDeleteConnectionRequest,
    ) -> RusotoFuture<BatchDeleteConnectionResponse, BatchDeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeleteConnectionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes one or more partitions in a batch operation.</p>
    fn batch_delete_partition(
        &self,
        input: BatchDeletePartitionRequest,
    ) -> RusotoFuture<BatchDeletePartitionResponse, BatchDeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeletePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeletePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDeletePartitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deletes multiple tables at once.</p> <note> <p>After completing this operation, you will no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>BatchDeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn batch_delete_table(
        &self,
        input: BatchDeleteTableRequest,
    ) -> RusotoFuture<BatchDeleteTableResponse, BatchDeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchDeleteTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified batch of versions of a table.</p>
    fn batch_delete_table_version(
        &self,
        input: BatchDeleteTableVersionRequest,
    ) -> RusotoFuture<BatchDeleteTableVersionResponse, BatchDeleteTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchDeleteTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchDeleteTableVersionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchDeleteTableVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves partitions in a batch request.</p>
    fn batch_get_partition(
        &self,
        input: BatchGetPartitionRequest,
    ) -> RusotoFuture<BatchGetPartitionResponse, BatchGetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchGetPartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetPartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetPartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops one or more job runs for a specified job definition.</p>
    fn batch_stop_job_run(
        &self,
        input: BatchStopJobRunRequest,
    ) -> RusotoFuture<BatchStopJobRunResponse, GlueBatchStopJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.BatchStopJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchStopJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GlueBatchStopJobRunError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a classifier in the user's account. This may be a <code>GrokClassifier</code>, an <code>XMLClassifier</code>, or abbrev <code>JsonClassifier</code>, depending on which field of the request is present.</p>
    fn create_classifier(
        &self,
        input: CreateClassifierRequest,
    ) -> RusotoFuture<CreateClassifierResponse, CreateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a connection definition in the Data Catalog.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<CreateConnectionResponse, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new crawler with specified targets, role, configuration, and optional schedule. At least one crawl target must be specified, in the <i>s3Targets</i> field, the <i>jdbcTargets</i> field, or the <i>DynamoDBTargets</i> field.</p>
    fn create_crawler(
        &self,
        input: CreateCrawlerRequest,
    ) -> RusotoFuture<CreateCrawlerResponse, CreateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new database in a Data Catalog.</p>
    fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> RusotoFuture<CreateDatabaseResponse, CreateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new DevEndpoint.</p>
    fn create_dev_endpoint(
        &self,
        input: CreateDevEndpointRequest,
    ) -> RusotoFuture<CreateDevEndpointResponse, CreateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new job definition.</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new partition.</p>
    fn create_partition(
        &self,
        input: CreatePartitionRequest,
    ) -> RusotoFuture<CreatePartitionResponse, CreatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Transforms a directed acyclic graph (DAG) into code.</p>
    fn create_script(
        &self,
        input: CreateScriptRequest,
    ) -> RusotoFuture<CreateScriptResponse, CreateScriptError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateScriptResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateScriptError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new security configuration.</p>
    fn create_security_configuration(
        &self,
        input: CreateSecurityConfigurationRequest,
    ) -> RusotoFuture<CreateSecurityConfigurationResponse, CreateSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new table definition in the Data Catalog.</p>
    fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> RusotoFuture<CreateTableResponse, CreateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new trigger.</p>
    fn create_trigger(
        &self,
        input: CreateTriggerRequest,
    ) -> RusotoFuture<CreateTriggerResponse, CreateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new function definition in the Data Catalog.</p>
    fn create_user_defined_function(
        &self,
        input: CreateUserDefinedFunctionRequest,
    ) -> RusotoFuture<CreateUserDefinedFunctionResponse, CreateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.CreateUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes a classifier from the Data Catalog.</p>
    fn delete_classifier(
        &self,
        input: DeleteClassifierRequest,
    ) -> RusotoFuture<DeleteClassifierResponse, DeleteClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a connection from the Data Catalog.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<DeleteConnectionResponse, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a specified crawler from the Data Catalog, unless the crawler state is <code>RUNNING</code>.</p>
    fn delete_crawler(
        &self,
        input: DeleteCrawlerRequest,
    ) -> RusotoFuture<DeleteCrawlerResponse, DeleteCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Removes a specified Database from a Data Catalog.</p> <note> <p>After completing this operation, you will no longer have access to the tables (and all table versions and partitions that might belong to the tables) and the user-defined functions in the deleted database. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>DeleteDatabase</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, <code>DeletePartition</code> or <code>BatchDeletePartition</code>, <code>DeleteUserDefinedFunction</code>, and <code>DeleteTable</code> or <code>BatchDeleteTable</code>, to delete any resources that belong to the database.</p> </note></p>
    fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> RusotoFuture<DeleteDatabaseResponse, DeleteDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified DevEndpoint.</p>
    fn delete_dev_endpoint(
        &self,
        input: DeleteDevEndpointRequest,
    ) -> RusotoFuture<DeleteDevEndpointResponse, DeleteDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified job definition. If the job definition is not found, no exception is thrown.</p>
    fn delete_job(
        &self,
        input: DeleteJobRequest,
    ) -> RusotoFuture<DeleteJobResponse, DeleteJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified partition.</p>
    fn delete_partition(
        &self,
        input: DeletePartitionRequest,
    ) -> RusotoFuture<DeletePartitionResponse, DeletePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeletePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeletePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified policy.</p>
    fn delete_resource_policy(
        &self,
        input: DeleteResourcePolicyRequest,
    ) -> RusotoFuture<DeleteResourcePolicyResponse, DeleteResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteResourcePolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a specified security configuration.</p>
    fn delete_security_configuration(
        &self,
        input: DeleteSecurityConfigurationRequest,
    ) -> RusotoFuture<DeleteSecurityConfigurationResponse, DeleteSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Removes a table definition from the Data Catalog.</p> <note> <p>After completing this operation, you will no longer have access to the table versions and partitions that belong to the deleted table. AWS Glue deletes these &quot;orphaned&quot; resources asynchronously in a timely manner, at the discretion of the service.</p> <p>To ensure immediate deletion of all related resources, before calling <code>DeleteTable</code>, use <code>DeleteTableVersion</code> or <code>BatchDeleteTableVersion</code>, and <code>DeletePartition</code> or <code>BatchDeletePartition</code>, to delete any resources that belong to the table.</p> </note></p>
    fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> RusotoFuture<DeleteTableResponse, DeleteTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified version of a table.</p>
    fn delete_table_version(
        &self,
        input: DeleteTableVersionRequest,
    ) -> RusotoFuture<DeleteTableVersionResponse, DeleteTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTableVersionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTableVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified trigger. If the trigger is not found, no exception is thrown.</p>
    fn delete_trigger(
        &self,
        input: DeleteTriggerRequest,
    ) -> RusotoFuture<DeleteTriggerResponse, DeleteTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing function definition from the Data Catalog.</p>
    fn delete_user_defined_function(
        &self,
        input: DeleteUserDefinedFunctionRequest,
    ) -> RusotoFuture<DeleteUserDefinedFunctionResponse, DeleteUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.DeleteUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the status of a migration operation.</p>
    fn get_catalog_import_status(
        &self,
        input: GetCatalogImportStatusRequest,
    ) -> RusotoFuture<GetCatalogImportStatusResponse, GetCatalogImportStatusError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCatalogImportStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCatalogImportStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCatalogImportStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieve a classifier by name.</p>
    fn get_classifier(
        &self,
        input: GetClassifierRequest,
    ) -> RusotoFuture<GetClassifierResponse, GetClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all classifier objects in the Data Catalog.</p>
    fn get_classifiers(
        &self,
        input: GetClassifiersRequest,
    ) -> RusotoFuture<GetClassifiersResponse, GetClassifiersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetClassifiers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetClassifiersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetClassifiersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a connection definition from the Data Catalog.</p>
    fn get_connection(
        &self,
        input: GetConnectionRequest,
    ) -> RusotoFuture<GetConnectionResponse, GetConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of connection definitions from the Data Catalog.</p>
    fn get_connections(
        &self,
        input: GetConnectionsRequest,
    ) -> RusotoFuture<GetConnectionsResponse, GetConnectionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for a specified crawler.</p>
    fn get_crawler(
        &self,
        input: GetCrawlerRequest,
    ) -> RusotoFuture<GetCrawlerResponse, GetCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metrics about specified crawlers.</p>
    fn get_crawler_metrics(
        &self,
        input: GetCrawlerMetricsRequest,
    ) -> RusotoFuture<GetCrawlerMetricsResponse, GetCrawlerMetricsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlerMetrics");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlerMetricsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlerMetricsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for all crawlers defined in the customer account.</p>
    fn get_crawlers(
        &self,
        input: GetCrawlersRequest,
    ) -> RusotoFuture<GetCrawlersResponse, GetCrawlersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetCrawlers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCrawlersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCrawlersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the security configuration for a specified catalog.</p>
    fn get_data_catalog_encryption_settings(
        &self,
        input: GetDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<GetDataCatalogEncryptionSettingsResponse, GetDataCatalogEncryptionSettingsError>
    {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDataCatalogEncryptionSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDataCatalogEncryptionSettingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDataCatalogEncryptionSettingsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Retrieves the definition of a specified database.</p>
    fn get_database(
        &self,
        input: GetDatabaseRequest,
    ) -> RusotoFuture<GetDatabaseResponse, GetDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all Databases defined in a given Data Catalog.</p>
    fn get_databases(
        &self,
        input: GetDatabasesRequest,
    ) -> RusotoFuture<GetDatabasesResponse, GetDatabasesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDatabasesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDatabasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Transforms a Python script into a directed acyclic graph (DAG). </p>
    fn get_dataflow_graph(
        &self,
        input: GetDataflowGraphRequest,
    ) -> RusotoFuture<GetDataflowGraphResponse, GetDataflowGraphError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDataflowGraph");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDataflowGraphResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDataflowGraphError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieves information about a specified DevEndpoint.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address, and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoint(
        &self,
        input: GetDevEndpointRequest,
    ) -> RusotoFuture<GetDevEndpointResponse, GetDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Retrieves all the DevEndpoints in this AWS account.</p> <note> <p>When you create a development endpoint in a virtual private cloud (VPC), AWS Glue returns only a private IP address and the public IP address field is not populated. When you create a non-VPC development endpoint, AWS Glue returns only a public IP address.</p> </note></p>
    fn get_dev_endpoints(
        &self,
        input: GetDevEndpointsRequest,
    ) -> RusotoFuture<GetDevEndpointsResponse, GetDevEndpointsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetDevEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDevEndpointsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDevEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves an existing job definition.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the metadata for a given job run.</p>
    fn get_job_run(
        &self,
        input: GetJobRunRequest,
    ) -> RusotoFuture<GetJobRunResponse, GetJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves metadata for all runs of a given job definition.</p>
    fn get_job_runs(
        &self,
        input: GetJobRunsRequest,
    ) -> RusotoFuture<GetJobRunsResponse, GetJobRunsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetJobRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves all current job definitions.</p>
    fn get_jobs(&self, input: GetJobsRequest) -> RusotoFuture<GetJobsResponse, GetJobsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates mappings.</p>
    fn get_mapping(
        &self,
        input: GetMappingRequest,
    ) -> RusotoFuture<GetMappingResponse, GetMappingError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetMapping");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetMappingResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a specified partition.</p>
    fn get_partition(
        &self,
        input: GetPartitionRequest,
    ) -> RusotoFuture<GetPartitionResponse, GetPartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about the partitions in a table.</p>
    fn get_partitions(
        &self,
        input: GetPartitionsRequest,
    ) -> RusotoFuture<GetPartitionsResponse, GetPartitionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPartitions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPartitionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPartitionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets code to perform a specified mapping.</p>
    fn get_plan(&self, input: GetPlanRequest) -> RusotoFuture<GetPlanResponse, GetPlanError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetPlan");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetPlanResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified resource policy.</p>
    fn get_resource_policy(
        &self,
    ) -> RusotoFuture<GetResourcePolicyResponse, GetResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetResourcePolicy");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourcePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified security configuration.</p>
    fn get_security_configuration(
        &self,
        input: GetSecurityConfigurationRequest,
    ) -> RusotoFuture<GetSecurityConfigurationResponse, GetSecurityConfigurationError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetSecurityConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSecurityConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSecurityConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a list of all security configurations.</p>
    fn get_security_configurations(
        &self,
        input: GetSecurityConfigurationsRequest,
    ) -> RusotoFuture<GetSecurityConfigurationsResponse, GetSecurityConfigurationsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetSecurityConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSecurityConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSecurityConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves the <code>Table</code> definition in a Data Catalog for a specified table.</p>
    fn get_table(&self, input: GetTableRequest) -> RusotoFuture<GetTableResponse, GetTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified version of a table.</p>
    fn get_table_version(
        &self,
        input: GetTableVersionRequest,
    ) -> RusotoFuture<GetTableVersionResponse, GetTableVersionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTableVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableVersionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of strings that identify available versions of a specified table.</p>
    fn get_table_versions(
        &self,
        input: GetTableVersionsRequest,
    ) -> RusotoFuture<GetTableVersionsResponse, GetTableVersionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTableVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTableVersionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTableVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the definitions of some or all of the tables in a given <code>Database</code>.</p>
    fn get_tables(
        &self,
        input: GetTablesRequest,
    ) -> RusotoFuture<GetTablesResponse, GetTablesError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTablesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTablesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the definition of a trigger.</p>
    fn get_trigger(
        &self,
        input: GetTriggerRequest,
    ) -> RusotoFuture<GetTriggerResponse, GetTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets all the triggers associated with a job.</p>
    fn get_triggers(
        &self,
        input: GetTriggersRequest,
    ) -> RusotoFuture<GetTriggersResponse, GetTriggersError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetTriggers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTriggersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTriggersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specified function definition from the Data Catalog.</p>
    fn get_user_defined_function(
        &self,
        input: GetUserDefinedFunctionRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionResponse, GetUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetUserDefinedFunctionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a multiple function definitions from the Data Catalog.</p>
    fn get_user_defined_functions(
        &self,
        input: GetUserDefinedFunctionsRequest,
    ) -> RusotoFuture<GetUserDefinedFunctionsResponse, GetUserDefinedFunctionsError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.GetUserDefinedFunctions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserDefinedFunctionsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUserDefinedFunctionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Imports an existing Athena Data Catalog to AWS Glue</p>
    fn import_catalog_to_glue(
        &self,
        input: ImportCatalogToGlueRequest,
    ) -> RusotoFuture<ImportCatalogToGlueResponse, ImportCatalogToGlueError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ImportCatalogToGlue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportCatalogToGlueResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportCatalogToGlueError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Sets the security configuration for a specified catalog. Once the configuration has been set, the specified encryption is applied to every catalog write thereafter.</p>
    fn put_data_catalog_encryption_settings(
        &self,
        input: PutDataCatalogEncryptionSettingsRequest,
    ) -> RusotoFuture<PutDataCatalogEncryptionSettingsResponse, PutDataCatalogEncryptionSettingsError>
    {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.PutDataCatalogEncryptionSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutDataCatalogEncryptionSettingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutDataCatalogEncryptionSettingsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Sets the Data Catalog resource policy for access control.</p>
    fn put_resource_policy(
        &self,
        input: PutResourcePolicyRequest,
    ) -> RusotoFuture<PutResourcePolicyResponse, PutResourcePolicyError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.PutResourcePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutResourcePolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutResourcePolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Resets a bookmark entry.</p>
    fn reset_job_bookmark(
        &self,
        input: ResetJobBookmarkRequest,
    ) -> RusotoFuture<ResetJobBookmarkResponse, ResetJobBookmarkError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.ResetJobBookmark");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ResetJobBookmarkResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ResetJobBookmarkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a crawl using the specified crawler, regardless of what is scheduled. If the crawler is already running, returns a <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-exceptions.html#aws-glue-api-exceptions-CrawlerRunningException">CrawlerRunningException</a>.</p>
    fn start_crawler(
        &self,
        input: StartCrawlerRequest,
    ) -> RusotoFuture<StartCrawlerResponse, StartCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the schedule state of the specified crawler to <code>SCHEDULED</code>, unless the crawler is already running or the schedule state is already <code>SCHEDULED</code>.</p>
    fn start_crawler_schedule(
        &self,
        input: StartCrawlerScheduleRequest,
    ) -> RusotoFuture<StartCrawlerScheduleResponse, StartCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts a job run using a job definition.</p>
    fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> RusotoFuture<StartJobRunResponse, StartJobRunError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartJobRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartJobRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartJobRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts an existing trigger. See <a href="http://docs.aws.amazon.com/glue/latest/dg/trigger-job.html">Triggering Jobs</a> for information about how different types of trigger are started.</p>
    fn start_trigger(
        &self,
        input: StartTriggerRequest,
    ) -> RusotoFuture<StartTriggerResponse, StartTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StartTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>If the specified crawler is running, stops the crawl.</p>
    fn stop_crawler(
        &self,
        input: StopCrawlerRequest,
    ) -> RusotoFuture<StopCrawlerResponse, StopCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the schedule state of the specified crawler to <code>NOT_SCHEDULED</code>, but does not stop the crawler if it is already running.</p>
    fn stop_crawler_schedule(
        &self,
        input: StopCrawlerScheduleRequest,
    ) -> RusotoFuture<StopCrawlerScheduleResponse, StopCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a specified trigger.</p>
    fn stop_trigger(
        &self,
        input: StopTriggerRequest,
    ) -> RusotoFuture<StopTriggerResponse, StopTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.StopTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies an existing classifier (a <code>GrokClassifier</code>, <code>XMLClassifier</code>, or <code>JsonClassifier</code>, depending on which field is present).</p>
    fn update_classifier(
        &self,
        input: UpdateClassifierRequest,
    ) -> RusotoFuture<UpdateClassifierResponse, UpdateClassifierError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateClassifier");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateClassifierResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateClassifierError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a connection definition in the Data Catalog.</p>
    fn update_connection(
        &self,
        input: UpdateConnectionRequest,
    ) -> RusotoFuture<UpdateConnectionResponse, UpdateConnectionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a crawler. If a crawler is running, you must stop it using <code>StopCrawler</code> before updating it.</p>
    fn update_crawler(
        &self,
        input: UpdateCrawlerRequest,
    ) -> RusotoFuture<UpdateCrawlerResponse, UpdateCrawlerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawler");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCrawlerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateCrawlerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the schedule of a crawler using a <code>cron</code> expression. </p>
    fn update_crawler_schedule(
        &self,
        input: UpdateCrawlerScheduleRequest,
    ) -> RusotoFuture<UpdateCrawlerScheduleResponse, UpdateCrawlerScheduleError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateCrawlerSchedule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCrawlerScheduleResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateCrawlerScheduleError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates an existing database definition in a Data Catalog.</p>
    fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> RusotoFuture<UpdateDatabaseResponse, UpdateDatabaseError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDatabaseResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDatabaseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a specified DevEndpoint.</p>
    fn update_dev_endpoint(
        &self,
        input: UpdateDevEndpointRequest,
    ) -> RusotoFuture<UpdateDevEndpointResponse, UpdateDevEndpointError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateDevEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDevEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDevEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing job definition.</p>
    fn update_job(
        &self,
        input: UpdateJobRequest,
    ) -> RusotoFuture<UpdateJobResponse, UpdateJobError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateJobResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a partition.</p>
    fn update_partition(
        &self,
        input: UpdatePartitionRequest,
    ) -> RusotoFuture<UpdatePartitionResponse, UpdatePartitionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdatePartition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdatePartitionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePartitionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a metadata table in the Data Catalog.</p>
    fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> RusotoFuture<UpdateTableResponse, UpdateTableError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTableResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTableError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a trigger definition.</p>
    fn update_trigger(
        &self,
        input: UpdateTriggerRequest,
    ) -> RusotoFuture<UpdateTriggerResponse, UpdateTriggerError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateTrigger");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateTriggerResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTriggerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing function definition in the Data Catalog.</p>
    fn update_user_defined_function(
        &self,
        input: UpdateUserDefinedFunctionRequest,
    ) -> RusotoFuture<UpdateUserDefinedFunctionResponse, UpdateUserDefinedFunctionError> {
        let mut request = SignedRequest::new("POST", "glue", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSGlue.UpdateUserDefinedFunction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateUserDefinedFunctionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUserDefinedFunctionError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
