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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Contains an array.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayValue {
    /// <p>An array of arrays.</p>
    #[serde(rename = "arrayValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<ArrayValue>>,
    /// <p>An array of Boolean values.</p>
    #[serde(rename = "booleanValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_values: Option<Vec<bool>>,
    /// <p>An array of integers.</p>
    #[serde(rename = "doubleValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_values: Option<Vec<f64>>,
    /// <p>An array of floating point numbers.</p>
    #[serde(rename = "longValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_values: Option<Vec<i64>>,
    /// <p>An array of strings.</p>
    #[serde(rename = "stringValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
}

/// <p>The request parameters represent the input of a SQL statement over an array of data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchExecuteStatementRequest {
    /// <p>The name of the database.</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>The parameter set for the batch operation.</p> <p>The maximum number of parameters in a parameter set is 1,000.</p>
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
    /// <p>The identifier of a transaction that was started by using the <code>BeginTransaction</code> operation. Specify the transaction ID of the transaction that you want to include the SQL statement in.</p> <p>If the SQL statement is not part of a transaction, don't set this parameter.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>The response elements represent the output of a SQL statement over an array of data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchExecuteStatementResponse {
    /// <p>The execution results of each batch entry.</p>
    #[serde(rename = "updateResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_results: Option<Vec<UpdateResult>>,
}

/// <p>The request parameters represent the input of a request to start a SQL transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>The response elements represent the output of a request to start a SQL transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BeginTransactionResponse {
    /// <p>The transaction ID of the transaction started by the call.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Contains the metadata for a column.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommitTransactionResponse {
    /// <p>The status of the commit operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

/// <p>The request parameters represent the input of a request to run one or more SQL statements.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>One or more SQL statements to run on the DB cluster.</p> <p>You can separate SQL statements from each other with a semicolon (;). Any valid SQL statement is permitted, including data definition, data manipulation, and commit statements. </p>
    #[serde(rename = "sqlStatements")]
    pub sql_statements: String,
}

/// <p>The response elements represent the output of a request to run one or more SQL statements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteSqlResponse {
    /// <p>The results of the SQL statement or statements.</p>
    #[serde(rename = "sqlStatementResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_statement_results: Option<Vec<SqlStatementResult>>,
}

/// <p>The request parameters represent the input of a request to run a SQL statement against a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteStatementRequest {
    /// <p><p>A value that indicates whether to continue running the statement after the call times out. By default, the statement stops running when the call times out.</p> <important> <p>For DDL statements, we recommend continuing to run the statement after the call times out. When a DDL statement terminates before it is finished running, it can result in errors and possibly corrupted data structures.</p> </important></p>
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
    /// <p>Options that control how the result set is returned.</p>
    #[serde(rename = "resultSetOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_options: Option<ResultSetOptions>,
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
    /// <p>The identifier of a transaction that was started by using the <code>BeginTransaction</code> operation. Specify the transaction ID of the transaction that you want to include the SQL statement in.</p> <p>If the SQL statement is not part of a transaction, don't set this parameter.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>The response elements represent the output of a request to run a SQL statement against a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteStatementResponse {
    /// <p>Metadata for the columns included in the results.</p>
    #[serde(rename = "columnMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
    /// <p><p>Values for fields generated during the request.</p> <pre><code> &lt;note&gt; &lt;p&gt;The &lt;code&gt;generatedFields&lt;/code&gt; data isn&#39;t supported by Aurora PostgreSQL. To get the values of generated fields, use the &lt;code&gt;RETURNING&lt;/code&gt; clause. For more information, see &lt;a href=&quot;https://www.postgresql.org/docs/10/dml-returning.html&quot;&gt;Returning Data From Modified Rows&lt;/a&gt; in the PostgreSQL documentation.&lt;/p&gt; &lt;/note&gt; </code></pre></p>
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
    /// <p>An array of values.</p>
    #[serde(rename = "arrayValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_value: Option<ArrayValue>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Record {
    /// <p>The values returned in the record.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
}

/// <p>The result set returned by a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Options that control how the result set is returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResultSetOptions {
    /// <p><p>A value that indicates how a field of <code>DECIMAL</code> type is represented in the response. The value of <code>STRING</code>, the default, specifies that it is converted to a String value. The value of <code>DOUBLE<em>OR</em>LONG</code> specifies that it is converted to a Long value if its scale is 0, or to a Double value otherwise.</p> <important> <p>Conversion to Double or Long can result in roundoff errors due to precision loss. We recommend converting to String, especially when working with currency values.</p> </important></p>
    #[serde(rename = "decimalReturnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_return_type: Option<String>,
}

/// <p>The request parameters represent the input of a request to perform a rollback of a transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>The response elements represent the output of a request to perform a rollback of a transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RollbackTransactionResponse {
    /// <p>The status of the rollback operation.</p>
    #[serde(rename = "transactionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

/// <p>A parameter used in a SQL statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SqlParameter {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>A hint that specifies the correct object type for data type mapping.</p> <p> <b>Values:</b> </p> <ul> <li> <p> <code>DECIMAL</code> - The corresponding <code>String</code> parameter value is sent as an object of <code>DECIMAL</code> type to the database.</p> </li> <li> <p> <code>TIMESTAMP</code> - The corresponding <code>String</code> parameter value is sent as an object of <code>TIMESTAMP</code> type to the database. The accepted format is <code>YYYY-MM-DD HH:MM:SS[.FFF]</code>.</p> </li> <li> <p> <code>TIME</code> - The corresponding <code>String</code> parameter value is sent as an object of <code>TIME</code> type to the database. The accepted format is <code>HH:MM:SS[.FFF]</code>.</p> </li> <li> <p> <code>DATE</code> - The corresponding <code>String</code> parameter value is sent as an object of <code>DATE</code> type to the database. The accepted format is <code>YYYY-MM-DD</code>.</p> </li> </ul></p>
    #[serde(rename = "typeHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hint: Option<String>,
    /// <p>The value of the parameter.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Field>,
}

/// <p><p>The result of a SQL statement.</p> <pre><code> &lt;important&gt; &lt;p&gt;This data type is deprecated.&lt;/p&gt; &lt;/important&gt; </code></pre></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StructValue {
    /// <p>The attributes returned in the record.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Value>>,
}

/// <p>The response elements represent the results of an update.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateResult {
    /// <p>Values for fields generated during the request.</p>
    #[serde(rename = "generatedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_fields: Option<Vec<Field>>,
}

/// <p><p>Contains the value of a column.</p> <pre><code> &lt;important&gt; &lt;p&gt;This data type is deprecated.&lt;/p&gt; &lt;/important&gt; </code></pre></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchExecuteStatementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchExecuteStatementError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchExecuteStatementError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchExecuteStatementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BatchExecuteStatementError::ServiceUnavailableError(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchExecuteStatementError::StatementTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchExecuteStatementError {}
/// Errors returned by BeginTransaction
#[derive(Debug, PartialEq)]
pub enum BeginTransactionError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BeginTransactionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BeginTransactionError::BadRequest(ref cause) => write!(f, "{}", cause),
            BeginTransactionError::Forbidden(ref cause) => write!(f, "{}", cause),
            BeginTransactionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BeginTransactionError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
            BeginTransactionError::StatementTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BeginTransactionError {}
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
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
    ServiceUnavailableError(String),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeout(String),
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
                "StatementTimeoutException" => {
                    return RusotoError::Service(CommitTransactionError::StatementTimeout(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CommitTransactionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommitTransactionError::BadRequest(ref cause) => write!(f, "{}", cause),
            CommitTransactionError::Forbidden(ref cause) => write!(f, "{}", cause),
            CommitTransactionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CommitTransactionError::NotFound(ref cause) => write!(f, "{}", cause),
            CommitTransactionError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
            CommitTransactionError::StatementTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CommitTransactionError {}
/// Errors returned by ExecuteSql
#[derive(Debug, PartialEq)]
pub enum ExecuteSqlError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExecuteSqlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteSqlError::BadRequest(ref cause) => write!(f, "{}", cause),
            ExecuteSqlError::Forbidden(ref cause) => write!(f, "{}", cause),
            ExecuteSqlError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ExecuteSqlError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExecuteSqlError {}
/// Errors returned by ExecuteStatement
#[derive(Debug, PartialEq)]
pub enum ExecuteStatementError {
    /// <p>There is an error in the call or in a SQL statement.</p>
    BadRequest(String),
    /// <p>There are insufficient privileges to make the call.</p>
    Forbidden(String),
    /// <p>An internal error occurred.</p>
    InternalServerError(String),
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ExecuteStatementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteStatementError::BadRequest(ref cause) => write!(f, "{}", cause),
            ExecuteStatementError::Forbidden(ref cause) => write!(f, "{}", cause),
            ExecuteStatementError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ExecuteStatementError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
            ExecuteStatementError::StatementTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExecuteStatementError {}
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
    /// <p>The service specified by the <code>resourceArn</code> parameter is not available.</p>
    ServiceUnavailableError(String),
    /// <p>The execution of the SQL statement timed out.</p>
    StatementTimeout(String),
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
                "StatementTimeoutException" => {
                    return RusotoError::Service(RollbackTransactionError::StatementTimeout(
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
impl fmt::Display for RollbackTransactionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RollbackTransactionError::BadRequest(ref cause) => write!(f, "{}", cause),
            RollbackTransactionError::Forbidden(ref cause) => write!(f, "{}", cause),
            RollbackTransactionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RollbackTransactionError::NotFound(ref cause) => write!(f, "{}", cause),
            RollbackTransactionError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
            RollbackTransactionError::StatementTimeout(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RollbackTransactionError {}
/// Trait representing the capabilities of the AWS RDS DataService API. AWS RDS DataService clients implement this trait.
#[async_trait]
pub trait RdsData {
    /// <p><p>Runs a batch SQL statement over an array of data.</p> <p>You can run bulk update and insert operations for multiple records using a DML statement with different parameter sets. Bulk operations can provide a significant performance improvement over individual insert and update operations.</p> <important> <p>If a call isn&#39;t part of a transaction because it doesn&#39;t include the <code>transactionID</code> parameter, changes that result from the call are committed automatically.</p> </important></p>
    async fn batch_execute_statement(
        &self,
        input: BatchExecuteStatementRequest,
    ) -> Result<BatchExecuteStatementResponse, RusotoError<BatchExecuteStatementError>>;

    /// <p><p>Starts a SQL transaction.</p> <pre><code> &lt;important&gt; &lt;p&gt;A transaction can run for a maximum of 24 hours. A transaction is terminated and rolled back automatically after 24 hours.&lt;/p&gt; &lt;p&gt;A transaction times out if no calls use its transaction ID in three minutes. If a transaction times out before it&#39;s committed, it&#39;s rolled back automatically.&lt;/p&gt; &lt;p&gt;DDL statements inside a transaction cause an implicit commit. We recommend that you run each DDL statement in a separate &lt;code&gt;ExecuteStatement&lt;/code&gt; call with &lt;code&gt;continueAfterTimeout&lt;/code&gt; enabled.&lt;/p&gt; &lt;/important&gt; </code></pre></p>
    async fn begin_transaction(
        &self,
        input: BeginTransactionRequest,
    ) -> Result<BeginTransactionResponse, RusotoError<BeginTransactionError>>;

    /// <p>Ends a SQL transaction started with the <code>BeginTransaction</code> operation and commits the changes.</p>
    async fn commit_transaction(
        &self,
        input: CommitTransactionRequest,
    ) -> Result<CommitTransactionResponse, RusotoError<CommitTransactionError>>;

    /// <p><p>Runs one or more SQL statements.</p> <important> <p>This operation is deprecated. Use the <code>BatchExecuteStatement</code> or <code>ExecuteStatement</code> operation.</p> </important></p>
    async fn execute_sql(
        &self,
        input: ExecuteSqlRequest,
    ) -> Result<ExecuteSqlResponse, RusotoError<ExecuteSqlError>>;

    /// <p>Runs a SQL statement against a database.</p> <important> <p>If a call isn't part of a transaction because it doesn't include the <code>transactionID</code> parameter, changes that result from the call are committed automatically.</p> </important> <p>The response size limit is 1 MB or 1,000 records. If the call returns more than 1 MB of response data or over 1,000 records, the call is terminated.</p>
    async fn execute_statement(
        &self,
        input: ExecuteStatementRequest,
    ) -> Result<ExecuteStatementResponse, RusotoError<ExecuteStatementError>>;

    /// <p>Performs a rollback of a transaction. Rolling back a transaction cancels its changes.</p>
    async fn rollback_transaction(
        &self,
        input: RollbackTransactionRequest,
    ) -> Result<RollbackTransactionResponse, RusotoError<RollbackTransactionError>>;
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

    pub fn new_with_client(client: Client, region: region::Region) -> RdsDataClient {
        RdsDataClient { client, region }
    }
}

#[async_trait]
impl RdsData for RdsDataClient {
    /// <p><p>Runs a batch SQL statement over an array of data.</p> <p>You can run bulk update and insert operations for multiple records using a DML statement with different parameter sets. Bulk operations can provide a significant performance improvement over individual insert and update operations.</p> <important> <p>If a call isn&#39;t part of a transaction because it doesn&#39;t include the <code>transactionID</code> parameter, changes that result from the call are committed automatically.</p> </important></p>
    async fn batch_execute_statement(
        &self,
        input: BatchExecuteStatementRequest,
    ) -> Result<BatchExecuteStatementResponse, RusotoError<BatchExecuteStatementError>> {
        let request_uri = "/BatchExecute";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchExecuteStatementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchExecuteStatementError::from_response(response))
        }
    }

    /// <p><p>Starts a SQL transaction.</p> <pre><code> &lt;important&gt; &lt;p&gt;A transaction can run for a maximum of 24 hours. A transaction is terminated and rolled back automatically after 24 hours.&lt;/p&gt; &lt;p&gt;A transaction times out if no calls use its transaction ID in three minutes. If a transaction times out before it&#39;s committed, it&#39;s rolled back automatically.&lt;/p&gt; &lt;p&gt;DDL statements inside a transaction cause an implicit commit. We recommend that you run each DDL statement in a separate &lt;code&gt;ExecuteStatement&lt;/code&gt; call with &lt;code&gt;continueAfterTimeout&lt;/code&gt; enabled.&lt;/p&gt; &lt;/important&gt; </code></pre></p>
    async fn begin_transaction(
        &self,
        input: BeginTransactionRequest,
    ) -> Result<BeginTransactionResponse, RusotoError<BeginTransactionError>> {
        let request_uri = "/BeginTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BeginTransactionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BeginTransactionError::from_response(response))
        }
    }

    /// <p>Ends a SQL transaction started with the <code>BeginTransaction</code> operation and commits the changes.</p>
    async fn commit_transaction(
        &self,
        input: CommitTransactionRequest,
    ) -> Result<CommitTransactionResponse, RusotoError<CommitTransactionError>> {
        let request_uri = "/CommitTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CommitTransactionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CommitTransactionError::from_response(response))
        }
    }

    /// <p><p>Runs one or more SQL statements.</p> <important> <p>This operation is deprecated. Use the <code>BatchExecuteStatement</code> or <code>ExecuteStatement</code> operation.</p> </important></p>
    async fn execute_sql(
        &self,
        input: ExecuteSqlRequest,
    ) -> Result<ExecuteSqlResponse, RusotoError<ExecuteSqlError>> {
        let request_uri = "/ExecuteSql";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExecuteSqlResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExecuteSqlError::from_response(response))
        }
    }

    /// <p>Runs a SQL statement against a database.</p> <important> <p>If a call isn't part of a transaction because it doesn't include the <code>transactionID</code> parameter, changes that result from the call are committed automatically.</p> </important> <p>The response size limit is 1 MB or 1,000 records. If the call returns more than 1 MB of response data or over 1,000 records, the call is terminated.</p>
    async fn execute_statement(
        &self,
        input: ExecuteStatementRequest,
    ) -> Result<ExecuteStatementResponse, RusotoError<ExecuteStatementError>> {
        let request_uri = "/Execute";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExecuteStatementResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExecuteStatementError::from_response(response))
        }
    }

    /// <p>Performs a rollback of a transaction. Rolling back a transaction cancels its changes.</p>
    async fn rollback_transaction(
        &self,
        input: RollbackTransactionRequest,
    ) -> Result<RollbackTransactionResponse, RusotoError<RollbackTransactionError>> {
        let request_uri = "/RollbackTransaction";

        let mut request = SignedRequest::new("POST", "rds-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RollbackTransactionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RollbackTransactionError::from_response(response))
        }
    }
}
