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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetNamedQueryInput {
    /// <p>An array of query IDs.</p>
    #[serde(rename = "NamedQueryIds")]
    pub named_query_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetNamedQueryOutput {
    /// <p>Information about the named query IDs submitted.</p>
    #[serde(rename = "NamedQueries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_queries: Option<Vec<NamedQuery>>,
    /// <p>Information about provided query IDs.</p>
    #[serde(rename = "UnprocessedNamedQueryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_named_query_ids: Option<Vec<UnprocessedNamedQueryId>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetQueryExecutionInput {
    /// <p>An array of query execution IDs.</p>
    #[serde(rename = "QueryExecutionIds")]
    pub query_execution_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetQueryExecutionOutput {
    /// <p>Information about a query execution.</p>
    #[serde(rename = "QueryExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_executions: Option<Vec<QueryExecution>>,
    /// <p>Information about the query executions that failed to run.</p>
    #[serde(rename = "UnprocessedQueryExecutionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_query_execution_ids: Option<Vec<UnprocessedQueryExecutionId>>,
}

/// <p>Information about the columns in a query execution result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ColumnInfo {
    /// <p>Indicates whether values in the column are case-sensitive.</p>
    #[serde(rename = "CaseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// <p>The catalog to which the query results belong.</p>
    #[serde(rename = "CatalogName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    /// <p>A column label.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The name of the column.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Indicates the column's nullable status.</p>
    #[serde(rename = "Nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    /// <p>For <code>DECIMAL</code> data types, specifies the total number of digits, up to 38. For performance reasons, we recommend up to 18 digits.</p>
    #[serde(rename = "Precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    /// <p>For <code>DECIMAL</code> data types, specifies the total number of digits in the fractional part of the value. Defaults to 0.</p>
    #[serde(rename = "Scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    /// <p>The schema name (database name) to which the query results belong.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The table name for the query results.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The data type of the column.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNamedQueryInput {
    /// <p><p>A unique case-sensitive string used to ensure the request to create the query is idempotent (executes only once). If another <code>CreateNamedQuery</code> request is received, the same response is returned and another query is not created. If a parameter has changed, for example, the <code>QueryString</code>, an error is returned.</p> <important> <p>This token is listed as not required because AWS SDKs (for example the AWS SDK for Java) auto-generate the token for users. If you are not using the AWS SDK or the AWS CLI, you must provide this token or the action will fail.</p> </important></p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The database to which the query belongs.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>The query description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The query name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The contents of the query with all query statements.</p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
    /// <p>The name of the workgroup in which the named query is being created.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateNamedQueryOutput {
    /// <p>The unique ID of the query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateWorkGroupInput {
    /// <p>The configuration for the workgroup, which includes the location in Amazon S3 where query results are stored, the encryption configuration, if any, used for encrypting query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup, the limit for the amount of bytes scanned (cutoff) per query, if it is specified, and whether workgroup's settings (specified with EnforceWorkGroupConfiguration) in the WorkGroupConfiguration override client-side settings. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkGroupConfiguration>,
    /// <p>The workgroup description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The workgroup name.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateWorkGroupOutput {}

/// <p>A piece of data (a field in the table).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Datum {
    /// <p>The value of the datum.</p>
    #[serde(rename = "VarCharValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_char_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNamedQueryInput {
    /// <p>The unique ID of the query to delete.</p>
    #[serde(rename = "NamedQueryId")]
    pub named_query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteNamedQueryOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteWorkGroupInput {
    /// <p>The option to delete the workgroup and its contents even if the workgroup contains any named queries.</p>
    #[serde(rename = "RecursiveDeleteOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_delete_option: Option<bool>,
    /// <p>The unique name of the workgroup to delete.</p>
    #[serde(rename = "WorkGroup")]
    pub work_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteWorkGroupOutput {}

/// <p>If query results are encrypted in Amazon S3, indicates the encryption option used (for example, <code>SSE-KMS</code> or <code>CSE-KMS</code>) and key information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// <p>Indicates whether Amazon S3 server-side encryption with Amazon S3-managed keys (<code>SSE-S3</code>), server-side encryption with KMS-managed keys (<code>SSE-KMS</code>), or client-side encryption with KMS-managed keys (CSE-KMS) is used.</p> <p>If a query runs in a workgroup and the workgroup overrides client-side settings, then the workgroup's setting for encryption is used. It specifies whether query results must be encrypted, for all queries that run in this workgroup. </p>
    #[serde(rename = "EncryptionOption")]
    pub encryption_option: String,
    /// <p>For <code>SSE-KMS</code> and <code>CSE-KMS</code>, this is the KMS key ARN or ID.</p>
    #[serde(rename = "KmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetNamedQueryInput {
    /// <p>The unique ID of the query. Use <a>ListNamedQueries</a> to get query IDs.</p>
    #[serde(rename = "NamedQueryId")]
    pub named_query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetNamedQueryOutput {
    /// <p>Information about the query.</p>
    #[serde(rename = "NamedQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query: Option<NamedQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetQueryExecutionInput {
    /// <p>The unique ID of the query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetQueryExecutionOutput {
    /// <p>Information about the query execution.</p>
    #[serde(rename = "QueryExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution: Option<QueryExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetQueryResultsInput {
    /// <p>The maximum number of results (rows) to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies where to start pagination if a previous request was truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique ID of the query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetQueryResultsOutput {
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The results of the query execution.</p>
    #[serde(rename = "ResultSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set: Option<ResultSet>,
    /// <p>The number of rows inserted with a CREATE TABLE AS SELECT statement. </p>
    #[serde(rename = "UpdateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetWorkGroupInput {
    /// <p>The name of the workgroup.</p>
    #[serde(rename = "WorkGroup")]
    pub work_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetWorkGroupOutput {
    /// <p>Information about the workgroup.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<WorkGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNamedQueriesInput {
    /// <p>The maximum number of queries to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies where to start pagination if a previous request was truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the workgroup from which the named queries are being returned.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListNamedQueriesOutput {
    /// <p>The list of unique query IDs.</p>
    #[serde(rename = "NamedQueryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_ids: Option<Vec<String>>,
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListQueryExecutionsInput {
    /// <p>The maximum number of query executions to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token that specifies where to start pagination if a previous request was truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the workgroup from which queries are being returned.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListQueryExecutionsOutput {
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique IDs of each query execution as an array of strings.</p>
    #[serde(rename = "QueryExecutionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListWorkGroupsInput {
    /// <p>The maximum number of workgroups to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListWorkGroupsOutput {
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of workgroups, including their names, descriptions, creation times, and states.</p>
    #[serde(rename = "WorkGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_groups: Option<Vec<WorkGroupSummary>>,
}

/// <p>A query, where <code>QueryString</code> is the list of SQL query statements that comprise the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NamedQuery {
    /// <p>The database to which the query belongs.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>The query description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The query name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The unique identifier of the query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
    /// <p>The SQL query statements that comprise the query.</p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
    /// <p>The name of the workgroup that contains the named query.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

/// <p>Information about a single instance of a query execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryExecution {
    /// <p>The SQL query statements which the query execution ran.</p>
    #[serde(rename = "Query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// <p>The database in which the query execution occurred.</p>
    #[serde(rename = "QueryExecutionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_context: Option<QueryExecutionContext>,
    /// <p>The unique identifier for each query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
    /// <p>The location in Amazon S3 where query results were stored and the encryption option, if any, used for query results. These are known as "client-side settings". If workgroup settings override client-side settings, then the query uses the location for the query results and the encryption configuration that are specified for the workgroup.</p>
    #[serde(rename = "ResultConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
    /// <p>The type of query statement that was run. <code>DDL</code> indicates DDL query statements. <code>DML</code> indicates DML (Data Manipulation Language) query statements, such as <code>CREATE TABLE AS SELECT</code>. <code>UTILITY</code> indicates query statements other than DDL and DML, such as <code>SHOW CREATE TABLE</code>, or <code>DESCRIBE &lt;table&gt;</code>.</p>
    #[serde(rename = "StatementType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<String>,
    /// <p>The amount of data scanned during the query execution and the amount of time that it took to execute, and the type of statement that was run.</p>
    #[serde(rename = "Statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<QueryExecutionStatistics>,
    /// <p>The completion date, current state, submission time, and state change reason (if applicable) for the query execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryExecutionStatus>,
    /// <p>The name of the workgroup in which the query ran.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

/// <p>The database in which the query execution occurs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryExecutionContext {
    /// <p>The name of the database.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

/// <p>The amount of data scanned during the query execution and the amount of time that it took to execute, and the type of statement that was run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryExecutionStatistics {
    /// <p>The number of bytes in the data that was queried.</p>
    #[serde(rename = "DataScannedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_scanned_in_bytes: Option<i64>,
    /// <p>The number of milliseconds that the query took to execute.</p>
    #[serde(rename = "EngineExecutionTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_execution_time_in_millis: Option<i64>,
}

/// <p>The completion date, current state, submission time, and state change reason (if applicable) for the query execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryExecutionStatus {
    /// <p>The date and time that the query completed.</p>
    #[serde(rename = "CompletionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date_time: Option<f64>,
    /// <p>The state of query execution. <code>QUEUED</code> state is listed but is not used by Athena and is reserved for future use. <code>RUNNING</code> indicates that the query has been submitted to the service, and Athena will execute the query as soon as resources are available. <code>SUCCEEDED</code> indicates that the query completed without errors. <code>FAILED</code> indicates that the query experienced an error and did not complete processing. <code>CANCELLED</code> indicates that a user input interrupted query execution. </p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Further detail about the status of the query.</p>
    #[serde(rename = "StateChangeReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
    /// <p>The date and time that the query was submitted.</p>
    #[serde(rename = "SubmissionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_date_time: Option<f64>,
}

/// <p>The location in Amazon S3 where query results are stored and the encryption option, if any, used for query results. These are known as "client-side settings". If workgroup settings override client-side settings, then the query uses the location for the query results and the encryption configuration that are specified for the workgroup.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultConfiguration {
    /// <p>If query results are encrypted in Amazon S3, indicates the encryption option used (for example, <code>SSE-KMS</code> or <code>CSE-KMS</code>) and key information. This is a client-side setting. If workgroup settings override client-side settings, then the query uses the encryption configuration that is specified for the workgroup, and also uses the location for storing query results specified in the workgroup. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a> and <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The location in Amazon S3 where your query results are stored, such as <code>s3://path/to/query/bucket/</code>. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Queries and Query Result Files.</a> If workgroup settings override client-side settings, then the query uses the location for the query results and the encryption configuration that are specified for the workgroup. The "workgroup settings override" is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
}

/// <p>The information about the updates in the query results, such as output location and encryption configuration for the query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResultConfigurationUpdates {
    /// <p>The encryption configuration for the query results.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The location in Amazon S3 where your query results are stored, such as <code>s3://path/to/query/bucket/</code>. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Queries and Query Result Files.</a> If workgroup settings override client-side settings, then the query uses the location for the query results and the encryption configuration that are specified for the workgroup. The "workgroup settings override" is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    /// <p>If set to "true", indicates that the previously-specified encryption configuration (also known as the client-side setting) for queries in this workgroup should be ignored and set to null. If set to "false" or not set, and a value is present in the EncryptionConfiguration in ResultConfigurationUpdates (the client-side setting), the EncryptionConfiguration in the workgroup's ResultConfiguration will be updated with the new value. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "RemoveEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_encryption_configuration: Option<bool>,
    /// <p>If set to "true", indicates that the previously-specified query results location (also known as a client-side setting) for queries in this workgroup should be ignored and set to null. If set to "false" or not set, and a value is present in the OutputLocation in ResultConfigurationUpdates (the client-side setting), the OutputLocation in the workgroup's ResultConfiguration will be updated with the new value. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "RemoveOutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_output_location: Option<bool>,
}

/// <p>The metadata and rows that comprise a query result set. The metadata describes the column structure and data types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultSet {
    /// <p>The metadata that describes the column structure and data types of a table of query results.</p>
    #[serde(rename = "ResultSetMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_metadata: Option<ResultSetMetadata>,
    /// <p>The rows in the table.</p>
    #[serde(rename = "Rows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Row>>,
}

/// <p>The metadata that describes the column structure and data types of a table of query results. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultSetMetadata {
    /// <p>Information about the columns returned in a query result metadata.</p>
    #[serde(rename = "ColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_info: Option<Vec<ColumnInfo>>,
}

/// <p>The rows that comprise a query result table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Row {
    /// <p>The data that populates a row in a query result table.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Datum>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartQueryExecutionInput {
    /// <p><p>A unique case-sensitive string used to ensure the request to create the query is idempotent (executes only once). If another <code>StartQueryExecution</code> request is received, the same response is returned and another query is not created. If a parameter has changed, for example, the <code>QueryString</code>, an error is returned.</p> <important> <p>This token is listed as not required because AWS SDKs (for example the AWS SDK for Java) auto-generate the token for users. If you are not using the AWS SDK or the AWS CLI, you must provide this token or the action will fail.</p> </important></p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The database within which the query executes.</p>
    #[serde(rename = "QueryExecutionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_context: Option<QueryExecutionContext>,
    /// <p>The SQL query statements to be executed.</p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
    /// <p>Specifies information about where and how to save the results of the query execution. If the query runs in a workgroup, then workgroup's settings may override query settings. This affects the query results location. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "ResultConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
    /// <p>The name of the workgroup in which the query is being started.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartQueryExecutionOutput {
    /// <p>The unique ID of the query that ran as a result of this request.</p>
    #[serde(rename = "QueryExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopQueryExecutionInput {
    /// <p>The unique ID of the query execution to stop.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopQueryExecutionOutput {}

/// <p>Information about a named query ID that could not be processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedNamedQueryId {
    /// <p>The error code returned when the processing request for the named query failed, if applicable.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message returned when the processing request for the named query failed, if applicable.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The unique identifier of the named query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

/// <p>Describes a query execution that failed to process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnprocessedQueryExecutionId {
    /// <p>The error code returned when the query execution failed to process, if applicable.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message returned when the query execution failed to process, if applicable.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The unique identifier of the query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateWorkGroupInput {
    /// <p>The workgroup configuration that will be updated for the given workgroup.</p>
    #[serde(rename = "ConfigurationUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_updates: Option<WorkGroupConfigurationUpdates>,
    /// <p>The workgroup description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The workgroup state that will be updated for the given workgroup.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The specified workgroup that will be updated.</p>
    #[serde(rename = "WorkGroup")]
    pub work_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateWorkGroupOutput {}

/// <p>A workgroup, which contains a name, description, creation time, state, and other configuration, listed under <a>WorkGroup$Configuration</a>. Each workgroup enables you to isolate queries for you or your group of users from other queries in the same account, to configure the query results location and the encryption configuration (known as workgroup settings), to enable sending query metrics to Amazon CloudWatch, and to establish per-query data usage control limits for all queries in a workgroup. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WorkGroup {
    /// <p>The configuration of the workgroup, which includes the location in Amazon S3 where query results are stored, the encryption configuration, if any, used for query results; whether the Amazon CloudWatch Metrics are enabled for the workgroup; whether workgroup settings override client-side settings; and the data usage limit for the amount of data scanned per query, if it is specified. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkGroupConfiguration>,
    /// <p>The date and time the workgroup was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The workgroup description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The workgroup name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The state of the workgroup: ENABLED or DISABLED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The configuration of the workgroup, which includes the location in Amazon S3 where query results are stored, the encryption option, if any, used for query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup and whether workgroup settings override query settings, and the data usage limit for the amount of data scanned per query, if it is specified. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkGroupConfiguration {
    /// <p>The upper data usage limit (cutoff) for the amount of bytes a single query in a workgroup is allowed to scan.</p>
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned_cutoff_per_query: Option<i64>,
    /// <p>If set to "true", the settings for the workgroup override client-side settings. If set to "false", client-side settings are used. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_work_group_configuration: Option<bool>,
    /// <p>Indicates that the Amazon CloudWatch metrics are enabled for the workgroup.</p>
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_cloud_watch_metrics_enabled: Option<bool>,
    /// <p>The configuration for the workgroup, which includes the location in Amazon S3 where query results are stored and the encryption option, if any, used for query results.</p>
    #[serde(rename = "ResultConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
}

/// <p>The configuration information that will be updated for this workgroup, which includes the location in Amazon S3 where query results are stored, the encryption option, if any, used for query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup, whether the workgroup settings override the client-side settings, and the data usage limit for the amount of bytes scanned per query, if it is specified.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WorkGroupConfigurationUpdates {
    /// <p>The upper limit (cutoff) for the amount of bytes a single query in a workgroup is allowed to scan.</p>
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned_cutoff_per_query: Option<i64>,
    /// <p>If set to "true", the settings for the workgroup override client-side settings. If set to "false" client-side settings are used. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_work_group_configuration: Option<bool>,
    /// <p>Indicates whether this workgroup enables publishing metrics to Amazon CloudWatch.</p>
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_cloud_watch_metrics_enabled: Option<bool>,
    /// <p>Indicates that the data usage control limit per query is removed. <a>WorkGroupConfiguration$BytesScannedCutoffPerQuery</a> </p>
    #[serde(rename = "RemoveBytesScannedCutoffPerQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_bytes_scanned_cutoff_per_query: Option<bool>,
    /// <p>The result configuration information about the queries in this workgroup that will be updated. Includes the updated results location and an updated option for encrypting query results.</p>
    #[serde(rename = "ResultConfigurationUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration_updates: Option<ResultConfigurationUpdates>,
}

/// <p>The summary information for the workgroup, which includes its name, state, description, and the date and time it was created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct WorkGroupSummary {
    /// <p>The workgroup creation date and time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The workgroup description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the workgroup.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the workgroup.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Errors returned by BatchGetNamedQuery
#[derive(Debug, PartialEq)]
pub enum BatchGetNamedQueryError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl BatchGetNamedQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetNamedQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchGetNamedQueryError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchGetNamedQueryError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetNamedQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetNamedQueryError {
    fn description(&self) -> &str {
        match *self {
            BatchGetNamedQueryError::InternalServer(ref cause) => cause,
            BatchGetNamedQueryError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetQueryExecution
#[derive(Debug, PartialEq)]
pub enum BatchGetQueryExecutionError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl BatchGetQueryExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetQueryExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchGetQueryExecutionError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchGetQueryExecutionError::InvalidRequest(
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
impl fmt::Display for BatchGetQueryExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetQueryExecutionError {
    fn description(&self) -> &str {
        match *self {
            BatchGetQueryExecutionError::InternalServer(ref cause) => cause,
            BatchGetQueryExecutionError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNamedQuery
#[derive(Debug, PartialEq)]
pub enum CreateNamedQueryError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl CreateNamedQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNamedQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateNamedQueryError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateNamedQueryError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateNamedQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNamedQueryError {
    fn description(&self) -> &str {
        match *self {
            CreateNamedQueryError::InternalServer(ref cause) => cause,
            CreateNamedQueryError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWorkGroup
#[derive(Debug, PartialEq)]
pub enum CreateWorkGroupError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl CreateWorkGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWorkGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateWorkGroupError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateWorkGroupError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateWorkGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWorkGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateWorkGroupError::InternalServer(ref cause) => cause,
            CreateWorkGroupError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNamedQuery
#[derive(Debug, PartialEq)]
pub enum DeleteNamedQueryError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl DeleteNamedQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNamedQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteNamedQueryError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteNamedQueryError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteNamedQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNamedQueryError {
    fn description(&self) -> &str {
        match *self {
            DeleteNamedQueryError::InternalServer(ref cause) => cause,
            DeleteNamedQueryError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWorkGroup
#[derive(Debug, PartialEq)]
pub enum DeleteWorkGroupError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl DeleteWorkGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWorkGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteWorkGroupError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteWorkGroupError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteWorkGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWorkGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteWorkGroupError::InternalServer(ref cause) => cause,
            DeleteWorkGroupError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetNamedQuery
#[derive(Debug, PartialEq)]
pub enum GetNamedQueryError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl GetNamedQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNamedQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetNamedQueryError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetNamedQueryError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetNamedQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetNamedQueryError {
    fn description(&self) -> &str {
        match *self {
            GetNamedQueryError::InternalServer(ref cause) => cause,
            GetNamedQueryError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQueryExecution
#[derive(Debug, PartialEq)]
pub enum GetQueryExecutionError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl GetQueryExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueryExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetQueryExecutionError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetQueryExecutionError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetQueryExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueryExecutionError {
    fn description(&self) -> &str {
        match *self {
            GetQueryExecutionError::InternalServer(ref cause) => cause,
            GetQueryExecutionError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetQueryResults
#[derive(Debug, PartialEq)]
pub enum GetQueryResultsError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl GetQueryResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueryResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetQueryResultsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetQueryResultsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetQueryResultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueryResultsError {
    fn description(&self) -> &str {
        match *self {
            GetQueryResultsError::InternalServer(ref cause) => cause,
            GetQueryResultsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWorkGroup
#[derive(Debug, PartialEq)]
pub enum GetWorkGroupError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl GetWorkGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWorkGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetWorkGroupError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetWorkGroupError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetWorkGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWorkGroupError {
    fn description(&self) -> &str {
        match *self {
            GetWorkGroupError::InternalServer(ref cause) => cause,
            GetWorkGroupError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListNamedQueries
#[derive(Debug, PartialEq)]
pub enum ListNamedQueriesError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl ListNamedQueriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNamedQueriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListNamedQueriesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListNamedQueriesError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListNamedQueriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNamedQueriesError {
    fn description(&self) -> &str {
        match *self {
            ListNamedQueriesError::InternalServer(ref cause) => cause,
            ListNamedQueriesError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListQueryExecutions
#[derive(Debug, PartialEq)]
pub enum ListQueryExecutionsError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl ListQueryExecutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueryExecutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListQueryExecutionsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListQueryExecutionsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListQueryExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListQueryExecutionsError {
    fn description(&self) -> &str {
        match *self {
            ListQueryExecutionsError::InternalServer(ref cause) => cause,
            ListQueryExecutionsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWorkGroups
#[derive(Debug, PartialEq)]
pub enum ListWorkGroupsError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl ListWorkGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWorkGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListWorkGroupsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListWorkGroupsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListWorkGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWorkGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListWorkGroupsError::InternalServer(ref cause) => cause,
            ListWorkGroupsError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by StartQueryExecution
#[derive(Debug, PartialEq)]
pub enum StartQueryExecutionError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>Indicates that the request was throttled.</p>
    TooManyRequests(String),
}

impl StartQueryExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartQueryExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartQueryExecutionError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartQueryExecutionError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartQueryExecutionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartQueryExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartQueryExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartQueryExecutionError::InternalServer(ref cause) => cause,
            StartQueryExecutionError::InvalidRequest(ref cause) => cause,
            StartQueryExecutionError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by StopQueryExecution
#[derive(Debug, PartialEq)]
pub enum StopQueryExecutionError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl StopQueryExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopQueryExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopQueryExecutionError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopQueryExecutionError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopQueryExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopQueryExecutionError {
    fn description(&self) -> &str {
        match *self {
            StopQueryExecutionError::InternalServer(ref cause) => cause,
            StopQueryExecutionError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateWorkGroup
#[derive(Debug, PartialEq)]
pub enum UpdateWorkGroupError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl UpdateWorkGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWorkGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateWorkGroupError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateWorkGroupError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateWorkGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateWorkGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateWorkGroupError::InternalServer(ref cause) => cause,
            UpdateWorkGroupError::InvalidRequest(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Athena API. Amazon Athena clients implement this trait.
pub trait Athena {
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Requires you to have access to the workgroup in which the queries were saved. Use <a>ListNamedQueriesInput</a> to get the list of named query IDs in the specified workgroup. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries differ from executed queries. Use <a>BatchGetQueryExecutionInput</a> to get details about each unique query execution, and <a>ListQueryExecutionsInput</a> to get a list of query execution IDs.</p>
    fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> RusotoFuture<BatchGetNamedQueryOutput, BatchGetNamedQueryError>;

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. Requires you to have access to the workgroup in which the queries ran. To get a list of query execution IDs, use <a>ListQueryExecutionsInput$WorkGroup</a>. Query executions differ from named (saved) queries. Use <a>BatchGetNamedQueryInput</a> to get details about named queries.</p>
    fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> RusotoFuture<BatchGetQueryExecutionOutput, BatchGetQueryExecutionError>;

    /// <p>Creates a named query in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> RusotoFuture<CreateNamedQueryOutput, CreateNamedQueryError>;

    /// <p>Creates a workgroup with the specified name.</p>
    fn create_work_group(
        &self,
        input: CreateWorkGroupInput,
    ) -> RusotoFuture<CreateWorkGroupOutput, CreateWorkGroupError>;

    /// <p>Deletes the named query if you have access to the workgroup in which the query was saved.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> RusotoFuture<DeleteNamedQueryOutput, DeleteNamedQueryError>;

    /// <p>Deletes the workgroup with the specified name. The primary workgroup cannot be deleted.</p>
    fn delete_work_group(
        &self,
        input: DeleteWorkGroupInput,
    ) -> RusotoFuture<DeleteWorkGroupOutput, DeleteWorkGroupError>;

    /// <p>Returns information about a single query. Requires that you have access to the workgroup in which the query was saved.</p>
    fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> RusotoFuture<GetNamedQueryOutput, GetNamedQueryError>;

    /// <p>Returns information about a single execution of a query if you have access to the workgroup in which the query ran. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> RusotoFuture<GetQueryExecutionOutput, GetQueryExecutionError>;

    /// <p>Returns the results of a single query execution specified by <code>QueryExecutionId</code> if you have access to the workgroup in which the query ran. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p>
    fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> RusotoFuture<GetQueryResultsOutput, GetQueryResultsError>;

    /// <p>Returns information about the workgroup with the speficied name.</p>
    fn get_work_group(
        &self,
        input: GetWorkGroupInput,
    ) -> RusotoFuture<GetWorkGroupOutput, GetWorkGroupError>;

    /// <p>Provides a list of available query IDs only for queries saved in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> RusotoFuture<ListNamedQueriesOutput, ListNamedQueriesError>;

    /// <p>Provides a list of available query execution IDs for the queries in the specified workgroup. Requires you to have access to the workgroup in which the queries ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> RusotoFuture<ListQueryExecutionsOutput, ListQueryExecutionsError>;

    /// <p>Lists available workgroups for the account.</p>
    fn list_work_groups(
        &self,
        input: ListWorkGroupsInput,
    ) -> RusotoFuture<ListWorkGroupsOutput, ListWorkGroupsError>;

    /// <p>Runs the SQL query statements contained in the <code>Query</code>. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> RusotoFuture<StartQueryExecutionOutput, StartQueryExecutionError>;

    /// <p>Stops a query execution. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> RusotoFuture<StopQueryExecutionOutput, StopQueryExecutionError>;

    /// <p>Updates the workgroup with the specified name. The workgroup's name cannot be changed.</p>
    fn update_work_group(
        &self,
        input: UpdateWorkGroupInput,
    ) -> RusotoFuture<UpdateWorkGroupOutput, UpdateWorkGroupError>;
}
/// A client for the Amazon Athena API.
#[derive(Clone)]
pub struct AthenaClient {
    client: Client,
    region: region::Region,
}

impl AthenaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AthenaClient {
        AthenaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AthenaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AthenaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Athena for AthenaClient {
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Requires you to have access to the workgroup in which the queries were saved. Use <a>ListNamedQueriesInput</a> to get the list of named query IDs in the specified workgroup. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries differ from executed queries. Use <a>BatchGetQueryExecutionInput</a> to get details about each unique query execution, and <a>ListQueryExecutionsInput</a> to get a list of query execution IDs.</p>
    fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> RusotoFuture<BatchGetNamedQueryOutput, BatchGetNamedQueryError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.BatchGetNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetNamedQueryOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchGetNamedQueryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. Requires you to have access to the workgroup in which the queries ran. To get a list of query execution IDs, use <a>ListQueryExecutionsInput$WorkGroup</a>. Query executions differ from named (saved) queries. Use <a>BatchGetNamedQueryInput</a> to get details about named queries.</p>
    fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> RusotoFuture<BatchGetQueryExecutionOutput, BatchGetQueryExecutionError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.BatchGetQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetQueryExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetQueryExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a named query in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> RusotoFuture<CreateNamedQueryOutput, CreateNamedQueryError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.CreateNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateNamedQueryOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateNamedQueryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a workgroup with the specified name.</p>
    fn create_work_group(
        &self,
        input: CreateWorkGroupInput,
    ) -> RusotoFuture<CreateWorkGroupOutput, CreateWorkGroupError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.CreateWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateWorkGroupOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateWorkGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the named query if you have access to the workgroup in which the query was saved.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> RusotoFuture<DeleteNamedQueryOutput, DeleteNamedQueryError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.DeleteNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteNamedQueryOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteNamedQueryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the workgroup with the specified name. The primary workgroup cannot be deleted.</p>
    fn delete_work_group(
        &self,
        input: DeleteWorkGroupInput,
    ) -> RusotoFuture<DeleteWorkGroupOutput, DeleteWorkGroupError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.DeleteWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteWorkGroupOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteWorkGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a single query. Requires that you have access to the workgroup in which the query was saved.</p>
    fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> RusotoFuture<GetNamedQueryOutput, GetNamedQueryError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.GetNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetNamedQueryOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetNamedQueryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a single execution of a query if you have access to the workgroup in which the query ran. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> RusotoFuture<GetQueryExecutionOutput, GetQueryExecutionError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.GetQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetQueryExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetQueryExecutionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the results of a single query execution specified by <code>QueryExecutionId</code> if you have access to the workgroup in which the query ran. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p>
    fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> RusotoFuture<GetQueryResultsOutput, GetQueryResultsError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.GetQueryResults");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetQueryResultsOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetQueryResultsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the workgroup with the speficied name.</p>
    fn get_work_group(
        &self,
        input: GetWorkGroupInput,
    ) -> RusotoFuture<GetWorkGroupOutput, GetWorkGroupError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.GetWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetWorkGroupOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetWorkGroupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides a list of available query IDs only for queries saved in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> RusotoFuture<ListNamedQueriesOutput, ListNamedQueriesError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.ListNamedQueries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListNamedQueriesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListNamedQueriesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides a list of available query execution IDs for the queries in the specified workgroup. Requires you to have access to the workgroup in which the queries ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> RusotoFuture<ListQueryExecutionsOutput, ListQueryExecutionsError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.ListQueryExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListQueryExecutionsOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListQueryExecutionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists available workgroups for the account.</p>
    fn list_work_groups(
        &self,
        input: ListWorkGroupsInput,
    ) -> RusotoFuture<ListWorkGroupsOutput, ListWorkGroupsError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.ListWorkGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListWorkGroupsOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListWorkGroupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Runs the SQL query statements contained in the <code>Query</code>. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> RusotoFuture<StartQueryExecutionOutput, StartQueryExecutionError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.StartQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartQueryExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartQueryExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a query execution. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> RusotoFuture<StopQueryExecutionOutput, StopQueryExecutionError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.StopQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopQueryExecutionOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopQueryExecutionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the workgroup with the specified name. The workgroup's name cannot be changed.</p>
    fn update_work_group(
        &self,
        input: UpdateWorkGroupInput,
    ) -> RusotoFuture<UpdateWorkGroupOutput, UpdateWorkGroupError> {
        let mut request = SignedRequest::new_with_content_type(
            "POST",
            "athena",
            &self.region,
            "/",
            "application/x-amz-json-1.1",
        );

        request.add_header("x-amz-target", "AmazonAthena.UpdateWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateWorkGroupOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateWorkGroupError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
