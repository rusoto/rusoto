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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>The request parameters represent the input of a SQL statement over an array of
/// data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchExecuteStatementRequest {
    /// <p>The name of the database.</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>The parameter set for the batch operation.</p>
    #[serde(rename = "parameterSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_sets: Option<Vec<Vec<SqlParameter>>>,
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The name of the database schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "secretArn")]
    pub secret_arn: String,
    /// <p>The SQL statement to run.</p>
    #[serde(rename = "sql")]
    pub sql: String,
    /// <p>The identifier of a transaction that was started by using the
    /// <code>BeginTransaction</code> operation. Specify the transaction ID of the
    /// transaction that you want to include the SQL statement in.</p>
    ///
    /// <pre><code>    &lt;p&gt;If the SQL statement is not part of a transaction, don&#39;t set this
    /// parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>The response elements represent the output of a SQL statement over an array of
/// data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchExecuteStatementResponse {
    /// <p>The execution results of each batch entry.</p>
    #[serde(rename = "updateResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_results: Option<Vec<UpdateResult>>,
}

/// <p>The request parameters represent the input of a request to start a SQL
/// transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BeginTransactionRequest {
    /// <p>The name of the database.</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The name of the database schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "secretArn")]
    pub secret_arn: String,
}

/// <p>The response elements represent the output of a request to start a SQL
/// transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BeginTransactionResponse {
    /// <p>The transaction ID of the transaction started by the call.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Contains the metadata for a column.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ColumnMetadata {
    /// <p>The type of the column.</p>
    #[serde(rename = "arrayBaseColumnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_base_column_type: Option<i64>,
    /// <p>A value that indicates whether the column increments automatically.</p>
    #[serde(rename = "isAutoIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_increment: Option<bool>,
    /// <p>A value that indicates whether the column is case-sensitive.</p>
    #[serde(rename = "isCaseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_case_sensitive: Option<bool>,
    /// <p>A value that indicates whether the column contains currency values.</p>
    #[serde(rename = "isCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_currency: Option<bool>,
    /// <p>A value that indicates whether an integer column is signed.</p>
    #[serde(rename = "isSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signed: Option<bool>,
    /// <p>The label for the column.</p>
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>The name of the column.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A value that indicates whether the column is nullable.</p>
    #[serde(rename = "nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<i64>,
    /// <p>The precision value of a decimal number column.</p>
    #[serde(rename = "precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    /// <p>The scale value of a decimal number column.</p>
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    /// <p>The name of the schema that owns the table that includes the column.</p>
    #[serde(rename = "schemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The name of the table that includes the column.</p>
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The type of the column.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    /// <p>The database-specific data type of the column.</p>
    #[serde(rename = "typeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// <p>The request parameters represent the input of a commit transaction request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CommitTransactionRequest {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "secretArn")]
    pub secret_arn: String,
    /// <p>The identifier of the transaction to end and commit.</p>
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

/// <p>The response elements represent the output of a commit transaction request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CommitTransactionResponse {
    /// <p>The status of the commit operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

/// <p>The request parameters represent the input of a request to run one or more SQL
/// statements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteSqlRequest {
    /// <p>The Amazon Resource Name (ARN) of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "awsSecretStoreArn")]
    pub aws_secret_store_arn: String,
    /// <p>The name of the database.</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>The ARN of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "dbClusterOrInstanceArn")]
    pub db_cluster_or_instance_arn: String,
    /// <p>The name of the database schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>One or more SQL statements to run on the DB cluster.</p>
    ///
    /// <pre><code>    &lt;p&gt;You can separate SQL statements from each other with a semicolon (;). Any valid SQL
    /// statement is permitted, including data definition, data manipulation, and commit
    /// statements. &lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "sqlStatements")]
    pub sql_statements: String,
}

/// <p>The response elements represent the output of a request to run one or more SQL
/// statements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExecuteSqlResponse {
    /// <p>The results of the SQL statement or statements.</p>
    #[serde(rename = "sqlStatementResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_statement_results: Option<Vec<SqlStatementResult>>,
}

/// <p>The request parameters represent the input of a request to run a SQL statement against
/// a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteStatementRequest {
    /// <p>A value that indicates whether to continue running the statement after
    /// the call times out. By default, the statement stops running when the call
    /// times out.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;For DDL statements, we recommend continuing to run the statement after
    /// the call times out. When a DDL statement terminates before it is finished
    /// running, it can result in errors and possibly corrupted data structures.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    #[serde(rename = "continueAfterTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_after_timeout: Option<bool>,
    /// <p>The name of the database.</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>A value that indicates whether to include metadata in the results.</p>
    #[serde(rename = "includeResultMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_result_metadata: Option<bool>,
    /// <p>The parameters for the SQL statement.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<SqlParameter>>,
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The name of the database schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "secretArn")]
    pub secret_arn: String,
    /// <p>The SQL statement to run.</p>
    #[serde(rename = "sql")]
    pub sql: String,
    /// <p>The identifier of a transaction that was started by using the
    /// <code>BeginTransaction</code> operation. Specify the transaction ID of the
    /// transaction that you want to include the SQL statement in.</p>
    ///
    /// <pre><code>    &lt;p&gt;If the SQL statement is not part of a transaction, don&#39;t set this parameter.&lt;/p&gt;
    /// </code></pre>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>The response elements represent the output of a request to run a SQL statement against
/// a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExecuteStatementResponse {
    /// <p>Metadata for the columns included in the results.</p>
    #[serde(rename = "columnMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
    /// <p>Values for fields generated during the request.</p>
    #[serde(rename = "generatedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_fields: Option<Vec<Field>>,
    /// <p>The number of records updated by the request.</p>
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_records_updated: Option<i64>,
    /// <p>The records returned by the SQL statement.</p>
    #[serde(rename = "records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Vec<Field>>>,
}

/// <p>Contains a value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// <p>A value of BLOB data type.</p>
    #[serde(rename = "blobValue")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_value: Option<bytes::Bytes>,
    /// <p>A value of Boolean data type.</p>
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    /// <p>A value of double data type.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>A NULL value.</p>
    #[serde(rename = "isNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    /// <p>A value of long data type.</p>
    #[serde(rename = "longValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    /// <p>A value of string data type.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>A record returned by a call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Record {
    /// <p>The values returned in the record.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
}

/// <p>The result set returned by a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultFrame {
    /// <p>The records in the result set.</p>
    #[serde(rename = "records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    /// <p>The result-set metadata in the result set.</p>
    #[serde(rename = "resultSetMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_metadata: Option<ResultSetMetadata>,
}

/// <p>The metadata of the result set returned by a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultSetMetadata {
    /// <p>The number of columns in the result set.</p>
    #[serde(rename = "columnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i64>,
    /// <p>The metadata of the columns in the result set.</p>
    #[serde(rename = "columnMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
}

/// <p>The request parameters represent the input of a request to perform a rollback of a
/// transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RollbackTransactionRequest {
    /// <p>The Amazon Resource Name (ARN) of the Aurora Serverless DB cluster.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The name or ARN of the secret that enables access to the DB cluster.</p>
    #[serde(rename = "secretArn")]
    pub secret_arn: String,
    /// <p>The identifier of the transaction to roll back.</p>
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

/// <p>The response elements represent the output of a request to perform a rollback of a
/// transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RollbackTransactionResponse {
    /// <p>The status of the rollback operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

/// <p>A parameter used in a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SqlParameter {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the parameter.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Field>,
}

/// <p>The result of a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SqlStatementResult {
    /// <p>The number of records updated by a SQL statement.</p>
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_records_updated: Option<i64>,
    /// <p>The result set of the SQL statement.</p>
    #[serde(rename = "resultFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_frame: Option<ResultFrame>,
}

/// <p>A structure value returned by a call.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StructValue {
    /// <p>The attributes returned in the record.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Value>>,
}

/// <p>The response elements represent the results of an update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateResult {
    /// <p>Values for fields generated during the request.</p>
    #[serde(rename = "generatedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_fields: Option<Vec<Field>>,
}

/// <p>Contains the value of a column.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Value {
    /// <p>An array of column values.</p>
    #[serde(rename = "arrayValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<Value>>,
    /// <p>A value for a column of big integer data type.</p>
    #[serde(rename = "bigIntValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_int_value: Option<i64>,
    /// <p>A value for a column of BIT data type.</p>
    #[serde(rename = "bitValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_value: Option<bool>,
    /// <p>A value for a column of BLOB data type.</p>
    #[serde(rename = "blobValue")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_value: Option<bytes::Bytes>,
    /// <p>A value for a column of double data type.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>A value for a column of integer data type.</p>
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    /// <p>A NULL value.</p>
    #[serde(rename = "isNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    /// <p>A value for a column of real data type.</p>
    #[serde(rename = "realValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_value: Option<f32>,
    /// <p>A value for a column of string data type.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    /// <p>A value for a column of STRUCT data type.</p>
    #[serde(rename = "structValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_value: Option<StructValue>,
}

/// Errors returned by BatchExecuteStatement
#[derive(Debug, PartialEq)]
pub enum BatchExecuteStatementError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeout(String),
}

impl BatchExecuteStatementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchExecuteStatementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BatchExecuteStatementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchExecuteStatementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BatchExecuteStatementError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(
                        BatchExecuteStatementError::ServiceUnavailableError(err.msg),
                    )
                }
                "StatementTimeoutException" => {
                    return RusotoError::Service(BatchExecuteStatementError::StatementTimeout(
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
impl fmt::Display for BatchExecuteStatementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchExecuteStatementError {
    fn description(&self) -> &str {
        match *self {
            BatchExecuteStatementError::BadRequest(ref cause) => cause,
            BatchExecuteStatementError::Forbidden(ref cause) => cause,
            BatchExecuteStatementError::InternalServerError(ref cause) => cause,
            BatchExecuteStatementError::ServiceUnavailableError(ref cause) => cause,
            BatchExecuteStatementError::StatementTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by BeginTransaction
#[derive(Debug, PartialEq)]
pub enum BeginTransactionError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeout(String),
}

impl BeginTransactionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BeginTransactionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(BeginTransactionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BeginTransactionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BeginTransactionError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(BeginTransactionError::ServiceUnavailableError(
                        err.msg,
                    ))
                }
                "StatementTimeoutException" => {
                    return RusotoError::Service(BeginTransactionError::StatementTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BeginTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BeginTransactionError {
    fn description(&self) -> &str {
        match *self {
            BeginTransactionError::BadRequest(ref cause) => cause,
            BeginTransactionError::Forbidden(ref cause) => cause,
            BeginTransactionError::InternalServerError(ref cause) => cause,
            BeginTransactionError::ServiceUnavailableError(ref cause) => cause,
            BeginTransactionError::StatementTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by CommitTransaction
#[derive(Debug, PartialEq)]
pub enum CommitTransactionError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The <code>resourceArn</code>, <code>secretArn</code>, or <code>transactionId</code> value can't be found.</p>
    NotFound(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
}

impl CommitTransactionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CommitTransactionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CommitTransactionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CommitTransactionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CommitTransactionError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CommitTransactionError::NotFound(err.msg))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(CommitTransactionError::ServiceUnavailableError(
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
impl fmt::Display for CommitTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CommitTransactionError {
    fn description(&self) -> &str {
        match *self {
            CommitTransactionError::BadRequest(ref cause) => cause,
            CommitTransactionError::Forbidden(ref cause) => cause,
            CommitTransactionError::InternalServerError(ref cause) => cause,
            CommitTransactionError::NotFound(ref cause) => cause,
            CommitTransactionError::ServiceUnavailableError(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecuteSql
#[derive(Debug, PartialEq)]
pub enum ExecuteSqlError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
}

impl ExecuteSqlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecuteSqlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ExecuteSqlError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ExecuteSqlError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ExecuteSqlError::InternalServerError(err.msg))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(ExecuteSqlError::ServiceUnavailableError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExecuteSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteSqlError {
    fn description(&self) -> &str {
        match *self {
            ExecuteSqlError::BadRequest(ref cause) => cause,
            ExecuteSqlError::Forbidden(ref cause) => cause,
            ExecuteSqlError::InternalServerError(ref cause) => cause,
            ExecuteSqlError::ServiceUnavailableError(ref cause) => cause,
        }
    }
}
/// Errors returned by ExecuteStatement
#[derive(Debug, PartialEq)]
pub enum ExecuteStatementError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeout(String),
}

impl ExecuteStatementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecuteStatementError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ExecuteStatementError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ExecuteStatementError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ExecuteStatementError::InternalServerError(
                        err.msg,
                    ))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(ExecuteStatementError::ServiceUnavailableError(
                        err.msg,
                    ))
                }
                "StatementTimeoutException" => {
                    return RusotoError::Service(ExecuteStatementError::StatementTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExecuteStatementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteStatementError {
    fn description(&self) -> &str {
        match *self {
            ExecuteStatementError::BadRequest(ref cause) => cause,
            ExecuteStatementError::Forbidden(ref cause) => cause,
            ExecuteStatementError::InternalServerError(ref cause) => cause,
            ExecuteStatementError::ServiceUnavailableError(ref cause) => cause,
            ExecuteStatementError::StatementTimeout(ref cause) => cause,
        }
    }
}
/// Errors returned by RollbackTransaction
#[derive(Debug, PartialEq)]
pub enum RollbackTransactionError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The <code>resourceArn</code>, <code>secretArn</code>, or <code>transactionId</code> value can't be found.</p>
    NotFound(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not
    /// available.</p>
    ServiceUnavailableError(String),
}

impl RollbackTransactionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RollbackTransactionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RollbackTransactionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RollbackTransactionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RollbackTransactionError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RollbackTransactionError::NotFound(err.msg))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(RollbackTransactionError::ServiceUnavailableError(
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
impl fmt::Display for RollbackTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RollbackTransactionError {
    fn description(&self) -> &str {
        match *self {
            RollbackTransactionError::BadRequest(ref cause) => cause,
            RollbackTransactionError::Forbidden(ref cause) => cause,
            RollbackTransactionError::InternalServerError(ref cause) => cause,
            RollbackTransactionError::NotFound(ref cause) => cause,
            RollbackTransactionError::ServiceUnavailableError(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS RDS DataService API. AWS RDS DataService clients implement this trait.
pub trait RdsData {
    /// <p>Runs a batch SQL statement over an array of data.</p>
    ///
    /// <pre><code>    &lt;p&gt;You can run bulk update and insert operations for multiple records using a DML
    /// statement with different parameter sets. Bulk operations can provide a significant
    /// performance improvement over individual insert and update operations.&lt;/p&gt;
    /// &lt;important&gt;
    /// &lt;p&gt;If a call isn&#39;t part of a transaction because it doesn&#39;t include the
    /// &lt;code&gt;transactionID&lt;/code&gt; parameter, changes that result from the call are
    /// committed automatically.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn batch_execute_statement(
        &self,
        input: BatchExecuteStatementRequest,
    ) -> RusotoFuture<BatchExecuteStatementResponse, BatchExecuteStatementError>;

    /// <p>Starts a SQL transaction.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;A transaction can run for a maximum of 24 hours. A transaction is terminated and
    /// rolled back automatically after 24 hours.&lt;/p&gt;
    /// &lt;p&gt;A transaction times out if no calls use its transaction ID in three minutes.
    /// If a transaction times out before it&#39;s committed, it&#39;s rolled back
    /// automatically.&lt;/p&gt;
    /// &lt;p&gt;DDL statements inside a transaction cause an implicit commit. We recommend
    /// that you run each DDL statement in a separate &lt;code&gt;ExecuteStatement&lt;/code&gt; call with
    /// &lt;code&gt;continueAfterTimeout&lt;/code&gt; enabled.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn begin_transaction(
        &self,
        input: BeginTransactionRequest,
    ) -> RusotoFuture<BeginTransactionResponse, BeginTransactionError>;

    /// <p>Ends a SQL transaction started with the <code>BeginTransaction</code> operation and
    /// commits the changes.</p>
    fn commit_transaction(
        &self,
        input: CommitTransactionRequest,
    ) -> RusotoFuture<CommitTransactionResponse, CommitTransactionError>;

    /// <p>Runs one or more SQL statements.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;This operation is deprecated. Use the &lt;code&gt;BatchExecuteStatement&lt;/code&gt; or
    /// &lt;code&gt;ExecuteStatement&lt;/code&gt; operation.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn execute_sql(
        &self,
        input: ExecuteSqlRequest,
    ) -> RusotoFuture<ExecuteSqlResponse, ExecuteSqlError>;

    /// <p>Runs a SQL statement against a database.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;If a call isn&#39;t part of a transaction because it doesn&#39;t include the
    /// &lt;code&gt;transactionID&lt;/code&gt; parameter, changes that result from the call are
    /// committed automatically.&lt;/p&gt;
    /// &lt;/important&gt;
    /// &lt;p&gt;The response size limit is 1 MB or 1,000 records. If the call returns more than 1 MB of response data or over 1,000 records, the call is terminated.&lt;/p&gt;
    /// </code></pre>
    fn execute_statement(
        &self,
        input: ExecuteStatementRequest,
    ) -> RusotoFuture<ExecuteStatementResponse, ExecuteStatementError>;

    /// <p>Performs a rollback of a transaction. Rolling back a transaction cancels its changes.</p>
    fn rollback_transaction(
        &self,
        input: RollbackTransactionRequest,
    ) -> RusotoFuture<RollbackTransactionResponse, RollbackTransactionError>;
}
/// A client for the AWS RDS DataService API.
#[derive(Clone)]
pub struct RdsDataClient {
    client: Client,
    region: region::Region,
}

impl RdsDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RdsDataClient {
        RdsDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RdsDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        RdsDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl RdsData for RdsDataClient {
    /// <p>Runs a batch SQL statement over an array of data.</p>
    ///
    /// <pre><code>    &lt;p&gt;You can run bulk update and insert operations for multiple records using a DML
    /// statement with different parameter sets. Bulk operations can provide a significant
    /// performance improvement over individual insert and update operations.&lt;/p&gt;
    /// &lt;important&gt;
    /// &lt;p&gt;If a call isn&#39;t part of a transaction because it doesn&#39;t include the
    /// &lt;code&gt;transactionID&lt;/code&gt; parameter, changes that result from the call are
    /// committed automatically.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn batch_execute_statement(
        &self,
        input: BatchExecuteStatementRequest,
    ) -> RusotoFuture<BatchExecuteStatementResponse, BatchExecuteStatementError> {
        let request_uri = "/BatchExecute";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| BatchExecuteStatementError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<BatchExecuteStatementResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<BatchExecuteStatementError>())
                            .and_then(|response| {
                                Err(BatchExecuteStatementError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Starts a SQL transaction.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;A transaction can run for a maximum of 24 hours. A transaction is terminated and
    /// rolled back automatically after 24 hours.&lt;/p&gt;
    /// &lt;p&gt;A transaction times out if no calls use its transaction ID in three minutes.
    /// If a transaction times out before it&#39;s committed, it&#39;s rolled back
    /// automatically.&lt;/p&gt;
    /// &lt;p&gt;DDL statements inside a transaction cause an implicit commit. We recommend
    /// that you run each DDL statement in a separate &lt;code&gt;ExecuteStatement&lt;/code&gt; call with
    /// &lt;code&gt;continueAfterTimeout&lt;/code&gt; enabled.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn begin_transaction(
        &self,
        input: BeginTransactionRequest,
    ) -> RusotoFuture<BeginTransactionResponse, BeginTransactionError> {
        let request_uri = "/BeginTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| BeginTransactionError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<BeginTransactionResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<BeginTransactionError>())
                            .and_then(|response| {
                                Err(BeginTransactionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Ends a SQL transaction started with the <code>BeginTransaction</code> operation and
    /// commits the changes.</p>
    fn commit_transaction(
        &self,
        input: CommitTransactionRequest,
    ) -> RusotoFuture<CommitTransactionResponse, CommitTransactionError> {
        let request_uri = "/CommitTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| CommitTransactionError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<CommitTransactionResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<CommitTransactionError>())
                            .and_then(|response| {
                                Err(CommitTransactionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Runs one or more SQL statements.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;This operation is deprecated. Use the &lt;code&gt;BatchExecuteStatement&lt;/code&gt; or
    /// &lt;code&gt;ExecuteStatement&lt;/code&gt; operation.&lt;/p&gt;
    /// &lt;/important&gt;
    /// </code></pre>
    fn execute_sql(
        &self,
        input: ExecuteSqlRequest,
    ) -> RusotoFuture<ExecuteSqlResponse, ExecuteSqlError> {
        let request_uri = "/ExecuteSql";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| ExecuteSqlError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<ExecuteSqlResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<ExecuteSqlError>())
                            .and_then(|response| Err(ExecuteSqlError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Runs a SQL statement against a database.</p>
    ///
    /// <pre><code>    &lt;important&gt;
    /// &lt;p&gt;If a call isn&#39;t part of a transaction because it doesn&#39;t include the
    /// &lt;code&gt;transactionID&lt;/code&gt; parameter, changes that result from the call are
    /// committed automatically.&lt;/p&gt;
    /// &lt;/important&gt;
    /// &lt;p&gt;The response size limit is 1 MB or 1,000 records. If the call returns more than 1 MB of response data or over 1,000 records, the call is terminated.&lt;/p&gt;
    /// </code></pre>
    fn execute_statement(
        &self,
        input: ExecuteStatementRequest,
    ) -> RusotoFuture<ExecuteStatementResponse, ExecuteStatementError> {
        let request_uri = "/Execute";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| ExecuteStatementError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<ExecuteStatementResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<ExecuteStatementError>())
                            .and_then(|response| {
                                Err(ExecuteStatementError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Performs a rollback of a transaction. Rolling back a transaction cancels its changes.</p>
    fn rollback_transaction(
        &self,
        input: RollbackTransactionRequest,
    ) -> RusotoFuture<RollbackTransactionResponse, RollbackTransactionError> {
        let request_uri = "/RollbackTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                response
                    .buffer()
                    .map_err(|e| RollbackTransactionError::from(e))
                    .map(|try_response| {
                        try_response.map_err(|e| e.into()).and_then(|response| {
                            let result = proto::json::ResponsePayload::new(&response)
                                .deserialize::<RollbackTransactionResponse, _>();

                            result
                        })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| e.into::<RollbackTransactionError>())
                            .and_then(|response| {
                                Err(RollbackTransactionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }
}
