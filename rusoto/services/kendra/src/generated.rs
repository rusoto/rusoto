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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
/// <p>Access Control List files for the documents in a data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessControlListConfiguration {
    /// <p>Path to the AWS S3 bucket that contains the ACL files.</p>
    #[serde(rename = "KeyPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
}

/// <p>Provides information about the column that should be used for filtering the query response by groups.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AclConfiguration {
    /// <p>A list of groups, separated by semi-colons, that filters a query response based on user context. The document is only returned to users that are in one of the groups specified in the <code>UserContext</code> field of the <a>Query</a> operation.</p>
    #[serde(rename = "AllowedGroupsColumnName")]
    pub allowed_groups_column_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdditionalResultAttribute {
    /// <p><p/></p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p><p/></p>
    #[serde(rename = "Value")]
    pub value: AdditionalResultAttributeValue,
    /// <p><p/></p>
    #[serde(rename = "ValueType")]
    pub value_type: String,
}

/// <p>An attribute returned with a document from a search.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AdditionalResultAttributeValue {
    /// <p>The text associated with the attribute and information about the highlight to apply to the text.</p>
    #[serde(rename = "TextWithHighlightsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_with_highlights_value: Option<TextWithHighlights>,
}

/// <p>Provides filtering the query results based on document attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttributeFilter {
    /// <p>Performs a logical <code>AND</code> operation on all supplied filters.</p>
    #[serde(rename = "AndAllFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_all_filters: Option<Vec<AttributeFilter>>,
    /// <p>Returns true when a document contains all of the specified document attributes.</p>
    #[serde(rename = "ContainsAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_all: Option<DocumentAttribute>,
    /// <p>Returns true when a document contains any of the specified document attributes.</p>
    #[serde(rename = "ContainsAny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_any: Option<DocumentAttribute>,
    /// <p>Performs an equals operation on two document attributes.</p>
    #[serde(rename = "EqualsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals_to: Option<DocumentAttribute>,
    /// <p>Performs a greater than operation on two document attributes. Use with a document attribute of type <code>Integer</code> or <code>Long</code>.</p>
    #[serde(rename = "GreaterThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<DocumentAttribute>,
    /// <p>Performs a greater or equals than operation on two document attributes. Use with a document attribute of type <code>Integer</code> or <code>Long</code>.</p>
    #[serde(rename = "GreaterThanOrEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equals: Option<DocumentAttribute>,
    /// <p>Performs a less than operation on two document attributes. Use with a document attribute of type <code>Integer</code> or <code>Long</code>.</p>
    #[serde(rename = "LessThan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<DocumentAttribute>,
    /// <p>Performs a less than or equals operation on two document attributes. Use with a document attribute of type <code>Integer</code> or <code>Long</code>.</p>
    #[serde(rename = "LessThanOrEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equals: Option<DocumentAttribute>,
    /// <p>Performs a logical <code>NOT</code> operation on all supplied filters.</p>
    #[serde(rename = "NotFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_filter: Box<Option<AttributeFilter>>,
    /// <p>Performs a logical <code>OR</code> operation on all supplied filters.</p>
    #[serde(rename = "OrAllFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_all_filters: Option<Vec<AttributeFilter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteDocumentRequest {
    /// <p>One or more identifiers for documents to delete from the index.</p>
    #[serde(rename = "DocumentIdList")]
    pub document_id_list: Vec<String>,
    /// <p>The identifier of the index that contains the documents to delete.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteDocumentResponse {
    /// <p>A list of documents that could not be removed from the index. Each entry contains an error message that indicates why the document couldn't be removed from the index.</p>
    #[serde(rename = "FailedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_documents: Option<Vec<BatchDeleteDocumentResponseFailedDocument>>,
}

/// <p>Provides information about documents that could not be removed from an index by the <a>BatchDeleteDocument</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteDocumentResponseFailedDocument {
    /// <p>The error code for why the document couldn't be removed from the index.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>An explanation for why the document couldn't be removed from the index.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the document that couldn't be removed from the index.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchPutDocumentRequest {
    /// <p>One or more documents to add to the index. </p> <p>Each document is limited to 5 Mb, the total size of the list is limited to 50 Mb.</p>
    #[serde(rename = "Documents")]
    pub documents: Vec<Document>,
    /// <p>The identifier of the index to add the documents to. You need to create the index first using the <a>CreateIndex</a> operation.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The Amazon Resource Name (ARN) of a role that is allowed to run the <code>BatchPutDocument</code> operation. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM Roles for Amazon Kendra</a>.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutDocumentResponse {
    /// <p>A list of documents that were not added to the index because the document failed a validation check. Each document contains an error message that indicates why the document couldn't be added to the index.</p> <p>If there was an error adding a document to an index the error is reported in your AWS CloudWatch log.</p>
    #[serde(rename = "FailedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_documents: Option<Vec<BatchPutDocumentResponseFailedDocument>>,
}

/// <p>Provides information about a document that could not be indexed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutDocumentResponseFailedDocument {
    /// <p>The type of error that caused the document to fail to be indexed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A description of the reason why the document could not be indexed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The unique identifier of the document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Gathers information about when a particular result was clicked by a user. Your application uses the <a>SubmitFeedback</a> operation to provide click information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ClickFeedback {
    /// <p>The Unix timestamp of the data and time that the result was clicked.</p>
    #[serde(rename = "ClickTime")]
    pub click_time: f64,
    /// <p>The unique identifier of the search result that was clicked.</p>
    #[serde(rename = "ResultId")]
    pub result_id: String,
}

/// <p>Provides information about how Amazon Kendra should use the columns of a database in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnConfiguration {
    /// <p>One to five columns that indicate when a document in the database has changed.</p>
    #[serde(rename = "ChangeDetectingColumns")]
    pub change_detecting_columns: Vec<String>,
    /// <p>The column that contains the contents of the document.</p>
    #[serde(rename = "DocumentDataColumnName")]
    pub document_data_column_name: String,
    /// <p>The column that provides the document's unique identifier.</p>
    #[serde(rename = "DocumentIdColumnName")]
    pub document_id_column_name: String,
    /// <p>The column that contains the title of the document.</p>
    #[serde(rename = "DocumentTitleColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_column_name: Option<String>,
    /// <p>An array of objects that map database column names to the corresponding fields in an index. You must first create the fields in the index using the <a>UpdateIndex</a> operation.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
}

/// <p>Provides the information necessary to connect to a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionConfiguration {
    /// <p>The name of the host for the database. Can be either a string (host.subdomain.domain.tld) or an IPv4 or IPv6 address.</p>
    #[serde(rename = "DatabaseHost")]
    pub database_host: String,
    /// <p>The name of the database containing the document data.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The port that the database uses for connections.</p>
    #[serde(rename = "DatabasePort")]
    pub database_port: i64,
    /// <p>The Amazon Resource Name (ARN) of credentials stored in AWS Secrets Manager. The credentials should be a user/password pair. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-database.html">Using a Database Data Source</a>. For more information about AWS Secrets Manager, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html"> What Is AWS Secrets Manager </a> in the <i>AWS Secrets Manager</i> user guide.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The name of the table that contains the document data.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataSourceRequest {
    /// <p>The connector configuration information that is required to access the repository.</p>
    #[serde(rename = "Configuration")]
    pub configuration: DataSourceConfiguration,
    /// <p>A description for the data source.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the index that should be associated with this data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>A unique name for the data source. A data source name can't be changed without deleting and recreating the data source.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the data source. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM Roles for Amazon Kendra</a>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Sets the frequency that Amazon Kendra will check the documents in your repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the <code>StartDataSourceSyncJob</code> operation to update the index.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The type of repository that contains the data source.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDataSourceResponse {
    /// <p>A unique identifier for the data source.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFaqRequest {
    /// <p>A description of the FAQ.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the index that contains the FAQ.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The name that should be associated with the FAQ.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of a role with permission to access the S3 bucket that contains the FAQs. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html">IAM Roles for Amazon Kendra</a>.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The S3 location of the FAQ input data.</p>
    #[serde(rename = "S3Path")]
    pub s3_path: S3Path,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFaqResponse {
    /// <p>The unique identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIndexRequest {
    /// <p>A description for the index.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name for the new index.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An IAM role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role used when you use the <code>BatchPutDocument</code> operation to index documents from an Amazon S3 bucket.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The identifier of the AWS KMS customer managed key (CMK) to use to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs.</p>
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIndexResponse {
    /// <p>The unique identifier of the index. Use this identifier when you query an index, set up a data source, or index a document.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Configuration information for a Amazon Kendra data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSourceConfiguration {
    /// <p>Provides information necessary to create a connector for a database.</p>
    #[serde(rename = "DatabaseConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_configuration: Option<DatabaseConfiguration>,
    /// <p>Provides information to create a connector for a document repository in an Amazon S3 bucket.</p>
    #[serde(rename = "S3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3DataSourceConfiguration>,
    /// <p>Provides information necessary to create a connector for a Microsoft SharePoint site.</p>
    #[serde(rename = "SharePointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_point_configuration: Option<SharePointConfiguration>,
}

/// <p>Summary information for a Amazon Kendra data source. Returned in a call to .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSourceSummary {
    /// <p>The UNIX datetime that the data source was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier for the data source.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the data source.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the data source. When the status is <code>ATIVE</code> the data source is ready to use.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the data source.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The UNIX datetime that the data source was lasted updated. </p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Provides information about a synchronization job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataSourceSyncJob {
    /// <p>If the reason that the synchronization failed is due to an error with the underlying data source, this field contains a code that identifies the error.</p>
    #[serde(rename = "DataSourceErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_error_code: Option<String>,
    /// <p>The UNIX datetime that the synchronization job was completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>If the <code>Status</code> field is set to <code>FAILED</code>, the <code>ErrorCode</code> field contains a the reason that the synchronization failed.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>If the <code>Status</code> field is set to <code>ERROR</code>, the <code>ErrorMessage</code> field contains a description of the error that caused the synchronization to fail.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>A unique identifier for the synchronization job.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// <p>The UNIX datetime that the synchronization job was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The execution status of the synchronization job. When the <code>Status</code> field is set to <code>SUCCEEDED</code>, the synchronization job is done. If the status code is set to <code>FAILED</code>, the <code>ErrorCode</code> and <code>ErrorMessage</code> fields give you the reason for the failure.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Maps a column or attribute in the data source to an index field. You must first create the fields in the index using the <a>UpdateIndex</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSourceToIndexFieldMapping {
    /// <p>The name of the column or attribute in the data source.</p>
    #[serde(rename = "DataSourceFieldName")]
    pub data_source_field_name: String,
    /// <p>The type of data stored in the column or attribute.</p>
    #[serde(rename = "DateFieldFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_field_format: Option<String>,
    /// <p>The name of the field in the index.</p>
    #[serde(rename = "IndexFieldName")]
    pub index_field_name: String,
}

/// <p>Provides information for connecting to an Amazon VPC.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSourceVpcConfiguration {
    /// <p>A list of identifiers of security groups within your Amazon VPC. The security groups should enable Amazon Kendra to connect to the data source.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>A list of identifiers for subnets within your Amazon VPC. The subnets should be able to connect to each other in the VPC, and they should have outgoing access to the Internet through a NAT device.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>Provides the information necessary to connect a database to an index. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    /// <p>Information about the database column that provides information for user context filtering.</p>
    #[serde(rename = "AclConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_configuration: Option<AclConfiguration>,
    /// <p>Information about where the index should get the document information from the database.</p>
    #[serde(rename = "ColumnConfiguration")]
    pub column_configuration: ColumnConfiguration,
    /// <p>The information necessary to connect to a database.</p>
    #[serde(rename = "ConnectionConfiguration")]
    pub connection_configuration: ConnectionConfiguration,
    /// <p>The type of database engine that runs the database.</p>
    #[serde(rename = "DatabaseEngineType")]
    pub database_engine_type: String,
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFaqRequest {
    /// <p>The identifier of the FAQ to remove.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The index to remove the FAQ from.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIndexRequest {
    /// <p>The identifier of the index to delete.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDataSourceRequest {
    /// <p>The unique identifier of the data source to describe.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDataSourceResponse {
    /// <p>Information that describes where the data source is located and how the data source is configured. The specific information in the description depends on the data source provider.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DataSourceConfiguration>,
    /// <p>The Unix timestamp of when the data source was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the data source.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a description of the error that caused the data source to fail.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the data source.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// <p>The name that you gave the data source when it was created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role that enables the data source to access its resources.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The schedule that Amazon Kendra will update the data source.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// <p>The current status of the data source. When the status is <code>ACTIVE</code> the data source is ready to use. When the status is <code>FAILED</code>, the <code>ErrorMessage</code> field contains the reason that the data source failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the data source.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Unix timestamp of when the data source was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFaqRequest {
    /// <p>The unique identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the FAQ.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFaqResponse {
    /// <p>The date and time that the FAQ was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the FAQ that you provided when it was created.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If the <code>Status</code> field is <code>FAILED</code>, the <code>ErrorMessage</code> field contains the reason why the FAQ failed.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identifier of the index that contains the FAQ.</p>
    #[serde(rename = "IndexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// <p>The name that you gave the FAQ when it was created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role that provides access to the S3 bucket containing the input files for the FAQ.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "S3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<S3Path>,
    /// <p>The status of the FAQ. It is ready to use when the status is <code>ACTIVE</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time that the FAQ was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIndexRequest {
    /// <p>The name of the index to describe.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIndexResponse {
    /// <p>The Unix datetime that the index was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the index.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Configuration settings for any metadata applied to the documents in the index.</p>
    #[serde(rename = "DocumentMetadataConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata_configurations: Option<Vec<DocumentMetadataConfiguration>>,
    /// <p>When th e<code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>the name of the index.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Provides information about the number of FAQ questions and answers and the number of text documents indexed.</p>
    #[serde(rename = "IndexStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_statistics: Option<IndexStatistics>,
    /// <p>The name of the index.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that gives Amazon Kendra permission to write to your Amazon Cloudwatch logs.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The identifier of the AWS KMS customer master key (CMK) used to encrypt your data. Amazon Kendra doesn't support asymmetric CMKs.</p>
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    /// <p>The current status of the index. When the value is <code>ACTIVE</code>, the index is ready for use. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field contains a message that explains why.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The Unix datetime that the index was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>A document in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Document {
    /// <p>Information to use for user context filtering.</p>
    #[serde(rename = "AccessControlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<Principal>>,
    /// <p>Custom attributes to apply to the document. Use the custom attributes to provide additional information for searching, to provide facets for refining searches, and to provide additional information in the query response.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<DocumentAttribute>>,
    /// <p>The contents of the document as a base-64 encoded string.</p>
    #[serde(rename = "Blob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<bytes::Bytes>,
    /// <p>The file type of the document in the <code>Blob</code> field.</p>
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>A unique identifier of the document in the index.</p>
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "S3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<S3Path>,
    /// <p>The title of the document.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>A custom attribute value assigned to a document. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentAttribute {
    /// <p>The identifier for the attribute.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    pub value: DocumentAttributeValue,
}

/// <p>The value of a custom document attribute. You can only provide one value for a custom attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentAttributeValue {
    /// <p>A date value expressed as seconds from the Unix epoch.</p>
    #[serde(rename = "DateValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_value: Option<f64>,
    /// <p>A long integer value.</p>
    #[serde(rename = "LongValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    /// <p>A list of strings. </p>
    #[serde(rename = "StringListValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list_value: Option<Vec<String>>,
    /// <p>A string, such as "department".</p>
    #[serde(rename = "StringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>Provides the count of documents that match a particular attribute when doing a faceted search.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentAttributeValueCountPair {
    /// <p>The number of documents in the response that have the attribute value for the key.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The value of the attribute. For example, "HR."</p>
    #[serde(rename = "DocumentAttributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_value: Option<DocumentAttributeValue>,
}

/// <p>Specifies the properties of a custom index field.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadataConfiguration {
    /// <p>The name of the index field.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Provides manual tuning parameters to determine how the field affects the search results.</p>
    #[serde(rename = "Relevance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance: Option<Relevance>,
    /// <p>Provides information about how the field is used during a search.</p>
    #[serde(rename = "Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Search>,
    /// <p>The data type of the index field. </p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Document metadata files that contain information such as the document access control information, source URI, document author, and custom attributes. Each metadata file contains metadata about a single document.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentsMetadataConfiguration {
    /// <p>A prefix used to filter metadata configuration files in the AWS S3 bucket. The S3 bucket might contain multiple metadata files. Use <code>S3Prefix</code> to include only the desired metadata files.</p>
    #[serde(rename = "S3Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
}

/// <p>Information a document attribute</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Facet {
    /// <p>The unique key for the document attribute.</p>
    #[serde(rename = "DocumentAttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_key: Option<String>,
}

/// <p>The facet values for the documents in the response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FacetResult {
    /// <p>The key for the facet values. This is the same as the <code>DocumentAttributeKey</code> provided in the query.</p>
    #[serde(rename = "DocumentAttributeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_key: Option<String>,
    /// <p>An array of key/value pairs, where the key is the value of the attribute and the count is the number of documents that share the key value.</p>
    #[serde(rename = "DocumentAttributeValueCountPairs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attribute_value_count_pairs: Option<Vec<DocumentAttributeValueCountPair>>,
}

/// <p>Provides statistical information about the FAQ questions and answers contained in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaqStatistics {
    /// <p>The total number of FAQ questions and answers contained in the index.</p>
    #[serde(rename = "IndexedQuestionAnswersCount")]
    pub indexed_question_answers_count: i64,
}

/// <p>Provides information about a frequently asked questions and answer contained in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaqSummary {
    /// <p>The UNIX datetime that the FAQ was added to the index.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The unique identifier of the FAQ.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name that you assigned the FAQ when you created or updated the FAQ.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the FAQ. When the status is <code>ACTIVE</code> the FAQ is ready for use.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The UNIX datetime that the FAQ was last updated.</p>
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Provides information that you can use to highlight a search result so that your users can quickly identify terms in the response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Highlight {
    /// <p>The zero-based location in the response string where the highlight starts.</p>
    #[serde(rename = "BeginOffset")]
    pub begin_offset: i64,
    /// <p>The zero-based location in the response string where the highlight ends.</p>
    #[serde(rename = "EndOffset")]
    pub end_offset: i64,
    /// <p>Indicates whether the response is the best response. True if this is the best response; otherwise, false.</p>
    #[serde(rename = "TopAnswer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_answer: Option<bool>,
}

/// <p>A summary of information about an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IndexConfigurationSummary {
    /// <p>The Unix timestamp when the index was created.</p>
    #[serde(rename = "CreatedAt")]
    pub created_at: f64,
    /// <p>A unique identifier for the index. Use this to identify the index when you are using operations such as <code>Query</code>, <code>DescribeIndex</code>, <code>UpdateIndex</code>, and <code>DeleteIndex</code>.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the index.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the index. When the status is <code>ACTIVE</code>, the index is ready to search.</p>
    #[serde(rename = "Status")]
    pub status: String,
    /// <p>The Unix timestamp when the index was last updated by the <code>UpdateIndex</code> operation.</p>
    #[serde(rename = "UpdatedAt")]
    pub updated_at: f64,
}

/// <p>Provides information about the number of documents and the number of questions and answers in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IndexStatistics {
    /// <p>The number of question and answer topics in the index.</p>
    #[serde(rename = "FaqStatistics")]
    pub faq_statistics: FaqStatistics,
    /// <p>The number of text documents indexed.</p>
    #[serde(rename = "TextDocumentStatistics")]
    pub text_document_statistics: TextDocumentStatistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSourceSyncJobsRequest {
    /// <p>The identifier of the data source.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The maximum number of synchronization jobs to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request to <code>GetDataSourceSyncJobHistory</code> was truncated, include the <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, the synchronization jobs returned in the list are limited to jobs between the specified dates. </p>
    #[serde(rename = "StartTimeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_filter: Option<TimeRange>,
    /// <p>When specified, only returns synchronization jobs with the <code>Status</code> field equal to the specified status.</p>
    #[serde(rename = "StatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSourceSyncJobsResponse {
    /// <p>A history of synchronization jobs for the data source.</p>
    #[serde(rename = "History")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<DataSourceSyncJob>>,
    /// <p>The <code>GetDataSourceSyncJobHistory</code> operation returns a page of vocabularies at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Kendra returns the NextPage token. Include the token in the next request to the <code>GetDataSourceSyncJobHistory</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataSourcesRequest {
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The maximum number of data sources to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Kendra returns a pagination token in the response. You can use this pagination token to retrieve the next set of data sources (<code>DataSourceSummaryItems</code>). </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataSourcesResponse {
    /// <p>If the response is truncated, Amazon Kendra returns this token that you can use in the subsequent request to retrieve the next set of data sources. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of summary information for one or more data sources.</p>
    #[serde(rename = "SummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_items: Option<Vec<DataSourceSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFaqsRequest {
    /// <p>The index that contains the FAQ lists.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The maximum number of FAQs to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request to <code>ListFaqs</code> was truncated, include the <code>NextToken</code> to fetch the next set of FAQs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFaqsResponse {
    /// <p>information about the FAQs associated with the specified index.</p>
    #[serde(rename = "FaqSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq_summary_items: Option<Vec<FaqSummary>>,
    /// <p>The <code>ListFaqs</code> operation returns a page of FAQs at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Kendra returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListFaqs</code> operation to return the next page of FAQs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIndicesRequest {
    /// <p>The maximum number of data sources to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Kendra returns a pagination token in the response. You can use this pagination token to retrieve the next set of indexes (<code>DataSourceSummaryItems</code>). </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIndicesResponse {
    /// <p>An array of summary information for one or more indexes.</p>
    #[serde(rename = "IndexConfigurationSummaryItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_configuration_summary_items: Option<Vec<IndexConfigurationSummary>>,
    /// <p>If the response is truncated, Amazon Kendra returns this token that you can use in the subsequent request to retrieve the next set of indexes.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Provides user and group information for document access filtering.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Principal {
    /// <p>Whether to allow or deny access to the principal.</p>
    #[serde(rename = "Access")]
    pub access: String,
    /// <p>The name of the user or group.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of principal.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryRequest {
    /// <p>Enables filtered searches based on document attributes. You can only provide one attribute filter; however, the <code>AndAllFilters</code>, <code>NotFilter</code>, and <code>OrAllFilters</code> parameters contain a list of other filters.</p> <p>The <code>AttributeFilter</code> parameter enables you to create a set of filtering rules that a document must satisfy to be included in the query results.</p>
    #[serde(rename = "AttributeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<AttributeFilter>,
    /// <p>An array of documents attributes. Amazon Kendra returns a count for each attribute key specified. You can use this information to help narrow the search for your user.</p>
    #[serde(rename = "Facets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<Facet>>,
    /// <p>The unique identifier of the index to search. The identifier is returned in the response from the operation.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>Query results are returned in pages the size of the <code>PageSize</code> parameter. By default, Amazon Kendra returns the first page of results. Use this parameter to get result pages after the first one.</p>
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// <p>Sets the number of results that are returned in each page of results. The default page size is 100.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>Sets the type of query. Only results for the specified query type are returned.</p>
    #[serde(rename = "QueryResultTypeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_type_filter: Option<String>,
    /// <p>The text to search for.</p>
    #[serde(rename = "QueryText")]
    pub query_text: String,
    /// <p>An array of document attributes to include in the response. No other document attributes are included in the response. By default all document attributes are included in the response. </p>
    #[serde(rename = "RequestedDocumentAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_document_attributes: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryResult {
    /// <p>Contains the facet results. A <code>FacetResult</code> contains the counts for each attribute key that was specified in the <code>Facets</code> input parameter.</p>
    #[serde(rename = "FacetResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_results: Option<Vec<FacetResult>>,
    /// <p>The unique identifier for the search. You use <code>QueryId</code> to identify the search when using the feedback API.</p>
    #[serde(rename = "QueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    /// <p>The results of the search.</p>
    #[serde(rename = "ResultItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_items: Option<Vec<QueryResultItem>>,
    /// <p>The number of items returned by the search. Use this to determine when you have requested the last set of results.</p>
    #[serde(rename = "TotalNumberOfResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_results: Option<i64>,
}

/// <p>A single query result.</p> <p>A query result contains information about a document returned by the query. This includes the original location of the document, a list of attributes assigned to the document, and relevant text from the document that satisfies the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryResultItem {
    /// <p><p/></p>
    #[serde(rename = "AdditionalAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_attributes: Option<Vec<AdditionalResultAttribute>>,
    /// <p>An array of document attributes for the document that the query result maps to. For example, the document author (Author) or the source URI (SourceUri) of the document.</p>
    #[serde(rename = "DocumentAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_attributes: Option<Vec<DocumentAttribute>>,
    /// <p>An extract of the text in the document. Contains information about highlighting the relevant terms in the excerpt.</p>
    #[serde(rename = "DocumentExcerpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_excerpt: Option<TextWithHighlights>,
    /// <p>The unique identifier for the document.</p>
    #[serde(rename = "DocumentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// <p>The title of the document. Contains the text of the title and information for highlighting the relevant terms in the title.</p>
    #[serde(rename = "DocumentTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title: Option<TextWithHighlights>,
    /// <p>The URI of the original location of the document.</p>
    #[serde(rename = "DocumentURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_uri: Option<String>,
    /// <p>The unique identifier for the query result.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of document. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information for manually tuning the relevance of a field in a search. When a query includes terms that match the field, the results are given a boost in the response based on these tuning parameters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relevance {
    /// <p>Specifies the time period that the boost applies to. For example, to make the boost apply to documents with the field value within the last month, you would use "2628000s". Once the field value is beyond the specified range, the effect of the boost drops off. The higher the importance, the faster the effect drops off. If you don't specify a value, the default is 3 months. The value of the field is a numeric string followed by the character "s", for example "86400s" for one day, or "604800s" for one week. </p> <p>Only applies to <code>DATE</code> fields.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// <p>Indicates that this field determines how "fresh" a document is. For example, if document 1 was created on November 5, and document 2 was created on October 31, document 1 is "fresher" than document 2. You can only set the <code>Freshness</code> field on one <code>DATE</code> type field. Only applies to <code>DATE</code> fields.</p>
    #[serde(rename = "Freshness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<bool>,
    /// <p>The relative importance of the field in the search. Larger numbers provide more of a boost than smaller numbers.</p>
    #[serde(rename = "Importance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub importance: Option<i64>,
    /// <p>Determines how values should be interpreted.</p> <p>When the <code>RankOrder</code> field is <code>ASCENDING</code>, higher numbers are better. For example, a document with a rating score of 10 is higher ranking than a document with a rating score of 1.</p> <p>When the <code>RankOrder</code> field is <code>DESCENDING</code>, lower numbers are better. For example, in a task tracking application, a priority 1 task is more important than a priority 5 task.</p> <p>Only applies to <code>LONG</code> and <code>DOUBLE</code> fields.</p>
    #[serde(rename = "RankOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_order: Option<String>,
    /// <p>A list of values that should be given a different boost when they appear in the result list. For example, if you are boosting a field called "department," query terms that match the department field are boosted in the result. However, you can add entries from the department field to boost documents with those values higher. </p> <p>For example, you can add entries to the map with names of departments. If you add "HR",5 and "Legal",3 those departments are given special attention when they appear in the metadata of a document. When those terms appear they are given the specified importance instead of the regular importance for the boost.</p>
    #[serde(rename = "ValueImportanceMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_importance_map: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>Provides feedback on how relevant a document is to a search. Your application uses the <a>SubmitFeedback</a> operation to provide relevance information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RelevanceFeedback {
    /// <p>Whether to document was relevant or not relevant to the search.</p>
    #[serde(rename = "RelevanceValue")]
    pub relevance_value: String,
    /// <p>The unique identifier of the search result that the user provided relevance feedback for.</p>
    #[serde(rename = "ResultId")]
    pub result_id: String,
}

/// <p>Provides configuration information for a data source to index documents in an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3DataSourceConfiguration {
    /// <p>Provides the path to the S3 bucket that contains the user context filtering files for the data source.</p>
    #[serde(rename = "AccessControlListConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list_configuration: Option<AccessControlListConfiguration>,
    /// <p>The name of the bucket that contains the documents.</p>
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "DocumentsMetadataConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_metadata_configuration: Option<DocumentsMetadataConfiguration>,
    /// <p>A list of glob patterns for documents that should not be indexed. If a document that matches an inclusion prefix also matches an exclusion pattern, the document is not indexed.</p> <p>For more information about glob patterns, see <a href="http://wikipedia.org/wiki/Glob_%28programming%29">glob (programming)</a> in <i>Wikipedia</i>.</p>
    #[serde(rename = "ExclusionPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    /// <p>A list of S3 prefixes for the documents that should be included in the index.</p>
    #[serde(rename = "InclusionPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_prefixes: Option<Vec<String>>,
}

/// <p>Information required to find a specific file in an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Path {
    /// <p>The name of the S3 bucket that contains the file.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>The name of the file.</p>
    #[serde(rename = "Key")]
    pub key: String,
}

/// <p>Provides information about how a custom index field is used during a search.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Search {
    /// <p>Determines whether the field is returned in the query response. The default is <code>true</code>.</p>
    #[serde(rename = "Displayable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayable: Option<bool>,
    /// <p>Indicates that the field can be used to create search facets, a count of results for each value in the field. The default is <code>false</code> .</p>
    #[serde(rename = "Facetable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facetable: Option<bool>,
    /// <p>Determines whether the field is used in the search. If the <code>Searchable</code> field is <code>true</code>, you can use relevance tuning to manually tune how Amazon Kendra weights the field in the search. The default is <code>true</code> for string fields and <code>false</code> for number and date fields.</p>
    #[serde(rename = "Searchable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
}

/// <p>Provides the identifier of the AWS KMS customer master key (CMK) used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerSideEncryptionConfiguration {
    /// <p>The identifier of the AWS KMS customer master key (CMK). Amazon Kendra doesn't support asymmetric CMKs.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

/// <p>Provides configuration information for connecting to a Microsoft SharePoint data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharePointConfiguration {
    /// <p> <code>TRUE</code> to include attachments to documents stored in your Microsoft SharePoint site in the index; otherwise, <code>FALSE</code>.</p>
    #[serde(rename = "CrawlAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_attachments: Option<bool>,
    /// <p>The Microsoft SharePoint attribute field that contains the title of the document.</p>
    #[serde(rename = "DocumentTitleFieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_title_field_name: Option<String>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Microsoft SharePoint attributes to custom fields in the Amazon Kendra index. You must first create the index fields using the operation before you map SharePoint attributes. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping Data Source Fields</a>.</p>
    #[serde(rename = "FieldMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping>>,
    /// <p>The Amazon Resource Name (ARN) of credentials stored in AWS Secrets Manager. The credentials should be a user/password pair. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-sharepoint.html">Using a Microsoft SharePoint Data Source</a>. For more information about AWS Secrets Manager, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html"> What Is AWS Secrets Manager </a> in the <i>AWS Secrets Manager</i> user guide.</p>
    #[serde(rename = "SecretArn")]
    pub secret_arn: String,
    /// <p>The version of Microsoft SharePoint that you are using as a data source.</p>
    #[serde(rename = "SharePointVersion")]
    pub share_point_version: String,
    /// <p>The URLs of the Microsoft SharePoint site that contains the documents that should be indexed.</p>
    #[serde(rename = "Urls")]
    pub urls: Vec<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<DataSourceVpcConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDataSourceSyncJobRequest {
    /// <p>The identifier of the data source to synchronize.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDataSourceSyncJobResponse {
    /// <p>Identifies a particular synchronization job.</p>
    #[serde(rename = "ExecutionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDataSourceSyncJobRequest {
    /// <p>The identifier of the data source for which to stop the synchronization jobs.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitFeedbackRequest {
    /// <p>Tells Amazon Kendra that a particular search result link was chosen by the user. </p>
    #[serde(rename = "ClickFeedbackItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_feedback_items: Option<Vec<ClickFeedback>>,
    /// <p>The identifier of the index that was queried.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The identifier of the specific query for which you are submitting feedback. The query ID is returned in the response to the operation.</p>
    #[serde(rename = "QueryId")]
    pub query_id: String,
    /// <p>Provides Amazon Kendra with relevant or not relevant feedback for whether a particular item was relevant to the search.</p>
    #[serde(rename = "RelevanceFeedbackItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance_feedback_items: Option<Vec<RelevanceFeedback>>,
}

/// <p>Provides information about text documents indexed in an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TextDocumentStatistics {
    /// <p>The number of text documents indexed.</p>
    #[serde(rename = "IndexedTextDocumentsCount")]
    pub indexed_text_documents_count: i64,
}

/// <p>Provides text and information about where to highlight the text.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TextWithHighlights {
    /// <p>The beginning and end of the text that should be highlighted.</p>
    #[serde(rename = "Highlights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<Highlight>>,
    /// <p>The text to display to the user.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// <p>Provides a range of time.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimeRange {
    /// <p>The UNIX datetime of the end of the time range.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The UNIX datetime of the beginning of the time range.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DataSourceConfiguration>,
    /// <p>The new description for the data source.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier of the data source to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The identifier of the index that contains the data source to update.</p>
    #[serde(rename = "IndexId")]
    pub index_id: String,
    /// <p>The name of the data source to update. The name of the data source can't be updated. To rename a data source you must delete the data source and re-create it.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the new role to use when the data source is accessing resources on your behalf.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The new update schedule for the data source.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIndexRequest {
    /// <p>A new description for the index.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The document metadata to update. </p>
    #[serde(rename = "DocumentMetadataConfigurationUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata_configuration_updates: Option<Vec<DocumentMetadataConfiguration>>,
    /// <p>The identifier of the index to update.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The name of the index to update.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A new IAM role that gives Amazon Kendra permission to access your Amazon CloudWatch logs.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// Errors returned by BatchDeleteDocument
#[derive(Debug, PartialEq)]
pub enum BatchDeleteDocumentError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl BatchDeleteDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchDeleteDocumentError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(BatchDeleteDocumentError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDeleteDocumentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BatchDeleteDocumentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchDeleteDocumentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeleteDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteDocumentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchDeleteDocumentError::Conflict(ref cause) => write!(f, "{}", cause),
            BatchDeleteDocumentError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDeleteDocumentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            BatchDeleteDocumentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteDocumentError {}
/// Errors returned by BatchPutDocument
#[derive(Debug, PartialEq)]
pub enum BatchPutDocumentError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    ServiceQuotaExceeded(String),
    /// <p><p/></p>
    Throttling(String),
}

impl BatchPutDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchPutDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchPutDocumentError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(BatchPutDocumentError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchPutDocumentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BatchPutDocumentError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(BatchPutDocumentError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchPutDocumentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchPutDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchPutDocumentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchPutDocumentError::Conflict(ref cause) => write!(f, "{}", cause),
            BatchPutDocumentError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchPutDocumentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            BatchPutDocumentError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            BatchPutDocumentError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchPutDocumentError {}
/// Errors returned by CreateDataSource
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceAlreadyExist(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    ServiceQuotaExceeded(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDataSourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDataSourceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDataSourceError::InternalServer(err.msg))
                }
                "ResourceAlreadyExistException" => {
                    return RusotoError::Service(CreateDataSourceError::ResourceAlreadyExist(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDataSourceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateDataSourceError::ServiceQuotaExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDataSourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataSourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::ResourceAlreadyExist(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataSourceError {}
/// Errors returned by CreateFaq
#[derive(Debug, PartialEq)]
pub enum CreateFaqError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    ServiceQuotaExceeded(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateFaqError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFaqError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateFaqError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateFaqError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateFaqError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFaqError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateFaqError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateFaqError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFaqError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFaqError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateFaqError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateFaqError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateFaqError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateFaqError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateFaqError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFaqError {}
/// Errors returned by CreateIndex
#[derive(Debug, PartialEq)]
pub enum CreateIndexError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceAlreadyExist(String),
    /// <p><p/></p>
    ServiceQuotaExceeded(String),
    /// <p><p/></p>
    Throttling(String),
}

impl CreateIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIndexError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateIndexError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateIndexError::InternalServer(err.msg))
                }
                "ResourceAlreadyExistException" => {
                    return RusotoError::Service(CreateIndexError::ResourceAlreadyExist(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateIndexError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateIndexError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateIndexError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateIndexError::ResourceAlreadyExist(ref cause) => write!(f, "{}", cause),
            CreateIndexError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIndexError {}
/// Errors returned by DeleteFaq
#[derive(Debug, PartialEq)]
pub enum DeleteFaqError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteFaqError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFaqError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteFaqError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteFaqError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteFaqError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFaqError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteFaqError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFaqError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFaqError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteFaqError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteFaqError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteFaqError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFaqError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFaqError {}
/// Errors returned by DeleteIndex
#[derive(Debug, PartialEq)]
pub enum DeleteIndexError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DeleteIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIndexError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteIndexError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteIndexError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteIndexError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteIndexError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteIndexError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteIndexError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteIndexError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIndexError {}
/// Errors returned by DescribeDataSource
#[derive(Debug, PartialEq)]
pub enum DescribeDataSourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DescribeDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDataSourceError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDataSourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDataSourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDataSourceError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDataSourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDataSourceError {}
/// Errors returned by DescribeFaq
#[derive(Debug, PartialEq)]
pub enum DescribeFaqError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DescribeFaqError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFaqError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeFaqError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeFaqError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFaqError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeFaqError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFaqError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFaqError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeFaqError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeFaqError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeFaqError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFaqError {}
/// Errors returned by DescribeIndex
#[derive(Debug, PartialEq)]
pub enum DescribeIndexError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl DescribeIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIndexError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeIndexError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeIndexError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIndexError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeIndexError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeIndexError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIndexError {}
/// Errors returned by ListDataSourceSyncJobs
#[derive(Debug, PartialEq)]
pub enum ListDataSourceSyncJobsError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListDataSourceSyncJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSourceSyncJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDataSourceSyncJobsError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListDataSourceSyncJobsError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListDataSourceSyncJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDataSourceSyncJobsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDataSourceSyncJobsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataSourceSyncJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataSourceSyncJobsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDataSourceSyncJobsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListDataSourceSyncJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDataSourceSyncJobsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDataSourceSyncJobsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSourceSyncJobsError {}
/// Errors returned by ListDataSources
#[derive(Debug, PartialEq)]
pub enum ListDataSourcesError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListDataSourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDataSourcesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListDataSourcesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDataSourcesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDataSourcesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDataSourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataSourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDataSourcesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataSourcesError {}
/// Errors returned by ListFaqs
#[derive(Debug, PartialEq)]
pub enum ListFaqsError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListFaqsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFaqsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFaqsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListFaqsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFaqsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFaqsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFaqsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFaqsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListFaqsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListFaqsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFaqsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFaqsError {}
/// Errors returned by ListIndices
#[derive(Debug, PartialEq)]
pub enum ListIndicesError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    Throttling(String),
}

impl ListIndicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIndicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListIndicesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListIndicesError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListIndicesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIndicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIndicesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListIndicesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListIndicesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIndicesError {}
/// Errors returned by Query
#[derive(Debug, PartialEq)]
pub enum QueryError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl QueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<QueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(QueryError::AccessDenied(err.msg))
                }
                "ConflictException" => return RusotoError::Service(QueryError::Conflict(err.msg)),
                "InternalServerException" => {
                    return RusotoError::Service(QueryError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(QueryError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(QueryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for QueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            QueryError::Conflict(ref cause) => write!(f, "{}", cause),
            QueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            QueryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            QueryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for QueryError {}
/// Errors returned by StartDataSourceSyncJob
#[derive(Debug, PartialEq)]
pub enum StartDataSourceSyncJobError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl StartDataSourceSyncJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDataSourceSyncJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartDataSourceSyncJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDataSourceSyncJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDataSourceSyncJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartDataSourceSyncJobError::Conflict(ref cause) => write!(f, "{}", cause),
            StartDataSourceSyncJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartDataSourceSyncJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartDataSourceSyncJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartDataSourceSyncJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDataSourceSyncJobError {}
/// Errors returned by StopDataSourceSyncJob
#[derive(Debug, PartialEq)]
pub enum StopDataSourceSyncJobError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl StopDataSourceSyncJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopDataSourceSyncJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopDataSourceSyncJobError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StopDataSourceSyncJobError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopDataSourceSyncJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopDataSourceSyncJobError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDataSourceSyncJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDataSourceSyncJobError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StopDataSourceSyncJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopDataSourceSyncJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopDataSourceSyncJobError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDataSourceSyncJobError {}
/// Errors returned by SubmitFeedback
#[derive(Debug, PartialEq)]
pub enum SubmitFeedbackError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    ResourceUnavailable(String),
    /// <p><p/></p>
    Throttling(String),
}

impl SubmitFeedbackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitFeedbackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitFeedbackError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(SubmitFeedbackError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SubmitFeedbackError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(SubmitFeedbackError::ResourceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SubmitFeedbackError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitFeedbackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitFeedbackError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::InternalServer(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            SubmitFeedbackError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitFeedbackError {}
/// Errors returned by UpdateDataSource
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourceError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UpdateDataSourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDataSourceError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDataSourceError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDataSourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDataSourceError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDataSourceError::Throttling(err.msg))
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
            UpdateDataSourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDataSourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDataSourceError {}
/// Errors returned by UpdateIndex
#[derive(Debug, PartialEq)]
pub enum UpdateIndexError {
    /// <p><p/></p>
    AccessDenied(String),
    /// <p><p/></p>
    Conflict(String),
    /// <p><p/></p>
    InternalServer(String),
    /// <p><p/></p>
    ResourceNotFound(String),
    /// <p><p/></p>
    Throttling(String),
}

impl UpdateIndexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIndexError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateIndexError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateIndexError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateIndexError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateIndexError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateIndexError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIndexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIndexError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateIndexError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateIndexError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateIndexError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateIndexError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIndexError {}
/// Trait representing the capabilities of the kendra API. kendra clients implement this trait.
pub trait Kendra {
    /// <p>Removes one or more documents from an index. The documents must have been added with the <a>BatchPutDocument</a> operation.</p> <p>The documents are deleted asynchronously. You can see the progress of the deletion by using AWS CloudWatch. Any error messages releated to the processing of the batch are sent to you CloudWatch log.</p>
    fn batch_delete_document(
        &self,
        input: BatchDeleteDocumentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        BatchDeleteDocumentResponse,
                        RusotoError<BatchDeleteDocumentError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Adds one or more documents to an index.</p> <p>The <code>BatchPutDocument</code> operation enables you to ingest inline documents or a set of documents stored in an Amazon S3 bucket. Use this operation to ingest your text and unstructured text into an index, add custom attributes to the documents, and to attach an access control list to the documents added to the index.</p> <p>The documents are indexed asynchronously. You can see the progress of the batch using AWS CloudWatch. Any error messages related to processing the batch are sent to your AWS CloudWatch log.</p>
    fn batch_put_document(
        &self,
        input: BatchPutDocumentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<BatchPutDocumentResponse, RusotoError<BatchPutDocumentError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates a data source that you use to with an Amazon Kendra index. </p> <p>You specify a name, connector type and description for your data source. You can choose between an S3 connector, a SharePoint Online connector, and a database connector.</p> <p>You also specify configuration information such as document metadata (author, source URI, and so on) and user context information.</p> <p> <code>CreateDataSource</code> is a synchronous operation. The operation returns 200 if the data source was successfully created. Otherwise, an exception is raised.</p>
    fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Creates an new set of frequently asked question (FAQ) questions and answers.</p>
    fn create_faq(
        &self,
        input: CreateFaqRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateFaqResponse, RusotoError<CreateFaqError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Creates a new Amazon Kendra index. Index creation is an asynchronous operation. To determine if index creation has completed, check the <code>Status</code> field returned from a call to . The <code>Status</code> field is set to <code>ACTIVE</code> when the index is ready to use.</p> <p>Once the index is active you can index your documents using the operation or using one of the supported data sources. </p>
    fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateIndexResponse, RusotoError<CreateIndexError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Removes an FAQ from an index.</p>
    fn delete_faq(
        &self,
        input: DeleteFaqRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteFaqError>>> + Send + 'static>>;

    /// <p>Deletes an existing Amazon Kendra index. An exception is not thrown if the index is already being deleted. While the index is being deleted, the <code>Status</code> field returned by a call to the <a>DescribeIndex</a> operation is set to <code>DELETING</code>.</p>
    fn delete_index(
        &self,
        input: DeleteIndexRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteIndexError>>> + Send + 'static>>;

    /// <p>Gets information about a Amazon Kendra data source.</p>
    fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDataSourceResponse,
                        RusotoError<DescribeDataSourceError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Gets information about an FAQ list.</p>
    fn describe_faq(
        &self,
        input: DescribeFaqRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeFaqResponse, RusotoError<DescribeFaqError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Describes an existing Amazon Kendra index</p>
    fn describe_index(
        &self,
        input: DescribeIndexRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeIndexResponse, RusotoError<DescribeIndexError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Gets statistics about synchronizing Amazon Kendra with a data source.</p>
    fn list_data_source_sync_jobs(
        &self,
        input: ListDataSourceSyncJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDataSourceSyncJobsResponse,
                        RusotoError<ListDataSourceSyncJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists the data sources that you have created.</p>
    fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Gets a list of FAQ lists associated with an index.</p>
    fn list_faqs(
        &self,
        input: ListFaqsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFaqsResponse, RusotoError<ListFaqsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Lists the Amazon Kendra indexes that you have created.</p>
    fn list_indices(
        &self,
        input: ListIndicesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListIndicesResponse, RusotoError<ListIndicesError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Searches an active index. Use this API to search your documents using query. The <code>Query</code> operation enables to do faceted search and to filter results based on document attributes.</p> <p>It also enables you to provide user context that Amazon Kendra uses to enforce document access control in the search results. </p> <p>Amazon Kendra searches your index for text content and question and answer (FAQ) content. By default the response contains three types of results.</p> <ul> <li> <p>Relevant passages</p> </li> <li> <p>Matching FAQs</p> </li> <li> <p>Relevant documents</p> </li> </ul> <p>You can specify that the query return only one type of result using the <code>QueryResultTypeConfig</code> parameter.</p>
    fn query(
        &self,
        input: QueryRequest,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, RusotoError<QueryError>>> + Send + 'static>>;

    /// <p>Starts a synchronization job for a data source. If a synchronization job is already in progress, Amazon Kendra returns a <code>ResourceInUseException</code> exception.</p>
    fn start_data_source_sync_job(
        &self,
        input: StartDataSourceSyncJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartDataSourceSyncJobResponse,
                        RusotoError<StartDataSourceSyncJobError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Stops a running synchronization job. You can't stop a scheduled synchronization job.</p>
    fn stop_data_source_sync_job(
        &self,
        input: StopDataSourceSyncJobRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<StopDataSourceSyncJobError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Enables you to provide feedback to Amazon Kendra to improve the performance of the service. </p>
    fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<SubmitFeedbackError>>> + Send + 'static>>;

    /// <p>Updates an existing Amazon Kendra data source.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<UpdateDataSourceError>>> + Send + 'static>,
    >;

    /// <p>Updates an existing Amazon Kendra index.</p>
    fn update_index(
        &self,
        input: UpdateIndexRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UpdateIndexError>>> + Send + 'static>>;
}
/// A client for the kendra API.
#[derive(Clone)]
pub struct KendraClient {
    client: Client,
    region: region::Region,
}

impl KendraClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KendraClient {
        KendraClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KendraClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KendraClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KendraClient {
        KendraClient { client, region }
    }
}

impl Kendra for KendraClient {
    /// <p>Removes one or more documents from an index. The documents must have been added with the <a>BatchPutDocument</a> operation.</p> <p>The documents are deleted asynchronously. You can see the progress of the deletion by using AWS CloudWatch. Any error messages releated to the processing of the batch are sent to you CloudWatch log.</p>
    fn batch_delete_document(
        &self,
        input: BatchDeleteDocumentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        BatchDeleteDocumentResponse,
                        RusotoError<BatchDeleteDocumentError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.BatchDeleteDocument",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<BatchDeleteDocumentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(BatchDeleteDocumentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Adds one or more documents to an index.</p> <p>The <code>BatchPutDocument</code> operation enables you to ingest inline documents or a set of documents stored in an Amazon S3 bucket. Use this operation to ingest your text and unstructured text into an index, add custom attributes to the documents, and to attach an access control list to the documents added to the index.</p> <p>The documents are indexed asynchronously. You can see the progress of the batch using AWS CloudWatch. Any error messages related to processing the batch are sent to your AWS CloudWatch log.</p>
    fn batch_put_document(
        &self,
        input: BatchPutDocumentRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<BatchPutDocumentResponse, RusotoError<BatchPutDocumentError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.BatchPutDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<BatchPutDocumentResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(BatchPutDocumentError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a data source that you use to with an Amazon Kendra index. </p> <p>You specify a name, connector type and description for your data source. You can choose between an S3 connector, a SharePoint Online connector, and a database connector.</p> <p>You also specify configuration information such as document metadata (author, source URI, and so on) and user context information.</p> <p> <code>CreateDataSource</code> is a synchronous operation. The operation returns 200 if the data source was successfully created. Otherwise, an exception is raised.</p>
    fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<CreateDataSourceResponse, RusotoError<CreateDataSourceError>>,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<CreateDataSourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateDataSourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates an new set of frequently asked question (FAQ) questions and answers.</p>
    fn create_faq(
        &self,
        input: CreateFaqRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateFaqResponse, RusotoError<CreateFaqError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<CreateFaqResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateFaqError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Creates a new Amazon Kendra index. Index creation is an asynchronous operation. To determine if index creation has completed, check the <code>Status</code> field returned from a call to . The <code>Status</code> field is set to <code>ACTIVE</code> when the index is ready to use.</p> <p>Once the index is active you can index your documents using the operation or using one of the supported data sources. </p>
    fn create_index(
        &self,
        input: CreateIndexRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<CreateIndexResponse, RusotoError<CreateIndexError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.CreateIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<CreateIndexResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(CreateIndexError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes an FAQ from an index.</p>
    fn delete_faq(
        &self,
        input: DeleteFaqRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteFaqError>>> + Send + 'static>>
    {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteFaqError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Deletes an existing Amazon Kendra index. An exception is not thrown if the index is already being deleted. While the index is being deleted, the <code>Status</code> field returned by a call to the <a>DescribeIndex</a> operation is set to <code>DELETING</code>.</p>
    fn delete_index(
        &self,
        input: DeleteIndexRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<DeleteIndexError>>> + Send + 'static>>
    {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.DeleteIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteIndexError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Gets information about a Amazon Kendra data source.</p>
    fn describe_data_source(
        &self,
        input: DescribeDataSourceRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeDataSourceResponse,
                        RusotoError<DescribeDataSourceError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.DescribeDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeDataSourceResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeDataSourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Gets information about an FAQ list.</p>
    fn describe_faq(
        &self,
        input: DescribeFaqRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeFaqResponse, RusotoError<DescribeFaqError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.DescribeFaq");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<DescribeFaqResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeFaqError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes an existing Amazon Kendra index</p>
    fn describe_index(
        &self,
        input: DescribeIndexRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DescribeIndexResponse, RusotoError<DescribeIndexError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.DescribeIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeIndexResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeIndexError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Gets statistics about synchronizing Amazon Kendra with a data source.</p>
    fn list_data_source_sync_jobs(
        &self,
        input: ListDataSourceSyncJobsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListDataSourceSyncJobsResponse,
                        RusotoError<ListDataSourceSyncJobsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.ListDataSourceSyncJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDataSourceSyncJobsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListDataSourceSyncJobsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the data sources that you have created.</p>
    fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListDataSourcesResponse, RusotoError<ListDataSourcesError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListDataSources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListDataSourcesResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListDataSourcesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Gets a list of FAQ lists associated with an index.</p>
    fn list_faqs(
        &self,
        input: ListFaqsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListFaqsResponse, RusotoError<ListFaqsError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListFaqs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<ListFaqsResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListFaqsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists the Amazon Kendra indexes that you have created.</p>
    fn list_indices(
        &self,
        input: ListIndicesRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListIndicesResponse, RusotoError<ListIndicesError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.ListIndices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<ListIndicesResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(ListIndicesError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Searches an active index. Use this API to search your documents using query. The <code>Query</code> operation enables to do faceted search and to filter results based on document attributes.</p> <p>It also enables you to provide user context that Amazon Kendra uses to enforce document access control in the search results. </p> <p>Amazon Kendra searches your index for text content and question and answer (FAQ) content. By default the response contains three types of results.</p> <ul> <li> <p>Relevant passages</p> </li> <li> <p>Matching FAQs</p> </li> <li> <p>Relevant documents</p> </li> </ul> <p>You can specify that the query return only one type of result using the <code>QueryResultTypeConfig</code> parameter.</p>
    fn query(
        &self,
        input: QueryRequest,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, RusotoError<QueryError>>> + Send + 'static>>
    {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.Query");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response).deserialize::<QueryResult, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(QueryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Starts a synchronization job for a data source. If a synchronization job is already in progress, Amazon Kendra returns a <code>ResourceInUseException</code> exception.</p>
    fn start_data_source_sync_job(
        &self,
        input: StartDataSourceSyncJobRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        StartDataSourceSyncJobResponse,
                        RusotoError<StartDataSourceSyncJobError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.StartDataSourceSyncJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                proto::json::ResponsePayload::new(&response)
                    .deserialize::<StartDataSourceSyncJobResponse, _>()
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(StartDataSourceSyncJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Stops a running synchronization job. You can't stop a scheduled synchronization job.</p>
    fn stop_data_source_sync_job(
        &self,
        input: StopDataSourceSyncJobRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<(), RusotoError<StopDataSourceSyncJobError>>>
                + Send
                + 'static,
        >,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSKendraFrontendService.StopDataSourceSyncJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(StopDataSourceSyncJobError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Enables you to provide feedback to Amazon Kendra to improve the performance of the service. </p>
    fn submit_feedback(
        &self,
        input: SubmitFeedbackRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<SubmitFeedbackError>>> + Send + 'static>>
    {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.SubmitFeedback");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(SubmitFeedbackError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates an existing Amazon Kendra data source.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), RusotoError<UpdateDataSourceError>>> + Send + 'static>,
    > {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.UpdateDataSource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateDataSourceError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Updates an existing Amazon Kendra index.</p>
    fn update_index(
        &self,
        input: UpdateIndexRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), RusotoError<UpdateIndexError>>> + Send + 'static>>
    {
        let mut request = SignedRequest::new("POST", "kendra", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSKendraFrontendService.UpdateIndex");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                std::mem::drop(response);
                Ok(())
            } else {
                let try_response = response.buffer().await;
                let response = try_response.map_err(RusotoError::HttpDispatch)?;
                Err(UpdateIndexError::from_response(response))
            }
        }
        .boxed()
    }
}
