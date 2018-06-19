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
pub struct BatchGetNamedQueryInput {
    /// <p>An array of query IDs.</p>
    #[serde(rename = "NamedQueryIds")]
    pub named_query_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>A brief explanation of the query.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The plain language name for the query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The text of the query itself. In other words, all query statements.</p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNamedQueryOutput {
    /// <p>The unique ID of the query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

/// <p>A piece of data (a field in the table).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DeleteNamedQueryOutput {}

/// <p>If query results are encrypted in Amazon S3, indicates the Amazon S3 encryption option used.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// <p>Indicates whether Amazon S3 server-side encryption with Amazon S3-managed keys (<code>SSE-S3</code>), server-side encryption with KMS-managed keys (<code>SSE-KMS</code>), or client-side encryption with KMS-managed keys (CSE-KMS) is used.</p>
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
pub struct GetQueryResultsOutput {
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The results of the query execution.</p>
    #[serde(rename = "ResultSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set: Option<ResultSet>,
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>A query, where <code>QueryString</code> is the SQL query statements that comprise the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NamedQuery {
    /// <p>The database to which the query belongs.</p>
    #[serde(rename = "Database")]
    pub database: String,
    /// <p>A brief description of the query.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The plain-language name of the query.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The unique identifier of the query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
    /// <p>The SQL query statements that comprise the query.</p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
}

/// <p>Information about a single instance of a query execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>The location in Amazon S3 where query results were stored and the encryption option, if any, used for query results.</p>
    #[serde(rename = "ResultConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
    /// <p>The amount of data scanned during the query execution and the amount of time that it took to execute.</p>
    #[serde(rename = "Statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<QueryExecutionStatistics>,
    /// <p>The completion date, current state, submission time, and state change reason (if applicable) for the query execution.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryExecutionStatus>,
}

/// <p>The database in which the query execution occurs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryExecutionContext {
    /// <p>The name of the database.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

/// <p>The amount of data scanned during the query execution and the amount of time that it took to execute.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct QueryExecutionStatus {
    /// <p>The date and time that the query completed.</p>
    #[serde(rename = "CompletionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date_time: Option<f64>,
    /// <p>The state of query execution. <code>SUBMITTED</code> indicates that the query is queued for execution. <code>RUNNING</code> indicates that the query is scanning data and returning results. <code>SUCCEEDED</code> indicates that the query completed without error. <code>FAILED</code> indicates that the query experienced an error and did not complete processing. <code>CANCELLED</code> indicates that user input interrupted query execution.</p>
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

/// <p>The location in Amazon S3 where query results are stored and the encryption option, if any, used for query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultConfiguration {
    /// <p>If query results are encrypted in S3, indicates the S3 encryption option used (for example, <code>SSE-KMS</code> or <code>CSE-KMS</code> and key information.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The location in S3 where query results are stored.</p>
    #[serde(rename = "OutputLocation")]
    pub output_location: String,
}

/// <p>The metadata and rows that comprise a query result set. The metadata describes the column structure and data types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// <p>The metadata that describes the column structure and data types of a table of query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResultSetMetadata {
    /// <p>Information about the columns in a query execution result.</p>
    #[serde(rename = "ColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_info: Option<Vec<ColumnInfo>>,
}

/// <p>The rows that comprise a query result table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
    /// <p>Specifies information about where and how to save the results of the query execution.</p>
    #[serde(rename = "ResultConfiguration")]
    pub result_configuration: ResultConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct StopQueryExecutionOutput {}

/// <p>Information about a named query ID that could not be processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

/// Errors returned by BatchGetNamedQuery
#[derive(Debug, PartialEq)]
pub enum BatchGetNamedQueryError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
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

impl BatchGetNamedQueryError {
    pub fn from_body(body: &str) -> BatchGetNamedQueryError {
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
                        BatchGetNamedQueryError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        BatchGetNamedQueryError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetNamedQueryError::Validation(error_message.to_string())
                    }
                    _ => BatchGetNamedQueryError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetNamedQueryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetNamedQueryError {
    fn from(err: serde_json::error::Error) -> BatchGetNamedQueryError {
        BatchGetNamedQueryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetNamedQueryError {
    fn from(err: CredentialsError) -> BatchGetNamedQueryError {
        BatchGetNamedQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetNamedQueryError {
    fn from(err: HttpDispatchError) -> BatchGetNamedQueryError {
        BatchGetNamedQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetNamedQueryError {
    fn from(err: io::Error) -> BatchGetNamedQueryError {
        BatchGetNamedQueryError::HttpDispatch(HttpDispatchError::from(err))
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
            BatchGetNamedQueryError::Validation(ref cause) => cause,
            BatchGetNamedQueryError::Credentials(ref err) => err.description(),
            BatchGetNamedQueryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetNamedQueryError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetQueryExecutionError {
    pub fn from_body(body: &str) -> BatchGetQueryExecutionError {
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
                        BatchGetQueryExecutionError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        BatchGetQueryExecutionError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetQueryExecutionError::Validation(error_message.to_string())
                    }
                    _ => BatchGetQueryExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetQueryExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetQueryExecutionError {
    fn from(err: serde_json::error::Error) -> BatchGetQueryExecutionError {
        BatchGetQueryExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetQueryExecutionError {
    fn from(err: CredentialsError) -> BatchGetQueryExecutionError {
        BatchGetQueryExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetQueryExecutionError {
    fn from(err: HttpDispatchError) -> BatchGetQueryExecutionError {
        BatchGetQueryExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetQueryExecutionError {
    fn from(err: io::Error) -> BatchGetQueryExecutionError {
        BatchGetQueryExecutionError::HttpDispatch(HttpDispatchError::from(err))
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
            BatchGetQueryExecutionError::Validation(ref cause) => cause,
            BatchGetQueryExecutionError::Credentials(ref err) => err.description(),
            BatchGetQueryExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchGetQueryExecutionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNamedQueryError {
    pub fn from_body(body: &str) -> CreateNamedQueryError {
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
                        CreateNamedQueryError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateNamedQueryError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateNamedQueryError::Validation(error_message.to_string())
                    }
                    _ => CreateNamedQueryError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNamedQueryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNamedQueryError {
    fn from(err: serde_json::error::Error) -> CreateNamedQueryError {
        CreateNamedQueryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNamedQueryError {
    fn from(err: CredentialsError) -> CreateNamedQueryError {
        CreateNamedQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNamedQueryError {
    fn from(err: HttpDispatchError) -> CreateNamedQueryError {
        CreateNamedQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNamedQueryError {
    fn from(err: io::Error) -> CreateNamedQueryError {
        CreateNamedQueryError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateNamedQueryError::Validation(ref cause) => cause,
            CreateNamedQueryError::Credentials(ref err) => err.description(),
            CreateNamedQueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateNamedQueryError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNamedQueryError {
    pub fn from_body(body: &str) -> DeleteNamedQueryError {
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
                        DeleteNamedQueryError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteNamedQueryError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteNamedQueryError::Validation(error_message.to_string())
                    }
                    _ => DeleteNamedQueryError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNamedQueryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNamedQueryError {
    fn from(err: serde_json::error::Error) -> DeleteNamedQueryError {
        DeleteNamedQueryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNamedQueryError {
    fn from(err: CredentialsError) -> DeleteNamedQueryError {
        DeleteNamedQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNamedQueryError {
    fn from(err: HttpDispatchError) -> DeleteNamedQueryError {
        DeleteNamedQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNamedQueryError {
    fn from(err: io::Error) -> DeleteNamedQueryError {
        DeleteNamedQueryError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteNamedQueryError::Validation(ref cause) => cause,
            DeleteNamedQueryError::Credentials(ref err) => err.description(),
            DeleteNamedQueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteNamedQueryError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetNamedQueryError {
    pub fn from_body(body: &str) -> GetNamedQueryError {
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
                        GetNamedQueryError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetNamedQueryError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetNamedQueryError::Validation(error_message.to_string())
                    }
                    _ => GetNamedQueryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetNamedQueryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetNamedQueryError {
    fn from(err: serde_json::error::Error) -> GetNamedQueryError {
        GetNamedQueryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetNamedQueryError {
    fn from(err: CredentialsError) -> GetNamedQueryError {
        GetNamedQueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetNamedQueryError {
    fn from(err: HttpDispatchError) -> GetNamedQueryError {
        GetNamedQueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetNamedQueryError {
    fn from(err: io::Error) -> GetNamedQueryError {
        GetNamedQueryError::HttpDispatch(HttpDispatchError::from(err))
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
            GetNamedQueryError::Validation(ref cause) => cause,
            GetNamedQueryError::Credentials(ref err) => err.description(),
            GetNamedQueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetNamedQueryError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQueryExecutionError {
    pub fn from_body(body: &str) -> GetQueryExecutionError {
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
                        GetQueryExecutionError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetQueryExecutionError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetQueryExecutionError::Validation(error_message.to_string())
                    }
                    _ => GetQueryExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetQueryExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetQueryExecutionError {
    fn from(err: serde_json::error::Error) -> GetQueryExecutionError {
        GetQueryExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetQueryExecutionError {
    fn from(err: CredentialsError) -> GetQueryExecutionError {
        GetQueryExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueryExecutionError {
    fn from(err: HttpDispatchError) -> GetQueryExecutionError {
        GetQueryExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueryExecutionError {
    fn from(err: io::Error) -> GetQueryExecutionError {
        GetQueryExecutionError::HttpDispatch(HttpDispatchError::from(err))
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
            GetQueryExecutionError::Validation(ref cause) => cause,
            GetQueryExecutionError::Credentials(ref err) => err.description(),
            GetQueryExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetQueryExecutionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetQueryResultsError {
    pub fn from_body(body: &str) -> GetQueryResultsError {
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
                        GetQueryResultsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetQueryResultsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetQueryResultsError::Validation(error_message.to_string())
                    }
                    _ => GetQueryResultsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetQueryResultsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetQueryResultsError {
    fn from(err: serde_json::error::Error) -> GetQueryResultsError {
        GetQueryResultsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetQueryResultsError {
    fn from(err: CredentialsError) -> GetQueryResultsError {
        GetQueryResultsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueryResultsError {
    fn from(err: HttpDispatchError) -> GetQueryResultsError {
        GetQueryResultsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueryResultsError {
    fn from(err: io::Error) -> GetQueryResultsError {
        GetQueryResultsError::HttpDispatch(HttpDispatchError::from(err))
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
            GetQueryResultsError::Validation(ref cause) => cause,
            GetQueryResultsError::Credentials(ref err) => err.description(),
            GetQueryResultsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetQueryResultsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListNamedQueriesError {
    pub fn from_body(body: &str) -> ListNamedQueriesError {
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
                        ListNamedQueriesError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListNamedQueriesError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListNamedQueriesError::Validation(error_message.to_string())
                    }
                    _ => ListNamedQueriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListNamedQueriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListNamedQueriesError {
    fn from(err: serde_json::error::Error) -> ListNamedQueriesError {
        ListNamedQueriesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListNamedQueriesError {
    fn from(err: CredentialsError) -> ListNamedQueriesError {
        ListNamedQueriesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListNamedQueriesError {
    fn from(err: HttpDispatchError) -> ListNamedQueriesError {
        ListNamedQueriesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListNamedQueriesError {
    fn from(err: io::Error) -> ListNamedQueriesError {
        ListNamedQueriesError::HttpDispatch(HttpDispatchError::from(err))
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
            ListNamedQueriesError::Validation(ref cause) => cause,
            ListNamedQueriesError::Credentials(ref err) => err.description(),
            ListNamedQueriesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListNamedQueriesError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListQueryExecutionsError {
    pub fn from_body(body: &str) -> ListQueryExecutionsError {
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
                        ListQueryExecutionsError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListQueryExecutionsError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListQueryExecutionsError::Validation(error_message.to_string())
                    }
                    _ => ListQueryExecutionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListQueryExecutionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListQueryExecutionsError {
    fn from(err: serde_json::error::Error) -> ListQueryExecutionsError {
        ListQueryExecutionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListQueryExecutionsError {
    fn from(err: CredentialsError) -> ListQueryExecutionsError {
        ListQueryExecutionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListQueryExecutionsError {
    fn from(err: HttpDispatchError) -> ListQueryExecutionsError {
        ListQueryExecutionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListQueryExecutionsError {
    fn from(err: io::Error) -> ListQueryExecutionsError {
        ListQueryExecutionsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListQueryExecutionsError::Validation(ref cause) => cause,
            ListQueryExecutionsError::Credentials(ref err) => err.description(),
            ListQueryExecutionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListQueryExecutionsError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartQueryExecutionError {
    pub fn from_body(body: &str) -> StartQueryExecutionError {
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
                        StartQueryExecutionError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StartQueryExecutionError::InvalidRequest(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        StartQueryExecutionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartQueryExecutionError::Validation(error_message.to_string())
                    }
                    _ => StartQueryExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartQueryExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartQueryExecutionError {
    fn from(err: serde_json::error::Error) -> StartQueryExecutionError {
        StartQueryExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartQueryExecutionError {
    fn from(err: CredentialsError) -> StartQueryExecutionError {
        StartQueryExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartQueryExecutionError {
    fn from(err: HttpDispatchError) -> StartQueryExecutionError {
        StartQueryExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartQueryExecutionError {
    fn from(err: io::Error) -> StartQueryExecutionError {
        StartQueryExecutionError::HttpDispatch(HttpDispatchError::from(err))
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
            StartQueryExecutionError::Validation(ref cause) => cause,
            StartQueryExecutionError::Credentials(ref err) => err.description(),
            StartQueryExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartQueryExecutionError::Unknown(ref cause) => cause,
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
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopQueryExecutionError {
    pub fn from_body(body: &str) -> StopQueryExecutionError {
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
                        StopQueryExecutionError::InternalServer(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StopQueryExecutionError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopQueryExecutionError::Validation(error_message.to_string())
                    }
                    _ => StopQueryExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopQueryExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopQueryExecutionError {
    fn from(err: serde_json::error::Error) -> StopQueryExecutionError {
        StopQueryExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopQueryExecutionError {
    fn from(err: CredentialsError) -> StopQueryExecutionError {
        StopQueryExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopQueryExecutionError {
    fn from(err: HttpDispatchError) -> StopQueryExecutionError {
        StopQueryExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopQueryExecutionError {
    fn from(err: io::Error) -> StopQueryExecutionError {
        StopQueryExecutionError::HttpDispatch(HttpDispatchError::from(err))
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
            StopQueryExecutionError::Validation(ref cause) => cause,
            StopQueryExecutionError::Credentials(ref err) => err.description(),
            StopQueryExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopQueryExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Athena API. Amazon Athena clients implement this trait.
pub trait Athena {
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Use <a>ListNamedQueries</a> to get the list of named query IDs. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries are different from executed queries. Use <a>BatchGetQueryExecution</a> to get details about each unique query execution, and <a>ListQueryExecutions</a> to get a list of query execution IDs.</p>
    fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> RusotoFuture<BatchGetNamedQueryOutput, BatchGetNamedQueryError>;

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. To get a list of query execution IDs, use <a>ListQueryExecutions</a>. Query executions are different from named (saved) queries. Use <a>BatchGetNamedQuery</a> to get details about named queries.</p>
    fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> RusotoFuture<BatchGetQueryExecutionOutput, BatchGetQueryExecutionError>;

    /// <p>Creates a named query.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> RusotoFuture<CreateNamedQueryOutput, CreateNamedQueryError>;

    /// <p>Deletes a named query.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> RusotoFuture<DeleteNamedQueryOutput, DeleteNamedQueryError>;

    /// <p>Returns information about a single query.</p>
    fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> RusotoFuture<GetNamedQueryOutput, GetNamedQueryError>;

    /// <p>Returns information about a single execution of a query. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> RusotoFuture<GetQueryExecutionOutput, GetQueryExecutionError>;

    /// <p>Returns the results of a single query execution specified by <code>QueryExecutionId</code>. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p>
    fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> RusotoFuture<GetQueryResultsOutput, GetQueryResultsError>;

    /// <p>Provides a list of all available query IDs.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> RusotoFuture<ListNamedQueriesOutput, ListNamedQueriesError>;

    /// <p>Provides a list of all available query execution IDs.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> RusotoFuture<ListQueryExecutionsOutput, ListQueryExecutionsError>;

    /// <p>Runs (executes) the SQL query statements contained in the <code>Query</code> string.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> RusotoFuture<StartQueryExecutionOutput, StartQueryExecutionError>;

    /// <p>Stops a query execution.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> RusotoFuture<StopQueryExecutionOutput, StopQueryExecutionError>;
}
/// A client for the Amazon Athena API.
pub struct AthenaClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl AthenaClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> AthenaClient {
        AthenaClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> AthenaClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AthenaClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Athena for AthenaClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Use <a>ListNamedQueries</a> to get the list of named query IDs. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries are different from executed queries. Use <a>BatchGetQueryExecution</a> to get details about each unique query execution, and <a>ListQueryExecutions</a> to get a list of query execution IDs.</p>
    fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> RusotoFuture<BatchGetNamedQueryOutput, BatchGetNamedQueryError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.BatchGetNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetNamedQueryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetNamedQueryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. To get a list of query execution IDs, use <a>ListQueryExecutions</a>. Query executions are different from named (saved) queries. Use <a>BatchGetNamedQuery</a> to get details about named queries.</p>
    fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> RusotoFuture<BatchGetQueryExecutionOutput, BatchGetQueryExecutionError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.BatchGetQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetQueryExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetQueryExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a named query.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> RusotoFuture<CreateNamedQueryOutput, CreateNamedQueryError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.CreateNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateNamedQueryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateNamedQueryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a named query.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> RusotoFuture<DeleteNamedQueryOutput, DeleteNamedQueryError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.DeleteNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteNamedQueryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNamedQueryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a single query.</p>
    fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> RusotoFuture<GetNamedQueryOutput, GetNamedQueryError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetNamedQueryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetNamedQueryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a single execution of a query. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> RusotoFuture<GetQueryExecutionOutput, GetQueryExecutionError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetQueryExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQueryExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the results of a single query execution specified by <code>QueryExecutionId</code>. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p>
    fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> RusotoFuture<GetQueryResultsOutput, GetQueryResultsError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetQueryResults");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetQueryResultsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetQueryResultsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides a list of all available query IDs.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> RusotoFuture<ListNamedQueriesOutput, ListNamedQueriesError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListNamedQueries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListNamedQueriesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListNamedQueriesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides a list of all available query execution IDs.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> RusotoFuture<ListQueryExecutionsOutput, ListQueryExecutionsError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListQueryExecutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListQueryExecutionsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListQueryExecutionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Runs (executes) the SQL query statements contained in the <code>Query</code> string.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> RusotoFuture<StartQueryExecutionOutput, StartQueryExecutionError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.StartQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartQueryExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartQueryExecutionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops a query execution.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> RusotoFuture<StopQueryExecutionOutput, StopQueryExecutionError> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.StopQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopQueryExecutionOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopQueryExecutionError::from_body(
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
