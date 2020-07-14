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
pub struct BatchGetNamedQueryInput {
    /// <p>An array of query IDs.</p>
    #[serde(rename = "NamedQueryIds")]
    pub named_query_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetQueryExecutionInput {
    /// <p>An array of query execution IDs.</p>
    #[serde(rename = "QueryExecutionIds")]
    pub query_execution_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Contains metadata for a column in a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Column {
    /// <p>Optional information about the column.</p>
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The name of the column.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The data type of the column.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about the columns in a query execution result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataCatalogInput {
    /// <p>A description of the data catalog to be created.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the data catalog to create. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies the Lambda function or functions to use for creating the data catalog. This is a mapping whose values depend on the catalog type. </p> <ul> <li> <p>For the <code>HIVE</code> data catalog type, use the following syntax. The <code>metadata-function</code> parameter is required. <code>The sdk-version</code> parameter is optional and defaults to the currently supported version.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, sdk-version=<i>version</em>number</i> </code> </p> </li> <li> <p>For the <code>LAMBDA</code> data catalog type, use one of the following sets of required parameters, but not both.</p> <ul> <li> <p>If you have one Lambda function that processes metadata and another for reading the actual data, use the following syntax. Both parameters are required.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, record-function=<i>lambda</em>arn</i> </code> </p> </li> <li> <p> If you have a composite Lambda function that processes both metadata and data, use the following syntax to specify your Lambda function.</p> <p> <code>function=<i>lambda_arn</i> </code> </p> </li> </ul> </li> <li> <p>The <code>GLUE</code> type has no parameters.</p> </li> </ul></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of comma separated tags to add to the data catalog that is created.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The type of data catalog to create: <code>LAMBDA</code> for a federated catalog, <code>GLUE</code> for AWS Glue Catalog, or <code>HIVE</code> for an external hive metastore.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataCatalogOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNamedQueryOutput {
    /// <p>The unique ID of the query.</p>
    #[serde(rename = "NamedQueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>A list of comma separated tags to add to the workgroup that is created.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWorkGroupOutput {}

/// <p>Contains information about a data catalog in an AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataCatalog {
    /// <p>An optional description of the data catalog.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the data catalog. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies the Lambda function or functions to use for the data catalog. This is a mapping whose values depend on the catalog type. </p> <ul> <li> <p>For the <code>HIVE</code> data catalog type, use the following syntax. The <code>metadata-function</code> parameter is required. <code>The sdk-version</code> parameter is optional and defaults to the currently supported version.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, sdk-version=<i>version</em>number</i> </code> </p> </li> <li> <p>For the <code>LAMBDA</code> data catalog type, use one of the following sets of required parameters, but not both.</p> <ul> <li> <p>If you have one Lambda function that processes metadata and another for reading the actual data, use the following syntax. Both parameters are required.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, record-function=<i>lambda</em>arn</i> </code> </p> </li> <li> <p> If you have a composite Lambda function that processes both metadata and data, use the following syntax to specify your Lambda function.</p> <p> <code>function=<i>lambda_arn</i> </code> </p> </li> </ul> </li> <li> <p>The <code>GLUE</code> type has no parameters.</p> </li> </ul></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of data catalog: <code>LAMBDA</code> for a federated catalog, <code>GLUE</code> for AWS Glue Catalog, or <code>HIVE</code> for an external hive metastore.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>The summary information for the data catalog, which includes its name and type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataCatalogSummary {
    /// <p>The name of the data catalog.</p>
    #[serde(rename = "CatalogName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    /// <p>The data catalog type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains metadata information for a database in a data catalog.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Database {
    /// <p>An optional description of the database.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the database.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A set of custom key/value pairs.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A piece of data (a field in the table).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Datum {
    /// <p>The value of the datum.</p>
    #[serde(rename = "VarCharValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_char_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataCatalogInput {
    /// <p>The name of the data catalog to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDataCatalogOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNamedQueryInput {
    /// <p>The unique ID of the query to delete.</p>
    #[serde(rename = "NamedQueryId")]
    pub named_query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNamedQueryOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataCatalogInput {
    /// <p>The name of the data catalog to return.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataCatalogOutput {
    /// <p>The data catalog returned.</p>
    #[serde(rename = "DataCatalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog: Option<DataCatalog>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDatabaseInput {
    /// <p>The name of the data catalog that contains the database to return.</p>
    #[serde(rename = "CatalogName")]
    pub catalog_name: String,
    /// <p>The name of the database to return.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDatabaseOutput {
    /// <p>The database returned.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNamedQueryInput {
    /// <p>The unique ID of the query. Use <a>ListNamedQueries</a> to get query IDs.</p>
    #[serde(rename = "NamedQueryId")]
    pub named_query_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNamedQueryOutput {
    /// <p>Information about the query.</p>
    #[serde(rename = "NamedQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query: Option<NamedQuery>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueryExecutionInput {
    /// <p>The unique ID of the query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQueryExecutionOutput {
    /// <p>Information about the query execution.</p>
    #[serde(rename = "QueryExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution: Option<QueryExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueryResultsInput {
    /// <p>The maximum number of results (rows) to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique ID of the query execution.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQueryResultsOutput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTableMetadataInput {
    /// <p>The name of the data catalog that contains the database and table metadata to return.</p>
    #[serde(rename = "CatalogName")]
    pub catalog_name: String,
    /// <p>The name of the database that contains the table metadata to return.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the table for which metadata is returned.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTableMetadataOutput {
    /// <p>An object that contains table metadata.</p>
    #[serde(rename = "TableMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_metadata: Option<TableMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWorkGroupInput {
    /// <p>The name of the workgroup.</p>
    #[serde(rename = "WorkGroup")]
    pub work_group: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWorkGroupOutput {
    /// <p>Information about the workgroup.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<WorkGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataCatalogsInput {
    /// <p>Specifies the maximum number of data catalogs to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataCatalogsOutput {
    /// <p>A summary list of data catalogs.</p>
    #[serde(rename = "DataCatalogsSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalogs_summary: Option<Vec<DataCatalogSummary>>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatabasesInput {
    /// <p>The name of the data catalog that contains the databases to return.</p>
    #[serde(rename = "CatalogName")]
    pub catalog_name: String,
    /// <p>Specifies the maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatabasesOutput {
    /// <p>A list of databases from a data catalog.</p>
    #[serde(rename = "DatabaseList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_list: Option<Vec<Database>>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNamedQueriesInput {
    /// <p>The maximum number of queries to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the workgroup from which the named queries are being returned. If a workgroup is not specified, the saved queries for the primary workgroup are returned.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNamedQueriesOutput {
    /// <p>The list of unique query IDs.</p>
    #[serde(rename = "NamedQueryIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_ids: Option<Vec<String>>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQueryExecutionsInput {
    /// <p>The maximum number of query executions to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the workgroup from which queries are being returned. If a workgroup is not specified, a list of available query execution IDs for the queries in the primary workgroup is returned.</p>
    #[serde(rename = "WorkGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTableMetadataInput {
    /// <p>The name of the data catalog for which table metadata should be returned.</p>
    #[serde(rename = "CatalogName")]
    pub catalog_name: String,
    /// <p>The name of the database for which table metadata should be returned.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>A regex filter that pattern-matches table names. If no expression is supplied, metadata for all tables are listed.</p>
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>Specifies the maximum number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTableMetadataOutput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the NextToken from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of table metadata.</p>
    #[serde(rename = "TableMetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_metadata_list: Option<Vec<TableMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>The maximum number of results to be returned per request that lists the tags for the resource.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or null if there are no additional results for this request, where the request lists the tags for the resource with the specified ARN.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Lists the tags for the resource with the specified ARN.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>A token to be used by the next request if this request is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of tags associated with the specified resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWorkGroupsInput {
    /// <p>The maximum number of workgroups to return in this request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWorkGroupsOutput {
    /// <p>A token generated by the Athena service that specifies where to continue pagination if a previous request was truncated. To obtain the next set of pages, pass in the <code>NextToken</code> from the response object of the previous page call.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Query execution statistics, such as the amount of data scanned, the amount of time that the query took to process, and the type of statement that was run.</p>
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

/// <p>The database and data catalog context in which the query execution occurs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryExecutionContext {
    /// <p>The name of the data catalog used in the query execution.</p>
    #[serde(rename = "Catalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    /// <p>The name of the database used in the query execution.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

/// <p>The amount of data scanned during the query execution and the amount of time that it took to execute, and the type of statement that was run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryExecutionStatistics {
    /// <p>The location and file name of a data manifest file. The manifest file is saved to the Athena query results location in Amazon S3. The manifest file tracks files that the query wrote to Amazon S3. If the query fails, the manifest file also tracks files that the query intended to write. The manifest is useful for identifying orphaned files resulting from a failed query. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Working with Query Results, Output Files, and Query History</a> in the <i>Amazon Athena User Guide</i>.</p>
    #[serde(rename = "DataManifestLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_manifest_location: Option<String>,
    /// <p>The number of bytes in the data that was queried.</p>
    #[serde(rename = "DataScannedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_scanned_in_bytes: Option<i64>,
    /// <p>The number of milliseconds that the query took to execute.</p>
    #[serde(rename = "EngineExecutionTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_execution_time_in_millis: Option<i64>,
    /// <p>The number of milliseconds that Athena took to plan the query processing flow. This includes the time spent retrieving table partitions from the data source. Note that because the query engine performs the query planning, query planning time is a subset of engine processing time.</p>
    #[serde(rename = "QueryPlanningTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_planning_time_in_millis: Option<i64>,
    /// <p>The number of milliseconds that the query was in your query queue waiting for resources. Note that if transient errors occur, Athena might automatically add the query back to the queue.</p>
    #[serde(rename = "QueryQueueTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_queue_time_in_millis: Option<i64>,
    /// <p>The number of milliseconds that Athena took to finalize and publish the query results after the query engine finished running the query.</p>
    #[serde(rename = "ServiceProcessingTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_processing_time_in_millis: Option<i64>,
    /// <p>The number of milliseconds that Athena took to run the query.</p>
    #[serde(rename = "TotalExecutionTimeInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_time_in_millis: Option<i64>,
}

/// <p>The completion date, current state, submission time, and state change reason (if applicable) for the query execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryExecutionStatus {
    /// <p>The date and time that the query completed.</p>
    #[serde(rename = "CompletionDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date_time: Option<f64>,
    /// <p><p>The state of query execution. <code>QUEUED</code> indicates that the query has been submitted to the service, and Athena will execute the query as soon as resources are available. <code>RUNNING</code> indicates that the query is in execution phase. <code>SUCCEEDED</code> indicates that the query completed without errors. <code>FAILED</code> indicates that the query experienced an error and did not complete processing. <code>CANCELLED</code> indicates that a user input interrupted query execution.</p> <note> <p>Athena automatically retries your queries in cases of certain transient errors. As a result, you may see the query state transition from <code>RUNNING</code> or <code>FAILED</code> to <code>QUEUED</code>. </p> </note></p>
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

/// <p>The location in Amazon S3 where query results are stored and the encryption option, if any, used for query results. These are known as "client-side settings". If workgroup settings override client-side settings, then the query uses the workgroup settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultConfiguration {
    /// <p>If query results are encrypted in Amazon S3, indicates the encryption option used (for example, <code>SSE-KMS</code> or <code>CSE-KMS</code>) and key information. This is a client-side setting. If workgroup settings override client-side settings, then the query uses the encryption configuration that is specified for the workgroup, and also uses the location for storing query results specified in the workgroup. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a> and <a href="https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html">Workgroup Settings Override Client-Side Settings</a>.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The location in Amazon S3 where your query results are stored, such as <code>s3://path/to/query/bucket/</code>. To run the query, you must specify the query results location using one of the ways: either for individual queries using either this setting (client-side), or in the workgroup, using <a>WorkGroupConfiguration</a>. If none of them is set, Athena issues an error that no output location is provided. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Query Results</a>. If workgroup settings override client-side settings, then the query uses the settings specified for the workgroup. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
}

/// <p>The information about the updates in the query results, such as output location and encryption configuration for the query results.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResultConfigurationUpdates {
    /// <p>The encryption configuration for the query results.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The location in Amazon S3 where your query results are stored, such as <code>s3://path/to/query/bucket/</code>. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Query Results</a> If workgroup settings override client-side settings, then the query uses the location for the query results and the encryption configuration that are specified for the workgroup. The "workgroup settings override" is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
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

/// <p>The metadata and rows that comprise a query result set. The metadata describes the column structure and data types. To return a <code>ResultSet</code> object, use <a>GetQueryResults</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The metadata that describes the column structure and data types of a table of query results. To return a <code>ResultSetMetadata</code> object, use <a>GetQueryResults</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResultSetMetadata {
    /// <p>Information about the columns returned in a query result metadata.</p>
    #[serde(rename = "ColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_info: Option<Vec<ColumnInfo>>,
}

/// <p>The rows that comprise a query result table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Row {
    /// <p>The data that populates a row in a query result table.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Datum>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartQueryExecutionOutput {
    /// <p>The unique ID of the query that ran as a result of this request.</p>
    #[serde(rename = "QueryExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopQueryExecutionInput {
    /// <p>The unique ID of the query execution to stop.</p>
    #[serde(rename = "QueryExecutionId")]
    pub query_execution_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopQueryExecutionOutput {}

/// <p>Contains metadata for a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TableMetadata {
    /// <p>A list of the columns in the table.</p>
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    /// <p>The time that the table was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The last time the table was accessed.</p>
    #[serde(rename = "LastAccessTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    /// <p>The name of the table.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A set of custom key/value pairs for table properties.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of the partition keys in the table.</p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    /// <p>The type of table. In Athena, only <code>EXTERNAL_TABLE</code> is supported.</p>
    #[serde(rename = "TableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
}

/// <p>A label that you assign to a resource. In Athena, a resource can be a workgroup or data catalog. Each tag consists of a key and an optional value, both of which you define. For example, you can use tags to categorize Athena workgroups or data catalogs by purpose, owner, or environment. Use a consistent set of tag keys to make it easier to search and filter workgroups or data catalogs in your account. For best practices, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">Tagging Best Practices</a>. Tag keys can be from 1 to 128 UTF-8 Unicode characters, and tag values can be from 0 to 256 UTF-8 Unicode characters. Tags can use letters and numbers representable in UTF-8, and the following characters: + - = . _ : / @. Tag keys and values are case-sensitive. Tag keys must be unique per resource. If you specify more than one tag, separate them by commas. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A tag key. The tag key length is from 1 to 128 Unicode characters in UTF-8. You can use letters and numbers representable in UTF-8, and the following characters: + - = . _ : / @. Tag keys are case-sensitive and must be unique per resource. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A tag value. The tag value length is from 0 to 256 Unicode characters in UTF-8. You can use letters and numbers representable in UTF-8, and the following characters: + - = . _ : / @. Tag values are case-sensitive. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceInput {
    /// <p>Specifies the ARN of the Athena resource (workgroup or data catalog) to which tags are to be added.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A collection of one or more tags, separated by commas, to be added to an Athena workgroup or data catalog resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceOutput {}

/// <p>Information about a named query ID that could not be processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceInput {
    /// <p>Specifies the ARN of the resource from which tags are to be removed.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A comma-separated list of one or more tag keys whose tags are to be removed from the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataCatalogInput {
    /// <p>New or modified text that describes the data catalog.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the data catalog to update. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Specifies the Lambda function or functions to use for updating the data catalog. This is a mapping whose values depend on the catalog type. </p> <ul> <li> <p>For the <code>HIVE</code> data catalog type, use the following syntax. The <code>metadata-function</code> parameter is required. <code>The sdk-version</code> parameter is optional and defaults to the currently supported version.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, sdk-version=<i>version</em>number</i> </code> </p> </li> <li> <p>For the <code>LAMBDA</code> data catalog type, use one of the following sets of required parameters, but not both.</p> <ul> <li> <p>If you have one Lambda function that processes metadata and another for reading the actual data, use the following syntax. Both parameters are required.</p> <p> <code>metadata-function=<i>lambda<em>arn</i>, record-function=<i>lambda</em>arn</i> </code> </p> </li> <li> <p> If you have a composite Lambda function that processes both metadata and data, use the following syntax to specify your Lambda function.</p> <p> <code>function=<i>lambda_arn</i> </code> </p> </li> </ul> </li> <li> <p>The <code>GLUE</code> type has no parameters.</p> </li> </ul></p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the type of data catalog to update. Specify <code>LAMBDA</code> for a federated catalog, <code>GLUE</code> for AWS Glue Catalog, or <code>HIVE</code> for an external hive metastore.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDataCatalogOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWorkGroupOutput {}

/// <p>A workgroup, which contains a name, description, creation time, state, and other configuration, listed under <a>WorkGroup$Configuration</a>. Each workgroup enables you to isolate queries for you or your group of users from other queries in the same account, to configure the query results location and the encryption configuration (known as workgroup settings), to enable sending query metrics to Amazon CloudWatch, and to establish per-query data usage control limits for all queries in a workgroup. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WorkGroup {
    /// <p>The configuration of the workgroup, which includes the location in Amazon S3 where query results are stored, the encryption configuration, if any, used for query results; whether the Amazon CloudWatch Metrics are enabled for the workgroup; whether workgroup settings override client-side settings; and the data usage limits for the amount of data scanned per query or per workgroup. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>.</p>
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

/// <p>The configuration of the workgroup, which includes the location in Amazon S3 where query results are stored, the encryption option, if any, used for query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup and whether workgroup settings override query settings, and the data usage limits for the amount of data scanned per query or per workgroup. The workgroup settings override is specified in EnforceWorkGroupConfiguration (true/false) in the WorkGroupConfiguration. See <a>WorkGroupConfiguration$EnforceWorkGroupConfiguration</a>. </p>
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
    /// <p>If set to <code>true</code>, allows members assigned to a workgroup to reference Amazon S3 Requester Pays buckets in queries. If set to <code>false</code>, workgroup members cannot query data from Requester Pays buckets, and queries that retrieve data from Requester Pays buckets cause an error. The default is <code>false</code>. For more information about Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html">Requester Pays Buckets</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    #[serde(rename = "RequesterPaysEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays_enabled: Option<bool>,
    /// <p>The configuration for the workgroup, which includes the location in Amazon S3 where query results are stored and the encryption option, if any, used for query results. To run the query, you must specify the query results location using one of the ways: either in the workgroup using this setting, or for individual queries (client-side), using <a>ResultConfiguration$OutputLocation</a>. If none of them is set, Athena issues an error that no output location is provided. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Query Results</a>.</p>
    #[serde(rename = "ResultConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
}

/// <p>The configuration information that will be updated for this workgroup, which includes the location in Amazon S3 where query results are stored, the encryption option, if any, used for query results, whether the Amazon CloudWatch Metrics are enabled for the workgroup, whether the workgroup settings override the client-side settings, and the data usage limit for the amount of bytes scanned per query, if it is specified.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>If set to <code>true</code>, allows members assigned to a workgroup to specify Amazon S3 Requester Pays buckets in queries. If set to <code>false</code>, workgroup members cannot query data from Requester Pays buckets, and queries that retrieve data from Requester Pays buckets cause an error. The default is <code>false</code>. For more information about Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html">Requester Pays Buckets</a> in the <i>Amazon Simple Storage Service Developer Guide</i>.</p>
    #[serde(rename = "RequesterPaysEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays_enabled: Option<bool>,
    /// <p>The result configuration information about the queries in this workgroup that will be updated. Includes the updated results location and an updated option for encrypting query results.</p>
    #[serde(rename = "ResultConfigurationUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration_updates: Option<ResultConfigurationUpdates>,
}

/// <p>The summary information for the workgroup, which includes its name, state, description, and the date and time it was created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetNamedQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetNamedQueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchGetNamedQueryError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetNamedQueryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetQueryExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetQueryExecutionError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchGetQueryExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetQueryExecutionError {}
/// Errors returned by CreateDataCatalog
#[derive(Debug, PartialEq)]
pub enum CreateDataCatalogError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl CreateDataCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataCatalogError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDataCatalogError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataCatalogError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataCatalogError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataCatalogError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateNamedQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNamedQueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateNamedQueryError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNamedQueryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWorkGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWorkGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateWorkGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWorkGroupError {}
/// Errors returned by DeleteDataCatalog
#[derive(Debug, PartialEq)]
pub enum DeleteDataCatalogError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl DeleteDataCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDataCatalogError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDataCatalogError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDataCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDataCatalogError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDataCatalogError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataCatalogError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNamedQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNamedQueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteNamedQueryError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNamedQueryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteWorkGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWorkGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteWorkGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWorkGroupError {}
/// Errors returned by GetDataCatalog
#[derive(Debug, PartialEq)]
pub enum GetDataCatalogError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl GetDataCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDataCatalogError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDataCatalogError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDataCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataCatalogError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDataCatalogError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataCatalogError {}
/// Errors returned by GetDatabase
#[derive(Debug, PartialEq)]
pub enum GetDatabaseError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>An exception that Athena received when it called a custom metastore. Occurs if the error is not caused by user input (<code>InvalidRequestException</code>) or from the Athena platform (<code>InternalServerException</code>). For example, if a user-created Lambda function is missing permissions, the Lambda <code>4XX</code> exception is returned in a <code>MetadataException</code>.</p>
    Metadata(String),
}

impl GetDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDatabaseError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDatabaseError::InvalidRequest(err.msg))
                }
                "MetadataException" => {
                    return RusotoError::Service(GetDatabaseError::Metadata(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDatabaseError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDatabaseError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetDatabaseError::Metadata(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDatabaseError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNamedQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNamedQueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetNamedQueryError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNamedQueryError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetQueryExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueryExecutionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetQueryExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQueryExecutionError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetQueryResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueryResultsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetQueryResultsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQueryResultsError {}
/// Errors returned by GetTableMetadata
#[derive(Debug, PartialEq)]
pub enum GetTableMetadataError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>An exception that Athena received when it called a custom metastore. Occurs if the error is not caused by user input (<code>InvalidRequestException</code>) or from the Athena platform (<code>InternalServerException</code>). For example, if a user-created Lambda function is missing permissions, the Lambda <code>4XX</code> exception is returned in a <code>MetadataException</code>.</p>
    Metadata(String),
}

impl GetTableMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTableMetadataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetTableMetadataError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetTableMetadataError::InvalidRequest(err.msg))
                }
                "MetadataException" => {
                    return RusotoError::Service(GetTableMetadataError::Metadata(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTableMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTableMetadataError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetTableMetadataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetTableMetadataError::Metadata(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTableMetadataError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWorkGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWorkGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetWorkGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWorkGroupError {}
/// Errors returned by ListDataCatalogs
#[derive(Debug, PartialEq)]
pub enum ListDataCatalogsError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl ListDataCatalogsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataCatalogsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDataCatalogsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDataCatalogsError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataCatalogsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataCatalogsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDataCatalogsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataCatalogsError {}
/// Errors returned by ListDatabases
#[derive(Debug, PartialEq)]
pub enum ListDatabasesError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>An exception that Athena received when it called a custom metastore. Occurs if the error is not caused by user input (<code>InvalidRequestException</code>) or from the Athena platform (<code>InternalServerException</code>). For example, if a user-created Lambda function is missing permissions, the Lambda <code>4XX</code> exception is returned in a <code>MetadataException</code>.</p>
    Metadata(String),
}

impl ListDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDatabasesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDatabasesError::InvalidRequest(err.msg))
                }
                "MetadataException" => {
                    return RusotoError::Service(ListDatabasesError::Metadata(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDatabasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatabasesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDatabasesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDatabasesError::Metadata(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatabasesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNamedQueriesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNamedQueriesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListNamedQueriesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNamedQueriesError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQueryExecutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueryExecutionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListQueryExecutionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueryExecutionsError {}
/// Errors returned by ListTableMetadata
#[derive(Debug, PartialEq)]
pub enum ListTableMetadataError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>An exception that Athena received when it called a custom metastore. Occurs if the error is not caused by user input (<code>InvalidRequestException</code>) or from the Athena platform (<code>InternalServerException</code>). For example, if a user-created Lambda function is missing permissions, the Lambda <code>4XX</code> exception is returned in a <code>MetadataException</code>.</p>
    Metadata(String),
}

impl ListTableMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTableMetadataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTableMetadataError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTableMetadataError::InvalidRequest(err.msg))
                }
                "MetadataException" => {
                    return RusotoError::Service(ListTableMetadataError::Metadata(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTableMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTableMetadataError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTableMetadataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTableMetadataError::Metadata(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTableMetadataError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>A resource, such as a workgroup, was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWorkGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWorkGroupsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListWorkGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWorkGroupsError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartQueryExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartQueryExecutionError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartQueryExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartQueryExecutionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartQueryExecutionError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopQueryExecutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopQueryExecutionError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopQueryExecutionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopQueryExecutionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>A resource, such as a workgroup, was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
    /// <p>A resource, such as a workgroup, was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDataCatalog
#[derive(Debug, PartialEq)]
pub enum UpdateDataCatalogError {
    /// <p>Indicates a platform issue, which may be due to a transient condition or outage.</p>
    InternalServer(String),
    /// <p>Indicates that something is wrong with the input to the request. For example, a required parameter may be missing or out of range.</p>
    InvalidRequest(String),
}

impl UpdateDataCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDataCatalogError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDataCatalogError::InvalidRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDataCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDataCatalogError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDataCatalogError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataCatalogError {}
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateWorkGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWorkGroupError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateWorkGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWorkGroupError {}
/// Trait representing the capabilities of the Amazon Athena API. Amazon Athena clients implement this trait.
#[async_trait]
pub trait Athena {
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Requires you to have access to the workgroup in which the queries were saved. Use <a>ListNamedQueriesInput</a> to get the list of named query IDs in the specified workgroup. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries differ from executed queries. Use <a>BatchGetQueryExecutionInput</a> to get details about each unique query execution, and <a>ListQueryExecutionsInput</a> to get a list of query execution IDs.</p>
    async fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> Result<BatchGetNamedQueryOutput, RusotoError<BatchGetNamedQueryError>>;

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. Requires you to have access to the workgroup in which the queries ran. To get a list of query execution IDs, use <a>ListQueryExecutionsInput$WorkGroup</a>. Query executions differ from named (saved) queries. Use <a>BatchGetNamedQueryInput</a> to get details about named queries.</p>
    async fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> Result<BatchGetQueryExecutionOutput, RusotoError<BatchGetQueryExecutionError>>;

    /// <p>Creates (registers) a data catalog with the specified name and properties. Catalogs created are visible to all users of the same AWS account.</p>
    async fn create_data_catalog(
        &self,
        input: CreateDataCatalogInput,
    ) -> Result<CreateDataCatalogOutput, RusotoError<CreateDataCatalogError>>;

    /// <p>Creates a named query in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> Result<CreateNamedQueryOutput, RusotoError<CreateNamedQueryError>>;

    /// <p>Creates a workgroup with the specified name.</p>
    async fn create_work_group(
        &self,
        input: CreateWorkGroupInput,
    ) -> Result<CreateWorkGroupOutput, RusotoError<CreateWorkGroupError>>;

    /// <p>Deletes a data catalog.</p>
    async fn delete_data_catalog(
        &self,
        input: DeleteDataCatalogInput,
    ) -> Result<DeleteDataCatalogOutput, RusotoError<DeleteDataCatalogError>>;

    /// <p>Deletes the named query if you have access to the workgroup in which the query was saved.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> Result<DeleteNamedQueryOutput, RusotoError<DeleteNamedQueryError>>;

    /// <p>Deletes the workgroup with the specified name. The primary workgroup cannot be deleted.</p>
    async fn delete_work_group(
        &self,
        input: DeleteWorkGroupInput,
    ) -> Result<DeleteWorkGroupOutput, RusotoError<DeleteWorkGroupError>>;

    /// <p>Returns the specified data catalog.</p>
    async fn get_data_catalog(
        &self,
        input: GetDataCatalogInput,
    ) -> Result<GetDataCatalogOutput, RusotoError<GetDataCatalogError>>;

    /// <p>Returns a database object for the specfied database and data catalog.</p>
    async fn get_database(
        &self,
        input: GetDatabaseInput,
    ) -> Result<GetDatabaseOutput, RusotoError<GetDatabaseError>>;

    /// <p>Returns information about a single query. Requires that you have access to the workgroup in which the query was saved.</p>
    async fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> Result<GetNamedQueryOutput, RusotoError<GetNamedQueryError>>;

    /// <p>Returns information about a single execution of a query if you have access to the workgroup in which the query ran. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    async fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> Result<GetQueryExecutionOutput, RusotoError<GetQueryExecutionError>>;

    /// <p><p>Streams the results of a single query execution specified by <code>QueryExecutionId</code> from the Athena query results location in Amazon S3. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Query Results</a> in the <i>Amazon Athena User Guide</i>. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p> <p>To stream query results successfully, the IAM principal with permission to call <code>GetQueryResults</code> also must have permissions to the Amazon S3 <code>GetObject</code> action for the Athena query results location.</p> <important> <p>IAM principals with permission to the Amazon S3 <code>GetObject</code> action for the query results location are able to retrieve query results from Amazon S3 even if permission to the <code>GetQueryResults</code> action is denied. To restrict user or role access, ensure that Amazon S3 permissions to the Athena query location are denied.</p> </important></p>
    async fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> Result<GetQueryResultsOutput, RusotoError<GetQueryResultsError>>;

    /// <p>Returns table metadata for the specified catalog, database, and table.</p>
    async fn get_table_metadata(
        &self,
        input: GetTableMetadataInput,
    ) -> Result<GetTableMetadataOutput, RusotoError<GetTableMetadataError>>;

    /// <p>Returns information about the workgroup with the specified name.</p>
    async fn get_work_group(
        &self,
        input: GetWorkGroupInput,
    ) -> Result<GetWorkGroupOutput, RusotoError<GetWorkGroupError>>;

    /// <p>Lists the data catalogs in the current AWS account.</p>
    async fn list_data_catalogs(
        &self,
        input: ListDataCatalogsInput,
    ) -> Result<ListDataCatalogsOutput, RusotoError<ListDataCatalogsError>>;

    /// <p>Lists the databases in the specified data catalog.</p>
    async fn list_databases(
        &self,
        input: ListDatabasesInput,
    ) -> Result<ListDatabasesOutput, RusotoError<ListDatabasesError>>;

    /// <p>Provides a list of available query IDs only for queries saved in the specified workgroup. Requires that you have access to the specified workgroup. If a workgroup is not specified, lists the saved queries for the primary workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> Result<ListNamedQueriesOutput, RusotoError<ListNamedQueriesError>>;

    /// <p>Provides a list of available query execution IDs for the queries in the specified workgroup. If a workgroup is not specified, returns a list of query execution IDs for the primary workgroup. Requires you to have access to the workgroup in which the queries ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> Result<ListQueryExecutionsOutput, RusotoError<ListQueryExecutionsError>>;

    /// <p>Lists the metadata for the tables in the specified data catalog database.</p>
    async fn list_table_metadata(
        &self,
        input: ListTableMetadataInput,
    ) -> Result<ListTableMetadataOutput, RusotoError<ListTableMetadataError>>;

    /// <p>Lists the tags associated with an Athena workgroup or data catalog resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists available workgroups for the account.</p>
    async fn list_work_groups(
        &self,
        input: ListWorkGroupsInput,
    ) -> Result<ListWorkGroupsOutput, RusotoError<ListWorkGroupsError>>;

    /// <p>Runs the SQL query statements contained in the <code>Query</code>. Requires you to have access to the workgroup in which the query ran. Running queries against an external catalog requires <a>GetDataCatalog</a> permission to the catalog. For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> Result<StartQueryExecutionOutput, RusotoError<StartQueryExecutionError>>;

    /// <p>Stops a query execution. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> Result<StopQueryExecutionOutput, RusotoError<StopQueryExecutionError>>;

    /// <p>Adds one or more tags to an Athena resource. A tag is a label that you assign to a resource. In Athena, a resource can be a workgroup or data catalog. Each tag consists of a key and an optional value, both of which you define. For example, you can use tags to categorize Athena workgroups or data catalogs by purpose, owner, or environment. Use a consistent set of tag keys to make it easier to search and filter workgroups or data catalogs in your account. For best practices, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">Tagging Best Practices</a>. Tag keys can be from 1 to 128 UTF-8 Unicode characters, and tag values can be from 0 to 256 UTF-8 Unicode characters. Tags can use letters and numbers representable in UTF-8, and the following characters: + - = . _ : / @. Tag keys and values are case-sensitive. Tag keys must be unique per resource. If you specify more than one tag, separate them by commas.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a data catalog or workgroup resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>>;

    /// <p>Updates the data catalog that has the specified name.</p>
    async fn update_data_catalog(
        &self,
        input: UpdateDataCatalogInput,
    ) -> Result<UpdateDataCatalogOutput, RusotoError<UpdateDataCatalogError>>;

    /// <p>Updates the workgroup with the specified name. The workgroup's name cannot be changed.</p>
    async fn update_work_group(
        &self,
        input: UpdateWorkGroupInput,
    ) -> Result<UpdateWorkGroupOutput, RusotoError<UpdateWorkGroupError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AthenaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AthenaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AthenaClient {
        AthenaClient { client, region }
    }
}

#[async_trait]
impl Athena for AthenaClient {
    /// <p>Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Requires you to have access to the workgroup in which the queries were saved. Use <a>ListNamedQueriesInput</a> to get the list of named query IDs in the specified workgroup. If information could not be retrieved for a submitted query ID, information about the query ID submitted is listed under <a>UnprocessedNamedQueryId</a>. Named queries differ from executed queries. Use <a>BatchGetQueryExecutionInput</a> to get details about each unique query execution, and <a>ListQueryExecutionsInput</a> to get a list of query execution IDs.</p>
    async fn batch_get_named_query(
        &self,
        input: BatchGetNamedQueryInput,
    ) -> Result<BatchGetNamedQueryOutput, RusotoError<BatchGetNamedQueryError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.BatchGetNamedQuery");
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
                .deserialize::<BatchGetNamedQueryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetNamedQueryError::from_response(response))
        }
    }

    /// <p>Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. Requires you to have access to the workgroup in which the queries ran. To get a list of query execution IDs, use <a>ListQueryExecutionsInput$WorkGroup</a>. Query executions differ from named (saved) queries. Use <a>BatchGetNamedQueryInput</a> to get details about named queries.</p>
    async fn batch_get_query_execution(
        &self,
        input: BatchGetQueryExecutionInput,
    ) -> Result<BatchGetQueryExecutionOutput, RusotoError<BatchGetQueryExecutionError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.BatchGetQueryExecution");
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
                .deserialize::<BatchGetQueryExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetQueryExecutionError::from_response(response))
        }
    }

    /// <p>Creates (registers) a data catalog with the specified name and properties. Catalogs created are visible to all users of the same AWS account.</p>
    async fn create_data_catalog(
        &self,
        input: CreateDataCatalogInput,
    ) -> Result<CreateDataCatalogOutput, RusotoError<CreateDataCatalogError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.CreateDataCatalog");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDataCatalogOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataCatalogError::from_response(response))
        }
    }

    /// <p>Creates a named query in the specified workgroup. Requires that you have access to the workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn create_named_query(
        &self,
        input: CreateNamedQueryInput,
    ) -> Result<CreateNamedQueryOutput, RusotoError<CreateNamedQueryError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.CreateNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateNamedQueryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateNamedQueryError::from_response(response))
        }
    }

    /// <p>Creates a workgroup with the specified name.</p>
    async fn create_work_group(
        &self,
        input: CreateWorkGroupInput,
    ) -> Result<CreateWorkGroupOutput, RusotoError<CreateWorkGroupError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.CreateWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateWorkGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateWorkGroupError::from_response(response))
        }
    }

    /// <p>Deletes a data catalog.</p>
    async fn delete_data_catalog(
        &self,
        input: DeleteDataCatalogInput,
    ) -> Result<DeleteDataCatalogOutput, RusotoError<DeleteDataCatalogError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.DeleteDataCatalog");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteDataCatalogOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataCatalogError::from_response(response))
        }
    }

    /// <p>Deletes the named query if you have access to the workgroup in which the query was saved.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn delete_named_query(
        &self,
        input: DeleteNamedQueryInput,
    ) -> Result<DeleteNamedQueryOutput, RusotoError<DeleteNamedQueryError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.DeleteNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteNamedQueryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteNamedQueryError::from_response(response))
        }
    }

    /// <p>Deletes the workgroup with the specified name. The primary workgroup cannot be deleted.</p>
    async fn delete_work_group(
        &self,
        input: DeleteWorkGroupInput,
    ) -> Result<DeleteWorkGroupOutput, RusotoError<DeleteWorkGroupError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.DeleteWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteWorkGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteWorkGroupError::from_response(response))
        }
    }

    /// <p>Returns the specified data catalog.</p>
    async fn get_data_catalog(
        &self,
        input: GetDataCatalogInput,
    ) -> Result<GetDataCatalogOutput, RusotoError<GetDataCatalogError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetDataCatalog");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDataCatalogOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataCatalogError::from_response(response))
        }
    }

    /// <p>Returns a database object for the specfied database and data catalog.</p>
    async fn get_database(
        &self,
        input: GetDatabaseInput,
    ) -> Result<GetDatabaseOutput, RusotoError<GetDatabaseError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDatabaseOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDatabaseError::from_response(response))
        }
    }

    /// <p>Returns information about a single query. Requires that you have access to the workgroup in which the query was saved.</p>
    async fn get_named_query(
        &self,
        input: GetNamedQueryInput,
    ) -> Result<GetNamedQueryOutput, RusotoError<GetNamedQueryError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetNamedQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetNamedQueryOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetNamedQueryError::from_response(response))
        }
    }

    /// <p>Returns information about a single execution of a query if you have access to the workgroup in which the query ran. Each time a query executes, information about the query execution is saved with a unique ID.</p>
    async fn get_query_execution(
        &self,
        input: GetQueryExecutionInput,
    ) -> Result<GetQueryExecutionOutput, RusotoError<GetQueryExecutionError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetQueryExecution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetQueryExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetQueryExecutionError::from_response(response))
        }
    }

    /// <p><p>Streams the results of a single query execution specified by <code>QueryExecutionId</code> from the Athena query results location in Amazon S3. For more information, see <a href="https://docs.aws.amazon.com/athena/latest/ug/querying.html">Query Results</a> in the <i>Amazon Athena User Guide</i>. This request does not execute the query but returns results. Use <a>StartQueryExecution</a> to run a query.</p> <p>To stream query results successfully, the IAM principal with permission to call <code>GetQueryResults</code> also must have permissions to the Amazon S3 <code>GetObject</code> action for the Athena query results location.</p> <important> <p>IAM principals with permission to the Amazon S3 <code>GetObject</code> action for the query results location are able to retrieve query results from Amazon S3 even if permission to the <code>GetQueryResults</code> action is denied. To restrict user or role access, ensure that Amazon S3 permissions to the Athena query location are denied.</p> </important></p>
    async fn get_query_results(
        &self,
        input: GetQueryResultsInput,
    ) -> Result<GetQueryResultsOutput, RusotoError<GetQueryResultsError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetQueryResults");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetQueryResultsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetQueryResultsError::from_response(response))
        }
    }

    /// <p>Returns table metadata for the specified catalog, database, and table.</p>
    async fn get_table_metadata(
        &self,
        input: GetTableMetadataInput,
    ) -> Result<GetTableMetadataOutput, RusotoError<GetTableMetadataError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetTableMetadata");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetTableMetadataOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTableMetadataError::from_response(response))
        }
    }

    /// <p>Returns information about the workgroup with the specified name.</p>
    async fn get_work_group(
        &self,
        input: GetWorkGroupInput,
    ) -> Result<GetWorkGroupOutput, RusotoError<GetWorkGroupError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.GetWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetWorkGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetWorkGroupError::from_response(response))
        }
    }

    /// <p>Lists the data catalogs in the current AWS account.</p>
    async fn list_data_catalogs(
        &self,
        input: ListDataCatalogsInput,
    ) -> Result<ListDataCatalogsOutput, RusotoError<ListDataCatalogsError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListDataCatalogs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDataCatalogsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataCatalogsError::from_response(response))
        }
    }

    /// <p>Lists the databases in the specified data catalog.</p>
    async fn list_databases(
        &self,
        input: ListDatabasesInput,
    ) -> Result<ListDatabasesOutput, RusotoError<ListDatabasesError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDatabasesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDatabasesError::from_response(response))
        }
    }

    /// <p>Provides a list of available query IDs only for queries saved in the specified workgroup. Requires that you have access to the specified workgroup. If a workgroup is not specified, lists the saved queries for the primary workgroup.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn list_named_queries(
        &self,
        input: ListNamedQueriesInput,
    ) -> Result<ListNamedQueriesOutput, RusotoError<ListNamedQueriesError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListNamedQueries");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListNamedQueriesOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListNamedQueriesError::from_response(response))
        }
    }

    /// <p>Provides a list of available query execution IDs for the queries in the specified workgroup. If a workgroup is not specified, returns a list of query execution IDs for the primary workgroup. Requires you to have access to the workgroup in which the queries ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn list_query_executions(
        &self,
        input: ListQueryExecutionsInput,
    ) -> Result<ListQueryExecutionsOutput, RusotoError<ListQueryExecutionsError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListQueryExecutions");
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
                .deserialize::<ListQueryExecutionsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListQueryExecutionsError::from_response(response))
        }
    }

    /// <p>Lists the metadata for the tables in the specified data catalog database.</p>
    async fn list_table_metadata(
        &self,
        input: ListTableMetadataInput,
    ) -> Result<ListTableMetadataOutput, RusotoError<ListTableMetadataError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListTableMetadata");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListTableMetadataOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTableMetadataError::from_response(response))
        }
    }

    /// <p>Lists the tags associated with an Athena workgroup or data catalog resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists available workgroups for the account.</p>
    async fn list_work_groups(
        &self,
        input: ListWorkGroupsInput,
    ) -> Result<ListWorkGroupsOutput, RusotoError<ListWorkGroupsError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.ListWorkGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListWorkGroupsOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListWorkGroupsError::from_response(response))
        }
    }

    /// <p>Runs the SQL query statements contained in the <code>Query</code>. Requires you to have access to the workgroup in which the query ran. Running queries against an external catalog requires <a>GetDataCatalog</a> permission to the catalog. For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn start_query_execution(
        &self,
        input: StartQueryExecutionInput,
    ) -> Result<StartQueryExecutionOutput, RusotoError<StartQueryExecutionError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.StartQueryExecution");
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
                .deserialize::<StartQueryExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartQueryExecutionError::from_response(response))
        }
    }

    /// <p>Stops a query execution. Requires you to have access to the workgroup in which the query ran.</p> <p>For code samples using the AWS SDK for Java, see <a href="http://docs.aws.amazon.com/athena/latest/ug/code-samples.html">Examples and Code Samples</a> in the <i>Amazon Athena User Guide</i>.</p>
    async fn stop_query_execution(
        &self,
        input: StopQueryExecutionInput,
    ) -> Result<StopQueryExecutionOutput, RusotoError<StopQueryExecutionError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.StopQueryExecution");
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
                .deserialize::<StopQueryExecutionOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopQueryExecutionError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to an Athena resource. A tag is a label that you assign to a resource. In Athena, a resource can be a workgroup or data catalog. Each tag consists of a key and an optional value, both of which you define. For example, you can use tags to categorize Athena workgroups or data catalogs by purpose, owner, or environment. Use a consistent set of tag keys to make it easier to search and filter workgroups or data catalogs in your account. For best practices, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">Tagging Best Practices</a>. Tag keys can be from 1 to 128 UTF-8 Unicode characters, and tag values can be from 0 to 256 UTF-8 Unicode characters. Tags can use letters and numbers representable in UTF-8, and the following characters: + - = . _ : / @. Tag keys and values are case-sensitive. Tag keys must be unique per resource. If you specify more than one tag, separate them by commas.</p>
    async fn tag_resource(
        &self,
        input: TagResourceInput,
    ) -> Result<TagResourceOutput, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from a data catalog or workgroup resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceInput,
    ) -> Result<UntagResourceOutput, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the data catalog that has the specified name.</p>
    async fn update_data_catalog(
        &self,
        input: UpdateDataCatalogInput,
    ) -> Result<UpdateDataCatalogOutput, RusotoError<UpdateDataCatalogError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.UpdateDataCatalog");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateDataCatalogOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDataCatalogError::from_response(response))
        }
    }

    /// <p>Updates the workgroup with the specified name. The workgroup's name cannot be changed.</p>
    async fn update_work_group(
        &self,
        input: UpdateWorkGroupInput,
    ) -> Result<UpdateWorkGroupOutput, RusotoError<UpdateWorkGroupError>> {
        let mut request = SignedRequest::new("POST", "athena", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonAthena.UpdateWorkGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateWorkGroupOutput, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateWorkGroupError::from_response(response))
        }
    }
}
